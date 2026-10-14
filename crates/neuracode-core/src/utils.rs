//! Utility functions for NeuraCode

use std::path::Path;

/// Calculate file hash for caching — streaming, avoids loading whole file
pub fn file_hash(path: &Path) -> Option<String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::io::Read;

    let mut file = std::fs::File::open(path).ok()?;
    let mut hasher = DefaultHasher::new();
    // Include length prefix to reduce collision across sizes
    if let Ok(meta) = file.metadata() {
        meta.len().hash(&mut hasher);
    }
    let mut buf = [0u8; 8192];
    loop {
        match file.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => buf[..n].hash(&mut hasher),
            Err(_) => return None,
        }
    }
    Some(format!("{:x}", hasher.finish()))
}

/// Format duration in human-readable form
pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();

    if secs > 60 {
        let mins = secs / 60;
        let secs = secs % 60;
        format!("{}m {}s", mins, secs)
    } else if secs > 0 {
        format!("{}.{}s", secs, millis / 100)
    } else {
        format!("{}ms", millis)
    }
}

/// Format bytes in human-readable form
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let exp = (bytes as f64).log(1024.0).min(UNITS.len() as f64 - 1.0) as usize;
    let value = bytes as f64 / 1024f64.powi(exp as i32);

    format!("{:.2} {}", value, UNITS[exp])
}

/// Truncate string to max length (char-based, UTF-8 safe)
pub fn truncate(s: &str, max_len: usize) -> String {
    let count = s.chars().count();
    if count <= max_len {
        s.to_string()
    } else {
        let keep = max_len.saturating_sub(3);
        let truncated: String = s.chars().take(keep).collect();
        format!("{}...", truncated)
    }
}

/// Check if a path should be ignored — matches full components, not substrings
pub fn should_ignore(path: &Path, patterns: &[String]) -> bool {
    let components: Vec<String> = path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();
    let path_str = path.to_string_lossy().to_string();
    patterns.iter().any(|p| {
        // Exact component match (fixes `target` killing `my-target-file.rs`)
        if components.iter().any(|c| c == p) {
            return true;
        }
        // Allow explicit glob-ish suffix/prefix patterns, else substring on separators
        if p.contains('/') || p.starts_with('.') {
            return path_str.contains(p);
        }
        // Fallback: match as path segment
        path_str.split('/').any(|seg| seg == p)
    })
}

/// Normalize path for cross-platform compatibility
pub fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// Get file extension
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
}

/// Check if a string is a valid identifier
pub fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    if !first.is_alphabetic() && first != '_' {
        return false;
    }

    chars.all(|c| c.is_alphanumeric() || c == '_')
}

/// Convert snake_case to camelCase
pub fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// Convert camelCase to snake_case
pub fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.extend(c.to_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

/// Calculate similarity between two strings (0.0 to 1.0)
pub fn string_similarity(a: &str, b: &str) -> f32 {
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();

    if a_lower == b_lower {
        return 1.0;
    }

    // Simple Jaccard similarity
    let a_words: std::collections::HashSet<&str> = a_lower.split_whitespace().collect();
    let b_words: std::collections::HashSet<&str> = b_lower.split_whitespace().collect();

    let intersection: std::collections::HashSet<_> = a_words.intersection(&b_words).collect();
    let union: std::collections::HashSet<_> = a_words.union(&b_words).collect();

    if union.is_empty() {
        0.0
    } else {
        intersection.len() as f32 / union.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_to_camel() {
        assert_eq!(snake_to_camel("hello_world"), "helloWorld");
        assert_eq!(snake_to_camel("foo_bar_baz"), "fooBarBaz");
        assert_eq!(snake_to_camel("single"), "single");
    }

    #[test]
    fn test_camel_to_snake() {
        assert_eq!(camel_to_snake("helloWorld"), "hello_world");
        assert_eq!(camel_to_snake("fooBarBaz"), "foo_bar_baz");
        assert_eq!(camel_to_snake("single"), "single");
    }

    #[test]
    fn test_string_similarity() {
        assert_eq!(string_similarity("hello world", "hello world"), 1.0);
        assert_eq!(string_similarity("hello", "world"), 0.0);
        assert!(string_similarity("hello world", "hello there") > 0.0);
    }

    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("foo"));
        assert!(is_valid_identifier("foo_bar"));
        assert!(is_valid_identifier("_foo"));
        assert!(!is_valid_identifier("123"));
        assert!(!is_valid_identifier(""));
    }
}
