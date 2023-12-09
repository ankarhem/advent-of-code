use rayon::prelude::*;

advent_of_code::solution!(2023, 9);

fn parse_input(input: &str) -> impl ParallelIterator<Item = Vec<i64>> + '_ {
    input.par_lines().filter(|l| !l.is_empty()).map(|l| {
        l.split_ascii_whitespace()
            .map(|s| s.parse::<i64>().expect("To be able to parse number `{l}`"))
            .collect::<Vec<_>>()
    })
}

// -- utility --
fn is_constant_difference(vec: &[i64]) -> bool {
    vec.windows(2).all(|w| w[1] - w[0] == 0)
}

fn to_difference_vec(vec: &[i64]) -> Vec<i64> {
    vec.windows(2).map(|w| w[1] - w[0]).collect()
}

// -- part_one --
fn next_number(vec: &[i64]) -> i64 {
    let diff = to_difference_vec(vec);
    let last = vec.last().unwrap();
    let diff_last = diff.last().unwrap();
    if is_constant_difference(&diff) {
        last + diff_last
    } else {
        last + next_number(&diff)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let input = parse_input(input);
    let sum = input.map(|v| next_number(&v)).sum();

    Some(sum)
}

// -- part_two --
fn previous_number(vec: &[i64]) -> i64 {
    let diff = to_difference_vec(vec);
    let first = vec.first().unwrap();
    let diff_first = diff.first().unwrap();
    if is_constant_difference(&diff) {
        first - diff_first
    } else {
        first - previous_number(&diff)
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = parse_input(input);
    let sum = input.map(|v| previous_number(&v)).sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3, 3, 3, 3, 3], true)]
    #[case(vec![-3, -3, -3, -3], true)]
    #[case(vec![-3, 0, 3, 6], false)]
    #[case(vec![0, 3, 6, 9, 12, 15], false)]
    #[case(vec![1, 3, 6, 10, 15, 21], false)]
    fn test_is_linear_difference(#[case] input: Vec<i64>, #[case] expected: bool) {
        assert_eq!(expected, is_constant_difference(&input));
    }

    #[rstest]
    #[case(vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0])]
    #[case(vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3])]
    #[case(vec![1, 3, 6, 10, 15, 21], vec![2, 3, 4, 5, 6])]
    fn test_to_difference(#[case] input: Vec<i64>, #[case] expected: Vec<i64>) {
        assert_eq!(expected, to_difference_vec(&input));
    }

    #[rstest]
    #[case(vec![3, 3, 3, 3, 3], 3)]
    #[case(vec![0, 3, 6, 9, 12, 15], 18)]
    #[case(vec![1, 3, 6, 10, 15, 21], 28)]
    #[case(vec![10, 13, 16, 21, 30, 45], 68)]
    fn test_next_number(#[case] input: Vec<i64>, #[case] expected: i64) {
        assert_eq!(expected, next_number(&input));
    }

    #[rstest]
    #[case(vec![3, 3, 3, 3, 3], 3)]
    #[case(vec![0, 3, 6, 9, 12, 15], -3)]
    #[case(vec![10, 13, 16, 21, 30, 45], 5)]
    fn test_previous_number(#[case] input: Vec<i64>, #[case] expected: i64) {
        assert_eq!(expected, previous_number(&input));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", YEAR, DAY));
        assert_eq!(114, result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", YEAR, DAY));
        assert_eq!(2, result.unwrap());
    }
}
