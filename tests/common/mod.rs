pub struct TestHelper;

impl TestHelper {
    pub fn get_valid_input() -> String {
        "a".to_string()
    }

    pub fn get_invalid_input() -> String {
        "9".to_string()
    }

    pub fn get_test_cases() -> Vec<(String, bool)> {
        vec![
            ("a".to_string(), true),
            ("b".to_string(), true),
            ("9".to_string(), false),
            ("!".to_string(), false),
        ]
    }
}
