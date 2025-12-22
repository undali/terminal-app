pub mod grep {
    use colored::Colorize;
    use std::env;
    use std::error::Error;
    use std::fs;

    fn print_color(text: &str, target: &str) {
        let lower = text.to_lowercase();
        let t = target.to_lowercase();

        if let Some(i) = lower.find(&t) {
            let j = i + target.len();
            print!("{}", &text[..i]);
            print!("{}", &text[i..j].green().bold());
            println!("{}", &text[j..]);
        } else {
            println!("{text}");
        }
    }

    pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(&config.filename)?;

        let results = if config.strict {
            search_strict(&contents, &config.query)
        } else {
            search_case_insensitive(&contents, &config.query)
        };

        for line in results {
            print_color(line, &config.query);
        }

        Ok(())
    }

    pub fn search_strict<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    pub fn search_case_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query))
            .collect()
    }

    pub struct Config {
        pub query: String,
        pub filename: String,
        pub strict: bool,
    }

    impl Config {
        pub fn new(args: &Vec<String>) -> Result<Config, &str> {
            if args.len() < 3 {
                Err("Not enough arguments!")
            } else {
                Ok(Config {
                    query: args[2].clone(),
                    filename: args[1].clone(),
                    strict: if let Ok(val) = env::var("STRICT") {
                        val == "1" || val == "true"
                    } else {
                        false
                    },
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grep::*;

    #[test]
    fn basic_search() {
        let data = "hello
how are you
today is tuesday
my name is tuhin
";

        assert_eq!(vec!["today is tuesday"], search_strict(data, "day"));
        assert_eq!(
            vec!["today is tuesday", "my name is tuhin"],
            search_strict(data, "is")
        )
    }

    #[test]
    fn check_search_strict() {
        let data = "hello
how are you
today is tuesday
my name is tuhin
every Day
It Is a ball
It IS a toy
";

        assert_eq!(vec!["It Is a ball"], search_strict(data, "Is"));
        assert_eq!(vec!["today is tuesday"], search_strict(data, "day"));
        assert_eq!(
            vec!["today is tuesday", "my name is tuhin"],
            search_strict(data, "is")
        )
    }

    #[test]
    fn check_search_case_insensitive() {
        let data = "hello
how are you
today is tuesday
my name is tuhin
";

        assert_eq!(
            vec!["today is tuesday"],
            search_case_insensitive(data, "Day")
        );
        assert_eq!(
            vec!["today is tuesday", "my name is tuhin"],
            search_case_insensitive(data, "iS")
        )
    }
}
