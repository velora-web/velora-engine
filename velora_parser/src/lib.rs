//! HTML and CSS parser for the Velora web engine
//! 
//! This crate provides HTML and CSS parsing functionality,
//! converting markup and stylesheets into structured data.

pub mod html;
pub mod css;
pub mod tokenizer;

pub use html::HtmlParser;
pub use css::{CssParser, CssRule, CssSelector};
pub use tokenizer::Tokenizer;

/// Re-export commonly used items for convenience
pub mod prelude {
    pub use super::html::HtmlParser;
    pub use super::css::{CssParser, CssRule, CssSelector};
    pub use super::tokenizer::Tokenizer;
}
