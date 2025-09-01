//! Document structure for the Velora web engine

use velora_core::{NodeId, VeloraResult};
use super::{Node, DomTree};

/// A complete HTML document
#[derive(Debug, Clone)]
pub struct Document {
    /// Unique identifier for this document
    id: NodeId,
    
    /// Document title
    title: Option<String>,
    
    /// Document URL
    url: Option<String>,
    
    /// Document character encoding
    encoding: String,
    
    /// The DOM tree containing all nodes
    dom_tree: DomTree,
}

impl Document {
    /// Create a new document
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            title: None,
            url: None,
            encoding: "UTF-8".to_string(),
            dom_tree: DomTree::new(),
        }
    }
    
    /// Get the document ID
    pub fn get_id(&self) -> NodeId {
        self.id
    }
    
    /// Get the document title
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    
    /// Set the document title
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }
    
    /// Get the document URL
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    
    /// Set the document URL
    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }
    
    /// Get the document encoding
    pub fn encoding(&self) -> &str {
        &self.encoding
    }
    
    /// Set the document encoding
    pub fn set_encoding(&mut self, encoding: String) {
        self.encoding = encoding;
    }
    
    /// Get the DOM tree
    pub fn get_dom_tree(&self) -> &DomTree {
        &self.dom_tree
    }
    
    /// Get a mutable reference to the DOM tree
    pub fn get_dom_tree_mut(&mut self) -> &mut DomTree {
        &mut self.dom_tree
    }
    
    /// Set the DOM tree
    pub fn set_dom_tree(&mut self, dom_tree: DomTree) {
        self.dom_tree = dom_tree;
    }
    
    /// Get the document element (html tag)
    pub fn document_element(&self) -> Option<&Node> {
        self.dom_tree.find_node_by_name("html")
    }
    
    /// Get the head element
    pub fn head(&self) -> Option<&Node> {
        self.dom_tree.find_node_by_name("head")
    }
    
    /// Get the body element
    pub fn body(&self) -> Option<&Node> {
        self.dom_tree.find_node_by_name("body")
    }
    
    /// Get an element by its ID
    pub fn get_element_by_id(&self, id: &str) -> Option<&Node> {
        self.dom_tree.find_element_by_id(id)
    }
    
    /// Get elements by tag name
    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<&Node> {
        self.dom_tree.find_nodes_by_name(tag_name)
    }
    
    /// Get elements by class name
    pub fn get_elements_by_class_name(&self, class_name: &str) -> Vec<&Node> {
        self.dom_tree.find_elements_by_class(class_name)
    }
    
    /// Create a new element
    pub fn create_element(&mut self, tag_name: &str) -> VeloraResult<NodeId> {
        self.dom_tree.create_element(tag_name)
    }
    
    /// Create a new text node
    pub fn create_text_node(&mut self, text: &str) -> VeloraResult<NodeId> {
        self.dom_tree.create_text_node(text)
    }
    
    /// Append a child to the document
    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) -> VeloraResult<()> {
        self.dom_tree.append_child(parent_id, child_id)
    }
    
    /// Remove a child from the document
    pub fn remove_child(&mut self, parent_id: NodeId, child_id: NodeId) -> VeloraResult<()> {
        self.dom_tree.remove_child(parent_id, child_id)
    }
    
    /// Duplicate the document
    pub fn duplicate(&self) -> Self {
        Self {
            id: NodeId(velora_core::next_id()),
            title: self.title.clone(),
            url: self.url.clone(),
            encoding: self.encoding.clone(),
            dom_tree: self.dom_tree.clone(),
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new(NodeId(velora_core::next_id()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_document_creation() {
        let doc = Document::new(NodeId(1));
        assert_eq!(doc.get_id(), NodeId(1));
        assert_eq!(doc.encoding(), "UTF-8");
        assert!(doc.title().is_none());
        assert!(doc.url().is_none());
    }
    
    #[test]
    fn test_document_title() {
        let mut doc = Document::new(NodeId(1));
        doc.set_title("Test Document".to_string());
        assert_eq!(doc.title(), Some("Test Document"));
    }
    
    #[test]
    fn test_document_url() {
        let mut doc = Document::new(NodeId(1));
        doc.set_url("https://example.com".to_string());
        assert_eq!(doc.url(), Some("https://example.com"));
    }
    
    #[test]
    fn test_document_encoding() {
        let mut doc = Document::new(NodeId(1));
        doc.set_encoding("ISO-8859-1".to_string());
        assert_eq!(doc.encoding(), "ISO-8859-1");
    }
}
