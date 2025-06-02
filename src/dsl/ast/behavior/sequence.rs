use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use super::oop::Oop;

#[derive(Debug, Clone, PartialEq)]
pub struct Sequence {
    pub oops: Vec<Oop>,
}

impl Sequence {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::sequence);

        let mut oops = Vec::new();
        for oop_pair in pair.into_inner() {
            if oop_pair.as_rule() == Rule::oop {
                oops.push(Oop::from_pair(oop_pair));
            }
        }

        Sequence { oops }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::sequence, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Sequence::from_pair(pair))
    }

    pub fn get_oops(&self) -> &[Oop] {
        &self.oops
    }

    pub fn len(&self) -> usize {
        self.oops.len()
    }

    pub fn is_empty(&self) -> bool {
        self.oops.is_empty()
    }
}
