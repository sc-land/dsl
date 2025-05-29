use pest::iterators::Pair;
use pest::Parser;
use crate::dsl::ast::pog::Pog;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone)]
pub struct Fly {
    pub raw: String,
    pub pogs: Vec<Pog>,
}

impl Fly {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::fly);
        let raw = pair.as_str().to_string();

        let mut pog = Vec::new();

        for pog_pair in pair.into_inner() {
            match pog_pair.as_rule() {
                Rule::pog => {
                    let pog_instance = Pog::from_pair(pog_pair);
                    pog.push(pog_instance);
                },
                _ => unreachable!("Unexpected rule in Fly::from_pest"),
            }
        }
        Fly { raw, pogs: pog }
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
        let input = "fly".to_string();
        let parsed = SCP::parse(Rule::fly, &input).unwrap();
        let fly = Fly::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(fly.pogs.len(), 1);
        assert_eq!(fly.raw, "fly");
        assert_eq!(fly.pogs[0].raw, "fly");
    }

    #[test]
    fn test_fly_from_string() {
        let input = "fly\nbug".to_string();
        let fly = Fly::from_string(input);
        assert_eq!(fly.pogs.len(), 2);
        assert_eq!(fly.raw, "fly\nbug");
        assert_eq!(fly.pogs[0].raw, "fly");
        assert_eq!(fly.pogs[1].raw, "bug");
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
