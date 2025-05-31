use pest::iterators::Pair;
use crate::dsl::ast::anatomy::Anatomy;
use crate::dsl::ast::behavior::Behavior;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone)]
pub enum Genome {
    Anatomy(Anatomy),
    Behavior(Behavior),
}

impl Genome {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::genome);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::anatomy => Genome::Anatomy(Anatomy::from_pair(inner_pair)),
            Rule::behavior => Genome::Behavior(Behavior::from_pair(inner_pair)),
            _ => panic!("Regra inesperada dentro de genome: {:?}", inner_pair.as_rule()),
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::dsl::parser::parser::SCP;

    use super::*;


    #[test]
    fn test_genome_from_pair_anatomy() {
        let input = "bug People".to_string();
        let parsed = SCP::parse(Rule::genome, &input).unwrap();
        let genome = Genome::from_pair(parsed.into_iter().next().unwrap());
        if let Genome::Anatomy(anatomy) = genome {
            assert_eq!(anatomy.get_bug().raw, "bug People");
        } else {
            panic!("Expected Anatomy variant");
        }
    }

    #[test]
    fn test_genome_from_pair_behavior() {
        let input = "bug = Bug.happens".to_string();
        let parsed = SCP::parse(Rule::genome, &input).unwrap();
        let genome = Genome::from_pair(parsed.into_iter().next().unwrap());
        if let Genome::Behavior(behavior) = genome {
            match behavior {
                Behavior::Assign(assign) => {
                    assert_eq!(assign.raw, "bug = Bug.happens");
                }
                _ => panic!("Expected Assign variant"),
            }
        } else {
            panic!("Expected Behavior variant");
        }
    }

    #[test]
    fn test_genome_from_pair_behavior_simple() {
        let input = "bug.fly".to_string();
        let parsed = SCP::parse(Rule::genome, &input).unwrap();
        let genome = Genome::from_pair(parsed.into_iter().next().unwrap());
        if let Genome::Behavior(behavior) = genome {
            match behavior {
                Behavior::Oop(oop) => {
                    assert_eq!(oop.raw, "bug.fly");
                }
                _ => panic!("Expected Oop variant"),
            }
        } else {
            panic!("Expected Behavior variant");
        }
    }
}
