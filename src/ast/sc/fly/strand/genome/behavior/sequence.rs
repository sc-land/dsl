use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::parser::parser::Rule;
use super::trace::Trace;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sequence {
    pub oops: Vec<Trace>,
}

impl Sequence {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::sequence);

        let mut oops = Vec::new();
        for oop_pair in pair.into_inner() {
            if oop_pair.as_rule() == Rule::trace {
                oops.push(Trace::from_pair(oop_pair));
            }
        }

        Sequence { oops }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::sequence, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Sequence::from_pair(pair))
    }

    pub fn get_oops(&self) -> &[Trace] {
        &self.oops
    }

    pub fn len(&self) -> usize {
        self.oops.len()
    }

    pub fn is_empty(&self) -> bool {
        self.oops.is_empty()
    }
}
