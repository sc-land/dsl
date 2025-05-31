pub mod literal;
pub mod specie;
pub mod tag;

pub use literal::Literal;
pub use specie::Specie;
pub use tag::Tag;

#[derive(Debug, Clone)]
pub enum Emitter {
    Specie(Specie),
    Tag(Tag),
    Literal(Literal),
}

impl Emitter {
    pub fn get_raw(&self) -> &str {
        match self {
            Emitter::Specie(specie) => specie.get_raw(),
            Emitter::Tag(tag) => tag.get_raw(),
            Emitter::Literal(literal) => literal.get_raw(),
        }
    }

    pub fn is_specie(&self) -> bool {
        matches!(self, Emitter::Specie(_))
    }

    pub fn is_tag(&self) -> bool {
        matches!(self, Emitter::Tag(_))
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Emitter::Literal(_))
    }

    pub fn as_literal(&self) -> Option<&Literal> {
        match self {
            Emitter::Literal(literal) => Some(literal),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emitter_specie() {
        let specie = Specie::new("Animal".to_string());
        let emitter = Emitter::Specie(specie);
        
        assert_eq!(emitter.get_raw(), "Animal");
        assert!(emitter.is_specie());
        assert!(!emitter.is_tag());
        assert!(!emitter.is_literal());
        assert!(emitter.as_literal().is_none());
    }

    #[test]
    fn test_emitter_tag() {
        let tag = Tag::new("method".to_string());
        let emitter = Emitter::Tag(tag);
        
        assert_eq!(emitter.get_raw(), "method");
        assert!(!emitter.is_specie());
        assert!(emitter.is_tag());
        assert!(!emitter.is_literal());
        assert!(emitter.as_literal().is_none());
    }

    #[test]
    fn test_emitter_literal() {
        let literal = Literal::Int { raw: "42".to_string() };
        let emitter = Emitter::Literal(literal);
        
        assert_eq!(emitter.get_raw(), "42");
        assert!(!emitter.is_specie());
        assert!(!emitter.is_tag());
        assert!(emitter.is_literal());
        
        let literal_ref = emitter.as_literal().unwrap();
        assert!(literal_ref.is_int());
    }
}
