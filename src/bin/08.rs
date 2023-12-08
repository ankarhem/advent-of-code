use core::panic;
use std::collections::HashMap;

use winnow::ascii::alphanumeric1;
use winnow::combinator::{delimited, separated_pair};
use winnow::prelude::*;
use winnow::token::take_until1;
use winnow::PResult;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Node<'a>(&'a str);

#[derive(Debug)]
struct Network<'a> {
    nodes: HashMap<Node<'a>, (Node<'a>, Node<'a>)>,
}

impl Network<'_> {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn calculate_distance<F>(&self, start: &Node, directions: &Vec<char>, cond: F) -> Option<u64>
    where
        F: Fn(&Node) -> bool,
    {
        let mut steps = 0;
        let mut current_node = start;

        while let Some((left, right)) = self.nodes.get(&current_node) {
            // if steps > 0 && current_node == start {
            //     return None; // Cannot reach target node
            // }

            if cond(current_node) {
                return Some(steps);
            }

            let next_direction = directions[steps as usize % directions.len()];
            let next_node = match next_direction {
                'L' => left,
                'R' => right,
                _ => panic!("Unknown direction"),
            };

            current_node = next_node;
            steps += 1;
        }

        Some(steps)
    }
}

advent_of_code::solution!(8);

fn parse_line<'a>(input: &mut &'a str) -> PResult<(Node<'a>, Node<'a>, Node<'a>)> {
    let id = alphanumeric1.parse_next(input)?;
    let _ = take_until1("(").parse_next(input)?;
    let (left, right) = delimited("(", separated_pair(alphanumeric1, ", ", alphanumeric1), ")")
        .parse_next(input)?;

    Ok((Node(id), Node(left), Node(right)))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().filter(|l| !l.is_empty());

    let directions = lines.next()?.chars().collect::<Vec<_>>();

    let mut network = Network::new();
    let lines = lines
        .map(|l| parse_line(&mut l.trim()))
        .collect::<PResult<Vec<_>>>()
        .ok()?;

    for line in lines {
        let (id, left, right) = line;
        network.nodes.entry(id).or_insert((left, right));
    }

    let start_node = Node("AAA");
    let target_node = Node("ZZZ");

    let distance = network.calculate_distance(&start_node, &directions, |n| n == &target_node)?;

    Some(distance)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().filter(|l| !l.is_empty());

    let directions = lines.next()?.chars().collect::<Vec<_>>();

    let mut network = Network::new();
    let node_list = lines
        .map(|l| parse_line(&mut l.trim()))
        .collect::<PResult<Vec<_>>>()
        .expect("Should parse node list");

    for line in node_list.clone() {
        let (id, left, right) = line;
        network.nodes.entry(id).or_insert((left, right));
    }

    let starting_nodes = node_list
        .iter()
        .filter(|(id, _, _)| id.0.ends_with('A'))
        .map(|(id, _, _)| id)
        .collect::<Vec<_>>();

    let node_distances = starting_nodes
        .iter()
        .map(|node| network.calculate_distance(node, &directions, |n| n.0.ends_with("Z")))
        .collect::<Option<Vec<_>>>()?;

    let steps = node_distances
        .iter()
        .fold(1, |acc, &x| num::integer::lcm(acc, x));

    Some(steps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("AAA = (BBB, CCC)", (Node("AAA"), Node("BBB"), Node("CCC")))]
    #[case("11A = (11B, XXX)", (Node("11A"), Node("11B"), Node("XXX")))]
    fn test_parse_node(#[case] input: &str, #[case] expected: (Node, Node, Node)) {
        let mut input = input;
        let result = parse_line(&mut input);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_part_one_example_one() {
        let example = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = part_one(example);
        assert_eq!(2, result.unwrap());
    }

    #[test]
    fn test_part_one_example_two() {
        let example = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part_one(example);
        assert_eq!(6, result.unwrap());
    }

    // Part Two

    // #[rstest]
    // #[case(12, vec![2, 2, 3])]
    // #[case(24, vec![2, 2, 2, 3])]
    // #[case(30, vec![2, 3, 5])]
    // fn test_factorize(#[case] input: u64, #[case] expected: Vec<u64>) {
    //     let result = factorize(input);
    //     assert_eq!(expected, result);
    // }

    #[test]
    fn test_part_two_example() {
        let example = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part_two(example);
        assert_eq!(6, result.unwrap());
    }

    #[test]
    fn test_part_two_custom_example() {
        let example = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)

22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)

XXX = (XXX, XXX)";
        let result = part_two(example);
        assert_eq!(6, result.unwrap());
    }
}
