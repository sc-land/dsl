use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Specie {
    // { ASCII_ALPHA_UPPER ~ (ASCII_ALPHANUMERIC | "_")* }
    pub raw: String,
}

impl Specie {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::specie);
        let raw = pair.as_str().to_string();
        Self { raw }
    }
}
