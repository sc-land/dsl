use pest::iterators::Pair;
use pest::Parser;
use crate::dsl::ast::genome::Genome;
use crate::dsl::parser::parser::{Rule, SCP};

#[derive(Debug, Clone)]
pub struct Strand {
    pub genome: Vec<Genome>,
}

impl Strand {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::strand);

        let mut genome = Vec::new();
        for genome_pair in pair.into_inner() {
            match genome_pair.as_rule() {
                Rule::genome => {
                    let genome_instance = Genome::from_pair(genome_pair);
                    genome.push(genome_instance);
                },
                _ => unreachable!("Unexpected rule in Strand::from_pair"),
            }
        }
        Strand { genome }
    }

    pub fn from_string(input: String) -> Self {
        let pair = SCP::parse(Rule::strand, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");
        Strand::from_pair(pair)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::dsl::ast::strand::Strand;

    #[test]
    fn test_strand_from_string() {
        // Carrega o arquivo de fixture usando o helper
        let path = "tests/fixtures/fragments/bug/two_bugs.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read input.sc file");

        assert!(!input.is_empty(), "Fixture file should not be empty");

        let strand = Strand::from_string(input.clone());

        // Verifica se o strand contém dois genomes
        assert_eq!(strand.genome.len(), 2, "Strand should contain two genomes");
    }
}
