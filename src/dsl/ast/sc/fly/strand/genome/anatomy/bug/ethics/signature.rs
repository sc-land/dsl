use serde::{Deserialize, Serialize};
use crate::dsl::ast::sc::fly::strand::genome::behavior::EthicsBind;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    pub binds: Option<Vec<EthicsBind>>,
}