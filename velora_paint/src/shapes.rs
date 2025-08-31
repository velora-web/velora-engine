//! Shape rendering for the Velora web engine

use velora_core::{VeloraResult, Point, Color, Rect};

/// Shape renderer for drawing geometric shapes
#[derive(Debug)]
pub struct ShapeRenderer {
    /// Current drawing color
    color: Color,
}

impl ShapeRenderer {
    /// Create a new shape renderer
    pub fn new() -> VeloraResult<Self> {
        Ok(Self {
            color: Color::black(),
        })
    }
    
    /// Set the drawing color
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    
    /// Draw a rectangle
    pub fn draw_rect(&self, _rect: Rect) -> VeloraResult<()> {
        // TODO: Implement rectangle drawing
        Ok(())
    }
    
    /// Draw a circle
    pub fn draw_circle(&self, _center: Point, _radius: f32) -> VeloraResult<()> {
        // TODO: Implement circle drawing
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shape_renderer_creation() {
        let renderer = ShapeRenderer::new();
        assert!(renderer.is_ok());
    }
    
    #[test]
    fn test_shape_renderer_color_setting() {
        let mut renderer = ShapeRenderer::new().unwrap();
        let new_color = Color::rgb(255, 0, 0);
        renderer.set_color(new_color);
        assert_eq!(renderer.color, new_color);
    }
    
    #[test]
    fn test_rectangle_drawing() {
        let renderer = ShapeRenderer::new().unwrap();
        let rect = Rect::new(0.0, 0.0, 100.0, 50.0);
        let result = renderer.draw_rect(rect);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_circle_drawing() {
        let renderer = ShapeRenderer::new().unwrap();
        let center = Point::new(50.0, 50.0);
        let radius = 25.0;
        let result = renderer.draw_circle(center, radius);
        assert!(result.is_ok());
    }
}
