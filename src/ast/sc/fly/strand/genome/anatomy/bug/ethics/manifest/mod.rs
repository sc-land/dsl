use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::behavior::Pact;
use crate::ast::sc::fly::strand::genome::behavior::trace::Trace;
use crate::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Manifest {
    Pacts(Vec<Pact>),
    March(Vec<Trace>),
}

impl Manifest {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::manifest);

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::pacts => {
                    let mut ethics_binds = Vec::new();
                    for ethics_bind_pair in inner_pair.into_inner() {
                        if ethics_bind_pair.as_rule() == Rule::pact {
                            ethics_binds.push(Pact::from_pair(ethics_bind_pair));
                        }
                    }
                    return Manifest::Pacts(ethics_binds);
                }
                Rule::march => {
                    let mut traces = Vec::new();
                    for trace_pair in inner_pair.into_inner() {
                        if trace_pair.as_rule() == Rule::trace {
                            traces.push(Trace::from_pair(trace_pair));
                        }
                    }
                    return Manifest::March(traces);
                }
                _ => panic!("Invalid rule in manifest: {:?}", inner_pair.as_rule()),
            }
        }

        // Default case - empty manifest becomes empty EthicsBinds
        Manifest::Pacts(Vec::new())
    }
}
