use itertools::Itertools;

advent_of_code::solution!(2021, 1);

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u32>().expect("Invalid digit {l}"));

    let mut answer = 0;

    numbers.tuple_windows().for_each(|(a, b)| {
        if a < b {
            answer += 1;
        }
    });

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", YEAR, DAY));
        assert_eq!(7, result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", YEAR, DAY));
        assert_eq!(result, None);
    }
}
