use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use super::binds::Binds;
use super::sequence::Sequence;

#[derive(Debug, Clone)]
pub enum Transport {
    Binds(Binds),
    Sequence(Sequence),
}

impl Transport {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::transport);

        let inner_pair = pair.into_inner().next().expect("Transport deve ter uma regra interna");
        match inner_pair.as_rule() {
            Rule::binds => Transport::Binds(Binds::from_pair(inner_pair)),
            Rule::sequence => Transport::Sequence(Sequence::from_pair(inner_pair)),
            _ => panic!("Regra inesperada dentro de transport: {:?}", inner_pair.as_rule()),
        }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::transport, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Transport::from_pair(pair))
    }

    pub fn get_raw(&self) -> &str {
        match self {
            Transport::Binds(binds) => binds.get_raw(),
            Transport::Sequence(sequence) => sequence.get_raw(),
        }
    }

    pub fn is_binds(&self) -> bool {
        matches!(self, Transport::Binds(_))
    }

    pub fn is_sequence(&self) -> bool {
        matches!(self, Transport::Sequence(_))
    }

    pub fn as_binds(&self) -> Option<&Binds> {
        match self {
            Transport::Binds(binds) => Some(binds),
            _ => None,
        }
    }

    pub fn as_sequence(&self) -> Option<&Sequence> {
        match self {
            Transport::Sequence(sequence) => Some(sequence),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::dsl::parser::parser::{Rule, SCP};
    use super::*;

    #[test]
    fn test_transport_binds() {
        let input = "name: Value, count: 42".to_string();
        let parsed = SCP::parse(Rule::transport, &input).unwrap();
        let transport = Transport::from_pair(parsed.into_iter().next().unwrap());

        assert!(transport.is_binds());
        assert!(!transport.is_sequence());

        let binds = transport.as_binds().unwrap();
        assert_eq!(binds.len(), 2);
        assert_eq!(binds.get_binds()[0].get_tag(), "name");
        assert_eq!(binds.get_binds()[1].get_tag(), "count");
    }

    #[test]
    fn test_transport_sequence() {
        let input = "Value, 42, Class.method".to_string();
        let parsed = SCP::parse(Rule::transport, &input).unwrap();
        let transport = Transport::from_pair(parsed.into_iter().next().unwrap());

        assert!(transport.is_sequence());
        assert!(!transport.is_binds());

        let sequence = transport.as_sequence().unwrap();
        assert_eq!(sequence.len(), 3);
        assert_eq!(sequence.get_oops()[0].get_emitter(), "Value");
        assert_eq!(sequence.get_oops()[1].get_emitter(), "42");
        assert_eq!(sequence.get_oops()[2].get_emitter(), "Class");
    }

    #[test]
    fn test_transport_from_string_binds() {
        let input = "x: 10, y: Point.new".to_string();
        let transport = Transport::from_string(input).unwrap();
        assert!(transport.is_binds());

        let binds = transport.as_binds().unwrap();
        assert_eq!(binds.len(), 2);
    }

    #[test]
    fn test_transport_from_string_sequence() {
        let input = "Point.x, Point.y".to_string();
        let transport = Transport::from_string(input).unwrap();
        assert!(transport.is_sequence());

        let sequence = transport.as_sequence().unwrap();
        assert_eq!(sequence.len(), 2);
    }

    #[test]
    fn test_transport_getters() {
        let input = "name: Value".to_string();
        let transport = Transport::from_string(input).unwrap();
        assert!(transport.is_binds());
        assert_eq!(transport.get_raw(), "name: Value");
    }
}
