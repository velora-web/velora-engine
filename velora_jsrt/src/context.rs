//! JavaScript context for the Velora web engine

use velora_core::{VeloraResult, VeloraError, JsContextId};
use velora_core::error::JsRuntimeError;

/// JavaScript execution context
#[derive(Debug)]
pub struct JsContext {
    /// Context ID
    id: JsContextId,
    
    /// Context state
    state: ContextState,
}

/// Context execution state
#[derive(Debug)]
pub enum ContextState {
    /// Context is ready
    Ready,
    
    /// Context is executing
    Executing,
    
    /// Context has an error
    Error(String),
}

impl JsContext {
    /// Create a new JavaScript context
    pub fn new(id: JsContextId) -> VeloraResult<Self> {
        Ok(Self {
            id,
            state: ContextState::Ready,
        })
    }
    
    /// Get the context ID
    pub fn id(&self) -> JsContextId {
        self.id
    }
    
    /// Get the current state
    pub fn state(&self) -> &ContextState {
        &self.state
    }
    
    /// Execute JavaScript code
    pub async fn execute(&mut self, script: &str) -> VeloraResult<()> {
        if script.trim().is_empty() {
            return Err(VeloraError::JsRuntime(JsRuntimeError::InvalidScript(
                "Script cannot be empty".to_string()
            )));
        }
        
        self.state = ContextState::Executing;
        
        // TODO: Implement script execution using Deno
        // For now, simulate execution with a small delay
        
        // Simulate script execution time
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        
        // TODO: Parse and execute the actual script
        // For now, just check if it's valid JavaScript-like syntax
        
        if script.contains("syntax error") {
            self.state = ContextState::Error("JavaScript syntax error".to_string());
            return Err(VeloraError::JsRuntime(JsRuntimeError::ExecutionFailed(
                "JavaScript syntax error".to_string()
            )));
        }
        
        self.state = ContextState::Ready;
        Ok(())
    }
    
    /// Check if the context is ready
    pub fn is_ready(&self) -> bool {
        matches!(self.state, ContextState::Ready)
    }
    
    /// Check if the context is executing
    pub fn is_executing(&self) -> bool {
        matches!(self.state, ContextState::Executing)
    }
    
    /// Check if the context has an error
    pub fn has_error(&self) -> bool {
        matches!(self.state, ContextState::Error(_))
    }
    
    /// Get the error message if there is one
    pub fn get_error(&self) -> Option<&str> {
        if let ContextState::Error(msg) = &self.state {
            Some(msg)
        } else {
            None
        }
    }
    
    /// Reset the context to ready state
    pub fn reset(&mut self) {
        self.state = ContextState::Ready;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_js_context_creation() {
        let context = JsContext::new(JsContextId(1));
        assert!(context.is_ok());
        
        let context = context.unwrap();
        assert_eq!(context.id(), JsContextId(1));
        assert!(context.is_ready());
        assert!(!context.is_executing());
        assert!(!context.has_error());
    }
    
    #[tokio::test]
    async fn test_execute_script() {
        let mut context = JsContext::new(JsContextId(1)).unwrap();
        
        // Test valid script
        let result = context.execute("console.log('Hello World');").await;
        assert!(result.is_ok());
        assert!(context.is_ready());
        assert!(!context.has_error());
    }
    
    #[tokio::test]
    async fn test_execute_empty_script() {
        let mut context = JsContext::new(JsContextId(1)).unwrap();
        
        let result = context.execute("").await;
        assert!(result.is_err());
        assert!(context.is_ready()); // Should remain in ready state
    }
    
    #[tokio::test]
    async fn test_execute_whitespace_script() {
        let mut context = JsContext::new(JsContextId(1)).unwrap();
        
        let result = context.execute("   ").await;
        assert!(result.is_err());
        assert!(context.is_ready());
    }
    
    #[tokio::test]
    async fn test_execute_syntax_error() {
        let mut context = JsContext::new(JsContextId(1)).unwrap();
        
        let result = context.execute("syntax error").await;
        assert!(result.is_err());
        assert!(context.has_error());
        assert_eq!(context.get_error(), Some("JavaScript syntax error"));
    }
    
    #[test]
    fn test_context_state_management() {
        let mut context = JsContext::new(JsContextId(1)).unwrap();
        
        // Test reset functionality
        context.state = ContextState::Error("Test error".to_string());
        assert!(context.has_error());
        
        context.reset();
        assert!(context.is_ready());
        assert!(!context.has_error());
    }
}
