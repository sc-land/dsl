use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::behavior::oop::Oop;

/// Condition represents a conditional expression in control flow statements
/// Based on the grammar: condition = { oop }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Condition {
    /// A conditional expression containing an oop
    Oop(Oop)
}

impl Condition {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::condition);

        let oop_pair = pair.into_inner().next().expect("Condition must have an oop");
        let oop = Oop::from_pair(oop_pair);

        Condition::Oop(oop)
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::condition, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Condition::from_pair(pair))
    }
}
