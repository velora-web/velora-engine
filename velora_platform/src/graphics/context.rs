//! Main graphics context for the Velora web engine

use velora_core::{VeloraResult, Size};
use std::sync::Arc;
use crate::window::Window;
use super::GraphicsConfig;
use super::buffers::BufferManager;
use super::pipeline::PipelineManager;
use log::info;
use wgpu::*;

/// WGPU-based graphics context for rendering
pub struct GraphicsContext<'a> {
    /// Graphics configuration
    config: GraphicsConfig,
    
    /// Current window size
    size: Size,
    
    /// WGPU instance
    instance: Instance,
    
    /// WGPU adapter
    adapter: Adapter,
    
    /// WGPU device
    device: Device,
    
    /// WGPU queue
    queue: Queue,
    
    /// WGPU surface
    surface: Option<Surface<'a>>,
    
    /// WGPU surface configuration
    surface_config: Option<SurfaceConfiguration>,
    
    /// Buffer manager
    buffer_manager: BufferManager,
    
    /// Pipeline manager
    pipeline_manager: PipelineManager,
    
    /// Current clear color
    clear_color: [f32; 4],
    
    /// Whether we have a real surface (window)
    has_surface: bool,
    
    /// Whether the context needs to be redrawn
    needs_redraw: bool,
    
    /// Depth buffer for proper rendering
    depth_buffer: Option<Texture>,
    depth_buffer_view: Option<TextureView>,
}

impl<'a> GraphicsContext<'a> {
    /// Create a new graphics context
    pub async fn new() -> VeloraResult<Self> {
        let config = GraphicsConfig::default();
        Self::with_config(config).await
    }
    
    /// Create a new graphics context with custom configuration
    pub async fn with_config(config: GraphicsConfig) -> VeloraResult<Self> {
        info!("Creating wgpu graphics context with config: {:?}", config);
        
        // Create wgpu instance with minimal configuration
        let instance = Instance::new(&InstanceDescriptor::default());
        
        info!("WGPU instance created successfully");
        
        // Request adapter
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .map_err(|e| velora_core::VeloraError::Unknown(format!("Failed to find suitable adapter: {}", e)))?;
        
        info!("WGPU adapter selected");
        
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor::default(),
            )
            .await
            .map_err(|e| velora_core::VeloraError::Unknown(format!("Failed to create device: {}", e)))?;
        
        info!("WGPU device and queue created successfully");
        
        Ok(Self {
            config,
            size: Size::new(0.0, 0.0),
            instance,
            adapter,
            device,
            queue,
            surface: None,
            surface_config: None,
            buffer_manager: BufferManager::new(),
            pipeline_manager: PipelineManager::new(),
            clear_color: [0.1, 0.2, 0.3, 1.0],
            has_surface: false,
            needs_redraw: false,
            depth_buffer: None,
            depth_buffer_view: None,
        })
    }
    
    /// Initialize the graphics context with a window
    pub async fn initialize(&mut self, window: &'a Arc<Window>, window_size: Size) -> VeloraResult<()> {
        info!("Initializing wgpu graphics context for window size: {}x{}", window_size.width, window_size.height);
        
        self.size = window_size;
        
        // Create surface from window using the winit window directly
        let winit_window = window.inner();
        let surface = self.instance.create_surface(winit_window)
            .map_err(|e| velora_core::VeloraError::Unknown(format!("Failed to create surface: {}", e)))?;
        self.surface = Some(surface);
        
        // Configure surface
        self.configure_surface()?;
        
        // Create depth buffer
        self.create_depth_buffer()?;
        
        // Create render pipeline
        let surface_format = self.surface_config.as_ref()
            .map(|config| config.format)
            .unwrap_or(TextureFormat::Bgra8UnormSrgb);
        self.pipeline_manager.create_render_pipeline(&self.device, surface_format)?;
        
        // Create basic vertex and index buffers for a quad
        self.buffer_manager.create_basic_buffers(&self.device, &self.queue)?;
        
        self.has_surface = true;
        self.needs_redraw = true;
        
        info!("WGPU graphics context initialized successfully");
        Ok(())
    }
    
    /// Configure the surface for rendering
    fn configure_surface(&mut self) -> VeloraResult<()> {
        let surface = self.surface.as_ref()
            .ok_or_else(|| velora_core::VeloraError::Unknown("Surface not created".into()))?;
        
        let surface_caps = surface.get_capabilities(&self.adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: self.size.width as u32,
            height: self.size.height as u32,
            present_mode: if self.config.vsync { 
                PresentMode::Fifo 
            } else { 
                PresentMode::Immediate 
            },
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        surface.configure(&self.device, &config);
        self.surface_config = Some(config);
        
        Ok(())
    }
    
    /// Create depth buffer for proper rendering
    fn create_depth_buffer(&mut self) -> VeloraResult<()> {
        let depth_texture = self.device.create_texture(&TextureDescriptor {
            size: Extent3d {
                width: self.size.width as u32,
                height: self.size.height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
            label: Some("depth_texture"),
            view_formats: &[],
        });
        
        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());
        
        self.depth_buffer = Some(depth_texture);
        self.depth_buffer_view = Some(depth_view);
        
        Ok(())
    }
    
    /// Get the current configuration
    pub fn config(&self) -> &GraphicsConfig {
        &self.config
    }
    
    /// Get the current size
    pub fn size(&self) -> Size {
        self.size
    }
    
    /// Set the size (for testing purposes)
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    
    /// Clear the screen with a color
    pub fn clear(&mut self, color: u32) {
        // Convert u32 color to RGBA float values
        let r = ((color >> 16) & 0xFF) as f32 / 255.0;
        let g = ((color >> 8) & 0xFF) as f32 / 255.0;
        let b = (color & 0xFF) as f32 / 255.0;
        let a = ((color >> 24) & 0xFF) as f32 / 255.0;
        
        self.clear_color = [r, g, b, a];
        info!("Clear requested with color: rgba({}, {}, {}, {})", r, g, b, a);
    }
    
    /// Present the frame to the screen
    pub fn present(&mut self) -> VeloraResult<()> {
        if !self.has_surface {
            return Err(velora_core::VeloraError::Unknown("No surface available for rendering".into()));
        }
        
        let surface = self.surface.as_ref()
            .ok_or_else(|| velora_core::VeloraError::Unknown("Surface not available".into()))?;
        
        let frame = surface.get_current_texture()
            .map_err(|e| velora_core::VeloraError::Unknown(format!("Failed to get current texture: {}", e)))?;
        
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: self.clear_color[0] as f64,
                            g: self.clear_color[1] as f64,
                            b: self.clear_color[2] as f64,
                            a: self.clear_color[3] as f64,
                        }),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: self.depth_buffer_view.as_ref()
                        .ok_or_else(|| velora_core::VeloraError::Unknown("Depth buffer not available".into()))?,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            
            if let Some(pipeline) = &self.pipeline_manager.render_pipeline {
                render_pass.set_pipeline(pipeline);
                
                if let Some(vertex_buffer) = &self.buffer_manager.vertex_buffer {
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    
                    if let Some(index_buffer) = &self.buffer_manager.index_buffer {
                        render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
                        render_pass.draw_indexed(0..6, 0, 0..1);
                    } else {
                        render_pass.draw(0..6, 0..1);
                    }
                }
            }
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        
        info!("Frame presented successfully with WGPU rendering");
        Ok(())
    }
    
    /// Resize the graphics context
    pub fn resize(&mut self, new_size: Size) -> VeloraResult<()> {
        info!("Resizing wgpu graphics context to: {}x{}", new_size.width, new_size.height);
        
        self.size = new_size;
        
        if let Some(surface) = &self.surface {
            // Reconfigure surface with new size
            if let Some(mut config) = self.surface_config.clone() {
                config.width = new_size.width as u32;
                config.height = new_size.height as u32;
                surface.configure(&self.device, &config);
                self.surface_config = Some(config);
            }
            
            // Recreate depth buffer with new size
            self.create_depth_buffer()?;
        }
        
        Ok(())
    }
    
    /// Check if redraw is needed
    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }
    
    /// Mark that redraw is no longer needed
    pub fn mark_redrawn(&mut self) {
        self.needs_redraw = false;
    }
}
