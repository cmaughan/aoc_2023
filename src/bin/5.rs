use itertools::Itertools;
use std::cmp::{self, min};

const TEST: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

fn resolve_seed(mut seed: i64, categories: &Vec<Vec<i64>>) -> i64 {
    for map in categories.iter().skip(1) {
        for (&dest, &source, &length) in map.iter().tuples() {
            if (seed >= source) && (seed < (source + length)) {
                seed += dest - source;
                break;
            }
        }
    }
    seed
}

#[aoc::main(5)]
fn main(input: &str) -> (usize, usize) {
    let categories = input
        .split("\n\n")
        .map(|s| {
            s.split(":")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter_map(|v| v.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let p1 = categories[0].iter().fold(i64::MAX, |lowest, &seed| {
        min(lowest, resolve_seed(seed, &categories))
    });

    let p2 = categories[0]
        .iter()
        .tuples()
        .flat_map(|(&s, &d)| (s..s + d))
        .fold(i64::MAX, |lowest, seed| {
            min(lowest, resolve_seed(seed, &categories))
        });

    // 51580674, 99751240
    (p1 as usize, p2 as usize)
}
