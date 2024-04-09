use regex::{escape, Captures, Regex};

const ANSI_RED: &str = "\x1b[31m";
const ANSI_DEFAULT: &str = "\x1b[0m";

pub fn highlight<'a>(line: &'a str, query: &str, color: &str) -> String {
    let regex = Regex::new(&format!(r"(?i){}", escape(query))).expect("regex constructing failed:");

    let result = regex.replace_all(line, |caps: &Captures| {
        format!("{color}{}{ANSI_DEFAULT}", &caps[0])
    });

    result.into_owned()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let lowercase_query = query.trim();

    contents
        .lines()
        .filter(|line| line.contains(lowercase_query))
        .map(|line| highlight(line, query, ANSI_RED))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let lowercase_query = query.trim().to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&lowercase_query))
        .map(|line| highlight(line, &query, ANSI_RED))
        .collect()
}
