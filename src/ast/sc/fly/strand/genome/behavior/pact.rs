use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::parser::parser::Rule;
use super::trace::Trace;
use crate::ast::sc::fly::strand::genome::anatomy::bug::gene::specie::Specie;
use crate::ast::sc::fly::strand::genome::anatomy::bug::gene::primor::Primor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bind {
    pub tag: Primor,
    pub oop: Trace,
}

impl Bind {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::bind);

        let mut inner = pair.into_inner();

        // Parse tag
        let tag_pair = inner.next().expect("Bind deve ter uma tag");
        let tag = Primor::from_pair(tag_pair);

        // Parse oop
        let oop_pair = inner.next().expect("Bind deve ter um oop");
        let oop = Trace::from_pair(oop_pair);

        Bind { tag, oop }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::bind, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Bind::from_pair(pair))
    }
}

/// EthicsBind represents function parameter bindings that use species instead of oop
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pact {
    pub tag: Primor,
    pub specie: Specie,
}

impl Pact {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::pact);

        let mut inner = pair.into_inner();

        // Parse tag
        let tag_pair = inner.next().expect("EthicsBind deve ter uma tag");
        let tag = Primor::from_pair(tag_pair);

        // Parse specie
        let specie_pair = inner.next().expect("EthicsBind deve ter uma specie");
        let specie = Specie::new(specie_pair.as_str().to_string());

        Pact { tag, specie }
    }
}
