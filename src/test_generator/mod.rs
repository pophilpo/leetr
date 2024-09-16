pub mod rust_generator;
pub mod traits; // If implemented

use crate::errors::TestGenerationError;
use crate::test_generator::rust_generator::RustTestGenerator;
use crate::test_generator::traits::TestGenerator; // If implemented

pub fn get_test_generator(language: &str) -> Result<Box<dyn TestGenerator>, TestGenerationError> {
    match language.to_lowercase().as_str() {
        "rust" => Ok(Box::new(RustTestGenerator)),
        _ => Err(TestGenerationError::InvalidTestCaseFormatError(format!(
            "Unsupported language: {}",
            language
        ))),
    }
}
