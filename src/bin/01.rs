advent_of_code::solution!(1);

use cond::cond;

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum();

    Some(sum)
}

fn find_number(input: &str) -> Option<u32> {
    if let Some(number) = input.get(0..1).and_then(|c| c.parse::<u32>().ok()) {
        return Some(number);
    }

    let length = input.len();

    let number = cond! {
        length >= 3 && &input[0..3] == "one" => 1,
        length >= 3 && &input[0..3] == "two" => 2,
        length >= 5 && &input[0..5] == "three" => 3,
        length >= 4 && &input[0..4] == "four" => 4,
        length >= 4 && &input[0..4] == "five" => 5,
        length >= 3 && &input[0..3] == "six" => 6,
        length >= 5 && &input[0..5] == "seven" => 7,
        length >= 5 && &input[0..5] == "eight" => 8,
        length >= 4 && &input[0..4] == "nine" => 9,
        _ => {
            None?
        },
    };

    Some(number)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            (0..line.len())
                .filter_map(|idx| find_number(&line[idx..]))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_part_one() {
        let part_one_example = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let result = part_one(part_one_example);
        assert_eq!(result.unwrap(), 142);
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    // Extra case which can fuck you up
    #[case("oneight", 18)]
    fn test_part_two_lines(#[case] input: &str, #[case] expected: u32) {
        let result = part_two(input);
        assert_eq!(expected, result.unwrap())
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(281, result.unwrap());
    }
}
