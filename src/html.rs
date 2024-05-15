use crate::errors::GenerateMarkdownError;
use crate::response_types::ContentResponse;
use html2md;
use std::fs;

pub fn generate_markdown(
    question_title: String,
    content: ContentResponse,
) -> Result<(), GenerateMarkdownError> {
    let title = format_title(&question_title);
    let markdown_content = html2md::parse_html(&content.data.question.content);

    let full_markdown = format!("# {}\n\n{}", title, markdown_content);

    // TODO: Add save path as var

    let filename = format!(
        "{}/{}",
        question_title.to_lowercase(),
        String::from("README.md"),
    );
    Ok(fs::write(filename, full_markdown)?)
}

fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();

    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

fn format_title(s: &str) -> String {
    s.split('-')
        .map(capitalize_word)
        .collect::<Vec<String>>()
        .join(" ")
}
