pub mod dsl;

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// Helper para carregar fixtures de fragmentos (usa a mesma implementação do tests/common/fixtures.rs)
    pub fn load_fragment(relative_path: &str) -> String {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let fixture_path = Path::new(project_root)
            .join("tests/fixtures/fragments")
            .join(relative_path);

        std::fs::read_to_string(&fixture_path)
            .unwrap_or_else(|_| panic!("Failed to read fixture file: {}", fixture_path.display()))
    }
}
