#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    // { !reserved ~ ASCII_ALPHA_LOWER ~ (ASCII_ALPHANUMERIC | "_")* }
    pub raw: String,
}

impl Tag {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn from_pair(pair: pest::iterators::Pair<crate::dsl::parser::parser::Rule>) -> Self {
        assert_eq!(pair.as_rule(), crate::dsl::parser::parser::Rule::tag);
        let raw = pair.as_str().to_string();
        Self { raw }
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }
}
