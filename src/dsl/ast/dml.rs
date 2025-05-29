#[derive(Debug, Clone)]
pub struct DML {
    pub raw: String,
}

impl DML {
    pub fn new(raw: String) -> Self {
        DML { raw }
    }
}