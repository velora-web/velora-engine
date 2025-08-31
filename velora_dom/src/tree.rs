//! DOM tree management for the Velora web engine

use velora_core::{NodeId, ElementId, VeloraResult, VeloraError};
use velora_core::error::DomError;
use super::{Node, Element};
use std::collections::HashMap;

/// A DOM tree that manages the hierarchical relationship between nodes
#[derive(Debug, Clone)]
pub struct DomTree {
    /// All nodes in the tree, indexed by their ID
    nodes: HashMap<NodeId, Node>,
    
    /// All elements in the tree, indexed by their ID
    elements: HashMap<ElementId, Element>,
    
    /// Root node ID
    root_id: Option<NodeId>,
    
    /// Next available node ID
    next_node_id: u64,
    
    /// Next available element ID
    next_element_id: u64,
}

impl DomTree {
    /// Create a new empty DOM tree
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            elements: HashMap::new(),
            root_id: None,
            next_node_id: 1,
            next_element_id: 1,
        }
    }
    
    /// Add a node to the tree
    pub fn add_node(&mut self, node: Node) {
        let node_id = node.id;
        self.nodes.insert(node_id, node);
        
        // Update next ID if necessary
        let id_value = node_id.0;
        if id_value >= self.next_node_id {
            self.next_node_id = id_value + 1;
        }
        
        // Set as root if it's the first node
        if self.root_id.is_none() {
            self.root_id = Some(node_id);
        }
    }
    
    /// Add an element to the tree
    pub fn add_element(&mut self, element: Element) {
        let element_id = element.id;
        self.elements.insert(element_id, element);
        
        // Update next ID if necessary
        let id_value = element_id.0;
        if id_value >= self.next_element_id {
            self.next_element_id = id_value + 1;
        }
    }
    
    /// Get a node by ID
    pub fn get_node(&self, node_id: NodeId) -> VeloraResult<&Node> {
        self.nodes.get(&node_id)
            .ok_or_else(|| VeloraError::Dom(DomError::NodeNotFound(format!("Node {} not found", node_id.0))))
    }
    
    /// Get a mutable reference to a node by ID
    pub fn get_node_mut(&mut self, node_id: NodeId) -> VeloraResult<&mut Node> {
        self.nodes.get_mut(&node_id)
            .ok_or_else(|| VeloraError::Dom(DomError::NodeNotFound(format!("Node {} not found", node_id.0))))
    }
    
    /// Get an element by ID
    pub fn get_element(&self, element_id: ElementId) -> VeloraResult<&Element> {
        self.elements.get(&element_id)
            .ok_or_else(|| VeloraError::Dom(DomError::NodeNotFound(format!("Element {} not found", element_id.0))))
    }
    
    /// Get a mutable reference to an element by ID
    pub fn get_element_mut(&mut self, element_id: ElementId) -> VeloraResult<&mut Element> {
        self.elements.get_mut(&element_id)
            .ok_or_else(|| VeloraError::Dom(DomError::NodeNotFound(format!("Element {} not found", element_id.0))))
    }
    
    /// Get the root node
    pub fn get_root(&self) -> Option<&Node> {
        self.root_id.and_then(|id| self.nodes.get(&id))
    }
    
    /// Set the root node
    pub fn set_root(&mut self, node_id: NodeId) -> VeloraResult<()> {
        if self.nodes.contains_key(&node_id) {
            self.root_id = Some(node_id);
            Ok(())
        } else {
            Err(VeloraError::Dom(DomError::NodeNotFound(format!("Cannot set root to non-existent node {}", node_id.0))))
        }
    }
    
    /// Find a node by its name (tag name for elements, text content for text nodes)
    pub fn find_node_by_name(&self, name: &str) -> Option<&Node> {
        self.nodes.values().find(|node| node.node_name == name)
    }
    
    /// Find all nodes by name
    pub fn find_nodes_by_name(&self, name: &str) -> Vec<&Node> {
        self.nodes.values()
            .filter(|node| node.node_name == name)
            .collect()
    }
    
    /// Find an element by its ID attribute
    pub fn find_element_by_id(&self, _id: &str) -> Option<&Node> {
        // This is a simplified search
        // In a real implementation, you'd need to traverse the tree
        // and check element attributes
        None
    }
    
    /// Find elements by class name
    pub fn find_elements_by_class(&self, _class_name: &str) -> Vec<&Node> {
        // This is a simplified search
        // In a real implementation, you'd need to traverse the tree
        // and check element attributes
        Vec::new()
    }
    
    /// Create a new element node
    pub fn create_element(&mut self, tag_name: &str) -> VeloraResult<NodeId> {
        let node_id = NodeId(self.next_node_id);
        let element_id = ElementId(self.next_element_id);
        
        // Create the element
        let element = Element::new(element_id, tag_name.to_string());
        self.add_element(element);
        
        // Create the node
        let mut node = Node::new_element(node_id, tag_name.to_string());
        node.set_element_id(element_id)?;
        self.add_node(node);
        
        // Update IDs
        self.next_node_id += 1;
        self.next_element_id += 1;
        
        Ok(node_id)
    }
    
    /// Create a new text node
    pub fn create_text_node(&mut self, text: &str) -> VeloraResult<NodeId> {
        let node_id = NodeId(self.next_node_id);
        
        let node = Node::new_text(node_id, text.to_string());
        self.add_node(node);
        
        self.next_node_id += 1;
        Ok(node_id)
    }
    
    /// Append a child to a parent node
    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) -> VeloraResult<()> {
        // First, get all the data we need to avoid multiple mutable borrows
        let last_child_id = {
            let parent = self.get_node(parent_id)?;
            parent.child_ids.iter().rev().nth(1).copied()
        };
        
        // Add child to parent
        {
            let parent = self.get_node_mut(parent_id)?;
            parent.add_child(child_id);
        }
        
        // Set parent on child
        {
            let child = self.get_node_mut(child_id)?;
            child.set_parent(parent_id);
        }
        
        // Update sibling relationships
        if let Some(last_child_id) = last_child_id {
            if let Ok(last_child) = self.get_node_mut(last_child_id) {
                last_child.set_next_sibling(child_id);
            }
            if let Ok(child) = self.get_node_mut(child_id) {
                child.set_previous_sibling(last_child_id);
            }
        }
        
        Ok(())
    }
    
    /// Remove a child from a parent node
    pub fn remove_child(&mut self, parent_id: NodeId, child_id: NodeId) -> VeloraResult<()> {
        // Get the child's sibling information before removing it
        let (prev_sibling_id, next_sibling_id) = {
            let child = self.get_node(child_id)?;
            (child.previous_sibling_id, child.next_sibling_id)
        };
        
        // Remove child from parent
        {
            let parent = self.get_node_mut(parent_id)?;
            if !parent.remove_child(child_id) {
                return Err(VeloraError::Dom(DomError::NodeNotFound(
                    format!("Child {} not found in parent {}", child_id.0, parent_id.0)
                )));
            }
        }
        
        // Clear parent and sibling relationships on child
        {
            let child = self.get_node_mut(child_id)?;
            child.clear_parent();
            child.clear_siblings();
        }
        
        // Update sibling relationships
        if let Some(prev_sibling_id) = prev_sibling_id {
            if let Ok(prev_sibling) = self.get_node_mut(prev_sibling_id) {
                if let Some(next_sibling_id) = next_sibling_id {
                    prev_sibling.set_next_sibling(next_sibling_id);
                } else {
                    prev_sibling.set_next_sibling(prev_sibling_id); // No next sibling, clear it
                }
            }
        }
        
        if let Some(next_sibling_id) = next_sibling_id {
            if let Ok(next_sibling) = self.get_node_mut(next_sibling_id) {
                if let Some(prev_sibling_id) = prev_sibling_id {
                    next_sibling.set_previous_sibling(prev_sibling_id);
                } else {
                    next_sibling.set_previous_sibling(next_sibling_id); // No prev sibling, clear it
                }
            }
        }
        
        Ok(())
    }
    
    /// Get the number of nodes in the tree
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    /// Get the number of elements in the tree
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }
    
    /// Check if the tree is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    
    /// Get all nodes in the tree
    pub fn get_all_nodes(&self) -> Vec<&Node> {
        self.nodes.values().collect()
    }
    
    /// Get all elements in the tree
    pub fn get_all_elements(&self) -> Vec<&Element> {
        self.elements.values().collect()
    }
    
    /// Traverse the tree in depth-first order
    pub fn traverse_dfs<F>(&self, mut visitor: F) -> VeloraResult<()>
    where
        F: FnMut(&Node) -> VeloraResult<bool>, // Return false to stop traversal
    {
        if let Some(root_id) = self.root_id {
            self.traverse_node_dfs(root_id, &mut visitor)?;
        }
        Ok(())
    }
    
    /// Traverse a specific node and its descendants
    fn traverse_node_dfs<F>(&self, node_id: NodeId, visitor: &mut F) -> VeloraResult<()>
    where
        F: FnMut(&Node) -> VeloraResult<bool>,
    {
        let node = self.get_node(node_id)?;
        
        // Visit current node
        if !visitor(node)? {
            return Ok(());
        }
        
        // Visit children
        for &child_id in &node.child_ids {
            self.traverse_node_dfs(child_id, visitor)?;
        }
        
        Ok(())
    }
}

impl Default for DomTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use velora_core::NodeId;
    
    #[test]
    fn test_dom_tree_creation() {
        let tree = DomTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.node_count(), 0);
        assert_eq!(tree.element_count(), 0);
    }
    
    #[test]
    fn test_add_node() {
        let mut tree = DomTree::new();
        let node = Node::new_element(NodeId(1), "div".to_string());
        
        tree.add_node(node);
        assert_eq!(tree.node_count(), 1);
        assert!(!tree.is_empty());
    }
    
    #[test]
    fn test_get_node() {
        let mut tree = DomTree::new();
        let node = Node::new_element(NodeId(1), "div".to_string());
        tree.add_node(node);
        
        let retrieved = tree.get_node(NodeId(1));
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap().node_name, "div");
    }
    
    #[test]
    fn test_get_nonexistent_node() {
        let tree = DomTree::new();
        let result = tree.get_node(NodeId(999));
        assert!(result.is_err());
    }
    
    #[test]
    fn test_create_element() {
        let mut tree = DomTree::new();
        let node_id = tree.create_element("span").unwrap();
        
        assert_eq!(tree.node_count(), 1);
        assert_eq!(tree.element_count(), 1);
        
        let node = tree.get_node(node_id).unwrap();
        assert_eq!(node.node_name, "span");
    }
    
    #[test]
    fn test_create_text_node() {
        let mut tree = DomTree::new();
        let node_id = tree.create_text_node("Hello World").unwrap();
        
        assert_eq!(tree.node_count(), 1);
        
        let node = tree.get_node(node_id).unwrap();
        assert_eq!(node.node_name, "#text");
        assert_eq!(node.node_value, Some("Hello World".to_string()));
    }
    
    #[test]
    fn test_append_child() {
        let mut tree = DomTree::new();
        let parent_id = tree.create_element("div").unwrap();
        let child_id = tree.create_element("span").unwrap();
        
        tree.append_child(parent_id, child_id).unwrap();
        
        let parent = tree.get_node(parent_id).unwrap();
        assert!(parent.has_child(child_id));
        
        let child = tree.get_node(child_id).unwrap();
        assert!(child.has_parent());
        assert_eq!(child.parent_id, Some(parent_id));
    }
    
    #[test]
    fn test_remove_child() {
        let mut tree = DomTree::new();
        let parent_id = tree.create_element("div").unwrap();
        let child_id = tree.create_element("span").unwrap();
        
        tree.append_child(parent_id, child_id).unwrap();
        assert!(tree.get_node(parent_id).unwrap().has_child(child_id));
        
        tree.remove_child(parent_id, child_id).unwrap();
        assert!(!tree.get_node(parent_id).unwrap().has_child(child_id));
    }
}
