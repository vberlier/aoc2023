#![allow(dead_code)]

const INPUT: &str = include_str!("day01.txt");

fn parse_calibration(s: &str) -> Option<u32> {
    let first = s.chars().find_map(|c| c.to_digit(10))?;
    let last = s.chars().rev().find_map(|c| c.to_digit(10))?;
    Some(first * 10 + last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let solution = INPUT.lines().filter_map(parse_calibration).sum::<u32>();
        insta::assert_snapshot!(solution.to_string(), @"54573");
    }

    #[test]
    fn part2() {
        let solution = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        .fold(INPUT.to_owned(), |s, (i, d)| {
            s.replace(d, &format!("{d}{n}{d}", n = i + 1))
        })
        .lines()
        .filter_map(parse_calibration)
        .sum::<u32>();
        insta::assert_snapshot!(solution.to_string(), @"54591");
    }
}
