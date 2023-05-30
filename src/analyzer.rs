/// The analyzer combines the tokenizer and all filters and applies them to a text.
use crate::filters::{Filters, Language};
use crate::tokenizer::Tokenizer;

/// `Analyzer` tokenize and applies filters to a text.
pub struct Analyzer {
    tokenizer: Tokenizer,
    filters: Filters,
}

/// `Default` assume that a text will be in English.
impl Default for Analyzer {
    fn default() -> Self {
        Self::new(Language::English)
    }
}

impl Analyzer {
    /// Creates an analyzer with custom language.
    pub fn new(language: Language) -> Self {
        Analyzer {
            tokenizer: Tokenizer::new(),
            filters: Filters::new(language),
        }
    }

    /// Applies tokenizer, lowercase, stop words, and stemming filters.
    pub fn analyze(&self, text: &str) -> Vec<String> {
        let tokens = self.tokenizer.tokenize(text);
        let low = self.filters.lowercase(tokens);
        let stopped = self.filters.stop_words(low);
        self.filters.stemming(stopped).collect()
    }
}

#[cfg(test)]
mod analyzer_tests {
    use crate::analyzer::Analyzer;
    use crate::filters::Language;

    #[test]
    fn test_analyze() {
        let analyzer = Analyzer::default();
        let text = "The rain, rain poured and poured, creating a rhythmic symphony of droplets on the windowpane!";
        let res: Vec<String> = analyzer.analyze(text);
        assert_eq!(res.len(), 7, "text analyze failed");
    }

    #[test]
    fn test_analyze_custom_lang() {
        let analyzer = Analyzer::new(Language::Russian);
        let text = "Дождь, дождь лил и лил, создавая на стекле окона ритмичную симфонию капель!";
        let res: Vec<String> = analyzer.analyze(text);
        assert_eq!(res.len(), 8, "custom lang text analyze failed");
    }
}
