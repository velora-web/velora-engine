//! Cross-platform platform implementation
//! 
//! This module provides a unified platform abstraction that works across
//! Windows, macOS, and Linux using winit for window management.

use velora_core::{VeloraResult, VeloraError, Size, Point};
use velora_core::error::PlatformError;
use super::window::{Window, WindowBuilder, WindowEvent};
use winit::{
    event::{Event, WindowEvent as WinitWindowEvent},
    event_loop::EventLoop,
    window::WindowId,
};
use std::collections::HashMap;
use std::sync::Arc;
use log::{debug, info, warn};

/// Cross-platform platform features
#[derive(Debug, Clone)]
pub struct PlatformFeatures {
    /// Whether hardware acceleration is available
    pub hardware_acceleration: bool,
    
    /// Current display scale factor
    pub display_scale: f32,
    
    /// Whether high contrast mode is enabled
    pub high_contrast: bool,
    
    /// Platform-specific theme support
    pub theme_support: bool,
}

impl Default for PlatformFeatures {
    fn default() -> Self {
        Self {
            hardware_acceleration: true,
            display_scale: 1.0,
            high_contrast: false,
            theme_support: true,
        }
    }
}

/// Unified cross-platform implementation
pub struct Platform {
    /// Event loop for handling window events
    event_loop: Option<EventLoop<()>>,
    
    /// Active windows
    windows: HashMap<WindowId, Arc<Window>>,
    
    /// Platform features
    features: PlatformFeatures,
    
    /// Event handlers
    event_handlers: Vec<Box<dyn Fn(&WindowEvent) + Send + Sync>>,
    
    /// Whether the platform is running
    running: bool,
}

impl Platform {
    /// Create a new cross-platform instance
    pub fn new() -> VeloraResult<Self> {
        debug!("Initializing cross-platform platform");
        
        let event_loop = EventLoop::new()
            .map_err(|e| VeloraError::Platform(PlatformError::GraphicsInit(e.to_string())))?;
        
        // Detect platform features
        let features = Self::detect_platform_features();
        
        info!("Cross-platform initialized with features: {:?}", features);
        
        Ok(Self {
            event_loop: Some(event_loop),
            windows: HashMap::new(),
            features,
            event_handlers: Vec::new(),
            running: false,
        })
    }
    
    /// Detect platform features
    fn detect_platform_features() -> PlatformFeatures {
        // In a real implementation, this would detect platform-specific capabilities
        // For now, return sensible defaults
        PlatformFeatures::default()
    }
    
    /// Create a window with the given title and size
    pub fn create_window(&mut self, title: &str, size: Size) -> VeloraResult<Arc<Window>> {
        let event_loop = self.event_loop
            .as_ref()
            .ok_or_else(|| VeloraError::Platform(PlatformError::GraphicsInit("Event loop not available".to_string())))?;
        
        let window = WindowBuilder::new()
            .with_title(title)
            .with_size(size)
            .build(event_loop)?;
        
        let window_id = window.inner().id();
        let window_arc = Arc::new(window);
        
        self.windows.insert(window_id, window_arc.clone());
        
        info!("Window created: {} (ID: {:?})", title, window_id);
        Ok(window_arc)
    }
    
    /// Create a window with custom configuration
    pub fn create_custom_window(&mut self, builder: WindowBuilder) -> VeloraResult<Arc<Window>> {
        let event_loop = self.event_loop
            .as_ref()
            .ok_or_else(|| VeloraError::Platform(PlatformError::GraphicsInit("Event loop not available".to_string())))?;
        
        let window = builder.build(event_loop)?;
        let window_id = window.inner().id();
        let window_arc = Arc::new(window);
        
        self.windows.insert(window_id, window_arc.clone());
        
        info!("Custom window created (ID: {:?})", window_id);
        Ok(window_arc)
    }
    
    /// Get a window by ID
    pub fn get_window(&self, window_id: WindowId) -> Option<&Arc<Window>> {
        self.windows.get(&window_id)
    }
    
    /// Get all active windows
    pub fn get_windows(&self) -> &HashMap<WindowId, Arc<Window>> {
        &self.windows
    }
    
    /// Close a specific window
    pub fn close_window(&mut self, window_id: WindowId) -> bool {
        if let Some(_window) = self.windows.remove(&window_id) {
            info!("Closing window: {:?}", window_id);
            true
        } else {
            warn!("Attempted to close non-existent window: {:?}", window_id);
            false
        }
    }
    
    /// Close all windows
    pub fn close_all_windows(&mut self) {
        info!("Closing all windows");
        self.windows.clear();
    }
    
    /// Add an event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&WindowEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }
    
    /// Run the event loop with the given main window
    pub fn run_event_loop(&mut self, main_window: Arc<Window>) -> VeloraResult<()> {
        if self.running {
            return Err(VeloraError::Platform(PlatformError::GraphicsInit("Event loop already running".to_string())));
        }
        
        let event_loop = self.event_loop
            .take()
            .ok_or_else(|| VeloraError::Platform(PlatformError::GraphicsInit("Event loop not available".to_string())))?;
        
        info!("Starting cross-platform event loop");
        
        // Store the main window if it's not already stored
        let main_window_id = main_window.inner().id();
        if !self.windows.contains_key(&main_window_id) {
            self.windows.insert(main_window_id, main_window.clone());
        }
        
        self.running = true;
        
        // Clone necessary data for the closure
        let mut windows = self.windows.clone();
        let event_handlers = &self.event_handlers;
        
        // Run the event loop
        let result = event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { window_id, event } => {
                    if let Some(window) = windows.get(&window_id) {
                        // Convert winit event to our WindowEvent
                        if let Some(window_event) = Self::convert_winit_event(&event) {
                            // Call event handlers
                            for handler in event_handlers {
                                handler(&window_event);
                            }
                            
                            // Handle window-specific events
                            match window_event {
                                WindowEvent::Closed => {
                                    info!("Window closed: {:?}", window_id);
                                    windows.remove(&window_id);
                                    
                                    // If no windows left, exit
                                    if windows.is_empty() {
                                        elwt.exit();
                                    }
                                }
                                _ => {
                                    // Request redraw for other events
                                    window.request_redraw();
                                }
                            }
                        }
                    }
                }
                Event::DeviceEvent { .. } => {
                    // Handle device events (keyboard, mouse, etc.)
                }
                Event::UserEvent(_) => {
                    // Handle user events
                }
                Event::Suspended => {
                    info!("Application suspended");
                }
                Event::Resumed => {
                    info!("Application resumed");
                }
                Event::AboutToWait => {
                    // Request redraw for all windows
                    for window in windows.values() {
                        window.request_redraw();
                    }
                }
                Event::LoopExiting => {
                    info!("Event loop exiting");
                }
                _ => {
                    // Handle other events
                }
            }
        });
        
        // Handle any errors from the event loop
        if let Err(e) = result {
            warn!("Event loop error: {}", e);
        }
        
        self.running = false;
        Ok(())
    }
    
    /// Convert winit window event to our WindowEvent
    fn convert_winit_event(event: &WinitWindowEvent) -> Option<WindowEvent> {
        match event {
            WinitWindowEvent::CloseRequested => Some(WindowEvent::Closed),
            WinitWindowEvent::Resized(size) => Some(WindowEvent::Resized(Size::new(size.width as f32, size.height as f32))),
            WinitWindowEvent::Moved(pos) => Some(WindowEvent::Moved(Point::new(pos.x as f32, pos.y as f32))),
            WinitWindowEvent::Focused(focused) => {
                if *focused {
                    Some(WindowEvent::Focused)
                } else {
                    Some(WindowEvent::Unfocused)
                }
            }
            _ => None,
        }
    }
    
    /// Get platform features
    pub fn get_features(&self) -> &PlatformFeatures {
        &self.features
    }
    
    /// Update platform features
    pub fn update_features(&mut self) {
        self.features = Self::detect_platform_features();
        debug!("Platform features updated: {:?}", self.features);
    }
    
    /// Enable platform features
    pub fn enable_platform_features(&mut self) -> VeloraResult<()> {
        info!("Enabling platform features");
        
        // Enable hardware acceleration if available
        if self.features.hardware_acceleration {
            debug!("Hardware acceleration is enabled");
        }
        
        // Apply platform theme if supported
        if self.features.theme_support {
            debug!("Theme support is enabled");
        }
        
        Ok(())
    }
    
    /// Check if the platform is running
    pub fn is_running(&self) -> bool {
        self.running
    }
    
    /// Clean up platform resources
    pub fn cleanup(&mut self) {
        debug!("Cleaning up platform");
        
        // Close all windows
        self.close_all_windows();
        
        // Clear event handlers
        self.event_handlers.clear();
        
        // Clear event loop
        self.event_loop = None;
        
        self.running = false;
        
        info!("Platform cleanup complete");
    }
}

impl Drop for Platform {
    fn drop(&mut self) {
        self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_features_default() {
        let features = PlatformFeatures::default();
        assert!(features.hardware_acceleration);
        assert_eq!(features.display_scale, 1.0);
        assert!(!features.high_contrast);
        assert!(features.theme_support);
    }
    
    #[test]
    fn test_platform_creation() {
        // This test would require a mock event loop in a real implementation
        // For now, we'll just test the default features
        let features = PlatformFeatures::default();
        assert_eq!(features.display_scale, 1.0);
    }
}
