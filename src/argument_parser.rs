use clap::{command, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches {
    let problem_url_arg = Arg::new("problem_url")
        .required(true)
        .short('u')
        .long("url")
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
