//! Figma API document structures

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaFile {
    #[serde(skip_deserializing, default)]
    pub file_key: String,
    pub name: String,
    pub version: String,
    pub last_modified: DateTime<Utc>,
    pub document: Document,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    pub editor_type: EditorType,
    #[serde(default)]
    pub components: HashMap<String, Component>,
    #[serde(default)]
    pub styles: HashMap<String, Style>,
    #[serde(default)]
    pub component_sets: HashMap<String, ComponentSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_access: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EditorType {
    Figma,
    Figjam,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_behavior: Option<String>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Node {
    Canvas {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        background_color: Option<Color>,
        #[serde(rename = "exportSettings", default)]
        export_settings: Vec<ExportSetting>,
        #[serde(default)]
        children: Vec<Node>,
    },

    Section {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(
            rename = "absoluteRenderBounds",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_render_bounds: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,

        #[serde(default)]
        strokes: Vec<Paint>,

        #[serde(rename = "strokeWeight", default)]
        stroke_weight: f64,

        #[serde(rename = "strokeAlign", default)]
        stroke_align: String,

        #[serde(rename = "sectionContentsHidden", default)]
        section_contents_hidden: bool,

        #[serde(default)]
        children: Vec<Node>,
    },

    Frame {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,

        #[serde(rename = "clipsContent", default)]
        clips_content: bool,

        #[serde(rename = "boundVariables", default, skip_serializing)]
        bound_variables: Option<serde_json::Value>,

        #[serde(default)]
        children: Vec<Node>,
    },

    Group {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        children: Vec<Node>,
    },

    Text {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(default)]
        characters: String,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        style: Option<TypeStyle>,
    },

    Rectangle {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(rename = "cornerRadius", default)]
        corner_radius: f64,

        #[serde(default)]
        fills: Vec<Paint>,
    },

    Vector {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,
    },

    Component {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(rename = "componentKey", skip_serializing_if = "Option::is_none")]
        component_key: Option<String>,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        children: Vec<Node>,
    },

    ComponentSet {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(rename = "componentKey", skip_serializing_if = "Option::is_none")]
        component_key: Option<String>,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        children: Vec<Node>,
    },

    Instance {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(rename = "componentId", default)]
        component_id: String,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        children: Vec<Node>,
    },

    ShapeWithText {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,
    },

    Ellipse {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,
    },

    Line {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,
    },

    Polygon {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,
    },

    Star {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,
    },

    BooleanOperation {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,

        #[serde(default)]
        children: Vec<Node>,
    },

    Sticky {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,
    },

    Connector {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,
    },

    Widget {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,
    },

    Table {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,

        #[serde(default)]
        children: Vec<Node>,
    },

    TableCell {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,

        #[serde(
            rename = "absoluteBoundingBox",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        absolute_bounding_box: Option<BoundingBox>,

        #[serde(default)]
        fills: Vec<Paint>,

        #[serde(default)]
        children: Vec<Node>,
    },
}

const fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

mod option_struct {
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        value.serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: DeserializeOwned,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value.is_null() {
            return Ok(None);
        }
        Ok(Some(
            serde_json::from_value(value).map_err(serde::de::Error::custom)?,
        ))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Paint {
    #[serde(rename = "type", default)]
    pub paint_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_struct"
    )]
    pub color: Option<Color>,
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    #[serde(rename = "blendMode", default)]
    pub blend_mode: String,
    #[serde(rename = "boundVariables", default, skip_serializing)]
    pub bound_variables: Option<serde_json::Value>,
}

const fn default_opacity() -> f64 {
    1.0
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<u16>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportSetting {
    pub suffix: String,
    pub format: String,
    #[serde(default)]
    pub constraint: ExportConstraint,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExportConstraint {
    #[serde(default = "default_scale")]
    pub scale: f64,
}

const fn default_scale() -> f64 {
    1.0
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSet {
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub style_type: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeType {
    Document,
    Canvas,
    Section,
    Frame,
    Group,
    Text,
    Rectangle,
    Vector,
    Component,
    Instance,
}

impl Node {
    pub fn id(&self) -> &str {
        match self {
            Self::Canvas { id, .. }
            | Self::Section { id, .. }
            | Self::Frame { id, .. }
            | Self::Group { id, .. }
            | Self::Text { id, .. }
            | Self::Rectangle { id, .. }
            | Self::Vector { id, .. }
            | Self::Component { id, .. }
            | Self::ComponentSet { id, .. }
            | Self::Instance { id, .. }
            | Self::ShapeWithText { id, .. }
            | Self::Ellipse { id, .. }
            | Self::Line { id, .. }
            | Self::Polygon { id, .. }
            | Self::Star { id, .. }
            | Self::BooleanOperation { id, .. }
            | Self::Sticky { id, .. }
            | Self::Connector { id, .. }
            | Self::Widget { id, .. }
            | Self::Table { id, .. }
            | Self::TableCell { id, .. } => id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Canvas { name, .. }
            | Self::Section { name, .. }
            | Self::Frame { name, .. }
            | Self::Group { name, .. }
            | Self::Text { name, .. }
            | Self::Rectangle { name, .. }
            | Self::Vector { name, .. }
            | Self::Component { name, .. }
            | Self::ComponentSet { name, .. }
            | Self::Instance { name, .. }
            | Self::ShapeWithText { name, .. }
            | Self::Ellipse { name, .. }
            | Self::Line { name, .. }
            | Self::Polygon { name, .. }
            | Self::Star { name, .. }
            | Self::BooleanOperation { name, .. }
            | Self::Sticky { name, .. }
            | Self::Connector { name, .. }
            | Self::Widget { name, .. }
            | Self::Table { name, .. }
            | Self::TableCell { name, .. } => name,
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            Self::Canvas { visible, .. }
            | Self::Section { visible, .. }
            | Self::Frame { visible, .. }
            | Self::Group { visible, .. }
            | Self::Text { visible, .. }
            | Self::Rectangle { visible, .. }
            | Self::Vector { visible, .. }
            | Self::Component { visible, .. }
            | Self::ComponentSet { visible, .. }
            | Self::Instance { visible, .. }
            | Self::ShapeWithText { visible, .. }
            | Self::Ellipse { visible, .. }
            | Self::Line { visible, .. }
            | Self::Polygon { visible, .. }
            | Self::Star { visible, .. }
            | Self::BooleanOperation { visible, .. }
            | Self::Sticky { visible, .. }
            | Self::Connector { visible, .. }
            | Self::Widget { visible, .. }
            | Self::Table { visible, .. }
            | Self::TableCell { visible, .. } => *visible,
        }
    }

    pub const fn node_type(&self) -> NodeType {
        match self {
            Self::Canvas { .. } => NodeType::Canvas,
            Self::Section { .. } => NodeType::Section,
            Self::Frame { .. } => NodeType::Frame,
            Self::Group { .. } => NodeType::Group,
            Self::Text { .. } => NodeType::Text,
            Self::Rectangle { .. } => NodeType::Rectangle,
            Self::Vector { .. } => NodeType::Vector,
            Self::Component { .. } => NodeType::Component,
            Self::Instance { .. } => NodeType::Instance,
            _ => NodeType::Vector, // Shape types treated as Vector for simplicity
        }
    }

    pub fn children(&self) -> Option<&[Self]> {
        match self {
            Self::Canvas { children, .. }
            | Self::Section { children, .. }
            | Self::Frame { children, .. }
            | Self::Group { children, .. }
            | Self::Component { children, .. }
            | Self::ComponentSet { children, .. }
            | Self::Instance { children, .. }
            | Self::BooleanOperation { children, .. }
            | Self::Table { children, .. }
            | Self::TableCell { children, .. } => Some(children),
            _ => None,
        }
    }
}
