use rayon::prelude::*;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }
    let paths: Vec<String> = args.into_iter().skip(1).collect();

    // Zoom through the vec with rayon
    paths.par_iter().for_each(|path| {
        let path = Path::new(path);
        if path.is_file() {
            process_file(path);
        } else {
            println!("{} is not a file", path.display());
        }
    });
}

fn process_file(path: &Path) {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let formatted = format_sql(&contents);
    println!("{}", formatted);
}

fn format_sql(input: &str) -> String {
    // List of SQL keywords to uppercase
    let keywords = [
        "select", "from", "where", "insert", "update", "delete", "as", "order", "by", "group",
        "join", "having", "limit", "offset", "and", "or", "not", "in", "like", "is", "null",
        "true", "false", "between", "exists", "case", "when", "then", "else", "end", "distinct",
    ];

    // Split the input into words and process each word
    input
        .split_whitespace()
        .map(|word| {
            // Check if the word is a keyword, if so, uppercase it
            if keywords.contains(&word.to_lowercase().as_str()) {
                word.to_uppercase()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let query = "";
        assert_eq!(format_sql(query), "");
    }

    #[test]
    fn test_basic_keywords() {
        let query = "select * from users where id = 1";
        assert_eq!(format_sql(query), "SELECT * FROM users WHERE id = 1");
    }

    #[test]
    fn test_mixed_case_keywords() {
        let query = "SeLeCt * fRoM users where ID = 1";
        assert_eq!(format_sql(query), "SELECT * FROM users WHERE ID = 1");
    }

    #[test]
    fn test_no_keywords() {
        let query = "users * id = 1";
        assert_eq!(format_sql(query), "users * id = 1");
    }
}
