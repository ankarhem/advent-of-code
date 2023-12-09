use winnow::ascii::line_ending;
use winnow::prelude::*;
use winnow::{
    ascii::{digit1, space1},
    combinator::{repeat, separated},
    IResult, PResult,
};

advent_of_code::solution!(9);

fn line_parser(input: &mut &str) -> PResult<Vec<u32>> {
    separated(0.., digit1.parse_to::<u32>(), space1).parse_next(input)
}

fn input_parser(input: &mut &str) -> PResult<Vec<Vec<u32>>> {
    separated(0.., line_parser, line_ending).parse_next(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input_parser.parse(input).unwrap();

    todo!()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
    fn test_line_parser(#[case] mut input: &str, #[case] expected: Vec<u32>) {
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

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
