//! Cache module - High-performance caching for NeuraCode

use dashmap::DashMap;
use parking_lot::RwLock;
use std::hash::Hash;
use std::time::{Duration, Instant};

/// Cache entry with expiration
struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

/// LRU Cache with TTL support
pub struct Cache<K, V> {
    map: DashMap<K, CacheEntry<V>>,
    max_size: usize,
    ttl: Duration,
    hits: RwLock<u64>,
    misses: RwLock<u64>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create a new cache
    pub fn new(max_size: usize, ttl_seconds: u64) -> Self {
        Self {
            map: DashMap::with_capacity(max_size),
            max_size,
            ttl: Duration::from_secs(ttl_seconds),
            hits: RwLock::new(0),
            misses: RwLock::new(0),
        }
    }
    
    /// Get a value from cache
    pub fn get(&self, key: &K) -> Option<V> {
        if let Some(entry) = self.map.get(key) {
            if entry.expires_at > Instant::now() {
                // Cache hit
                *self.hits.write() += 1;
                return Some(entry.value.clone());
            }
        }
        
        // Cache miss
        *self.misses.write() += 1;
        None
    }
    
    /// Insert a value into cache
    pub fn insert(&self, key: K, value: V) {
        // Evict expired entries if at capacity
        if self.map.len() >= self.max_size {
            self.evict_expired();
        }
        
        // If still at capacity, evict oldest
        if self.map.len() >= self.max_size {
            self.evict_oldest();
        }
        
        let entry = CacheEntry {
            value,
            expires_at: Instant::now() + self.ttl,
        };
        
        self.map.insert(key, entry);
    }
    
    /// Remove a key from cache
    pub fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key).map(|(_, entry)| entry.value)
    }
    
    /// Clear all entries
    pub fn clear(&self) {
        self.map.clear();
    }
    
    /// Get cache size
    pub fn len(&self) -> usize {
        self.map.len()
    }
    
    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let hits = *self.hits.read();
        let misses = *self.misses.read();
        let total = hits + misses;
        let hit_rate = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };
        
        CacheStats {
            size: self.map.len(),
            max_size: self.max_size,
            hits,
            misses,
            hit_rate,
        }
    }
    
    /// Evict expired entries
    fn evict_expired(&self) {
        let now = Instant::now();
        self.map.retain(|_, entry| entry.expires_at > now);
    }
    
    /// Evict oldest entry (simple approach: remove first)
    fn evict_oldest(&self) {
        if let Some(key) = self.map.iter().next().map(|e| e.key().clone()) {
            self.map.remove(&key);
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Copy)]
pub struct CacheStats {
    pub size: usize,
    pub max_size: usize,
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
}

/// Search result cache
pub type SearchCache = Cache<String, Vec<crate::types::SearchResult>>;

/// Context cache
pub type ContextCache = Cache<String, crate::types::ContextPackage>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_basic() {
        let cache: Cache<String, i32> = Cache::new(10, 60);
        
        cache.insert("key1".to_string(), 100);
        assert_eq!(cache.get(&"key1".to_string()), Some(100));
        
        cache.insert("key2".to_string(), 200);
        assert_eq!(cache.get(&"key2".to_string()), Some(200));
        
        assert_eq!(cache.len(), 2);
    }
    
    #[test]
    fn test_cache_expiration() {
        let cache: Cache<String, i32> = Cache::new(10, 0);
        
        cache.insert("key1".to_string(), 100);
        std::thread::sleep(Duration::from_millis(10));
        
        // Should be expired
        assert_eq!(cache.get(&"key1".to_string()), None);
    }
    
    #[test]
    fn test_cache_stats() {
        let cache: Cache<String, i32> = Cache::new(10, 60);
        
        cache.insert("key1".to_string(), 100);
        let _ = cache.get(&"key1".to_string()); // hit
        let _ = cache.get(&"key2".to_string()); // miss
        
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_rate, 0.5);
    }
}
