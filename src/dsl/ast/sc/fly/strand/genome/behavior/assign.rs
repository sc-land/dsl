use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::sc::fly::strand::genome::anatomy::bug::gene::tag::Tag;
use crate::dsl::ast::sc::fly::strand::genome::behavior::trace::Trace;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pollinate {
    pub raw: String,
    pub tag: Tag,
    pub oop: Trace,
}

impl Pollinate {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::assign);
        let raw = pair.as_str().to_string();

        let mut inner = pair.into_inner();

        // Parse tag
        let tag_pair = inner.next().expect("Assign deve ter uma tag");
        let tag = Tag::from_pair(tag_pair);

        // Skip the "=" symbol (it's not captured as a rule)

        // Parse oop
        let oop_pair = inner.next().expect("Assign deve ter um objeto oop");
        let oop = Trace::from_pair(oop_pair);

        Pollinate { raw, tag, oop }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::assign, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Pollinate::from_pair(pair))
    }
}

