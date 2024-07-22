use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use colored::*;
use log::info;
use strum::VariantNames;

use crate::errors::{GetResponseError, ProjectGeneratorError};
use crate::html;
use crate::project_templates::{PYTHON_TEMPLATE, RUST_TEMPLATE};
use crate::queries;
use crate::response_types::{ContentResponse, EditorResponse, ProblemSetResponse, Response};

#[derive(strum_macros::Display, strum_macros::VariantNames)]
pub enum ProjectType {
    Rust(String),
    Python3(String),
}

impl TryFrom<String> for ProjectType {
    type Error = ProjectGeneratorError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_ascii_lowercase().as_str() {
            "rust" => Ok(Self::Rust("Rust".to_string())),
            "python" => Ok(Self::Python3("Python3".to_string())),
            other => Err(ProjectGeneratorError::LanguageSupportIsNotAvailable(
                other.to_string(),
                ProjectType::VARIANTS.join(", "),
            )),
        }
    }
}

pub struct Generator {
    lang: ProjectType,
    project_title: String,
    directory: String,
}

impl Generator {
    pub fn new(
        lang: String,
        project_title: String,
        dir: Option<String>,
    ) -> Result<Self, ProjectGeneratorError> {
        let directory = match dir {
            Some(d) => d,
            None => project_title.clone(),
        };

        if fs::metadata(&directory).map_or(false, |metadata| metadata.is_dir()) {
            return Err(ProjectGeneratorError::DirectoryExists(directory));
        }

        let lang = ProjectType::try_from(lang)?;

        Ok(Self {
            lang,
            project_title,
            directory,
        })
    }

    pub fn generate_project(&self) -> Result<(), ProjectGeneratorError> {
        self.init()?;
        let content = self.get_problem_content()?;

        html::generate_markdown(self.project_title.clone(), &content, self.directory.clone())?;

        info!(
            "{}",
            format!(
                "🌟 LeetCode project was created in dir {} with language {}",
                &self.directory.bold(),
                &self.lang
            )
            .green()
        );

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_problem_set(&self) -> Result<Response, GetResponseError> {
        let query = queries::GraphQLPayload::problem_set_query();
        query.get_response::<ProblemSetResponse>()
    }

    fn get_problem_content(&self) -> Result<String, ProjectGeneratorError> {
        let query = queries::GraphQLPayload::content_query(self.project_title.clone());
        let response = query.get_response::<ContentResponse>()?;

        // FIXME: pass lang heere
        let content = response.get_content(String::from("rust"));
        Ok(content.unwrap())
    }

    fn get_editor_code(&self, project_lang: String) -> Result<String, ProjectGeneratorError> {
        let query = queries::GraphQLPayload::editor_data_query(self.project_title.clone());
        let response = query.get_response::<EditorResponse>()?;
        let content = response.get_content(project_lang);

        Ok(content.unwrap())
    }

    fn init(&self) -> Result<(), ProjectGeneratorError> {
        match &self.lang {
            ProjectType::Rust(lang) => {
                let code = self.get_editor_code(lang.to_string())?;

                // Insert todo inside the code;
                let insert_str = r#"todo!("Implement a solution");"#;

                let string_to_find = format!("fn {}", &self.project_title.replace('-', "_"));
                let new_code = if let Some(fn_pos) = code.find(&string_to_find) {
                    if let Some(brace_pos) = code[fn_pos..].find('{') {
                        let insert_pos = fn_pos + brace_pos + 2;
                        format!(
                            "{}        {}\n{}",
                            &code[..insert_pos],
                            insert_str,
                            &code[insert_pos..]
                        )
                    } else {
                        code
                    }
                } else {
                    code
                };

                let new_code = RUST_TEMPLATE.replace("{solution code}", &new_code);
                Command::new("cargo")
                    .arg("--quiet")
                    .arg("new")
                    .arg(&self.directory)
                    .status()?;

                let path: PathBuf = [&self.directory, "src", "main.rs"].iter().collect();

                let mut file = File::create(path)?;

                Ok(file.write_all(new_code.as_bytes())?)
            }

            ProjectType::Python3(lang) => {
                let code = self.get_editor_code(lang.to_string())?;
                let new_code = PYTHON_TEMPLATE.replace("{solution code}", &code);

                fs::create_dir(&self.directory)?;

                let path: PathBuf = [&self.directory, "main.py"].iter().collect();

                let mut file = File::create(path)?;

                Ok(file.write_all(new_code.as_bytes())?)
            }
        }
    }
}
