//! Input handling for browser UI interactions

use velora_core::{VeloraResult, Size, Point};
use super::ui::BrowserUI;
use log::{debug, info};

/// Input event types
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// Mouse click event
    MouseClick {
        position: Point,
        button: MouseButton,
    },
    
    /// Mouse move event
    MouseMove {
        position: Point,
    },
    
    /// Key press event
    KeyPress {
        key: Key,
        modifiers: KeyModifiers,
    },
    
    /// Key release event
    KeyRelease {
        key: Key,
        modifiers: KeyModifiers,
    },
    
    /// Text input event
    TextInput {
        text: String,
    },
    
    /// Window resize event
    WindowResize {
        new_size: Size,
    },
}

/// Mouse button types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Key types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    /// Navigation keys
    Backspace,
    Tab,
    Enter,
    Escape,
    Space,
    
    /// Arrow keys
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    
    /// Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    
    /// Modifier keys
    Shift,
    Control,
    Alt,
    Meta,
    
    /// Other keys
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,
    
    /// Special keys
    Unknown,
}

/// Key modifier flags
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyModifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub meta: bool,
}

/// UI element hit test result
#[derive(Debug, Clone)]
pub enum HitTestResult {
    /// No element hit
    None,
    
    /// Tab hit
    Tab {
        tab_id: String,
        is_close_button: bool,
    },
    
    /// New tab button hit
    NewTabButton,
    
    /// Navigation button hit
    NavigationButton {
        button_type: NavigationButtonType,
    },
    
    /// URL input field hit
    UrlInput,
    
    /// Content area hit
    ContentArea,
}

/// Navigation button types
#[derive(Debug, Clone, Copy)]
pub enum NavigationButtonType {
    Back,
    Forward,
    Refresh,
}

/// Input handler for browser UI
pub struct InputHandler {
    /// Current mouse position
    mouse_position: Point,
    
    /// Whether left mouse button is pressed
    left_mouse_pressed: bool,
    
    /// Whether URL input is focused
    url_input_focused: bool,
    
    /// Current clipboard content
    _clipboard_content: String,
}

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Self {
        Self {
            mouse_position: Point::new(0.0, 0.0),
            left_mouse_pressed: false,
            url_input_focused: false,
            _clipboard_content: String::new(),
        }
    }
    
    /// Handle an input event
    pub fn handle_event(&mut self, event: InputEvent, ui: &mut BrowserUI) -> VeloraResult<()> {
        match event {
            InputEvent::MouseClick { position, button } => {
                self.handle_mouse_click(position, button, ui)?;
            }
            InputEvent::MouseMove { position } => {
                self.mouse_position = position;
            }
            InputEvent::KeyPress { key, modifiers } => {
                self.handle_key_press(key, modifiers, ui)?;
            }
            InputEvent::KeyRelease { key, modifiers: _ } => {
                self.handle_key_release(key, ui)?;
            }
            InputEvent::TextInput { text } => {
                self.handle_text_input(text, ui)?;
            }
            InputEvent::WindowResize { new_size } => {
                ui.update_layout(new_size);
            }
        }
        
        Ok(())
    }
    
    /// Handle mouse click events
    fn handle_mouse_click(&mut self, position: Point, button: MouseButton, ui: &mut BrowserUI) -> VeloraResult<()> {
        if button == MouseButton::Left {
            self.left_mouse_pressed = true;
            
            // Perform hit testing
            let hit_result = self.hit_test(position, ui);
            
            match hit_result {
                HitTestResult::Tab { tab_id, is_close_button } => {
                    if is_close_button {
                        // Close the tab
                        ui.tab_bar.close_tab(&tab_id)?;
                        
                        // Update toolbar for new active tab
                        if let Some(tab) = ui.tab_bar.get_active_tab() {
                            ui.toolbar.update_for_tab(tab);
                        }
                    } else {
                        // Switch to the tab
                        ui.switch_to_tab(&tab_id)?;
                    }
                }
                HitTestResult::NewTabButton => {
                    // Create a new tab
                    let tab_id = ui.create_tab("about:blank".to_string());
                    info!("Created new tab: {}", tab_id);
                }
                HitTestResult::NavigationButton { button_type } => {
                    match button_type {
                        NavigationButtonType::Back => {
                            if let Some(url) = ui.go_back()? {
                                info!("Navigated back to: {}", url);
                            }
                        }
                        NavigationButtonType::Forward => {
                            if let Some(url) = ui.go_forward()? {
                                info!("Navigated forward to: {}", url);
                            }
                        }
                        NavigationButtonType::Refresh => {
                            ui.refresh_current_tab()?;
                            info!("Refreshed current tab");
                        }
                    }
                }
                HitTestResult::UrlInput => {
                    // Focus the URL input
                    self.url_input_focused = true;
                    ui.toolbar.set_url_input_focused(true);
                    debug!("URL input focused");
                }
                HitTestResult::ContentArea => {
                    // Unfocus URL input
                    self.url_input_focused = false;
                    ui.toolbar.set_url_input_focused(false);
                    debug!("URL input unfocused");
                }
                HitTestResult::None => {
                    // Unfocus URL input
                    self.url_input_focused = false;
                    ui.toolbar.set_url_input_focused(false);
                }
            }
        }
        
        Ok(())
    }
    
    /// Handle key press events
    fn handle_key_press(&mut self, key: Key, modifiers: KeyModifiers, ui: &mut BrowserUI) -> VeloraResult<()> {
        if self.url_input_focused {
            match key {
                Key::Enter => {
                    // Navigate to the URL in the input field
                    let url = ui.toolbar.url_input_text.clone();
                    if !url.is_empty() {
                        let url_clone = url.clone();
                        ui.navigate_current_tab(url)?;
                        info!("Navigating to: {}", url_clone);
                    }
                    
                    // Unfocus the input
                    self.url_input_focused = false;
                    ui.toolbar.set_url_input_focused(false);
                }
                Key::Escape => {
                    // Cancel URL input
                    self.url_input_focused = false;
                    ui.toolbar.set_url_input_focused(false);
                    
                    // Restore original URL
                    if let Some(tab) = ui.tab_bar.get_active_tab() {
                        ui.toolbar.set_url_input_text(tab.url.clone());
                    }
                }
                Key::Tab => {
                    // Handle tab completion (placeholder)
                    debug!("Tab completion requested");
                }
                _ => {
                    // Other keys are handled by text input
                }
            }
        } else {
            // Handle global keyboard shortcuts
            match (key, modifiers) {
                (Key::T, KeyModifiers { control: true, .. }) => {
                    // Ctrl+T: New tab
                    let tab_id = ui.create_tab("about:blank".to_string());
                    info!("Created new tab with Ctrl+T: {}", tab_id);
                }
                (Key::W, KeyModifiers { control: true, .. }) => {
                    // Ctrl+W: Close current tab
                    ui.close_current_tab()?;
                    info!("Closed current tab with Ctrl+W");
                }
                (Key::R, KeyModifiers { control: true, .. }) => {
                    // Ctrl+R: Refresh
                    ui.refresh_current_tab()?;
                    info!("Refreshed with Ctrl+R");
                }
                (Key::L, KeyModifiers { control: true, .. }) => {
                    // Ctrl+L: Focus URL input
                    self.url_input_focused = true;
                    ui.toolbar.set_url_input_focused(true);
                    debug!("URL input focused with Ctrl+L");
                }
                (Key::Key1, KeyModifiers { control: true, .. }) |
                (Key::Key2, KeyModifiers { control: true, .. }) |
                (Key::Key3, KeyModifiers { control: true, .. }) |
                (Key::Key4, KeyModifiers { control: true, .. }) |
                (Key::Key5, KeyModifiers { control: true, .. }) |
                (Key::Key6, KeyModifiers { control: true, .. }) |
                (Key::Key7, KeyModifiers { control: true, .. }) |
                (Key::Key8, KeyModifiers { control: true, .. }) |
                (Key::Key9, KeyModifiers { control: true, .. }) => {
                    // Ctrl+1-9: Switch to tab by number
                    let tab_number = match key {
                        Key::Key1 => 1, Key::Key2 => 2, Key::Key3 => 3, Key::Key4 => 4, Key::Key5 => 5,
                        Key::Key6 => 6, Key::Key7 => 7, Key::Key8 => 8, Key::Key9 => 9,
                        _ => return Ok(()),
                    };
                    
                    let tabs: Vec<_> = ui.tab_bar.get_all_tabs().keys().cloned().collect();
                    if tab_number <= tabs.len() {
                        let tab_id = &tabs[tab_number - 1];
                        ui.switch_to_tab(tab_id)?;
                        info!("Switched to tab {} with Ctrl+{}", tab_id, tab_number);
                    }
                }
                (Key::ArrowLeft, KeyModifiers { alt: true, .. }) => {
                    // Alt+Left: Go back
                    if let Some(url) = ui.go_back()? {
                        info!("Navigated back with Alt+Left to: {}", url);
                    }
                }
                (Key::ArrowRight, KeyModifiers { alt: true, .. }) => {
                    // Alt+Right: Go forward
                    if let Some(url) = ui.go_forward()? {
                        info!("Navigated forward with Alt+Right to: {}", url);
                    }
                }
                _ => {
                    // Unhandled key combination
                    debug!("Unhandled key combination: {:?} with modifiers: {:?}", key, modifiers);
                }
            }
        }
        
        Ok(())
    }
    
    /// Handle key release events
    fn handle_key_release(&mut self, key: Key, _ui: &mut BrowserUI) -> VeloraResult<()> {
        if key == Key::Shift || key == Key::Control || key == Key::Alt || key == Key::Meta {
            // Modifier key released
            debug!("Modifier key released: {:?}", key);
        }
        
        Ok(())
    }
    
    /// Handle text input events
    fn handle_text_input(&mut self, text: String, ui: &mut BrowserUI) -> VeloraResult<()> {
        if self.url_input_focused {
            // Update URL input text
            ui.toolbar.set_url_input_text(text);
        }
        
        Ok(())
    }
    
    /// Perform hit testing at the given position
    fn hit_test(&self, position: Point, ui: &BrowserUI) -> HitTestResult {
        let (x, y) = (position.x, position.y);
        
        // Test tab bar area
        if y < ui.layout.tab_bar_height {
            return self.hit_test_tab_bar(x, y, ui);
        }
        
        // Test toolbar area
        if y < ui.layout.tab_bar_height + ui.layout.toolbar_height {
            return self.hit_test_toolbar(x, y, ui);
        }
        
        // Content area
        HitTestResult::ContentArea
    }
    
    /// Hit test the tab bar
    fn hit_test_tab_bar(&self, x: f32, y: f32, ui: &BrowserUI) -> HitTestResult {
        let mut x_offset = 0.0;
        
        for (tab_id, tab) in ui.tab_bar.get_all_tabs() {
            let tab_width = self.calculate_tab_width(&tab.title);
            
            if x >= x_offset && x < x_offset + tab_width {
                // Check if click is on close button
                let close_button_x = x_offset + tab_width - 20.0;
                if x >= close_button_x && x < close_button_x + 15.0 && y >= 5.0 && y < 20.0 {
                    return HitTestResult::Tab {
                        tab_id: tab_id.clone(),
                        is_close_button: true,
                    };
                }
                
                return HitTestResult::Tab {
                    tab_id: tab_id.clone(),
                    is_close_button: false,
                };
            }
            
            x_offset += tab_width;
        }
        
        // Check new tab button
        let new_tab_x = x_offset;
        if x >= new_tab_x && x < new_tab_x + 30.0 && y >= 0.0 && y < ui.layout.tab_bar_height {
            return HitTestResult::NewTabButton;
        }
        
        HitTestResult::None
    }
    
    /// Hit test the toolbar
    fn hit_test_toolbar(&self, x: f32, y: f32, ui: &BrowserUI) -> HitTestResult {
        let button_y = ui.layout.tab_bar_height + 10.0;
        let button_size = 30.0;
        let button_spacing = 40.0;
        
        // Test navigation buttons
        let back_x = 10.0;
        if x >= back_x && x < back_x + button_size && y >= button_y && y < button_y + button_size {
            return HitTestResult::NavigationButton {
                button_type: NavigationButtonType::Back,
            };
        }
        
        let forward_x = back_x + button_spacing;
        if x >= forward_x && x < forward_x + button_size && y >= button_y && y < button_y + button_size {
            return HitTestResult::NavigationButton {
                button_type: NavigationButtonType::Forward,
            };
        }
        
        let refresh_x = forward_x + button_spacing;
        if x >= refresh_x && x < refresh_x + button_size && y >= button_y && y < button_y + button_size {
            return HitTestResult::NavigationButton {
                button_type: NavigationButtonType::Refresh,
            };
        }
        
        // Test URL input field
        let url_input_x = refresh_x + button_spacing + 20.0;
        let url_input_width = ui.layout.window_size.width - (url_input_x + 10.0);
        if x >= url_input_x && x < url_input_x + url_input_width && y >= button_y && y < button_y + button_size {
            return HitTestResult::UrlInput;
        }
        
        HitTestResult::None
    }
    
    /// Calculate tab width (same logic as renderer)
    fn calculate_tab_width(&self, title: &str) -> f32 {
        let min_width = 120.0;
        let title_width = title.len() as f32 * 8.0;
        (min_width + title_width).min(200.0)
    }
    
    /// Get current mouse position
    pub fn mouse_position(&self) -> Point {
        self.mouse_position
    }
    
    /// Check if URL input is focused
    pub fn is_url_input_focused(&self) -> bool {
        self.url_input_focused
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_input_handler_creation() {
        let handler = InputHandler::new();
        assert_eq!(handler.mouse_position, Point::new(0.0, 0.0));
        assert!(!handler.left_mouse_pressed);
        assert!(!handler.url_input_focused);
    }
    
    #[test]
    fn test_key_modifiers() {
        let mut modifiers = KeyModifiers::default();
        assert!(!modifiers.shift);
        assert!(!modifiers.control);
        assert!(!modifiers.alt);
        assert!(!modifiers.meta);
        
        modifiers.shift = true;
        modifiers.control = true;
        assert!(modifiers.shift);
        assert!(modifiers.control);
    }
    
    #[test]
    fn test_hit_test_result() {
        let result = HitTestResult::Tab {
            tab_id: "tab_1".to_string(),
            is_close_button: false,
        };
        
        match result {
            HitTestResult::Tab { tab_id, is_close_button } => {
                assert_eq!(tab_id, "tab_1");
                assert!(!is_close_button);
            }
            _ => panic!("Expected Tab result"),
        }
    }
}
