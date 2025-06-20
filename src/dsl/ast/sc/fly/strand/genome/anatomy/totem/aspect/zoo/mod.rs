use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Zoo {
    Plain(Vec<String>),
    Signed(Vec<Biome>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Biome {
    pub primor: String,
    pub specie: String,
}

impl Zoo {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::zoo);

        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::plain => {
                let species: Vec<String> = inner.into_inner()
                    .map(|p| p.as_str().to_string())
                    .collect();
                Zoo::Plain(species)
            }
            Rule::signed => {
                let biomes: Vec<Biome> = inner.into_inner()
                    .map(|biome_pair| Biome::from_pair(biome_pair))
                    .collect();
                Zoo::Signed(biomes)
            }
            _ => panic!("Regra inesperada dentro de zoo: {:?}", inner.as_rule()),
        }
    }
}

impl Biome {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::biome);

        let mut inner = pair.into_inner();
        let primor = inner.next().unwrap().as_str().to_string();
        let specie = inner.next().unwrap().as_str().to_string();

        Biome { primor, specie }
    }
}
