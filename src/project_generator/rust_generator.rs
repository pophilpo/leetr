use crate::project_generator::metadata::Metadata;
use crate::project_generator::traits::ProjectGenerator;
use crate::{errors::ProjectGeneratorError, project_generator::example_types::ExampleType};
use convert_case::{Case, Casing};

use quote::ToTokens;
use syn::{File, Item, ItemFn, ItemImpl, ReturnType};

use log::{debug, error, info, warn};

use super::example_types::Example;

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

    fn generate_tests(&self, examples: Vec<Example>) -> String {
        let mut test_functions: Vec<String> = Vec::new();
        let rust_test_string = include_str!("code_snippets/rust_test.rs");
        for (fn_count, example) in examples.iter().enumerate() {
            let fn_name = example.fn_name.clone().to_case(Case::Snake);
            let output = example.output.value.to_string_rust();
            let input = example
                .inputs
                .iter()
                .map(|input| input.value.to_string_rust())
                .collect::<Vec<String>>()
                .join(", ");

            let function_call_string = format!("{}({})", fn_name, input);
            let assert_call_string = format!("assert_eq!({}, {});", function_call_string, output);

            let test_string = rust_test_string
                .replace("!fn_name!", &fn_name)
                .replace("!fn_count!", &fn_count.to_string())
                .replace("!assert!", &assert_call_string);

            test_functions.push(test_string);
        }
        let test_functions_string = test_functions.join("\n");
        let rust_test_mod_string = include_str!("code_snippets/rust_test_mod.rs");
        let result = rust_test_mod_string.replace("!tests!", &test_functions_string);

        result
    }
}
