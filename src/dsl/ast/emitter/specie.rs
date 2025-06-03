#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Specie {
    // { ASCII_ALPHA_UPPER ~ (ASCII_ALPHANUMERIC | "_")* }
    pub raw: String,
}

impl Specie {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }

    pub fn from_pair(pair: pest::iterators::Pair<crate::dsl::parser::parser::Rule>) -> Self {
        assert_eq!(pair.as_rule(), crate::dsl::parser::parser::Rule::specie);
        let raw = pair.as_str().to_string();
        Self { raw }
    }
}
