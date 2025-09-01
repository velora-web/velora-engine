//! Buffer management for graphics rendering

use wgpu::*;
use velora_core::VeloraResult;
use super::vertex::Vertex;

/// Buffer manager for graphics rendering
pub struct BufferManager {
    /// Vertex buffer for rendering
    pub vertex_buffer: Option<Buffer>,
    
    /// Index buffer for rendering
    pub index_buffer: Option<Buffer>,
}

impl BufferManager {
    /// Create a new buffer manager
    pub fn new() -> Self {
        Self {
            vertex_buffer: None,
            index_buffer: None,
        }
    }
}

impl Default for BufferManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferManager {
    /// Create basic vertex and index buffers for a quad
    pub fn create_basic_buffers(&mut self, device: &Device, queue: &Queue) -> VeloraResult<()> {
        // Create a simple colored box (two triangles)
        let vertices = [
            // Top-left triangle
            Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0, 1.0]), // Red
            Vertex::new([ 0.5, -0.5, 0.0], [0.0, 1.0, 0.0, 1.0]), // Green
            Vertex::new([ 0.5,  0.5, 0.0], [0.0, 0.0, 1.0, 1.0]), // Blue
            // Bottom-right triangle
            Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0, 1.0]), // Red
            Vertex::new([ 0.5,  0.5, 0.0], [0.0, 0.0, 1.0, 1.0]), // Blue
            Vertex::new([-0.5,  0.5, 0.0], [1.0, 1.0, 0.0, 1.0]), // Yellow
        ];
        
        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (vertices.len() * std::mem::size_of::<Vertex>()) as BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Upload vertex data
        queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        self.vertex_buffer = Some(vertex_buffer);
        
        // Create index buffer (not strictly needed for this simple case, but good practice)
        let indices: [u16; 6] = [0, 1, 2, 3, 4, 5];
        let index_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Index Buffer"),
            size: (indices.len() * std::mem::size_of::<u16>()) as BufferAddress,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Upload index data
        queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(&indices));
        
        self.index_buffer = Some(index_buffer);
        
        Ok(())
    }
    
    /// Update vertex buffer with new data
    pub fn update_vertex_buffer(&self, queue: &Queue, vertices: &[Vertex]) {
        if let Some(vertex_buffer) = &self.vertex_buffer {
            queue.write_buffer(vertex_buffer, 0, bytemuck::cast_slice(vertices));
        }
    }
}
