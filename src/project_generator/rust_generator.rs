use crate::project_generator::metadata::Metadata;
use crate::project_generator::traits::ProjectGenerator;
use crate::{errors::ProjectGeneratorError, project_generator::example_types::ExampleType};
use convert_case::{Case, Casing};

use quote::ToTokens;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use log::{debug, error, info, warn};

use super::example_types::Example;

pub struct RustProjectGenerator {
    code_snippet: String,
    examples_string: Vec<String>,
    metadata: String,
    problem_title: String,
}

impl ProjectGenerator for RustProjectGenerator {
    fn new(
        code_snippet: String,
        examples_string: Vec<String>,
        metadata: String,
        problem_title: String,
    ) -> Self {
        Self {
            code_snippet,
            examples_string,
            metadata,
            problem_title,
        }
    }

    fn generate_project(
        &self,
        folder_name: Option<String>,
        html: String,
    ) -> Result<(), ProjectGeneratorError> {
        let folder_name = match folder_name {
            Some(name) => name,
            None => self.problem_title.clone(),
        };

        if Path::new(&folder_name).exists() {
            error!("Folder with name {} already exists.", folder_name);
        }

        let output = Command::new("cargo")
            .arg("new")
            .arg("--lib")
            .arg(&folder_name)
            .output()?;

        if output.status.success() {
            info!("Generated new rust project {}", folder_name);
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Error running cargo command: {}", error);
            exit(1);
        }

        let markdown_path: PathBuf = Path::new(&folder_name).join("README.md");
        let mut markdown = File::create(&markdown_path)?;

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(markdown_path)?;

        let title_header = format!("<h1>{}</h1>\n", self.problem_title.to_case(Case::Title));

        file.write_all(&title_header.as_bytes())?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    fn parse_metadata(&self) -> Result<Metadata, ProjectGeneratorError> {
        let json: Metadata = serde_json::from_str(&self.metadata)?;
        Ok(json)
    }

    fn get_complete_code(&self, examples: Vec<Example>) -> String {
        let mut lines = Vec::new();
        lines.push(String::from("struct Solution {}"));

        for line in self.code_snippet.lines() {
            if line.trim().contains("fn") {
                lines.push(line.to_string());
                lines.push(String::from("        todo!(\"Implement Solution\");"));
            } else {
                lines.push(line.to_string());
            }
        }

        let tests = self.generate_tests(examples);
        lines.push(tests);
        let result = lines.join("\n");
        result
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

            let function_call_string = format!("Solution::{}({})", fn_name, input);
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
