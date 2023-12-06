use itertools::Itertools;

advent_of_code::solution!(6);

fn distance(charge_time: &u64, duration: &u64) -> u64 {
    let speed = charge_time; // 1mm/ms
    return speed * (duration - charge_time);
}

#[derive(Debug)]
struct Game {
    duration: u64,
    record_distance: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let games = lines.chunks(2).flat_map(|chunks| {
        let times = &chunks[0];
        let distances = &chunks[1];
        times.iter().enumerate().map(|(index, time)| Game {
            duration: *time,
            record_distance: distances[index],
        })
    });

    let margin_of_err = games
        .map(|game| {
            (0..game.duration)
                .map(move |charge_time| distance(&charge_time, &game.duration))
                .filter(move |d| *d > game.record_distance)
                .count()
        })
        .fold(1, |acc, x| acc * x);

    Some(margin_of_err as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut numbers = l.split_whitespace().filter_map(|s| s.parse::<u32>().ok());
            Itertools::join(&mut numbers, "")
        })
        .collect::<Vec<String>>();
    let duration = lines[0].parse::<u64>().unwrap();
    let record_distance = lines[1].parse::<u64>().unwrap();

    let count = (0..duration)
        .map(move |charge_time| distance(&charge_time, &duration))
        .filter(move |d| *d > record_distance)
        .count();

    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(288, result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(71503, result.unwrap());
    }
}
