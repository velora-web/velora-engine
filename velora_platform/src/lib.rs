//! Cross-platform window management and graphics for the Velora web engine
//! 
//! This crate provides platform-independent abstractions for:
//! - Window creation and management
//! - Graphics context initialization
//! - Input handling
//! - Platform-specific optimizations

pub mod window;
pub mod graphics;
pub mod input;
pub mod platform;

pub use window::{Window, WindowBuilder, WindowEvent};
pub use graphics::{GraphicsContext, GraphicsConfig};
pub use input::{InputHandler, InputEvent};
pub use platform::{Platform, PlatformFeatures};

/// Re-export commonly used items for convenience
pub mod prelude {
    pub use super::window::{Window, WindowBuilder, WindowEvent};
    pub use super::graphics::{GraphicsContext, GraphicsConfig};
    pub use super::input::{InputHandler, InputEvent};
    pub use super::platform::{Platform, PlatformFeatures};
}
