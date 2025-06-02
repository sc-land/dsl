use std::fs;
use sc_dsl::dsl::parser::tree::Tree;

fn main() {
    // Read the input file
    let input = fs::read_to_string("tests/fixtures/fragments/program/anatomy.sc")
        .expect("Failed to read input.sc file");

    // Parse the input using the Tree parser
    match Tree::parse_input(input) {
        Ok(ast) => {
            println!("Successfully parsed AST:");
            println!("{:#?}", ast);
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
