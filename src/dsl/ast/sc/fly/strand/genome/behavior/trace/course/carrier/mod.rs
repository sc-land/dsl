use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::sc::fly::strand::genome::behavior::transport::Transport;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Carrier {
    pub transport: Option<Transport>,
}

impl Carrier {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::carrier);

        // Parse optional transport
        let transport = pair.into_inner().next().map(|transport_pair| Transport::from_pair(transport_pair));

        Carrier { transport }
    }
}

