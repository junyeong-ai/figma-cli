//! Figma API document structures

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
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

#[derive(Debug, Clone, Serialize)]
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

        #[serde(default)]
        characters: String,

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

    /// Catch-all for unknown node types
    Other {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        #[serde(default)]
        characters: Option<String>,
        #[serde(default)]
        children: Vec<Node>,
    },
}

const fn default_true() -> bool {
    true
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let node_type = value
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN");

        match node_type {
            "CANVAS" => deserialize_variant::<CanvasData, D>(value).map(|d| Node::Canvas {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                background_color: d.background_color,
                export_settings: d.export_settings,
                children: d.children,
            }),
            "SECTION" => deserialize_variant::<SectionData, D>(value).map(|d| Node::Section {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                absolute_render_bounds: d.absolute_render_bounds,
                fills: d.fills,
                strokes: d.strokes,
                stroke_weight: d.stroke_weight,
                stroke_align: d.stroke_align,
                section_contents_hidden: d.section_contents_hidden,
                children: d.children,
            }),
            "FRAME" => deserialize_variant::<FrameData, D>(value).map(|d| Node::Frame {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                fills: d.fills,
                clips_content: d.clips_content,
                bound_variables: d.bound_variables,
                children: d.children,
            }),
            "GROUP" => deserialize_variant::<GroupData, D>(value).map(|d| Node::Group {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                children: d.children,
            }),
            "TEXT" => deserialize_variant::<TextData, D>(value).map(|d| Node::Text {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                characters: d.characters,
                absolute_bounding_box: d.absolute_bounding_box,
                style: d.style,
            }),
            "RECTANGLE" => deserialize_variant::<RectangleData, D>(value).map(|d| Node::Rectangle {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                corner_radius: d.corner_radius,
                fills: d.fills,
            }),
            "VECTOR" => deserialize_variant::<VectorData, D>(value).map(|d| Node::Vector {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                fills: d.fills,
            }),
            "COMPONENT" => deserialize_variant::<ComponentData, D>(value).map(|d| Node::Component {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                component_key: d.component_key,
                absolute_bounding_box: d.absolute_bounding_box,
                children: d.children,
            }),
            "COMPONENT_SET" => {
                deserialize_variant::<ComponentSetData, D>(value).map(|d| Node::ComponentSet {
                    node_type: d.node_type,
                    id: d.id,
                    name: d.name,
                    visible: d.visible,
                    locked: d.locked,
                    component_key: d.component_key,
                    absolute_bounding_box: d.absolute_bounding_box,
                    children: d.children,
                })
            }
            "INSTANCE" => deserialize_variant::<InstanceData, D>(value).map(|d| Node::Instance {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                component_id: d.component_id,
                absolute_bounding_box: d.absolute_bounding_box,
                children: d.children,
            }),
            "SHAPE_WITH_TEXT" => {
                deserialize_variant::<ShapeWithTextData, D>(value).map(|d| Node::ShapeWithText {
                    node_type: d.node_type,
                    id: d.id,
                    name: d.name,
                    visible: d.visible,
                    locked: d.locked,
                    absolute_bounding_box: d.absolute_bounding_box,
                    fills: d.fills,
                })
            }
            "ELLIPSE" => deserialize_variant::<EllipseData, D>(value).map(|d| Node::Ellipse {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                fills: d.fills,
            }),
            "LINE" => deserialize_variant::<LineData, D>(value).map(|d| Node::Line {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                fills: d.fills,
            }),
            "REGULAR_POLYGON" => {
                deserialize_variant::<PolygonData, D>(value).map(|d| Node::Polygon {
                    node_type: d.node_type,
                    id: d.id,
                    name: d.name,
                    visible: d.visible,
                    locked: d.locked,
                    absolute_bounding_box: d.absolute_bounding_box,
                    fills: d.fills,
                })
            }
            "STAR" => deserialize_variant::<StarData, D>(value).map(|d| Node::Star {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                fills: d.fills,
            }),
            "BOOLEAN_OPERATION" => {
                deserialize_variant::<BooleanOperationData, D>(value).map(|d| Node::BooleanOperation {
                    node_type: d.node_type,
                    id: d.id,
                    name: d.name,
                    visible: d.visible,
                    locked: d.locked,
                    absolute_bounding_box: d.absolute_bounding_box,
                    fills: d.fills,
                    children: d.children,
                })
            }
            "STICKY" => deserialize_variant::<StickyData, D>(value).map(|d| Node::Sticky {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                characters: d.characters,
                absolute_bounding_box: d.absolute_bounding_box,
                fills: d.fills,
            }),
            "CONNECTOR" => deserialize_variant::<ConnectorData, D>(value).map(|d| Node::Connector {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                fills: d.fills,
            }),
            "WIDGET" => deserialize_variant::<WidgetData, D>(value).map(|d| Node::Widget {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                fills: d.fills,
            }),
            "TABLE" => deserialize_variant::<TableData, D>(value).map(|d| Node::Table {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                fills: d.fills,
                children: d.children,
            }),
            "TABLE_CELL" => deserialize_variant::<TableCellData, D>(value).map(|d| Node::TableCell {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                absolute_bounding_box: d.absolute_bounding_box,
                fills: d.fills,
                children: d.children,
            }),
            _ => deserialize_variant::<OtherData, D>(value).map(|d| Node::Other {
                node_type: d.node_type,
                id: d.id,
                name: d.name,
                visible: d.visible,
                locked: d.locked,
                characters: d.characters,
                children: d.children,
            }),
        }
    }
}

fn deserialize_variant<'de, T, D>(value: Value) -> Result<T, D::Error>
where
    T: serde::de::DeserializeOwned,
    D: Deserializer<'de>,
{
    serde_json::from_value(value).map_err(serde::de::Error::custom)
}

// Helper structs for deserialization
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CanvasData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "backgroundColor", default, with = "option_struct")]
    background_color: Option<Color>,
    #[serde(rename = "exportSettings", default)]
    export_settings: Vec<ExportSetting>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SectionData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
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
struct FrameData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
    #[serde(rename = "clipsContent", default)]
    clips_content: bool,
    #[serde(rename = "boundVariables", default, skip_serializing)]
    bound_variables: Option<Value>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GroupData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TextData {
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
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default, with = "option_struct")]
    style: Option<TypeStyle>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RectangleData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(rename = "cornerRadius", default)]
    corner_radius: f64,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VectorData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComponentData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "componentKey")]
    component_key: Option<String>,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComponentSetData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "componentKey")]
    component_key: Option<String>,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstanceData {
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
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShapeWithTextData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EllipseData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LineData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PolygonData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StarData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BooleanOperationData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StickyData {
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
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConnectorData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WidgetData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableCellData {
    #[serde(rename = "type")]
    node_type: String,
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
    #[serde(rename = "absoluteBoundingBox", default, with = "option_struct")]
    absolute_bounding_box: Option<BoundingBox>,
    #[serde(default)]
    fills: Vec<Paint>,
    #[serde(default)]
    children: Vec<Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtherData {
    #[serde(rename = "type", default)]
    node_type: String,
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default)]
    locked: bool,
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
            | Self::TableCell { id, .. }
            | Self::Other { id, .. } => id,
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
            | Self::TableCell { name, .. }
            | Self::Other { name, .. } => name,
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
            | Self::TableCell { visible, .. }
            | Self::Other { visible, .. } => *visible,
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
