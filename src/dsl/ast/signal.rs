use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::behavior::Behavior;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Signal {
    Behavior(Behavior),
}

impl Signal {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::signal);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::behavior => Signal::Behavior(Behavior::from_pair(inner_pair)),
            _ => panic!("Unexpected rule in signal: {:?}", inner_pair.as_rule()),
        }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::signal, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Signal::from_pair(pair))
    }
}
