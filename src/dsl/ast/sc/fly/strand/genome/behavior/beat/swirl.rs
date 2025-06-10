use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::sc::fly::strand::genome::anatomy::bug::ethics::matrix::Matrix;
use crate::dsl::ast::sc::fly::strand::genome::behavior::trace::Trace;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Swirl {
    pub condition: Trace,
    pub block: Matrix,
}

impl Swirl {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::swirl);

        let mut pairs = pair.into_inner();

        // Parse condition
        let condition_pair = pairs
            .next()
            .expect("While statement should have a condition");
        let condition = Trace::from_pair(condition_pair);

        // Parse loop body block
        let matrix_pair = pairs
            .next()
            .expect("While statement should have a body block");
        let block = Matrix::from_pair(matrix_pair);

        Swirl {
            condition,
            block,
        }
    }
}
