use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::parser::parser::Rule;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity {
    pub primor: String,
    pub specie: String,
}

impl Entity {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::entity);

        let mut inner = pair.into_inner();
        let primor = inner.next().unwrap().as_str().to_string();
        let specie = inner.next().unwrap().as_str().to_string();

        Entity { primor, specie }
    }
}
