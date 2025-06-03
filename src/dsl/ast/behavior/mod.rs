pub mod assign;
pub mod oop;
pub mod trail;
pub mod transport;
pub mod binds;
pub mod bind;
pub mod sequence;

use pest::{iterators::Pair, Parser};
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::{Rule, SCP};
use self::assign::Assign;
use self::oop::Oop;
use crate::dsl::ast::statement::Statement;

// Re-export Trail types for easier access
pub use trail::{Trail, Catalysis, Carrier};
pub use binds::{Binds, EthicsBinds};
pub use bind::{Bind, EthicsBind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Behavior {
    Statement(Statement),
    Assign(Assign),
    Oop(Oop),
}

impl Behavior {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::behavior);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::statement => Behavior::Statement(Statement::from_pair(inner_pair)),
            Rule::assign => Behavior::Assign(Assign::from_pair(inner_pair)),
            Rule::oop => Behavior::Oop(Oop::from_pair(inner_pair)),
            _ => panic!("Regra inesperada dentro de behavior: {:?}", inner_pair.as_rule()),
        }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut pairs = SCP::parse(Rule::behavior, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Behavior::from_pair(pair))
    }

    pub fn get_oop(&self) -> Option<&Oop> {
        match self {
            Behavior::Oop(oop) => Some(oop),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dsl::ast::behavior::Behavior;


    #[test]
    fn test_behavior_statement_while() {
        use std::fs;

        // Carrega o fragmento de statement (while)
        let path = "tests/fixtures/fragments/behavior/statement_while.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read statement_while.sc file");

        // Verifica que o arquivo não está vazio
        assert!(!input.is_empty(), "Fixture file should not be empty");

        // Testa o parse usando from_string
        let behavior = Behavior::from_string(input.clone())
            .expect("Should parse statement behavior successfully");

        // Verifica se é um Statement
        match behavior {
            Behavior::Statement(_stmt) => {
                assert!(input.contains("while"), "Input should contain 'while'");
                assert!(input.contains("counter"), "Input should contain 'counter'");
                assert!(input.contains("end"), "Input should contain 'end'");
            },
            _ => panic!("Expected Statement behavior, got: {:?}", behavior),
        }
    }

    #[test]
    fn test_behavior_assign_simple() {
        use std::fs;

        // Carrega o fragmento de assign
        let path = "tests/fixtures/fragments/behavior/assign_simple.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read assign_simple.sc file");

        // Verifica que o arquivo não está vazio
        assert!(!input.is_empty(), "Fixture file should not be empty");

        // Testa o parse usando from_string
        let behavior = Behavior::from_string(input.clone())
            .expect("Should parse assign behavior successfully");

        // Verifica se é um Assign
        match behavior {
            Behavior::Assign(_assign) => {
                assert!(input.contains("result"), "Input should contain 'result'");
                assert!(input.contains("="), "Input should contain '='");
                assert!(input.contains("calculator"), "Input should contain 'calculator'");
            },
            _ => panic!("Expected Assign behavior, got: {:?}", behavior),
        }
    }

    #[test]
    fn test_behavior_oop_method_chain() {
        use std::fs;

        // Carrega o fragmento de oop
        let path = "tests/fixtures/fragments/behavior/oop_method_chain.sc".to_string();
        let input = fs::read_to_string(path)
            .expect("Failed to read oop_method_chain.sc file");

        // Verifica que o arquivo não está vazio
        assert!(!input.is_empty(), "Fixture file should not be empty");

        // Testa o parse usando from_string
        let behavior = Behavior::from_string(input.clone())
            .expect("Should parse oop behavior successfully");

        // Verifica se conseguimos extrair o oop usando o método utilitário
        assert!(behavior.get_oop().is_some(), "Should be able to get oop");

        // Verifica se é um Oop
        if let Behavior::Oop(ref _oop) = behavior {
            assert!(input.contains("user"), "Input should contain 'user'");
            assert!(input.contains("getName"), "Input should contain 'getName'");
            assert!(input.contains("toUpperCase"), "Input should contain 'toUpperCase'");
        } else {
            panic!("Expected Oop behavior, got: {:?}", behavior);
        }
    }

    #[test]
    fn test_behavior_types_distinction() {
        use std::fs;

        // Testa que conseguimos distinguir entre os diferentes tipos de Behavior

        // Statement
        let statement_path = "tests/fixtures/fragments/behavior/statement_while.sc".to_string();
        let statement_input = fs::read_to_string(statement_path)
            .expect("Failed to read statement_while.sc file");
        let statement_behavior = Behavior::from_string(statement_input)
            .expect("Should parse statement");
        assert!(matches!(statement_behavior, Behavior::Statement(_)));
        assert!(statement_behavior.get_oop().is_none());

        // Assign
        let assign_path = "tests/fixtures/fragments/behavior/assign_simple.sc".to_string();
        let assign_input = fs::read_to_string(assign_path)
            .expect("Failed to read assign_simple.sc file");
        let assign_behavior = Behavior::from_string(assign_input)
            .expect("Should parse assign");
        assert!(matches!(assign_behavior, Behavior::Assign(_)));
        assert!(assign_behavior.get_oop().is_none());

        // Oop
        let oop_path = "tests/fixtures/fragments/behavior/oop_method_chain.sc".to_string();
        let oop_input = fs::read_to_string(oop_path)
            .expect("Failed to read oop_method_chain.sc file");
        let oop_behavior = Behavior::from_string(oop_input)
            .expect("Should parse oop");
        assert!(matches!(oop_behavior, Behavior::Oop(_)));
        assert!(oop_behavior.get_oop().is_some());
    }
}
