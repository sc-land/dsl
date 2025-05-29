use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone)]
pub struct DML {
    pub raw: String,
}

impl DML {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::dml);

        let raw = pair.as_str().to_string();

        DML { raw }
    }
}
