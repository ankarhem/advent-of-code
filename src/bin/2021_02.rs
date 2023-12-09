use itertools::Itertools;

advent_of_code::solution!(2021, 2);

#[derive(Debug)]
enum Instruction {
    Forward(u8),
    Down(u8),
    Up(u8),
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().filter(|l| !l.is_empty()).map(|l| {
        let (command, number) = l
            .split_ascii_whitespace()
            .collect_tuple()
            .expect("Invalid line {l}");

        let number = number.parse::<u8>().expect("Invalid number {number}");
        match command {
            "forward" => Instruction::Forward(number),
            "down" => Instruction::Down(number),
            "up" => Instruction::Up(number),
            _ => panic!("Invalid command {command}"),
        }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_input(input);

    let (position, depth): (u32, u32) =
        instructions.fold((0, 0), |(position, depth), command| match command {
            Instruction::Forward(number) => (position + number as u32, depth),
            Instruction::Down(number) => (position, depth + number as u32),
            Instruction::Up(number) => (position, depth - number as u32),
        });

    Some(position * depth)
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
        assert_eq!(150, result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", YEAR, DAY));
        assert_eq!(None, result);
    }
}
