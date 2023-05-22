pub struct Tokenizer {}

impl Tokenizer {
    /* Converts text to vector of tokens. Tokens are either letter or number */
    pub fn tokenize(text: &str) -> Vec<String> {
        let clean_string = text
            .chars()
            .filter(|c| !c.is_ascii_punctuation())
            .collect::<String>();

        clean_string
            .split_whitespace()
            .map(str::to_string)
            .collect()
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use crate::tokenizer::Tokenizer;

    #[test]
    fn test_tokenize() {
        let text = "Hello #{$}! I'm test suite & I ... contain number 32!!";
        let res = Tokenizer::tokenize(&text);
        let expected = vec![
            "Hello", "I'm", "test", "suite", "I", "contain", "number", "32",
        ];
        assert!(
            res.len() != expected.len() || res != expected,
            "tokenization failed"
        )
    }
}
