use crate::dsl::parser::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug)]
pub struct SC {
    pub raw: String,
}

impl SC {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::sc);

        let text = pair.as_str().to_string();
        SC { raw: text }
    }
}
