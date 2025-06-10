use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::sc::fly::strand::genome::Behavior;
use crate::dsl::parser::parser::{Rule, SCP};

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
        let mut pairs = SCP::parse(Rule::signal, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Signal::from_pair(pair))
    }
}
