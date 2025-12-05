//! Extraction output structures

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::document::EditorType;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractionResult {
    pub metadata: FileMetadata,
    pub structure: DocumentStructure,
    pub texts: Vec<ExtractedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<DesignElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageReference>>,
    pub stats: ExtractionStats,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    pub file_key: String,
    pub file_name: String,
    pub version: String,
    pub last_modified: DateTime<Utc>,
    pub extracted_at: DateTime<Utc>,
    pub editor_type: EditorType,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentStructure {
    pub pages: Vec<PageInfo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub id: String,
    pub name: String,
    pub frame_count: usize,
    pub text_node_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TextNodeType {
    Text,
    Sticky,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractedText {
    pub node_id: String,
    pub node_type: TextNodeType,
    pub text: String,
    pub path: HierarchyPath,
    pub sequence_number: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<TextStyleInfo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HierarchyPath {
    pub page_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_name: Option<String>,
    pub frame_names: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStyleInfo {
    pub font_family: String,
    pub font_size: f64,
    pub font_weight: u16,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignElement {
    pub id: String,
    pub name: String,
    pub element_type: String,
    pub path: HierarchyPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
    pub child_count: usize,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageReference {
    pub node_id: String,
    pub node_name: String,
    pub path: HierarchyPath,
    pub image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractionStats {
    pub total_pages: usize,
    pub total_frames: usize,
    pub total_text_nodes: usize,
    pub total_characters: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_images: Option<usize>,
    pub extraction_time_ms: u64,
    pub memory_size_mb: f64,
}

impl ExtractionResult {
    pub fn new(
        metadata: FileMetadata,
        structure: DocumentStructure,
        texts: Vec<ExtractedText>,
    ) -> Self {
        let total_characters: usize = texts.iter().map(|t| t.text.len()).sum();
        let stats = ExtractionStats {
            total_pages: structure.pages.len(),
            total_frames: structure.pages.iter().map(|p| p.frame_count).sum(),
            total_text_nodes: texts.len(),
            total_characters,
            total_images: None,
            extraction_time_ms: 0,
            memory_size_mb: 0.0,
        };

        Self {
            metadata,
            structure,
            texts,
            elements: None,
            images: None,
            stats,
        }
    }

    pub const fn with_stats(mut self, stats: ExtractionStats) -> Self {
        self.stats = stats;
        self
    }

    pub fn with_elements(mut self, elements: Vec<DesignElement>) -> Self {
        self.elements = Some(elements);
        self
    }

    pub fn with_images(mut self, images: Vec<ImageReference>) -> Self {
        self.images = Some(images);
        self
    }
}

impl HierarchyPath {
    pub const fn new(page_name: String, frame_names: Vec<String>) -> Self {
        Self {
            page_name,
            section_name: None,
            frame_names,
            group_names: None,
        }
    }

    pub fn with_section(mut self, section: String) -> Self {
        self.section_name = Some(section);
        self
    }

    pub fn with_groups(mut self, groups: Vec<String>) -> Self {
        self.group_names = Some(groups);
        self
    }

    pub fn to_path_string(&self) -> String {
        let mut parts = vec![self.page_name.clone()];
        if let Some(section) = &self.section_name {
            parts.push(section.clone());
        }
        parts.extend(self.frame_names.iter().cloned());
        if let Some(groups) = &self.group_names {
            parts.extend(groups.iter().cloned());
        }
        parts.join(" > ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchy_path_to_string() {
        let path = HierarchyPath::new(
            "Design System".to_string(),
            vec!["Components".to_string(), "Buttons".to_string()],
        );

        assert_eq!(
            path.to_path_string(),
            "Design System > Components > Buttons"
        );
    }

    #[test]
    fn test_hierarchy_path_with_groups() {
        let path = HierarchyPath::new("Page1".to_string(), vec!["Frame1".to_string()])
            .with_groups(vec!["Group1".to_string(), "Group2".to_string()]);

        assert_eq!(path.to_path_string(), "Page1 > Frame1 > Group1 > Group2");
    }

    #[test]
    fn test_extraction_result_builder() {
        let metadata = FileMetadata {
            file_key: "ABC123".to_string(),
            file_name: "Test File".to_string(),
            version: "1".to_string(),
            last_modified: chrono::Utc::now(),
            extracted_at: chrono::Utc::now(),
            editor_type: crate::models::document::EditorType::Figma,
        };

        let structure = DocumentStructure { pages: vec![] };

        let texts = vec![ExtractedText {
            node_id: "1:1".to_string(),
            node_type: TextNodeType::Text,
            text: "Test".to_string(),
            path: HierarchyPath::new("Page1".to_string(), vec![]),
            sequence_number: 0,
            style: None,
        }];

        let result = ExtractionResult::new(metadata, structure, texts);

        assert_eq!(result.metadata.file_key, "ABC123");
        assert_eq!(result.texts.len(), 1);
        assert_eq!(result.stats.total_text_nodes, 1);
        assert_eq!(result.stats.total_characters, 4);
    }

    #[test]
    fn test_extraction_result_with_stats() {
        let metadata = FileMetadata {
            file_key: "ABC123".to_string(),
            file_name: "Test File".to_string(),
            version: "1".to_string(),
            last_modified: chrono::Utc::now(),
            extracted_at: chrono::Utc::now(),
            editor_type: crate::models::document::EditorType::Figma,
        };

        let structure = DocumentStructure { pages: vec![] };
        let texts = vec![];

        let custom_stats = ExtractionStats {
            total_pages: 5,
            total_frames: 10,
            total_text_nodes: 100,
            total_characters: 1000,
            total_images: Some(20),
            extraction_time_ms: 500,
            memory_size_mb: 2.5,
        };

        let result = ExtractionResult::new(metadata, structure, texts).with_stats(custom_stats);

        assert_eq!(result.stats.total_pages, 5);
        assert_eq!(result.stats.extraction_time_ms, 500);
        assert!((result.stats.memory_size_mb - 2.5).abs() < f64::EPSILON);
    }
}
