use pest::{iterators::Pair, Parser};
use crate::dsl::parser::parser::{Rule, SCP};

#[derive(Debug, Clone)]
pub struct Bug {
    pub raw: String,
    pub specie: String,
}

impl Bug {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::bug);

        let raw = pair.as_str().to_string();

        let mut inner = pair.into_inner();
        let specie = inner.next()
            .expect("Bug deve ter uma espécie")
            .as_str()
            .to_string();

        Bug { raw, specie }
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
    use pest::Parser;

    use crate::dsl::ast::bug::Bug;
    use crate::dsl::parser::parser::{Rule, SCP};

    #[test]
    fn test_bug_from_pair() {
        let input = "bug Cat".to_string();
        let parsed = SCP::parse(Rule::bug, &input).unwrap();
        let bug = Bug::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(bug.raw, "bug Cat");
    }

    #[test]
    fn test_bug_from_string() {
        let input = "bug Dog".to_string();
        let bug = Bug::from_string(input);
        assert_eq!(bug.raw, "bug Dog");
    }

    /// testar a sintaxe errada de bug
    /// "bug" sem especie - deve falhar no parser
    #[test]
    fn test_bug_from_string_invalid() {
        let input = "bug".to_string();
        let result = SCP::parse(Rule::bug, &input);

        // Deve falhar porque "bug" sem espécie não é uma sintaxe válida
        assert!(result.is_err(), "Parser deveria falhar com input inválido");

        // Verifica se o erro menciona que esperava uma espécie
        let error = result.unwrap_err();
        let error_string = format!("{:?}", error);
        assert!(error_string.contains("specie"), "Erro deveria mencionar que esperava uma espécie");
    }

    #[test]
    fn test_bug_from_string_invalid_empty() {
        let input = "".to_string();
        let result = SCP::parse(Rule::bug, &input);
        assert!(result.is_err(), "Parser deveria falhar com input vazio");
    }

    #[test]
    fn test_bug_from_string_invalid_wrong_keyword() {
        let input = "insect Cat".to_string();
        let result = SCP::parse(Rule::bug, &input);
        assert!(result.is_err(), "Parser deveria falhar com palavra-chave incorreta");
    }

    #[test]
    fn test_bug_from_string_invalid_lowercase_specie() {
        let input = "bug cat".to_string();
        let result = SCP::parse(Rule::bug, &input);
        assert!(result.is_err(), "Parser deveria falhar com espécie em letra minúscula");
    }

}

