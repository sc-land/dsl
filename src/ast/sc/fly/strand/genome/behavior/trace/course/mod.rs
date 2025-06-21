pub mod carrier;
mod catalysis;

use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::behavior::trace::course::carrier::Carrier;
use crate::ast::sc::fly::strand::genome::behavior::trace::course::catalysis::Catalysis;
use crate::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Course {
    Catalysis(Catalysis),
    Carrier(Carrier),
}

impl Course {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::course);

        let inner_pair = pair.into_inner().next().expect("Trail must have an inner rule");
        match inner_pair.as_rule() {
            Rule::catalysis => Course::Catalysis(Catalysis::from_pair(inner_pair)),
            Rule::carrier => Course::Carrier(Carrier::from_pair(inner_pair)),
            _ => panic!("Unexpected rule inside trail: {:?}", inner_pair.as_rule()),
        }
    }
}
