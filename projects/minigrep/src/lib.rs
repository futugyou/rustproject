use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(args) => args,
            None => return Err("not have query"),
        };
        let filename = match args.next() {
            Some(args) => args,
            None => return Err("not have file"),
        };
        let case_sensitive = env::var("case_sensitive").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    // println!("content text \n:{}", content);
    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|l| l.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line.trim());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "admiring";
        let content = "\
        How public like a Frog
        To tell your name the livelong June 
        To an admiring Bog!";

        assert_eq!(vec!["To an admiring Bog!"], search(query, content));
    }
    #[test]
    fn case_insensitive() {
        let query = "Admiring";
        let content = "\
        How public like a Frog
        To tell your name the livelong June 
        To an admiring Bog!";

        assert_eq!(
            vec!["To an admiring Bog!"],
            search_case_insensitive(query, content)
        );
    }
}
