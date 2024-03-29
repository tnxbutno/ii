/// `Filters` include stop words, lowercase, and stemming filters.
use rust_stemmers::{Algorithm, Stemmer};
use std::borrow::Cow;
use std::collections::HashSet;

pub struct Filters {
    stop_words_list: HashSet<String>,
    stemmer: Stemmer,
}

/// Available languages for stemming.
pub enum Language {
    Arabic,
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Greek,
    Hungarian,
    Italian,
    Norwegian,
    Portuguese,
    Romanian,
    Russian,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
}

impl Language {
    /// Get algorithm matching variant of language
    fn get(&self) -> Algorithm {
        use self::Language::*;
        match self {
            Arabic => Algorithm::Arabic,
            Danish => Algorithm::Danish,
            Dutch => Algorithm::Dutch,
            English => Algorithm::English,
            Finnish => Algorithm::Finnish,
            French => Algorithm::French,
            German => Algorithm::German,
            Greek => Algorithm::Greek,
            Hungarian => Algorithm::Hungarian,
            Italian => Algorithm::Italian,
            Norwegian => Algorithm::Norwegian,
            Portuguese => Algorithm::Portuguese,
            Romanian => Algorithm::Romanian,
            Russian => Algorithm::Russian,
            Spanish => Algorithm::Spanish,
            Swedish => Algorithm::Swedish,
            Tamil => Algorithm::Tamil,
            Turkish => Algorithm::Turkish,
        }
    }

    /// Get stopwords for a given language.
    fn get_stopwords(&self) -> HashSet<String> {
        use self::Language::*;
        use crate::stopwords;
        let stop_words = match self {
            Arabic => stopwords::ARABIC,
            Danish => stopwords::DANISH,
            Dutch => stopwords::DUTCH,
            English => stopwords::ENGLISH,
            Finnish => stopwords::FINNISH,
            French => stopwords::FRENCH,
            German => stopwords::GERMAN,
            Greek => stopwords::GREEK,
            Hungarian => stopwords::HUNGARIAN,
            Italian => stopwords::ITALIAN,
            Norwegian => stopwords::NORWEGIAN,
            Portuguese => stopwords::PORTUGUESE,
            Romanian => stopwords::ROMANIAN,
            Russian => stopwords::RUSSIAN,
            Spanish => stopwords::SPANISH,
            Swedish => stopwords::SWEDISH,
            Tamil => stopwords::TAMIL,
            Turkish => stopwords::TURKISH,
        };
        stop_words
            .iter()
            .map(|&word| word.to_owned())
            .collect::<HashSet<String>>()
    }
}

/// `Default` assume that a text will be in English.
impl Default for Filters {
    fn default() -> Self {
        Filters::new(Language::English)
    }
}

impl Filters {
    /// Creates a `Filter` instance with custom language.
    pub fn new(language: Language) -> Self {
        Filters {
            stop_words_list: language.get_stopwords(),
            stemmer: Stemmer::create(language.get()),
        }
    }

    /// Makes all tokens lowercase.
    pub fn lowercase<I>(&self, tokens: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = String>,
    {
        tokens.map(|s| s.to_lowercase())
    }

    /// Removes stop words from tokens.
    pub fn stop_words<I>(&self, tokens: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = String>,
    {
        let set_of_tokens: HashSet<String> = tokens.into_iter().collect();
        set_of_tokens
            .difference(&self.stop_words_list)
            .cloned()
            .collect::<Vec<String>>()
            .into_iter()
    }

    /// Applies stemming technique to all tokens.
    pub fn stemming<'a, I>(&'a self, tokens: I) -> impl Iterator<Item = String> + 'a
    where
        I: Iterator<Item = String> + 'a,
    {
        tokens.map(|t| match self.stemmer.stem(&t) {
            Cow::Owned(stemmed_str) => stemmed_str,
            Cow::Borrowed(stemmed_str) => stemmed_str.to_string(),
        })
    }
}

#[cfg(test)]
mod filters_tests {
    use crate::filters::Filters;
    use crate::filters::Language::Russian;

    #[test]
    fn test_lowercase() {
        let filter = Filters::default();
        let tokens = ["HELLO", "THIS", "IS", "PATRICK"]
            .into_iter()
            .map(str::to_string);

        let res: Vec<String> = filter.lowercase(tokens).collect();
        let expected = ["hello", "this", "is", "patrick"];
        assert_eq!(res, expected, "lowering case failed");
    }

    #[test]
    fn test_stop_words_default() {
        let filter = Filters::default();
        let tokens = ["as", "stay", "a", "will"].into_iter().map(str::to_string);

        let res: Vec<String> = filter.stop_words(tokens).collect();
        let expected = ["stay"];
        assert_eq!(res, expected, "stop words failed");
    }

    #[test]
    fn test_stop_words_custom_lang() {
        let filter = Filters::new(Russian);
        let tokens = ["я", "бы", "тут", "остался"]
            .into_iter()
            .map(str::to_string);

        let res: Vec<String> = filter.stop_words(tokens).collect();
        let expected = ["остался"];
        assert_eq!(res, expected, "stop words for custom lang failed");
    }

    #[test]
    fn test_stemming_default() {
        let filter = Filters::default();
        let tokens = ["worked", "working", "works", "works"]
            .into_iter()
            .map(str::to_string);

        let res: Vec<String> = filter.stemming(tokens).collect();
        let expected = ["work", "work", "work", "work"];
        assert_eq!(res, expected, "stemming failed");
    }

    #[test]
    fn test_stemming_custom_lang() {
        let filter = Filters::new(Russian);
        let tokens = ["работал", "работаю", "работает", "работает"]
            .into_iter()
            .map(str::to_string);

        let res: Vec<String> = filter.stemming(tokens).collect();
        let expected = ["работа", "работа", "работа", "работа"];
        assert_eq!(res, expected, "stemming custom lang failed");
    }
}
