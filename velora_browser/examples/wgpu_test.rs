//! Simple example to test wgpu graphics context

use velora_core::Size;
use velora_platform::graphics::GraphicsContext;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    info!("🚀 Testing wgpu graphics context...");
    
    // Create a wgpu graphics context
    let mut graphics = GraphicsContext::new().await?;
    info!("✅ WGPU graphics context created successfully");
    
    // Initialize with a dummy size (we don't have a real window yet)
    let dummy_size = Size::new(800.0, 600.0);
    // For testing, we'll skip the window initialization since we don't have a real window
    graphics.set_size(dummy_size);
    info!("✅ Graphics context initialized with size: {}x{}", dummy_size.width, dummy_size.height);
    
    // Test basic operations
    graphics.clear(0xFF0000FF); // Red background
    info!("✅ Clear operation completed");
    
    graphics.present()?;
    info!("✅ Present operation completed");
    
    // Test resize
    let new_size = Size::new(1024.0, 768.0);
    graphics.resize(new_size)?;
    info!("✅ Resize operation completed to: {}x{}", new_size.width, new_size.height);
    
    info!("🎉 All wgpu graphics tests passed!");
    
    Ok(())
}
