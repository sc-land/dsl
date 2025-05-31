use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Bit { raw: String },
    Hex { raw: String },
    Int { raw: String },
    Str { raw: String },
    Decimal { raw: String },
}

impl Literal {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        // O pair pode ser diretamente uma das regras de literal ou uma regra literal intermediária
        let actual_pair = if pair.as_rule() == Rule::literal {
            pair.into_inner().next().expect("Literal deve ter uma regra interna")
        } else {
            pair
        };

        match actual_pair.as_rule() {
            Rule::bit => Literal::Bit { raw: actual_pair.as_str().to_string() },
            Rule::hex => Literal::Hex { raw: actual_pair.as_str().to_string() },
            Rule::int => Literal::Int { raw: actual_pair.as_str().to_string() },
            Rule::str => Literal::Str { raw: actual_pair.as_str().to_string() },
            Rule::decimal => Literal::Decimal { raw: actual_pair.as_str().to_string() },
            _ => panic!("Tipo de literal inesperado: {:?}", actual_pair.as_rule()),
        }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::literal, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Literal::from_pair(pair))
    }

    pub fn get_raw(&self) -> &str {
        match self {
            Literal::Bit { raw } => raw,
            Literal::Hex { raw } => raw,
            Literal::Int { raw } => raw,
            Literal::Str { raw } => raw,
            Literal::Decimal { raw } => raw,
        }
    }

    pub fn is_bit(&self) -> bool {
        matches!(self, Literal::Bit { .. })
    }

    pub fn is_hex(&self) -> bool {
        matches!(self, Literal::Hex { .. })
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Literal::Int { .. })
    }

    pub fn is_str(&self) -> bool {
        matches!(self, Literal::Str { .. })
    }

    pub fn is_decimal(&self) -> bool {
        matches!(self, Literal::Decimal { .. })
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Literal::Bit { .. } | Literal::Hex { .. } | Literal::Int { .. } | Literal::Decimal { .. })
    }

    /// Retorna o valor string sem as aspas (apenas para literais string)
    pub fn get_string_value(&self) -> Option<String> {
        match self {
            Literal::Str { raw } => {
                if raw.len() >= 2 && raw.starts_with('"') && raw.ends_with('"') {
                    Some(raw[1..raw.len()-1].to_string())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// Retorna o valor inteiro (para int, hex e bit)
    pub fn get_int_value(&self) -> Option<i64> {
        match self {
            Literal::Int { raw } => raw.parse().ok(),
            Literal::Hex { raw } => {
                if raw.starts_with("0x") || raw.starts_with("0X") {
                    i64::from_str_radix(&raw[2..], 16).ok()
                } else {
                    None
                }
            },
            Literal::Bit { raw } => {
                if raw.starts_with("0b") || raw.starts_with("0B") {
                    i64::from_str_radix(&raw[2..], 2).ok()
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// Retorna o valor decimal (para decimal)
    pub fn get_decimal_value(&self) -> Option<f64> {
        match self {
            Literal::Decimal { raw } => raw.parse().ok(),
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
    fn test_literal_bit() {
        let input = "0b1010".to_string();
        let literal = Literal::from_string(input).unwrap();
        assert_eq!(literal.get_raw(), "0b1010");
        assert!(literal.is_bit());
        assert!(literal.is_numeric());
        assert!(!literal.is_str());
    }

    #[test]
    fn test_literal_hex() {
        let input = "0xFF".to_string();
        let literal = Literal::from_string(input).unwrap();
        assert_eq!(literal.get_raw(), "0xFF");
        assert!(literal.is_hex());
        assert!(literal.is_numeric());
        assert!(!literal.is_str());
    }

    #[test]
    fn test_literal_int() {
        let input = "42".to_string();
        let literal = Literal::from_string(input).unwrap();
        assert_eq!(literal.get_raw(), "42");
        assert!(literal.is_int());
        assert!(literal.is_numeric());
        assert!(!literal.is_str());
    }

    #[test]
    fn test_literal_negative_int() {
        let input = "-123".to_string();
        let literal = Literal::from_string(input).unwrap();
        assert_eq!(literal.get_raw(), "-123");
        assert!(literal.is_int());
        assert!(literal.is_numeric());
    }

    #[test]
    fn test_literal_str() {
        let input = "\"hello world\"".to_string();
        let literal = Literal::from_string(input).unwrap();
        assert_eq!(literal.get_raw(), "\"hello world\"");
        assert!(literal.is_str());
        assert!(!literal.is_numeric());
        assert_eq!(literal.get_string_value(), Some("hello world".to_string()));
    }

    #[test]
    fn test_literal_decimal() {
        let input = "3.14".to_string();
        let literal = Literal::from_string(input).unwrap();
        assert_eq!(literal.get_raw(), "3.14");
        assert!(literal.is_decimal());
        assert!(literal.is_numeric());
        assert!(!literal.is_str());
    }

    #[test]
    fn test_literal_negative_decimal() {
        let input = "-2.5".to_string();
        let literal = Literal::from_string(input).unwrap();
        assert_eq!(literal.get_raw(), "-2.5");
        assert!(literal.is_decimal());
        assert!(literal.is_numeric());
    }

    #[test]
    fn test_literal_from_pair() {
        let input = "0x123".to_string();
        let parsed = SCP::parse(Rule::literal, &input).unwrap();
        let literal = Literal::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(literal.get_raw(), "0x123");
        assert!(literal.is_hex());
    }

    #[test]
    fn test_literal_equality() {
        let literal1 = Literal::Int { raw: "42".to_string() };
        let literal2 = Literal::Int { raw: "42".to_string() };
        let literal3 = Literal::Str { raw: "42".to_string() };

        assert_eq!(literal1, literal2);
        assert_ne!(literal1, literal3);
    }

    #[test]
    fn test_get_string_value() {
        let literal = Literal::Str { raw: "\"hello world\"".to_string() };
        assert_eq!(literal.get_string_value(), Some("hello world".to_string()));

        let literal_empty = Literal::Str { raw: "\"\"".to_string() };
        assert_eq!(literal_empty.get_string_value(), Some("".to_string()));

        let literal_int = Literal::Int { raw: "42".to_string() };
        assert_eq!(literal_int.get_string_value(), None);
    }

    #[test]
    fn test_get_int_value() {
        let literal_int = Literal::Int { raw: "42".to_string() };
        assert_eq!(literal_int.get_int_value(), Some(42));

        let literal_neg = Literal::Int { raw: "-123".to_string() };
        assert_eq!(literal_neg.get_int_value(), Some(-123));

        let literal_hex = Literal::Hex { raw: "0xFF".to_string() };
        assert_eq!(literal_hex.get_int_value(), Some(255));

        let literal_bit = Literal::Bit { raw: "0b1010".to_string() };
        assert_eq!(literal_bit.get_int_value(), Some(10));

        let literal_str = Literal::Str { raw: "\"42\"".to_string() };
        assert_eq!(literal_str.get_int_value(), None);
    }

    #[test]
    fn test_get_decimal_value() {
        let literal_decimal = Literal::Decimal { raw: "3.14".to_string() };
        assert_eq!(literal_decimal.get_decimal_value(), Some(3.14));

        let literal_neg_decimal = Literal::Decimal { raw: "-2.5".to_string() };
        assert_eq!(literal_neg_decimal.get_decimal_value(), Some(-2.5));

        let literal_int = Literal::Int { raw: "42".to_string() };
        assert_eq!(literal_int.get_decimal_value(), None);
    }
}
