#![feature(iter_map_windows)]
advent_of_code::solution!(3);

fn has_symbols(input: &str) -> bool {
    !input
        .chars()
        .filter(|c| !c.is_ascii_digit() && *c != '.')
        .collect::<Vec<_>>()
        .is_empty()
}

pub fn part_one(input: &str) -> Option<u32> {
    let line_length = input.lines().next()?.len();

    // add a line to start and end of input with .'s
    let padded_input = format!(
        "{}\n{}\n{}",
        ".".repeat(line_length),
        input,
        ".".repeat(line_length)
    );
    let sum = padded_input
        .lines()
        .filter(|line| !line.is_empty())
        .map_windows(|[prev, curr, next]| {
            let mut numbers = vec![];
            let mut number: Option<u32> = None;
            let mut start: Option<usize> = None;

            // If the current character is a digit, mutate number,
            // account for multi-digit numbers (* 10 on the previous number and add the current digit)
            // Keep track of the start (and implicitly the end in the loop) in order to slice out surrounding areas
            // Check if the surrounding area has any symbols in it (anything that isn't a digit or a .)
            // If it does, add the current number to the list of numbers

            curr.chars().enumerate().for_each(|(index, c)| {
                if c.is_ascii_digit() {
                    if number.is_none() {
                        start = match index {
                            0 => Some(0),
                            _ => Some(index - 1),
                        };
                    }
                    number = Some(match number {
                        Some(n) => n * 10 + c.to_digit(10).unwrap(),
                        None => c.to_digit(10).unwrap(),
                    });
                } else if let Some(curr_num) = number {
                    let s = start.expect("start should be set");

                    if has_symbols(&curr[s..=index])
                        || has_symbols(&prev[s..=index])
                        || has_symbols(&next[s..=index])
                    {
                        numbers.push(curr_num);
                    }
                    number = None;
                    start = None;
                }
            });

            // handle numbers at the end of the line
            // as the loop will have terminated if the last character was a digit
            if let Some(curr_num) = number {
                let s = start.expect("start should be set");
                if has_symbols(&curr[s..]) || has_symbols(&prev[s..]) || has_symbols(&next[s..]) {
                    numbers.push(curr_num);
                }
            }

            numbers
        })
        .flatten()
        .sum::<u32>();

    Some(sum)
}

/// --- Part Two ---

fn get_number(line: &str, from: usize) -> Option<u32> {
    let numbers_to_end = line[from..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();

    let numbers_to_start = line[..=from]
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .skip(1)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    let combined = format!("{numbers_to_start}{numbers_to_end}");

    combined.parse::<u32>().ok()
}

fn get_numbers(line: &str, c: usize) -> Vec<u32> {
    let mut numbers = vec![];
    //     v
    //   ..2..
    //   ..*..
    if let Some(n) = get_number(line, c) {
        numbers.push(n);
    } else {
        //    v v
        //   .2.2.
        //   ..*..
        if let Some(n) = get_number(line, c - 1) {
            numbers.push(n);
        }
        if let Some(n) = get_number(line, c + 1) {
            numbers.push(n);
        }
    }

    numbers
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_length = input.lines().next()?.len();
    // add a line to start and end of input with .'s
    let padded_input = format!(
        "{}\n{}\n{}",
        ".".repeat(line_length),
        input,
        ".".repeat(line_length)
    );
    let sum = padded_input
        .lines()
        .filter(|line| !line.is_empty())
        .map_windows(|[prev, curr, next]| {
            let gear_ratios: Vec<u32> = curr
                .char_indices()
                .filter_map(|(index, char)| if char == '*' { Some(index) } else { None })
                .filter_map(|c| {
                    let all_numbers: Vec<u32> = get_numbers(prev, c)
                        .into_iter()
                        .chain(get_numbers(curr, c))
                        .chain(get_numbers(next, c))
                        .collect();

                    if all_numbers.len() != 2 {
                        return None;
                    }

                    Some(all_numbers[0] * all_numbers[1])
                })
                .collect();

            gear_ratios
        })
        .flatten()
        .sum::<u32>();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(4361, result.unwrap());
    }

    #[rstest]
    #[case(".2.4.", 2, None)]
    #[case("..345", 2, Some(345))]
    #[case("123..", 2, Some(123))]
    #[case("12345", 2, Some(12345))]
    fn test_get_number(#[case] input: &str, #[case] from: usize, #[case] expected: Option<u32>) {
        let result = get_number(input, from);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(467835, result.unwrap());
    }
}
