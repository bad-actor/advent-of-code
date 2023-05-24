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
    let mut santa: (i32, i32) = (0, 0);
    let mut houses = HashSet::from([santa]);
    for line in lines {
        let directions = line.unwrap();
        visit_houses(&mut [&mut santa], directions, &mut houses)
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
        visit_houses(&mut [&mut santa, &mut robot], directions, &mut houses)
    }
    println!("{}", houses.len());
}

fn visit_houses(
    location: &mut [&mut (i32, i32)],
    directions: String,
    houses: &mut HashSet<(i32, i32)>,
) {
    for (i, d) in directions.chars().enumerate() {
        update_location(location[i % location.len()], d);
        houses.insert(*location[i % location.len()]);
    }
}

fn update_location(location: &mut (i32, i32), direction: char) {
    match direction {
        '<' => {
            location.0 -= 1;
        }
        '>' => {
            location.0 += 1;
        }
        '^' => {
            location.1 += 1;
        }
        'v' => {
            location.1 -= 1;
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_1() {
        let mut santa = (0, 0);
        let mut houses = HashSet::from_iter([santa]);
        visit_houses(&mut [&mut santa], ">".to_string(), &mut houses);
        assert_eq!((1, 0), santa);
        assert_eq!(2, houses.len());
    }

    #[test]
    fn test_p1_2() {
        let mut santa = (0, 0);
        let mut houses = HashSet::from_iter([santa]);
        visit_houses(&mut [&mut santa], "^>v<".to_string(), &mut houses);
        assert_eq!((0, 0), santa);
        assert_eq!(4, houses.len());
    }

    #[test]
    fn test_p1_3() {
        let mut santa = (0, 0);
        let mut houses = HashSet::from_iter([santa]);
        visit_houses(&mut [&mut santa], "^v^v^v^v^v".to_string(), &mut houses);
        assert_eq!((0, 0), santa);
        assert_eq!(2, houses.len());
    }

    #[test]
    fn test_p2_1() {
        let mut santa = (0, 0);
        let mut robot = (0, 0);
        let mut houses = HashSet::from_iter([santa, robot]);
        visit_houses(&mut [&mut santa, &mut robot], "^v".to_string(), &mut houses);
        assert_eq!((0, 1), santa);
        assert_eq!((0, -1), robot);
        assert_eq!(3, houses.len());
    }

    #[test]
    fn test_p2_2() {
        let mut santa = (0, 0);
        let mut robot = (0, 0);
        let mut houses = HashSet::from_iter([santa, robot]);
        visit_houses(
            &mut [&mut santa, &mut robot],
            "^>v<".to_string(),
            &mut houses,
        );
        assert_eq!((0, 0), santa);
        assert_eq!((0, 0), robot);
        assert_eq!(3, houses.len());
    }

    #[test]
    fn test_p2_3() {
        let mut santa = (0, 0);
        let mut robot = (0, 0);
        let mut houses = HashSet::from_iter([santa, robot]);
        visit_houses(
            &mut [&mut santa, &mut robot],
            "^v^v^v^v^v".to_string(),
            &mut houses,
        );
        assert_eq!((0, 5), santa);
        assert_eq!((0, -5), robot);
        assert_eq!(11, houses.len());
    }
}
