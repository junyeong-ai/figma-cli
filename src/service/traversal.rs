//! Document tree traversal

use crate::models::document::{Document, Node, NodeData};

pub trait NodeVisitor {
    fn visit_node(&mut self, node: &Node, depth: usize, path: &[String]);
}

pub fn traverse_document<V: NodeVisitor>(document: &Document, visitor: &mut V) {
    let mut path = vec![document.name.clone()];

    for child in &document.children {
        traverse_node(child, visitor, 1, &mut path);
    }
}

pub fn traverse_pages<V: NodeVisitor>(document: &Document, page_ids: &[String], visitor: &mut V) {
    let mut path = vec![document.name.clone()];

    for child in &document.children {
        if matches!(&child.data, NodeData::Canvas { .. })
            && page_ids.contains(&child.id().to_string())
        {
            traverse_node(child, visitor, 1, &mut path);
        }
    }
}

fn traverse_node<V: NodeVisitor>(
    node: &Node,
    visitor: &mut V,
    depth: usize,
    path: &mut Vec<String>,
) {
    visitor.visit_node(node, depth, path);

    if let Some(children) = node.children()
        && !children.is_empty()
    {
        path.push(node.name().to_string());
        for child in children {
            traverse_node(child, visitor, depth + 1, path);
        }
        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::document::{Color, NodeBase};

    struct CountingVisitor {
        count: usize,
    }

    impl NodeVisitor for CountingVisitor {
        fn visit_node(&mut self, _node: &Node, _depth: usize, _path: &[String]) {
            self.count += 1;
        }
    }

    #[test]
    fn test_traverse_empty_document() {
        let doc = Document {
            id: "0:0".to_string(),
            name: "Test Doc".to_string(),
            node_type: "DOCUMENT".to_string(),
            scroll_behavior: None,
            children: vec![],
        };

        let mut visitor = CountingVisitor { count: 0 };
        traverse_document(&doc, &mut visitor);

        assert_eq!(visitor.count, 0);
    }

    #[test]
    fn test_traverse_with_children() {
        let doc = Document {
            id: "0:0".to_string(),
            name: "Test Doc".to_string(),
            node_type: "DOCUMENT".to_string(),
            scroll_behavior: None,
            children: vec![Node {
                base: NodeBase {
                    node_type: "CANVAS".to_string(),
                    id: "0:1".to_string(),
                    name: "Page 1".to_string(),
                    visible: true,
                    locked: false,
                },
                data: NodeData::Canvas {
                    background_color: Some(Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    export_settings: vec![],
                    children: vec![Node {
                        base: NodeBase {
                            node_type: "FRAME".to_string(),
                            id: "0:2".to_string(),
                            name: "Frame 1".to_string(),
                            visible: true,
                            locked: false,
                        },
                        data: NodeData::Frame {
                            absolute_bounding_box: None,
                            fills: vec![],
                            clips_content: false,
                            children: vec![],
                        },
                    }],
                },
            }],
        };

        let mut visitor = CountingVisitor { count: 0 };
        traverse_document(&doc, &mut visitor);

        assert_eq!(visitor.count, 2);
    }
}
