//! Velora Browser Library
//! 
//! This library provides the core browser functionality for the Velora web engine.
//! It includes DOM management, HTML/CSS parsing, layout, painting, and UI components.

pub mod browser;
pub mod ui;
pub mod ui_renderer;
pub mod input_handler;

// Re-export main types for convenience
pub use browser::Browser;
pub use ui::{BrowserUI, Tab, BrowserToolbar, TabBar};
pub use ui_renderer::{UIRenderer, ColorScheme, UIState, RenderMode};
