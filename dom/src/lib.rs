//! Minimal DOM types placeholder crate.

/// Very small placeholder node type to establish crate boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeId(pub u64);

/// Placeholder document type.
#[derive(Debug, Default)]
pub struct Document {
    pub root_id: Option<NodeId>,
}

