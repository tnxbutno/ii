use rust_stemmers::{Algorithm, Stemmer};
use std::borrow::Cow;
use std::collections::HashSet;

pub struct Filters {
    stop_words_list: HashSet<String>,
    stemmer: Stemmer,
}

impl Filters {
    pub fn new() -> Self {
        let stop_words_list: HashSet<String> = vec![
            "a", "an", "and", "are", "as", "at", "be", "but", "by", "for", "if", "in", "into",
            "is", "it", "no", "not", "of", "on", "or", "such", "that", "the", "their", "then",
            "there", "these", "they", "this", "to", "was", "will", "with",
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
        Filters {
            stop_words_list,
            stemmer: Stemmer::create(Algorithm::English),
        }
    }

    pub fn lower_case(&self, tokens: Vec<String>) -> Vec<String> {
        tokens
            .iter()
            .map(|s| s.to_lowercase())
            .collect::<Vec<String>>()
    }

    pub fn stop_words(&self, tokens: Vec<String>) -> Vec<String> {
        let set_of_tokens: HashSet<String> = tokens.into_iter().collect();
        set_of_tokens
            .difference(&self.stop_words_list)
            .cloned()
            .collect()
    }

    pub fn stemming(&self, tokens: Vec<String>) -> Vec<String> {
        tokens
            .iter()
            .map(|t| match self.stemmer.stem(t) {
                Cow::Owned(stemmed_str) => stemmed_str,
                Cow::Borrowed(stemmed_str) => stemmed_str.to_string(),
            })
            .collect()
    }
}

#[cfg(test)]
mod filters_tests {
    use crate::filters::Filters;

    #[test]
    fn test_lowercase() {
        let filter = Filters::new();
        let tokens = vec!["HELLO", "THIS", "IS", "PATRICK"]
            .into_iter()
            .map(str::to_string)
            .collect();
        let res = filter.lower_case(tokens);
        let expected = vec!["hello", "this", "is", "patrick"];
        assert_eq!(res, expected, "lowering case failed");
    }

    #[test]
    fn test_stop_words() {
        let filter = Filters::new();
        let tokens = vec!["as", "stay", "a", "will"]
            .into_iter()
            .map(str::to_string)
            .collect();
        let res = filter.stop_words(tokens);
        let expected = vec!["stay"];
        assert_eq!(res, expected, "stop words failed")
    }

    #[test]
    fn test_stemming() {
        let filter = Filters::new();
        let tokens = vec!["worked", "working", "works", "works"]
            .into_iter()
            .map(str::to_string)
            .collect();
        let res = filter.stemming(tokens);
        let expected = vec!["work", "work", "work", "work"];
        assert_eq!(res, expected, "stemming failed")
    }
}
