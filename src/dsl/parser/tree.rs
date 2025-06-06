use pest::Parser;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::sc::SC;
use crate::dsl::parser::parser::{Rule, SCP};
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
    pub fn parse_input(input: String) -> Result<Self, TreeParseError> {
        let parsed = SCP::parse(Rule::sc, &input);
        match parsed {
            Ok(parsed) => {
                let sc = SC::from_pair(parsed.clone().next().unwrap());
                Ok(Tree { sc })
            }
            Err(e) => Err(TreeParseError::from(e)),
        }
    }

    // #[deprecated(since = "próxima versão", note = "Este método será removido em versões futuras")]
    // pub fn get_sc(&self) -> &SC {
    //     &self.sc
    // }
}
