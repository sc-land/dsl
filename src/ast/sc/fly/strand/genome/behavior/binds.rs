use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::parser::parser::Rule;
use super::pact::{Bind, Pact};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Binds {
    pub binds: Vec<Bind>,
}

/// EthicsBinds represents function parameter bindings for ethics functions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthicsBinds {
    pub binds: Vec<Pact>,
}

impl Binds {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::binds);

        let mut binds = Vec::new();
        for bind_pair in pair.into_inner() {
            if bind_pair.as_rule() == Rule::bind {
                binds.push(Bind::from_pair(bind_pair));
            }
        }

        Binds { binds }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::binds, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Binds::from_pair(pair))
    }
}
