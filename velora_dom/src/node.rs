//! DOM node types and structures

use velora_core::{NodeId, ElementId, VeloraResult};
use velora_core::error::DomError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of DOM nodes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    Element,
    Text,
    Comment,
    Document,
    DocumentType,
    ProcessingInstruction,
}

/// A DOM node in the document tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for this node
    pub id: NodeId,
    
    /// Type of this node
    pub node_type: NodeType,
    
    /// Node name (tag name for elements, text content for text nodes)
    pub node_name: String,
    
    /// Node value (text content, comment text, etc.)
    pub node_value: Option<String>,
    
    /// Parent node ID (None for root)
    pub parent_id: Option<NodeId>,
    
    /// Child node IDs
    pub child_ids: Vec<NodeId>,
    
    /// Previous sibling node ID
    pub previous_sibling_id: Option<NodeId>,
    
    /// Next sibling node ID
    pub next_sibling_id: Option<NodeId>,
    
    /// Element ID if this is an element node
    pub element_id: Option<ElementId>,
    
    /// Additional node data
    pub data: HashMap<String, serde_json::Value>,
}

impl Node {
    /// Create a new node
    pub fn new(
        id: NodeId,
        node_type: NodeType,
        node_name: String,
        node_value: Option<String>,
    ) -> Self {
        Self {
            id,
            node_type,
            node_name,
            node_value,
            parent_id: None,
            child_ids: Vec::new(),
            previous_sibling_id: None,
            next_sibling_id: None,
            element_id: None,
            data: HashMap::new(),
        }
    }
    
    /// Create a new element node
    pub fn new_element(id: NodeId, tag_name: String) -> Self {
        Self::new(id, NodeType::Element, tag_name, None)
    }
    
    /// Create a new text node
    pub fn new_text(id: NodeId, text: String) -> Self {
        Self::new(id, NodeType::Text, "#text".to_string(), Some(text))
    }
    
    /// Create a new comment node
    pub fn new_comment(id: NodeId, comment: String) -> Self {
        Self::new(id, NodeType::Comment, "#comment".to_string(), Some(comment))
    }
    
    /// Create a new document node
    pub fn new_document(id: NodeId) -> Self {
        Self::new(id, NodeType::Document, "#document".to_string(), None)
    }
    
    /// Check if this node is an element
    pub fn is_element(&self) -> bool {
        matches!(self.node_type, NodeType::Element)
    }
    
    /// Check if this node is a text node
    pub fn is_text(&self) -> bool {
        matches!(self.node_type, NodeType::Text)
    }
    
    /// Check if this node is a comment
    pub fn is_comment(&self) -> bool {
        matches!(self.node_type, NodeType::Comment)
    }
    
    /// Check if this node is a document
    pub fn is_document(&self) -> bool {
        matches!(self.node_type, NodeType::Document)
    }
    
    /// Check if this node has children
    pub fn has_children(&self) -> bool {
        !self.child_ids.is_empty()
    }
    
    /// Check if this node has a parent
    pub fn has_parent(&self) -> bool {
        self.parent_id.is_some()
    }
    
    /// Check if this node has siblings
    pub fn has_siblings(&self) -> bool {
        self.previous_sibling_id.is_some() || self.next_sibling_id.is_some()
    }
    
    /// Get the number of children
    pub fn child_count(&self) -> usize {
        self.child_ids.len()
    }
    
    /// Check if this node is a leaf (no children)
    pub fn is_leaf(&self) -> bool {
        self.child_ids.is_empty()
    }
    
    /// Check if this node is the root (no parent)
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }
    
    /// Add a child node
    pub fn add_child(&mut self, child_id: NodeId) {
        if !self.child_ids.contains(&child_id) {
            self.child_ids.push(child_id);
        }
    }
    
    /// Remove a child node
    pub fn remove_child(&mut self, child_id: NodeId) -> bool {
        if let Some(pos) = self.child_ids.iter().position(|&id| id == child_id) {
            self.child_ids.remove(pos);
            true
        } else {
            false
        }
    }
    
    /// Check if this node has a specific child
    pub fn has_child(&self, child_id: NodeId) -> bool {
        self.child_ids.contains(&child_id)
    }
    
    /// Get the first child ID
    pub fn first_child(&self) -> Option<NodeId> {
        self.child_ids.first().copied()
    }
    
    /// Get the last child ID
    pub fn last_child(&self) -> Option<NodeId> {
        self.child_ids.last().copied()
    }
    
    /// Set the parent node
    pub fn set_parent(&mut self, parent_id: NodeId) {
        self.parent_id = Some(parent_id);
    }
    
    /// Clear the parent node
    pub fn clear_parent(&mut self) {
        self.parent_id = None;
    }
    
    /// Set the previous sibling
    pub fn set_previous_sibling(&mut self, sibling_id: NodeId) {
        self.previous_sibling_id = Some(sibling_id);
    }
    
    /// Set the next sibling
    pub fn set_next_sibling(&mut self, sibling_id: NodeId) {
        self.next_sibling_id = Some(sibling_id);
    }
    
    /// Clear sibling relationships
    pub fn clear_siblings(&mut self) {
        self.previous_sibling_id = None;
        self.next_sibling_id = None;
    }
    
    /// Set element ID for element nodes
    pub fn set_element_id(&mut self, element_id: ElementId) -> VeloraResult<()> {
        if self.is_element() {
            self.element_id = Some(element_id);
            Ok(())
        } else {
            Err(velora_core::VeloraError::Dom(
                DomError::InvalidNodeType
            ))
        }
    }
    
    /// Get element ID if this is an element node
    pub fn get_element_id(&self) -> Option<ElementId> {
        self.element_id
    }
    
    /// Set custom data
    pub fn set_data(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }
    
    /// Get custom data
    pub fn get_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }
    
    /// Remove custom data
    pub fn remove_data(&mut self, key: &str) -> Option<serde_json::Value> {
        self.data.remove(key)
    }
    
    /// Get text content (recursively for element nodes)
    pub fn get_text_content(&self) -> String {
        match self.node_type {
            NodeType::Text => self.node_value.clone().unwrap_or_default(),
            NodeType::Element => {
                // For elements, we'd need to traverse children
                // This is a simplified version
                String::new()
            }
            _ => self.node_value.clone().unwrap_or_default(),
        }
    }
    
    /// Clone this node with a new ID
    pub fn clone_with_id(&self, new_id: NodeId) -> Self {
        Self {
            id: new_id,
            node_type: self.node_type.clone(),
            node_name: self.node_name.clone(),
            node_value: self.node_value.clone(),
            parent_id: None, // Reset parent
            child_ids: Vec::new(), // Reset children
            previous_sibling_id: None, // Reset siblings
            next_sibling_id: None,
            element_id: None, // Reset element ID
            data: self.data.clone(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use velora_core::{NodeId, ElementId};
    
    #[test]
    fn test_node_creation() {
        let node = Node::new(
            NodeId(1),
            NodeType::Element,
            "div".to_string(),
            None,
        );
        
        assert_eq!(node.id, NodeId(1));
        assert_eq!(node.node_type, NodeType::Element);
        assert_eq!(node.node_name, "div");
        assert!(node.node_value.is_none());
    }
    
    #[test]
    fn test_element_node() {
        let node = Node::new_element(NodeId(1), "div".to_string());
        assert!(node.is_element());
        assert!(!node.is_text());
        assert!(!node.is_comment());
        assert!(!node.is_document());
    }
    
    #[test]
    fn test_text_node() {
        let node = Node::new_text(NodeId(2), "Hello World".to_string());
        assert!(node.is_text());
        assert_eq!(node.node_value, Some("Hello World".to_string()));
    }
    
    #[test]
    fn test_comment_node() {
        let node = Node::new_comment(NodeId(3), "This is a comment".to_string());
        assert!(node.is_comment());
        assert_eq!(node.node_value, Some("This is a comment".to_string()));
    }
    
    #[test]
    fn test_document_node() {
        let node = Node::new_document(NodeId(4));
        assert!(node.is_document());
        assert!(node.is_root());
    }
    
    #[test]
    fn test_child_management() {
        let mut node = Node::new_element(NodeId(1), "div".to_string());
        let child_id = NodeId(2);
        
        assert_eq!(node.child_count(), 0);
        assert!(node.is_leaf());
        
        node.add_child(child_id);
        assert_eq!(node.child_count(), 1);
        assert!(!node.is_leaf());
        assert!(node.has_child(child_id));
        
        assert!(node.remove_child(child_id));
        assert_eq!(node.child_count(), 0);
        assert!(node.is_leaf());
    }
    
    #[test]
    fn test_parent_management() {
        let mut node = Node::new_element(NodeId(1), "div".to_string());
        let parent_id = NodeId(0);
        
        assert!(node.is_root());
        
        node.set_parent(parent_id);
        assert!(!node.is_root());
        assert_eq!(node.parent_id, Some(parent_id));
        
        node.clear_parent();
        assert!(node.is_root());
    }
    
    #[test]
    fn test_element_id() {
        let mut node = Node::new_element(NodeId(1), "div".to_string());
        let element_id = ElementId(1);
        
        assert!(node.set_element_id(element_id).is_ok());
        assert_eq!(node.get_element_id(), Some(element_id));
        
        // Test setting element ID on non-element node
        let mut text_node = Node::new_text(NodeId(2), "text".to_string());
        assert!(text_node.set_element_id(element_id).is_err());
    }
    
    #[test]
    fn test_data_management() {
        let mut node = Node::new_element(NodeId(1), "div".to_string());
        
        node.set_data("key1".to_string(), serde_json::json!("value1"));
        node.set_data("key2".to_string(), serde_json::json!(42));
        
        assert_eq!(node.get_data("key1"), Some(&serde_json::json!("value1")));
        assert_eq!(node.get_data("key2"), Some(&serde_json::json!(42)));
        assert_eq!(node.get_data("key3"), None);
        
        assert_eq!(node.remove_data("key1"), Some(serde_json::json!("value1")));
        assert_eq!(node.get_data("key1"), None);
    }
}
