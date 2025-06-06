use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::emitter::{Emitter, Literal, Specie, Tag};
use super::trail::Trail;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Oop {
    pub emitter: Emitter,
    pub trails: Vec<Trail>,
}

impl Oop {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::oop);

        let mut inner = pair.into_inner();

        let emitter_pair = inner.next().expect("Oop must have an emitter");
        let emitter = match emitter_pair.as_rule() {
            Rule::emitter => {
                let mut emitter_inner = emitter_pair.into_inner();
                let inner_pair = emitter_inner.next().expect("Emitter must have inner content");
                match inner_pair.as_rule() {
                    Rule::specie => Emitter::Specie(Specie { raw: inner_pair.as_str().to_string() }),
                    Rule::tag => Emitter::Tag(Tag { raw: inner_pair.as_str().to_string() }),
                    Rule::literal => Emitter::Literal(Literal::from_pair(inner_pair)),
                    _ => panic!("Unexpected inner emitter type: {:?}", inner_pair.as_rule()),
                }
            },
            Rule::specie => Emitter::Specie(Specie { raw: emitter_pair.as_str().to_string() }),
            Rule::tag => Emitter::Tag(Tag { raw: emitter_pair.as_str().to_string() }),
            Rule::literal => Emitter::Literal(Literal::from_pair(emitter_pair)),
            _ => panic!("Unexpected emitter type: {:?}", emitter_pair.as_rule()),
        };

        let mut trails = Vec::new();
        for trail_pair in inner {
            trails.push(Trail::from_pair(trail_pair));
        }

        Oop { emitter, trails }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::oop, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Oop::from_pair(pair))
    }
}
