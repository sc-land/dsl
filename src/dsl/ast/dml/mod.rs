pub mod assign;
pub mod oop;

use pest::{iterators::Pair, Parser};
use crate::dsl::parser::parser::{Rule, SCP};
use self::assign::Assign;
use self::oop::Oop;

#[derive(Debug, Clone)]
pub enum DML {
    Assign(Assign),
    Oop(Oop),
}

impl DML {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::dml);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::assign => DML::Assign(Assign::from_pair(inner_pair)),
            Rule::oop => DML::Oop(Oop::from_pair(inner_pair)),
            _ => panic!("Regra inesperada dentro de dml: {:?}", inner_pair.as_rule()),
        }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut pairs = SCP::parse(Rule::dml, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(DML::from_pair(pair))
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::dsl::parser::parser::{Rule, SCP};
    use super::*;

    #[test]
    fn test_dml_from_pair_oop() {
        let input = "bug.happens".to_string();
        let parsed = SCP::parse(Rule::dml, &input).unwrap();

        let dml = DML::from_pair(parsed.into_iter().next().unwrap());
        if let DML::Oop(oop) = dml {
            assert_eq!(oop.raw, "bug.happens");
        } else {
            panic!("Expected Oop variant");
        }
    }

    #[test]
    fn test_dml_from_string_oop() {
        let input = "bug.happens".to_string();
        let parsed = DML::from_string(input).unwrap();
        if let DML::Oop(oop) = parsed {
            assert_eq!(oop.raw, "bug.happens");
        } else {
            panic!("Expected Oop variant");
        }
    }

    #[test]
    fn test_dml_from_pair_assign() {
        let input = "bug = Happens.now".to_string();
        let parsed = SCP::parse(Rule::dml, &input).unwrap();

        let dml = DML::from_pair(parsed.into_iter().next().unwrap());
        if let DML::Assign(assign) = dml {
            assert_eq!(assign.raw, "bug = Happens.now");
        } else {
            panic!("Expected Assign variant");
        }
    }

    #[test]
    fn test_dml_from_string_assign() {
        let input = "bug = Happens.now".to_string();
        let parsed = DML::from_string(input).unwrap();
        if let DML::Assign(assign) = parsed {
            assert_eq!(assign.raw, "bug = Happens.now");
        } else {
            panic!("Expected Assign variant");
        }
    }
}
