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
