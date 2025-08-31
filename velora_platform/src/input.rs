//! Input handling for the Velora web engine

use velora_core::Point;

/// Input event types
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// Mouse movement
    MouseMove(Point),
    
    /// Mouse button press
    MouseDown(Point, u8),
    
    /// Mouse button release
    MouseUp(Point, u8),
    
    /// Mouse wheel scroll
    MouseWheel(Point, f32),
    
    /// Key press
    KeyDown(u32),
    
    /// Key release
    KeyUp(u32),
    
    /// Text input
    TextInput(char),
}

/// Input handler for processing user input
#[derive(Debug)]
pub struct InputHandler {
    /// Input event queue
    events: Vec<InputEvent>,
}

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }
    
    /// Process an input event
    pub fn process_event(&mut self, event: InputEvent) {
        self.events.push(event);
    }
    
    /// Get all pending events
    pub fn get_events(&mut self) -> Vec<InputEvent> {
        let events = self.events.clone();
        self.events.clear();
        events
    }
}
