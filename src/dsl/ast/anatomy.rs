use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::bug::Bug;
use crate::dsl::parser::parser::{Rule, SCP};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Anatomy {
    Bug(Bug),
}

impl Anatomy {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let inner = pair.into_inner();
        let bug = Bug::from_pair(inner.clone().next().unwrap());
        Anatomy::Bug(bug)
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut pairs = SCP::parse(Rule::anatomy, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Anatomy::from_pair(pair))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::dsl::ast::anatomy::Anatomy;


    #[test]
    fn test_anatomy_from_string() {
        // Carrega o arquivo de fixture usando o helper
        let path = "tests/fixtures/fragments/bug/complete.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read complete.sc file");

        // Verifica que o arquivo não está vazio
        assert!(!input.is_empty(), "Fixture file should not be empty");

        // Testa o parse
        let anatomy = Anatomy::from_string(input.clone())
            .expect("Failed to parse anatomy");

        // Verifica se temos um bug válido
        match anatomy {
            Anatomy::Bug(bug) => {
                assert_eq!(bug.specie.raw, "TestBug", "Bug species should be TestBug");
                assert!(!bug.genes.is_empty(), "Bug should have genes");
                assert!(!bug.ethics.is_empty(), "Bug should have ethics");
            }
        }
    }

    #[test]
    fn test_anatomy_structure() {
        // Carrega o arquivo de fixture usando o helper
        let path = "tests/fixtures/fragments/bug/complete.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read complete.sc file");

        // Testa o parse
        let anatomy = Anatomy::from_string(input)
            .expect("Failed to parse anatomy");

        // Verifica a estrutura
        match anatomy {
            Anatomy::Bug(bug) => {
                assert_eq!(bug.specie.raw, "TestBug", "Bug species should be TestBug");
                assert_eq!(bug.genes.len(), 1, "Bug should have exactly 1 gene");
                assert_eq!(bug.ethics.len(), 4, "Bug should have exactly 4 ethics");
            }
        }
    }

    #[test]
    fn test_anatomy_clone() {
        // Testa se a clonagem funciona corretamente
        let input = "bug CloneBug\n  gene z Bool\nend".to_string();

        let anatomy = Anatomy::from_string(input)
            .expect("Failed to parse anatomy");
        let cloned_anatomy = anatomy.clone();

        // Verifica que ambos são iguais
        match (anatomy, cloned_anatomy) {
            (Anatomy::Bug(bug), Anatomy::Bug(cloned_bug)) => {
                assert_eq!(bug.specie, cloned_bug.specie, "Species should be equal");
                assert_eq!(bug.genes.len(), cloned_bug.genes.len(), "Gene count should be equal");
                assert_eq!(bug.ethics.len(), cloned_bug.ethics.len(), "Ethics count should be equal");
            }
        }
    }

    #[test]
    fn test_anatomy_direct_access() {
        // Testa acesso direto aos atributos
        let input = "bug DirectBug\n  gene a Int\n  gene b String\nend".to_string();

        let anatomy = Anatomy::from_string(input)
            .expect("Failed to parse anatomy");

        // Testa acesso direto aos atributos
        match anatomy {
            Anatomy::Bug(bug) => {
                assert_eq!(bug.specie.raw, "DirectBug");
                assert_eq!(bug.genes.len(), 2);
                assert_eq!(bug.ethics.len(), 0);
            }
        }
    }
}
