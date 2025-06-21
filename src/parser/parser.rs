use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sc.dsl"]
pub struct SCP;
