use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::anatomy::bug::gene::specie::Specie;
use crate::ast::sc::fly::strand::genome::anatomy::bug::gene::primor::Primor;
use crate::ast::sc::fly::strand::genome::behavior::trace::forager::literal::Literal;
use crate::ast::sc::fly::strand::genome::behavior::trace::forager::self_ref::SelfRef;

pub mod literal;
pub mod self_ref;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Forager {
    Specie(Specie),
    Primor(Primor),
    Literal(Literal),
    SelfRef(SelfRef),
}
