//! Rendering engine for the Velora web engine
//! 
//! This crate provides software rendering capabilities,
//! including text rendering, shapes, and images.

pub mod renderer;
pub mod text;
pub mod shapes;
pub mod images;

pub use renderer::Renderer;
pub use text::TextRenderer;
pub use shapes::ShapeRenderer;
pub use images::ImageRenderer;

/// Re-export commonly used items for convenience
pub mod prelude {
    pub use super::renderer::Renderer;
    pub use super::text::TextRenderer;
    pub use super::shapes::ShapeRenderer;
    pub use super::images::ImageRenderer;
}
