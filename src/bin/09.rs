use rayon::prelude::*;
use winnow::ascii::line_ending;
use winnow::prelude::*;
use winnow::{
    ascii::{dec_int, space1},
    combinator::separated,
    PResult,
};

advent_of_code::solution!(9);

// -- parser --
fn digit_parser(input: &mut &str) -> PResult<i64> {
    dec_int.parse_next(input)
}

fn line_parser(input: &mut &str) -> PResult<Vec<i64>> {
    separated(0.., digit_parser, space1)
        .map(|s| s)
        .parse_next(input)
}

fn input_parser(input: &mut &str) -> PResult<Vec<Vec<i64>>> {
    separated(0.., line_parser, line_ending).parse_next(input)
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
    let input = input_parser
        .parse(input)
        .map_err(|e| e.to_string())
        .unwrap();
    let sum = input.par_iter().map(|v| next_number(v)).sum();

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
    let input = input_parser
        .parse(input)
        .map_err(|e| e.to_string())
        .unwrap();
    let sum = input.par_iter().map(|v| previous_number(v)).sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("0 3 6 9 12 15", vec![0, 3, 6, 9, 12, 15])]
    #[case("1 3 6 10 15 21", vec![1, 3, 6, 10, 15, 21])]
    #[case("10 13 16 21 30 45", vec![10, 13, 16, 21, 30, 45])]
    #[case("-10 2 -4", vec![-10, 2, -4])]
    fn test_line_parser(#[case] mut input: &str, #[case] expected: Vec<i64>) {
        let result = line_parser.parse_next(&mut input);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_input_parser() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];

        assert_eq!(expected, input_parser.parse(input).unwrap());
    }

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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(114, result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(2, result.unwrap());
    }
}
