use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use super::oop::Oop;

#[derive(Debug, Clone)]
pub struct Bind {
    pub raw: String,
    pub tag: String,
    pub oop: Oop,
}

impl Bind {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::bind);
        let raw = pair.as_str().to_string();

        let mut inner = pair.into_inner();

        // Parse tag
        let tag_pair = inner.next().expect("Bind deve ter uma tag");
        let tag = tag_pair.as_str().to_string();

        // Parse oop
        let oop_pair = inner.next().expect("Bind deve ter um oop");
        let oop = Oop::from_pair(oop_pair);

        Bind { raw, tag, oop }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::bind, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Bind::from_pair(pair))
    }

    pub fn get_tag(&self) -> &str {
        &self.tag
    }

    pub fn get_oop(&self) -> &Oop {
        &self.oop
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::dsl::parser::parser::{Rule, SCP};
    use super::*;

    #[test]
    fn test_bind_from_pair() {
        let input = "name: Value".to_string();
        let parsed = SCP::parse(Rule::bind, &input).unwrap();
        let bind = Bind::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(bind.raw, "name: Value");
        assert_eq!(bind.tag, "name");
        assert_eq!(bind.oop.get_emitter(), "Value");
    }

    #[test]
    fn test_bind_from_string() {
        let input = "method: Class.call".to_string();
        let bind = Bind::from_string(input).unwrap();
        assert_eq!(bind.get_tag(), "method");
        assert_eq!(bind.get_oop().get_emitter(), "Class");
        assert_eq!(bind.get_oop().get_trails().len(), 1);
    }

    #[test]
    fn test_bind_with_literal() {
        let input = "count: 42".to_string();
        let bind = Bind::from_string(input).unwrap();
        assert_eq!(bind.get_tag(), "count");
        assert_eq!(bind.get_oop().get_emitter(), "42");
        assert!(bind.get_oop().emitter_is_literal());
    }

    #[test]
    fn test_bind_getters() {
        let input = "test: SimpleValue".to_string();
        let bind = Bind::from_string(input).unwrap();
        assert_eq!(bind.get_tag(), "test");
        assert_eq!(bind.get_oop().get_emitter(), "SimpleValue");
        assert!(!bind.get_oop().has_trails());
    }
}
