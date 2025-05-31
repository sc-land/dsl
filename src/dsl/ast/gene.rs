use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;
use crate::dsl::ast::emitter::{Tag, Specie};

#[derive(Debug, Clone)]
pub struct Gene {
    pub raw: String,
    pub tag: Tag,
    pub specie: Specie,
}

impl Gene {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::gene);

        let raw = pair.as_str().to_string();

        let mut inner = pair.into_inner();

        let tag_str = inner.next()
            .expect("Gene deve ter uma tag")
            .as_str()
            .to_string();
        let tag = Tag::new(tag_str);

        let specie_str = inner.next()
            .expect("Gene deve ter uma espécie")
            .as_str()
            .to_string();
        let specie = Specie::new(specie_str);

        Gene { raw, tag, specie }
    }

    pub fn get_tag(&self) -> &Tag {
        &self.tag
    }

    pub fn get_specie(&self) -> &Specie {
        &self.specie
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::dsl::ast::gene::Gene;
    use crate::dsl::parser::parser::{Rule, SCP};    #[test]
    fn test_gene_from_pair() {
        let input = "gene protein Cat".to_string();
        let parsed = SCP::parse(Rule::gene, &input).unwrap();
        let gene = Gene::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(gene.raw, "gene protein Cat");
        assert_eq!(gene.get_tag().get_raw(), "protein");
        assert_eq!(gene.get_specie().get_raw(), "Cat");
    }

    #[test]
    fn test_gene_getters() {
        let input = "gene enzyme Dog".to_string();
        let parsed = SCP::parse(Rule::gene, &input).unwrap();
        let gene = Gene::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(gene.get_raw(), "gene enzyme Dog");
        assert_eq!(gene.get_tag().get_raw(), "enzyme");
        assert_eq!(gene.get_specie().get_raw(), "Dog");
    }

    #[test]
    fn test_gene_with_complex_names() {
        let input = "gene complex_protein AdvancedSpecies".to_string();
        let parsed = SCP::parse(Rule::gene, &input).unwrap();
        let gene = Gene::from_pair(parsed.into_iter().next().unwrap());
        assert_eq!(gene.get_tag().get_raw(), "complex_protein");
        assert_eq!(gene.get_specie().get_raw(), "AdvancedSpecies");
    }

    #[test]
    fn test_gene_struct_getters() {
        let input = "gene enzyme Dog".to_string();
        let parsed = SCP::parse(Rule::gene, &input).unwrap();
        let gene = Gene::from_pair(parsed.into_iter().next().unwrap());

        let tag_struct = gene.get_tag();
        let specie_struct = gene.get_specie();

        assert_eq!(tag_struct.get_raw(), "enzyme");
        assert_eq!(specie_struct.get_raw(), "Dog");
    }
}
