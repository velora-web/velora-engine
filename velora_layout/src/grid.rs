//! CSS Grid layout implementation for the Velora web engine

use velora_core::{Size, Rect, VeloraResult, VeloraError};
use velora_core::error::LayoutError;

/// Grid container properties
#[derive(Debug, Clone)]
pub struct GridContainer {
    /// Grid template columns
    pub template_columns: Vec<String>,
    
    /// Grid template rows
    pub template_rows: Vec<String>,
    
    /// Grid gap between columns
    pub column_gap: f32,
    
    /// Grid gap between rows
    pub row_gap: f32,
}

/// Grid item properties
#[derive(Debug, Clone)]
pub struct GridItem {
    /// Grid column start
    pub column_start: Option<i32>,
    
    /// Grid column end
    pub column_end: Option<i32>,
    
    /// Grid row start
    pub row_start: Option<i32>,
    
    /// Grid row end
    pub row_end: Option<i32>,
}

impl GridContainer {
    /// Create a new grid container
    pub fn new() -> Self {
        Self {
            template_columns: vec!["1fr".to_string()],
            template_rows: vec!["1fr".to_string()],
            column_gap: 0.0,
            row_gap: 0.0,
        }
    }
}

impl GridItem {
    /// Create a new grid item
    pub fn new() -> Self {
        Self {
            column_start: None,
            column_end: None,
            row_start: None,
            row_end: None,
        }
    }
}

impl Default for GridContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for GridItem {
    fn default() -> Self {
        Self::new()
    }
}

/// CSS Grid layout calculator
#[derive(Debug, Clone)]
pub struct GridLayout {
    /// Grid container properties
    container: GridContainer,
    /// Grid items to layout
    items: Vec<GridItem>,
}

impl GridLayout {
    /// Create a new grid layout calculator
    pub fn new(container: GridContainer) -> Self {
        Self {
            container,
            items: Vec::new(),
        }
    }
    
    /// Add an item to the grid
    pub fn add_item(&mut self, item: GridItem) {
        self.items.push(item);
    }
    
    /// Calculate the grid layout
    pub fn calculate_layout(&self, container_size: Size) -> VeloraResult<Vec<Rect>> {
        if self.items.is_empty() {
            return Ok(vec![]);
        }
        
        // TODO: Implement actual CSS Grid layout calculation
        // For now, return a simple grid layout
        
        let mut results = Vec::new();
        let columns = self.container.template_columns.len();
        let rows = self.container.template_rows.len();
        
        if columns == 0 || rows == 0 {
            return Err(VeloraError::Layout(LayoutError::InvalidConstraints(
                "Grid must have at least one column and one row".to_string()
            )));
        }
        
        let cell_width = (container_size.width - (columns as f32 - 1.0) * self.container.column_gap) / columns as f32;
        let cell_height = (container_size.height - (rows as f32 - 1.0) * self.container.row_gap) / rows as f32;
        
        for (index, _item) in self.items.iter().enumerate() {
            let col = index % columns;
            let row = index / columns;
            
            let x = col as f32 * (cell_width + self.container.column_gap);
            let y = row as f32 * (cell_height + self.container.row_gap);
            
            let rect = Rect::new(x, y, cell_width, cell_height);
            results.push(rect);
        }
        
        Ok(results)
    }
    
    /// Get the container properties
    pub fn container(&self) -> &GridContainer {
        &self.container
    }
    
    /// Get the items
    pub fn items(&self) -> &[GridItem] {
        &self.items
    }
    
    /// Get the number of items
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    
    /// Get the number of columns
    pub fn column_count(&self) -> usize {
        self.container.template_columns.len()
    }
    
    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        self.container.template_rows.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grid_container_creation() {
        let container = GridContainer::new();
        assert_eq!(container.template_columns.len(), 1);
        assert_eq!(container.template_rows.len(), 1);
        assert_eq!(container.column_gap, 0.0);
        assert_eq!(container.row_gap, 0.0);
    }
    
    #[test]
    fn test_grid_item_creation() {
        let item = GridItem::new();
        assert_eq!(item.column_start, None);
        assert_eq!(item.column_end, None);
        assert_eq!(item.row_start, None);
        assert_eq!(item.row_end, None);
    }
    
    #[test]
    fn test_grid_layout_creation() {
        let container = GridContainer::new();
        let layout = GridLayout::new(container);
        
        assert_eq!(layout.item_count(), 0);
        assert_eq!(layout.column_count(), 1);
        assert_eq!(layout.row_count(), 1);
    }
    
    #[test]
    fn test_grid_layout_add_item() {
        let container = GridContainer::new();
        let mut layout = GridLayout::new(container);
        
        let item = GridItem::new();
        layout.add_item(item);
        
        assert_eq!(layout.item_count(), 1);
    }
    
    #[test]
    fn test_grid_layout_calculation() {
        let container = GridContainer::new();
        let mut layout = GridLayout::new(container);
        
        let item = GridItem::new();
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
    fn test_grid_layout_empty() {
        let container = GridContainer::new();
        let layout = GridLayout::new(container);
        
        let container_size = Size::new(200.0, 100.0);
        let result = layout.calculate_layout(container_size);
        
        assert!(result.is_ok());
        let rects = result.unwrap();
        assert_eq!(rects.len(), 0);
    }
}
