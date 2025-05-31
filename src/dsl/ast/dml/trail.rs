use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq)]
pub struct Catalysis {
    pub raw: String,
}

impl Catalysis {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::catalysis);
        let raw = pair.as_str().to_string();
        Catalysis { raw }
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Carrier {
    pub raw: String,
}

impl Carrier {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::carrier);
        let raw = pair.as_str().to_string();
        Carrier { raw }
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Trail {
    Catalysis(Catalysis),
    Carrier(Carrier),
}

impl Trail {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::trail);

        let inner_pair = pair.into_inner().next().expect("Trail deve ter uma regra interna");
        match inner_pair.as_rule() {
            Rule::catalysis => Trail::Catalysis(Catalysis::from_pair(inner_pair)),
            Rule::carrier => Trail::Carrier(Carrier::from_pair(inner_pair)),
            _ => panic!("Regra inesperada dentro de trail: {:?}", inner_pair.as_rule()),
        }
    }

    pub fn get_raw(&self) -> &str {
        match self {
            Trail::Catalysis(catalysis) => catalysis.get_raw(),
            Trail::Carrier(carrier) => carrier.get_raw(),
        }
    }

    pub fn is_catalysis(&self) -> bool {
        matches!(self, Trail::Catalysis(_))
    }

    pub fn is_carrier(&self) -> bool {
        matches!(self, Trail::Carrier(_))
    }

    pub fn as_catalysis(&self) -> Option<&Catalysis> {
        match self {
            Trail::Catalysis(catalysis) => Some(catalysis),
            _ => None,
        }
    }

    pub fn as_carrier(&self) -> Option<&Carrier> {
        match self {
            Trail::Carrier(carrier) => Some(carrier),
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
    fn test_catalysis_from_pair() {
        let input = ".method".to_string();
        let parsed = SCP::parse(Rule::catalysis, &input).unwrap();
        let catalysis = Catalysis::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(catalysis.get_raw(), ".method");
        assert_eq!(catalysis.raw, ".method");
    }

    #[test]
    fn test_carrier_from_pair() {
        let input = "(arg1, arg2)".to_string();
        let parsed = SCP::parse(Rule::carrier, &input).unwrap();
        let carrier = Carrier::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(carrier.get_raw(), "(arg1, arg2)");
        assert_eq!(carrier.raw, "(arg1, arg2)");
    }

    #[test]
    fn test_trail_catalysis() {
        let input = ".method".to_string();
        let parsed = SCP::parse(Rule::trail, &input).unwrap();
        let trail = Trail::from_pair(parsed.into_iter().next().unwrap());

        assert!(trail.is_catalysis());
        assert!(!trail.is_carrier());
        assert_eq!(trail.get_raw(), ".method");

        let catalysis = trail.as_catalysis().unwrap();
        assert_eq!(catalysis.get_raw(), ".method");
    }

    #[test]
    fn test_trail_carrier() {
        let input = "(arg1, arg2)".to_string();
        let parsed = SCP::parse(Rule::trail, &input).unwrap();
        let trail = Trail::from_pair(parsed.into_iter().next().unwrap());

        assert!(trail.is_carrier());
        assert!(!trail.is_catalysis());
        assert_eq!(trail.get_raw(), "(arg1, arg2)");

        let carrier = trail.as_carrier().unwrap();
        assert_eq!(carrier.get_raw(), "(arg1, arg2)");
    }

    #[test]
    fn test_complex_catalysis_with_carrier() {
        let input = ".method(arg1, arg2)".to_string();
        let parsed = SCP::parse(Rule::catalysis, &input).unwrap();
        let catalysis = Catalysis::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(catalysis.get_raw(), ".method(arg1, arg2)");
    }

    #[test]
    fn test_empty_carrier() {
        let input = "()".to_string();
        let parsed = SCP::parse(Rule::carrier, &input).unwrap();
        let carrier = Carrier::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(carrier.get_raw(), "()");
    }
}
