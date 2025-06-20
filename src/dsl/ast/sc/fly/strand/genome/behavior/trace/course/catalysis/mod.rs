use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::sc::fly::strand::genome::anatomy::bug::gene::primor::Primor;
use crate::dsl::ast::sc::fly::strand::genome::behavior::trace::course::carrier::Carrier;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Catalysis {
    pub tag: Primor,
    pub carrier: Option<Carrier>,
}

impl Catalysis {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::catalysis);

        let mut inner = pair.into_inner();

        // Parse tag (skip the "." token)
        let tag_pair = inner.next().expect("Catalysis deve ter uma tag");
        let tag = Primor::from_pair(tag_pair);

        // Parse optional carrier
        let carrier = inner.next().map(|carrier_pair| Carrier::from_pair(carrier_pair));

        Catalysis { tag, carrier }
    }
}

