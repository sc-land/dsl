pub mod literal;
pub mod specie;
pub mod tag;
pub mod self_ref;

use serde::{Deserialize, Serialize};
pub use literal::Literal;
pub use specie::Specie;
pub use tag::Tag;
pub use self_ref::SelfRef;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Emitter {
    Specie(Specie),
    Tag(Tag),
    Literal(Literal),
    SelfRef(SelfRef),
}
