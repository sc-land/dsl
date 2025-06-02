use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::emitter::{Tag, Specie};
use crate::dsl::ast::behavior::bind::EthicsBind;
use crate::dsl::ast::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct Ethics {
    pub tag: Tag,
    pub signature: Option<Signature>,
    pub feedback: Option<Feedback>,
    pub body: Option<Matrix>,
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub binds: Option<Vec<EthicsBind>>,
}

#[derive(Debug, Clone)]
pub struct Feedback {
    pub specie: Specie,
}

impl Ethics {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::ethics);

        let mut inner = pair.into_inner();
        let mut tag = None;
        let mut signature = None;
        let mut feedback = None;
        let mut body = None;

        while let Some(inner_pair) = inner.next() {
            match inner_pair.as_rule() {
                Rule::tag => {
                    tag = Some(Tag::new(inner_pair.as_str().to_string()));
                }
                Rule::signature => {
                    signature = Some(Signature::from_pair(inner_pair));
                }
                Rule::specie => {
                    feedback = Some(Feedback { specie: Specie::new(inner_pair.as_str().to_string()) });
                }
                Rule::matrix => {
                    body = Some(Matrix::from_pair(inner_pair));
                }
                _ => {}
            }
        }

        Ethics {
            tag: tag.expect("Ethics must have a tag"),
            signature,
            feedback,
            body
        }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

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

impl Feedback {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::feedback);

        let inner_pair = pair.into_inner().next().expect("Feedback must have a specie");
        let specie = Specie::new(inner_pair.as_str().to_string());

        Feedback { specie }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_simple_ethics() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/simple_ethics.sc").expect("Failed to read simple_ethics fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        assert_eq!(ethics.tag.raw, "simple_method");
        assert!(ethics.signature.is_none());
        assert!(ethics.feedback.is_none());
        assert!(ethics.body.is_none());
    }

    #[test]
    fn test_ethics_with_feedback() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/ethics_with_feedback.sc").expect("Failed to read ethics_with_feedback fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        assert_eq!(ethics.tag.raw, "method_with_return");
        assert!(ethics.signature.is_none());
        assert!(ethics.feedback.is_some());
        assert!(ethics.body.is_none());

        let feedback = ethics.feedback.as_ref().expect("Should have feedback");
        assert_eq!(feedback.specie.raw, "String");
    }

    #[test]
    fn test_ethics_with_signature() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/ethics_with_signature.sc").expect("Failed to read ethics_with_signature fragment");
        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        assert_eq!(ethics.tag.raw, "method_with_params");
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

        assert_eq!(ethics.tag.raw, "full_method");
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
        assert_eq!(feedback.specie.raw, "Boolean");
    }

    #[test]
    fn test_ethics_from_fixture() {
        let input = fs::read_to_string("tests/fixtures/fragments/ethics/simple_ethics.sc")
            .expect("Failed to read simple_ethics fixture");

        let ethics = Ethics::from_string(input.clone()).expect("Failed to parse ethics");

        assert_eq!(ethics.tag.raw, "simple_method");
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
        assert_eq!(feedback.specie.raw, "Boolean");
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

        assert_eq!(ethics.tag.raw, cloned.tag.raw);
        assert_eq!(ethics.signature.is_some(), cloned.signature.is_some());
        assert_eq!(ethics.feedback.is_some(), cloned.feedback.is_some());
        assert_eq!(ethics.body.is_some(), cloned.body.is_some());
    }
}
