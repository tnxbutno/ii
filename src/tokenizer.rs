pub struct Tokenizer {}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {}
    }

    /* Converts text to vector of tokens. Tokens are either letter or number */
    pub fn tokenize(&self, text: &str) -> impl Iterator<Item = String> {
        text.chars()
            .filter(|c| !c.is_ascii_punctuation())
            .collect::<String>()
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<String>>()
            .into_iter()
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use crate::tokenizer::Tokenizer;

    #[test]
    fn test_tokenize() {
        let text = "Hello #{$}! I'm test suite & I ... contain number 32!!";
        let tokenizer = Tokenizer::new();
        let res: Vec<String> = tokenizer.tokenize(&text).collect();
        let expected = vec![
            "Hello", "Im", "test", "suite", "I", "contain", "number", "32",
        ];
        assert_eq!(res, expected, "tokenization failed")
    }
}
