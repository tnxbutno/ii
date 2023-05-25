use crate::filters::Filters;
use crate::tokenizer::Tokenizer;

pub struct Analyzer {
    tokenizer: Tokenizer,
    filters: Filters,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {
            tokenizer: Tokenizer::new(),
            filters: Filters::new(),
        }
    }

    pub fn analyze(&self, text: &str) -> Vec<String> {
        let tokens = self.tokenizer.tokenize(text);
        let low = self.filters.lower_case(tokens);
        let stopped = self.filters.stop_words(low);
        self.filters.stemming(stopped)
    }
}

#[cfg(test)]
mod analyzer_tests {
    use crate::analyzer::Analyzer;

    #[test]
    fn test_analyze() {
        let analyzer = Analyzer::new();
        let text = "The rain, rain poured and poured, creating a rhythmic symphony of droplets on the windowpane!";
        let res = analyzer.analyze(text);
        assert_eq!(res.len(), 7, "text analyze failed")
    }
}
