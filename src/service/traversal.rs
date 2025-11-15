//! Document tree traversal

use crate::models::document::{Document, Node};

/// Visitor pattern for traversing document nodes
pub trait NodeVisitor {
    fn visit_node(&mut self, node: &Node, depth: usize, path: &[String]);
}

/// Traverse the document tree depth-first
pub fn traverse_document<V: NodeVisitor>(document: &Document, visitor: &mut V) {
    let mut path = vec![document.name.clone()];

    for child in &document.children {
        traverse_node(child, visitor, 1, &mut path);
    }
}

fn traverse_node<V: NodeVisitor>(
    node: &Node,
    visitor: &mut V,
    depth: usize,
    path: &mut Vec<String>,
) {
    // Visit current node
    visitor.visit_node(node, depth, path);

    // Get children based on node type
    let children = get_children(node);

    if !children.is_empty() {
        // Add current node name to path
        path.push(get_node_name(node).to_string());

        // Visit children
        for child in children {
            traverse_node(child, visitor, depth + 1, path);
        }

        // Remove current node from path
        path.pop();
    }
}

fn get_children(node: &Node) -> &[Node] {
    match node {
        Node::Canvas { children, .. }
        | Node::Section { children, .. }
        | Node::Frame { children, .. }
        | Node::Group { children, .. }
        | Node::Component { children, .. }
        | Node::ComponentSet { children, .. }
        | Node::Instance { children, .. }
        | Node::BooleanOperation { children, .. }
        | Node::Table { children, .. }
        | Node::TableCell { children, .. } => children,
        _ => &[],
    }
}

fn get_node_name(node: &Node) -> &str {
    node.name()
}

#[cfg(test)]
mod tests {
    use super::*;

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
            children: vec![Node::Canvas {
                node_type: "CANVAS".to_string(),
                id: "0:1".to_string(),
                name: "Page 1".to_string(),
                visible: true,
                locked: false,
                background_color: Some(crate::models::document::Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                }),
                export_settings: vec![],
                children: vec![Node::Frame {
                    node_type: "FRAME".to_string(),
                    id: "0:2".to_string(),
                    name: "Frame 1".to_string(),
                    visible: true,
                    locked: false,
                    absolute_bounding_box: None,
                    fills: vec![],
                    clips_content: false,
                    bound_variables: None,
                    children: vec![],
                }],
            }],
        };

        let mut visitor = CountingVisitor { count: 0 };
        traverse_document(&doc, &mut visitor);

        assert_eq!(visitor.count, 2);
    }
}
