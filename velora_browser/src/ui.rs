//! Browser UI components for tabs, navigation, and URL input

use velora_core::{VeloraResult, Size};
use std::collections::HashMap;
use log::{debug, info};

/// Tab information
#[derive(Debug, Clone)]
pub struct Tab {
    /// Unique identifier for the tab
    pub id: String,
    
    /// Tab title (usually the page title)
    pub title: String,
    
    /// Current URL
    pub url: String,
    
    /// Whether the tab is loading
    pub loading: bool,
    
    /// Whether the tab can go back
    pub can_go_back: bool,
    
    /// Whether the tab can go forward
    pub can_go_forward: bool,
    
    /// Tab history for back/forward navigation
    pub history: Vec<String>,
    
    /// Current position in history
    pub history_index: usize,
}

impl Tab {
    /// Create a new tab
    pub fn new(id: String, url: String) -> Self {
        let history = vec![url.clone()];
        
        Self {
            id,
            title: "New Tab".to_string(),
            url,
            loading: false,
            can_go_back: false,
            can_go_forward: false,
            history,
            history_index: 0,
        }
    }
    
    /// Navigate to a new URL
    pub fn navigate_to(&mut self, url: String) {
        // Add current URL to history if it's different
        if self.url != url {
            // Truncate history from current position
            self.history.truncate(self.history_index + 1);
            self.history.push(url.clone());
            self.history_index = self.history.len() - 1;
            
            self.url = url;
            self.loading = true;
            self.update_navigation_state();
        }
    }
    
    /// Go back in history
    pub fn go_back(&mut self) -> Option<String> {
        if self.can_go_back {
            self.history_index = self.history_index.saturating_sub(1);
            let url = self.history[self.history_index].clone();
            self.url = url.clone();
            self.update_navigation_state();
            Some(url)
        } else {
            None
        }
    }
    
    /// Go forward in history
    pub fn go_forward(&mut self) -> Option<String> {
        if self.can_go_forward {
            self.history_index = (self.history_index + 1).min(self.history.len() - 1);
            let url = self.history[self.history_index].clone();
            self.url = url.clone();
            self.update_navigation_state();
            Some(url)
        } else {
            None
        }
    }
    
    /// Update navigation state based on history
    fn update_navigation_state(&mut self) {
        self.can_go_back = self.history_index > 0;
        self.can_go_forward = self.history_index < self.history.len() - 1;
    }
    
    /// Set loading state
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }
    
    /// Set tab title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
}

/// Browser toolbar with navigation controls
#[derive(Debug)]
pub struct BrowserToolbar {
    /// Back button state
    pub back_enabled: bool,
    
    /// Forward button state
    pub forward_enabled: bool,
    
    /// Refresh button state
    pub refresh_enabled: bool,
    
    /// Current URL being displayed
    pub current_url: String,
    
    /// URL input field text
    pub url_input_text: String,
    
    /// Whether the URL input is focused
    pub url_input_focused: bool,
}

impl BrowserToolbar {
    /// Create a new toolbar
    pub fn new() -> Self {
        Self {
            back_enabled: false,
            forward_enabled: false,
            refresh_enabled: true,
            current_url: String::new(),
            url_input_text: String::new(),
            url_input_focused: false,
        }
    }
    
    /// Update toolbar state based on current tab
    pub fn update_for_tab(&mut self, tab: &Tab) {
        self.back_enabled = tab.can_go_back;
        self.forward_enabled = tab.can_go_forward;
        self.refresh_enabled = true;
        self.current_url = tab.url.clone();
        self.url_input_text = tab.url.clone();
    }
    
    /// Set URL input text
    pub fn set_url_input_text(&mut self, text: String) {
        self.url_input_text = text;
    }
    
    /// Set URL input focus state
    pub fn set_url_input_focused(&mut self, focused: bool) {
        self.url_input_focused = focused;
    }
}

/// Tab bar for managing multiple tabs
#[derive(Debug)]
pub struct TabBar {
    /// All tabs
    pub tabs: HashMap<String, Tab>,
    
    /// Currently active tab ID
    pub active_tab_id: Option<String>,
    
    /// Next tab ID to assign
    pub next_tab_id: u32,
}

impl TabBar {
    /// Create a new tab bar
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
            active_tab_id: None,
            next_tab_id: 1,
        }
    }
    
    /// Create a new tab
    pub fn create_tab(&mut self, url: String) -> String {
        let tab_id = format!("tab_{}", self.next_tab_id);
        self.next_tab_id += 1;
        
        let tab = Tab::new(tab_id.clone(), url);
        self.tabs.insert(tab_id.clone(), tab);
        
        // Set as active if it's the first tab
        if self.active_tab_id.is_none() {
            self.active_tab_id = Some(tab_id.clone());
        }
        
        tab_id
    }
    
    /// Close a tab
    pub fn close_tab(&mut self, tab_id: &str) -> VeloraResult<()> {
        if let Some(tab) = self.tabs.remove(tab_id) {
            // If we're closing the active tab, switch to another one
            if self.active_tab_id.as_ref() == Some(&tab_id.to_string()) {
                self.active_tab_id = self.tabs.keys().next().cloned();
            }
            
            info!("Closed tab: {}", tab.title);
            Ok(())
        } else {
            Err(velora_core::VeloraError::InvalidState(
                format!("Tab not found: {}", tab_id)
            ))
        }
    }
    
    /// Switch to a tab
    pub fn switch_to_tab(&mut self, tab_id: &str) -> VeloraResult<()> {
        if self.tabs.contains_key(tab_id) {
            self.active_tab_id = Some(tab_id.to_string());
            debug!("Switched to tab: {}", tab_id);
            Ok(())
        } else {
            Err(velora_core::VeloraError::InvalidState(
                format!("Tab not found: {}", tab_id)
            ))
        }
    }
    
    /// Get the active tab
    pub fn get_active_tab(&self) -> Option<&Tab> {
        self.active_tab_id
            .as_ref()
            .and_then(|id| self.tabs.get(id))
    }
    
    /// Get a mutable reference to the active tab
    pub fn get_active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.active_tab_id
            .as_ref()
            .and_then(|id| self.tabs.get_mut(id))
    }
    
    /// Get tab by ID
    pub fn get_tab(&self, tab_id: &str) -> Option<&Tab> {
        self.tabs.get(tab_id)
    }
    
    /// Get mutable tab by ID
    pub fn get_tab_mut(&mut self, tab_id: &str) -> Option<&mut Tab> {
        self.tabs.get_mut(tab_id)
    }
    
    /// Get all tabs
    pub fn get_all_tabs(&self) -> &HashMap<String, Tab> {
        &self.tabs
    }
    
    /// Get tab count
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }
}

/// Browser UI manager
#[derive(Debug)]
pub struct BrowserUI {
    /// Tab bar
    pub tab_bar: TabBar,
    
    /// Toolbar
    pub toolbar: BrowserToolbar,
    
    /// UI dimensions and layout
    pub layout: UILayout,
}

impl BrowserUI {
    /// Create a new browser UI
    pub fn new() -> Self {
        let mut ui = Self {
            tab_bar: TabBar::new(),
            toolbar: BrowserToolbar::new(),
            layout: UILayout::default(),
        };
        
        // Create initial tab
        ui.tab_bar.create_tab("about:blank".to_string());
        
        ui
    }
    
    /// Create a new tab
    pub fn create_tab(&mut self, url: String) -> String {
        let tab_id = self.tab_bar.create_tab(url);
        
        // Update toolbar for new tab
        if let Some(tab) = self.tab_bar.get_tab(&tab_id) {
            self.toolbar.update_for_tab(tab);
        }
        
        tab_id
    }
    
    /// Close the current tab
    pub fn close_current_tab(&mut self) -> VeloraResult<()> {
        if let Some(active_id) = &self.tab_bar.active_tab_id {
            let active_id_clone = active_id.clone();
            self.tab_bar.close_tab(&active_id_clone)?;
            
            // Update toolbar for new active tab
            if let Some(tab) = self.tab_bar.get_active_tab() {
                self.toolbar.update_for_tab(tab);
            }
        }
        
        Ok(())
    }
    
    /// Navigate to URL in current tab
    pub fn navigate_current_tab(&mut self, url: String) -> VeloraResult<()> {
        if let Some(tab) = self.tab_bar.get_active_tab_mut() {
            tab.navigate_to(url.clone());
            self.toolbar.update_for_tab(tab);
        }
        
        Ok(())
    }
    
    /// Go back in current tab
    pub fn go_back(&mut self) -> VeloraResult<Option<String>> {
        if let Some(tab) = self.tab_bar.get_active_tab_mut() {
            let url = tab.go_back();
            self.toolbar.update_for_tab(tab);
            Ok(url)
        } else {
            Ok(None)
        }
    }
    
    /// Go forward in current tab
    pub fn go_forward(&mut self) -> VeloraResult<Option<String>> {
        if let Some(tab) = self.tab_bar.get_active_tab_mut() {
            let url = tab.go_forward();
            self.toolbar.update_for_tab(tab);
            Ok(url)
        } else {
            Ok(None)
        }
    }
    
    /// Refresh current tab
    pub fn refresh_current_tab(&mut self) -> VeloraResult<()> {
        if let Some(tab) = self.tab_bar.get_active_tab_mut() {
            let current_url = tab.url.clone();
            tab.navigate_to(current_url);
            self.toolbar.update_for_tab(tab);
        }
        
        Ok(())
    }
    
    /// Switch to a specific tab
    pub fn switch_to_tab(&mut self, tab_id: &str) -> VeloraResult<()> {
        self.tab_bar.switch_to_tab(tab_id)?;
        
        // Update toolbar for new active tab
        if let Some(tab) = self.tab_bar.get_active_tab() {
            self.toolbar.update_for_tab(tab);
        }
        
        Ok(())
    }
    
    /// Update UI layout
    pub fn update_layout(&mut self, window_size: Size) {
        self.layout.update(window_size);
    }
}

/// UI layout information
#[derive(Debug, Clone)]
pub struct UILayout {
    /// Window size
    pub window_size: Size,
    
    /// Tab bar height
    pub tab_bar_height: f32,
    
    /// Toolbar height
    pub toolbar_height: f32,
    
    /// Content area
    pub content_area: ContentArea,
}

impl Default for UILayout {
    fn default() -> Self {
        Self {
            window_size: Size::new(1280.0, 720.0),
            tab_bar_height: 40.0,
            toolbar_height: 50.0,
            content_area: ContentArea::default(),
        }
    }
}

impl UILayout {
    /// Update layout for new window size
    pub fn update(&mut self, window_size: Size) {
        self.window_size = window_size;
        self.content_area.update(window_size, self.tab_bar_height, self.toolbar_height);
    }
}

/// Content area layout
#[derive(Debug, Clone)]
pub struct ContentArea {
    /// Position and size of the content area
    pub rect: (f32, f32, f32, f32), // (x, y, width, height)
}

impl Default for ContentArea {
    fn default() -> Self {
        Self {
            rect: (0.0, 90.0, 1280.0, 630.0), // Below tab bar and toolbar
        }
    }
}

impl ContentArea {
    /// Update content area for new dimensions
    pub fn update(&mut self, window_size: Size, tab_bar_height: f32, toolbar_height: f32) {
        let x = 0.0;
        let y = tab_bar_height + toolbar_height;
        let width = window_size.width;
        let height = window_size.height - y;
        
        self.rect = (x, y, width, height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tab_creation() {
        let mut tab_bar = TabBar::new();
        let tab_id = tab_bar.create_tab("https://example.com".to_string());
        
        assert!(tab_bar.tabs.contains_key(&tab_id));
        assert_eq!(tab_bar.active_tab_id, Some(tab_id));
    }
    
    #[test]
    fn test_tab_navigation() {
        let mut tab = Tab::new("tab_1".to_string(), "https://example.com".to_string());
        
        // Navigate to new URL
        tab.navigate_to("https://google.com".to_string());
        assert_eq!(tab.url, "https://google.com");
        assert_eq!(tab.history.len(), 2);
        assert_eq!(tab.history_index, 1);
        
        // Go back
        let back_url = tab.go_back();
        assert_eq!(back_url, Some("https://example.com".to_string()));
        assert_eq!(tab.history_index, 0);
    }
    
    #[test]
    fn test_browser_ui() {
        let mut ui = BrowserUI::new();
        
        // Should have one initial tab
        assert_eq!(ui.tab_bar.tab_count(), 1);
        assert!(ui.tab_bar.active_tab_id.is_some());
        
        // Create new tab
        let tab_id = ui.create_tab("https://example.com".to_string());
        assert_eq!(ui.tab_bar.tab_count(), 2);
        
        // Switch to new tab
        ui.switch_to_tab(&tab_id).unwrap();
        assert_eq!(ui.tab_bar.active_tab_id, Some(tab_id));
    }
}
