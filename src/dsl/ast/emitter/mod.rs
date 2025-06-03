pub mod literal;
pub mod specie;
pub mod tag;
pub mod self_ref;

pub use literal::Literal;
pub use specie::Specie;
pub use tag::Tag;
pub use self_ref::SelfRef;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Emitter {
    Specie(Specie),
    Tag(Tag),
    Literal(Literal),
    SelfRef(SelfRef),
}

impl Emitter {
    pub fn get_raw(&self) -> &str {
        match self {
            Emitter::Specie(specie) => specie.get_raw(),
            Emitter::Tag(tag) => tag.get_raw(),
            Emitter::Literal(literal) => literal.get_raw(),
            Emitter::SelfRef(self_ref) => self_ref.get_raw(),
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

    pub fn is_self_ref(&self) -> bool {
        matches!(self, Emitter::SelfRef(_))
    }

    pub fn as_literal(&self) -> Option<&Literal> {
        match self {
            Emitter::Literal(literal) => Some(literal),
            _ => None,
        }
    }

    pub fn as_self_ref(&self) -> Option<&SelfRef> {
        match self {
            Emitter::SelfRef(self_ref) => Some(self_ref),
            _ => None,
        }
    }
}
