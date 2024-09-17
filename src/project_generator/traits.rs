use crate::errors::ProjectGeneratorError;
use crate::project_generator::example_types::InputType;
use crate::project_generator::metadata::Metadata;
pub trait ProjectGenerator {
    fn new(code_snippet: String, examples_string: Vec<String>, metadata: String) -> Self;
    fn parse_metadata(&self) -> Result<Metadata, ProjectGeneratorError>;
    fn fix_code_snippet(&self);
    fn generate_tests(&self);
}
