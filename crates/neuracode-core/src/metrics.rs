//! Metrics module - Performance monitoring for NeuraCode

use parking_lot::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance metrics
pub struct Metrics {
    counters: RwLock<HashMap<String, u64>>,
    timers: RwLock<HashMap<String, TimerStats>>,
    gauges: RwLock<HashMap<String, f64>>,
}

/// Timer statistics
#[derive(Debug, Clone)]
pub struct TimerStats {
    pub count: u64,
    pub total_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
}

impl TimerStats {
    fn new() -> Self {
        Self {
            count: 0,
            total_duration: Duration::ZERO,
            min_duration: Duration::MAX,
            max_duration: Duration::ZERO,
        }
    }
    
    fn record(&mut self, duration: Duration) {
        self.count += 1;
        self.total_duration += duration;
        self.min_duration = self.min_duration.min(duration);
        self.max_duration = self.max_duration.max(duration);
    }
    
    pub fn avg_duration(&self) -> Duration {
        if self.count > 0 {
            self.total_duration / self.count as u32
        } else {
            Duration::ZERO
        }
    }
}

impl Metrics {
    /// Create new metrics collector
    pub fn new() -> Self {
        Self {
            counters: RwLock::new(HashMap::new()),
            timers: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
        }
    }
    
    /// Increment a counter
    pub fn increment(&self, name: &str) {
        let mut counters = self.counters.write();
        *counters.entry(name.to_string()).or_insert(0) += 1;
    }
    
    /// Record a timer
    pub fn record_timer(&self, name: &str, duration: Duration) {
        let mut timers = self.timers.write();
        timers
            .entry(name.to_string())
            .or_insert_with(TimerStats::new)
            .record(duration);
    }
    
    /// Set a gauge value
    pub fn set_gauge(&self, name: &str, value: f64) {
        self.gauges.write().insert(name.to_string(), value);
    }
    
    /// Get counter value
    pub fn get_counter(&self, name: &str) -> u64 {
        self.counters.read().get(name).copied().unwrap_or(0)
    }
    
    /// Get timer stats
    pub fn get_timer(&self, name: &str) -> Option<TimerStats> {
        self.timers.read().get(name).cloned()
    }
    
    /// Get gauge value
    pub fn get_gauge(&self, name: &str) -> f64 {
        self.gauges.read().get(name).copied().unwrap_or(0.0)
    }
    
    /// Get all metrics as a report
    pub fn report(&self) -> MetricsReport {
        MetricsReport {
            counters: self.counters.read().clone(),
            timers: self.timers.read().clone(),
            gauges: self.gauges.read().clone(),
        }
    }
    
    /// Reset all metrics
    pub fn reset(&self) {
        self.counters.write().clear();
        self.timers.write().clear();
        self.gauges.write().clear();
    }
}

/// Metrics report
#[derive(Debug, Clone)]
pub struct MetricsReport {
    pub counters: HashMap<String, u64>,
    pub timers: HashMap<String, TimerStats>,
    pub gauges: HashMap<String, f64>,
}

/// Timer guard for automatic timing
pub struct TimerGuard<'a> {
    metrics: &'a Metrics,
    name: String,
    start: Instant,
}

impl<'a> TimerGuard<'a> {
    pub fn new(metrics: &'a Metrics, name: &str) -> Self {
        Self {
            metrics,
            name: name.to_string(),
            start: Instant::now(),
        }
    }
}

impl<'a> Drop for TimerGuard<'a> {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        self.metrics.record_timer(&self.name, duration);
    }
}

/// Macro for timing a block
#[macro_export]
macro_rules! timed {
    ($metrics:expr, $name:expr, $block:block) => {{
        let _guard = $crate::metrics::TimerGuard::new($metrics, $name);
        $block
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_counter() {
        let metrics = Metrics::new();
        
        metrics.increment("requests");
        metrics.increment("requests");
        metrics.increment("errors");
        
        assert_eq!(metrics.get_counter("requests"), 2);
        assert_eq!(metrics.get_counter("errors"), 1);
        assert_eq!(metrics.get_counter("unknown"), 0);
    }
    
    #[test]
    fn test_timer() {
        let metrics = Metrics::new();
        
        metrics.record_timer("query", Duration::from_millis(100));
        metrics.record_timer("query", Duration::from_millis(200));
        
        let stats = metrics.get_timer("query").unwrap();
        assert_eq!(stats.count, 2);
        assert_eq!(stats.avg_duration(), Duration::from_millis(150));
    }
    
    #[test]
    fn test_gauge() {
        let metrics = Metrics::new();
        
        metrics.set_gauge("memory_usage", 1024.0);
        
        assert_eq!(metrics.get_gauge("memory_usage"), 1024.0);
    }
}
