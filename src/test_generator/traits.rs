use crate::errors::TestGenerationError;
pub trait TestGenerator {
    fn generate_tests(
        &self,
        code_snippet: &str,
        test_cases: &[String],
    ) -> Result<String, TestGenerationError>;
}
