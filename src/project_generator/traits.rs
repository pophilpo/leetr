use super::example_types::Example;
use crate::errors::ProjectGeneratorError;
use crate::project_generator::metadata::Metadata;

pub trait ProjectGenerator {
    fn new(code_snippet: String, examples_string: Vec<String>, metadata: String) -> Self;
    fn parse_metadata(&self) -> Result<Metadata, ProjectGeneratorError>;
    fn get_complete_code(&self, examples: Vec<Example>) -> String;
    fn generate_tests(&self, examples: Vec<Example>) -> String;
}
