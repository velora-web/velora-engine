//! Network layer for the Velora web engine
//! 
//! This crate provides HTTP client functionality, resource loading,
//! and network request management.

pub mod client;
pub mod resource;
pub mod cache;

pub use client::HttpClient;
pub use resource::ResourceLoader;
pub use cache::ResourceCache;

/// Re-export commonly used items for convenience
pub mod prelude {
    pub use super::client::HttpClient;
    pub use super::resource::ResourceLoader;
    pub use super::cache::ResourceCache;
}
