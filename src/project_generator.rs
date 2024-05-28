use crate::config::Config;
use crate::html;
use crate::response_types::{ContentResponse, Response};
use std::process::Command;

use crate::errors::ProjectGeneratorError;
use crate::queries;

#[derive(Debug)]
pub enum ProjectType {
    Rust(String),
}

impl From<String> for ProjectType {
    fn from(s: String) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "rust" => Self::Rust("rust".to_string()),
            _ => Self::Rust("rust".to_string()),
        }
    }
}

pub struct Generator {
    config: Config,
    project_title: String,
}

impl Generator {
    pub fn new(config: Config, project_title: String) -> Self {
        Self {
            config,
            project_title,
        }
    }

    pub async fn generate_project(&self) -> Result<(), ProjectGeneratorError> {
        self.init()?;
        let content = self.get_problem_content().await?;
        html::generate_markdown(self.project_title.clone(), &content)?;
        Ok(())
    }

    async fn get_problem_content(&self) -> Result<String, ProjectGeneratorError> {
        let query = queries::GraphQLPayload::content_query(self.project_title.clone());
        let response = query.get_response().await?;

        // FIXME: pass lang heere
        //
        let content = response.get_content(String::from("rust"));
        Ok(content.unwrap())
    }

    fn init(&self) -> Result<(), ProjectGeneratorError> {
        match &self.config.default_lang {
            ProjectType::Rust(_) => {
                Command::new("cargo")
                    .arg("new")
                    .arg(&self.project_title)
                    .status()?;
            }

            _ => unreachable!(),
        }
        Ok(())
    }
}
