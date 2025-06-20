use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};
use anatomy::Anatomy;
pub(crate) use behavior::Behavior;
use crate::dsl::parser::parser::{Rule, SCP};

pub mod anatomy;
pub mod behavior;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Genome {
    Anatomy(Anatomy),
    Behavior(Behavior),
}

impl Genome {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::genome);

        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::anatomy => Genome::Anatomy(Anatomy::from_pair(inner)),
            Rule::behavior => Genome::Behavior(Behavior::from_pair(inner)),
            _ => panic!("Regra inesperada dentro de genome: {:?}", inner.as_rule()),
        }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut pairs = SCP::parse(Rule::genome, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Genome::from_pair(pair))
    }

    pub fn is_anatomy(&self) -> bool {
        matches!(self, Genome::Anatomy(_))
    }

    pub fn is_behavior(&self) -> bool {
        matches!(self, Genome::Behavior(_))
    }

    pub fn as_anatomy(&self) -> Option<&Anatomy> {
        match self {
            Genome::Anatomy(anatomy) => Some(anatomy),
            _ => None,
        }
    }

    pub fn as_behavior(&self) -> Option<&Behavior> {
        match self {
            Genome::Behavior(behavior) => Some(behavior),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::Genome;
    use crate::dsl::ast::sc::fly::strand::genome::anatomy::Anatomy;
    use crate::dsl::ast::sc::fly::strand::genome::behavior::Behavior;

    #[test]
    fn test_genome_with_anatomy() {
        // Carrega o fixture de anatomy
        let path = "tests/fixtures/fragments/genome/simple_anatomy.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read simple_anatomy.sc file");

        // Verifica que o arquivo não está vazio
        assert!(!input.is_empty(), "Fixture file should not be empty");

        // Testa o parse usando from_string
        let genome = Genome::from_string(input)
            .expect("Failed to parse genome");

        // Verifica se é um Anatomy usando métodos utilitários
        assert!(genome.is_anatomy(), "Genome should be Anatomy");
        assert!(!genome.is_behavior(), "Genome should not be Behavior");

        // Verifica o conteúdo
        let anatomy = genome.as_anatomy().expect("Should be anatomy");

        // Como Anatomy é um enum com apenas Bug, podemos verificar através do pattern matching
        match anatomy {
            Anatomy::Bug(bug) => {
                assert_eq!(bug.specie.raw, "Cat", "Bug species should be Cat");
                assert_eq!(bug.genes.len(), 2, "Bug should have exactly 2 genes");
                assert_eq!(bug.genes[0].tag.raw, "energia", "First gene should be energia");
                assert_eq!(bug.genes[1].tag.raw, "folego", "Second gene should be folego");
                assert_eq!(bug.ethics.len(), 0, "Bug should have no ethics");
            }
            Anatomy::Totem(_totem) => todo!()
        }
    }

    #[test]
    fn test_genome_with_behavior() {
        // Carrega o fixture de behavior
        let path = "tests/fixtures/fragments/genome/simple_behavior.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read simple_behavior.sc file");

        // Verifica que o arquivo não está vazio
        assert!(!input.is_empty(), "Fixture file should not be empty");

        // Verifica o conteúdo antes do parse
        assert!(input.contains("x = 42"), "Input should contain assignment x = 42");

        // Testa o parse usando from_string
        let genome = Genome::from_string(input)
            .expect("Failed to parse genome");

        // Verifica se é um Behavior usando métodos utilitários
        assert!(genome.is_behavior(), "Genome should be Behavior");
        assert!(!genome.is_anatomy(), "Genome should not be Anatomy");

        // Verifica o conteúdo
        let behavior = genome.as_behavior().expect("Should be behavior");
        assert!(matches!(behavior, Behavior::Pollinate(_)), "Should be an assign behavior");
    }

    #[test]
    fn test_genome_types_distinction() {
        // Testa que conseguimos distinguir entre os diferentes tipos de Genome

        // Anatomy
        let anatomy_path = "tests/fixtures/fragments/genome/simple_anatomy.sc".to_string();
        let anatomy_input = fs::read_to_string(anatomy_path)
            .expect("Failed to read simple_anatomy.sc file");
        let anatomy_genome = Genome::from_string(anatomy_input)
            .expect("Should parse anatomy");
        assert!(anatomy_genome.is_anatomy());
        assert!(!anatomy_genome.is_behavior());

        // Behavior
        let behavior_path = "tests/fixtures/fragments/genome/simple_behavior.sc".to_string();
        let behavior_input = fs::read_to_string(behavior_path)
            .expect("Failed to read simple_behavior.sc file");
        let behavior_genome = Genome::from_string(behavior_input)
            .expect("Should parse behavior");
        assert!(behavior_genome.is_behavior());
        assert!(!behavior_genome.is_anatomy());
    }
}
