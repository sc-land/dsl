use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct Nucleus {
    pub raw: String,
    pub matrix: Matrix,
}

impl Nucleus {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::nucleus);
        let raw = pair.as_str().to_string();

        let matrix_pair = pair.into_inner()
            .find(|p| p.as_rule() == Rule::matrix)
            .expect("Nucleus must have a matrix");

        let matrix = Matrix::from_pair(matrix_pair);

        Nucleus { raw, matrix }
    }

    pub fn from_string(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        use pest::Parser;
        use crate::dsl::parser::parser::SCP;

        let mut pairs = SCP::parse(Rule::nucleus, &input)?;
        let pair = pairs.next().ok_or("No pair found")?;
        Ok(Nucleus::from_pair(pair))
    }
}
