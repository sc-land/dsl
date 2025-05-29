use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone)]
pub struct Oop {
    pub raw: String,
    pub emitter: String,
    pub trails: Vec<String>,
}

impl Oop {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::oop);
        let raw = pair.as_str().to_string();

        let mut inner = pair.into_inner();

        // Parse emitter (specie | tag | literal)
        let emitter_pair = inner.next().expect("Oop deve ter um emitter");
        let emitter = emitter_pair.as_str().to_string();

        // Parse trails (catalysis | carrier)*
        let mut trails = Vec::new();
        for trail_pair in inner {
            trails.push(trail_pair.as_str().to_string());
        }

        Oop { raw, emitter, trails }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::oop, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Oop::from_pair(pair))
    }

    pub fn get_emitter(&self) -> &str {
        &self.emitter
    }

    pub fn get_trails(&self) -> &[String] {
        &self.trails
    }

    pub fn has_trails(&self) -> bool {
        !self.trails.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::dsl::parser::parser::{Rule, SCP};
    use super::*;

    #[test]
    fn test_oop_from_pair() {
        let input = "bug.happens".to_string();
        let parsed = SCP::parse(Rule::oop, &input).unwrap();
        let oop = Oop::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(oop.raw, "bug.happens");
        assert_eq!(oop.emitter, "bug");
        assert_eq!(oop.trails.len(), 1);
        assert_eq!(oop.trails[0], ".happens");
    }

    #[test]
    fn test_oop_simple_emitter() {
        let input = "Happens".to_string();
        let parsed = SCP::parse(Rule::oop, &input).unwrap();
        let oop = Oop::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(oop.raw, "Happens");
        assert_eq!(oop.emitter, "Happens");
        assert_eq!(oop.trails.len(), 0);
    }

    #[test]
    fn test_oop_from_string() {
        let input = "Class.method".to_string();
        let oop = Oop::from_string(input).unwrap();
        assert_eq!(oop.get_emitter(), "Class");
        assert!(oop.has_trails());
        assert_eq!(oop.get_trails().len(), 1);
        assert_eq!(oop.get_trails()[0], ".method");
    }

    #[test]
    fn test_oop_getters() {
        let input = "Simple".to_string();
        let oop = Oop::from_string(input).unwrap();
        assert_eq!(oop.get_emitter(), "Simple");
        assert!(!oop.has_trails());
        assert_eq!(oop.get_trails().len(), 0);
    }
}
