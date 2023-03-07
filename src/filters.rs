pub struct Filter {}

impl Filter {
    pub fn lower_case(tokens: Vec<&str>) -> impl Iterator<Item = String> {
        tokens.iter().map(str::to_ascii_lowercase)
    }

    pub fn stop_words(tokens: Vec<&str>) -> impl Iterator<Item = String> {
        todo!()
    }

    pub fn stemming(tokens: Vec<&str>) -> impl Iterator<Item = String> {
        todo!()
    }
}