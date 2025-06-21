use pest::Parser;
use serde::{Deserialize, Serialize};
use crate::ast::sc::SC;
use crate::parser::parser::{Rule, SCP};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TreeParseError {
    #[error("SC Parsing failed{0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("Falha ao processar árvore vazia")]
    EmptyTree,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tree {
    pub sc: SC,
}

impl Tree {
    pub fn parse(input: String) -> Result<Self, TreeParseError> {
        let parsed = SCP::parse(Rule::sc, &input);
        match parsed {
            Ok(parsed) => {
                let sc = SC::from_pair(parsed.clone().next().unwrap());
                Ok(Tree { sc })
            }
            Err(e) => Err(TreeParseError::from(e)),
        }
    }
}
