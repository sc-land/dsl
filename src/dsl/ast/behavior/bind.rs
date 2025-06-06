use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::Rule;
use super::oop::Oop;
use crate::dsl::ast::emitter::{tag::Tag, specie::Specie};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bind {
    pub tag: Tag,
    pub oop: Oop,
}

impl Bind {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::bind);

        let mut inner = pair.into_inner();

        // Parse tag
        let tag_pair = inner.next().expect("Bind deve ter uma tag");
        let tag = Tag::from_pair(tag_pair);

        // Parse oop
        let oop_pair = inner.next().expect("Bind deve ter um oop");
        let oop = Oop::from_pair(oop_pair);

        Bind { tag, oop }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::bind, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Bind::from_pair(pair))
    }
}

/// EthicsBind represents function parameter bindings that use species instead of oop
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthicsBind {
    pub tag: Tag,
    pub specie: Specie,
}

impl EthicsBind {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::ethics_bind);

        let mut inner = pair.into_inner();

        // Parse tag
        let tag_pair = inner.next().expect("EthicsBind deve ter uma tag");
        let tag = Tag::from_pair(tag_pair);

        // Parse specie
        let specie_pair = inner.next().expect("EthicsBind deve ter uma specie");
        let specie = Specie::new(specie_pair.as_str().to_string());

        EthicsBind { tag, specie }
    }
}
