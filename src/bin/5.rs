use itertools::Itertools;
use std::{
    cmp::{self, max, min},
    collections::HashSet,
};

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
    let categories = TEST
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

    #[derive(Eq, PartialEq, Hash, Copy, Clone)]
    struct Span {
        begin: i64,
        end: i64,
    }
    let seed_spans: Vec<Span> = categories[0]
        .iter()
        .tuples()
        .map(|(&s, &d)| Span {
            begin: s,
            end: s + d,
        })
        .collect();

    let mut new_spans: HashSet<Span> = HashSet::new();

    let mut p2 = i64::MAX;
    //for s in &seed_spans {
    new_spans.clear();
    new_spans.insert(Span{begin: 79, end: 80});

    for cat in categories.iter().skip(1) {
        //println!("Category");
        for s in new_spans.clone().iter() {
            //println!("Seed: {},{}", s.begin, s.end);
            for (&dest, &source, &length) in cat.iter().tuples() {
                let diff = dest - source;

                let overlap = Span {
                    begin: max(source, s.begin),
                    end: min(s.end, source + length),
                };

                if overlap.begin < overlap.end {
                    if s.begin < overlap.begin {
                        new_spans.insert(Span {
                            begin: s.begin,
                            end: overlap.begin,
                        });
                        //println!("new span: {},{}", s.begin, overlap.begin);
                    } else if s.end > overlap.end {
                        new_spans.insert(Span {
                            begin: overlap.end,
                            end: s.end,
                        });
                        //println!("new span: {},{}", overlap.end, s.end);
                    }

                    new_spans.insert(Span {
                        begin: overlap.begin + diff,
                        end: overlap.end + diff,
                    });
                    //println!("new span (overlap): {},{}", overlap.begin + diff, overlap.end + diff);

                    break;
                }
            }
        }

        let mut low = i64::MAX;
        for s in new_spans.iter() {
            low = min(low, s.begin);
        }
        //println!("Min: {}", low);
        p2 = min(p2, low);
    }

    //}

    // 51580674, 99751240
    (p1 as usize, p2 as usize)
}
