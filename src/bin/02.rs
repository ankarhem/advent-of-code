advent_of_code::solution!(2);

use lazy_static::lazy_static;
use regex::Regex;

struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

fn process_line(s: &str) -> Game {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Game (?<id>\d+):").unwrap();
    }
    let captures = RE.captures(s).expect("String to have id");
    let id = &captures["id"].parse::<u32>().expect("id to be a number");

    let reds = s
        .split(" red")
        .flat_map(|g| {
            g.char_indices()
                .nth_back(1)
                .and_then(|(idx, _)| g.get(idx..).and_then(|c| c.trim().parse::<u32>().ok()))
        })
        .max()
        .unwrap_or(0);
    let greens = s
        .split(" green")
        .flat_map(|g| {
            g.char_indices()
                .nth_back(1)
                .and_then(|(idx, _)| g.get(idx..).and_then(|c| c.trim().parse::<u32>().ok()))
        })
        .max()
        .unwrap_or(0);
    let blues = s
        .split(" blue")
        .flat_map(|g| {
            g.char_indices()
                .nth_back(1)
                .and_then(|(idx, _)| g.get(idx..).and_then(|c| c.trim().parse::<u32>().ok()))
        })
        .max()
        .unwrap_or(0);

    Game {
        id: *id,
        max_red: reds,
        max_green: greens,
        max_blue: blues,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let sum = input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| {
            let game = process_line(line);

            if game.max_red > max_red || game.max_green > max_green || game.max_blue > max_blue {
                return None;
            }

            Some(game.id)
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let power = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let game = process_line(line);
            game.max_red * game.max_green * game.max_blue
        })
        .sum();

    Some(power)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 2)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        0
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        0
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 5)]
    // Extra cases
    #[case("Game 100: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 100)]
    fn test_part_one_lines(#[case] input: &str, #[case] expected: u32) {
        let result = part_one(input);
        assert_eq!(expected, result.unwrap())
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(8, result.unwrap());
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    // Extra cases
    fn test_part_two_lines(#[case] input: &str, #[case] expected: u32) {
        let result = part_two(input);
        assert_eq!(expected, result.unwrap())
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(2286, result.unwrap());
    }
}
