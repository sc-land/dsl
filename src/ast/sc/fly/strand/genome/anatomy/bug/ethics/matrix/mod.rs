pub mod signal;

use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::anatomy::bug::ethics::matrix::signal::Signal;
use crate::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Matrix {
    pub signals: Vec<Signal>,
}

impl Matrix {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::matrix);

        let mut signals = Vec::new();
        for signal_pair in pair.into_inner() {
            if signal_pair.as_rule() == Rule::signal {
                signals.push(Signal::from_pair(signal_pair));
            }
        }

        Matrix { signals }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::matrix, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Matrix::from_pair(pair))
    }
}
