//! Summary format builder for AI-optimized output

use std::collections::{BTreeMap, HashSet};
use std::fmt::Write;

use crate::models::extraction::{ExtractionResult, TextNodeType};

const MIN_DESCRIPTION_LENGTH: usize = 50;

#[derive(Default)]
struct SectionContent {
    notes: Vec<String>,
    descriptions: Vec<String>,
    labels: Vec<String>,
}

pub fn format_summary(result: &ExtractionResult) -> String {
    let mut seen = HashSet::new();
    let mut pages: BTreeMap<String, BTreeMap<String, SectionContent>> = BTreeMap::new();

    for text in &result.texts {
        let normalized = text.text.trim().to_lowercase();
        if !seen.insert(normalized) {
            continue;
        }

        let page = &text.path.page_name;
        let section = text
            .path
            .section_name
            .clone()
            .unwrap_or_else(|| "General".to_string());

        let content = pages
            .entry(page.clone())
            .or_default()
            .entry(section)
            .or_default();

        let trimmed = text.text.trim().to_string();
        match text.node_type {
            TextNodeType::Sticky => content.notes.push(trimmed),
            TextNodeType::Text => {
                if is_description(&trimmed) {
                    content.descriptions.push(trimmed);
                } else {
                    content.labels.push(trimmed);
                }
            }
        }
    }

    build_output(&result.metadata.file_name, &pages)
}

fn is_description(text: &str) -> bool {
    text.len() > MIN_DESCRIPTION_LENGTH || text.contains('\n')
}

fn build_output(
    file_name: &str,
    pages: &BTreeMap<String, BTreeMap<String, SectionContent>>,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# {file_name}\n");

    for (page, sections) in pages {
        let _ = writeln!(out, "## {page}\n");

        for (section, content) in sections {
            let _ = writeln!(out, "### {section}\n");

            if !content.notes.is_empty() {
                let _ = writeln!(out, "#### Notes\n");
                for note in &content.notes {
                    let _ = writeln!(out, "> {note}\n");
                }
            }

            if !content.descriptions.is_empty() {
                let _ = writeln!(out, "#### Descriptions\n");
                for desc in &content.descriptions {
                    let _ = writeln!(out, "- {desc}");
                }
                out.push('\n');
            }

            if !content.labels.is_empty() {
                let _ = writeln!(out, "#### UI Labels\n");
                for label in &content.labels {
                    let _ = writeln!(out, "- {label}");
                }
                out.push('\n');
            }

            let _ = writeln!(out, "---\n");
        }
    }

    out.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::document::EditorType;
    use crate::models::extraction::{
        DocumentStructure, ExtractedText, ExtractionStats, FileMetadata, HierarchyPath,
    };
    use chrono::Utc;

    fn create_test_result(texts: Vec<ExtractedText>) -> ExtractionResult {
        ExtractionResult {
            metadata: FileMetadata {
                file_key: "test".to_string(),
                file_name: "Test File".to_string(),
                version: "1".to_string(),
                last_modified: Utc::now(),
                extracted_at: Utc::now(),
                editor_type: EditorType::Figma,
            },
            structure: DocumentStructure { pages: vec![] },
            texts,
            elements: None,
            images: None,
            stats: ExtractionStats {
                total_pages: 0,
                total_frames: 0,
                total_text_nodes: 0,
                total_characters: 0,
                total_images: None,
                extraction_time_ms: 0,
                memory_size_mb: 0.0,
            },
        }
    }

    #[test]
    fn test_format_summary_basic() {
        let texts = vec![
            ExtractedText {
                node_id: "1".to_string(),
                node_type: TextNodeType::Text,
                text: "Submit".to_string(),
                path: HierarchyPath {
                    page_name: "Page 1".to_string(),
                    section_name: Some("Section A".to_string()),
                    frame_names: vec![],
                    group_names: None,
                },
                sequence_number: 0,
                style: None,
            },
            ExtractedText {
                node_id: "2".to_string(),
                node_type: TextNodeType::Sticky,
                text: "TODO: Add validation".to_string(),
                path: HierarchyPath {
                    page_name: "Page 1".to_string(),
                    section_name: Some("Section A".to_string()),
                    frame_names: vec![],
                    group_names: None,
                },
                sequence_number: 1,
                style: None,
            },
        ];

        let result = create_test_result(texts);
        let output = format_summary(&result);

        assert!(output.contains("# Test File"));
        assert!(output.contains("## Page 1"));
        assert!(output.contains("### Section A"));
        assert!(output.contains("#### Notes"));
        assert!(output.contains("TODO: Add validation"));
        assert!(output.contains("#### UI Labels"));
        assert!(output.contains("Submit"));
    }

    #[test]
    fn test_deduplication() {
        let texts = vec![
            ExtractedText {
                node_id: "1".to_string(),
                node_type: TextNodeType::Text,
                text: "Duplicate".to_string(),
                path: HierarchyPath::new("Page".to_string(), vec![]),
                sequence_number: 0,
                style: None,
            },
            ExtractedText {
                node_id: "2".to_string(),
                node_type: TextNodeType::Text,
                text: "Duplicate".to_string(),
                path: HierarchyPath::new("Page".to_string(), vec![]),
                sequence_number: 1,
                style: None,
            },
        ];

        let result = create_test_result(texts);
        let output = format_summary(&result);

        assert_eq!(output.matches("Duplicate").count(), 1);
    }
}
