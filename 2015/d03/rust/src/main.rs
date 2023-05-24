use std::collections::HashSet;
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
    let mut location: (i32, i32) = (0, 0);
    let mut houses = HashSet::from([location]);
    for line in lines {
        let directions = line.unwrap();
        for d in directions.chars() {
            location = update_location(location, d);
            houses.insert(location);
        }
    }
    println!("{}", houses.len());
}

fn p2() {
    let filename = "input_1.txt";
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut santa: (i32, i32) = (0, 0);
    let mut robot: (i32, i32) = (0, 0);
    let mut houses = HashSet::from([santa, robot]);
    for line in lines {
        let directions = line.unwrap();
        for (i, d) in directions.chars().enumerate() {
            if i % 2 == 0 {
                santa = update_location(santa, d);
                houses.insert(santa);
            } else {
                robot = update_location(robot, d);
                houses.insert(robot);
            }
        }
    }
    println!("{}", houses.len());
}

fn update_location(start: (i32, i32), direction: char) -> (i32, i32) {
    let mut location = start;
    if direction == '<' {
        location.0 -= 1;
    } else if direction == '>' {
        location.0 += 1;
    } else if direction == '^' {
        location.1 += 1;
    } else if direction == 'v' {
        location.1 -= 1;
    }
    return location;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_location() {
        let (location, houses) = visit_houses((0, 0), ">".to_string(), 1);
        assert_eq!((1, 0), location);
        assert_eq!(1, houses.len());

        let (location, houses) = visit_houses((0, 0), "^>v<".to_string(), 1);
        assert_eq!((0, 0), location);
        assert_eq!(4, houses.len());

        let (location, houses) = visit_houses((0, 0), "^v^v^v^v^v".to_string(), 1);
        assert_eq!((0, 0), location);
        assert_eq!(2, houses.len());
    }
}
