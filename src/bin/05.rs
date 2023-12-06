#![feature(iter_map_windows)]
advent_of_code::solution!(5);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Seed(u64);

fn advance_seed(seed: &mut u64, maps: &Vec<CategoryMap>) {
    let map = maps
        .iter()
        .find(|m| *seed >= m.source_start && *seed < m.source_start + m.length);

    if let Some(map) = map {
        let diff: i64 = *seed as i64 - map.source_start as i64;
        *seed = (map.dest_start as i64 + diff) as u64;
    }
}

#[derive(Debug)]
struct CategoryMap {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

fn to_category_map(s: &str) -> Vec<CategoryMap> {
    s.lines()
        .flat_map(|l| l.split_whitespace().filter_map(|s| s.parse::<u64>().ok()))
        .collect::<Vec<u64>>()
        .chunks(3)
        .map(|c| CategoryMap {
            dest_start: c[0],
            source_start: c[1],
            length: c[2],
        })
        .collect::<Vec<CategoryMap>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sections = input.split("\n\n");
    let mut seeds = sections
        .next()?
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let map_sections: Vec<Vec<CategoryMap>> = sections.map(to_category_map).collect();

    for map in map_sections {
        seeds.iter_mut().for_each(|seed| advance_seed(seed, &map));
    }

    Some(seeds.into_iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let seed_ranges = input
        .lines()
        .next()?
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let mut seeds = seed_ranges
        .chunks(2)
        .flat_map(|c| {
            let start = c[0];
            let length = c[1];
            (0..length).map(|i| start + i).collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>();

    let map_sections: Vec<Vec<CategoryMap>> =
        input.split("\n\n").skip(1).map(to_category_map).collect();

    for map in map_sections {
        seeds.iter_mut().for_each(|seed| advance_seed(seed, &map));
    }

    Some(seeds.into_iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(79, [52,50,48], 81)]
    #[case(14, [52,50,48], 14)]
    #[case(55, [52,50,48], 57)]
    #[case(13, [52,50,48], 13)]
    fn test_advance(#[case] seed: u64, #[case] map: [u64; 3], #[case] expected: u64) {
        let map = vec![CategoryMap {
            dest_start: map[0],
            source_start: map[1],
            length: map[2],
        }];
        let mut seed = seed;
        advance_seed(&mut seed, &map);

        assert_eq!(expected, seed);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(35, result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(46, result.unwrap());
    }
}
