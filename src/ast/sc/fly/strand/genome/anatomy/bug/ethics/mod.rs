pub mod signature;
pub mod matrix;

use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::anatomy::bug::ethics::matrix::Matrix;
use crate::parser::parser::Rule;
use crate::ast::sc::fly::strand::genome::anatomy::bug::ethics::signature::Signature;
use crate::ast::sc::fly::strand::genome::anatomy::bug::gene::primor::Primor;
use crate::ast::sc::fly::strand::genome::behavior::EthicsBind;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ethics {
    pub primor: Primor,
    pub signature: Option<Signature>,
    pub feedback: Option<Feedback>,
    pub body: Option<Matrix>,
}



impl Ethics {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::ethics);

        let mut inner = pair.into_inner();
        let mut primor = None;
        let mut signature = None;
        let mut feedback = None;
        let mut body = None;

        while let Some(inner_pair) = inner.next() {
            match inner_pair.as_rule() {
                Rule::primor => {
                    primor = Some(Primor::new(inner_pair.as_str().to_string()));
                }
                Rule::signature => {
                    signature = Some(Signature::from_pair(inner_pair));
                }
                Rule::feedback => {
                    feedback = Some(Feedback::from_pair(inner_pair));
                }
                Rule::matrix => {
                    body = Some(Matrix::from_pair(inner_pair));
                }
                _ => { panic!("Unexpected rule in ethics: {:?}", inner_pair.as_rule()); }
            }
        }

        Ethics {
            primor: primor.expect("Ethics must have a tag"),
            signature,
            feedback,
            body
        }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::ethics, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Ethics::from_pair(pair))
    }
}

impl Signature {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::signature);

        let mut ethics_binds = Vec::new();
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::ethics_binds => {
                    for ethics_bind_pair in inner_pair.into_inner() {
                        if ethics_bind_pair.as_rule() == Rule::ethics_bind {
                            ethics_binds.push(EthicsBind::from_pair(ethics_bind_pair));
                        }
                    }
                }
                Rule::ethics_bind => {
                    ethics_binds.push(EthicsBind::from_pair(inner_pair));
                }
                _ => {}
            }
        }
        let binds = if ethics_binds.is_empty() { None } else { Some(ethics_binds) };
        Signature { binds }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feedback {
    // { ASCII_ALPHA_UPPER ~ (ASCII_ALPHANUMERIC | "_")* }
    pub raw: String,
}

impl Feedback {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::feedback);
        let raw = pair.as_str().to_string();
        Self { raw }
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::ast::sc::fly::strand::genome::anatomy::bug::ethics::{matrix::signal::Signal, Ethics};


    #[test]
    fn test_ethics_with_body() {
        // Carrega o arquivo de teste que contém um ethics com corpo/matriz
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/ethics_with_body.sc")
            .expect("Failed to read ethics with body fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        // Verifica que os atributos básicos foram extraídos corretamente
        assert_eq!(ethics.primor.raw, "corre", "O tag do ethics deve ser 'corre'");
        assert!(ethics.signature.is_none(), "Ethics não deve ter assinatura");
        assert!(ethics.feedback.is_none(), "Ethics não deve ter retorno/feedback");

        // Verifica a existência e conteúdo do corpo
        assert!(ethics.body.is_some(), "Ethics deve ter um corpo/matrix");

        if let Some(body) = &ethics.body {
            assert!(!body.signals.is_empty(), "O corpo do ethics deve conter sinais");
            assert_eq!(body.signals.len(), 2, "O corpo deve conter exatamente 2 sinais");
            // Podemos adicionar mais verificações específicas sobre os sinais aqui
        }
    }

    #[test]
    fn test_simple_ethics() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/simple_ethics.sc").expect("Failed to read simple_ethics fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        assert_eq!(ethics.primor.raw, "simple_method");
        assert!(ethics.signature.is_none());
        assert!(ethics.feedback.is_none());
        assert!(ethics.body.is_none());
    }

    #[test]
    fn test_ethics_with_feedback() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/ethics_with_feedback.sc").expect("Failed to read ethics_with_feedback fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        assert_eq!(ethics.primor.raw, "method_with_return");
        assert!(ethics.signature.is_none());
        assert!(ethics.feedback.is_some());
        assert!(ethics.body.is_none());

        let feedback = ethics.feedback.as_ref().expect("Should have feedback");
        assert_eq!(feedback.raw, "String");
    }

    #[test]
    fn test_ethics_with_signature() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/ethics_with_signature.sc").expect("Failed to read ethics_with_signature fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        assert_eq!(ethics.primor.raw, "method_with_params");
        assert!(ethics.signature.is_some());
        assert!(ethics.feedback.is_none());
        assert!(ethics.body.is_none());

        let signature = ethics.signature.as_ref().expect("Should have signature");
        assert!(signature.binds.is_some());

        let binds = signature.binds.as_ref().expect("Should have binds");
        assert_eq!(binds.len(), 1);
        assert_eq!(binds[0].tag.raw, "param1");
        assert_eq!(binds[0].specie.raw, "Int");
    }

    #[test]
    fn test_ethics_with_multiple_params() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/ethics_with_multiple_params.sc").expect("Failed to read ethics_with_multiple_params fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        let signature = ethics.signature.as_ref().expect("Should have signature");
        let binds = signature.binds.as_ref().expect("Should have binds");
        assert_eq!(binds.len(), 2);

        // Test individual bind methods
        let first_bind = &binds[0];
        assert_eq!(first_bind.tag.raw, "param1");
        assert_eq!(first_bind.specie.raw, "Int");

        let second_bind = &binds[1];
        assert_eq!(second_bind.tag.raw, "param2");
        assert_eq!(second_bind.specie.raw, "String");
    }

    #[test]
    fn test_ethics_with_signature_and_feedback() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/ethics_with_signature_and_feedback.sc").expect("Failed to read ethics_with_signature_and_feedback fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        assert_eq!(ethics.primor.raw, "full_method");
        assert!(ethics.signature.is_some());
        assert!(ethics.feedback.is_some());
        assert!(ethics.body.is_none());

        // Test signature
        let signature = ethics.signature.as_ref().expect("Should have signature");
        assert!(signature.binds.is_some());

        let binds = signature.binds.as_ref().expect("Should have binds");
        assert_eq!(binds.len(), 2);

        // Test feedback
        let feedback = ethics.feedback.as_ref().expect("Should have feedback");
        assert_eq!(feedback.raw, "Boolean");
    }

    #[test]
    fn test_ethics_from_fixture() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/simple_ethics.sc")
            .expect("Failed to read simple_ethics fixture");

        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        assert_eq!(ethics.primor.raw, "simple_method");
        assert!(ethics.signature.is_none());
        assert!(ethics.feedback.is_none());
        assert!(ethics.body.is_none());
    }

    #[test]
    fn test_ethics_feedback_methods() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/ethics_feedback_methods.sc").expect("Failed to read ethics_feedback_methods fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        let feedback = ethics.feedback.as_ref().expect("Should have feedback");

        // Test feedback methods
        assert_eq!(feedback.raw, "Boolean");
    }

    #[test]
    fn test_ethics_signature_methods() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/ethics_signature_methods.sc").expect("Failed to read ethics_signature_methods fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        let signature = ethics.signature.as_ref().expect("Should have signature");

        // Test signature methods
        assert!(signature.binds.is_none()); // Empty params
    }

    #[test]
    fn test_ethics_clone() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/ethics_clone.sc").expect("Failed to read ethics_clone fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");
        let cloned = ethics.clone();

        assert_eq!(ethics.primor.raw, cloned.primor.raw);
        assert_eq!(ethics.signature.is_some(), cloned.signature.is_some());
        assert_eq!(ethics.feedback.is_some(), cloned.feedback.is_some());
        assert_eq!(ethics.body.is_some(), cloned.body.is_some());
    }

    #[test]
    fn test_ethics_body_content() {
        // Carrega o arquivo de teste que contém um ethics com corpo/matriz
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/ethics_with_body.sc")
            .expect("Failed to read ethics with body fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        // Verifica a existência do corpo
        let body = ethics.body.as_ref().expect("Ethics deve ter um corpo/matrix");


        // Verifica a estrutura dos sinais (comportamentos)
        assert_eq!(body.signals.len(), 2, "O corpo deve ter exatamente 2 sinais");

        // Podemos verificar mais detalhes dos sinais quando necessário:
        for (i, signal) in body.signals.iter().enumerate() {
            // Como nosso Signal enum atualmente só tem uma variante (Behavior), podemos usar um match direto
            let behavior = match signal {
                Signal::Behavior(b) => b
            };

            // Verifica se o comportamento é do tipo Assign
            match behavior {
                crate::ast::sc::fly::strand::genome::Behavior::Pollinate(assign) => {
                    // Verifica as atribuições específicas
                    if i == 0 {
                        assert_eq!(assign.tag.raw, "vai", "Primeira atribuição deve ser para 'vai'");
                    } else if i == 1 {
                        assert_eq!(assign.tag.raw, "volta", "Segunda atribuição deve ser para 'volta'");
                    }

                    // Verifica se o valor atribuído contém folego.div(2)
                    assert!(assign.raw.contains("folego.div(2)"),
                            "O valor atribuído deve conter 'folego.div(2)'");
                },
                _ => panic!("Esperava-se um comportamento do tipo Assign")
            }
        }
    }
}
