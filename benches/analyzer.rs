use criterion::{criterion_group, criterion_main, Criterion};
use inverted_index::analyzer::Analyzer;

const TEXT: &str = include_str!("war_and_peace.txt");

pub fn analyzer_benchmark(c: &mut Criterion) {
    let analyzer = Analyzer::new();
    c.bench_function("analyze-large-text", |b| b.iter(|| analyzer.analyze(TEXT)));
}

criterion_group!(benches, analyzer_benchmark);
criterion_main!(benches);
