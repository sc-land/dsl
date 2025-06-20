use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::sc::fly::strand::genome::anatomy::totem::folklore::runes::Runes;
use crate::dsl::parser::parser::Rule;

pub mod runes;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Folklore {
    pub prophecy: String,
    pub runes: Option<Runes>,
}

impl Folklore {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::aspect);

        let mut inner = pair.into_inner();
        let emblem = inner.next().unwrap().as_str().to_string();

        let zoo = inner.next().map(|zoo_pair| Runes::from_pair(zoo_pair));

        Folklore { prophecy: emblem, runes: zoo }
    }
}
