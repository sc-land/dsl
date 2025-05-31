use pest::iterators::Pair;
use pest::Parser;
use crate::dsl::ast::genome::Genome;
use crate::dsl::parser::parser::{Rule, SCP};

#[derive(Debug, Clone)]
pub struct Strand {
    pub raw: String,
    pub genome: Vec<Genome>,
}

impl Strand {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::strand);
        let raw = pair.as_str().to_string();

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
        Strand { raw, genome }
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
    use pest::Parser;
    use crate::dsl::parser::parser::SCP;
    use super::*;


    #[test]
    fn test_strand_from_pair() {
        let input = "bug People".to_string();
        let parsed = SCP::parse(Rule::strand, &input).unwrap();
        let strand = Strand::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(strand.raw, "bug People");
        assert_eq!(strand.genome.len(), 1);
    }

    #[test]
    fn test_strand_from_string() {
        let input = "bug ATCG".to_string();
        let strand = Strand::from_string(input);
        assert_eq!(strand.raw, "bug ATCG");
        assert_eq!(strand.genome.len(), 1);
    }
}
