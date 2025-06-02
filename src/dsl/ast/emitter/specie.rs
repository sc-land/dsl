#[derive(Debug, Clone, PartialEq)]
pub struct Specie {
    // { ASCII_ALPHA_UPPER ~ (ASCII_ALPHANUMERIC | "_")* }
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
