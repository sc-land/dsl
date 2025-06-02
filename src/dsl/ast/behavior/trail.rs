use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::emitter::Tag;
use super::transport::Transport;

#[derive(Debug, Clone, PartialEq)]
pub struct Catalysis {
    pub tag: Tag,
    pub carrier: Option<Carrier>,
}

impl Catalysis {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::catalysis);

        let mut inner = pair.into_inner();

        // Parse tag (skip the "." token)
        let tag_pair = inner.next().expect("Catalysis deve ter uma tag");
        let tag = Tag::from_pair(tag_pair);

        // Parse optional carrier
        let carrier = inner.next().map(|carrier_pair| Carrier::from_pair(carrier_pair));

        Catalysis { tag, carrier }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Carrier {
    pub transport: Option<Transport>,
}

impl Carrier {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::carrier);

        // Parse optional transport
        let transport = pair.into_inner().next().map(|transport_pair| Transport::from_pair(transport_pair));

        Carrier { transport }
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

        let inner_pair = pair.into_inner().next().expect("Trail must have an inner rule");
        match inner_pair.as_rule() {
            Rule::catalysis => Trail::Catalysis(Catalysis::from_pair(inner_pair)),
            Rule::carrier => Trail::Carrier(Carrier::from_pair(inner_pair)),
            _ => panic!("Unexpected rule inside trail: {:?}", inner_pair.as_rule()),
        }
    }
}
