use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    p1();
}

fn p1() {
    let filename = "input_1.txt";
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut location: (i32, i32) = (0, 0);
    let mut houses = HashSet::from([location]);
    for line in lines {
        let updates = visit_houses(location, line.unwrap());
        location = updates.0;
        houses.extend(updates.1);
    }
    println!("{}", houses.len())
}

fn visit_houses(start: (i32, i32), directions: String) -> ((i32, i32), HashSet<(i32, i32)>) {
    let mut location = start;
    let mut houses = HashSet::new();
    for d in directions.chars() {
        if d == '<' {
            location.0 -= 1;
        } else if d == '>' {
            location.0 += 1;
        } else if d == '^' {
            location.1 += 1;
        } else if d == 'v' {
            location.1 -= 1;
        }

        houses.insert(location);
    }

    return (location, houses);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let (location, houses) = visit_houses((0, 0), ">".to_string());
        assert_eq!((1, 0), location);
        assert_eq!(1, houses.len());

        let (location, houses) = visit_houses((0, 0), "^>v<".to_string());
        assert_eq!((0, 0), location);
        assert_eq!(4, houses.len());

        let (location, houses) = visit_houses((0, 0), "^v^v^v^v^v".to_string());
        assert_eq!((0, 0), location);
        assert_eq!(2, houses.len());
    }
}
