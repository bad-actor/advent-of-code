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
    let mut floor = 0;
    for line in lines {
        let update = follow_directions(line.unwrap(), None);
        floor += update.0;
    }
    println!("{}", floor)
}

fn p2() {
    let filename = "input_1.txt";
    let stop_floor = -1;
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut floor = 0;
    let mut char_index: usize = 0;
    for line in lines {
        let update = follow_directions(line.unwrap(), Some(stop_floor));
        floor += update.0;
        char_index = update.1;

        if floor == stop_floor {
            break;
        }
    }
    println!("{} (index = {})", floor, char_index)
}

fn follow_directions(directions: String, stop_floor: Option<i32>) -> (i32, usize) {
    let mut d = 0;
    let mut i: usize = 0;
    for (ci, c) in directions.chars().enumerate() {
        if c == '(' {
            d += 1;
        } else if c == ')' {
            d -= 1;
        }

        i = ci + 1;

        if !stop_floor.is_none() && d == stop_floor.unwrap() {
            break;
        }
    }
    return (d, i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_1() {
        assert_eq!(0, follow_directions("(())".to_string(), None).0);
        assert_eq!(0, follow_directions("()()".to_string(), None).0);
    }

    #[test]
    fn test_p1_2() {
        assert_eq!(3, follow_directions("(((".to_string(), None).0);
        assert_eq!(3, follow_directions("(()(()(".to_string(), None).0);
        assert_eq!(3, follow_directions("))(((((".to_string(), None).0);
    }

    #[test]
    fn test_p1_3() {
        assert_eq!(-1, follow_directions("())".to_string(), None).0);
        assert_eq!(-1, follow_directions("))(".to_string(), None).0);
    }

    #[test]
    fn test_p1_4() {
        assert_eq!(-3, follow_directions(")))".to_string(), None).0);
    }

    #[test]
    fn test_p2_1() {
        assert_eq!((-1, 1), follow_directions(")".to_string(), Some(-1)));
    }

    #[test]
    fn test_p2_2() {
        assert_eq!((-1, 5), follow_directions("()())".to_string(), Some(-1)));
    }
}
