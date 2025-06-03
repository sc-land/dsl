use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::Rule;
use super::bind::{Bind, EthicsBind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Binds {
    pub binds: Vec<Bind>,
}

/// EthicsBinds represents function parameter bindings for ethics functions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthicsBinds {
    pub binds: Vec<EthicsBind>,
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
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::binds, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Binds::from_pair(pair))
    }

    pub fn from_ethics_binds_pair(pair: Pair<Rule>) -> EthicsBinds {
        assert_eq!(pair.as_rule(), Rule::ethics_binds);

        let mut binds = Vec::new();
        for ethics_bind_pair in pair.into_inner() {
            if ethics_bind_pair.as_rule() == Rule::ethics_bind {
                binds.push(EthicsBind::from_pair(ethics_bind_pair));
            }
        }

        EthicsBinds { binds }
    }

    pub fn get_binds(&self) -> &[Bind] {
        &self.binds
    }

    pub fn len(&self) -> usize {
        self.binds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.binds.is_empty()
    }
}

impl EthicsBinds {
    pub fn get_binds(&self) -> &[EthicsBind] {
        &self.binds
    }

    pub fn len(&self) -> usize {
        self.binds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.binds.is_empty()
    }
}
