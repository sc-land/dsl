use pest::iterators::Pair;
use crate::dsl::ast::behavior::binds::Binds;
use crate::dsl::ast::behavior::sequence::Sequence;
use crate::dsl::parser::parser::Rule;


#[derive(Debug, Clone, PartialEq, Eq)]
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
