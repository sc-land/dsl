use pest::{iterators::Pair, Parser};
use crate::dsl::parser::parser::{Rule, SCP};
use crate::dsl::ast::gene::Gene;
use super::ethics::Ethics;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bug {
    pub specie: String,
    pub genes: Vec<Gene>,
    pub ethics: Vec<Ethics>,
}

impl Bug {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::bug);

        let mut inner = pair.into_inner();
        let specie = inner.next()
            .expect("Bug deve ter uma espécie")
            .as_str()
            .to_string();

        let mut genes = Vec::new();
        let mut ethics = Vec::new();
        for inner_pair in inner {
            match inner_pair.as_rule() {
                Rule::gene => genes.push(Gene::from_pair(inner_pair)),
                Rule::ethics => ethics.push(Ethics::from_pair(inner_pair)),
                _ => {}
            }
        }

        Bug { specie, genes, ethics }
    }

    pub fn from_string(input: String) -> Self {
        let pair = SCP::parse(Rule::bug, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");
        Bug::from_pair(pair)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::Bug;

    #[test]
    fn test_bug_from_string() {
        // Carrega o fixture de bug completo
        let path = "tests/fixtures/fragments/bug/complete.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read complete.sc file");

        // Verifica que o arquivo não está vazio
        assert!(!input.is_empty(), "Fixture file should not be empty");

        // Testa o parse usando from_string
        let bug = Bug::from_string(input);

        // Verifica o nome da espécie
        assert_eq!(bug.specie, "TestBug", "Bug species should be TestBug");

        // Verifica que tem o conteúdo esperado
        // assert!(bug.raw.contains("TestBug"), "Bug should contain species TestBug");
        // assert!(bug.raw.contains("gene x Int"), "Bug should contain gene declaration");
        // assert!(bug.raw.contains("ethics test_method"), "Bug should contain ethics declaration");
        // precisa ser verificado de modo adequado
    }

    #[test]
    fn test_bug_structure() {
        // Carrega o fixture de bug completo
        let path = "tests/fixtures/fragments/bug/complete.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read complete.sc file");

        // Testa o parse
        let bug = Bug::from_string(input);

        // Verifica genes usando atributos diretos
        assert!(!bug.genes.is_empty(), "Bug should have genes");
        assert_eq!(bug.genes.len(), 1, "Bug should have exactly 1 gene");

        // Verifica ethics usando atributos diretos
        assert!(!bug.ethics.is_empty(), "Bug should have ethics");
        assert_eq!(bug.ethics.len(), 4, "Bug should have exactly 4 ethics");
    }

    #[test]
    fn test_bug_from_pair() {
        // Este teste usa from_string que internamente usa from_pair
        let input = "bug SimpleBug\n  gene y String\nend".to_string();

        let bug = Bug::from_string(input);

        assert_eq!(bug.specie, "SimpleBug", "Bug species should be SimpleBug");
        assert!(!bug.genes.is_empty(), "Bug should have genes");
        assert_eq!(bug.genes.len(), 1, "Bug should have exactly 1 gene");
        assert!(bug.ethics.is_empty(), "Bug should not have ethics");
        assert_eq!(bug.ethics.len(), 0, "Bug should have 0 ethics");
    }

    // #[test]
    // fn test_bug_raw_content() {
    //     // Carrega o fixture completo
    //     let path = "tests/fixtures/fragments/bug/complete.sc".to_string();
    //     let input = fs::read_to_string(path)
    //         .expect("Failed to read complete.sc file");

    //     let bug = Bug::from_string(input.clone());

    //     // Verifica que o raw contém os elementos esperados
    //     assert!(bug.raw.contains("bug TestBug"), "Raw should contain bug declaration");
    //     assert!(bug.raw.contains("gene x Int"), "Raw should contain gene x Int");
    //     assert!(bug.raw.contains("ethics test_method"), "Raw should contain ethics test_method");
    //     assert!(bug.raw.contains("ethics test_method2()"), "Raw should contain ethics test_method2()");
    //     assert!(bug.raw.contains("ethics test_method3(a: Int) Int"), "Raw should contain ethics test_method3 with parameters");
    //     assert!(bug.raw.contains("x = 42"), "Raw should contain assignment x = 42");
    //     assert!(bug.raw.contains("return x"), "Raw should contain return statement");
    // }
    //precisa ser testado de forma adequada

    #[test]
    fn test_bug_clone() {
        // Testa se a clonagem funciona corretamente
        let input = "bug CloneBug\n  gene z Bool\nend".to_string();

        let bug = Bug::from_string(input);
        let cloned_bug = bug.clone();

        // Verifica que ambos são iguais
        assert_eq!(bug.specie, cloned_bug.specie, "Species should be equal");
        assert_eq!(bug.genes.len(), cloned_bug.genes.len(), "Gene count should be equal");
        assert_eq!(bug.ethics.len(), cloned_bug.ethics.len(), "Ethics count should be equal");
    }

    #[test]
    fn test_bug_direct_access() {
        // Testa acesso direto aos atributos
        let input = "bug DirectBug\n  gene a Int\n  gene b String\nend".to_string();

        let bug = Bug::from_string(input);

        // Testa acesso direto aos atributos
        assert_eq!(bug.specie, "DirectBug");
        assert_eq!(bug.genes.len(), 2);
        assert_eq!(bug.ethics.len(), 0);
    }

    #[test]
    fn test_bug_empty_collections() {
        // Testa bug sem genes e ethics
        let input = "bug EmptyBug\nend".to_string();

        let bug = Bug::from_string(input);

        assert_eq!(bug.specie, "EmptyBug");
        assert!(bug.genes.is_empty(), "Should have no genes");
        assert!(bug.ethics.is_empty(), "Should have no ethics");
        assert_eq!(bug.genes.len(), 0);
        assert_eq!(bug.ethics.len(), 0);
    }
}

