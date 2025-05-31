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

#[test]
fn test_get_sc() {
    let input = "fly";
    let tree = Tree::parse_input(input.to_string()).unwrap();

    // Testa se conseguimos acessar o SC através do método get_sc()
    let sc = tree.get_sc();
    assert!(!sc.fly.raw.is_empty(), "SC deve ter um fly válido com conteúdo");

    // Testa se o acesso direto e via método retornam a mesma referência
    assert_eq!(std::ptr::eq(&tree.sc, tree.get_sc()), true, "get_sc() deve retornar referência para o mesmo objeto");
}
