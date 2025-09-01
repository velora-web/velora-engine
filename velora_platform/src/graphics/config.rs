//! Graphics configuration for the Velora web engine

/// Graphics configuration
#[derive(Debug, Clone)]
pub struct GraphicsConfig {
    /// Whether to enable vsync
    pub vsync: bool,
    
    /// Anti-aliasing level
    pub antialiasing: u32,
    
    /// Maximum frame rate
    pub max_fps: Option<u32>,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            vsync: true,
            antialiasing: 4,
            max_fps: None,
        }
    }
}
