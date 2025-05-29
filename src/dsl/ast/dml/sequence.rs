use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use super::oop::Oop;

#[derive(Debug, Clone)]
pub struct Sequence {
    pub raw: String,
    pub oops: Vec<Oop>,
}

impl Sequence {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::sequence);
        let raw = pair.as_str().to_string();

        let mut oops = Vec::new();
        for oop_pair in pair.into_inner() {
            if oop_pair.as_rule() == Rule::oop {
                oops.push(Oop::from_pair(oop_pair));
            }
        }

        Sequence { raw, oops }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::sequence, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Sequence::from_pair(pair))
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
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

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::dsl::parser::parser::{Rule, SCP};
    use super::*;

    #[test]
    fn test_sequence_single() {
        let input = "Value".to_string();
        let parsed = SCP::parse(Rule::sequence, &input).unwrap();
        let sequence = Sequence::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(sequence.raw, "Value");
        assert_eq!(sequence.len(), 1);
        assert_eq!(sequence.get_oops()[0].get_emitter(), "Value");
    }

    #[test]
    fn test_sequence_multiple() {
        let input = "Value, 42, Class.method".to_string();
        let sequence = Sequence::from_string(input).unwrap();
        assert_eq!(sequence.len(), 3);

        assert_eq!(sequence.get_oops()[0].get_emitter(), "Value");
        assert_eq!(sequence.get_oops()[1].get_emitter(), "42");
        assert_eq!(sequence.get_oops()[2].get_emitter(), "Class");
        assert_eq!(sequence.get_oops()[2].get_trails().len(), 1);
    }

    #[test]
    fn test_sequence_from_string() {
        let input = "Point.x, Point.y".to_string();
        let sequence = Sequence::from_string(input).unwrap();
        assert!(!sequence.is_empty());
        assert_eq!(sequence.len(), 2);
        assert_eq!(sequence.get_oops()[0].get_emitter(), "Point");
        assert_eq!(sequence.get_oops()[1].get_emitter(), "Point");
    }

    #[test]
    fn test_sequence_with_literals() {
        let input = "\"hello\", 42, 0xFF".to_string();
        let sequence = Sequence::from_string(input).unwrap();
        assert_eq!(sequence.len(), 3);

        assert!(sequence.get_oops()[0].emitter_is_literal());
        assert!(sequence.get_oops()[1].emitter_is_literal());
        assert!(sequence.get_oops()[2].emitter_is_literal());
    }

    #[test]
    fn test_sequence_getters() {
        let input = "SimpleValue, AnotherValue".to_string();
        let sequence = Sequence::from_string(input).unwrap();
        assert_eq!(sequence.len(), 2);
        assert!(!sequence.is_empty());
        assert_eq!(sequence.get_oops()[0].get_emitter(), "SimpleValue");
        assert_eq!(sequence.get_oops()[1].get_emitter(), "AnotherValue");
    }
}
