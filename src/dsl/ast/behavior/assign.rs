use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::behavior::oop::Oop;
use crate::dsl::ast::emitter::tag::Tag;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assign {
    pub raw: String,
    pub tag: Tag,
    pub oop: Oop,
}

impl Assign {
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
        let oop = Oop::from_pair(oop_pair);

        Assign { raw, tag, oop }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::assign, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Assign::from_pair(pair))
    }

    pub fn get_tag(&self) -> &str {
        self.tag.get_raw()
    }

    pub fn get_oop(&self) -> &Oop {
        &self.oop
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }
}

