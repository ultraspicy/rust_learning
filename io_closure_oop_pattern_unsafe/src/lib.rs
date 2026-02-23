use std::error::Error;
use std::fs;
use std::vec;
use anyhow::Error as AnyErr;
use anyhow::anyhow;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }
    
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build (args: &[String]) -> Result<Config, AnyErr> {
        if args.len() < 3 {
            return Err(anyhow!("not enough arg"));
        }

        let query = &args[1];
        let file_path = &args[2];
        // if use explicitly set case sentivity, use it
        // otherwise, use env var `IGNORE_CASE`
        // default is true
        let mut ignore_case = true;
        if args.len() == 4 {
            ignore_case = args[3].parse()?;
        } else {
            match env::var("IGNORE_CASE") {
                Ok(v) => {ignore_case = v.parse()?},
                Err(v) => {
                    match v {
                        env::VarError::NotPresent => {eprintln!("IGNORE_CASE is not set either from input nor from environment var. Leave it as default true")},
                        env::VarError::NotUnicode(_) => {},
                        _ => {},
                    }
                },
            }
        }
        
        Ok(Config { 
            query: query.to_string(), 
            file_path: file_path.to_string(),
            ignore_case: ignore_case
         })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect::<Vec<&str>>()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

// trait object vs generics
// A generic type parameter can be substituted with only one concrete type at a time 
// whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime
// downside of using trait object 1) worse dev exp, need to be explicit about type for vec 
// 2) dynamic dispatch meaning worse runtime experience
pub struct Screen {
    pub component: Vec<Box<dyn Draw>>,
}

pub struct Button {
    pub length: usize,
    pub width: usize,
}

impl Draw for Button {
    fn draw(&self) {
        println!("the length of this button is {}, and width is {}", self.length, self.width);
    }
}

pub struct SelectBox {
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("the options for select box are {:?}", self.options);
    }
}

pub struct ScreenV2<T> where T: Draw {
    pub component: Vec<T>,
}

pub trait Draw {
    fn draw(&self);
}

impl Screen {
    pub fn draw(&self) {
        self.component.iter().for_each(|c| c.draw());
    }
}

pub fn build_ui() {
    let s = Screen {
        component:  vec![
            Box::new(Button {length: 1, width: 2}),
            Box::new(SelectBox {options: vec![String::from("op1"), String::from("op2")]}),
        ],
    };
    s.draw();
}

// state pattern