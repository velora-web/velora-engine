//! Cross-platform window management using winit

use velora_core::{VeloraResult, VeloraError, Size, Point};
use velora_core::error::PlatformError;
use winit::{
    event::{Event, WindowEvent as WinitWindowEvent},
    event_loop::EventLoop,
    window::{Window as WinitWindow, WindowAttributes},
    dpi::LogicalSize,
};
use std::sync::Arc;
use log::{debug, info};
use raw_window_handle::HasWindowHandle;

/// Window configuration options
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Window title
    pub title: String,
    
    /// Initial window size
    pub size: Size,
    
    /// Whether the window is resizable
    pub resizable: bool,
    
    /// Whether the window is maximized by default
    pub maximized: bool,
    
    /// Whether the window is fullscreen
    pub fullscreen: bool,
    
    /// Whether the window is visible
    pub visible: bool,
    
    /// Whether the window should be decorated (title bar, borders)
    pub decorated: bool,
    
    /// Whether the window should always be on top
    pub always_on_top: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Velora Engine".to_string(),
            size: Size::new(800.0, 600.0),
            resizable: true,
            maximized: false,
            fullscreen: false,
            visible: true,
            decorated: true,
            always_on_top: false,
        }
    }
}

/// Window builder for creating windows with custom configurations
pub struct WindowBuilder {
    config: WindowConfig,
}

impl WindowBuilder {
    /// Create a new window builder with default configuration
    pub fn new() -> Self {
        Self {
            config: WindowConfig::default(),
        }
    }
    
    /// Set the window title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }
    
    /// Set the window size
    pub fn with_size(mut self, size: Size) -> Self {
        self.config.size = size;
        self
    }
    
    /// Set whether the window is resizable
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.config.resizable = resizable;
        self
    }
    
    /// Set whether the window is maximized
    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.config.maximized = maximized;
        self
    }
    
    /// Set whether the window is fullscreen
    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.config.fullscreen = fullscreen;
        self
    }
    
    /// Set whether the window is visible
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.config.visible = visible;
        self
    }
    
    /// Set whether the window is decorated
    pub fn with_decorated(mut self, decorated: bool) -> Self {
        self.config.decorated = decorated;
        self
    }
    
    /// Set whether the window is always on top
    pub fn with_always_on_top(mut self, always_on_top: bool) -> Self {
        self.config.always_on_top = always_on_top;
        self
    }
    
    /// Build the window
    pub fn build(self, event_loop: &EventLoop<()>) -> VeloraResult<Window> {
        debug!("Building window with config: {:?}", self.config);
        
        let mut attributes = WindowAttributes::default()
            .with_title(&self.config.title)
            .with_inner_size(LogicalSize::new(
                self.config.size.width,
                self.config.size.height,
            ))
            .with_resizable(self.config.resizable)
            .with_decorations(self.config.decorated);
        
        if self.config.maximized {
            attributes = attributes.with_maximized(true);
        }
        
        if !self.config.visible {
            attributes = attributes.with_visible(false);
        }
        
        #[allow(deprecated)]
        let winit_window = event_loop
            .create_window(attributes)
            .map_err(|e| VeloraError::Platform(PlatformError::WindowCreation(e.to_string())))?;
        
        if self.config.fullscreen {
            winit_window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        }
        
        let window = Window {
            inner: Arc::new(winit_window),
            config: self.config,
        };
        
        info!("Window created successfully: {}", window.config.title);
        Ok(window)
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Window events that can be handled
#[derive(Debug, Clone)]
pub enum WindowEvent {
    /// Window was resized
    Resized(Size),
    
    /// Window was moved
    Moved(Point),
    
    /// Window gained focus
    Focused,
    
    /// Window lost focus
    Unfocused,
    
    /// Window was closed
    Closed,
    
    /// Window was maximized
    Maximized,
    
    /// Window was minimized
    Minimized,
    
    /// Window was restored from minimized state
    Restored,
    
    /// Window entered fullscreen mode
    EnteredFullscreen,
    
    /// Window exited fullscreen mode
    ExitedFullscreen,
}

/// A cross-platform window
pub struct Window {
    /// The underlying winit window
    inner: Arc<WinitWindow>,
    
    /// Window configuration
    config: WindowConfig,
}

impl Window {
    /// Create a new window builder
    pub fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }
    
    /// Get the window title
    pub fn title(&self) -> &str {
        &self.config.title
    }
    
    /// Set the window title
    pub fn set_title(&self, title: impl Into<String>) {
        let title = title.into();
        self.inner.set_title(&title);
        // Note: We can't update our config here since self is not mutable
        // In a real implementation, you might want to use interior mutability
    }
    
    /// Get the current window size
    pub fn size(&self) -> Size {
        let size = self.inner.inner_size();
        Size::new(size.width as f32, size.height as f32)
    }
    
    /// Set the window size
    pub fn set_size(&self, size: Size) {
        let _ = self.inner.request_inner_size(LogicalSize::new(size.width, size.height));
    }
    
    /// Get the current window position
    pub fn position(&self) -> Point {
        if let Ok(pos) = self.inner.outer_position() {
            Point::new(pos.x as f32, pos.y as f32)
        } else {
            Point::zero()
        }
    }
    
    /// Set the window position
    pub fn set_position(&self, position: Point) {
        self.inner.set_outer_position(winit::dpi::LogicalPosition::new(
            position.x,
            position.y,
        ));
    }
    
    /// Check if the window is visible
    pub fn is_visible(&self) -> bool {
        self.inner.is_visible().unwrap_or(false)
    }
    
    /// Show the window
    pub fn show(&self) {
        self.inner.set_visible(true);
    }
    
    /// Hide the window
    pub fn hide(&self) {
        self.inner.set_visible(false);
    }
    
    /// Check if the window is maximized
    pub fn is_maximized(&self) -> bool {
        self.inner.is_maximized()
    }
    
    /// Maximize the window
    pub fn maximize(&self) {
        self.inner.set_maximized(true);
    }
    
    /// Restore the window from maximized state
    pub fn restore(&self) {
        self.inner.set_maximized(false);
    }
    
    /// Check if the window is fullscreen
    pub fn is_fullscreen(&self) -> bool {
        self.inner.fullscreen().is_some()
    }
    
    /// Enter fullscreen mode
    pub fn enter_fullscreen(&self) {
        self.inner.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    }
    
    /// Exit fullscreen mode
    pub fn exit_fullscreen(&self) {
        self.inner.set_fullscreen(None);
    }
    
    /// Check if the window is focused
    pub fn is_focused(&self) -> bool {
        self.inner.has_focus()
    }
    
    /// Focus the window
    pub fn focus(&self) {
        self.inner.focus_window();
    }
    
    /// Request a redraw of the window
    pub fn request_redraw(&self) {
        self.inner.request_redraw();
    }
    
    /// Get the window handle for graphics operations
    pub fn window_handle(&self) -> raw_window_handle::WindowHandle<'_> {
        // Try to get the window handle, but if it fails, we'll panic
        // since this is a critical operation for graphics
        self.inner.window_handle().expect("Failed to get window handle")
    }
    
    /// Get the underlying winit window
    pub fn inner(&self) -> &Arc<WinitWindow> {
        &self.inner
    }
    
    /// Get the window configuration
    pub fn config(&self) -> &WindowConfig {
        &self.config
    }
    
    /// Process window events and convert them to our event types
    pub fn process_event(&self, event: &Event<()>) -> Option<WindowEvent> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WinitWindowEvent::Resized(size) => Some(WindowEvent::Resized(Size::new(
                    size.width as f32,
                    size.height as f32,
                ))),
                WinitWindowEvent::Moved(pos) => Some(WindowEvent::Moved(Point::new(
                    pos.x as f32,
                    pos.y as f32,
                ))),
                WinitWindowEvent::Focused(focused) => {
                    if *focused {
                        Some(WindowEvent::Focused)
                    } else {
                        Some(WindowEvent::Unfocused)
                    }
                }
                WinitWindowEvent::CloseRequested => Some(WindowEvent::Closed),
                WinitWindowEvent::RedrawRequested => {
                    self.request_redraw();
                    None
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl std::fmt::Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("title", &self.config.title)
            .field("size", &self.size())
            .field("position", &self.position())
            .field("visible", &self.is_visible())
            .field("focused", &self.is_focused())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_window_config_default() {
        let config = WindowConfig::default();
        assert_eq!(config.title, "Velora Engine");
        assert_eq!(config.size, Size::new(800.0, 600.0));
        assert!(config.resizable);
        assert!(!config.maximized);
        assert!(!config.fullscreen);
        assert!(config.visible);
        assert!(config.decorated);
        assert!(!config.always_on_top);
    }
    
    #[test]
    fn test_window_builder() {
        let builder = WindowBuilder::new()
            .with_title("Test Window")
            .with_size(Size::new(1024.0, 768.0))
            .with_resizable(false)
            .with_maximized(true);
        
        assert_eq!(builder.config.title, "Test Window");
        assert_eq!(builder.config.size, Size::new(1024.0, 768.0));
        assert!(!builder.config.resizable);
        assert!(builder.config.maximized);
    }
    
    #[test]
    fn test_window_builder_methods() {
        let builder = WindowBuilder::new()
            .with_fullscreen(true)
            .with_visible(false)
            .with_decorated(false)
            .with_always_on_top(true);
        
        assert!(builder.config.fullscreen);
        assert!(!builder.config.visible);
        assert!(!builder.config.decorated);
        assert!(builder.config.always_on_top);
    }
}
