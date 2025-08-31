//! Core types and identifiers for the Velora web engine

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for DOM nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

/// Unique identifier for DOM elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ElementId(pub u64);

/// Unique identifier for style rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StyleId(pub u64);

/// Unique identifier for layout boxes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LayoutId(pub u64);

/// Unique identifier for paint operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaintId(pub u64);

/// Unique identifier for network resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceId(pub u64);

/// Unique identifier for JavaScript contexts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JsContextId(pub u64);

/// 2D point with floating-point coordinates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// 2D size with floating-point dimensions
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    
    pub fn zero() -> Self {
        Self { width: 0.0, height: 0.0 }
    }
}

/// 2D rectangle with position and size
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn from_point_size(point: Point, size: Size) -> Self {
        Self {
            x: point.x,
            y: point.y,
            width: size.width,
            height: size.height,
        }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }
    }
    
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }
    
    pub fn intersects(&self, other: &Rect) -> bool {
        !(self.x + self.width <= other.x || other.x + other.width <= self.x ||
          self.y + self.height <= other.y || other.y + other.height <= self.y)
    }
}

/// Color representation with RGBA components
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn black() -> Self {
        Self::rgb(0, 0, 0)
    }
    
    pub fn white() -> Self {
        Self::rgb(255, 255, 255)
    }
    
    pub fn transparent() -> Self {
        Self::rgba(0, 0, 0, 0)
    }
}

/// CSS property value that can be different types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CssValue {
    Keyword(String),
    String(String),
    Number(f32),
    Percentage(f32),
    Length(f32, CssUnit),
    Color(Color),
    Url(String),
    Function(String, Vec<CssValue>),
    List(Vec<CssValue>),
}

/// CSS length units
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CssUnit {
    Px,
    Em,
    Rem,
    Vw,
    Vh,
    Percent,
    Auto,
    None,
}

/// Display property values
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    Flex,
    Grid,
    None,
}

/// Position property values
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

/// Flexbox direction
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

/// Justify content alignment
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Align items alignment
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

/// Event types that can be handled
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    Click,
    MouseDown,
    MouseUp,
    MouseMove,
    KeyDown,
    KeyUp,
    Load,
    Unload,
    Scroll,
    Resize,
    Custom(String),
}

/// Event with associated data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub target: NodeId,
    pub current_target: NodeId,
    pub timestamp: f64,
    pub data: HashMap<String, serde_json::Value>,
}

impl Event {
    pub fn new(event_type: EventType, target: NodeId) -> Self {
        Self {
            event_type,
            target,
            current_target: target,
            timestamp: 0.0, // Will be set by event system
            data: HashMap::new(),
        }
    }
}

/// HTTP method for network requests
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Patch,
}

/// HTTP status codes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpStatus {
    pub code: u16,
    pub reason: String,
}

impl HttpStatus {
    pub fn new(code: u16, reason: String) -> Self {
        Self { code, reason }
    }
    
    pub fn ok() -> Self {
        Self::new(200, "OK".to_string())
    }
    
    pub fn not_found() -> Self {
        Self::new(404, "Not Found".to_string())
    }
    
    pub fn internal_server_error() -> Self {
        Self::new(500, "Internal Server Error".to_string())
    }
    
    pub fn is_success(&self) -> bool {
        self.code >= 200 && self.code < 300
    }
    
    pub fn is_client_error(&self) -> bool {
        self.code >= 400 && self.code < 500
    }
    
    pub fn is_server_error(&self) -> bool {
        self.code >= 500 && self.code < 600
    }
}
