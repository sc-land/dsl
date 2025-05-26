use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "dsl/sc.dsl"]
pub struct SCP;
