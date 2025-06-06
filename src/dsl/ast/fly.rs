use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::strand::Strand;
use crate::dsl::parser::parser::{Rule, SCP};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fly {
    pub strand: Strand,
}

impl Fly {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::fly);

        let strand_pair = pair.into_inner().next().expect("Fly should have a strand");
        assert_eq!(strand_pair.as_rule(), Rule::strand);
        let strand = Strand::from_pair(strand_pair);

        Fly { strand }
    }

    pub fn from_string(input: String) -> Self {
        let pair = SCP::parse(Rule::fly, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");
        Fly::from_pair(pair)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::Fly;

    #[test]
    fn test_fly_with_two_bugs() {
        // Carrega o fragmento com dois bugs
        let path = "tests/fixtures/fragments/bug/two_bugs.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read input.sc file");

        // Verifica que o arquivo não está vazio
        assert!(!input.is_empty(), "Fixture file should not be empty");

        // Testa o parse
        let fly = Fly::from_string(input.clone());

        // Verifica se o strand contém dois genomes
        assert_eq!(fly.strand.genome.len(), 2, "Strand should contain two genomes");

        // Verifica se o conteúdo contém os nomes dos bugs
        assert!(input.contains("Dog"), "Input should contain bug Dog");
        assert!(input.contains("Cog"), "Input should contain bug Cog");
    }
}

