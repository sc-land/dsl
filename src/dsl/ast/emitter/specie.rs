#[derive(Debug, Clone)]
pub struct Specie {
    pub raw: String,
}

impl Specie {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specie_creation() {
        let specie = Specie::new("Animal".to_string());
        assert_eq!(specie.get_raw(), "Animal");
        assert_eq!(specie.raw, "Animal");
    }

    #[test]
    fn test_specie_debug() {
        let specie = Specie::new("Cat".to_string());
        let debug_str = format!("{:?}", specie);
        assert!(debug_str.contains("Cat"));
    }
}
