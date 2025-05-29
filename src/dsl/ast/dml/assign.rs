use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::dml::oop::Oop;

#[derive(Debug, Clone)]
pub struct Assign {
    pub raw: String,
    pub tag: String,
    pub oop: Oop,
}

impl Assign {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::assign);
        let raw = pair.as_str().to_string();

        let mut inner = pair.into_inner();

        // Parse tag
        let tag_pair = inner.next().expect("Assign deve ter uma tag");
        let tag = tag_pair.as_str().to_string();

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
    fn test_assign_from_pair() {
        let input = "bug = Happens.now".to_string();
        let parsed = SCP::parse(Rule::assign, &input).unwrap();
        let assign = Assign::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(assign.raw, "bug = Happens.now");
        assert_eq!(assign.tag, "bug");
        assert_eq!(assign.oop.raw, "Happens.now");
    }

    #[test]
    fn test_assign_from_string() {
        let input = "variable = Method.call".to_string();
        let assign = Assign::from_string(input).unwrap();
        assert_eq!(assign.get_tag(), "variable");
        assert_eq!(assign.get_oop().get_emitter(), "Method");
        assert_eq!(assign.get_oop().get_trails().len(), 1);
        assert_eq!(assign.get_oop().get_trails()[0], ".call");
    }

    #[test]
    fn test_assign_from_string_2() {
        let input = "variable = Method.call.another_call".to_string();
        let assign = Assign::from_string(input).unwrap();
        assert_eq!(assign.get_tag(), "variable");
        assert_eq!(assign.get_oop().get_emitter(), "Method");
        assert_eq!(assign.get_oop().get_trails().len(), 2);
        assert_eq!(assign.get_oop().get_trails()[0], ".call");
        assert_eq!(assign.get_oop().get_trails()[1], ".another_call");
    }

    #[test]
    fn test_assign_getters() {
        let input = "test = SimpleValue".to_string();
        let assign = Assign::from_string(input).unwrap();
        assert_eq!(assign.get_tag(), "test");
        assert_eq!(assign.get_oop().get_emitter(), "SimpleValue");
        assert!(!assign.get_oop().has_trails());
    }
}
