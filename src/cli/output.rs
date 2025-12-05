use crate::cli::args::OutputFormat;
use crate::models::extraction::ExtractionResult;
use anyhow::Result;
use std::fmt::Write as _;
use std::fs;

pub fn format_output(
    result: &ExtractionResult,
    format: OutputFormat,
    output_path: Option<&str>,
    pretty: bool,
) -> Result<()> {
    let formatted = match format {
        OutputFormat::Json => format_json(result, pretty)?,
        OutputFormat::Text => format_text(result),
        OutputFormat::Markdown => format_markdown(result),
        OutputFormat::Summary => super::summary::format_summary(result),
    };

    if let Some(path) = output_path {
        fs::write(path, formatted)?;
        tracing::info!("Output written to: {}", path);
    } else {
        println!("{formatted}");
    }

    Ok(())
}

fn format_json(result: &ExtractionResult, pretty: bool) -> Result<String> {
    if pretty {
        Ok(serde_json::to_string_pretty(result)?)
    } else {
        Ok(serde_json::to_string(result)?)
    }
}

fn format_text(result: &ExtractionResult) -> String {
    let mut output = String::new();

    let _ = writeln!(output, "File: {}", result.metadata.file_name);
    let _ = writeln!(output, "Version: {}", result.metadata.version);
    let _ = write!(output, "Extracted: {}\n\n", result.metadata.extracted_at);

    output.push_str("Statistics:\n");
    let _ = writeln!(output, "  Pages: {}", result.stats.total_pages);
    let _ = writeln!(output, "  Frames: {}", result.stats.total_frames);
    let _ = writeln!(output, "  Text nodes: {}", result.stats.total_text_nodes);
    let _ = writeln!(output, "  Characters: {}", result.stats.total_characters);
    let _ = writeln!(
        output,
        "  Extraction time: {}ms",
        result.stats.extraction_time_ms
    );
    let _ = write!(output, "  Memory: {:.2}MB\n\n", result.stats.memory_size_mb);

    output.push_str("Text Content:\n");
    output.push_str(&"=".repeat(80));
    output.push_str("\n\n");

    for text in &result.texts {
        let _ = writeln!(output, "Path: {}", text.path.to_path_string());
        let _ = writeln!(output, "Node ID: {}", text.node_id);
        if let Some(style) = &text.style {
            let _ = writeln!(
                output,
                "Style: {} {}pt (weight: {})",
                style.font_family, style.font_size, style.font_weight
            );
        }
        let _ = write!(output, "\n{}\n\n", text.text);
        output.push_str(&"-".repeat(80));
        output.push_str("\n\n");
    }

    output
}

fn format_markdown(result: &ExtractionResult) -> String {
    let mut output = String::new();

    let _ = write!(output, "# {}\n\n", result.metadata.file_name);
    let _ = writeln!(output, "**Version:** {}", result.metadata.version);
    let _ = write!(
        output,
        "**Extracted:** {}\n\n",
        result.metadata.extracted_at
    );

    output.push_str("## Statistics\n\n");
    output.push_str("| Metric | Value |\n");
    output.push_str("|--------|-------|\n");
    let _ = writeln!(output, "| Pages | {} |", result.stats.total_pages);
    let _ = writeln!(output, "| Frames | {} |", result.stats.total_frames);
    let _ = writeln!(output, "| Text nodes | {} |", result.stats.total_text_nodes);
    let _ = writeln!(output, "| Characters | {} |", result.stats.total_characters);
    let _ = writeln!(
        output,
        "| Extraction time | {}ms |",
        result.stats.extraction_time_ms
    );
    let _ = write!(
        output,
        "| Memory | {:.2}MB |\n\n",
        result.stats.memory_size_mb
    );

    output.push_str("## Document Structure\n\n");
    for page in &result.structure.pages {
        let _ = writeln!(
            output,
            "- **{}** ({} frames, {} text nodes)",
            page.name, page.frame_count, page.text_node_count
        );
    }
    output.push('\n');

    output.push_str("## Text Content\n\n");

    let mut current_page = String::new();
    for text in &result.texts {
        if text.path.page_name != current_page {
            current_page.clone_from(&text.path.page_name);
            let _ = write!(output, "### {current_page}\n\n");
        }

        let _ = writeln!(output, "**Path:** {}", text.path.to_path_string());
        if let Some(style) = &text.style {
            let _ = writeln!(output, "*{}pt {}*", style.font_size, style.font_family);
        }
        let _ = write!(output, "\n{}\n\n", text.text);
        output.push_str("---\n\n");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::extraction::*;
    use chrono::Utc;

    fn create_test_result() -> ExtractionResult {
        ExtractionResult {
            metadata: FileMetadata {
                file_key: "test123".to_string(),
                file_name: "Test File".to_string(),
                version: "1.0".to_string(),
                last_modified: Utc::now(),
                extracted_at: Utc::now(),
                editor_type: crate::models::document::EditorType::Figma,
            },
            structure: DocumentStructure {
                pages: vec![PageInfo {
                    id: "0:1".to_string(),
                    name: "Page 1".to_string(),
                    frame_count: 2,
                    text_node_count: 3,
                }],
            },
            texts: vec![ExtractedText {
                node_id: "1:1".to_string(),
                node_type: TextNodeType::Text,
                text: "Hello, World!".to_string(),
                path: HierarchyPath::new("Page 1".to_string(), vec!["Frame 1".to_string()]),
                sequence_number: 0,
                style: Some(TextStyleInfo {
                    font_family: "Inter".to_string(),
                    font_size: 16.0,
                    font_weight: 400,
                }),
            }],
            elements: None,
            images: None,
            stats: ExtractionStats {
                total_pages: 1,
                total_frames: 2,
                total_text_nodes: 1,
                total_characters: 13,
                total_images: None,
                extraction_time_ms: 100,
                memory_size_mb: 0.5,
            },
        }
    }

    #[test]
    fn test_format_json() {
        let result = create_test_result();
        let json = format_json(&result, false).unwrap();
        assert!(json.contains("Test File"));
        assert!(json.contains("Hello, World!"));
    }

    #[test]
    fn test_format_text() {
        let result = create_test_result();
        let text = format_text(&result);
        assert!(text.contains("Test File"));
        assert!(text.contains("Hello, World!"));
        assert!(text.contains("Statistics:"));
    }

    #[test]
    fn test_format_markdown() {
        let result = create_test_result();
        let md = format_markdown(&result);
        assert!(md.contains("# Test File"));
        assert!(md.contains("Hello, World!"));
        assert!(md.contains("## Statistics"));
    }
}
