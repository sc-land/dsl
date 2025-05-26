pub mod dsl;

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::dsl::parser::parser::{Rule, SCP};

    #[test]
    fn it_works() {
        let input = "a".to_string();
        let parsed = SCP::parse(Rule::sc, &input);
        assert!(parsed.is_ok(), "The parser should successfully parse alphabetic input");
    }

    #[test]
    fn does_not_work() {
        let input = "9".to_string();
        let parsed = SCP::parse(Rule::sc, &input);
        assert!(parsed.is_err(), "The parser should fail when given a number");
    }
}
