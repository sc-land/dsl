use pest::{iterators::Pair, Parser};
use crate::dsl::parser::parser::{Rule, SCP};

#[derive(Debug, Clone)]
pub struct Bug {
    pub raw: String,
}

impl Bug {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::bug);

        let raw = pair.as_str().to_string();

        Bug { raw }
    }

    pub fn from_string(input: String) -> Self {
        let pair = SCP::parse(Rule::bug, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");
        Bug::from_pair(pair)
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::dsl::ast::bug::Bug;
    use crate::dsl::parser::parser::{Rule, SCP};

    #[test]
    fn test_bug_from_pair() {
        let input = "bug Cat".to_string();
        let parsed = SCP::parse(Rule::bug, &input).unwrap();
        let bug = Bug::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(bug.raw, "bug Cat");
    }

    #[test]
    fn test_bug_from_string() {
        let input = "bug Dog".to_string();
        let bug = Bug::from_string(input);
        assert_eq!(bug.raw, "bug Dog");
    }

}

