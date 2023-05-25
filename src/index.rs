use crate::analyzer::Analyzer;
use std::collections::{HashMap, HashSet};

pub struct InvertedIndex {
    idx: HashMap<String, HashSet<u64>>,
    analyzer: Analyzer,
}

pub struct Document {
    id: u64,
    text: String,
}

impl InvertedIndex {
    // Creates empty inverted index
    pub fn new() -> Self {
        InvertedIndex {
            idx: HashMap::new(),
            analyzer: Analyzer::new(),
        }
    }

    // Index document
    pub fn add(&mut self, docs: Vec<Document>) {
        for doc in docs.iter() {
            for token in self.analyzer.analyze(doc.text.as_str()) {
                match self.idx.get_mut(&*token) {
                    None => {
                        let v = HashSet::from([doc.id]);
                        self.idx.insert(token, v);
                    }
                    Some(v) => {
                        v.insert(doc.id);
                    }
                }
            }
        }
    }

    pub fn search(&self, text: &str) -> HashSet<u64> {
        let mut result: HashSet<u64> = HashSet::new();
        for token in self.analyzer.analyze(text) {
            match self.idx.get(&*token) {
                None => {}
                Some(ids) => {
                    if result.is_empty() {
                        result = ids.clone()
                    }
                    result = result.intersection(ids).cloned().collect()
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod index_tests {
    use crate::index::{Document, InvertedIndex};

    #[test]
    fn add_test() {
        let mut idx = InvertedIndex::new();
        let doc = vec![
            Document {
                id: 1,
                text: "The quick brown fox jumped over the lazy dog".to_string(),
            },
            Document {
                id: 2,
                text: "Quick brown foxes leap over lazy dogs in summer".to_string(),
            },
        ];
        idx.add(doc);
        let result = idx.search("dogs in summer");
        assert_eq!(result.get(&2), Some(&2), "smoke test for index failed")
    }
}
