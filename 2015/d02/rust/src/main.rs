use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    p1();
}

fn p1() {
    let filename = "input_1.txt";
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut total = 0;
    for line in lines {
        let dimensions = line.unwrap();
        let mut iter = dimensions.splitn(3, 'x');
        let length: i32 = iter.next().unwrap().parse().unwrap();
        let width: i32 = iter.next().unwrap().parse().unwrap();
        let height: i32 = iter.next().unwrap().parse().unwrap();
        total += calculate_area(length, width, height)
    }
    println!("{}", total)
}

fn calculate_area(length: i32, width: i32, height: i32) -> i32 {
    let side_1 = length * width;
    let side_2 = width * height;
    let side_3 = length * height;
    let area = 2 * side_1 + 2 * side_2 + 2 * side_3;
    let extra = [side_1, side_2, side_3]
        .into_iter()
        .reduce(i32::min)
        .unwrap();
    return area + extra;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_1() {
        assert_eq!(58, calculate_area(2, 3, 4));
        assert_eq!(43, calculate_area(1, 1, 10));
    }
}
