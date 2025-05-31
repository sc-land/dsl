use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use super::bind::Bind;

#[derive(Debug, Clone)]
pub struct Binds {
    pub raw: String,
    pub binds: Vec<Bind>,
}

impl Binds {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::binds);
        let raw = pair.as_str().to_string();

        let mut binds = Vec::new();
        for bind_pair in pair.into_inner() {
            if bind_pair.as_rule() == Rule::bind {
                binds.push(Bind::from_pair(bind_pair));
            }
        }

        Binds { raw, binds }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::binds, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Binds::from_pair(pair))
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }

    pub fn get_binds(&self) -> &[Bind] {
        &self.binds
    }

    pub fn len(&self) -> usize {
        self.binds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.binds.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::dsl::parser::parser::{Rule, SCP};
    use super::*;

    #[test]
    fn test_binds_single() {
        let input = "name: Value".to_string();
        let parsed = SCP::parse(Rule::binds, &input).unwrap();
        let binds = Binds::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(binds.raw, "name: Value");
        assert_eq!(binds.len(), 1);
        assert_eq!(binds.get_binds()[0].get_tag(), "name");
    }

    #[test]
    fn test_binds_multiple() {
        let input = "name: Value, count: 42, method: Class.call".to_string();
        let binds = Binds::from_string(input).unwrap();
        assert_eq!(binds.len(), 3);

        assert_eq!(binds.get_binds()[0].get_tag(), "name");
        assert_eq!(binds.get_binds()[0].get_oop().get_emitter(), "Value");

        assert_eq!(binds.get_binds()[1].get_tag(), "count");
        assert_eq!(binds.get_binds()[1].get_oop().get_emitter(), "42");

        assert_eq!(binds.get_binds()[2].get_tag(), "method");
        assert_eq!(binds.get_binds()[2].get_oop().get_emitter(), "Class");
    }

    #[test]
    fn test_binds_from_string() {
        let input = "x: 10, y: Point.new".to_string();
        let binds = Binds::from_string(input).unwrap();
        assert!(!binds.is_empty());
        assert_eq!(binds.len(), 2);
        assert_eq!(binds.get_binds()[0].get_tag(), "x");
        assert_eq!(binds.get_binds()[1].get_tag(), "y");
    }

    #[test]
    fn test_binds_getters() {
        let input = "param: SimpleValue".to_string();
        let binds = Binds::from_string(input).unwrap();
        assert_eq!(binds.len(), 1);
        assert!(!binds.is_empty());
        let bind = &binds.get_binds()[0];
        assert_eq!(bind.get_tag(), "param");
        assert_eq!(bind.get_oop().get_emitter(), "SimpleValue");
    }
}
