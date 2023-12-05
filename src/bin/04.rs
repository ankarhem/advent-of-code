advent_of_code::solution!(4);

use std::cmp;

fn to_winning_cards(line: &str) -> u32 {
    let mut parts = line.split('|');
    let winners = parts
        .next()
        .expect("line to have a |")
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let wins = parts
        .next()
        .expect("line to have a |")
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .filter(|n| winners.contains(n))
        .count();

    wins as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let points = input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| {
            let winning_cards = to_winning_cards(line);

            if winning_cards == 0 {
                None
            } else {
                let power = cmp::max(0, winning_cards as i32 - 1) as u32;
                let round_points = 2_u32.pow(power);

                Some(round_points)
            }
        })
        .sum::<u32>();

    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().filter(|l| !l.is_empty()).rev();
    let mut recursive_worth: Vec<u32> = vec![];

    for line in lines {
        let wins = to_winning_cards(line);
        let worth = 1 + recursive_worth
            .iter()
            .rev()
            .take(wins as usize)
            .sum::<u32>();
        recursive_worth.push(worth);
    }

    Some(recursive_worth.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn examples_part_one(#[case] input: &str, #[case] expected: u32) {
        let result = part_one(input);
        assert_eq!(expected, result.unwrap())
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(13, result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(30, result.unwrap());
    }
}
