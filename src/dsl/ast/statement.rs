use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::emitter::Tag;
use crate::dsl::ast::behavior::oop::Oop;
use crate::dsl::ast::condition::Condition;
use crate::dsl::ast::matrix::Matrix;

#[derive(Debug, Clone)]
pub enum Statement {
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub raw: String,
    pub condition: Condition,
    pub then_block: Matrix,
    pub elsif_blocks: Vec<ElsifBlock>,
    pub else_block: Option<Matrix>,
}

#[derive(Debug, Clone)]
pub struct ElsifBlock {
    pub raw: String,
    pub condition: Condition,
    pub block: Matrix,
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub raw: String,
    pub condition: Condition,
    pub block: Matrix,
}

#[derive(Debug, Clone)]
pub struct ForStatement {
    pub raw: String,
    pub variable: Tag,
    pub iterable: Oop,
    pub block: Matrix,
}

impl Statement {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::statement);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::r#if => Statement::If(IfStatement::from_pair(inner_pair)),
            Rule::r#while => Statement::While(WhileStatement::from_pair(inner_pair)),
            Rule::r#for => Statement::For(ForStatement::from_pair(inner_pair)),
            _ => panic!("Unexpected rule in statement: {:?}", inner_pair.as_rule()),
        }
    }

    pub fn is_if(&self) -> bool {
        matches!(self, Statement::If(_))
    }

    pub fn is_while(&self) -> bool {
        matches!(self, Statement::While(_))
    }

    pub fn is_for(&self) -> bool {
        matches!(self, Statement::For(_))
    }

    pub fn get_raw(&self) -> &str {
        match self {
            Statement::If(if_stmt) => &if_stmt.raw,
            Statement::While(while_stmt) => &while_stmt.raw,
            Statement::For(for_stmt) => &for_stmt.raw,
        }
    }
}

impl IfStatement {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::r#if);
        let raw = pair.as_str().to_string();
        println!("IfStatement raw: {}", raw);

        let mut pairs = pair.into_inner();

        // Parse condition
        let condition_pair = pairs.next().expect("If statement should have a condition");
        println!("Condition pair rule: {:?}", condition_pair.as_rule());
        let condition = Condition::from_pair(condition_pair);

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
                Rule::elsif => {
                    let raw_elsif = current_pair.as_str().to_string();
                    println!("Elsif raw: {}", raw_elsif);
                    let mut elsif_pairs = current_pair.into_inner();

                    // Parse elsif condition
                    let elsif_condition_pair = elsif_pairs.next().expect("Elsif block should have a condition");
                    println!("Elsif condition pair rule: {:?}", elsif_condition_pair.as_rule());
                    let elsif_condition = Condition::from_pair(elsif_condition_pair);

                    // Parse elsif matrix block
                    let elsif_matrix_pair = elsif_pairs.next().expect("Elsif block should have a matrix block");
                    println!("Elsif matrix pair rule: {:?}", elsif_matrix_pair.as_rule());
                    let elsif_matrix = Matrix::from_pair(elsif_matrix_pair);

                    elsif_blocks.push(ElsifBlock {
                        raw: raw_elsif,
                        condition: elsif_condition,
                        block: elsif_matrix,
                    });
                },
                Rule::r#else => {
                    let else_matrix_pair = current_pair.into_inner()
                        .next().expect("Else block should have a matrix");
                    else_block = Some(Matrix::from_pair(else_matrix_pair));
                },
                Rule::if_ends => {
                    // Ignore the end token
                    continue;
                },
                _ => panic!("Unexpected rule in if statement: {:?}", current_pair.as_rule()),
            }
        }

        println!("Number of elsif blocks: {}", elsif_blocks.len());

        IfStatement {
            raw,
            condition,
            then_block,
            elsif_blocks,
            else_block,
        }
    }

    pub fn has_elsif(&self) -> bool {
        !self.elsif_blocks.is_empty()
    }

    pub fn has_else(&self) -> bool {
        self.else_block.is_some()
    }
}

impl WhileStatement {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::r#while);
        let raw = pair.as_str().to_string();

        let mut pairs = pair.into_inner();

        // Parse condition
        let condition_pair = pairs.next().expect("While statement should have a condition");
        let condition = Condition::from_pair(condition_pair);

        // Parse loop body block
        let matrix_pair = pairs.next().expect("While statement should have a body block");
        let block = Matrix::from_pair(matrix_pair);

        WhileStatement { raw, condition, block }
    }
}

impl ForStatement {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::r#for);
        let raw = pair.as_str().to_string();

        let mut pairs = pair.into_inner();

        // Parse variable (each)
        let variable_pair = pairs.next().expect("For statement should have a variable");
        assert_eq!(variable_pair.as_rule(), Rule::each);
        let tag_pair = variable_pair.into_inner().next().expect("Each should have a tag");
        let variable = Tag::new(tag_pair.as_str().to_string());

        // Parse iterable (oop)
        let oop_pair = pairs.next().expect("For statement should have an iterable oop");
        let iterable = Oop::from_pair(oop_pair);

        // Parse loop body block
        let matrix_pair = pairs.next().expect("For statement should have a body block");
        let block = Matrix::from_pair(matrix_pair);

        ForStatement { raw, variable, iterable, block }
    }

    pub fn get_variable(&self) -> &Tag {
        &self.variable
    }
}

impl ElsifBlock {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::elsif);
        let raw = pair.as_str().to_string();

        let mut pairs = pair.into_inner();

        // Parse condition
        let condition_pair = pairs.next().expect("Elsif block should have a condition");
        let condition = Condition::from_pair(condition_pair);

        // Parse block
        let matrix_pair = pairs.next().expect("Elsif block should have a matrix block");
        let block = Matrix::from_pair(matrix_pair);

        ElsifBlock { raw, condition, block }
    }
}
