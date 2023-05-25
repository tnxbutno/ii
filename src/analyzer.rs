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
