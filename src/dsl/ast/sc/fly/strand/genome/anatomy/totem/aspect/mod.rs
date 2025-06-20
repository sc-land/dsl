use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::sc::fly::strand::genome::anatomy::totem::aspect::zoo::Zoo;
use crate::dsl::parser::parser::Rule;

pub mod zoo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Aspect {
    pub emblem: String,
    pub zoo: Option<Zoo>,
}

impl Aspect {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::aspect);

        let mut inner = pair.into_inner();
        let emblem = inner.next().unwrap().as_str().to_string();

        let zoo = inner.next().map(|zoo_pair| Zoo::from_pair(zoo_pair));

        Aspect { emblem, zoo }
    }
}
