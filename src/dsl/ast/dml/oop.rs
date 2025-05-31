use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::emitter::{Emitter, Literal, Specie, Tag};
use super::trail::Trail;

#[derive(Debug, Clone)]
pub struct Oop {
    pub raw: String,
    pub emitter: Emitter,
    pub trails: Vec<Trail>,
}

impl Oop {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::oop);
        let raw = pair.as_str().to_string();

        let mut inner = pair.into_inner();

        // Parse emitter (specie | tag | literal)
        let emitter_pair = inner.next().expect("Oop deve ter um emitter");
        let emitter = if emitter_pair.as_rule() == Rule::emitter {
            // Se é uma regra emitter, pega a regra interna
            let inner_emitter = emitter_pair.into_inner().next().expect("Emitter deve ter uma regra interna");
            match inner_emitter.as_rule() {
                Rule::specie => Emitter::Specie(Specie { raw: inner_emitter.as_str().to_string() }),
                Rule::tag => Emitter::Tag(Tag { raw: inner_emitter.as_str().to_string() }),
                Rule::bit | Rule::hex | Rule::decimal | Rule::int | Rule::str => {
                    Emitter::Literal(Literal::from_pair(inner_emitter))
                },
                Rule::literal => {
                    Emitter::Literal(Literal::from_pair(inner_emitter))
                },
                _ => panic!("Tipo de emitter inesperado: {:?}", inner_emitter.as_rule()),
            }
        } else {
            // Se não é uma regra emitter, trata diretamente
            match emitter_pair.as_rule() {
                Rule::specie => Emitter::Specie(Specie { raw: emitter_pair.as_str().to_string() }),
                Rule::tag => Emitter::Tag(Tag { raw: emitter_pair.as_str().to_string() }),
                Rule::bit | Rule::hex | Rule::decimal | Rule::int | Rule::str => {
                    Emitter::Literal(Literal::from_pair(emitter_pair))
                },
                Rule::literal => {
                    Emitter::Literal(Literal::from_pair(emitter_pair))
                },
                _ => panic!("Tipo de emitter inesperado: {:?}", emitter_pair.as_rule()),
            }
        };

        // Parse trails (catalysis | carrier)*
        let mut trails = Vec::new();
        for trail_pair in inner {
            trails.push(Trail::from_pair(trail_pair));
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
        self.emitter.get_raw()
    }

    pub fn get_emitter_enum(&self) -> &Emitter {
        &self.emitter
    }

    pub fn get_trails(&self) -> &[Trail] {
        &self.trails
    }

    pub fn has_trails(&self) -> bool {
        !self.trails.is_empty()
    }

    pub fn emitter_is_specie(&self) -> bool {
        self.emitter.is_specie()
    }

    pub fn emitter_is_tag(&self) -> bool {
        self.emitter.is_tag()
    }

    pub fn emitter_is_literal(&self) -> bool {
        self.emitter.is_literal()
    }

    pub fn get_emitter_literal(&self) -> Option<&Literal> {
        self.emitter.as_literal()
    }

    // Trail-specific helper methods
    pub fn get_catalysis_trails(&self) -> Vec<&super::trail::Catalysis> {
        self.trails.iter()
            .filter_map(|trail| trail.as_catalysis())
            .collect()
    }

    pub fn get_carrier_trails(&self) -> Vec<&super::trail::Carrier> {
        self.trails.iter()
            .filter_map(|trail| trail.as_carrier())
            .collect()
    }

    pub fn has_catalysis(&self) -> bool {
        self.trails.iter().any(|trail| trail.is_catalysis())
    }

    pub fn has_carrier(&self) -> bool {
        self.trails.iter().any(|trail| trail.is_carrier())
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
        assert_eq!(oop.get_emitter(), "bug");
        assert!(oop.emitter_is_tag());
        assert_eq!(oop.trails.len(), 1);
        assert_eq!(oop.trails[0].get_raw(), ".happens");
        assert!(oop.trails[0].is_catalysis());
    }

    #[test]
    fn test_oop_simple_emitter() {
        let input = "Happens".to_string();
        let parsed = SCP::parse(Rule::oop, &input).unwrap();
        let oop = Oop::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(oop.raw, "Happens");
        assert_eq!(oop.get_emitter(), "Happens");
        assert!(oop.emitter_is_specie());
        assert_eq!(oop.trails.len(), 0);
    }

    #[test]
    fn test_oop_from_string() {
        let input = "Class.method".to_string();
        let oop = Oop::from_string(input).unwrap();
        assert_eq!(oop.get_emitter(), "Class");
        assert!(oop.has_trails());
        assert_eq!(oop.get_trails().len(), 1);
        assert_eq!(oop.get_trails()[0].get_raw(), ".method");
        assert!(oop.get_trails()[0].is_catalysis());
    }

    #[test]
    fn test_oop_getters() {
        let input = "Simple".to_string();
        let oop = Oop::from_string(input).unwrap();
        assert_eq!(oop.get_emitter(), "Simple");
        assert!(!oop.has_trails());
        assert_eq!(oop.get_trails().len(), 0);
    }

    #[test]
    fn test_literal_as_emitter() {
        let input = "\"0xFF\"".to_string();
        let oop = Oop::from_string(input).unwrap();
        assert_eq!(oop.get_emitter(), "\"0xFF\"");
        assert!(oop.emitter_is_literal());
        assert!(!oop.has_trails());
        assert_eq!(oop.get_trails().len(), 0);

        // Teste específico para literal
        let literal = oop.get_emitter_literal().unwrap();
        assert!(literal.is_str());
        assert!(!literal.is_numeric());
    }

    #[test]
    fn test_hex_literal_as_emitter() {
        let input = "0xFF".to_string();
        let oop = Oop::from_string(input).unwrap();
        assert_eq!(oop.get_emitter(), "0xFF");
        assert!(oop.emitter_is_literal());

        let literal = oop.get_emitter_literal().unwrap();
        assert!(literal.is_hex());
        assert!(literal.is_numeric());
    }

    #[test]
    fn test_int_literal_as_emitter() {
        let input = "42".to_string();
        let oop = Oop::from_string(input).unwrap();
        assert_eq!(oop.get_emitter(), "42");
        assert!(oop.emitter_is_literal());

        let literal = oop.get_emitter_literal().unwrap();
        assert!(literal.is_int());
        assert!(literal.is_numeric());
    }

    #[test]
    fn test_comprehensive_oop_with_literals() {
        // Teste com diferentes tipos de emitters literais
        let test_cases = vec![
            ("42", "int"),
            ("0xFF", "hex"),
            ("0b1010", "bit"),
            ("3.14", "decimal"),
            ("\"hello\"", "str"),
            ("Variable", "specie"),
            ("method", "tag"),
        ];

        for (input, expected_type) in test_cases {
            let oop = Oop::from_string(input.to_string()).unwrap();
            println!("Testing input: {} -> expected: {}", input, expected_type);

            match expected_type {
                "int" => {
                    assert!(oop.emitter_is_literal());
                    let literal = oop.get_emitter_literal().unwrap();
                    assert!(literal.is_int());
                    assert_eq!(literal.get_int_value(), Some(42));
                },
                "hex" => {
                    assert!(oop.emitter_is_literal());
                    let literal = oop.get_emitter_literal().unwrap();
                    assert!(literal.is_hex());
                    assert_eq!(literal.get_int_value(), Some(255));
                },
                "bit" => {
                    assert!(oop.emitter_is_literal());
                    let literal = oop.get_emitter_literal().unwrap();
                    assert!(literal.is_bit());
                    assert_eq!(literal.get_int_value(), Some(10));
                },
                "decimal" => {
                    assert!(oop.emitter_is_literal());
                    let literal = oop.get_emitter_literal().unwrap();
                    assert!(literal.is_decimal());
                    assert_eq!(literal.get_decimal_value(), Some(3.14));
                },
                "str" => {
                    assert!(oop.emitter_is_literal());
                    let literal = oop.get_emitter_literal().unwrap();
                    assert!(literal.is_str());
                    assert_eq!(literal.get_string_value(), Some("hello".to_string()));
                },
                "specie" => {
                    assert!(oop.emitter_is_specie());
                    assert!(!oop.emitter_is_literal());
                    assert_eq!(oop.get_emitter(), "Variable");
                },
                "tag" => {
                    assert!(oop.emitter_is_tag());
                    assert!(!oop.emitter_is_literal());
                    assert_eq!(oop.get_emitter(), "method");
                },
                _ => panic!("Unexpected test case type: {}", expected_type),
            }
        }
    }

    #[test]
    fn test_trail_helper_methods() {
        let input = "Object.method().call".to_string();
        let oop = Oop::from_string(input).unwrap();

        assert_eq!(oop.get_emitter(), "Object");
        assert_eq!(oop.trails.len(), 2);

        // Test catalysis methods
        assert!(oop.has_catalysis());
        let catalysis_trails = oop.get_catalysis_trails();
        assert_eq!(catalysis_trails.len(), 2);
        assert_eq!(catalysis_trails[0].get_raw(), ".method()");
        assert_eq!(catalysis_trails[1].get_raw(), ".call");

        // Test carrier methods - this input doesn't have standalone carriers
        assert!(!oop.has_carrier());
        let carrier_trails = oop.get_carrier_trails();
        assert_eq!(carrier_trails.len(), 0);
    }

    #[test]
    fn test_mixed_trails() {
        let input = "Object.method(arg1, arg2)".to_string();
        let oop = Oop::from_string(input).unwrap();

        assert_eq!(oop.get_emitter(), "Object");
        assert_eq!(oop.trails.len(), 1);

        // Should have catalysis (which includes the carrier)
        assert!(oop.has_catalysis());
        assert!(!oop.has_carrier()); // No standalone carrier

        let catalysis_trails = oop.get_catalysis_trails();
        assert_eq!(catalysis_trails.len(), 1);
        assert_eq!(catalysis_trails[0].get_raw(), ".method(arg1, arg2)");
    }
}
