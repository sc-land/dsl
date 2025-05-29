use pest::iterators::Pair;
use pest::Parser;
use crate::dsl::ast::bug::Bug;
use crate::dsl::parser::parser::{Rule, SCP};

#[derive(Debug, Clone)]
pub enum DDL {
    Bug(Bug),
}

impl DDL {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let inner = pair.into_inner();
        let bug = Bug::from_pair(inner.clone().next().unwrap());
        DDL::Bug(bug)
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut pairs = SCP::parse(Rule::ddl, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(DDL::from_pair(pair))
    }

    pub fn get_bug(&self) -> &Bug {
        match self {
            DDL::Bug(bug) => bug,
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;
    use crate::dsl::parser::parser::SCP;

    #[test]
    fn parse_ddl_from_valid_pair_should_succeed() {
        let input = "bug Cat".to_string();
        let parsed = SCP::parse(Rule::ddl, &input).unwrap();
        let ddl = DDL::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(ddl.get_bug().raw, "bug Cat");
    }

    #[test]
    fn parse_ddl_from_valid_string_should_succeed() {
        let input = "bug Dog".to_string();
        let ddl = DDL::from_string(input).unwrap();
        assert_eq!(ddl.get_bug().raw, "bug Dog");
    }

    #[test]
    fn get_bug_should_return_parsed_bug_instance() {
        let input = "bug Cat".to_string();
        let ddl = DDL::from_string(input).unwrap();
        assert_eq!(ddl.get_bug().raw, "bug Cat");
    }

    #[test]
    fn parse_ddl_from_empty_string_should_return_error() {
        let input = "".to_string();
        let result = DDL::from_string(input);

        assert!(result.is_err());
    }

    #[test]
    fn parse_ddl_from_empty_pair_should_fail() {
        let input = "".to_string();
        let parse_result = SCP::parse(Rule::ddl, &input);
        assert!(parse_result.is_err());

        let error = parse_result.unwrap_err();
        let error_string = format!("{:?}", error);
        assert!(error_string.contains("bug"));
    }

    #[test]
    fn parse_ddl_with_invalid_syntax_should_fail() {
        let input = "bug !Cat".to_string();
        let result = DDL::from_string(input);
        assert!(result.is_err());
    }
}
