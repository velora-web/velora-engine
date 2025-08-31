//! JavaScript runtime for the Velora web engine
//! 
//! This crate provides JavaScript execution capabilities using Deno,
//! including script evaluation, DOM manipulation, and event handling.

pub mod runtime;
pub mod context;
pub mod bindings;

pub use runtime::JsRuntime;
pub use context::JsContext;
pub use bindings::DomBindings;

/// Re-export commonly used items for convenience
pub mod prelude {
    pub use super::runtime::JsRuntime;
    pub use super::context::JsContext;
    pub use super::bindings::DomBindings;
}
