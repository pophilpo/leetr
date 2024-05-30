use crate::html;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use crate::errors::{GetResponseError, ProjectGeneratorError};
use crate::project_templates::{PYTHON_TEMPLATE, RUST_TEMPLATE};
use crate::queries;
use crate::response_types::Response;

pub enum ProjectType {
    Rust(String),
    Python3(String),
}

impl From<String> for ProjectType {
    fn from(s: String) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "rust" => Self::Rust("Rust".to_string()),
            "python" => Self::Python3("Python3".to_string()),
            _ => Self::Rust("Rust".to_string()),
        }
    }
}

pub struct Generator {
    lang: ProjectType,
    project_title: String,
    directory: String,
}

impl Generator {
    pub fn new(lang: String, project_title: String, dir: Option<String>) -> Self {
        let directory = match dir {
            Some(d) => d,
            None => project_title.clone(),
        };

        let lang = ProjectType::from(lang);

        Self {
            lang,
            project_title,
            directory,
        }
    }

    pub async fn generate_project(&self) -> Result<(), ProjectGeneratorError> {
        self.init().await?;
        let content = self.get_problem_content().await?;

        html::generate_markdown(self.project_title.clone(), &content, self.directory.clone())?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_problem_set(&self) -> Result<Response, GetResponseError> {
        let query = queries::GraphQLPayload::problem_set_query();
        query.get_response().await
    }

    async fn get_problem_content(&self) -> Result<String, ProjectGeneratorError> {
        let query = queries::GraphQLPayload::content_query(self.project_title.clone());
        let response = query.get_response().await?;

        // FIXME: pass lang heere
        let content = response.get_content(String::from("rust"));
        Ok(content.unwrap())
    }

    async fn get_editor_code(&self, project_lang: String) -> Result<String, ProjectGeneratorError> {
        let query = queries::GraphQLPayload::editor_data_query(self.project_title.clone());
        let response = query.get_response().await?;
        let content = response.get_content(project_lang);

        Ok(content.unwrap())
    }

    async fn init(&self) -> Result<(), ProjectGeneratorError> {
        match &self.lang {
            ProjectType::Rust(lang) => {
                let code = self.get_editor_code(lang.to_string()).await?;

                // Insert todo inside the code;
                let insert_str = r#"todo!("Implement a solution");"#;

                let string_to_find = format!("fn {}", &self.project_title.replace('-', "_"));
                let new_code = if let Some(fn_pos) = code.find(&string_to_find) {
                    if let Some(brace_pos) = code[fn_pos..].find('{') {
                        let insert_pos = fn_pos + brace_pos + 2; // +1 to insert right after '{'
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
                    .arg("new")
                    .arg(&self.directory)
                    .status()?;

                let path: PathBuf = [&self.directory, "src", "main.rs"].iter().collect();

                let mut file = File::create(path)?;

                Ok(file.write_all(new_code.as_bytes())?)
            }

            ProjectType::Python3(lang) => {
                let code = self.get_editor_code(lang.to_string()).await?;
                let new_code = PYTHON_TEMPLATE.replace("{solution code}", &code);

                fs::create_dir(&self.directory)?;

                let path: PathBuf = [&self.directory, "main.py"].iter().collect();

                let mut file = File::create(path)?;

                Ok(file.write_all(new_code.as_bytes())?)
            }
        }
    }
}
