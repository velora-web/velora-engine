//! DOM bindings for JavaScript in the Velora web engine

use velora_core::{VeloraResult, VeloraError};
use velora_core::error::JsRuntimeError;

/// DOM bindings for JavaScript
#[derive(Debug)]
pub struct DomBindings {
    /// Binding registry
    bindings: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl DomBindings {
    /// Create new DOM bindings
    pub fn new() -> VeloraResult<Self> {
        Ok(Self {
            bindings: std::collections::HashMap::new(),
        })
    }
    
    /// Register a binding
    pub fn register_binding(&mut self, name: &str, binding: Box<dyn std::any::Any>) -> VeloraResult<()> {
        if self.bindings.contains_key(name) {
            return Err(VeloraError::JsRuntime(JsRuntimeError::InvalidScript(
                format!("Binding '{}' already exists", name)
            )));
        }
        
        self.bindings.insert(name.to_string(), binding);
        Ok(())
    }
    
    /// Get a binding
    pub fn get_binding(&self, name: &str) -> Option<&Box<dyn std::any::Any>> {
        self.bindings.get(name)
    }
    
    /// Check if a binding exists
    pub fn has_binding(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }
    
    /// Remove a binding
    pub fn remove_binding(&mut self, name: &str) -> bool {
        self.bindings.remove(name).is_some()
    }
    
    /// Get the number of bindings
    pub fn binding_count(&self) -> usize {
        self.bindings.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dom_bindings_creation() {
        let bindings = DomBindings::new();
        assert!(bindings.is_ok());
        
        let bindings = bindings.unwrap();
        assert_eq!(bindings.binding_count(), 0);
    }
    
    #[test]
    fn test_register_binding() {
        let mut bindings = DomBindings::new().unwrap();
        
        let test_binding = Box::new("test_value");
        let result = bindings.register_binding("test", test_binding);
        assert!(result.is_ok());
        
        assert_eq!(bindings.binding_count(), 1);
        assert!(bindings.has_binding("test"));
    }
    
    #[test]
    fn test_duplicate_binding() {
        let mut bindings = DomBindings::new().unwrap();
        
        let binding1 = Box::new("value1");
        let binding2 = Box::new("value2");
        
        bindings.register_binding("test", binding1).unwrap();
        let result = bindings.register_binding("test", binding2);
        assert!(result.is_err());
        
        assert_eq!(bindings.binding_count(), 1);
    }
    
    #[test]
    fn test_get_binding() {
        let mut bindings = DomBindings::new().unwrap();
        
        let test_value = "test_value";
        let test_binding = Box::new(test_value);
        bindings.register_binding("test", test_binding).unwrap();
        
        let retrieved = bindings.get_binding("test");
        assert!(retrieved.is_some());
        
        // Note: We can't easily test the actual value due to Any trait limitations
        // In a real implementation, you'd use downcast_ref or similar
    }
    
    #[test]
    fn test_remove_binding() {
        let mut bindings = DomBindings::new().unwrap();
        
        let test_binding = Box::new("test_value");
        bindings.register_binding("test", test_binding).unwrap();
        assert_eq!(bindings.binding_count(), 1);
        
        assert!(bindings.remove_binding("test"));
        assert_eq!(bindings.binding_count(), 0);
        assert!(!bindings.has_binding("test"));
    }
}
