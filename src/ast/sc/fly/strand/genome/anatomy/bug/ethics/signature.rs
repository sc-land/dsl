use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::behavior::EthicsBind;
use crate::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    pub binds: Option<Vec<EthicsBind>>,
}

impl Signature {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::signature);

        let mut ethics_binds = Vec::new();
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::ethics_binds => {
                    for ethics_bind_pair in inner_pair.into_inner() {
                        if ethics_bind_pair.as_rule() == Rule::ethics_bind {
                            ethics_binds.push(EthicsBind::from_pair(ethics_bind_pair));
                        }
                    }
                }
                Rule::sequence => {

                }
                _ => panic!("Invalid rule in signature"),
            }
        }
        let binds = if ethics_binds.is_empty() { None } else { Some(ethics_binds) };
        Signature { binds }
    }
}
