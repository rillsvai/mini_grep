mod config;
mod search;

pub use config::*;
use search::*;

use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path())?;

    let result = match config.ignore_case() {
        true => search_case_insensitive(config.query(), &file_content),
        _ => search(config.query(), &file_content),
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::search_case_insensitive;

    use super::search;

    #[test]
    fn search_returns_one_line() {
        let query = "duct";
        let contents = "/
Hello my dear.
   I have some expensive products.
DuCt please!";

        let result = search(query, contents);

        assert_eq!(vec!["   I have some expensive products."], result);
    }

    #[test]
    fn search_case_insensitive_two_lines() {
        let query: &str = "DucT";
        let contents = "/
Hello my dear.
  I have some expensive products.
give me ProDUcT please!";

        let result = search_case_insensitive(query, contents);

        assert_eq!(
            vec![
                "  I have some expensive products.",
                "give me ProDUcT please!"
            ],
            result
        );
    }
}
