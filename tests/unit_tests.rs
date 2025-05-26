mod common;

use pest::Parser;
use sc_dsl::dsl::parser::parser::{Rule, SCP};
use sc_dsl::dsl::parser::tree::Tree;
use crate::common::TestHelper;

#[test]
fn it_works() {
    let input = TestHelper::get_valid_input();
    let parsed = SCP::parse(Rule::sc, &input);
    assert!(parsed.is_ok(), "The parser should successfully parse alphabetic input");
}

#[test]
fn does_not_work() {
    let input = TestHelper::get_invalid_input();
    let parsed = SCP::parse(Rule::sc, &input);
    assert!(parsed.is_err(), "The parser should fail when given a number");
}

#[test]
fn parse_tree_works() {
    let input = TestHelper::get_valid_input();
    let parsed = Tree::parse_sc(input);
    assert!(parsed.is_ok(), "?");
}

#[test]
fn parse_tree_does_not_works() {
    let input = TestHelper::get_invalid_input();
    let parsed = Tree::parse_sc(input);
    assert!(parsed.is_err(), "The parser should fail when given a number");
}
