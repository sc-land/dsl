use crate::common::TestHelper;
use sc_dsl::dsl::parser::tree::Tree;

#[test]
fn test_tree_construction() {
    let input = TestHelper::get_valid_input();
    let result = Tree::parse_input(input);
    assert!(result.is_ok(), "A árvore deve ser construída com sucesso");
}

#[test]
fn test_tree_invalid_input() {
    let input = TestHelper::get_invalid_input();
    let result = Tree::parse_input(input);
    assert!(result.is_err(), "A árvore deve falhar com entrada inválida");
}

#[test]
fn test_parse_sc() {
    let input = "fly";
    let tree = Tree::parse_input(input.to_string()).unwrap();
    println!("{:#?}", tree);
    // assert_eq!(tree.sc.fly.pog[0].genome[0].raw, "f", "Primeiro bios deve ser 'f'");
    // assert_eq!(tree.sc.fly.pog[0].genome[1].raw, "l", "Primeiro bios deve ser 'f'");
    
}
