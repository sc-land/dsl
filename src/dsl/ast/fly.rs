use pest::iterators::Pair;
use pest::Parser;
use crate::dsl::ast::strand::Strand;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone)]
pub struct Fly {
    pub raw: String,
    pub strands: Vec<Strand>,
}

impl Fly {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::fly);
        let raw = pair.as_str().to_string();

        let mut strands = Vec::new();

        for strand_pair in pair.into_inner() {
            match strand_pair.as_rule() {
                Rule::strand => {
                    let strand_instance = Strand::from_pair(strand_pair);
                    strands.push(strand_instance);
                },
                _ => unreachable!("Unexpected rule in Fly::from_pair"),
            }
        }
        Fly { raw, strands }
    }

    pub fn from_string(input: String) -> Self {
        let pair = crate::dsl::parser::parser::SCP::parse(Rule::fly, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");
        Fly::from_pair(pair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::parser::parser::SCP;

    #[test]
    fn test_fly_from_pair() {
        let input = "bug Cat".to_string();
        let parsed = SCP::parse(Rule::fly, &input).unwrap();
        let fly = Fly::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(fly.strands.len(), 1);
        assert_eq!(fly.raw, "bug Cat");
        assert_eq!(fly.strands[0].raw, "bug Cat");
    }

    #[test]
    fn test_fly_from_string() {
        let input = "bug Cat\nbug Dog".to_string();
        let fly = Fly::from_string(input);
        // Com a nova gramática, temos 1 strand contendo 2 genomes
        assert_eq!(fly.strands.len(), 1);
        assert_eq!(fly.raw, "bug Cat\nbug Dog");
        assert_eq!(fly.strands[0].raw, "bug Cat\nbug Dog");
        assert_eq!(fly.strands[0].genome.len(), 2);
    }

    #[test]
    fn test_fly_empty_input() {
        let input = "".to_string();
        let result = SCP::parse(Rule::fly, &input);
        assert!(result.is_err());
    }

    #[test]
    fn test_fly_invalid_input() {
        let input = "!".to_string();
        let result = SCP::parse(Rule::fly, &input);
        assert!(result.is_err());
    }
}
