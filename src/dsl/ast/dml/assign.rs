use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone)]
pub struct Assign {
    pub raw: String,
}

impl Assign {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::assign);
        let raw = pair.as_str().to_string();
        Assign { raw }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::dsl::parser::parser::{Rule, SCP};
    use super::*;

    #[test]
    fn test_assign_from_pair() {
        let input = "bug = Happens.now".to_string();
        let parsed = SCP::parse(Rule::assign, &input).unwrap();
        let assign = Assign::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(assign.raw, "bug = Happens.now");
    }
}
