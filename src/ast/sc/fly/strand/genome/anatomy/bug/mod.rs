pub mod gene;
pub mod ethics;

use pest::{iterators::Pair, Parser};
use serde::{Deserialize, Serialize};
use crate::parser::parser::{Rule, SCP};
use crate::ast::sc::fly::strand::genome::anatomy::bug::ethics::Ethics;
use crate::ast::sc::fly::strand::genome::anatomy::bug::gene::Gene;
use crate::ast::sc::fly::strand::genome::anatomy::bug::gene::specie::Specie;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bug {
    pub specie: Specie,
    pub genes: Vec<Gene>,
    pub ethics: Vec<Ethics>,
}

impl Bug {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::bug);

        let mut specie = None;
        let mut genes = Vec::new();
        let mut ethics = Vec::new();

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::specie => {
                    specie = Some(Specie::from_pair(inner));
                }
                Rule::gene => genes.push(Gene::from_pair(inner)),
                Rule::ethics => ethics.push(Ethics::from_pair(inner)),
                _ => todo!()
            }
        }

        // Make sure specie is initialized, otherwise panic with a helpful message
        let specie = specie.expect("Bug deve ter uma espécie");
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
    use crate::parser::parser::SCP;

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
        assert_eq!(bug.specie.raw, "TestBug", "Bug species should be TestBug");

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

        assert_eq!(bug.specie.raw, "SimpleBug", "Bug species should be SimpleBug");
        assert!(!bug.genes.is_empty(), "Bug should have genes");
        assert_eq!(bug.genes.len(), 1, "Bug should have exactly 1 gene");
        assert!(bug.ethics.is_empty(), "Bug should not have ethics");
        assert_eq!(bug.ethics.len(), 0, "Bug should have 0 ethics");
    }

    #[test]
    fn test_bug_detailed_content() {
        // Carrega o fixture completo
        let path = "tests/fixtures/fragments/bug/complete.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read complete.sc file");

        let bug = Bug::from_string(input);

        // Verifica os metadados do bug
        assert_eq!(bug.specie.raw, "TestBug", "Bug species should be TestBug");

        // Verifica os genes
        assert_eq!(bug.genes.len(), 1, "Bug should have 1 gene");
        assert_eq!(bug.genes[0].primor.raw, "x", "Gene should be named 'x'");
        assert_eq!(bug.genes[0].specie.raw, "Int", "Gene 'x' should have type Int");

        // Verifica os métodos de ethics
        assert_eq!(bug.ethics.len(), 4, "Bug should have 4 ethics methods");

        // Encontra os ethics pelo nome
        let simple_method = &bug.ethics.iter().find(|e| e.primor.raw == "simple_method").expect("simple_method not found");
        let test_method = &bug.ethics.iter().find(|e| e.primor.raw == "test_method").expect("test_method not found");
        let test_method2 = &bug.ethics.iter().find(|e| e.primor.raw == "test_method2").expect("test_method2 not found");
        let test_method3 = &bug.ethics.iter().find(|e| e.primor.raw == "test_method3").expect("test_method3 not found");

        // Verifica simple_method
        assert_eq!(simple_method.primor.raw, "simple_method", "First ethics should be named 'simple_method'");
        assert!(simple_method.signature.is_none(), "simple_method should have no signature");
        assert!(simple_method.body.is_none(), "simple_method should not have a body");

        // Verifica test_method
        assert_eq!(test_method.primor.raw, "test_method", "Ethics should be named 'test_method'");
        assert!(test_method.body.is_some(), "test_method should have a body");

        if let Some(body) = &test_method.body {
            assert!(!body.signals.is_empty(), "test_method body should have signals");
            // Verifique que o corpo contém instruções para atribuir x = 42
        }

        // Verifica test_method2
        assert_eq!(test_method2.primor.raw, "test_method2", "Ethics should be named 'test_method2'");
        assert!(test_method2.signature.is_some(), "test_method2 should have a signature");
        assert!(test_method2.body.is_some(), "test_method2 should have a body");


        // Verifica test_method3 com parâmetros e feedback
        assert_eq!(test_method3.primor.raw, "test_method3", "Ethics should be named 'test_method3'");
        assert!(test_method3.signature.is_some(), "test_method3 should have a signature");
        assert!(test_method3.feedback.is_some(), "test_method3 should have a feedback type");

        if let Some(feedback) = &test_method3.feedback {
            assert_eq!(feedback.raw, "Int", "test_method3 should return Int");
        }

    }

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
    fn test_bug_low_level_parse() {
        // Testa parse de baixo nível usando SCP::parse diretamente
        use crate::parser::parser::Rule;
        use pest::Parser;

        let input = "bug CloneBug\n  gene z Bool\nend".to_string();

        // Parse diretamente com a regra bug
        let pairs = SCP::parse(Rule::bug, &input)
            .expect("Failed to parse bug");

        // Obtém o primeiro par do resultado
        let pair = pairs.peek().expect("No pairs found");

        // Cria o Bug a partir do par
        let bug = Bug::from_pair(pair);

        // Verifica o conteúdo básico
        assert_eq!(bug.specie.raw, "CloneBug", "Bug species should be CloneBug");
        assert_eq!(bug.genes.len(), 1, "Bug should have 1 gene");
        assert_eq!(bug.ethics.len(), 0, "Bug should have 0 ethics");
    }

    #[test]
    fn test_bug_direct_access() {
        // Testa acesso direto aos atributos
        let input = "bug DirectBug\n  gene a Int\n  gene b String\nend".to_string();

        let bug = Bug::from_string(input);

        // Testa acesso direto aos atributos
        assert_eq!(bug.specie.raw, "DirectBug");
        assert_eq!(bug.genes.len(), 2);
        assert_eq!(bug.ethics.len(), 0);
    }

    #[test]
    fn test_bug_empty_collections() {
        // Testa bug sem genes e ethics
        let input = "bug EmptyBug\nend".to_string();

        let bug = Bug::from_string(input);

        assert_eq!(bug.specie.raw, "EmptyBug");
        assert!(bug.genes.is_empty(), "Should have no genes");
        assert!(bug.ethics.is_empty(), "Should have no ethics");
        assert_eq!(bug.genes.len(), 0);
        assert_eq!(bug.ethics.len(), 0);
    }
}

