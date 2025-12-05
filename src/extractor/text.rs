//! Text node extraction

use crate::models::document::Node;
use crate::models::extraction::{ExtractedText, HierarchyPath};
use crate::service::traversal::NodeVisitor;

/// Extracts text content from document nodes
pub struct TextExtractor {
    texts: Vec<ExtractedText>,
    sequence_number: usize,
}

impl TextExtractor {
    pub const fn new() -> Self {
        Self {
            texts: Vec::new(),
            sequence_number: 0,
        }
    }

    pub fn into_texts(self) -> Vec<ExtractedText> {
        self.texts
    }

    pub const fn count(&self) -> usize {
        self.texts.len()
    }
}

impl Default for TextExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeVisitor for TextExtractor {
    fn visit_node(&mut self, node: &Node, _depth: usize, path: &[String]) {
        let (id, characters, style) = match node {
            Node::Text {
                id,
                characters,
                style,
                ..
            } => (id, characters, style.as_ref()),
            Node::Sticky { id, characters, .. } => (id, characters, None),
            _ => return,
        };

        if characters.trim().is_empty() {
            return;
        }

        let style_info = style.map(|s| crate::models::extraction::TextStyleInfo {
            font_family: s
                .font_family
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            font_size: s.font_size.unwrap_or(16.0),
            font_weight: s.font_weight.unwrap_or(400),
        });

        self.texts.push(ExtractedText {
            node_id: id.clone(),
            text: characters.clone(),
            path: build_hierarchy_path(path),
            sequence_number: self.sequence_number,
            style: style_info,
        });
        self.sequence_number += 1;
    }
}

fn build_hierarchy_path(path: &[String]) -> HierarchyPath {
    // Path structure: [Document, Canvas, Frame, ...]
    // Canvas = Page, subsequent items before Text node = Frame hierarchy

    let mut page_name = "Unknown Page".to_string();
    let mut frame_names = Vec::new();
    let mut group_names = Vec::new();

    let mut iter = path.iter().skip(1); // Skip document name

    // First item after document is the Canvas (Page)
    if let Some(canvas) = iter.next() {
        page_name = canvas.clone();
    }

    // Remaining items are frames and groups
    for item in iter {
        // Simple heuristic: if it contains "Frame" or starts with uppercase, it's likely a frame
        // Otherwise, it might be a group
        if item.contains("Frame") || item.chars().next().is_some_and(char::is_uppercase) {
            frame_names.push(item.clone());
        } else {
            group_names.push(item.clone());
        }
    }

    HierarchyPath {
        page_name,
        frame_names,
        group_names: if group_names.is_empty() {
            None
        } else {
            Some(group_names)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::document::TypeStyle;

    fn create_text_node(id: &str, text: &str) -> Node {
        Node::Text {
            node_type: "TEXT".to_string(),
            id: id.to_string(),
            name: "Text".to_string(),
            visible: true,
            locked: false,
            characters: text.to_string(),
            absolute_bounding_box: None,
            style: Some(TypeStyle {
                font_family: Some("Inter".to_string()),
                font_size: Some(16.0),
                font_weight: Some(400),
            }),
        }
    }

    #[test]
    fn test_text_extraction() {
        let mut extractor = TextExtractor::new();
        let path = vec![
            "Document".to_string(),
            "Page 1".to_string(),
            "Frame 1".to_string(),
        ];

        let node = create_text_node("1:1", "Hello, World!");
        extractor.visit_node(&node, 2, &path);

        let texts = extractor.into_texts();
        assert_eq!(texts.len(), 1);
        assert_eq!(texts[0].text, "Hello, World!");
        assert_eq!(texts[0].path.page_name, "Page 1");
        assert!(texts[0].path.frame_names.contains(&"Frame 1".to_string()));
    }

    #[test]
    fn test_skip_empty_text() {
        let mut extractor = TextExtractor::new();
        let path = vec!["Document".to_string()];

        let node = create_text_node("1:1", "   ");
        extractor.visit_node(&node, 1, &path);

        assert_eq!(extractor.count(), 0);
    }

    #[test]
    fn test_sequence_numbers() {
        let mut extractor = TextExtractor::new();
        let path = vec!["Document".to_string()];

        extractor.visit_node(&create_text_node("1:1", "First"), 1, &path);
        extractor.visit_node(&create_text_node("1:2", "Second"), 1, &path);
        extractor.visit_node(&create_text_node("1:3", "Third"), 1, &path);

        let texts = extractor.into_texts();
        assert_eq!(texts[0].sequence_number, 0);
        assert_eq!(texts[1].sequence_number, 1);
        assert_eq!(texts[2].sequence_number, 2);
    }

    #[test]
    fn test_sticky_extraction() {
        let mut extractor = TextExtractor::new();
        let path = vec![
            "Document".to_string(),
            "Page 1".to_string(),
            "Section 1".to_string(),
        ];

        let sticky = Node::Sticky {
            node_type: "STICKY".to_string(),
            id: "2:1".to_string(),
            name: "Planning Note".to_string(),
            visible: true,
            locked: false,
            characters: "개발자 확인 필요: 성능 이슈 체크".to_string(),
            absolute_bounding_box: None,
            fills: vec![],
        };

        extractor.visit_node(&sticky, 2, &path);

        let texts = extractor.into_texts();
        assert_eq!(texts.len(), 1);
        assert_eq!(texts[0].text, "개발자 확인 필요: 성능 이슈 체크");
        assert!(texts[0].style.is_none());
    }
}
