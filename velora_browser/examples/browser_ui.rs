//! Browser UI example demonstrating tabs, toolbar, and content rendering

use velora_browser::browser::Browser;
use velora_core::{VeloraResult, Size};
use log::info;

fn main() -> VeloraResult<()> {
    // Initialize logging
    env_logger::init();
    
    info!("🚀 Starting Velora Browser UI Example");
    
    // Create a browser instance with default configuration
    let mut browser = Browser::new_default();
    
    // Initialize UI components
    browser.initialize_ui()?;
    
    // Create some example tabs
    let tab1_id = browser.create_tab("https://example.com".to_string());
    let tab2_id = browser.create_tab("https://google.com".to_string());
    let tab3_id = browser.create_tab("https://github.com".to_string());
    
    info!("Created tabs: {}, {}, {}", tab1_id, tab2_id, tab3_id);
    
    // Switch between tabs
    browser.switch_to_tab(&tab2_id)?;
    info!("Switched to tab: {}", tab2_id);
    
    // Navigate in current tab
    browser.navigate_current_tab("https://rust-lang.org".to_string())?;
    info!("Navigated to rust-lang.org");
    
    // Go back
    if let Some(url) = browser.go_back()? {
        info!("Went back to: {}", url);
    }
    
    // Go forward
    if let Some(url) = browser.go_forward()? {
        info!("Went forward to: {}", url);
    }
    
    // Refresh current tab
    browser.refresh_current_tab()?;
    info!("Refreshed current tab");
    
    // Close a tab
    browser.close_current_tab()?;
    info!("Closed current tab");
    
    // Demonstrate tab information
    info!("Current tab information:");
    if let Some(current_tab) = browser.get_current_tab() {
        info!("  Current tab: '{}' at {}", current_tab.title, current_tab.url);
        info!("  Can go back: {}", current_tab.can_go_back);
        info!("  Can go forward: {}", current_tab.can_go_forward);
        info!("  Loading: {}", current_tab.loading);
    }
    
    info!("Total tabs: {}", browser.get_tab_count());
    
    // Create a test image to demonstrate UI rendering
    browser.create_test_image("browser_ui_demo.png")?;
    info!("Created test UI image");
    
    // Handle window resize
    let new_size = Size::new(1600.0, 900.0);
    browser.handle_window_resize(new_size)?;
    info!("Resized window to {}x{}", new_size.width, new_size.height);
    
    // Render the UI
    browser.render_ui()?;
    info!("Rendered UI");
    
    info!("✅ Browser UI example completed successfully!");
    
    Ok(())
}
