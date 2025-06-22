pub mod entity;

use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::anatomy::totem::folklore::runes::entity::Entity;
use crate::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Runes {
    Whispers(Vec<String>),
    Invocations(Vec<Entity>),
}

impl Runes {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::runes);

        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::whispers => {
                let species: Vec<String> = inner.into_inner()
                    .map(|p| p.as_str().to_string())
                    .collect();
                Runes::Whispers(species)
            }
            Rule::invocations => {
                let biomes: Vec<Entity> = inner.into_inner()
                    .map(|biome_pair| Entity::from_pair(biome_pair))
                    .collect();
                Runes::Invocations(biomes)
            }
            _ => panic!("Regra inesperada dentro de zoo: {:?}", inner.as_rule()),
        }
    }
}
