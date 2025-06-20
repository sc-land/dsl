pub mod course;
pub mod forager;

use pest::Parser;
use crate::dsl::parser::parser::SCP;
use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::sc::fly::strand::genome::anatomy::bug::gene::specie::Specie;
use crate::dsl::ast::sc::fly::strand::genome::anatomy::bug::gene::primor::Primor;
use crate::dsl::ast::sc::fly::strand::genome::behavior::trace::course::Course;
use crate::dsl::ast::sc::fly::strand::genome::behavior::trace::forager::Forager;
use crate::dsl::ast::sc::fly::strand::genome::behavior::trace::forager::literal::Literal;
use crate::dsl::ast::sc::fly::strand::genome::behavior::trace::forager::self_ref::SelfRef;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trace {
    pub forager: Forager,
    pub courses: Vec<Course>,
}

impl Trace {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::trace);

        let mut inner = pair.into_inner();

        let emitter_pair = inner.next().expect("Oop must have an emitter");
        let emitter = match emitter_pair.as_rule() {
            Rule::emitter => {
                let mut emitter_inner = emitter_pair.into_inner();
                let inner_pair = emitter_inner.next().expect("Emitter must have inner content");
                match inner_pair.as_rule() {
                    Rule::specie => Forager::Specie(Specie { raw: inner_pair.as_str().to_string() }),
                    Rule::primor => Forager::Primor(Primor { raw: inner_pair.as_str().to_string() }),
                    Rule::literal => Forager::Literal(Literal::from_pair(inner_pair)),
                    Rule::self_ref => Forager::SelfRef(SelfRef::from_pair(inner_pair)),
                    _ => panic!("Unexpected inner emitter type: {:?}", inner_pair.as_rule()),
                }
            },
            Rule::specie => Forager::Specie(Specie { raw: emitter_pair.as_str().to_string() }),
            Rule::primor => Forager::Primor(Primor { raw: emitter_pair.as_str().to_string() }),
            Rule::literal => Forager::Literal(Literal::from_pair(emitter_pair)),
            Rule::self_ref => Forager::SelfRef(SelfRef::from_pair(emitter_pair)),
            _ => panic!("Unexpected emitter type: {:?}", emitter_pair.as_rule()),
        };

        let mut trails = Vec::new();
        for trail_pair in inner {
            trails.push(Course::from_pair(trail_pair));
        }

        Trace { forager: emitter, courses: trails }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut pairs = SCP::parse(Rule::trace, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Trace::from_pair(pair))
    }
}
