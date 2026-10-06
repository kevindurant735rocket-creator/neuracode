//! Indexing benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use neuracode_core::{CodeBrain, NeuraCodeConfig};
use tempfile::TempDir;

fn bench_index_small_project(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a small project
    for i in 0..10 {
        let file_path = temp_dir.path().join(format!("file_{}.rs", i));
        std::fs::write(&file_path, format!(r#"
fn function_{}() {{
    println!("Function {}");
}}

struct Struct_{} {{
    field: i32,
}}
"#, i, i, i)).unwrap();
    }
    
    let config = NeuraCodeConfig::default();
    
    c.bench_function("index_10_files", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let code_brain = CodeBrain::new(&config).await.unwrap();
                black_box(code_brain.index(temp_dir.path()).await.unwrap());
            });
    });
}

fn bench_index_medium_project(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a medium project
    for i in 0..100 {
        let file_path = temp_dir.path().join(format!("file_{}.rs", i));
        std::fs::write(&file_path, format!(r#"
fn function_{}() {{
    println!("Function {}");
}}

struct Struct_{} {{
    field: i32,
}}

impl Struct_{} {{
    fn new() -> Self {{
        Self {{ field: 0 }}
    }}
}}
"#, i, i, i, i)).unwrap();
    }
    
    let config = NeuraCodeConfig::default();
    
    c.bench_function("index_100_files", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let code_brain = CodeBrain::new(&config).await.unwrap();
                black_box(code_brain.index(temp_dir.path()).await.unwrap());
            });
    });
}

criterion_group!(benches, bench_index_small_project, bench_index_medium_project);
criterion_main!(benches);
