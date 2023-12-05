#![feature(iter_map_windows)]

advent_of_code::solution!(3);

fn has_symbols(input: &str) -> bool {
    input
        .char_indices()
        .filter(|(index, c)| !c.is_digit(10) && *c != '.')
        .collect::<Vec<_>>()
        .len()
        > 0
}

pub fn part_one(input: &str) -> Option<u32> {
    let line_length = input.lines().next()?.len();

    // add a line to start and end of input with .
    let paddedInput = format!(
        "{}\n{}\n{}",
        ".".repeat(line_length),
        input,
        ".".repeat(line_length)
    );
    let sum = paddedInput
        .lines()
        .filter(|line| !line.is_empty())
        .map_windows(|[prev, curr, next]| {
            let mut numbers = vec![];
            let mut number: Option<u32> = None;
            let mut start: Option<usize> = None;

            // If the current character is a digit, mutate number, account for multi-digit numbers (* 10 on the previous number and add the current digit)
            // Keep track of the start (and implicitly the end in the loop) in order to slice out surrounding areas
            // Check if the surrounding area has any symbols in it (anything that isn't a digit or a .)
            // If it does, add the current number to the list of numbers

            curr.chars().enumerate().for_each(|(index, c)| {
                if c.is_digit(10) {
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
                } else {
                    if let Some(curr_num) = number {
                        let s = start.expect("start should be set");

                        dbg!(&prev.len());
                        dbg!(&curr.len());
                        dbg!(&next.len());
                        if has_symbols(&curr[s..=index])
                            || has_symbols(&prev[s..=index])
                            || has_symbols(&next[s..=index])
                        {
                            numbers.push(curr_num);
                        }
                        number = None;
                        start = None;
                    }
                }
            });

            // handle numbers at the end
            if let Some(curr_num) = number {
                let s = start.expect("start should be set");
                if has_symbols(&curr[s..]) || has_symbols(&prev[s..]) || has_symbols(&next[s..]) {
                    numbers.push(curr_num);
                }
            }

            numbers
        })
        .flatten()
        .inspect(|n| {
            dbg!(n);
        })
        .sum::<u32>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(4361, result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
