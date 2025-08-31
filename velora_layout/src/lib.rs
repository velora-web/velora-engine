//! Layout engine for the Velora web engine
//! 
//! This crate provides layout calculations for DOM elements,
//! including box model, flexbox, and grid layouts.

pub mod box_model;
pub mod flexbox;
pub mod grid;
pub mod layout_tree;

pub use box_model::{BoxModel, BoxSizing};
pub use flexbox::{FlexContainer, FlexItem, FlexboxLayout};
pub use grid::{GridContainer, GridItem, GridLayout};
pub use layout_tree::{LayoutTree, LayoutNode};

/// Re-export commonly used items for convenience
pub mod prelude {
    pub use super::box_model::{BoxModel, BoxSizing};
    pub use super::flexbox::{FlexContainer, FlexItem, FlexboxLayout};
    pub use super::grid::{GridContainer, GridItem, GridLayout};
    pub use super::layout_tree::{LayoutTree, LayoutNode};
}
