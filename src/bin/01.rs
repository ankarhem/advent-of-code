advent_of_code::solution!(1);

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

pub fn part_two(input: &str) -> Option<u32> {
    let new_input = input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    part_one(&new_input)
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
