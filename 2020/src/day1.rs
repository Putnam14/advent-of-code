use aoc_runner_derive::{aoc, aoc_generator};

const TOTAL_TARGET: u32 = 2020;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<u32> {
    let mut output: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    output.sort();
    output
}

#[aoc(day1, part1)]
fn part1(expenses: &[u32]) -> u32 {
    let mut target;
    for(i, x) in expenses.iter().enumerate() {
        if x > &TOTAL_TARGET {
            continue;
        }
        target = TOTAL_TARGET - x;
        for y in expenses[i..].iter() {
            if y == &target {
                return x * y
            }
        }
    }
    0
}

#[aoc(day1, part2)]
fn part2(expenses: &[u32]) -> u32 {
    let mut target;
    for (i, x) in expenses.iter().enumerate() {
        if x > &TOTAL_TARGET {
            continue;
        }
        target = &TOTAL_TARGET - x;
        for (j, y) in expenses[i..].iter().enumerate() {
            if y > &target {
                continue;
            }
            target = target - y;
            for z in expenses[j..].iter() {
                if z == &target {
                    return x * y * z
                }
            }
            target = target + y
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[1010,1010], 1_020_100)]
    fn part1_test(input: &[u32], expected: u32) {
        assert_eq!(part1(input), expected);
    }

    #[test_case(&[10,1000,1010], 10_100_000)]
    fn part2_test(input: &[u32], expected: u32) {
        assert_eq!(part2(input), expected);
    }
}
