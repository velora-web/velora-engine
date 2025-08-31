//! JavaScript runtime for the Velora web engine

use velora_core::{VeloraResult, VeloraError, JsContextId};
use velora_core::error::JsRuntimeError;
use super::context::JsContext;

/// JavaScript runtime using Deno
#[derive(Debug)]
pub struct JsRuntime {
    /// JavaScript contexts
    contexts: std::collections::HashMap<JsContextId, JsContext>,
    
    /// Next available context ID
    next_context_id: u64,
}

impl JsRuntime {
    /// Create a new JavaScript runtime
    pub fn new() -> VeloraResult<Self> {
        Ok(Self {
            contexts: std::collections::HashMap::new(),
            next_context_id: 1,
        })
    }
    
    /// Create a new JavaScript context
    pub fn create_context(&mut self) -> VeloraResult<JsContextId> {
        let context_id = JsContextId(self.next_context_id);
        let context = JsContext::new(context_id)?;
        
        self.contexts.insert(context_id, context);
        self.next_context_id += 1;
        
        Ok(context_id)
    }
    
    /// Execute JavaScript code in a context
    pub async fn execute_script(&self, context_id: JsContextId, script: &str) -> VeloraResult<()> {
        // TODO: Implement script execution using Deno
        // For now, validate the context exists and return success
        
        if !self.contexts.contains_key(&context_id) {
            return Err(VeloraError::JsRuntime(JsRuntimeError::ExecutionFailed(
                format!("Context {} not found", context_id.0)
            )));
        }
        
        if script.trim().is_empty() {
            return Err(VeloraError::JsRuntime(JsRuntimeError::InvalidScript(
                "Script cannot be empty".to_string()
            )));
        }
        
        // TODO: Actually execute the script using Deno
        // For now, just return success
        Ok(())
    }
    
    /// Get a context by ID
    pub fn get_context(&self, context_id: JsContextId) -> Option<&JsContext> {
        self.contexts.get(&context_id)
    }
    
    /// Get a mutable reference to a context by ID
    pub fn get_context_mut(&mut self, context_id: JsContextId) -> Option<&mut JsContext> {
        self.contexts.get_mut(&context_id)
    }
    
    /// Remove a context
    pub fn remove_context(&mut self, context_id: JsContextId) -> bool {
        self.contexts.remove(&context_id).is_some()
    }
    
    /// Get the number of contexts
    pub fn context_count(&self) -> usize {
        self.contexts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_js_runtime_creation() {
        let runtime = JsRuntime::new();
        assert!(runtime.is_ok());
        
        let runtime = runtime.unwrap();
        assert_eq!(runtime.context_count(), 0);
    }
    
    #[tokio::test]
    async fn test_create_context() {
        let mut runtime = JsRuntime::new().unwrap();
        
        let context_id = runtime.create_context();
        assert!(context_id.is_ok());
        
        let context_id = context_id.unwrap();
        assert_eq!(runtime.context_count(), 1);
        assert!(runtime.get_context(context_id).is_some());
    }
    
    #[tokio::test]
    async fn test_execute_script() {
        let mut runtime = JsRuntime::new().unwrap();
        let context_id = runtime.create_context().unwrap();
        
        // Test valid script
        let result = runtime.execute_script(context_id, "console.log('Hello World');").await;
        assert!(result.is_ok());
        
        // Test empty script
        let result = runtime.execute_script(context_id, "").await;
        assert!(result.is_err());
        
        // Test whitespace-only script
        let result = runtime.execute_script(context_id, "   ").await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_execute_script_invalid_context() {
        let runtime = JsRuntime::new().unwrap();
        let invalid_context_id = JsContextId(999);
        
        let result = runtime.execute_script(invalid_context_id, "console.log('test');").await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_remove_context() {
        let mut runtime = JsRuntime::new().unwrap();
        let context_id = runtime.create_context().unwrap();
        
        assert_eq!(runtime.context_count(), 1);
        
        assert!(runtime.remove_context(context_id));
        assert_eq!(runtime.context_count(), 0);
        assert!(runtime.get_context(context_id).is_none());
    }
}
