#[derive(Debug, Clone)]
pub struct Tag {
    pub raw: String,
}

impl Tag {
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
    fn test_tag_creation() {
        let tag = Tag::new("method".to_string());
        assert_eq!(tag.get_raw(), "method");
        assert_eq!(tag.raw, "method");
    }

    #[test]
    fn test_tag_debug() {
        let tag = Tag::new("variable".to_string());
        let debug_str = format!("{:?}", tag);
        assert!(debug_str.contains("variable"));
    }
}
