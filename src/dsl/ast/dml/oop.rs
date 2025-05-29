use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone)]
pub struct Oop {
    pub raw: String,
}

impl Oop {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::oop);
        let raw = pair.as_str().to_string();
        Oop { raw }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::dsl::parser::parser::{Rule, SCP};
    use super::*;

    #[test]
    fn test_oop_from_pair() {
        let input = "bug.happens".to_string();
        let parsed = SCP::parse(Rule::oop, &input).unwrap();
        let oop = Oop::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(oop.raw, "bug.happens");
    }
}
