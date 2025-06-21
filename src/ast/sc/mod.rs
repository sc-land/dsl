pub mod fly;

use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::Fly;
use crate::parser::parser::{Rule, SCP};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SC {
    pub fly: Fly,
}

impl SC {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::sc);

        let fly = Fly::from_pair(pair.into_inner().next().unwrap());

        SC { fly }
    }

    pub fn parse(input: String) -> Self {
        let pair = SCP::parse(Rule::sc, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");
        SC::from_pair(pair)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use pest::Parser;

    use crate::parser::parser::{Rule, SCP};
    use crate::ast::sc::SC;

    #[test]
    fn test_sc_parse_anatomy() {
        // Carrega o arquivo de fixture usando o helper
        let input = fs::read_to_string("tests/fixtures/fragments/program/anatomy.sc").unwrap();

        // Verifica que o arquivo não está vazio
        assert!(!input.is_empty(), "Fixture file should not be empty");

        // Testa o parse
        let sc = SC::parse(input.clone());

        // Verifica se o fly foi parseado corretamente
        assert!(!sc.fly.strand.genome.is_empty(), "Fly should have genomes");

        // Verifica se o fly contém o nome do bug
        assert!(input.contains("Dog"), "Input should contain the bug name");
    }

    #[test]
    fn test_sc_parse_behavior() {
        // Carrega o arquivo de fixture usando o helper
        let input = fs::read_to_string("tests/fixtures/fragments/program/behavior.sc").unwrap();

        // Verifica que o arquivo não está vazio
        assert!(!input.is_empty(), "Fixture file should not be empty");

        // Testa o parse
        let sc = SC::parse(input.clone());

        // Verifica se o fly foi parseado corretamente
        assert!(!sc.fly.strand.genome.is_empty(), "Fly should have genomes");
    }

    #[test]
    fn test_sc_parse_validation() {
        let input = fs::read_to_string("tests/fixtures/fragments/program/behavior.sc").unwrap();
        let sc = SC::parse(input.clone());

        // Verifica se temos um fly válido
        assert!(!sc.fly.strand.genome.is_empty(), "Fly should have genomes");
    }

    #[test]
    fn test_sc_from_pair_and_parse_consistency() {
        let input = fs::read_to_string("tests/fixtures/fragments/program/anatomy.sc").unwrap();

        // Parse usando parse
        let sc_parse = SC::parse(input.clone());

        // Parse manual para comparação
        let pair = SCP::parse(Rule::sc, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");

        let sc_from_pair = SC::from_pair(pair);

        // Ambos devem produzir o mesmo resultado
        assert_eq!(
            sc_parse.fly.strand.genome.len(),
            sc_from_pair.fly.strand.genome.len()
        );
    }
}
