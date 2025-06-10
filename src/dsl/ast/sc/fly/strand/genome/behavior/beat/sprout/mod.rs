mod splice;

use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::ast::sc::fly::strand::genome::anatomy::bug::ethics::matrix::Matrix;
use crate::dsl::ast::sc::fly::strand::genome::behavior::beat::sprout::splice::Splice;
use crate::dsl::ast::sc::fly::strand::genome::behavior::trace::Trace;
use crate::dsl::parser::parser::Rule;

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
        println!("Condition pair rule: {:?}", condition_pair.as_rule());
        let condition = Trace::from_pair(condition_pair);

        // Parse then block
        let matrix_pair = pairs.next().expect("If statement should have a then block");
        println!("Matrix pair rule: {:?}", matrix_pair.as_rule());
        let then_block = Matrix::from_pair(matrix_pair);

        let mut elsif_blocks = Vec::new();
        let mut else_block = None;

        // Parse elsif and else blocks
        while let Some(current_pair) = pairs.next() {
            println!("Current pair rule: {:?}", current_pair.as_rule());
            match current_pair.as_rule() {
                Rule::splice => {
                    let raw_elsif = current_pair.as_str().to_string();
                    println!("Elsif raw: {}", raw_elsif);
                    let mut elsif_pairs = current_pair.into_inner();

                    // Parse elsif condition
                    let elsif_condition_pair = elsif_pairs
                        .next()
                        .expect("Elsif block should have a condition");
                    println!(
                        "Elsif condition pair rule: {:?}",
                        elsif_condition_pair.as_rule()
                    );
                    let elsif_condition = Trace::from_pair(elsif_condition_pair);

                    // Parse elsif matrix block
                    let elsif_matrix_pair = elsif_pairs
                        .next()
                        .expect("Elsif block should have a matrix block");
                    println!("Elsif matrix pair rule: {:?}", elsif_matrix_pair.as_rule());
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
                Rule::if_ends => {
                    // Ignore the end token
                    continue;
                }
                _ => panic!(
                    "Unexpected rule in if statement: {:?}",
                    current_pair.as_rule()
                ),
            }
        }

        println!("Number of elsif blocks: {}", elsif_blocks.len());

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
