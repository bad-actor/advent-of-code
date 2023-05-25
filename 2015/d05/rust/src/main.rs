use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    p1();
}

fn p1() {
    let filename = "input_1.txt";
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut count = 0;
    for line in lines {
        if is_nice(line.unwrap().as_str()) {
            count += 1;
        }
    }
    println!("{}", count)
}

fn is_nice(s: &str) -> bool {
    return count_vowels(s) > 2
        && count_longest_streak(s) > 1
        && !contains_substring(s, &["ab", "cd", "pq", "xy"]);
}

fn count_vowels(s: &str) -> i32 {
    let mut count = 0;
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                count += 1;
            }
            _ => {}
        }
    }
    return count;
}

fn count_longest_streak(s: &str) -> i32 {
    let mut streak = 0;
    let mut longest = 0;
    let mut previous: Option<char> = None;
    for c in s.chars() {
        match previous {
            None => {
                streak = 1;
            }
            Some(p) => {
                if p == c {
                    streak += 1;
                } else {
                    streak = 1;
                }

                if streak > longest {
                    longest = streak;
                }
            }
        }
        previous = Some(c);
    }
    return longest;
}

fn contains_substring(s: &str, substrings: &[&str]) -> bool {
    for ss in substrings {
        if s.contains(ss) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowels() {
        assert_eq!(3, count_vowels("ugknbfddgicrmopn"));
        assert_eq!(3, count_vowels("aaa"));
        assert_eq!(1, count_vowels("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_count_longest_streak() {
        assert_eq!(2, count_longest_streak("ugknbfddgicrmopn"));
        assert_eq!(3, count_longest_streak("aaa"));
        assert_eq!(1, count_longest_streak("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_contains_substring() {
        assert!(!contains_substring(
            "ugknbfddgicrmopn",
            &["ab", "cd", "pq", "xy"]
        ));
        assert!(!contains_substring("aaa", &["ab", "cd", "pq", "xy"]));
        assert!(contains_substring(
            "haegwjzuvuyypxyu",
            &["ab", "cd", "pq", "xy"]
        ));
    }
}
