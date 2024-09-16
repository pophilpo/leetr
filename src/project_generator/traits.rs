use crate::errors::TestGenerationError;
pub trait ProjectGenerator {
    fn fix_code_snippet(&self);
    fn extract_test_types(&self);
    fn generate_tests(&self) -> Result<String, TestGenerationError>;
}
