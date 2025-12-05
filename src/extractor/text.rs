//! Text node extraction

use crate::models::document::{Node, NodeData};
use crate::models::extraction::{ExtractedText, HierarchyPath, TextNodeType, TextStyleInfo};
use crate::service::traversal::NodeVisitor;

#[derive(Default)]
pub struct TextExtractor {
    texts: Vec<ExtractedText>,
    sequence_number: usize,
}

impl TextExtractor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_texts(self) -> Vec<ExtractedText> {
        self.texts
    }

    pub fn count(&self) -> usize {
        self.texts.len()
    }
}

impl NodeVisitor for TextExtractor {
    fn visit_node(&mut self, node: &Node, _depth: usize, path: &[String]) {
        let (node_type, characters, style) = match &node.data {
            NodeData::Text {
                characters, style, ..
            } => (TextNodeType::Text, characters.as_str(), style.as_ref()),
            NodeData::Sticky { characters, .. } => {
                (TextNodeType::Sticky, characters.as_str(), None)
            }
            NodeData::Other {
                characters: Some(chars),
                ..
            } if !chars.trim().is_empty() => (TextNodeType::Text, chars.as_str(), None),
            _ => return,
        };

        if characters.trim().is_empty() {
            return;
        }

        let style_info = style.map(|s| TextStyleInfo {
            font_family: s
                .font_family
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            font_size: s.font_size.unwrap_or(16.0),
            font_weight: s.font_weight.unwrap_or(400),
        });

        self.texts.push(ExtractedText {
            node_id: node.id().to_string(),
            node_type,
            text: characters.to_string(),
            path: build_hierarchy_path(path),
            sequence_number: self.sequence_number,
            style: style_info,
        });
        self.sequence_number += 1;
    }
}

fn build_hierarchy_path(path: &[String]) -> HierarchyPath {
    let mut iter = path.iter().skip(1);

    let page_name = iter
        .next()
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());

    let mut section_name = None;
    let mut frame_names = Vec::new();

    for name in iter {
        if section_name.is_none() && is_section_name(name) {
            section_name = Some(name.clone());
        } else {
            frame_names.push(name.clone());
        }
    }

    HierarchyPath {
        page_name,
        section_name,
        frame_names,
        group_names: None,
    }
}

fn is_section_name(name: &str) -> bool {
    name.to_lowercase().contains("section") || name.contains(" > ") || name.starts_with("> ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::document::{NodeBase, TypeStyle};

    fn create_text_node(id: &str, text: &str) -> Node {
        Node {
            base: NodeBase {
                node_type: "TEXT".to_string(),
                id: id.to_string(),
                name: "Text".to_string(),
                visible: true,
                locked: false,
            },
            data: NodeData::Text {
                characters: text.to_string(),
                absolute_bounding_box: None,
                style: Some(TypeStyle {
                    font_family: Some("Inter".to_string()),
                    font_size: Some(16.0),
                    font_weight: Some(400),
                }),
            },
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
        assert_eq!(texts[0].node_type, TextNodeType::Text);
        assert_eq!(texts[0].text, "Hello, World!");
        assert_eq!(texts[0].path.page_name, "Page 1");
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
            "Section > Features".to_string(),
            "Frame 1".to_string(),
        ];

        let sticky = Node {
            base: NodeBase {
                node_type: "STICKY".to_string(),
                id: "2:1".to_string(),
                name: "Note".to_string(),
                visible: true,
                locked: false,
            },
            data: NodeData::Sticky {
                characters: "TODO: Review this implementation".to_string(),
                absolute_bounding_box: None,
                fills: vec![],
            },
        };

        extractor.visit_node(&sticky, 2, &path);

        let texts = extractor.into_texts();
        assert_eq!(texts.len(), 1);
        assert_eq!(texts[0].node_type, TextNodeType::Sticky);
        assert_eq!(texts[0].text, "TODO: Review this implementation");
        assert_eq!(
            texts[0].path.section_name,
            Some("Section > Features".to_string())
        );
        assert!(texts[0].style.is_none());
    }
}
