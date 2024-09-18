use crate::project_generator::metadata::Metadata;
use crate::project_generator::traits::ProjectGenerator;
use crate::{errors::ProjectGeneratorError, project_generator::example_types::ExampleType};

use quote::ToTokens;
use syn::{File, Item, ItemFn, ItemImpl, ReturnType};

use log::{debug, error, info, warn};

pub struct RustProjectGenerator {
    code_snippet: String,
    examples_string: Vec<String>,
    metadata: String,
}

impl ProjectGenerator for RustProjectGenerator {
    fn new(code_snippet: String, examples_string: Vec<String>, metadata: String) -> Self {
        Self {
            code_snippet,
            examples_string,
            metadata,
        }
    }

    fn parse_metadata(&self) -> Result<Metadata, ProjectGeneratorError> {
        let json: Metadata = serde_json::from_str(&self.metadata)?;
        Ok(json)
    }

    fn fix_code_snippet(&self) {
        todo!();
    }

    fn generate_tests(&self) {
        todo!();
    }
}
