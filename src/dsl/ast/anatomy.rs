use pest::iterators::Pair;
use pest::Parser;
use crate::dsl::ast::bug::Bug;
use crate::dsl::parser::parser::{Rule, SCP};

#[derive(Debug, Clone)]
pub enum Anatomy {
    Bug(Bug),
}

impl Anatomy {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let inner = pair.into_inner();
        let bug = Bug::from_pair(inner.clone().next().unwrap());
        Anatomy::Bug(bug)
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut pairs = SCP::parse(Rule::anatomy, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Anatomy::from_pair(pair))
    }

    pub fn get_bug(&self) -> &Bug {
        match self {
            Anatomy::Bug(bug) => bug,
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;
    use crate::dsl::parser::parser::SCP;

    #[test]
    fn parse_anatomy_from_valid_pair_should_succeed() {
        let input = "bug Cat".to_string();
        let parsed = SCP::parse(Rule::anatomy, &input).unwrap();
        let anatomy = Anatomy::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(anatomy.get_bug().raw, "bug Cat");
    }

    #[test]
    fn parse_anatomy_from_valid_string_should_succeed() {
        let input = "bug Dog".to_string();
        let anatomy = Anatomy::from_string(input).unwrap();
        assert_eq!(anatomy.get_bug().raw, "bug Dog");
    }

    #[test]
    fn get_bug_should_return_parsed_bug_instance() {
        let input = "bug Cat".to_string();
        let anatomy = Anatomy::from_string(input).unwrap();
        assert_eq!(anatomy.get_bug().raw, "bug Cat");
    }

    #[test]
    fn parse_anatomy_from_empty_string_should_return_error() {
        let input = "".to_string();
        let result = Anatomy::from_string(input);

        assert!(result.is_err());
    }

    #[test]
    fn parse_anatomy_from_empty_pair_should_fail() {
        let input = "".to_string();
        let parse_result = SCP::parse(Rule::anatomy, &input);
        assert!(parse_result.is_err());
    }

    #[test]
    fn parse_anatomy_with_invalid_syntax_should_fail() {
        let input = "bug !Cat".to_string();
        let result = Anatomy::from_string(input);
        assert!(result.is_err());
    }
}
