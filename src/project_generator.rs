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

pub struct Generator {}
