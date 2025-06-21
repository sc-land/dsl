mod splice;

use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::ast::sc::fly::strand::genome::anatomy::bug::ethics::matrix::Matrix;
use crate::ast::sc::fly::strand::genome::behavior::beat::sprout::splice::Splice;
use crate::ast::sc::fly::strand::genome::behavior::trace::Trace;
use crate::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sprout {
    pub condition: Trace,
    pub matrix: Matrix,
    pub splices: Vec<Splice>,
    pub den: Option<Matrix>,
}
impl Sprout {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::sprout);

        let mut pairs = pair.into_inner();

        // Parse condition
        let condition_pair = pairs.next().expect("If statement should have a condition");
        let condition = Trace::from_pair(condition_pair);

        // Parse then block
        let matrix_pair = pairs.next().expect("If statement should have a then block");
        let then_block = Matrix::from_pair(matrix_pair);

        let mut elsif_blocks = Vec::new();
        let mut else_block = None;

        // Parse elsif and else blocks
        while let Some(current_pair) = pairs.next() {
            match current_pair.as_rule() {
                Rule::splice => {
                    let raw_elsif = current_pair.as_str().to_string();
                    let mut elsif_pairs = current_pair.into_inner();

                    // Parse elsif condition
                    let elsif_condition_pair = elsif_pairs
                        .next()
                        .expect("Elsif block should have a condition");
                    let elsif_condition = Trace::from_pair(elsif_condition_pair);

                    // Parse elsif matrix block
                    let elsif_matrix_pair = elsif_pairs
                        .next()
                        .expect("Elsif block should have a matrix block");
                    let elsif_matrix = Matrix::from_pair(elsif_matrix_pair);

                    elsif_blocks.push(Splice {
                        raw: raw_elsif,
                        condition: elsif_condition,
                        block: elsif_matrix,
                    });
                }
                Rule::den => {
                    let else_matrix_pair = current_pair
                        .into_inner()
                        .next()
                        .expect("Else block should have a matrix");
                    else_block = Some(Matrix::from_pair(else_matrix_pair));
                }
                _ => panic!(
                    "Unexpected rule in if statement: {:?}",
                    current_pair.as_rule()
                ),
            }
        }

        Sprout {
            condition,
            matrix: then_block,
            splices: elsif_blocks,
            den: else_block,
        }
    }

    pub fn has_elsif(&self) -> bool {
        !self.splices.is_empty()
    }

    pub fn has_else(&self) -> bool {
        self.den.is_some()
    }
}
