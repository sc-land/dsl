use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::behavior::trace::Trace;
use crate::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Nectar {
    pub font: Trace,
}

impl Nectar {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::nectar);

        // A regra de retorno contém um oop que é o valor a ser retornado
        let oop_pair = pair
            .into_inner()
            .next()
            .expect("Return statement should have a value");
        let font = Trace::from_pair(oop_pair);

        Nectar { font }
    }
}
