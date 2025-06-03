use crate::dsl::ast::fly::Fly;
use crate::dsl::parser::parser::{Rule, SCP};
use pest::Parser;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    use pest::Parser;

    use crate::{
        dsl::{
            ast::sc::SC,
            parser::parser::{Rule, SCP},
        },
        tests::load_fragment,
    };

    #[test]
    fn test_sc_parse_anatomy() {
        // Carrega o arquivo de fixture usando o helper
        let input = load_fragment("program/anatomy.sc");

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
        let input = load_fragment("program/behavior.sc");

        // Verifica que o arquivo não está vazio
        assert!(!input.is_empty(), "Fixture file should not be empty");

        // Testa o parse
        let sc = SC::parse(input.clone());

        // Verifica se o fly foi parseado corretamente
        assert!(!sc.fly.strand.genome.is_empty(), "Fly should have genomes");
    }

    #[test]
    fn test_sc_parse_validation() {
        let input = load_fragment("program/behavior.sc");
        let sc = SC::parse(input.clone());

        // Verifica se temos um fly válido
        assert!(!sc.fly.strand.genome.is_empty(), "Fly should have genomes");
    }

    #[test]
    fn test_sc_from_pair_and_parse_consistency() {
        let input = load_fragment("program/anatomy.sc");

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
