use crate::{
    ensure_provider_response_shape, redact_failure, ErrorCode, InputKind, Language, Provider,
    ProviderRequest, Tone, TranslateFailure, MAX_SEGMENTS, MAX_SEGMENT_BYTES,
};

pub fn translate_document(
    content: &str,
    input_kind: InputKind,
    provider: &impl Provider,
) -> Result<String, TranslateFailure> {
    let mut segment_count = 0;
    let translated = match input_kind {
        InputKind::Text => translate_visible_text(content, provider, &mut segment_count),
        InputKind::Markdown => translate_markdown(content, provider, &mut segment_count),
    }?;

    if segment_count == 0 {
        return Err(no_translatable_segments());
    }

    Ok(translated)
}

fn translate_markdown(
    content: &str,
    provider: &impl Provider,
    segment_count: &mut usize,
) -> Result<String, TranslateFailure> {
    let mut output = String::new();
    let mut in_fence: Option<String> = None;
    let mut in_frontmatter = false;
    let mut frontmatter_checked = false;
    let mut in_html_block: Option<HtmlBlock> = None;
    let mut in_code_span: Option<usize> = None;

    for line in content.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\r', '\n']);
        let start = trimmed.trim_start();

        if !frontmatter_checked {
            frontmatter_checked = true;
            if trimmed == "---" {
                in_frontmatter = true;
                output.push_str(line);
                continue;
            }
        }

        if in_frontmatter {
            output.push_str(line);
            if trimmed == "---" {
                in_frontmatter = false;
            }
            continue;
        }

        if let Some(marker) = &in_fence {
            output.push_str(line);
            if starts_with_fence_marker(start, marker) {
                in_fence = None;
            }
            continue;
        }

        if let Some(marker) = opening_fence_marker(start) {
            in_fence = Some(marker);
            output.push_str(line);
            continue;
        }

        if let Some(html_block) = in_html_block {
            output.push_str(line);
            let blank_line_closes_block =
                html_block.closes_on_blank_line() && trimmed.trim().is_empty();
            if html_block.closes_with_tag(start) || blank_line_closes_block {
                in_html_block = None;
            }
            continue;
        }

        if let Some(html_block) = html_block(start) {
            output.push_str(line);
            if !html_block.closes_with_tag(start) && !is_self_closing_html_block(start) {
                in_html_block = Some(html_block);
            }
            continue;
        }

        let translated =
            translate_inline_visible(line, provider, &mut in_code_span, segment_count)?;
        output.push_str(&translated);
    }

    Ok(output)
}

fn translate_inline_visible(
    line: &str,
    provider: &impl Provider,
    in_code_span: &mut Option<usize>,
    segment_count: &mut usize,
) -> Result<String, TranslateFailure> {
    let mut output = String::new();
    let mut cursor = 0;

    if let Some(tick_count) = *in_code_span {
        match find_closing_backticks(line, 0, tick_count) {
            Some(end_start) => {
                let end = end_start + tick_count;
                output.push_str(&line[..end]);
                cursor = end;
                *in_code_span = None;
            }
            None => {
                output.push_str(line);
                return Ok(output);
            }
        }
    }

    while let Some(relative_start) = line[cursor..].find('`') {
        let start = cursor + relative_start;
        if start > cursor {
            let translated =
                translate_visible_markdown_text(&line[cursor..start], provider, segment_count)?;
            output.push_str(&translated);
        }

        let tick_count = count_backticks(&line[start..]);
        let after_open = start + tick_count;
        let closing = find_closing_backticks(line, after_open, tick_count);
        match closing {
            Some(end_start) => {
                let end = end_start + tick_count;
                output.push_str(&line[start..end]);
                cursor = end;
            }
            None => {
                output.push_str(&line[start..]);
                *in_code_span = Some(tick_count);
                return Ok(output);
            }
        }
    }

    if cursor < line.len() {
        let translated = translate_visible_markdown_text(&line[cursor..], provider, segment_count)?;
        output.push_str(&translated);
    }

    Ok(output)
}

fn translate_visible_text(
    text: &str,
    provider: &impl Provider,
    segment_count: &mut usize,
) -> Result<String, TranslateFailure> {
    if text.trim().is_empty() {
        return Ok(text.to_string());
    }

    let segments = split_translatable_text(text);
    reserve_segments(segment_count, segments.len())?;
    let request = ProviderRequest::new(
        segments,
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )?;
    let response = provider.translate(&request).map_err(redact_failure)?;
    ensure_provider_response_shape(&request, &response)?;

    Ok(response.translated_segments.concat())
}

fn translate_visible_markdown_text(
    text: &str,
    provider: &impl Provider,
    segment_count: &mut usize,
) -> Result<String, TranslateFailure> {
    let mut output = String::new();
    let mut cursor = 0;

    while let Some(relative_start) = text[cursor..].find("](") {
        let marker_start = cursor + relative_start;
        let destination_start = marker_start + 2;
        if let Some(destination_end) = find_link_destination_end(text, destination_start) {
            let translated =
                translate_visible_text(&text[cursor..destination_start], provider, segment_count)?;
            output.push_str(&translated);
            output.push_str(&text[destination_start..destination_end]);
            cursor = destination_end;
        } else {
            break;
        }
    }

    if cursor < text.len() {
        let translated = translate_visible_text(&text[cursor..], provider, segment_count)?;
        output.push_str(&translated);
    }

    Ok(output)
}

fn reserve_segments(segment_count: &mut usize, additional: usize) -> Result<(), TranslateFailure> {
    let next = segment_count.checked_add(additional).ok_or_else(|| {
        TranslateFailure::new(
            ErrorCode::FileTooLarge,
            "The request contains too many translatable segments.",
        )
    })?;
    if next > MAX_SEGMENTS {
        return Err(TranslateFailure::new(
            ErrorCode::FileTooLarge,
            "The request contains too many translatable segments.",
        ));
    }

    *segment_count = next;
    Ok(())
}

fn no_translatable_segments() -> TranslateFailure {
    TranslateFailure::new(
        ErrorCode::NoTranslatableSegments,
        "No translatable segments were found.",
    )
}

fn opening_fence_marker(line: &str) -> Option<String> {
    let first = line.as_bytes().first().copied()?;
    if first != b'`' && first != b'~' {
        return None;
    }
    let count = line.bytes().take_while(|byte| *byte == first).count();
    if count >= 3 {
        Some((first as char).to_string().repeat(count))
    } else {
        None
    }
}

fn starts_with_fence_marker(line: &str, marker: &str) -> bool {
    line.starts_with(marker)
}

#[derive(Clone, Copy)]
enum HtmlBlockEnd {
    ClosingTag,
    ClosingTagOrBlankLine,
}

#[derive(Clone, Copy)]
struct HtmlBlock {
    expected_close_tag: &'static str,
    end: HtmlBlockEnd,
}

impl HtmlBlock {
    fn closes_with_tag(self, line: &str) -> bool {
        line.to_ascii_lowercase().contains(self.expected_close_tag)
    }

    const fn closes_on_blank_line(self) -> bool {
        matches!(self.end, HtmlBlockEnd::ClosingTagOrBlankLine)
    }
}

fn html_block(line: &str) -> Option<HtmlBlock> {
    let lower = line.to_ascii_lowercase();
    if lower.starts_with("<div") {
        Some(HtmlBlock {
            expected_close_tag: "</div>",
            end: HtmlBlockEnd::ClosingTagOrBlankLine,
        })
    } else if lower.starts_with("<table") {
        Some(HtmlBlock {
            expected_close_tag: "</table>",
            end: HtmlBlockEnd::ClosingTagOrBlankLine,
        })
    } else if lower.starts_with("<pre") {
        Some(HtmlBlock {
            expected_close_tag: "</pre>",
            end: HtmlBlockEnd::ClosingTag,
        })
    } else if lower.starts_with("<script") {
        Some(HtmlBlock {
            expected_close_tag: "</script>",
            end: HtmlBlockEnd::ClosingTag,
        })
    } else if lower.starts_with("<style") {
        Some(HtmlBlock {
            expected_close_tag: "</style>",
            end: HtmlBlockEnd::ClosingTag,
        })
    } else {
        None
    }
}

fn is_self_closing_html_block(line: &str) -> bool {
    line.trim_end().ends_with("/>")
}

fn count_backticks(text: &str) -> usize {
    text.bytes().take_while(|byte| *byte == b'`').count()
}

fn find_closing_backticks(line: &str, start: usize, count: usize) -> Option<usize> {
    let marker = "`".repeat(count);
    line[start..].find(&marker).map(|relative| start + relative)
}

fn split_translatable_text(text: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let mut end = (start + MAX_SEGMENT_BYTES).min(text.len());
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        segments.push(text[start..end].to_string());
        start = end;
    }
    segments
}

fn find_link_destination_end(text: &str, destination_start: usize) -> Option<usize> {
    let mut depth = 1_usize;
    for (relative, ch) in text[destination_start..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(destination_start + relative + ch.len_utf8());
                }
            }
            _ => {}
        }
    }
    None
}
