use crate::{
    ensure_provider_response_shape, redact_failure, ErrorCode, InputKind, Language, Provider,
    ProviderRequest, Tone, TranslateFailure,
};

pub fn translate_document(
    content: &str,
    input_kind: InputKind,
    provider: &impl Provider,
) -> Result<String, TranslateFailure> {
    let translated = match input_kind {
        InputKind::Text => translate_visible_text(content, provider),
        InputKind::Markdown => translate_markdown(content, provider),
    }?;

    if translated.segment_count == 0 {
        return Err(no_translatable_segments());
    }

    Ok(translated.text)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TranslatedDocument {
    text: String,
    segment_count: usize,
}

fn translate_markdown(
    content: &str,
    provider: &impl Provider,
) -> Result<TranslatedDocument, TranslateFailure> {
    let mut output = String::new();
    let mut segment_count = 0;
    let mut in_fence: Option<String> = None;
    let mut in_frontmatter = false;
    let mut frontmatter_checked = false;
    let mut in_html_block = false;

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

        if in_html_block {
            output.push_str(line);
            if closes_html_block(start) {
                in_html_block = false;
            }
            continue;
        }

        if opens_html_block(start) {
            output.push_str(line);
            if !closes_html_block(start) {
                in_html_block = true;
            }
            continue;
        }

        let translated = translate_inline_visible(line, provider)?;
        output.push_str(&translated.text);
        segment_count += translated.segment_count;
    }

    Ok(TranslatedDocument {
        text: output,
        segment_count,
    })
}

fn translate_inline_visible(
    line: &str,
    provider: &impl Provider,
) -> Result<TranslatedDocument, TranslateFailure> {
    let mut output = String::new();
    let mut segment_count = 0;
    let mut cursor = 0;

    while let Some(relative_start) = line[cursor..].find('`') {
        let start = cursor + relative_start;
        if start > cursor {
            let translated = translate_visible_markdown_text(&line[cursor..start], provider)?;
            output.push_str(&translated.text);
            segment_count += translated.segment_count;
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
                return Ok(TranslatedDocument {
                    text: output,
                    segment_count,
                });
            }
        }
    }

    if cursor < line.len() {
        let translated = translate_visible_markdown_text(&line[cursor..], provider)?;
        output.push_str(&translated.text);
        segment_count += translated.segment_count;
    }

    Ok(TranslatedDocument {
        text: output,
        segment_count,
    })
}

fn translate_visible_text(
    text: &str,
    provider: &impl Provider,
) -> Result<TranslatedDocument, TranslateFailure> {
    if text.trim().is_empty() {
        return Ok(TranslatedDocument {
            text: text.to_string(),
            segment_count: 0,
        });
    }

    let request = ProviderRequest::new(
        vec![text.to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )?;
    let response = provider.translate(&request).map_err(redact_failure)?;
    ensure_provider_response_shape(&request, &response)?;

    let translated = response
        .translated_segments
        .into_iter()
        .next()
        .ok_or_else(|| {
            TranslateFailure::new(
                ErrorCode::ProviderFailed,
                "Provider returned no translated segments.",
            )
        })?;

    Ok(TranslatedDocument {
        text: translated,
        segment_count: 1,
    })
}

fn translate_visible_markdown_text(
    text: &str,
    provider: &impl Provider,
) -> Result<TranslatedDocument, TranslateFailure> {
    let mut output = String::new();
    let mut segment_count = 0;
    let mut cursor = 0;

    while let Some(relative_start) = text[cursor..].find("](") {
        let marker_start = cursor + relative_start;
        let destination_start = marker_start + 2;
        if let Some(relative_end) = text[destination_start..].find(')') {
            let destination_end = destination_start + relative_end + 1;
            let translated = translate_visible_text(&text[cursor..destination_start], provider)?;
            output.push_str(&translated.text);
            segment_count += translated.segment_count;
            output.push_str(&text[destination_start..destination_end]);
            cursor = destination_end;
        } else {
            break;
        }
    }

    if cursor < text.len() {
        let translated = translate_visible_text(&text[cursor..], provider)?;
        output.push_str(&translated.text);
        segment_count += translated.segment_count;
    }

    Ok(TranslatedDocument {
        text: output,
        segment_count,
    })
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

fn opens_html_block(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    lower.starts_with("<div")
        || lower.starts_with("<table")
        || lower.starts_with("<pre")
        || lower.starts_with("<script")
        || lower.starts_with("<style")
}

fn closes_html_block(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    lower.contains("</div>")
        || lower.contains("</table>")
        || lower.contains("</pre>")
        || lower.contains("</script>")
        || lower.contains("</style>")
}

fn count_backticks(text: &str) -> usize {
    text.bytes().take_while(|byte| *byte == b'`').count()
}

fn find_closing_backticks(line: &str, start: usize, count: usize) -> Option<usize> {
    let marker = "`".repeat(count);
    line[start..].find(&marker).map(|relative| start + relative)
}
