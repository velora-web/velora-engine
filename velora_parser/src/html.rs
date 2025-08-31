//! HTML parser implementation using html5ever

use velora_core::{VeloraResult, VeloraError, ParserError, NodeId, ElementId, next_id};
use velora_dom::{Node, NodeType, Element};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever::interface::QualName;
use markup5ever::parse::ParseOpts;
use markup5ever::tree_builder::TreeBuilderOpts;
use std::collections::HashMap;
use log::{debug, error};

/// HTML parser that converts HTML markup into DOM structures
pub struct HtmlParser {
    /// Parser options for HTML5 parsing
    parse_opts: ParseOpts,
    
    /// Tree builder options
    tree_builder_opts: TreeBuilderOpts,
}

impl HtmlParser {
    /// Create a new HTML parser with default options
    pub fn new() -> Self {
        Self {
            parse_opts: ParseOpts::default(),
            tree_builder_opts: TreeBuilderOpts::default(),
        }
    }
    
    /// Create a new HTML parser with custom options
    pub fn with_options(parse_opts: ParseOpts, tree_builder_opts: TreeBuilderOpts) -> Self {
        Self {
            parse_opts,
            tree_builder_opts,
        }
    }
    
    /// Parse HTML string into a DOM document
    pub fn parse_html(&self, html: &str) -> VeloraResult<velora_dom::Document> {
        debug!("Parsing HTML document of {} bytes", html.len());
        
        let mut dom_tree = velora_dom::DomTree::new();
        let document_id = NodeId(next_id());
        let document_element_id = ElementId(next_id());
        
        // Create document node
        let document_node = Node::new_document(document_id);
        dom_tree.add_node(document_node);
        
        // Create document element
        let document_element = Element::new(document_element_id, "html".to_string());
        dom_tree.add_element(document_element);
        
        // Create HTML element node
        let html_node_id = NodeId(next_id());
        let mut html_node = Node::new_element(html_node_id, "html".to_string());
        html_node.set_element_id(document_element_id)?;
        html_node.set_parent(document_id);
        dom_tree.add_node(html_node);
        
        // Add HTML node as child of document
        dom_tree.get_node_mut(document_id)?.add_child(html_node_id);
        
        // Parse the HTML content
        let sink = HtmlSink::new(&mut dom_tree, document_id, html_node_id);
        let mut parser = parse_document(sink, self.parse_opts.clone());
        
        // Feed the HTML content
        parser.process(TendrilSink::from(html));
        
        // Consume the parser to get the final result
        let _result = parser.end();
        
        // Create and return the document
        let mut document = velora_dom::Document::new(document_id);
        document.set_dom_tree(dom_tree);
        
        Ok(document)
    }
    
    /// Parse HTML fragment (without document wrapper)
    pub fn parse_fragment(&self, html: &str, context_element: &str) -> VeloraResult<Vec<Node>> {
        debug!("Parsing HTML fragment with context element: {}", context_element);
        
        // For fragments, we'll create a temporary document and extract the body content
        let temp_html = format!("<html><body>{}</body></html>", html);
        let document = self.parse_html(&temp_html)?;
        
        // Extract body children
        let dom_tree = document.get_dom_tree();
        let body_node = dom_tree
            .find_node_by_name("body")
            .ok_or_else(|| VeloraError::Parser(ParserError::HtmlParsing("Body element not found".to_string())))?;
        
        let body_children: Vec<Node> = body_node
            .child_ids
            .iter()
            .filter_map(|&child_id| dom_tree.get_node(child_id).ok().cloned())
            .collect();
        
        Ok(body_children)
    }
    
    /// Parse HTML with error recovery
    pub fn parse_html_with_recovery(&self, html: &str) -> VeloraResult<velora_dom::Document> {
        // This would implement more robust error recovery
        // For now, we'll use the basic parser
        self.parse_html(html)
    }
}

impl Default for HtmlParser {
    fn default() -> Self {
        Self::new()
    }
}

/// HTML sink that builds the DOM tree during parsing
struct HtmlSink<'a> {
    dom_tree: &'a mut velora_dom::DomTree,
    document_id: NodeId,
    current_parent_id: NodeId,
}

impl<'a> HtmlSink<'a> {
    fn new(dom_tree: &'a mut velora_dom::DomTree, document_id: NodeId, root_id: NodeId) -> Self {
        Self {
            dom_tree,
            document_id,
            current_parent_id: root_id,
        }
    }
    
    fn create_element(&mut self, name: &str, attributes: HashMap<String, String>) -> NodeId {
        let node_id = NodeId(next_id());
        let element_id = ElementId(next_id());
        
        // Create the element
        let mut element = Element::new(element_id, name.to_string());
        
        // Set attributes
        for (key, value) in attributes {
            element.set_attribute(key, value);
        }
        
        // Add element to DOM tree
        self.dom_tree.add_element(element);
        
        // Create the node
        let mut node = Node::new_element(node_id, name.to_string());
        node.set_element_id(element_id).unwrap();
        node.set_parent(self.current_parent_id);
        
        // Add node to DOM tree
        self.dom_tree.add_node(node);
        
        // Add as child of current parent
        if let Ok(parent_node) = self.dom_tree.get_node_mut(self.current_parent_id) {
            parent_node.add_child(node_id);
        }
        
        node_id
    }
    
    fn create_text_node(&mut self, text: &str) -> NodeId {
        let node_id = NodeId(next_id());
        let mut node = Node::new_text(node_id, text.to_string());
        node.set_parent(self.current_parent_id);
        
        // Add node to DOM tree
        self.dom_tree.add_node(node);
        
        // Add as child of current parent
        if let Ok(parent_node) = self.dom_tree.get_node_mut(self.current_parent_id) {
            parent_node.add_child(node_id);
        }
        
        node_id
    }
    
    fn create_comment_node(&mut self, comment: &str) -> NodeId {
        let node_id = NodeId(next_id());
        let mut node = Node::new_comment(node_id, comment.to_string());
        node.set_parent(self.current_parent_id);
        
        // Add node to DOM tree
        self.dom_tree.add_node(node);
        
        // Add as child of current parent
        if let Ok(parent_node) = self.dom_tree.get_node_mut(self.current_parent_id) {
            parent_node.add_child(node_id);
        }
        
        node_id
    }
}

impl<'a> TendrilSink for HtmlSink<'a> {
    type Output = ();
    
    fn process(&mut self, input: TendrilSink) {
        // This is a simplified implementation
        // In a real implementation, we would process the HTML tokens
        // and build the DOM tree accordingly
        
        debug!("Processing HTML input");
        
        // For now, we'll create a simple structure
        // In practice, this would be much more complex
        let body_id = self.create_element("body", HashMap::new());
        let head_id = self.create_element("head", HashMap::new());
        
        // Add some basic content
        let title_id = self.create_element("title", HashMap::new());
        let title_text_id = self.create_text_node("Document Title");
        
        // Update parent relationships
        if let Ok(head_node) = self.dom_tree.get_node_mut(head_id) {
            head_node.add_child(title_id);
        }
        
        if let Ok(title_node) = self.dom_tree.get_node_mut(title_id) {
            title_node.add_child(title_text_id);
        }
    }
    
    fn end(&mut self) -> Self::Output {
        debug!("Finished parsing HTML document");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_html_parser_creation() {
        let parser = HtmlParser::new();
        assert!(parser.parse_opts.scripting_enabled);
    }
    
    #[test]
    fn test_parse_simple_html() {
        let parser = HtmlParser::new();
        let html = "<html><head><title>Test</title></head><body><h1>Hello</h1></body></html>";
        
        let result = parser.parse_html(html);
        assert!(result.is_ok());
        
        let document = result.unwrap();
        assert_eq!(document.get_id(), NodeId(1));
    }
    
    #[test]
    fn test_parse_html_fragment() {
        let parser = HtmlParser::new();
        let html = "<div>Hello <span>World</span></div>";
        
        let result = parser.parse_fragment(html, "div");
        assert!(result.is_ok());
        
        let nodes = result.unwrap();
        assert!(!nodes.is_empty());
    }
}
