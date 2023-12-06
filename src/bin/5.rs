use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::VecDeque,
};

fn resolve_seed(mut seed: u64, categories: &Vec<Vec<u64>>) -> u64 {
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
fn main(part: i32, _input: &str) -> usize {

    // Get the mapped ranges
    let categories = _input
        .split("\n\n")
        .map(|s| {
            s.split(":")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter_map(|v| v.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if part == 0 {
        // Part 0, just a mapped search
        categories[0].iter().fold(u64::MAX, |lowest, &seed| {
            min(lowest, resolve_seed(seed, &categories))
        }) as usize
    } else {
        // Part 1 keep the seeds in a deque so I can add more as I go
        let mut seeds: VecDeque<(u64, u64)> = categories[0]
            .iter()
            .tuples()
            .map(|(&s, &d)| (s, s+d))
            .collect();

        for cat in categories.iter().skip(1) {
        
            let mut new_seeds: VecDeque<(u64, u64)> = VecDeque::new();

            while let Some(seed) = seeds.pop_front() {

                let mut overlapped = false;
                for (&dest, &source, &length) in cat.iter().tuples() {
                    let overlap_begin = max(seed.0, source);
                    let overlap_end = min(seed.1, source + length);

                    if overlap_begin < overlap_end {
                        new_seeds.push_back((overlap_begin + (dest - source), overlap_end + (dest - source)));

                        if seed.0 < overlap_begin {
                            seeds.push_back((seed.0, overlap_begin));
                        } else if seed.1 > overlap_end {
                            seeds.push_back((overlap_end, seed.1 ));
                        }
                        overlapped = true;
                        break;
                    }
                }

                if !overlapped {
                    new_seeds.push_back(seed);
                }
            }

            seeds = new_seeds;
        }

        seeds.iter().min_by_key(|val| val.0).unwrap().0 as usize
    }

    // 35, 46
    // 51580674, 99751240
}
