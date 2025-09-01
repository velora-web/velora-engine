//! Platform abstraction layer for the Velora web engine
//! 
//! This crate provides cross-platform abstractions for:
//! - Window management using winit
//! - Graphics rendering using WGPU
//! - Input handling
//! - Platform-specific features

pub mod window;
pub mod graphics;
pub mod input;
pub mod platform;

pub use window::{Window, WindowBuilder, WindowEvent, WindowConfig};
pub use graphics::{GraphicsContext, GraphicsConfig, Vertex};
pub use input::InputHandler;
pub use platform::{Platform, PlatformBuilder, PlatformConfig};

// Re-export common types
pub use velora_core::{VeloraResult, Size, Point};

/// Platform prelude module for easy importing
pub mod prelude {
    pub use super::{
        Window, WindowBuilder, WindowEvent, WindowConfig,
        GraphicsContext, GraphicsConfig, Vertex,
        InputHandler,
        Platform, PlatformBuilder, PlatformConfig,
        VeloraResult, Size, Point,
    };
}
