use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Mutex;

fn main() {
    p1();
}

fn p1() {
    let filename = "input_1.txt";
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut signals: HashMap<String, Command> = HashMap::new();

    for line in lines {
        let s = line.as_ref().unwrap().as_str();
        let command = parse_command(s);
        signals.insert(command.output.to_owned(), command);
    }
    let signal = calculate_signal("a", &signals);
    println!("{}", signal)
}

fn parse_command(cmd: &str) -> Command {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(?P<arg_1>[a-z|\d]+)?\s?(?P<cmd>AND|OR|NOT|LSHIFT|RSHIFT)?\s?(?P<arg_2>[a-z|\d]+)? -> (?P<output>\w+)$",
        )
        .unwrap();
    }

    let captures = RE.captures(cmd).unwrap();

    return Command {
        command_type: match captures.name("cmd") {
            Some(v) => match v.as_str() {
                "AND" => CommandType::And,
                "OR" => CommandType::Or,
                "NOT" => CommandType::Not,
                "LSHIFT" => CommandType::LShift,
                "RSHIFT" => CommandType::RShift,
                _ => panic!("Invalid command"),
            },
            None => CommandType::Assign,
        },
        arg_1: match captures.name("arg_1") {
            Some(v) => Some(v.as_str().to_string()),
            None => None,
        },
        arg_2: match captures.name("arg_2") {
            Some(v) => Some(v.as_str().to_string()),
            None => None,
        },
        output: captures.name("output").unwrap().as_str().to_string(),
    };
}

fn calculate_signal(signal: &str, signals: &HashMap<String, Command>) -> u16 {
    lazy_static! {
        static ref CACHE: Mutex<HashMap<String, u16>> = Mutex::new(HashMap::new());
    }

    if CACHE.lock().unwrap().contains_key(signal) {
        return CACHE.lock().unwrap()[signal];
    }

    let command = &signals[signal];
    let arg_1: Option<u16>;
    let arg_2: Option<u16>;
    match &command.arg_1 {
        None => arg_1 = None,
        Some(s) => match s.parse::<u16>() {
            Ok(v) => arg_1 = Some(v),
            Err(_e) => arg_1 = Some(calculate_signal(s.as_str(), signals)),
        },
    }
    match &command.arg_2 {
        None => arg_2 = None,
        Some(s) => match s.parse::<u16>() {
            Ok(v) => arg_2 = Some(v),
            Err(_e) => arg_2 = Some(calculate_signal(s.as_str(), signals)),
        },
    }
    let result = execute_command(command.command_type, arg_1, arg_2);
    CACHE.lock().unwrap().insert(signal.to_string(), result);
    return result;
}

fn execute_command(command_type: CommandType, arg_1: Option<u16>, arg_2: Option<u16>) -> u16 {
    match command_type {
        CommandType::Assign => {
            return arg_1.unwrap();
        }
        CommandType::And => {
            return arg_1.unwrap() & arg_2.unwrap();
        }
        CommandType::Or => {
            return arg_1.unwrap() | arg_2.unwrap();
        }
        CommandType::Not => {
            return !arg_2.unwrap();
        }
        CommandType::LShift => {
            return arg_1.unwrap() << arg_2.unwrap();
        }
        CommandType::RShift => {
            return arg_1.unwrap() >> arg_2.unwrap();
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Command {
    command_type: CommandType,
    arg_1: Option<String>,
    arg_2: Option<String>,
    output: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CommandType {
    Assign,
    And,
    Or,
    Not,
    RShift,
    LShift,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_assign_1() {
        let c = parse_command("123 -> x");
        assert_eq!(
            c,
            Command {
                command_type: CommandType::Assign,
                arg_1: Some("123".to_string()),
                arg_2: None,
                output: "x".to_string()
            }
        );
    }
    #[test]
    fn test_parse_command_assign_2() {
        let c = parse_command("456 -> y");
        assert_eq!(
            c,
            Command {
                command_type: CommandType::Assign,
                arg_1: Some("456".to_string()),
                arg_2: None,
                output: "y".to_string()
            }
        );
    }
    #[test]
    fn test_parse_command_and() {
        let c = parse_command("x AND y -> d");
        assert_eq!(
            c,
            Command {
                command_type: CommandType::And,
                arg_1: Some("x".to_string()),
                arg_2: Some("y".to_string()),
                output: "d".to_string()
            }
        );
    }
    #[test]
    fn test_parse_command_or() {
        let c = parse_command("x OR y -> e");
        assert_eq!(
            c,
            Command {
                command_type: CommandType::Or,
                arg_1: Some("x".to_string()),
                arg_2: Some("y".to_string()),
                output: "e".to_string()
            }
        );
    }
    #[test]
    fn test_parse_command_lshift() {
        let c = parse_command("x LSHIFT 2 -> f");
        assert_eq!(
            c,
            Command {
                command_type: CommandType::LShift,
                arg_1: Some("x".to_string()),
                arg_2: Some("2".to_string()),
                output: "f".to_string()
            }
        );
    }
    #[test]
    fn test_parse_command_rshift() {
        let c = parse_command("y RSHIFT 2 -> g");
        assert_eq!(
            c,
            Command {
                command_type: CommandType::RShift,
                arg_1: Some("y".to_string()),
                arg_2: Some("2".to_string()),
                output: "g".to_string()
            }
        );
    }
    #[test]
    fn test_parse_command_not_1() {
        let c = parse_command("NOT x -> h");
        assert_eq!(
            c,
            Command {
                command_type: CommandType::Not,
                arg_1: None,
                arg_2: Some("x".to_string()),
                output: "h".to_string()
            }
        );
    }
    #[test]
    fn test_parse_command_not_2() {
        let c = parse_command("NOT y -> i");
        assert_eq!(
            c,
            Command {
                command_type: CommandType::Not,
                arg_1: None,
                arg_2: Some("y".to_string()),
                output: "i".to_string()
            }
        );
    }
    #[test]
    fn test_calculate_signal() {
        let mut signals: HashMap<String, Command> = HashMap::new();
        signals.insert(
            "x".to_string(),
            Command {
                command_type: CommandType::Assign,
                arg_1: Some("123".to_string()),
                arg_2: None,
                output: "x".to_string(),
            },
        );
        signals.insert(
            "y".to_string(),
            Command {
                command_type: CommandType::Assign,
                arg_1: Some("456".to_string()),
                arg_2: None,
                output: "y".to_string(),
            },
        );
        signals.insert(
            "d".to_string(),
            Command {
                command_type: CommandType::And,
                arg_1: Some("x".to_string()),
                arg_2: Some("y".to_string()),
                output: "d".to_string(),
            },
        );
        signals.insert(
            "e".to_string(),
            Command {
                command_type: CommandType::Or,
                arg_1: Some("x".to_string()),
                arg_2: Some("y".to_string()),
                output: "e".to_string(),
            },
        );
        signals.insert(
            "f".to_string(),
            Command {
                command_type: CommandType::LShift,
                arg_1: Some("x".to_string()),
                arg_2: Some("2".to_string()),
                output: "f".to_string(),
            },
        );
        signals.insert(
            "g".to_string(),
            Command {
                command_type: CommandType::RShift,
                arg_1: Some("y".to_string()),
                arg_2: Some("2".to_string()),
                output: "g".to_string(),
            },
        );
        signals.insert(
            "h".to_string(),
            Command {
                command_type: CommandType::Not,
                arg_1: None,
                arg_2: Some("x".to_string()),
                output: "h".to_string(),
            },
        );
        signals.insert(
            "i".to_string(),
            Command {
                command_type: CommandType::Not,
                arg_1: None,
                arg_2: Some("y".to_string()),
                output: "i".to_string(),
            },
        );
        assert_eq!(calculate_signal("d", &signals), 72);
        assert_eq!(calculate_signal("e", &signals), 507);
        assert_eq!(calculate_signal("f", &signals), 492);
        assert_eq!(calculate_signal("g", &signals), 114);
        assert_eq!(calculate_signal("h", &signals), 65412);
        assert_eq!(calculate_signal("i", &signals), 65079);
        assert_eq!(calculate_signal("x", &signals), 123);
        assert_eq!(calculate_signal("y", &signals), 456);
    }
}
