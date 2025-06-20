use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::sc::fly::strand::genome::anatomy::bug::gene::specie::Specie;
use crate::dsl::ast::sc::fly::strand::genome::anatomy::totem::aspect::Aspect;
use crate::dsl::parser::parser::{Rule, SCP};

pub mod aspect;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Totem {
    pub insignia: Insignia,
    pub aspects: Vec<Aspect>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Insignia {
    // { ASCII_ALPHA_UPPER ~ (ASCII_ALPHANUMERIC | "_")* }
    pub raw: String,
}

impl Insignia {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::insignia);
        let raw = pair.as_str().to_string();
        Self { raw }
    }
}

impl Totem {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::totem);

        let mut insignia = None;
        let mut aspects = Vec::new();

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::insignia => {
                    insignia = Some(Insignia::from_pair(inner));
                }
                Rule::aspect => aspects.push(Aspect::from_pair(inner)),
                _ => todo!()
            }
        }

        let insignia = insignia.expect("deve ter uma insignia");
        Totem { insignia, aspects }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut pairs = SCP::parse(Rule::totem, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Totem::from_pair(pair))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::dsl::ast::sc::fly::strand::genome::anatomy::Anatomy;
    use crate::dsl::ast::sc::fly::strand::genome::Genome;
    use crate::dsl::parser::tree::Tree;

    #[test]
    fn test(){
        let input = fs::read_to_string("tests/fixtures/fragments/totem/complete.sc")
            .expect("Failed to read input.sc file");

        let tree = Tree::parse(input).unwrap();
        assert_eq!(tree.sc.fly.strand.genome.len(), 8);
        for gene in tree.sc.fly.strand.genome {
            match gene {
                Genome::Anatomy(anatomy) => {
                    match anatomy {
                        Anatomy::Bug(_) => todo!(),
                        Anatomy::Totem(t) => {
                            println!("t: {:#?}", t);
                        },
                    }
                }
                Genome::Behavior(_) => {todo!()}
            }
        }
    }
}