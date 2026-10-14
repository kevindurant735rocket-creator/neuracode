//! Cache module - High-performance caching for NeuraCode

use dashmap::DashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Cache entry with expiration + LRU timestamp
struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
    last_access: Instant,
}

/// LRU Cache with TTL support
pub struct Cache<K, V> {
    map: DashMap<K, CacheEntry<V>>,
    max_size: usize,
    ttl: Duration,
    hits: AtomicU64,
    misses: AtomicU64,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create a new cache
    pub fn new(max_size: usize, ttl_seconds: u64) -> Self {
        Self {
            map: DashMap::with_capacity(max_size.max(1)),
            max_size,
            ttl: Duration::from_secs(ttl_seconds),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    /// Get a value from cache
    pub fn get(&self, key: &K) -> Option<V> {
        let now = Instant::now();
        if let Some(mut entry) = self.map.get_mut(key) {
            if entry.expires_at > now {
                // Cache hit — update LRU timestamp
                self.hits.fetch_add(1, Ordering::Relaxed);
                entry.last_access = now;
                return Some(entry.value.clone());
            }
            // Expired — remove to avoid zombie occupancy
            drop(entry);
            self.map.remove(key);
        }

        // Cache miss
        self.misses.fetch_add(1, Ordering::Relaxed);
        None
    }

    /// Insert a value into cache
    pub fn insert(&self, key: K, value: V) {
        if self.max_size == 0 {
            return;
        }
        // Evict expired entries if at capacity
        if self.map.len() >= self.max_size {
            self.evict_expired();
        }

        // If still at capacity, evict true LRU
        if self.map.len() >= self.max_size {
            self.evict_oldest();
        }

        let now = Instant::now();
        let entry = CacheEntry {
            value,
            expires_at: now + self.ttl,
            last_access: now,
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

    /// Get cache statistics (excludes expired entries from size)
    pub fn stats(&self) -> CacheStats {
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
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

    /// Evict oldest entry by last_access (true LRU)
    fn evict_oldest(&self) {
        let oldest = self
            .map
            .iter()
            .min_by_key(|e| e.value().last_access)
            .map(|e| e.key().clone());
        if let Some(key) = oldest {
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
