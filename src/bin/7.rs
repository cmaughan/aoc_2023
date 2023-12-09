use std::{cmp::{Ordering, self}, collections::HashMap};

use itertools::Itertools;

#[aoc::main(7)]

fn main(part: i32, input: &str) -> usize {
    //Q4QKK 465
    let data = input
        .lines()
        .flat_map(|l| {
            l.split(" ")
                .tuples::<(_, _)>()
                .map(|(a, b)| (a, b.parse::<u64>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let values: HashMap<char, u64> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    if part == 0 {
        let mut sorted_cards = Vec::new();

        // For each row
        for c in data {
            let mut map = HashMap::new();

            // For each character in the cards
            c.0.chars().for_each(|c| {
                let count = map.entry(c).or_insert(0 as u64);
                *count += 1;
            });

            // Score all the card entries
            let score = map.iter().fold(0, |acc, (_c, v)| {
                acc + match v {
                    1 => 0 as u64,
                    2 => 1,
                    3 => 10,
                    4 => 100,
                    5 => 1000,
                    _ => 0,
                }
            });

            sorted_cards.push((c.0, c.1, score));
        }

        sorted_cards.sort_by(|&left, &right| {
            let or = left.2.cmp(&right.2);
            if or == Ordering::Equal {
                for (a, b) in left.0.chars().zip(right.0.chars()) {
                    let a1 = values.get(&a).unwrap();
                    let b1 = values.get(&b).unwrap();
                    let order = a1.cmp(&b1);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
            }
            or
        });

        sorted_cards.iter().enumerate().map(|(index, (_, r, _))| {
            r * (index as u64 + 1)
        }).sum::<u64>() as usize
    } else {
        0
    }

    //251136060

}
