use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::behavior::Binds;
use crate::ast::sc::fly::strand::genome::behavior::march::March;
use crate::parser::parser::Rule;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Transport {
    Binds(Binds),
    Sequence(March),
}

impl Transport {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::transport);

        let inner_pair = pair.into_inner().next().expect("Transport deve ter uma regra interna");
        match inner_pair.as_rule() {
            Rule::binds => Transport::Binds(Binds::from_pair(inner_pair)),
            Rule::march => Transport::Sequence(March::from_pair(inner_pair)),
            _ => panic!("Regra inesperada dentro de transport: {:?}", inner_pair.as_rule()),
        }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::transport, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Transport::from_pair(pair))
    }

    pub fn is_binds(&self) -> bool {
        matches!(self, Transport::Binds(_))
    }

    pub fn is_sequence(&self) -> bool {
        matches!(self, Transport::Sequence(_))
    }

    pub fn as_binds(&self) -> Option<&Binds> {
        match self {
            Transport::Binds(binds) => Some(binds),
            _ => None,
        }
    }

    pub fn as_sequence(&self) -> Option<&March> {
        match self {
            Transport::Sequence(sequence) => Some(sequence),
            _ => None,
        }
    }
}
