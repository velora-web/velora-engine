//! DOM implementation for the Velora web engine
//! 
//! This crate provides the Document Object Model implementation,
//! including HTML parsing, DOM tree structure, and node manipulation.

pub mod document;
pub mod element;
pub mod node;
pub mod parser;
pub mod tree;

pub use document::Document;
pub use element::Element;
pub use node::{Node, NodeType};
pub use parser::HtmlParser;
pub use tree::DomTree;

/// Re-export commonly used items for convenience
pub mod prelude {
    pub use super::document::Document;
    pub use super::element::Element;
    pub use super::node::{Node, NodeType};
    pub use super::parser::HtmlParser;
    pub use super::tree::DomTree;
}
