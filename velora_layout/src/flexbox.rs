//! Flexbox layout implementation for the Velora web engine

use velora_core::{Size, Point, Rect, VeloraResult};

/// Flex container properties
#[derive(Debug, Clone)]
pub struct FlexContainer {
    /// Flex direction
    pub direction: velora_core::FlexDirection,
    
    /// Justify content alignment
    pub justify_content: velora_core::JustifyContent,
    
    /// Align items alignment
    pub align_items: velora_core::AlignItems,
    
    /// Whether items wrap to new lines
    pub wrap: bool,
}

/// Flex item properties
#[derive(Debug, Clone)]
pub struct FlexItem {
    /// Flex grow factor
    pub flex_grow: f32,
    
    /// Flex shrink factor
    pub flex_shrink: f32,
    
    /// Flex basis
    pub flex_basis: Option<f32>,
    
    /// Align self alignment
    pub align_self: velora_core::AlignItems,
}

impl FlexContainer {
    /// Create a new flex container
    pub fn new() -> Self {
        Self {
            direction: velora_core::FlexDirection::Row,
            justify_content: velora_core::JustifyContent::FlexStart,
            align_items: velora_core::AlignItems::Stretch,
            wrap: false,
        }
    }
}

impl FlexItem {
    /// Create a new flex item
    pub fn new() -> Self {
        Self {
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            align_self: velora_core::AlignItems::Stretch,
        }
    }
}

impl Default for FlexContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FlexItem {
    fn default() -> Self {
        Self::new()
    }
}

/// Flexbox layout calculator
#[derive(Debug, Clone)]
pub struct FlexboxLayout {
    /// Container properties
    container: FlexContainer,
    /// Items to layout
    items: Vec<FlexItem>,
}

impl FlexboxLayout {
    /// Create a new flexbox layout calculator
    pub fn new(container: FlexContainer) -> Self {
        Self {
            container,
            items: Vec::new(),
        }
    }
    
    /// Add an item to the layout
    pub fn add_item(&mut self, item: FlexItem) {
        self.items.push(item);
    }
    
    /// Calculate the layout for all items
    pub fn calculate_layout(&self, _container_size: Size) -> VeloraResult<Vec<Rect>> {
        if self.items.is_empty() {
            return Ok(vec![]);
        }
        
        // TODO: Implement actual flexbox layout calculation
        // For now, return a simple stacked layout
        
        let mut results = Vec::new();
        let mut current_pos = Point::new(0.0, 0.0);
        
        for item in &self.items {
            let item_size = Size::new(
                item.flex_basis.unwrap_or(100.0),
                50.0 // Default height
            );
            
            let rect = Rect::from_point_size(current_pos, item_size);
            results.push(rect);
            
            // Move to next position based on direction
            match self.container.direction {
                velora_core::FlexDirection::Row | velora_core::FlexDirection::RowReverse => {
                    current_pos.x += item_size.width;
                }
                velora_core::FlexDirection::Column | velora_core::FlexDirection::ColumnReverse => {
                    current_pos.y += item_size.height;
                }
            }
        }
        
        Ok(results)
    }
    
    /// Get the container properties
    pub fn container(&self) -> &FlexContainer {
        &self.container
    }
    
    /// Get the items
    pub fn items(&self) -> &[FlexItem] {
        &self.items
    }
    
    /// Get the number of items
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_flex_container_creation() {
        let container = FlexContainer::new();
        assert_eq!(container.direction, velora_core::FlexDirection::Row);
        assert_eq!(container.justify_content, velora_core::JustifyContent::FlexStart);
        assert_eq!(container.align_items, velora_core::AlignItems::Stretch);
        assert!(!container.wrap);
    }
    
    #[test]
    fn test_flex_item_creation() {
        let item = FlexItem::new();
        assert_eq!(item.flex_grow, 0.0);
        assert_eq!(item.flex_shrink, 1.0);
        assert_eq!(item.flex_basis, None);
        assert_eq!(item.align_self, velora_core::AlignItems::Stretch);
    }
    
    #[test]
    fn test_flexbox_layout_creation() {
        let container = FlexContainer::new();
        let layout = FlexboxLayout::new(container);
        
        assert_eq!(layout.item_count(), 0);
        assert_eq!(layout.container().direction, velora_core::FlexDirection::Row);
    }
    
    #[test]
    fn test_flexbox_layout_add_item() {
        let container = FlexContainer::new();
        let mut layout = FlexboxLayout::new(container);
        
        let item = FlexItem::new();
        layout.add_item(item);
        
        assert_eq!(layout.item_count(), 1);
    }
    
    #[test]
    fn test_flexbox_layout_calculation() {
        let container = FlexContainer::new();
        let mut layout = FlexboxLayout::new(container);
        
        let item = FlexItem::new();
        layout.add_item(item);
        
        let container_size = Size::new(200.0, 100.0);
        let result = layout.calculate_layout(container_size);
        
        assert!(result.is_ok());
        let rects = result.unwrap();
        assert_eq!(rects.len(), 1);
        
        // Check first item position
        let first_rect = &rects[0];
        assert_eq!(first_rect.x, 0.0);
        assert_eq!(first_rect.y, 0.0);
    }
    
    #[test]
    fn test_flexbox_layout_empty() {
        let container = FlexContainer::new();
        let layout = FlexboxLayout::new(container);
        let container_size = Size::new(200.0, 100.0);
        let result = layout.calculate_layout(container_size);
        
        assert!(result.is_ok());
        let rects = result.unwrap();
        assert_eq!(rects.len(), 0);
    }
}
