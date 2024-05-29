use crate::errors::ProjectGeneratorError;
use clap::{command, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches {
    let problem_url_arg = Arg::new("problem_url")
        .required(true)
        .help("Leetcode problem url");

    let directory_arg = Arg::new("directory")
        .required(false)
        .short('d')
        .long("dir")
        .help("Create a directory with custom name for the problem");

    let language_arg = Arg::new("language")
        .required(false)
        .short('l')
        .long("lang")
        .default_value("rust")
        .help("Programming language used to setup the project");

    command!()
        .arg(problem_url_arg)
        .arg(directory_arg)
        .arg(language_arg)
        .get_matches()
}

pub fn get_title(input: String) -> Result<String, ProjectGeneratorError> {
    if input.contains("https://leetcode.com/problems") {
        input
            .trim_end_matches('/')
            .rsplit("https://leetcode.com/problems")
            .next()
            .and_then(|s| s.split('/').nth(1))
            .map(|title| title.to_string())
            .ok_or_else(|| {
                ProjectGeneratorError::TitleExtractionError(format!(
                    "Could not extract title from {}",
                    input
                ))
            })
    } else {
        Ok(input)
    }
}
#[cfg(test)]
mod tests {

    use super::get_title;

    #[test]
    fn test_get_title() {
        let input = String::from("https://leetcode.com/problems/two-sum/description/");
        let expected = String::from("two-sum");

        assert_eq!(get_title(input).unwrap(), expected);

        let input = String::from("two-sum");

        assert_eq!(get_title(input.clone()).unwrap(), input);

        let input = String::from("https://leetcode.com/problems/two-sum");
        let expected = String::from("two-sum");

        assert_eq!(get_title(input).unwrap(), expected);
    }

    #[test]
    fn test_get_title_fails() {
        let input = String::from("https://leetcode.com/problems/");
        assert!(get_title(input).is_err());
    }
}
