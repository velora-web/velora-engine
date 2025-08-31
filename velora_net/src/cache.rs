//! Resource caching for the Velora web engine

use velora_core::VeloraResult;
use super::resource::CachedResource;
use std::collections::HashMap;

/// Resource cache for storing fetched resources
#[derive(Debug)]
pub struct ResourceCache {
    /// Cached resources
    resources: HashMap<String, CachedResource>,
    
    /// Maximum cache size in bytes
    max_size: usize,
    
    /// Current cache size in bytes
    current_size: usize,
}

impl ResourceCache {
    /// Create a new resource cache
    pub fn new(max_size: usize) -> Self {
        Self {
            resources: HashMap::new(),
            max_size,
            current_size: 0,
        }
    }
    
    /// Get a cached resource
    pub fn get(&self, url: &str) -> Option<&CachedResource> {
        self.resources.get(url)
    }
    
    /// Store a resource in the cache
    pub fn store(&mut self, url: String, resource: CachedResource) -> VeloraResult<()> {
        // TODO: Implement cache eviction when full
        let resource_size = resource.data.len();
        
        // Check if adding this resource would exceed max size
        if self.current_size + resource_size > self.max_size {
            // TODO: Implement proper cache eviction strategy
            return Err(velora_core::VeloraError::Network(
                velora_core::error::NetworkError::RequestFailed("Cache full".to_string())
            ));
        }
        
        self.resources.insert(url, resource);
        self.current_size += resource_size;
        Ok(())
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.resources.clear();
        self.current_size = 0;
    }
    
    /// Get current cache size in bytes
    pub fn current_size(&self) -> usize {
        self.current_size
    }
    
    /// Get maximum cache size in bytes
    pub fn max_size(&self) -> usize {
        self.max_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resource_cache_creation() {
        let cache = ResourceCache::new(1024);
        assert_eq!(cache.max_size(), 1024);
        assert_eq!(cache.current_size(), 0);
    }
    
    #[test]
    fn test_cache_store_and_get() {
        let mut cache = ResourceCache::new(1024);
        let resource = CachedResource {
            data: b"Hello, World!".to_vec(),
            content_type: "text/plain".to_string(),
            timestamp: std::time::SystemTime::now(),
        };
        
        let result = cache.store("test.txt".to_string(), resource);
        assert!(result.is_ok());
        
        let cached = cache.get("test.txt");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().content_type, "text/plain");
    }
    
    #[test]
    fn test_cache_clear() {
        let mut cache = ResourceCache::new(1024);
        let resource = CachedResource {
            data: b"Hello, World!".to_vec(),
            content_type: "text/plain".to_string(),
            timestamp: std::time::SystemTime::now(),
        };
        
        cache.store("test.txt".to_string(), resource).unwrap();
        assert_eq!(cache.current_size(), 13); // "Hello, World!" is 13 bytes
        
        cache.clear();
        assert_eq!(cache.current_size(), 0);
        assert!(cache.get("test.txt").is_none());
    }
}
