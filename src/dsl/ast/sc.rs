use crate::dsl::parser::parser::{Rule, SCP};
use pest::iterators::Pair;
use pest::Parser;
use crate::dsl::ast::fly::Fly;

#[derive(Debug, Clone)]
pub struct SC {
    pub raw: String,
    pub fly: Fly,
}

impl SC {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::sc);
        let raw = pair.as_str().to_string();

        let fly = Fly::from_pair(pair.into_inner().next().unwrap());

        SC { raw, fly }
    }

    pub fn from_string(input: String) -> Self {
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
    use super::*;
    use crate::dsl::parser::parser::SCP;

    #[test]
    fn test_sc_from_pair() {
        let input = "fly".to_string();
        let parsed = SCP::parse(Rule::sc, &input).unwrap();
        let sc = SC::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(sc.raw, "fly");
    }

    #[test]
    fn test_sc_from_string() {
        let input = "fly".to_string();
        let sc = SC::from_string(input);
        assert_eq!(sc.raw, "fly");
    }

    #[test]
    fn test_sc_empty_input() {
        let input = "".to_string();
        let result = SCP::parse(Rule::sc, &input);
        assert!(result.is_err());
    }

    #[test]
    fn test_sc_invalid_input() {
        let input = "!".to_string();
        let result = SCP::parse(Rule::sc, &input);
        assert!(result.is_err());
    }
}
