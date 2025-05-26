mod common;
mod tree;
mod parser;

use common::TestHelper;
use sc_dsl::dsl::parser::tree::Tree;
use sc_dsl::dsl::parser::parser::{Rule, SCP};
use pest::Parser;



#[test]
fn test_full_parser_pipeline() {
    let test_cases = TestHelper::get_test_cases();

    for (input, should_pass) in test_cases {
        let parser_result = SCP::parse(Rule::sc, &input);
        let tree_result = Tree::parse_sc(input.clone());

        if should_pass {
            assert!(parser_result.is_ok(), "Parser failed for valid input");
            assert!(tree_result.is_ok(), "Tree failed for valid input");
        } else {
            assert!(parser_result.is_err(), "Parser should not accept invalid input");
            assert!(tree_result.is_err(), "Tree should not accept invalid input");
        }
    }
}
