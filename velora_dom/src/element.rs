//! DOM element implementation for the Velora web engine

use velora_core::ElementId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A DOM element with attributes and properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    /// Unique identifier for this element
    pub id: ElementId,
    
    /// Tag name of this element
    pub tag_name: String,
    
    /// Element attributes
    pub attributes: HashMap<String, String>,
    
    /// Element properties
    pub properties: HashMap<String, serde_json::Value>,
    
    /// Element classes
    pub classes: Vec<String>,
    
    /// Element ID attribute
    pub element_id: Option<String>,
    
    /// Element style attribute
    pub style: Option<String>,
    
    /// Element dataset (data-* attributes)
    pub dataset: HashMap<String, String>,
}

impl Element {
    /// Create a new element with the given tag name
    pub fn new(id: ElementId, tag_name: String) -> Self {
        Self {
            id,
            tag_name,
            attributes: HashMap::new(),
            properties: HashMap::new(),
            classes: Vec::new(),
            element_id: None,
            style: None,
            dataset: HashMap::new(),
        }
    }
    
    /// Get the tag name
    pub fn tag_name(&self) -> &str {
        &self.tag_name
    }
    
    /// Set the tag name
    pub fn set_tag_name(&mut self, tag_name: String) {
        self.tag_name = tag_name;
    }
    
    /// Get an attribute value
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }
    
    /// Set an attribute value
    pub fn set_attribute(&mut self, name: String, value: String) {
        // Handle special attributes
        match name.as_str() {
            "id" => self.element_id = Some(value.clone()),
            "class" => self.classes = value.split_whitespace().map(|s| s.to_string()).collect(),
            "style" => self.style = Some(value.clone()),
            _ => {
                if let Some(stripped) = name.strip_prefix("data-") {
                    let key = stripped.to_string(); // Remove "data-" prefix
                    self.dataset.insert(key, value.clone());
                }
            }
        }
        
        self.attributes.insert(name, value);
    }
    
    /// Remove an attribute
    pub fn remove_attribute(&mut self, name: &str) -> Option<String> {
        let value = self.attributes.remove(name);
        
        // Handle special attributes
        match name {
            "id" => self.element_id = None,
            "class" => self.classes.clear(),
            "style" => self.style = None,
            _ => {
                if let Some(stripped) = name.strip_prefix("data-") {
                    let key = stripped.to_string();
                    self.dataset.remove(&key);
                }
            }
        }
        
        value
    }
    
    /// Check if an attribute exists
    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }
    
    /// Get all attribute names
    pub fn get_attribute_names(&self) -> Vec<&String> {
        self.attributes.keys().collect()
    }
    
    /// Get all attributes as a reference
    pub fn get_attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }
    
    /// Get a property value
    pub fn get_property(&self, name: &str) -> Option<&serde_json::Value> {
        self.properties.get(name)
    }
    
    /// Set a property value
    pub fn set_property(&mut self, name: String, value: serde_json::Value) {
        self.properties.insert(name, value);
    }
    
    /// Remove a property
    pub fn remove_property(&mut self, name: &str) -> Option<serde_json::Value> {
        self.properties.remove(name)
    }
    
    /// Check if a property exists
    pub fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }
    
    /// Get all property names
    pub fn get_property_names(&self) -> Vec<&String> {
        self.properties.keys().collect()
    }
    
    /// Get all properties as a reference
    pub fn get_properties(&self) -> &HashMap<String, serde_json::Value> {
        &self.properties
    }
    
    /// Get the element ID attribute
    pub fn get_id(&self) -> Option<&str> {
        self.element_id.as_deref()
    }
    
    /// Set the element ID attribute
    pub fn set_id(&mut self, id: Option<String>) {
        self.element_id = id.clone();
        
        if let Some(id_value) = id {
            self.attributes.insert("id".to_string(), id_value);
        } else {
            self.attributes.remove("id");
        }
    }
    
    /// Get the element classes
    pub fn get_classes(&self) -> &[String] {
        &self.classes
    }
    
    /// Add a class to the element
    pub fn add_class(&mut self, class: String) {
        if !self.classes.contains(&class) {
            self.classes.push(class);
            self.update_class_attribute();
        }
    }
    
    /// Remove a class from the element
    pub fn remove_class(&mut self, class: &str) -> bool {
        if let Some(pos) = self.classes.iter().position(|c| c == class) {
            self.classes.remove(pos);
            self.update_class_attribute();
            true
        } else {
            false
        }
    }
    
    /// Check if the element has a specific class
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.contains(&class.to_string())
    }
    
    /// Toggle a class on the element
    pub fn toggle_class(&mut self, class: String) -> bool {
        if self.has_class(&class) {
            self.remove_class(&class);
            false
        } else {
            self.add_class(class);
            true
        }
    }
    
    /// Get the style attribute
    pub fn get_style(&self) -> Option<&str> {
        self.style.as_deref()
    }
    
    /// Set the style attribute
    pub fn set_style(&mut self, style: Option<String>) {
        self.style = style.clone();
        
        if let Some(style_value) = style {
            self.attributes.insert("style".to_string(), style_value);
        } else {
            self.attributes.remove("style");
        }
    }
    
    /// Get a dataset value
    pub fn get_dataset(&self, key: &str) -> Option<&str> {
        self.dataset.get(key).map(|s| s.as_str())
    }
    
    /// Set a dataset value
    pub fn set_dataset(&mut self, key: String, value: String) {
        self.dataset.insert(key.clone(), value.clone());
        let attr_name = format!("data-{}", key);
        self.attributes.insert(attr_name, value);
    }
    
    /// Remove a dataset value
    pub fn remove_dataset(&mut self, key: &str) -> Option<String> {
        let value = self.dataset.remove(key);
        let attr_name = format!("data-{}", key);
        self.attributes.remove(&attr_name);
        value
    }
    
    /// Check if the element is a void element (self-closing)
    pub fn is_void_element(&self) -> bool {
        matches!(
            self.tag_name.as_str(),
            "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" |
            "link" | "meta" | "param" | "source" | "track" | "wbr"
        )
    }
    
    /// Check if the element is a block-level element
    pub fn is_block_element(&self) -> bool {
        matches!(
            self.tag_name.as_str(),
            "address" | "article" | "aside" | "blockquote" | "canvas" | "dd" | "div" |
            "dl" | "dt" | "fieldset" | "figcaption" | "figure" | "footer" | "form" |
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "header" | "hr" | "li" |
            "main" | "nav" | "noscript" | "ol" | "p" | "pre" | "section" | "table" |
            "tfoot" | "ul" | "video"
        )
    }
    
    /// Check if the element is an inline element
    pub fn is_inline_element(&self) -> bool {
        !self.is_block_element() && !self.is_void_element()
    }
    
    /// Update the class attribute based on the classes vector
    fn update_class_attribute(&mut self) {
        if self.classes.is_empty() {
            self.attributes.remove("class");
        } else {
            let class_value = self.classes.join(" ");
            self.attributes.insert("class".to_string(), class_value);
        }
    }
    
    /// Clone this element with a new ID
    pub fn clone_with_id(&self, new_id: ElementId) -> Self {
        Self {
            id: new_id,
            tag_name: self.tag_name.clone(),
            attributes: self.attributes.clone(),
            properties: self.properties.clone(),
            classes: self.classes.clone(),
            element_id: self.element_id.clone(),
            style: self.style.clone(),
            dataset: self.dataset.clone(),
        }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Element {}

impl std::hash::Hash for Element {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use velora_core::ElementId;
    
    #[test]
    fn test_element_creation() {
        let element = Element::new(ElementId(1), "div".to_string());
        assert_eq!(element.tag_name(), "div");
        assert_eq!(element.id, ElementId(1));
        assert!(element.attributes.is_empty());
        assert!(element.properties.is_empty());
        assert!(element.classes.is_empty());
    }
    
    #[test]
    fn test_attribute_management() {
        let mut element = Element::new(ElementId(1), "div".to_string());
        
        element.set_attribute("class".to_string(), "container".to_string());
        assert_eq!(element.get_attribute("class"), Some("container"));
        assert!(element.has_attribute("class"));
        
        element.remove_attribute("class");
        assert_eq!(element.get_attribute("class"), None);
        assert!(!element.has_attribute("class"));
    }
    
    #[test]
    fn test_class_management() {
        let mut element = Element::new(ElementId(1), "div".to_string());
        
        element.add_class("container".to_string());
        element.add_class("header".to_string());
        
        assert!(element.has_class("container"));
        assert!(element.has_class("header"));
        assert_eq!(element.get_classes().len(), 2);
        
        element.remove_class("header");
        assert!(!element.has_class("header"));
        assert_eq!(element.get_classes().len(), 1);
    }
    
    #[test]
    fn test_id_management() {
        let mut element = Element::new(ElementId(1), "div".to_string());
        
        element.set_id(Some("main".to_string()));
        assert_eq!(element.get_id(), Some("main"));
        assert_eq!(element.get_attribute("id"), Some("main"));
        
        element.set_id(None);
        assert_eq!(element.get_id(), None);
        assert_eq!(element.get_attribute("id"), None);
    }
    
    #[test]
    fn test_style_management() {
        let mut element = Element::new(ElementId(1), "div".to_string());
        
        element.set_style(Some("color: red;".to_string()));
        assert_eq!(element.get_style(), Some("color: red;"));
        assert_eq!(element.get_attribute("style"), Some("color: red;"));
        
        element.set_style(None);
        assert_eq!(element.get_style(), None);
        assert_eq!(element.get_attribute("style"), None);
    }
    
    #[test]
    fn test_dataset_management() {
        let mut element = Element::new(ElementId(1), "div".to_string());
        
        element.set_dataset("key".to_string(), "value".to_string());
        assert_eq!(element.get_dataset("key"), Some("value"));
        assert_eq!(element.get_attribute("data-key"), Some("value"));
        
        element.remove_dataset("key");
        assert_eq!(element.get_dataset("key"), None);
        assert_eq!(element.get_attribute("data-key"), None);
    }
    
    #[test]
    fn test_element_types() {
        let div = Element::new(ElementId(1), "div".to_string());
        let span = Element::new(ElementId(2), "span".to_string());
        let img = Element::new(ElementId(3), "img".to_string());
        
        assert!(div.is_block_element());
        assert!(span.is_inline_element());
        assert!(img.is_void_element());
    }
    
    #[test]
    fn test_property_management() {
        let mut element = Element::new(ElementId(1), "div".to_string());
        
        element.set_property("checked".to_string(), serde_json::json!(true));
        assert!(element.has_property("checked"));
        assert_eq!(element.get_property("checked"), Some(&serde_json::json!(true)));
        
        element.remove_property("checked");
        assert!(!element.has_property("checked"));
    }
}
