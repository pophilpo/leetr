use crate::html;
use crate::config::Config;
use crate::response_types::ContentResponse;
use std::error;
use std::fs;
use std::process::Command;

use crate::project_templates;
use crate::queries;

#[derive(Debug)]
pub enum ProjectType {
    Rust,
}

impl From<String> for ProjectType {
    fn from(s: String) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "rust" => Self::Rust,
            _ => Self::Rust,
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

     pub async fn generate_project(&self) -> Result<(), Box<dyn error::Error>> {
        self.init()?;
        let content = self.get_problem_content().await?;
        html::generate_markdown(self.project_title.clone(), content)?;
        Ok(())
    }

    async fn get_problem_content(&self) -> Result<ContentResponse, Box<dyn error::Error>> {

        let query = queries::GraphQLPayload::content_query(self.project_title.clone());
        println!("{:?}", query);
        Ok(query.get_response().await?)

    }

    fn init(&self) -> Result<(), Box<dyn error::Error>> {

        match &self.config.default_lang {
            ProjectType::Rust => {

                Command::new("cargo").arg("new").arg(&self.project_title).status()?;

            }

            _ => unreachable!()

        }
        Ok(())

    }
}
