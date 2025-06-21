pub mod sprout;
pub mod swirl;
pub mod crawl;
pub mod nectar;

use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::behavior::beat::crawl::Crawl;
use crate::ast::sc::fly::strand::genome::behavior::beat::nectar::Nectar;
use crate::ast::sc::fly::strand::genome::behavior::beat::sprout::Sprout;
use crate::ast::sc::fly::strand::genome::behavior::beat::swirl::Swirl;
use crate::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Beat {
    Sprout(Sprout),
    Swirl(Swirl),
    Crawl(Crawl),
    Nectar(Nectar),
}

impl Beat {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::beat);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::sprout => Beat::Sprout(Sprout::from_pair(inner_pair)),
            Rule::swirl => Beat::Swirl(Swirl::from_pair(inner_pair)),
            Rule::crawl => Beat::Crawl(Crawl::from_pair(inner_pair)),
            Rule::nectar => Beat::Nectar(Nectar::from_pair(inner_pair)),
            _ => panic!("Unexpected rule in statement: {:?}", inner_pair.as_rule()),
        }
    }
}
