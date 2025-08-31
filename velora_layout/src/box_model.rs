//! Box model implementation for the Velora web engine

use velora_core::{Size, Rect, VeloraResult, VeloraError};
use velora_core::error::LayoutError;

/// Box model with margin, border, padding, and content areas
#[derive(Debug, Clone)]
pub struct BoxModel {
    /// Content dimensions
    pub content: Rect,
    
    /// Padding dimensions
    pub padding: Rect,
    
    /// Border dimensions
    pub border: Rect,
    
    /// Margin dimensions
    pub margin: Rect,
}

/// Box sizing model
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

impl BoxModel {
    /// Create a new box model
    pub fn new(content: Rect) -> Self {
        Self {
            content,
            padding: Rect::zero(),
            border: Rect::zero(),
            margin: Rect::zero(),
        }
    }
    
    /// Get the total box dimensions
    pub fn total_size(&self) -> Size {
        Size::new(
            self.content.width + self.padding.width + self.border.width + self.margin.width,
            self.content.height + self.padding.height + self.border.height + self.margin.height,
        )
    }
    
    /// Set padding dimensions
    pub fn set_padding(&mut self, padding: Rect) {
        self.padding = padding;
    }
    
    /// Set border dimensions
    pub fn set_border(&mut self, border: Rect) {
        self.border = border;
    }
    
    /// Set margin dimensions
    pub fn set_margin(&mut self, margin: Rect) {
        self.margin = margin;
    }
    
    /// Get the content area
    pub fn content_area(&self) -> Rect {
        self.content
    }
    
    /// Get the padding box (content + padding)
    pub fn padding_box(&self) -> Rect {
        Rect::new(
            self.content.x - self.padding.x,
            self.content.y - self.padding.y,
            self.content.width + self.padding.width,
            self.content.height + self.padding.height,
        )
    }
    
    /// Get the border box (content + padding + border)
    pub fn border_box(&self) -> Rect {
        let padding_box = self.padding_box();
        Rect::new(
            padding_box.x - self.border.x,
            padding_box.y - self.border.y,
            padding_box.width + self.border.width,
            padding_box.height + self.border.height,
        )
    }
    
    /// Get the margin box (content + padding + border + margin)
    pub fn margin_box(&self) -> Rect {
        let border_box = self.border_box();
        Rect::new(
            border_box.x - self.margin.x,
            border_box.y - self.margin.y,
            border_box.width + self.margin.width,
            border_box.height + self.margin.height,
        )
    }
    
    /// Calculate box model with specific sizing
    pub fn calculate_with_sizing(&self, sizing: BoxSizing, available_size: Size) -> VeloraResult<Size> {
        match sizing {
            BoxSizing::ContentBox => {
                // Content box sizing: available size includes padding, border, and margin
                let content_width = available_size.width - self.padding.width - self.border.width - self.margin.width;
                let content_height = available_size.height - self.padding.height - self.border.height - self.margin.height;
                
                if content_width < 0.0 || content_height < 0.0 {
                    return Err(VeloraError::Layout(LayoutError::InvalidConstraints(
                        "Available size too small for box model".to_string()
                    )));
                }
                
                Ok(Size::new(content_width, content_height))
            }
            BoxSizing::BorderBox => {
                // Border box sizing: available size includes only margin
                let content_width = available_size.width - self.margin.width;
                let content_height = available_size.height - self.margin.height;
                
                if content_width < 0.0 || content_height < 0.0 {
                    return Err(VeloraError::Layout(LayoutError::InvalidConstraints(
                        "Available size too small for box model".to_string()
                    )));
                }
                
                Ok(Size::new(content_width, content_height))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_box_model_creation() {
        let content = Rect::new(10.0, 20.0, 100.0, 50.0);
        let box_model = BoxModel::new(content);
        
        assert_eq!(box_model.content, content);
        assert_eq!(box_model.padding, Rect::zero());
        assert_eq!(box_model.border, Rect::zero());
        assert_eq!(box_model.margin, Rect::zero());
    }
    
    #[test]
    fn test_total_size_calculation() {
        let content = Rect::new(0.0, 0.0, 100.0, 50.0);
        let mut box_model = BoxModel::new(content);
        
        // Set padding, border, and margin
        box_model.set_padding(Rect::new(0.0, 0.0, 20.0, 10.0));
        box_model.set_border(Rect::new(0.0, 0.0, 5.0, 5.0));
        box_model.set_margin(Rect::new(0.0, 0.0, 15.0, 8.0));
        
        let total_size = box_model.total_size();
        assert_eq!(total_size.width, 140.0); // 100 + 20 + 5 + 15
        assert_eq!(total_size.height, 73.0);  // 50 + 10 + 5 + 8
    }
    
    #[test]
    fn test_padding_box() {
        let content = Rect::new(10.0, 20.0, 100.0, 50.0);
        let mut box_model = BoxModel::new(content);
        box_model.set_padding(Rect::new(0.0, 0.0, 20.0, 10.0));
        
        let padding_box = box_model.padding_box();
        assert_eq!(padding_box.x, 10.0);
        assert_eq!(padding_box.y, 20.0);
        assert_eq!(padding_box.width, 120.0);
        assert_eq!(padding_box.height, 60.0);
    }
    
    #[test]
    fn test_border_box() {
        let content = Rect::new(10.0, 20.0, 100.0, 50.0);
        let mut box_model = BoxModel::new(content);
        box_model.set_padding(Rect::new(0.0, 0.0, 20.0, 10.0));
        box_model.set_border(Rect::new(0.0, 0.0, 5.0, 5.0));
        
        let border_box = box_model.border_box();
        assert_eq!(border_box.x, 10.0);
        assert_eq!(border_box.y, 20.0);
        assert_eq!(border_box.width, 125.0);
        assert_eq!(border_box.height, 65.0);
    }
    
    #[test]
    fn test_margin_box() {
        let content = Rect::new(10.0, 20.0, 100.0, 50.0);
        let mut box_model = BoxModel::new(content);
        box_model.set_padding(Rect::new(0.0, 0.0, 20.0, 10.0));
        box_model.set_border(Rect::new(0.0, 0.0, 5.0, 5.0));
        box_model.set_margin(Rect::new(0.0, 0.0, 15.0, 8.0));
        
        let margin_box = box_model.margin_box();
        assert_eq!(margin_box.x, 10.0);
        assert_eq!(margin_box.y, 20.0);
        assert_eq!(margin_box.width, 140.0);
        assert_eq!(margin_box.height, 73.0);
    }
    
    #[test]
    fn test_calculate_with_sizing_content_box() {
        let content = Rect::new(0.0, 0.0, 100.0, 50.0);
        let mut box_model = BoxModel::new(content);
        box_model.set_padding(Rect::new(0.0, 0.0, 20.0, 10.0));
        box_model.set_border(Rect::new(0.0, 0.0, 5.0, 5.0));
        box_model.set_margin(Rect::new(0.0, 0.0, 15.0, 8.0));
        
        let available_size = Size::new(200.0, 100.0);
        let result = box_model.calculate_with_sizing(BoxSizing::ContentBox, available_size);
        
        assert!(result.is_ok());
        let content_size = result.unwrap();
        assert_eq!(content_size.width, 160.0); // 200 - 20 - 5 - 15
        assert_eq!(content_size.height, 77.0);  // 100 - 10 - 5 - 8
    }
    
    #[test]
    fn test_calculate_with_sizing_border_box() {
        let content = Rect::new(0.0, 0.0, 100.0, 50.0);
        let mut box_model = BoxModel::new(content);
        box_model.set_margin(Rect::new(0.0, 0.0, 15.0, 8.0));
        
        let available_size = Size::new(200.0, 100.0);
        let result = box_model.calculate_with_sizing(BoxSizing::BorderBox, available_size);
        
        assert!(result.is_ok());
        let content_size = result.unwrap();
        assert_eq!(content_size.width, 185.0); // 200 - 15
        assert_eq!(content_size.height, 92.0);  // 100 - 8
    }
    
    #[test]
    fn test_calculate_with_sizing_invalid_constraints() {
        let content = Rect::new(0.0, 0.0, 100.0, 50.0);
        let mut box_model = BoxModel::new(content);
        box_model.set_padding(Rect::new(0.0, 0.0, 200.0, 100.0)); // Too large
        
        let available_size = Size::new(100.0, 50.0);
        let result = box_model.calculate_with_sizing(BoxSizing::ContentBox, available_size);
        
        assert!(result.is_err());
    }
}
