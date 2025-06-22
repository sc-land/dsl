use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::behavior::EthicsBind;
use crate::ast::sc::fly::strand::genome::behavior::trace::Trace;
use crate::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Signature {
    EthicsBinds(Vec<EthicsBind>),
    Sequence(Vec<Trace>),
}

impl Signature {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::signature);

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::ethics_binds => {
                    let mut ethics_binds = Vec::new();
                    for ethics_bind_pair in inner_pair.into_inner() {
                        if ethics_bind_pair.as_rule() == Rule::ethics_bind {
                            ethics_binds.push(EthicsBind::from_pair(ethics_bind_pair));
                        }
                    }
                    return Signature::EthicsBinds(ethics_binds);
                }
                Rule::sequence => {
                    let mut traces = Vec::new();
                    for trace_pair in inner_pair.into_inner() {
                        if trace_pair.as_rule() == Rule::trace {
                            traces.push(Trace::from_pair(trace_pair));
                        }
                    }
                    return Signature::Sequence(traces);
                }
                _ => panic!("Invalid rule in signature: {:?}", inner_pair.as_rule()),
            }
        }
        
        // Default case - empty signature becomes empty EthicsBinds
        Signature::EthicsBinds(Vec::new())
    }
}
