use crate::common::TestHelper;
use pest::Parser;
use sc_dsl::dsl::parser::parser::{Rule, SCP};

#[test]
fn test_basic_parser() {
    let input = TestHelper::get_valid_input();
    let parsed = SCP::parse(Rule::sc, &input);
    assert!(parsed.is_ok(), "The parser should process valid input");
}

#[test]
fn test_invalid_input() {
    let input = TestHelper::get_invalid_input();
    let parsed = SCP::parse(Rule::sc, &input);
    assert!(parsed.is_err(), "The parser should fail with invalid input");
}
