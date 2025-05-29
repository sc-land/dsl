use pest::iterators::Pair;
use crate::dsl::ast::ddl::DDL;
use crate::dsl::ast::dml::DML;
use crate::dsl::parser::parser::Rule;

#[derive(Debug, Clone)]
pub enum Genome {
    DDL(DDL),
    DML(DML),
}

impl Genome {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::genome);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::ddl => Genome::DDL(DDL::from_pair(inner_pair)),
            Rule::dml => Genome::DML(DML::from_pair(inner_pair)),
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
    fn test_genome_from_pair_ddl() {
        let input = "bug People".to_string();
        let parsed = SCP::parse(Rule::genome, &input).unwrap();
        let genome = Genome::from_pair(parsed.into_iter().next().unwrap());
        if let Genome::DDL(ddl) = genome {
            assert_eq!(ddl.bug.raw, "bug People");
        } else {
            panic!("Expected DDL variant");
        }
    }

    #[test]
    fn test_genome_from_pair_dml() {
        let input = "bug = Bug.happens".to_string();
        let parsed = SCP::parse(Rule::genome, &input).unwrap();
        let genome = Genome::from_pair(parsed.into_iter().next().unwrap());
        if let Genome::DML(dml) = genome {
            match dml {
                crate::dsl::ast::dml::DML::Assign(assign) => {
                    assert_eq!(assign.raw, "bug = Bug.happens");
                }
                _ => panic!("Expected Assign variant"),
            }
        } else {
            panic!("Expected DML variant");
        }
    }

    #[test]
    fn test_genome_from_pair_dml_simple() {
        let input = "bug.fly".to_string();
        let parsed = SCP::parse(Rule::genome, &input).unwrap();
        let genome = Genome::from_pair(parsed.into_iter().next().unwrap());
        if let Genome::DML(dml) = genome {
            match dml {
                crate::dsl::ast::dml::DML::Oop(oop) => {
                    assert_eq!(oop.raw, "bug.fly");
                }
                _ => panic!("Expected Oop variant"),
            }
        } else {
            panic!("Expected DML variant");
        }
    }
}
