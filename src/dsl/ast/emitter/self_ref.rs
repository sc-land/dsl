use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq)]
pub struct SelfRef {
    pub raw: String,
}

impl SelfRef {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::self_ref);
        let raw = pair.as_str().to_string();

        SelfRef { raw }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::self_ref, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(SelfRef::from_pair(pair))
    }

    pub fn new() -> Self {
        SelfRef {
            raw: "$".to_string(),
        }
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }

    pub fn is_self_reference(&self) -> bool {
        self.raw == "$"
    }
}

impl Default for SelfRef {
    fn default() -> Self {
        Self::new()
    }
}
