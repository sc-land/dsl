use crate::common::TestHelper;
use sc_dsl::dsl::parser::tree::Tree;

#[test]
fn test_tree_construction() {
    let input = TestHelper::get_valid_input();
    let result = Tree::parse_sc(input);
    assert!(result.is_ok(), "A árvore deve ser construída com sucesso");
}

#[test]
fn test_tree_invalid_input() {
    let input = TestHelper::get_invalid_input();
    let result = Tree::parse_sc(input);
    assert!(result.is_err(), "A árvore deve falhar com entrada inválida");
}
