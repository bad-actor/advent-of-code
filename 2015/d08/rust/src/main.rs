use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    p1();
    p2();
}

fn p1() {
    let filename = "input_1.txt";
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut code_chars = 0;
    let mut string_chars = 0;

    for line in lines {
        let s = line.as_ref().unwrap().as_str();
        code_chars += count_code_chars(s);
        string_chars += count_string_chars(s);
    }
    println!("{}", code_chars - string_chars)
}

fn p2() {
    let filename = "input_1.txt";
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut code_chars = 0;
    let mut encoded_chars = 0;

    for line in lines {
        let s = line.as_ref().unwrap().as_str();
        code_chars += count_code_chars(s);
        let encoded_str = encode_string(s);
        encoded_chars += count_code_chars(encoded_str.as_str());
    }
    println!("{}", encoded_chars - code_chars)
}

fn count_code_chars(s: &str) -> i32 {
    return s.chars().count().try_into().unwrap();
}

fn count_string_chars(s: &str) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(\\x[\da-f]{2}|\\"|\\\\|[^"])+?"#).unwrap();
    }
    return RE.captures_iter(s).count().try_into().unwrap();
}

fn encode_string(s: &str) -> String {
    let mut encoded_string = s.replace("\\", r"\\");
    encoded_string = encoded_string.replace("\"", r#"\""#);
    return format!("\"{}\"", encoded_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_code_chars() {
        assert_eq!(count_code_chars(r#""""#), 2);
        assert_eq!(count_code_chars(r#""abc""#), 5);
        assert_eq!(count_code_chars(r#""aaa\"aaa""#), 10);
        assert_eq!(count_code_chars(r#""\x27""#), 6);
    }

    #[test]
    fn test_count_string_chars() {
        assert_eq!(count_string_chars(r#""""#), 0);
        assert_eq!(count_string_chars(r#""abc""#), 3);
        assert_eq!(count_string_chars(r#""aaa\"aaa""#), 7);
        assert_eq!(count_string_chars(r#""\x27""#), 1);
    }

    #[test]
    fn test_encode_string() {
        assert_eq!(encode_string(r#""""#), r#""\"\"""#.to_string());
        assert_eq!(encode_string(r#""abc""#), r#""\"abc\"""#.to_string());
        assert_eq!(
            encode_string(r#""aaa\"aaa""#),
            r#""\"aaa\\\"aaa\"""#.to_string()
        );
        assert_eq!(encode_string(r#""\x27""#), r#""\"\\x27\"""#.to_string());
    }
}
