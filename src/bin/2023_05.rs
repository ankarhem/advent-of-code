use std::ops::Range;

use rayon::prelude::*;

advent_of_code::solution!(2023, 5);

#[derive(Debug)]
struct CategoryMaps {
    maps: Vec<(Range<u64>, Range<u64>)>,
}

impl CategoryMaps {
    fn convert(&self, seed: u64) -> u64 {
        let m = self.maps.iter().find(|(r, _)| r.contains(&seed));

        let Some((source, dest)) = m else {
            return seed;
        };

        let offset = seed - source.start;
        dest.start + offset
    }
}

fn to_category_maps(s: &str) -> CategoryMaps {
    let maps = s
        .lines()
        .flat_map(|l| l.split_whitespace().filter_map(|s| s.parse::<u64>().ok()))
        .collect::<Vec<u64>>()
        .chunks(3)
        .map(|c| {
            let dest_start = c[0];
            let source_start = c[1];
            let length = c[2];
            let dest_range = dest_start..dest_start + length;
            let source_range = source_start..source_start + length;
            (source_range, dest_range)
        })
        .collect::<Vec<_>>();
    CategoryMaps { maps }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sections = input.split("\n\n");
    let seeds = sections
        .next()?
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let categories: Vec<CategoryMaps> = sections.map(to_category_maps).collect();

    let min = seeds
        .into_iter()
        .map(|seed| {
            categories
                .iter()
                .fold(seed, |seed, category| category.convert(seed))
        })
        .min();

    Some(min.unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let seed_ranges = input
        .lines()
        .next()?
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| c[0]..c[0] + c[1])
        .collect::<Vec<_>>();

    let categories: Vec<CategoryMaps> = input.split("\n\n").skip(1).map(to_category_maps).collect();

    let min = seed_ranges
        .into_par_iter()
        .flat_map(|r| r)
        .map(|seed| {
            categories
                .iter()
                .fold(seed, |seed_state, category| category.convert(seed_state))
        })
        .min();

    min
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
        let source_range = map[1]..map[1] + map[2];
        let dest_range = map[0]..map[0] + map[2];
        let category = CategoryMaps {
            maps: vec![(source_range, dest_range)],
        };
        let seed = category.convert(seed);

        assert_eq!(expected, seed);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", YEAR, DAY));
        assert_eq!(35, result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", YEAR, DAY));
        assert_eq!(46, result.unwrap());
    }
}
