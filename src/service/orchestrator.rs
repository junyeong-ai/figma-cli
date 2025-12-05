//! Main extraction orchestration

use crate::client::{FigmaClient, Result};
use crate::extractor::TextExtractor;
use crate::models::config::FilterCriteria;
use crate::models::document::FigmaFile;
use crate::models::extraction::{
    DocumentStructure, ExtractionResult, ExtractionStats, FileMetadata, PageInfo,
};
use crate::service::traversal::{traverse_document, traverse_pages};
use chrono::Utc;
use std::time::Instant;

/// Orchestrates the extraction process
pub struct Orchestrator {
    client: FigmaClient,
}

impl Orchestrator {
    pub const fn new(client: FigmaClient) -> Self {
        Self { client }
    }

    /// Extract content from a Figma file
    pub async fn extract(
        &self,
        file_key: &str,
        filter: FilterCriteria,
        depth: Option<u32>,
    ) -> Result<ExtractionResult> {
        let start_time = Instant::now();

        tracing::info!("Starting extraction for file: {}", file_key);

        // Fetch the file
        let file = self.client.get_file(file_key, depth).await?;

        tracing::info!(
            "File fetched: {} (version: {}, pages: {})",
            file.name,
            file.version,
            file.document.children.len()
        );

        // Filter and extract
        let (texts, structure) = self.extract_content(&file, &filter);

        // Calculate statistics
        let extraction_time_ms = start_time.elapsed().as_millis() as u64;
        let total_characters: usize = texts.iter().map(|t| t.text.len()).sum();

        let stats = ExtractionStats {
            total_pages: structure.pages.len(),
            total_frames: structure.pages.iter().map(|p| p.frame_count).sum(),
            total_text_nodes: texts.len(),
            total_characters,
            total_images: None,
            extraction_time_ms,
            memory_size_mb: estimate_memory_size(&texts),
        };

        // Build metadata
        let metadata = FileMetadata {
            file_key: file.file_key.clone(),
            file_name: file.name.clone(),
            version: file.version.clone(),
            last_modified: file.last_modified,
            extracted_at: Utc::now(),
            editor_type: file.editor_type,
        };

        tracing::info!(
            "Extraction complete: {} text nodes, {} characters in {}ms",
            stats.total_text_nodes,
            stats.total_characters,
            extraction_time_ms
        );

        Ok(ExtractionResult {
            metadata,
            structure,
            texts,
            elements: None,
            images: None,
            stats,
        })
    }

    fn extract_content(
        &self,
        file: &FigmaFile,
        filter: &FilterCriteria,
    ) -> (
        Vec<crate::models::extraction::ExtractedText>,
        DocumentStructure,
    ) {
        let mut pages = Vec::new();
        let mut filtered_page_ids = Vec::new();

        for child in &file.document.children {
            if let crate::models::document::Node::Canvas {
                id, name, children, ..
            } = child
            {
                if !filter.matches_page(name) || !filter.matches_page_id(id) {
                    continue;
                }

                filtered_page_ids.push(id.clone());
                pages.push(PageInfo {
                    id: id.clone(),
                    name: name.clone(),
                    frame_count: count_frames(children),
                    text_node_count: count_text_nodes(children),
                });
            }
        }

        let structure = DocumentStructure { pages };

        let mut text_extractor = TextExtractor::new();
        if filter.is_empty() {
            traverse_document(&file.document, &mut text_extractor);
        } else {
            traverse_pages(&file.document, &filtered_page_ids, &mut text_extractor);
        }

        (text_extractor.into_texts(), structure)
    }
}

fn count_frames(nodes: &[crate::models::document::Node]) -> usize {
    nodes
        .iter()
        .filter(|n| matches!(n, crate::models::document::Node::Frame { .. }))
        .count()
}

fn count_text_nodes(nodes: &[crate::models::document::Node]) -> usize {
    use crate::models::document::Node;

    nodes.iter().fold(0, |count, node| {
        count
            + match node {
                Node::Text { .. } | Node::Sticky { .. } => 1,
                Node::Frame { children, .. }
                | Node::Group { children, .. }
                | Node::Section { children, .. }
                | Node::Component { children, .. }
                | Node::ComponentSet { children, .. }
                | Node::Instance { children, .. } => count_text_nodes(children),
                _ => 0,
            }
    })
}

fn estimate_memory_size(texts: &[crate::models::extraction::ExtractedText]) -> f64 {
    let text_bytes: usize = texts.iter().map(|t| t.text.len()).sum();
    let overhead_per_item = 200; // Rough estimate for struct overhead
    let total_bytes = text_bytes + (texts.len() * overhead_per_item);
    (total_bytes as f64) / (1024.0 * 1024.0) // Convert to MB
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_estimation() {
        use crate::models::extraction::{ExtractedText, HierarchyPath, TextNodeType};

        let texts = vec![
            ExtractedText {
                node_id: "1:1".to_string(),
                node_type: TextNodeType::Text,
                text: "a".repeat(1000),
                path: HierarchyPath::new("Page".to_string(), vec![]),
                sequence_number: 0,
                style: None,
            };
            10
        ];

        let size_mb = estimate_memory_size(&texts);
        assert!(size_mb > 0.0);
        assert!(size_mb < 1.0);
    }
}
