use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

fn main() {
    p1();
}

fn p1() {
    let filename = "input_1.txt";
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut lights = vec![vec![false; 1000]; 1000];
    let mut count = 0;

    for line in lines {
        let s = line.as_ref().unwrap().as_str();
        let command = parse_command(s);
        for light in iterate_over_patch(&mut lights, command.x, command.y) {
            execute_command(command.command_type, light);
        }
    }
    for light in iterate_over_patch(&mut lights, 0..=999, 0..=999) {
        if *light {
            count += 1
        }
    }
    println!("{}", count)
}

fn parse_command(cmd: &str) -> Command {
    lazy_static! {
        static ref RE: Regex = Regex::new(concat!(
            r"^(?P<cmd>turn on|turn off|toggle) (?P<x1>\d+),(?P<y1>\d+) ",
            r"through (?P<x2>\d+),(?P<y2>\d+)$"
        ))
        .unwrap();
    }

    let captures = RE.captures(cmd).unwrap();
    let x1 = captures.name("x1").unwrap().as_str().parse().unwrap();
    let x2 = captures.name("x2").unwrap().as_str().parse().unwrap();
    let y1 = captures.name("y1").unwrap().as_str().parse().unwrap();
    let y2 = captures.name("y2").unwrap().as_str().parse().unwrap();
    return Command {
        command_type: match captures.name("cmd").unwrap().as_str() {
            "turn on" => CommandType::TurnOn,
            "turn off" => CommandType::TurnOff,
            "toggle" => CommandType::Toggle,
            _ => panic!("Invalid command"),
        },
        x: min(x1, x2)..=max(x1, x2),
        y: min(y1, y2)..=max(y1, y2),
    };
}

fn iterate_over_patch<'a, T>(
    m: &'a mut Vec<Vec<T>>,
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
) -> impl Iterator<Item = &'a mut T> {
    return m[x].iter_mut().flat_map(move |r| r[y.clone()].iter_mut());
}

fn execute_command(command_type: CommandType, light: &mut bool) {
    match command_type {
        CommandType::TurnOn => {
            *light = true;
        }
        CommandType::TurnOff => {
            *light = false;
        }
        CommandType::Toggle => {
            *light = !*light;
        }
    }
}

struct Command {
    command_type: CommandType,
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum CommandType {
    TurnOn,
    TurnOff,
    Toggle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterate_over_matrix() {
        let mut m = vec![
            vec![false, false, true],
            vec![false, true, false],
            vec![true, true, false],
        ];
        let mut v = Vec::new();
        for e in iterate_over_patch(&mut m, 0..=1, 1..=2) {
            v.push(*e)
        }
        assert_eq!(v, [false, true, true, false])
    }

    #[test]
    fn test_parse_command() {
        let command = parse_command("turn on 0,0 through 999,999");
        assert_eq!(command.command_type, CommandType::TurnOn);
        assert_eq!(command.x, 0..=0);
        assert_eq!(command.y, 999..=999);

        let command = parse_command("toggle 0,0 through 999,0");
        assert_eq!(command.command_type, CommandType::Toggle);
        assert_eq!(command.x, 0..=0);
        assert_eq!(command.y, 999..=0);

        let command = parse_command("turn off 499,499 through 500,500");
        assert_eq!(command.command_type, CommandType::TurnOff);
        assert_eq!(command.x, 499..=499);
        assert_eq!(command.y, 500..=500);
    }

    #[test]
    fn test_execute_command_turn_on() {
        let mut m = vec![
            vec![false, false, true],
            vec![false, true, false],
            vec![true, true, false],
        ];
        execute_command(CommandType::TurnOn, &mut m[0][1]);
        assert_eq!(m[0][1], true);
        execute_command(CommandType::TurnOn, &mut m[0][2]);
        assert_eq!(m[0][2], true);
    }

    #[test]
    fn test_execute_command_turn_off() {
        let mut m = vec![
            vec![false, false, true],
            vec![false, true, false],
            vec![true, true, false],
        ];
        execute_command(CommandType::TurnOff, &mut m[0][1]);
        assert_eq!(m[0][1], false);
        execute_command(CommandType::TurnOff, &mut m[0][2]);
        assert_eq!(m[0][2], false);
    }

    #[test]
    fn test_execute_command_toggle() {
        let mut m = vec![
            vec![false, false, true],
            vec![false, true, false],
            vec![true, true, false],
        ];
        execute_command(CommandType::Toggle, &mut m[0][1]);
        assert_eq!(m[0][1], true);
        execute_command(CommandType::TurnOff, &mut m[0][2]);
        assert_eq!(m[0][2], false);
    }
    #[test]
    fn test_command_all_on() {
        let mut lights = vec![vec![false; 1000]; 1000];
        let command = parse_command("turn on 0,0 through 999,999");
        for light in iterate_over_patch(&mut lights, command.x, command.y) {
            execute_command(command.command_type, light);
        }
        let mut count = 0;
        for light in iterate_over_patch(&mut lights, 0..=999, 0..=999) {
            if *light {
                count += 1
            }
        }
        assert_eq!(count, 1000000);
    }
}
