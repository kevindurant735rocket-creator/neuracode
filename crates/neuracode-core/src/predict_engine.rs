//! Predict Engine - Anticipatory context preparation
//!
//! This module predicts what context an agent will need based on the task,
//! and prepares it before the agent asks.

use crate::error::Result;
use crate::types::*;
use crate::NeuraCodeConfig;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{debug, info, instrument};

/// Predict Engine - Predicts and prepares context for tasks
#[allow(dead_code)]
pub struct PredictEngine {
    /// Configuration
    config: NeuraCodeConfig,

    /// Task patterns learned from history (bounded)
    task_patterns: HashMap<String, TaskType>,

    /// Context cache (interior mutability so predict(&self) can populate)
    context_cache: parking_lot::RwLock<HashMap<TaskType, ContextPackage>>,

    /// Ordered classification rules: (pattern, type, weight) — deterministic
    rules: Vec<(String, TaskType, u32)>,
}

impl PredictEngine {
    /// Create a new PredictEngine
    pub fn new(config: &NeuraCodeConfig) -> Self {
        let mut task_patterns = HashMap::new();

        // Initialize with common patterns
        Self::init_task_patterns(&mut task_patterns);
        let rules = Self::build_rules();

        Self {
            config: config.clone(),
            task_patterns,
            context_cache: parking_lot::RwLock::new(HashMap::new()),
            rules,
        }
    }

    /// Initialize task patterns
    fn init_task_patterns(patterns: &mut HashMap<String, TaskType>) {
        // Bug fix patterns
        patterns.insert("fix".to_string(), TaskType::BugFix);
        patterns.insert("bug".to_string(), TaskType::BugFix);
        patterns.insert("error".to_string(), TaskType::BugFix);
        patterns.insert("crash".to_string(), TaskType::BugFix);
        patterns.insert("broken".to_string(), TaskType::BugFix);
        patterns.insert("issue".to_string(), TaskType::BugFix);
        patterns.insert("problem".to_string(), TaskType::BugFix);

        // Refactor patterns
        patterns.insert("refactor".to_string(), TaskType::Refactor);
        patterns.insert("clean".to_string(), TaskType::Refactor);
        patterns.insert("simplify".to_string(), TaskType::Refactor);
        patterns.insert("restructure".to_string(), TaskType::Refactor);

        // New feature patterns
        patterns.insert("add".to_string(), TaskType::NewFeature);
        patterns.insert("feature".to_string(), TaskType::NewFeature);
        patterns.insert("implement".to_string(), TaskType::NewFeature);
        patterns.insert("create".to_string(), TaskType::NewFeature);
        patterns.insert("new".to_string(), TaskType::NewFeature);

        // Code review patterns
        patterns.insert("review".to_string(), TaskType::CodeReview);
        patterns.insert("check".to_string(), TaskType::CodeReview);
        patterns.insert("audit".to_string(), TaskType::CodeReview);

        // Performance patterns
        patterns.insert("performance".to_string(), TaskType::PerformanceOptimization);
        patterns.insert("optimize".to_string(), TaskType::PerformanceOptimization);
        patterns.insert("speed".to_string(), TaskType::PerformanceOptimization);
        patterns.insert("slow".to_string(), TaskType::PerformanceOptimization);
        patterns.insert("cache".to_string(), TaskType::PerformanceOptimization);

        // Security patterns
        patterns.insert("security".to_string(), TaskType::SecurityFix);
        patterns.insert("vulnerability".to_string(), TaskType::SecurityFix);
        patterns.insert("xss".to_string(), TaskType::SecurityFix);
        patterns.insert("injection".to_string(), TaskType::SecurityFix);
        patterns.insert("auth".to_string(), TaskType::SecurityFix);

        // Documentation patterns
        patterns.insert("document".to_string(), TaskType::Documentation);
        patterns.insert("docs".to_string(), TaskType::Documentation);
        patterns.insert("comment".to_string(), TaskType::Documentation);
        patterns.insert("readme".to_string(), TaskType::Documentation);

        // Testing patterns
        patterns.insert("test".to_string(), TaskType::Testing);
        patterns.insert("spec".to_string(), TaskType::Testing);
        patterns.insert("coverage".to_string(), TaskType::Testing);

        // Architecture patterns
        patterns.insert("architecture".to_string(), TaskType::Architecture);
        patterns.insert("design".to_string(), TaskType::Architecture);
        patterns.insert("pattern".to_string(), TaskType::Architecture);
    }

    /// Ordered deterministic rules with weights (higher = more specific)
    fn build_rules() -> Vec<(String, TaskType, u32)> {
        vec![
            ("vulnerability".into(), TaskType::SecurityFix, 10),
            ("injection".into(), TaskType::SecurityFix, 10),
            ("xss".into(), TaskType::SecurityFix, 10),
            ("performance".into(), TaskType::PerformanceOptimization, 9),
            ("optimize".into(), TaskType::PerformanceOptimization, 9),
            ("architecture".into(), TaskType::Architecture, 9),
            ("refactor".into(), TaskType::Refactor, 8),
            ("restructure".into(), TaskType::Refactor, 8),
            ("coverage".into(), TaskType::Testing, 8),
            ("document".into(), TaskType::Documentation, 7),
            ("readme".into(), TaskType::Documentation, 7),
            ("review".into(), TaskType::CodeReview, 7),
            ("audit".into(), TaskType::CodeReview, 7),
            ("crash".into(), TaskType::BugFix, 6),
            ("broken".into(), TaskType::BugFix, 6),
            ("error".into(), TaskType::BugFix, 5),
            ("bug".into(), TaskType::BugFix, 5),
            ("fix".into(), TaskType::BugFix, 4),
            ("issue".into(), TaskType::BugFix, 4),
            ("problem".into(), TaskType::BugFix, 4),
            ("security".into(), TaskType::SecurityFix, 4),
            ("auth".into(), TaskType::SecurityFix, 3),
            ("test".into(), TaskType::Testing, 3),
            ("spec".into(), TaskType::Testing, 3),
            ("design".into(), TaskType::Architecture, 3),
            ("pattern".into(), TaskType::Architecture, 3),
            ("clean".into(), TaskType::Refactor, 2),
            ("simplify".into(), TaskType::Refactor, 2),
            ("slow".into(), TaskType::PerformanceOptimization, 2),
            ("speed".into(), TaskType::PerformanceOptimization, 2),
            ("cache".into(), TaskType::PerformanceOptimization, 2),
            ("feature".into(), TaskType::NewFeature, 2),
            ("implement".into(), TaskType::NewFeature, 2),
            ("create".into(), TaskType::NewFeature, 1),
            ("add".into(), TaskType::NewFeature, 1),
            ("new".into(), TaskType::NewFeature, 1),
            ("check".into(), TaskType::CodeReview, 1),
            ("docs".into(), TaskType::Documentation, 1),
            ("comment".into(), TaskType::Documentation, 1),
        ]
    }

    /// Predict context for a task
    #[instrument(skip(self))]
    pub async fn predict(&self, task: &str) -> Result<ContextPackage> {
        info!("Predicting context for task: {}", task);

        // Classify the task
        let task_type = self.classify_task(task);
        debug!("Classified as: {:?}", task_type);

        // Check cache
        if let Some(cached) = self.context_cache.read().get(&task_type).cloned() {
            debug!("Using cached context");
            return Ok(cached);
        }

        // Build context package
        let context = self.build_context(task_type).await?;

        // Populate cache (bounded to 10 task types — naturally bounded)
        self.context_cache
            .write()
            .insert(task_type, context.clone());

        Ok(context)
    }

    /// Classify a task — deterministic weighted match, no HashMap iteration randomness
    pub fn classify_task(&self, task: &str) -> TaskType {
        let task_lower = task.to_lowercase();

        // Exact learned-phrase match first (bounded map)
        if let Some(t) = self.task_patterns.get(&task_lower) {
            return *t;
        }
        let mut best: Option<(u32, usize, TaskType)> = None;
        for (idx, (pat, ty, w)) in self.rules.iter().enumerate() {
            if task_lower.contains(pat.as_str()) {
                let score = *w * 1000 + (100 - pat.len() as u32);
                match best {
                    Some((bs, _, _)) if bs >= score => {}
                    _ => best = Some((score, idx, *ty)),
                }
            }
        }
        best.map(|(_, _, t)| t).unwrap_or(TaskType::General)
    }

    /// Build context package for a task type
    async fn build_context(&self, task_type: TaskType) -> Result<ContextPackage> {
        let mut context = ContextPackage {
            task_type,
            relevant_files: Vec::new(),
            test_files: Vec::new(),
            recent_commits: Vec::new(),
            error_logs: Vec::new(),
            similar_fixes: Vec::new(),
            architecture: None,
            dependencies: Vec::new(),
        };

        match task_type {
            TaskType::BugFix => {
                // For bug fixes, we need:
                // - Recent commits (what changed recently)
                // - Error logs
                // - Test files
                context.recent_commits = self.get_recent_commits(10).await;
                context.error_logs = self.get_error_logs().await;
                context.test_files = self.find_test_files().await;
                context.similar_fixes = self.find_similar_fixes().await;
            }
            TaskType::Refactor => {
                // For refactoring, we need:
                // - Dependency graph
                // - Test coverage
                // - Architecture info
                context.test_files = self.find_test_files().await;
                context.architecture = self.get_architecture_info().await;
                context.dependencies = self.get_dependency_info().await;
            }
            TaskType::NewFeature => {
                // For new features, we need:
                // - Architecture info
                // - Similar existing features
                // - Test patterns
                context.architecture = self.get_architecture_info().await;
                context.test_files = self.find_test_files().await;
                context.relevant_files = self.find_similar_features().await;
            }
            TaskType::CodeReview => {
                // For code review, we need:
                // - Recent changes
                // - Related tests
                // - Architecture context
                context.recent_commits = self.get_recent_commits(5).await;
                context.test_files = self.find_test_files().await;
                context.architecture = self.get_architecture_info().await;
            }
            TaskType::PerformanceOptimization => {
                // For performance, we need:
                // - Hot files
                // - Dependencies
                // - Architecture
                context.architecture = self.get_architecture_info().await;
                context.dependencies = self.get_dependency_info().await;
                context.relevant_files = self.find_hot_files().await;
            }
            TaskType::SecurityFix => {
                // For security, we need:
                // - Auth-related files
                // - Input handling
                // - Dependencies
                context.relevant_files = self.find_security_files().await;
                context.dependencies = self.get_dependency_info().await;
            }
            TaskType::Documentation => {
                // For documentation, we need:
                // - Public APIs
                // - Main modules
                context.relevant_files = self.find_public_apis().await;
            }
            TaskType::Testing => {
                // For testing, we need:
                // - Test patterns
                // - Untested code
                context.test_files = self.find_test_files().await;
                context.relevant_files = self.find_untested_code().await;
            }
            TaskType::Architecture => {
                // For architecture, we need:
                // - Full architecture
                // - Dependencies
                // - Data flow
                context.architecture = self.get_architecture_info().await;
                context.dependencies = self.get_dependency_info().await;
            }
            TaskType::General => {
                // For general tasks, provide a balanced context
                context.recent_commits = self.get_recent_commits(5).await;
                context.architecture = self.get_architecture_info().await;
            }
        }

        Ok(context)
    }

    /// Get recent commits
    async fn get_recent_commits(&self, limit: usize) -> Vec<CommitInfo> {
        // Try to get commits from git
        let output = std::process::Command::new("git")
            .args([
                "log",
                "--oneline",
                "-n",
                &limit.to_string(),
                "--pretty=format:%H|%s|%an|%ad",
                "--date=short",
            ])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .filter_map(|line| {
                        let parts: Vec<&str> = line.splitn(4, '|').collect();
                        if parts.len() == 4 {
                            // Parse date string to DateTime
                            let date = chrono::NaiveDate::parse_from_str(parts[3], "%Y-%m-%d")
                                .ok()
                                .map(|d| {
                                    chrono::DateTime::from_naive_utc_and_offset(
                                        d.and_hms_opt(0, 0, 0).unwrap(),
                                        chrono::Utc,
                                    )
                                })
                                .unwrap_or_else(chrono::Utc::now);

                            Some(CommitInfo {
                                hash: parts[0].to_string(),
                                message: parts[1].to_string(),
                                author: parts[2].to_string(),
                                timestamp: date,
                                files_changed: Vec::new(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    /// Get error logs
    async fn get_error_logs(&self) -> Vec<String> {
        // Search for common log files
        let log_paths = [
            "logs/error.log",
            "log/error.log",
            "logs/app.log",
            "tmp/error.log",
        ];

        let mut logs = Vec::new();
        for path in &log_paths {
            if let Ok(content) = std::fs::read_to_string(path) {
                // Get last 50 lines
                let lines: Vec<&str> = content.lines().collect();
                let start = lines.len().saturating_sub(50);
                logs.extend(lines[start..].iter().map(|s| s.to_string()));
            }
        }

        logs
    }

    /// Find test files
    async fn find_test_files(&self) -> Vec<PathBuf> {
        let mut test_files = Vec::new();

        // Common test file patterns
        let test_patterns = [
            "test",
            "tests",
            "spec",
            "specs",
            "__tests__",
            "_test",
            "_spec",
        ];

        // Walk the directory
        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                if test_patterns.iter().any(|p| name.contains(p)) {
                    if path.is_dir() {
                        // Recursively find test files
                        self.collect_test_files(&path, &mut test_files);
                    } else if path.is_file() {
                        test_files.push(path);
                    }
                }
            }
        }

        test_files
    }

    /// Recursively collect test files
    fn collect_test_files(&self, dir: &std::path::Path, files: &mut Vec<PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    self.collect_test_files(&path, files);
                } else if path.is_file() {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    if name.contains("test") || name.contains("spec") {
                        files.push(path);
                    }
                }
            }
        }
    }

    /// Find similar fixes
    async fn find_similar_fixes(&self) -> Vec<FixInfo> {
        // Search git history for similar fixes
        let output = std::process::Command::new("git")
            .args([
                "log",
                "--all",
                "--grep=fix",
                "--oneline",
                "-n",
                "20",
                "--pretty=format:%H|%s",
            ])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .filter_map(|line| {
                        let parts: Vec<&str> = line.splitn(2, '|').collect();
                        if parts.len() == 2 {
                            Some(FixInfo {
                                commit_hash: parts[0].to_string(),
                                description: parts[1].to_string(),
                                files_changed: Vec::new(),
                                diff: String::new(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    /// Get architecture info — scans repo root (cwd), returns layers actually found
    async fn get_architecture_info(&self) -> Option<ArchitectureInfo> {
        // Analyze directory structure to detect architecture
        let mut has_src = false;
        let mut has_lib = false;
        let mut has_tests = false;
        let mut has_bins = false;

        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        if let Ok(entries) = std::fs::read_dir(&root) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if name == "src" {
                    has_src = true;
                }
                if name == "lib" {
                    has_lib = true;
                }
                if name == "test" || name == "tests" {
                    has_tests = true;
                }
                if name == "crates" || name == "bin" {
                    has_bins = true;
                }
            }
        }

        let pattern = if has_src && has_lib {
            "Library Architecture"
        } else if has_src && has_tests {
            "Application Architecture"
        } else if has_src || has_bins {
            "Simple Architecture"
        } else {
            "Unknown Architecture"
        };

        Some(ArchitectureInfo {
            pattern: pattern.to_string(),
            layers: vec![],
            components: vec![],
            data_flow: vec![],
        })
    }

    /// Get dependency info
    async fn get_dependency_info(&self) -> Vec<DependencyInfo> {
        let mut deps = Vec::new();

        // Check for package.json
        if let Ok(content) = std::fs::read_to_string("package.json") {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(dependencies) = json.get("dependencies").and_then(|d| d.as_object()) {
                    for (name, version) in dependencies {
                        deps.push(DependencyInfo {
                            name: name.clone(),
                            version: version.as_str().unwrap_or("").to_string(),
                            kind: DependencyKind::Runtime,
                            is_direct: true,
                        });
                    }
                }
                if let Some(dev_dependencies) =
                    json.get("devDependencies").and_then(|d| d.as_object())
                {
                    for (name, version) in dev_dependencies {
                        deps.push(DependencyInfo {
                            name: name.clone(),
                            version: version.as_str().unwrap_or("").to_string(),
                            kind: DependencyKind::Development,
                            is_direct: true,
                        });
                    }
                }
            }
        }

        // Check for Cargo.toml (simple parsing)
        if let Ok(content) = std::fs::read_to_string("Cargo.toml") {
            let mut in_deps = false;
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with("[dependencies]") {
                    in_deps = true;
                    continue;
                }
                if line.starts_with("[") {
                    in_deps = false;
                    continue;
                }
                if in_deps && !line.is_empty() && !line.starts_with('#') {
                    let name = line.split('=').next().unwrap_or("").trim().to_string();
                    if !name.is_empty() {
                        deps.push(DependencyInfo {
                            name,
                            version: "unknown".to_string(),
                            kind: DependencyKind::Runtime,
                            is_direct: true,
                        });
                    }
                }
            }
        }

        // Check for requirements.txt
        if let Ok(content) = std::fs::read_to_string("requirements.txt") {
            for line in content.lines() {
                let line = line.trim();
                if !line.is_empty() && !line.starts_with('#') {
                    let parts: Vec<&str> = line.split("==").collect();
                    if parts.len() == 2 {
                        deps.push(DependencyInfo {
                            name: parts[0].to_string(),
                            version: parts[1].to_string(),
                            kind: DependencyKind::Runtime,
                            is_direct: true,
                        });
                    }
                }
            }
        }

        deps
    }

    /// Find similar features
    async fn find_similar_features(&self) -> Vec<PathBuf> {
        // Find files with similar names or content
        let mut features = Vec::new();

        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    if name.contains("feature")
                        || name.contains("service")
                        || name.contains("module")
                    {
                        features.push(path);
                    }
                }
            }
        }

        features
    }

    /// Find hot files
    async fn find_hot_files(&self) -> Vec<PathBuf> {
        // Find recently modified files
        let mut files = Vec::new();

        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            let age = std::time::SystemTime::now()
                                .duration_since(modified)
                                .unwrap_or_default()
                                .as_secs();
                            // Files modified in the last 7 days
                            if age < 7 * 24 * 60 * 60 {
                                files.push(path);
                            }
                        }
                    }
                }
            }
        }

        files
    }

    /// Find security-related files
    async fn find_security_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();

        let security_patterns = ["auth", "security", "login", "password", "token", "crypto"];

        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    if security_patterns.iter().any(|p| name.contains(p)) {
                        files.push(path);
                    }
                }
            }
        }

        files
    }

    /// Find public APIs
    async fn find_public_apis(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();

        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    if name.contains("api") || name.contains("public") || name.contains("export") {
                        files.push(path);
                    }
                }
            }
        }

        files
    }

    /// Find untested code
    async fn find_untested_code(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();

        // Find source files without corresponding test files
        if let Ok(entries) = std::fs::read_dir("src") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    if !name.contains("test") {
                        files.push(path);
                    }
                }
            }
        }

        files
    }

    /// Learn from a task and its outcome (bounded to avoid unbounded growth)
    pub fn learn(&mut self, task: &str, task_type: TaskType, success: bool) {
        if !success {
            return;
        }
        // Only memorize short canonical phrases, not full sentences
        let key = task.to_lowercase().trim().to_string();
        if key.len() > 64 || key.is_empty() {
            return;
        }
        if self.task_patterns.len() >= 500 {
            // Evict an arbitrary entry to stay bounded
            if let Some(k) = self.task_patterns.keys().next().cloned() {
                self.task_patterns.remove(&k);
            }
        }
        self.task_patterns.insert(key, task_type);
    }

    /// Pre-fetch context for likely next tasks
    pub async fn prefetch(&self, current_task: &str) {
        let current_type = self.classify_task(current_task);

        // Predict likely next tasks based on current task
        let likely_next = self.predict_next_tasks(current_type);

        for next_type in likely_next {
            // Pre-fetch context in background
            let _ = self.build_context(next_type).await;
        }
    }

    /// Predict likely next tasks
    fn predict_next_tasks(&self, current: TaskType) -> Vec<TaskType> {
        match current {
            TaskType::BugFix => vec![TaskType::Testing, TaskType::CodeReview],
            TaskType::NewFeature => vec![TaskType::Testing, TaskType::Documentation],
            TaskType::Refactor => vec![TaskType::Testing, TaskType::CodeReview],
            TaskType::CodeReview => vec![TaskType::BugFix, TaskType::Refactor],
            _ => vec![TaskType::General],
        }
    }
}
