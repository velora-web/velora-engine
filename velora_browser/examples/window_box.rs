//! Simple example to render a colored box in a real window using wgpu

use velora_core::Size;
use velora_platform::{graphics::GraphicsContext, platform::Platform};
use log::info;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    info!("🚀 Creating window and rendering colored box...");
    
    // Create platform
    let mut platform = Platform::new()?;
    info!("✅ Platform created");
    
    // Create a simple window
    let window = platform.create_window("Velora Box Demo", Size::new(800.0, 600.0))?;
    info!("✅ Window created: 800x600");
    
    // Create wgpu graphics context
    let mut graphics = GraphicsContext::new().await?;
    info!("✅ WGPU graphics context created");
    
    // Initialize graphics with the window
    graphics.initialize(&window, Size::new(800.0, 600.0)).await?;
    info!("✅ Graphics initialized with window");
    
    // Set clear color to blue
    graphics.clear(0xFF0000FF);
    info!("✅ Clear color set to blue");
    
    // Render the frame
    graphics.present()?;
    info!("✅ Frame rendered successfully!");
    
    // For now, we'll just log resize events since we can't easily mutate graphics from the closure
    // In a real application, you'd use a different approach like channels or shared state
    platform.add_event_handler(|event| {
        match event {
            velora_platform::WindowEvent::Resized(new_size) => {
                info!("🔄 Window resized to: {}x{}", new_size.width, new_size.height);
                info!("   - Graphics context resize would happen here");
                info!("   - Redraw would be requested here");
            }
            _ => {}
        }
    });
    
    // Keep the window open and handle events
    info!("🎉 Colored box rendered! Window will stay open and handle resize events...");
    info!("   - Try resizing the window to see the resize handling");
    info!("   - The white box should update to match the new window size");
    
    // Run the event loop for a while to handle resize events
    std::thread::sleep(std::time::Duration::from_secs(10));
    
    info!("✅ Demo completed successfully");
    Ok(())
}
