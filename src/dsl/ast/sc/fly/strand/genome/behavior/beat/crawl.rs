use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::sc::fly::strand::genome::anatomy::bug::ethics::matrix::Matrix;
use crate::dsl::ast::sc::fly::strand::genome::anatomy::bug::gene::primor::Primor;
use crate::dsl::ast::sc::fly::strand::genome::behavior::trace::Trace;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Crawl {
    pub raw: String,
    pub variable: Primor,
    pub iterable: Trace,
    pub block: Matrix,
}

impl Crawl {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::crawl);
        let raw = pair.as_str().to_string();

        let mut pairs = pair.into_inner();

        // Parse variable (each)
        let variable_pair = pairs.next().expect("For statement should have a variable");
        assert_eq!(variable_pair.as_rule(), Rule::each);
        let tag_pair = variable_pair
            .into_inner()
            .next()
            .expect("Each should have a tag");
        let variable = Primor::new(tag_pair.as_str().to_string());

        // Parse iterable (oop)
        let oop_pair = pairs
            .next()
            .expect("For statement should have an iterable oop");
        let iterable = Trace::from_pair(oop_pair);

        // Parse loop body block
        let matrix_pair = pairs
            .next()
            .expect("For statement should have a body block");
        let block = Matrix::from_pair(matrix_pair);

        Crawl {
            raw,
            variable,
            iterable,
            block,
        }
    }
}
