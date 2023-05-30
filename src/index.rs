/// This is the place where the Inverted Index is implemented.
use crate::analyzer::Analyzer;
use crate::filters::Language;
use std::collections::{HashMap, HashSet};

/// Currently, `InvertedIndex` implements with `HashMap<String, HashSet<u64>>`.
/// With HashSet as a value, we can efficiently perform union, difference, intersection,
/// and symmetric differences operations.
/// Additionally, HashSet enables us to have distinct values for a given key.
pub struct InvertedIndex {
    idx: HashMap<String, HashSet<u64>>,
    analyzer: Analyzer,
}

/// `Document` represents text that's needs to be indexed.
/// `id`: is a unique text id
/// `text`: is a text which will be divided into tokens and added to an index.
#[derive(Clone)]
pub struct Document {
    pub id: u64,
    pub text: String,
}

/// Creates empty index.
/// `Default` assume that a text will be in English.
impl Default for InvertedIndex {
    fn default() -> Self {
        Self::new(Language::English)
    }
}

impl InvertedIndex {
    /// Creates empty index with custom language.
    pub fn new(language: Language) -> Self {
        InvertedIndex {
            idx: HashMap::new(),
            analyzer: Analyzer::new(language),
        }
    }

    /// Index document
    pub fn add(&mut self, docs: &[Document]) {
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

    /// Search text in index
    pub fn search(&self, text: &str) -> HashSet<u64> {
        let mut result: HashSet<u64> = HashSet::new();
        for token in self.analyzer.analyze(text) {
            match self.idx.get(&*token) {
                None => {}
                Some(ids) => {
                    if result.is_empty() {
                        result = ids.clone();
                    }
                    result = result.intersection(ids).copied().collect();
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
        let mut idx = InvertedIndex::default();
        let doc = [
            Document {
                id: 1,
                text: "The quick brown fox jumped over the lazy dog".to_string(),
            },
            Document {
                id: 2,
                text: "Quick brown foxes leap over lazy dogs in summer".to_string(),
            },
        ];
        idx.add(&doc);
        let result = idx.search("dogs in summer");
        assert_eq!(result.get(&2), Some(&2), "smoke test for index failed");
    }
}
