use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Primor {
    // { !reserved ~ ASCII_ALPHA_LOWER ~ (ASCII_ALPHANUMERIC | "_")* }
    pub raw: String,
}

impl Primor {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn from_pair(pair: pest::iterators::Pair<crate::parser::parser::Rule>) -> Self {
        assert_eq!(pair.as_rule(), crate::parser::parser::Rule::primor);
        let raw = pair.as_str().to_string();
        Self { raw }
    }
}
