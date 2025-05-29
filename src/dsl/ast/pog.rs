use pest::iterators::Pair;
use pest::Parser;
use crate::dsl::ast::genome::Genome;
use crate::dsl::parser::parser::{Rule, SCP};

#[derive(Debug, Clone)]
pub struct Pog {
    pub raw: String,
    pub genome: Vec<Genome>,
}

impl Pog {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::pog);
        let raw = pair.as_str().to_string();

        let mut genome = Vec::new();
        for genome_pair in pair.into_inner() {
            match genome_pair.as_rule() {
                Rule::genome => {
                    let pog_instance = Genome::from_pair(genome_pair);
                    genome.push(pog_instance);
                },
                _ => unreachable!("Unexpected rule in Fly::from_pest"),
            }
        }
        Pog { raw, genome }
    }

    pub fn from_string(input: String) -> Self {
        let pair = SCP::parse(Rule::pog, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");
        Pog::from_pair(pair)
    }

}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::dsl::parser::parser::SCP;
    use super::*;


    #[test]
    fn test_pog_from_pair() {
        let input = "bug People".to_string();
        let parsed = SCP::parse(Rule::pog, &input).unwrap();
        let pog = Pog::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(pog.raw, "bug People");
        assert_eq!(pog.genome.len(), 1);
    }

    #[test]
    fn test_pog_from_string() {
        let input = "bug\nATCG".to_string();
        let pog = Pog::from_string(input);
        assert_eq!(pog.raw, "bug\nATCG");
        assert_eq!(pog.genome.len(), 1);
    }
}
