use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::{Rule, SCP};
use crate::dsl::ast::emitter::{Tag, Specie};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gene {
    pub tag: Tag,
    pub specie: Specie,
}

impl Gene {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::gene);

        let mut inner = pair.into_inner();

        let tag_str = inner.next()
            .expect("Gene deve ter uma tag")
            .as_str()
            .to_string();
        let tag = Tag::new(tag_str);

        let specie_str = inner.next()
            .expect("Gene deve ter uma espécie")
            .as_str()
            .to_string();
        let specie = Specie::new(specie_str);

        Gene { tag, specie }
    }

    pub fn from_string(input: String) -> Self {
        let pair = SCP::parse(Rule::gene, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");
        Gene::from_pair(pair)
    }
}
