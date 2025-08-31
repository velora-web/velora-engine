//! Layout tree implementation for the Velora web engine

use velora_core::{NodeId, VeloraResult, VeloraError, Size, Rect};
use velora_core::error::LayoutError;
use super::{BoxModel, FlexboxLayout, GridLayout};
use std::collections::HashMap;

/// Layout node information
#[derive(Debug, Clone)]
pub struct LayoutNode {
    /// Node ID
    pub node_id: NodeId,
    /// Box model for this node
    pub box_model: BoxModel,
    /// Flexbox layout (if applicable)
    pub flexbox: Option<FlexboxLayout>,
    /// Grid layout (if applicable)
    pub grid: Option<GridLayout>,
    /// Parent node ID
    pub parent_id: Option<NodeId>,
    /// Child node IDs
    pub child_ids: Vec<NodeId>,
}

/// A layout tree that manages the layout of DOM elements
#[derive(Debug, Clone)]
pub struct LayoutTree {
    /// Root layout node ID
    root_id: Option<NodeId>,
    /// Layout nodes indexed by NodeId
    nodes: HashMap<NodeId, LayoutNode>,
}

impl LayoutTree {
    /// Create a new layout tree
    pub fn new() -> Self {
        Self { 
            root_id: None,
            nodes: HashMap::new(),
        }
    }
    
    /// Set the root layout node
    pub fn set_root(&mut self, node_id: NodeId) {
        self.root_id = Some(node_id);
    }
    
    /// Get the root layout node ID
    pub fn get_root(&self) -> Option<NodeId> {
        self.root_id
    }
    
    /// Add a layout node
    pub fn add_node(&mut self, node: LayoutNode) -> VeloraResult<()> {
        let node_id = node.node_id;
        
        if self.nodes.contains_key(&node_id) {
            return Err(VeloraError::Layout(LayoutError::InvalidConstraints(
                format!("Layout node {} already exists", node_id.0)
            )));
        }
        
        self.nodes.insert(node_id, node);
        
        // Set as root if it's the first node
        if self.root_id.is_none() {
            self.root_id = Some(node_id);
        }
        
        Ok(())
    }
    
    /// Get a layout node by ID
    pub fn get_node(&self, node_id: NodeId) -> Option<&LayoutNode> {
        self.nodes.get(&node_id)
    }
    
    /// Get a mutable reference to a layout node by ID
    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut LayoutNode> {
        self.nodes.get_mut(&node_id)
    }
    
    /// Remove a layout node
    pub fn remove_node(&mut self, node_id: NodeId) -> bool {
        self.nodes.remove(&node_id).is_some()
    }
    
    /// Get the number of nodes in the tree
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    /// Check if the tree is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    
    /// Calculate layout for all nodes
    pub fn calculate_layout(&self, container_size: Size) -> VeloraResult<HashMap<NodeId, Rect>> {
        let mut results = HashMap::new();
        
        if let Some(root_id) = self.root_id {
            self.calculate_node_layout(root_id, container_size, &mut results)?;
        }
        
        Ok(results)
    }
    
    /// Calculate layout for a specific node and its descendants
    fn calculate_node_layout(
        &self,
        node_id: NodeId,
        available_size: Size,
        results: &mut HashMap<NodeId, Rect>,
    ) -> VeloraResult<()> {
        let node = self.get_node(node_id)
            .ok_or_else(|| VeloraError::Layout(LayoutError::InvalidConstraints(
                format!("Layout node {} not found", node_id.0)
            )))?;
        
        // Calculate this node's layout
        let node_rect = if let Some(flexbox) = &node.flexbox {
            // Use flexbox layout
            let rects = flexbox.calculate_layout(available_size)?;
            if !rects.is_empty() {
                rects[0] // Take the first rect as the node's position
            } else {
                Rect::new(0.0, 0.0, available_size.width, available_size.height)
            }
        } else if let Some(grid) = &node.grid {
            // Use grid layout
            let rects = grid.calculate_layout(available_size)?;
            if !rects.is_empty() {
                rects[0] // Take the first rect as the node's position
            } else {
                Rect::new(0.0, 0.0, available_size.width, available_size.height)
            }
        } else {
            // Use box model layout
            let content_size = node.box_model.calculate_with_sizing(
                super::BoxSizing::ContentBox,
                available_size
            )?;
            Rect::new(0.0, 0.0, content_size.width, content_size.height)
        };
        
        results.insert(node_id, node_rect);
        
        // Calculate children layouts
        for &child_id in &node.child_ids {
            let child_size = Size::new(node_rect.width, node_rect.height);
            self.calculate_node_layout(child_id, child_size, results)?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_layout_tree_creation() {
        let tree = LayoutTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.node_count(), 0);
        assert_eq!(tree.get_root(), None);
    }
    
    #[test]
    fn test_add_layout_node() {
        let mut tree = LayoutTree::new();
        
        let box_model = BoxModel::new(Rect::new(0.0, 0.0, 100.0, 50.0));
        let node = LayoutNode {
            node_id: NodeId(1),
            box_model,
            flexbox: None,
            grid: None,
            parent_id: None,
            child_ids: vec![],
        };
        
        let result = tree.add_node(node);
        assert!(result.is_ok());
        assert_eq!(tree.node_count(), 1);
        assert!(!tree.is_empty());
        assert_eq!(tree.get_root(), Some(NodeId(1)));
    }
    
    #[test]
    fn test_add_duplicate_node() {
        let mut tree = LayoutTree::new();
        
        let box_model = BoxModel::new(Rect::new(0.0, 0.0, 100.0, 50.0));
        let node = LayoutNode {
            node_id: NodeId(1),
            box_model,
            flexbox: None,
            grid: None,
            parent_id: None,
            child_ids: vec![],
        };
        
        tree.add_node(node.clone()).unwrap();
        
        let result = tree.add_node(node);
        assert!(result.is_err());
        assert_eq!(tree.node_count(), 1);
    }
    
    #[test]
    fn test_get_layout_node() {
        let mut tree = LayoutTree::new();
        
        let box_model = BoxModel::new(Rect::new(0.0, 0.0, 100.0, 50.0));
        let node = LayoutNode {
            node_id: NodeId(1),
            box_model: box_model.clone(),
            flexbox: None,
            grid: None,
            parent_id: None,
            child_ids: vec![],
        };
        
        tree.add_node(node).unwrap();
        
        let retrieved = tree.get_node(NodeId(1));
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().box_model.content, box_model.content);
    }
    
    #[test]
    fn test_remove_layout_node() {
        let mut tree = LayoutTree::new();
        
        let box_model = BoxModel::new(Rect::new(0.0, 0.0, 100.0, 50.0));
        let node = LayoutNode {
            node_id: NodeId(1),
            box_model,
            flexbox: None,
            grid: None,
            parent_id: None,
            child_ids: vec![],
        };
        
        tree.add_node(node).unwrap();
        assert_eq!(tree.node_count(), 1);
        
        assert!(tree.remove_node(NodeId(1)));
        assert_eq!(tree.node_count(), 0);
        assert!(tree.is_empty());
    }
    
    #[test]
    fn test_calculate_layout() {
        let mut tree = LayoutTree::new();
        
        let box_model = BoxModel::new(Rect::new(0.0, 0.0, 100.0, 50.0));
        let node = LayoutNode {
            node_id: NodeId(1),
            box_model,
            flexbox: None,
            grid: None,
            parent_id: None,
            child_ids: vec![],
        };
        
        tree.add_node(node).unwrap();
        
        let container_size = Size::new(200.0, 100.0);
        let result = tree.calculate_layout(container_size);
        
        assert!(result.is_ok());
        let layouts = result.unwrap();
        assert_eq!(layouts.len(), 1);
        assert!(layouts.contains_key(&NodeId(1)));
    }
}

impl Default for LayoutTree {
    fn default() -> Self {
        Self::new()
    }
}
