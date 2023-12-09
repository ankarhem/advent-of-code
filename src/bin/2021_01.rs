use itertools::Itertools;

advent_of_code::solution!(2021, 1);

fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u32>().expect("Invalid digit {l}"))
}

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = parse_input(input);

    let mut answer = 0;
    numbers.tuple_windows().for_each(|(a, b)| {
        if a < b {
            answer += 1;
        }
    });

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = parse_input(input);

    let mut answer = 0;
    let mut previous_sum: Option<u32> = None;
    numbers.tuple_windows().for_each(|(a, b, c)| {
        let sum = a + b + c;
        if let Some(previous_sum) = previous_sum {
            if previous_sum < sum {
                answer += 1;
            }
        }
        previous_sum = Some(sum);
    });

    Some(answer)
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
        assert_eq!(5, result.unwrap());
    }
}
