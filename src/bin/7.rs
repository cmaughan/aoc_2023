
use itertools::Itertools;
const TEST : &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

fn score_card(part: i32, card: &[u8]) -> Vec<u8> {
    
    let mut face_counts = vec![0 as u8; 13 + 5];
    let mut secondary_sort = vec![0 as u8; 5];
    let remap_1 = b"23456789TJQKA";
    let remap_2 = b"J23456789TQKA";
    for index in 0..5 {
        // Count up the card values into the data
        let mut pos = 0;
        let card_char = card[index as usize];
        if part == 0 {
            pos = remap_1.iter().position(|c| *c == card_char).unwrap();
            face_counts[pos] += 1;
        } else {
            pos = remap_2.iter().position(|c| *c == card_char).unwrap();
            if card_char != b'J' {
                face_counts[pos] += 1;
            }
        }
        secondary_sort[index] = pos as u8;
    }
    // Sort them to figure out who wins
    face_counts.sort_by(|a, b| b.cmp(a));

    // make a secondary ordering to resolve matches
    for index in 0..5 {
        face_counts[index + 13] = secondary_sort[index];
        if part == 1 && card[index as usize] == b'J' {
            face_counts[0] += 1;
        }
    }
    face_counts
}

#[aoc::main(7)]
fn main(part: i32, input: &str) -> usize {
    //Q4QKK 465
    let mut data = input
        .lines()
        .flat_map(|l| {
            l.split(" ")
                .tuples::<(_, _)>()
                .map(|(a, b)| (a, b.parse::<u64>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();


    data.sort_by_key(|(card, _)| score_card(part, card.as_bytes()));

    data
        .iter()
        .enumerate()
        .map(|(index, (card, r))| {
            r * (index as u64 + 1)
        })
        .sum::<u64>() as usize

    //251136060, 251339174
}
