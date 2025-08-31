//! Core types and utilities for the Velora web engine
//! 
//! This crate provides the foundational types, error handling, and utilities
//! that are shared across all subsystems of the web engine.

pub mod error;
pub mod types;
pub mod utils;

pub use error::{VeloraError, VeloraResult};
pub use types::*;
pub use utils::*;

/// Re-export commonly used items for convenience
pub mod prelude {
    pub use super::error::{VeloraError, VeloraResult};
    pub use super::types::{NodeId, ElementId, StyleId, LayoutId, PaintId};
    pub use super::utils::IdGenerator;
}
