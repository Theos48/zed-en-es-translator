#include <algorithm>
#include <array>
#include <cctype>
#include <cstdint>
#include <iostream>
#include <limits>
#include <memory>
#include <optional>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

#ifndef TRANSLATOR_EMBEDDED_CONTROLLED_FIXTURE
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wcomment"
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#pragma GCC diagnostic ignored "-Wignored-qualifiers"
#pragma GCC diagnostic ignored "-Wpedantic"
#pragma GCC diagnostic ignored "-Wreorder"
#pragma GCC diagnostic ignored "-Wsign-compare"
#pragma GCC diagnostic ignored "-Wunknown-pragmas"
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-value"
#include "translator/parser.h"
#include "translator/response_options.h"
#include "translator/service.h"
#include "translator/translation_model.h"
#pragma GCC diagnostic pop
#endif

namespace {

constexpr std::size_t kMaximumWireBytes = 32U * 1024U;
constexpr std::size_t kMaximumSegments = 256U;
constexpr std::size_t kMaximumSegmentBytes = 4U * 1024U;
constexpr std::size_t kMaximumAggregateBytes = 20U * 1024U;

class Parser {
 public:
  explicit Parser(std::string_view input) : input_(input) {}

  bool consume(char expected) {
    whitespace();
    if (position_ >= input_.size() || input_[position_] != expected) {
      return false;
    }
    ++position_;
    return true;
  }

  std::optional<std::uint32_t> unsigned_integer() {
    whitespace();
    if (position_ >= input_.size() || !ascii_digit(input_[position_])) {
      return std::nullopt;
    }
    std::uint64_t value = 0;
    do {
      value = value * 10U + static_cast<unsigned>(input_[position_] - '0');
      if (value > std::numeric_limits<std::uint32_t>::max()) {
        return std::nullopt;
      }
      ++position_;
    } while (position_ < input_.size() && ascii_digit(input_[position_]));
    return static_cast<std::uint32_t>(value);
  }

  std::optional<std::string> string() {
    whitespace();
    if (position_ >= input_.size() || input_[position_++] != '"') {
      return std::nullopt;
    }
    std::string output;
    while (position_ < input_.size()) {
      const unsigned char byte = static_cast<unsigned char>(input_[position_++]);
      if (byte == '"') {
        return valid_utf8(output) ? std::optional<std::string>(std::move(output))
                                  : std::nullopt;
      }
      if (byte < 0x20U) {
        return std::nullopt;
      }
      if (byte != '\\') {
        output.push_back(static_cast<char>(byte));
        continue;
      }
      if (position_ >= input_.size()) {
        return std::nullopt;
      }
      const char escaped = input_[position_++];
      switch (escaped) {
        case '"':
        case '\\':
        case '/':
          output.push_back(escaped);
          break;
        case 'b':
          output.push_back('\b');
          break;
        case 'f':
          output.push_back('\f');
          break;
        case 'n':
          output.push_back('\n');
          break;
        case 'r':
          output.push_back('\r');
          break;
        case 't':
          output.push_back('\t');
          break;
        case 'u': {
          const auto code_point = unicode_escape();
          if (!code_point.has_value() || !append_utf8(*code_point, output)) {
            return std::nullopt;
          }
          break;
        }
        default:
          return std::nullopt;
      }
    }
    return std::nullopt;
  }

  std::optional<std::vector<std::string>> string_array() {
    if (!consume('[')) {
      return std::nullopt;
    }
    std::vector<std::string> values;
    whitespace();
    if (peek(']')) {
      ++position_;
      return values;
    }
    while (true) {
      auto value = string();
      if (!value.has_value()) {
        return std::nullopt;
      }
      values.push_back(std::move(*value));
      whitespace();
      if (peek(']')) {
        ++position_;
        return values;
      }
      if (!consume(',')) {
        return std::nullopt;
      }
    }
  }

  bool finished() {
    whitespace();
    return position_ == input_.size();
  }

 private:
  static bool ascii_digit(char value) { return value >= '0' && value <= '9'; }

  void whitespace() {
    while (position_ < input_.size() &&
           std::isspace(static_cast<unsigned char>(input_[position_])) != 0) {
      ++position_;
    }
  }

  bool peek(char expected) const {
    return position_ < input_.size() && input_[position_] == expected;
  }

  static std::optional<std::uint16_t> hex_digit(char value) {
    if (value >= '0' && value <= '9') {
      return static_cast<std::uint16_t>(value - '0');
    }
    if (value >= 'a' && value <= 'f') {
      return static_cast<std::uint16_t>(10 + value - 'a');
    }
    if (value >= 'A' && value <= 'F') {
      return static_cast<std::uint16_t>(10 + value - 'A');
    }
    return std::nullopt;
  }

  std::optional<std::uint32_t> code_unit() {
    if (input_.size() - position_ < 4U) {
      return std::nullopt;
    }
    std::uint32_t value = 0;
    for (int index = 0; index < 4; ++index) {
      const auto digit = hex_digit(input_[position_++]);
      if (!digit.has_value()) {
        return std::nullopt;
      }
      value = value * 16U + *digit;
    }
    return value;
  }

  std::optional<std::uint32_t> unicode_escape() {
    const auto first = code_unit();
    if (!first.has_value()) {
      return std::nullopt;
    }
    if (*first < 0xD800U || *first > 0xDFFFU) {
      return first;
    }
    if (*first > 0xDBFFU || input_.size() - position_ < 6U ||
        input_[position_] != '\\' || input_[position_ + 1U] != 'u') {
      return std::nullopt;
    }
    position_ += 2U;
    const auto second = code_unit();
    if (!second.has_value() || *second < 0xDC00U || *second > 0xDFFFU) {
      return std::nullopt;
    }
    return 0x10000U + ((*first - 0xD800U) << 10U) + (*second - 0xDC00U);
  }

  static bool append_utf8(std::uint32_t code_point, std::string& output) {
    if (code_point <= 0x7FU) {
      output.push_back(static_cast<char>(code_point));
    } else if (code_point <= 0x7FFU) {
      output.push_back(static_cast<char>(0xC0U | (code_point >> 6U)));
      output.push_back(static_cast<char>(0x80U | (code_point & 0x3FU)));
    } else if (code_point <= 0xFFFFU) {
      output.push_back(static_cast<char>(0xE0U | (code_point >> 12U)));
      output.push_back(static_cast<char>(0x80U | ((code_point >> 6U) & 0x3FU)));
      output.push_back(static_cast<char>(0x80U | (code_point & 0x3FU)));
    } else if (code_point <= 0x10FFFFU) {
      output.push_back(static_cast<char>(0xF0U | (code_point >> 18U)));
      output.push_back(static_cast<char>(0x80U | ((code_point >> 12U) & 0x3FU)));
      output.push_back(static_cast<char>(0x80U | ((code_point >> 6U) & 0x3FU)));
      output.push_back(static_cast<char>(0x80U | (code_point & 0x3FU)));
    } else {
      return false;
    }
    return true;
  }

  static bool valid_utf8(std::string_view value) {
    std::size_t position = 0;
    while (position < value.size()) {
      const auto lead = static_cast<unsigned char>(value[position]);
      std::size_t continuation = 0;
      std::uint32_t minimum = 0;
      std::uint32_t code_point = 0;
      if (lead <= 0x7FU) {
        ++position;
        continue;
      }
      if ((lead & 0xE0U) == 0xC0U) {
        continuation = 1;
        minimum = 0x80U;
        code_point = lead & 0x1FU;
      } else if ((lead & 0xF0U) == 0xE0U) {
        continuation = 2;
        minimum = 0x800U;
        code_point = lead & 0x0FU;
      } else if ((lead & 0xF8U) == 0xF0U) {
        continuation = 3;
        minimum = 0x10000U;
        code_point = lead & 0x07U;
      } else {
        return false;
      }
      if (value.size() - position <= continuation) {
        return false;
      }
      for (std::size_t index = 1; index <= continuation; ++index) {
        const auto byte = static_cast<unsigned char>(value[position + index]);
        if ((byte & 0xC0U) != 0x80U) {
          return false;
        }
        code_point = (code_point << 6U) | (byte & 0x3FU);
      }
      if (code_point < minimum || code_point > 0x10FFFFU ||
          (code_point >= 0xD800U && code_point <= 0xDFFFU)) {
        return false;
      }
      position += continuation + 1U;
    }
    return true;
  }

  std::string_view input_;
  std::size_t position_ = 0;
};

struct Request {
  std::vector<std::string> segments;
};

std::optional<Request> parse_request(std::string_view input) {
  Parser parser(input);
  if (!parser.consume('{')) {
    return std::nullopt;
  }
  bool wire_seen = false;
  bool source_seen = false;
  bool target_seen = false;
  bool tone_seen = false;
  bool preserve_seen = false;
  bool segments_seen = false;
  Request request;
  while (true) {
    auto key = parser.string();
    if (!key.has_value() || !parser.consume(':')) {
      return std::nullopt;
    }
    if (*key == "wire_version" && !wire_seen) {
      wire_seen = true;
      const auto value = parser.unsigned_integer();
      if (!value.has_value() || *value != 1U) {
        return std::nullopt;
      }
    } else if (*key == "source_language" && !source_seen) {
      source_seen = true;
      if (parser.string() != std::optional<std::string>("en")) {
        return std::nullopt;
      }
    } else if (*key == "target_language" && !target_seen) {
      target_seen = true;
      if (parser.string() != std::optional<std::string>("es")) {
        return std::nullopt;
      }
    } else if (*key == "tone" && !tone_seen) {
      tone_seen = true;
      if (parser.string() != std::optional<std::string>("technical_neutral")) {
        return std::nullopt;
      }
    } else if (*key == "preserve" && !preserve_seen) {
      preserve_seen = true;
      const auto values = parser.string_array();
      const std::vector<std::string> expected = {"markdown_structure", "code", "links"};
      if (!values.has_value() || *values != expected) {
        return std::nullopt;
      }
    } else if (*key == "segments" && !segments_seen) {
      segments_seen = true;
      auto values = parser.string_array();
      if (!values.has_value()) {
        return std::nullopt;
      }
      request.segments = std::move(*values);
    } else {
      return std::nullopt;
    }
    if (parser.consume('}')) {
      break;
    }
    if (!parser.consume(',')) {
      return std::nullopt;
    }
  }
  if (!parser.finished() || !wire_seen || !source_seen || !target_seen || !tone_seen ||
      !preserve_seen || !segments_seen || request.segments.empty() ||
      request.segments.size() > kMaximumSegments) {
    return std::nullopt;
  }
  std::size_t aggregate = 0;
  for (const auto& segment : request.segments) {
    if (segment.empty() || segment.size() > kMaximumSegmentBytes ||
        aggregate > kMaximumAggregateBytes - segment.size()) {
      return std::nullopt;
    }
    aggregate += segment.size();
  }
  return request;
}

std::string json_string(std::string_view input) {
  std::string output = "\"";
  for (const unsigned char byte : input) {
    switch (byte) {
      case '"':
        output += "\\\"";
        break;
      case '\\':
        output += "\\\\";
        break;
      case '\n':
        output += "\\n";
        break;
      case '\r':
        output += "\\r";
        break;
      case '\t':
        output += "\\t";
        break;
      default:
        if (byte < 0x20U) {
          constexpr std::array<char, 16> hexadecimal = {'0', '1', '2', '3', '4', '5', '6', '7',
                                                        '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'};
          output += "\\u00";
          output.push_back(hexadecimal[byte >> 4U]);
          output.push_back(hexadecimal[byte & 0x0FU]);
        } else {
          output.push_back(static_cast<char>(byte));
        }
    }
  }
  output.push_back('"');
  return output;
}

#ifdef TRANSLATOR_EMBEDDED_CONTROLLED_FIXTURE
std::string controlled_translation(std::string_view input) {
  if (input == "Public synthetic text.") {
    return "Texto sintetico publico.";
  }
  if (input == "One.") {
    return "Uno.";
  }
  if (input == "Two.") {
    return "Dos.";
  }
  return "Texto sintetico controlado.";
}
#endif

#ifndef TRANSLATOR_EMBEDDED_CONTROLLED_FIXTURE

struct ModelPaths {
  std::string model;
  std::string vocabulary;
  std::string lexical_shortlist;
};

bool safe_object_path(std::string_view value) {
  constexpr std::string_view prefix = "objects/";
  if (value.size() <= prefix.size() + 65U || value.substr(0U, prefix.size()) != prefix) {
    return false;
  }
  const auto digest = value.substr(prefix.size(), 64U);
  if (digest.size() != 64U ||
      !std::all_of(digest.begin(), digest.end(), [](unsigned char byte) {
        return std::isxdigit(byte) != 0;
      }) ||
      value[prefix.size() + digest.size()] != '/') {
    return false;
  }
  const auto basename = value.substr(prefix.size() + digest.size() + 1U);
  return !basename.empty() && basename.size() <= 128U &&
         std::all_of(basename.begin(), basename.end(), [](unsigned char byte) {
           return std::isalnum(byte) != 0 || byte == '.' || byte == '_' || byte == '-';
         });
}

std::optional<ModelPaths> parse_model_paths(int argc, char** argv) {
  if (argc != 7) {
    return std::nullopt;
  }
  ModelPaths paths;
  bool model_seen = false;
  bool vocabulary_seen = false;
  bool shortlist_seen = false;
  for (int index = 1; index < argc; index += 2) {
    const std::string_view flag(argv[index]);
    const std::string_view value(argv[index + 1]);
    if (!safe_object_path(value)) {
      return std::nullopt;
    }
    if (flag == "--model" && !model_seen) {
      model_seen = true;
      paths.model = value;
    } else if (flag == "--vocabulary" && !vocabulary_seen) {
      vocabulary_seen = true;
      paths.vocabulary = value;
    } else if (flag == "--lexical-shortlist" && !shortlist_seen) {
      shortlist_seen = true;
      paths.lexical_shortlist = value;
    } else {
      return std::nullopt;
    }
  }
  if (!model_seen || !vocabulary_seen || !shortlist_seen) {
    return std::nullopt;
  }
  return paths;
}

std::string model_config(const ModelPaths& paths) {
  std::ostringstream config;
  config << "models: [" << paths.model << "]\n"
         << "vocabs: [" << paths.vocabulary << ", " << paths.vocabulary << "]\n"
         << "shortlist: [" << paths.lexical_shortlist << ", false]\n"
         << "beam-size: 1\n"
         << "normalize: 1.0\n"
         << "word-penalty: 0\n"
         << "max-length-break: 128\n"
         << "mini-batch-words: 1024\n"
         << "workspace: 128\n"
         << "max-length-factor: 2.0\n"
         << "skip-cost: true\n"
         << "quiet: true\n"
         << "quiet-translation: true\n"
         << "gemm-precision: int8shiftAlphaAll\n"
         << "alignment: soft\n";
  return config.str();
}

std::optional<std::vector<std::string>> translate_with_bergamot(
    const ModelPaths& paths, std::vector<std::string> segments) {
  using marian::bergamot::BlockingService;
  using marian::bergamot::ResponseOptions;
  using marian::bergamot::TranslationModel;
  using marian::bergamot::parseOptionsFromString;

  BlockingService::Config service_config;
  service_config.cacheSize = 0;
  service_config.logger.level = "off";
  BlockingService service(service_config);
  const auto options = parseOptionsFromString(model_config(paths));
  auto model = std::make_shared<TranslationModel>(options);
  const std::vector<ResponseOptions> response_options(segments.size());
  const auto responses =
      service.translateMultiple(std::move(model), std::move(segments), response_options);
  std::vector<std::string> translations;
  translations.reserve(responses.size());
  for (const auto& response : responses) {
    if (response.target.text.empty()) {
      return std::nullopt;
    }
    translations.push_back(response.target.text);
  }
  return translations;
}

#endif

int fail(std::string_view error = "INVALID_REQUEST") {
  std::cout << R"({"wire_version":1,"error":)" << json_string(error) << '}';
  return 1;
}

}  // namespace

int main(int argc, char** argv) {
#ifdef TRANSLATOR_EMBEDDED_CONTROLLED_FIXTURE
  static_cast<void>(argv);
  if (argc != 1) {
    return fail();
  }
#else
  const auto model_paths = parse_model_paths(argc, argv);
  if (!model_paths.has_value()) {
    return fail();
  }
#endif
  std::string input;
  input.reserve(kMaximumWireBytes);
  char byte = 0;
  while (std::cin.get(byte)) {
    if (input.size() == kMaximumWireBytes) {
      return fail();
    }
    input.push_back(byte);
  }
  const auto request = parse_request(input);
  if (!request.has_value()) {
    return fail();
  }

#ifdef TRANSLATOR_EMBEDDED_CONTROLLED_FIXTURE
  std::vector<std::string> translations;
  translations.reserve(request->segments.size());
  for (const auto& segment : request->segments) {
    translations.push_back(controlled_translation(segment));
  }
#else
  std::optional<std::vector<std::string>> translated;
  try {
    translated = translate_with_bergamot(*model_paths, std::move(request->segments));
  } catch (...) {
    return fail("RUNTIME_FAILED");
  }
  if (!translated.has_value()) {
    return fail("RUNTIME_FAILED");
  }
  auto& translations = *translated;
#endif

  std::cout << R"({"wire_version":1,"translations":[)";
  for (std::size_t index = 0; index < translations.size(); ++index) {
    if (index != 0U) {
      std::cout << ',';
    }
    std::cout << json_string(translations[index]);
  }
  std::cout << "]}";
  return 0;
}
