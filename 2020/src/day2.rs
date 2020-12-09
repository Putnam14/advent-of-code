use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}


#[aoc(day2, part1)]
fn part1(passwords: &[String]) -> u32 {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): (\w+)").unwrap();
    let mut valid_passwords: u32 = 0;
    for x in passwords.iter() {
        let cap = re.captures(x).unwrap();
        let min = &cap[1].parse::<usize>().unwrap();
        let max = &cap[2].parse::<usize>().unwrap();
        let c = &cap[3];
        let pw = &cap[4];
        let count = pw.matches(c).count();
        if &count >= &min && &count <= &max {
            valid_passwords = &valid_passwords + 1;
        }
    }
    valid_passwords
}


#[aoc(day2, part2)]
fn part2(passwords: &[String]) -> u32 {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut valid_passwords = 0;
    for x in passwords.iter() {
        let cap = re.captures(x).unwrap();
        let first_index: usize = cap[1].parse().unwrap();
        let second_index: usize = cap[2].parse().unwrap();
        let character = &cap[3].parse::<char>().unwrap();
        let password = &cap[4];
        let mut count = 0;
        if password.as_bytes()[first_index - 1] as char == *character {
            count = &count + 1;
        }
        if password.as_bytes()[second_index - 1] as char == *character {
            count = &count + 1;
        }
        if count == 1 {
            valid_passwords = &valid_passwords + 1;
        }
    }
    valid_passwords
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&["1-2 a: aab".to_string(),"1-2 a: aaa".to_string()], 1)]
    fn part1_test(input: &[String], expected: u32) {
        assert_eq!(part1(input), expected);
    }

    #[test_case(&["1-2 a: aab".to_string(),"1-2 a: bac".to_string()], 1)]
    fn part2_test(input: &[String], expected: u32) {
        assert_eq!(part2(input), expected);
    }
}