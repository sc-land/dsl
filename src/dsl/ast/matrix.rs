use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::signal::Signal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix {
    pub raw: String,
    pub signals: Vec<Signal>,
}

impl Matrix {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::matrix);
        let raw = pair.as_str().to_string();

        let mut signals = Vec::new();
        for signal_pair in pair.into_inner() {
            if signal_pair.as_rule() == Rule::signal {
                signals.push(Signal::from_pair(signal_pair));
            }
        }

        Matrix { raw, signals }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::matrix, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Matrix::from_pair(pair))
    }
}
