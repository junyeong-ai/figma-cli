//! Figma API document structures
//!
//! Optimized node structure using composition pattern to eliminate field duplication.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
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

/// Common fields shared by all node types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBase {
    #[serde(rename = "type")]
    pub node_type: String,
    pub id: String,
    pub name: String,
    #[serde(default = "default_true")]
    pub visible: bool,
    #[serde(default)]
    pub locked: bool,
}

/// Unified node structure using composition
#[derive(Debug, Clone)]
pub struct Node {
    pub base: NodeBase,
    pub data: NodeData,
}

/// Variant-specific data for each node type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeData {
    Canvas {
        #[serde(rename = "backgroundColor", default, with = "option_struct")]
        background_color: Option<Color>,
        #[serde(rename = "exportSettings", default)]
        export_settings: Vec<ExportSetting>,
        #[serde(default)]
        children: Vec<Node>,
    },
    Section {
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(rename = "absoluteRenderBounds", default, with = "option_struct")]
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
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default)]
        fills: Vec<Paint>,
        #[serde(rename = "clipsContent", default)]
        clips_content: bool,
        #[serde(default)]
        children: Vec<Node>,
    },
    Group {
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default)]
        children: Vec<Node>,
    },
    Text {
        #[serde(default)]
        characters: String,
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default, with = "option_struct")]
        style: Option<TypeStyle>,
    },
    Rectangle {
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(rename = "cornerRadius", default)]
        corner_radius: f64,
        #[serde(default)]
        fills: Vec<Paint>,
    },
    Vector {
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default)]
        fills: Vec<Paint>,
    },
    Component {
        #[serde(rename = "componentKey")]
        component_key: Option<String>,
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default)]
        children: Vec<Node>,
    },
    ComponentSet {
        #[serde(rename = "componentKey")]
        component_key: Option<String>,
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default)]
        children: Vec<Node>,
    },
    Instance {
        #[serde(rename = "componentId", default)]
        component_id: String,
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default)]
        children: Vec<Node>,
    },
    Sticky {
        #[serde(default)]
        characters: String,
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default)]
        fills: Vec<Paint>,
    },
    BooleanOperation {
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default)]
        fills: Vec<Paint>,
        #[serde(default)]
        children: Vec<Node>,
    },
    Table {
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default)]
        fills: Vec<Paint>,
        #[serde(default)]
        children: Vec<Node>,
    },
    TableCell {
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default)]
        fills: Vec<Paint>,
        #[serde(default)]
        children: Vec<Node>,
    },
    Shape {
        #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
        absolute_bounding_box: Option<BoundingBox>,
        #[serde(default)]
        fills: Vec<Paint>,
    },
    Other {
        #[serde(default)]
        characters: Option<String>,
        #[serde(default)]
        children: Vec<Node>,
    },
}

impl Node {
    #[inline]
    pub fn id(&self) -> &str {
        &self.base.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.base.name
    }

    #[inline]
    pub fn is_visible(&self) -> bool {
        self.base.visible
    }

    #[inline]
    pub fn node_type_str(&self) -> &str {
        &self.base.node_type
    }

    pub fn children(&self) -> Option<&[Node]> {
        match &self.data {
            NodeData::Canvas { children, .. }
            | NodeData::Section { children, .. }
            | NodeData::Frame { children, .. }
            | NodeData::Group { children, .. }
            | NodeData::Component { children, .. }
            | NodeData::ComponentSet { children, .. }
            | NodeData::Instance { children, .. }
            | NodeData::BooleanOperation { children, .. }
            | NodeData::Table { children, .. }
            | NodeData::TableCell { children, .. }
            | NodeData::Other { children, .. } => Some(children),
            _ => None,
        }
    }

    pub fn characters(&self) -> Option<&str> {
        match &self.data {
            NodeData::Text { characters, .. } | NodeData::Sticky { characters, .. } => {
                Some(characters)
            }
            NodeData::Other { characters, .. } => characters.as_deref(),
            _ => None,
        }
    }

    pub fn style(&self) -> Option<&TypeStyle> {
        match &self.data {
            NodeData::Text { style, .. } => style.as_ref(),
            _ => None,
        }
    }

    pub fn is_text_node(&self) -> bool {
        matches!(self.base.node_type.as_str(), "TEXT" | "STICKY")
    }

    pub fn is_container(&self) -> bool {
        self.children().is_some()
    }
}

const fn default_true() -> bool {
    true
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("type", &self.base.node_type)?;
        map.serialize_entry("id", &self.base.id)?;
        map.serialize_entry("name", &self.base.name)?;
        map.serialize_entry("visible", &self.base.visible)?;
        map.serialize_entry("locked", &self.base.locked)?;

        match &self.data {
            NodeData::Canvas {
                background_color,
                export_settings,
                children,
            } => {
                if let Some(bg) = background_color {
                    map.serialize_entry("backgroundColor", bg)?;
                }
                map.serialize_entry("exportSettings", export_settings)?;
                map.serialize_entry("children", children)?;
            }
            NodeData::Section {
                absolute_bounding_box,
                fills,
                strokes,
                stroke_weight,
                stroke_align,
                section_contents_hidden,
                children,
                ..
            } => {
                if let Some(bb) = absolute_bounding_box {
                    map.serialize_entry("absoluteBoundingBox", bb)?;
                }
                map.serialize_entry("fills", fills)?;
                map.serialize_entry("strokes", strokes)?;
                map.serialize_entry("strokeWeight", stroke_weight)?;
                map.serialize_entry("strokeAlign", stroke_align)?;
                map.serialize_entry("sectionContentsHidden", section_contents_hidden)?;
                map.serialize_entry("children", children)?;
            }
            NodeData::Frame {
                absolute_bounding_box,
                fills,
                clips_content,
                children,
            } => {
                if let Some(bb) = absolute_bounding_box {
                    map.serialize_entry("absoluteBoundingBox", bb)?;
                }
                map.serialize_entry("fills", fills)?;
                map.serialize_entry("clipsContent", clips_content)?;
                map.serialize_entry("children", children)?;
            }
            NodeData::Group {
                absolute_bounding_box,
                children,
            } => {
                if let Some(bb) = absolute_bounding_box {
                    map.serialize_entry("absoluteBoundingBox", bb)?;
                }
                map.serialize_entry("children", children)?;
            }
            NodeData::Text {
                characters,
                absolute_bounding_box,
                style,
            } => {
                map.serialize_entry("characters", characters)?;
                if let Some(bb) = absolute_bounding_box {
                    map.serialize_entry("absoluteBoundingBox", bb)?;
                }
                if let Some(s) = style {
                    map.serialize_entry("style", s)?;
                }
            }
            NodeData::Rectangle {
                absolute_bounding_box,
                corner_radius,
                fills,
            } => {
                if let Some(bb) = absolute_bounding_box {
                    map.serialize_entry("absoluteBoundingBox", bb)?;
                }
                map.serialize_entry("cornerRadius", corner_radius)?;
                map.serialize_entry("fills", fills)?;
            }
            NodeData::Vector {
                absolute_bounding_box,
                fills,
            }
            | NodeData::Shape {
                absolute_bounding_box,
                fills,
            } => {
                if let Some(bb) = absolute_bounding_box {
                    map.serialize_entry("absoluteBoundingBox", bb)?;
                }
                map.serialize_entry("fills", fills)?;
            }
            NodeData::Component {
                component_key,
                absolute_bounding_box,
                children,
            }
            | NodeData::ComponentSet {
                component_key,
                absolute_bounding_box,
                children,
            } => {
                if let Some(key) = component_key {
                    map.serialize_entry("componentKey", key)?;
                }
                if let Some(bb) = absolute_bounding_box {
                    map.serialize_entry("absoluteBoundingBox", bb)?;
                }
                map.serialize_entry("children", children)?;
            }
            NodeData::Instance {
                component_id,
                absolute_bounding_box,
                children,
            } => {
                map.serialize_entry("componentId", component_id)?;
                if let Some(bb) = absolute_bounding_box {
                    map.serialize_entry("absoluteBoundingBox", bb)?;
                }
                map.serialize_entry("children", children)?;
            }
            NodeData::Sticky {
                characters,
                absolute_bounding_box,
                fills,
            } => {
                map.serialize_entry("characters", characters)?;
                if let Some(bb) = absolute_bounding_box {
                    map.serialize_entry("absoluteBoundingBox", bb)?;
                }
                map.serialize_entry("fills", fills)?;
            }
            NodeData::BooleanOperation {
                absolute_bounding_box,
                fills,
                children,
            }
            | NodeData::Table {
                absolute_bounding_box,
                fills,
                children,
            }
            | NodeData::TableCell {
                absolute_bounding_box,
                fills,
                children,
            } => {
                if let Some(bb) = absolute_bounding_box {
                    map.serialize_entry("absoluteBoundingBox", bb)?;
                }
                map.serialize_entry("fills", fills)?;
                map.serialize_entry("children", children)?;
            }
            NodeData::Other {
                characters,
                children,
            } => {
                if let Some(c) = characters {
                    map.serialize_entry("characters", c)?;
                }
                map.serialize_entry("children", children)?;
            }
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let base: NodeBase =
            serde_json::from_value(value.clone()).map_err(serde::de::Error::custom)?;

        let data = match base.node_type.as_str() {
            "CANVAS" => {
                let d: CanvasRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Canvas {
                    background_color: d.background_color,
                    export_settings: d.export_settings,
                    children: d.children,
                }
            }
            "SECTION" => {
                let d: SectionRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Section {
                    absolute_bounding_box: d.absolute_bounding_box,
                    absolute_render_bounds: d.absolute_render_bounds,
                    fills: d.fills,
                    strokes: d.strokes,
                    stroke_weight: d.stroke_weight,
                    stroke_align: d.stroke_align,
                    section_contents_hidden: d.section_contents_hidden,
                    children: d.children,
                }
            }
            "FRAME" => {
                let d: FrameRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Frame {
                    absolute_bounding_box: d.absolute_bounding_box,
                    fills: d.fills,
                    clips_content: d.clips_content,
                    children: d.children,
                }
            }
            "GROUP" => {
                let d: GroupRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Group {
                    absolute_bounding_box: d.absolute_bounding_box,
                    children: d.children,
                }
            }
            "TEXT" => {
                let d: TextRaw = serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Text {
                    characters: d.characters,
                    absolute_bounding_box: d.absolute_bounding_box,
                    style: d.style,
                }
            }
            "RECTANGLE" => {
                let d: RectangleRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Rectangle {
                    absolute_bounding_box: d.absolute_bounding_box,
                    corner_radius: d.corner_radius,
                    fills: d.fills,
                }
            }
            "VECTOR" | "ELLIPSE" | "LINE" | "REGULAR_POLYGON" | "STAR" | "SHAPE_WITH_TEXT"
            | "CONNECTOR" | "WIDGET" => {
                let d: ShapeRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Shape {
                    absolute_bounding_box: d.absolute_bounding_box,
                    fills: d.fills,
                }
            }
            "COMPONENT" => {
                let d: ComponentRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Component {
                    component_key: d.component_key,
                    absolute_bounding_box: d.absolute_bounding_box,
                    children: d.children,
                }
            }
            "COMPONENT_SET" => {
                let d: ComponentRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::ComponentSet {
                    component_key: d.component_key,
                    absolute_bounding_box: d.absolute_bounding_box,
                    children: d.children,
                }
            }
            "INSTANCE" => {
                let d: InstanceRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Instance {
                    component_id: d.component_id,
                    absolute_bounding_box: d.absolute_bounding_box,
                    children: d.children,
                }
            }
            "STICKY" => {
                let d: StickyRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Sticky {
                    characters: d.characters,
                    absolute_bounding_box: d.absolute_bounding_box,
                    fills: d.fills,
                }
            }
            "BOOLEAN_OPERATION" => {
                let d: ContainerWithFillsRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::BooleanOperation {
                    absolute_bounding_box: d.absolute_bounding_box,
                    fills: d.fills,
                    children: d.children,
                }
            }
            "TABLE" => {
                let d: ContainerWithFillsRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Table {
                    absolute_bounding_box: d.absolute_bounding_box,
                    fills: d.fills,
                    children: d.children,
                }
            }
            "TABLE_CELL" => {
                let d: ContainerWithFillsRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::TableCell {
                    absolute_bounding_box: d.absolute_bounding_box,
                    fills: d.fills,
                    children: d.children,
                }
            }
            _ => {
                let d: OtherRaw =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                NodeData::Other {
                    characters: d.characters,
                    children: d.children,
                }
            }
        };

        Ok(Node { base, data })
    }
}

// Raw deserialization helpers (internal use only)
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CanvasRaw {
    #[serde(rename = "backgroundColor", default, with = "option_struct")]
    background_color: Option<Color>,
    #[serde(rename = "exportSettings", default)]
    export_settings: Vec<ExportSetting>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SectionRaw {
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(rename = "absoluteRenderBounds", default, with = "option_struct")]
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
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FrameRaw {
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
    #[serde(rename = "clipsContent", default)]
    clips_content: bool,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GroupRaw {
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TextRaw {
    #[serde(default)]
    characters: String,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default, with = "option_struct")]
    style: Option<TypeStyle>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RectangleRaw {
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(rename = "cornerRadius", default)]
    corner_radius: f64,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShapeRaw {
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComponentRaw {
    #[serde(rename = "componentKey")]
    component_key: Option<String>,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstanceRaw {
    #[serde(rename = "componentId", default)]
    component_id: String,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StickyRaw {
    #[serde(default)]
    characters: String,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContainerWithFillsRaw {
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtherRaw {
    #[serde(default)]
    characters: Option<String>,
    #[serde(default)]
    children: Vec<Node>,
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
