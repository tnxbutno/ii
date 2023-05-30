use criterion::{criterion_group, criterion_main, Criterion};
use inverted_index::index::{Document, InvertedIndex};

const TEXT: &str = include_str!("war_and_peace.txt");

pub fn index_benchmark(c: &mut Criterion) {
    let docs = TEXT
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(index, line)| Document {
            id: index as u64,
            text: line.to_string(),
        })
        .collect::<Vec<Document>>();

    c.bench_function("index-large-text", |b| {
        b.iter(|| {
            let mut index = InvertedIndex::default();
            index.add(&docs)
        })
    });
}

criterion_group!(benches, index_benchmark);
criterion_main!(benches);
