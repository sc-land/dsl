use pest::iterators::Pair;
use crate::dsl::ast::bug::Bug;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone)]
pub struct DDL {
    pub bug: Bug,
}

impl DDL {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let inner = pair.into_inner();
        let bug = Bug::from_pair(inner.clone().next().unwrap());
        DDL { bug }
    }
}
