use itertools::Itertools;


fn score_card(part: i32, card: &str) -> u64 {
    let mut counts: [u8; 15] = [0; 15];
    let mut score: u64 = 0;
    for c in 0..15 {
        counts[c] = 0;
    }

    let mut jokers: u8 = 0;
    // For each character in the cards
    card.chars().enumerate().for_each(|(index, c)| {
        let count_index: u64;
        if c.is_digit(10) {
            count_index = c as u64 - b'0' as u64;
        } else {
            count_index = match c {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => 0,
            }
        }

        if c == 'J' {
            jokers += 1;
            if part == 0 {
                counts[count_index as usize] += 1;
                score |= count_index << (5 * (5 - index));
            }
        } else {
            counts[count_index as usize] += 1;
            score |= count_index << (5 * (5 - index));
        }
    });

    if part == 1 {
        let index = counts
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0;
        counts[index] += jokers;
    }

    // Score all the card entries
    let face_score = counts.iter().fold(0, |acc, v| {
        acc + match v {
            1 => 0 as u64,
            2 => 2,
            3 => 8,
            4 => 32,
            5 => 64,
            _ => 0,
        }
    });

    score |= face_score << 32;
    score
}

#[aoc::main(7)]
fn main(part: i32, input: &str) -> usize {

    input
        .lines()
        .flat_map(|l| {
            l.split(" ")
                .tuples::<(_, _)>()
                .map(|(a, b)| (a, b.parse::<u64>().unwrap()))
                .collect::<Vec<_>>()
        })
        .map(|(card, order)| (card, order, score_card(part, card)))
        .sorted_by_key(|(_, _, score)| *score)
        .enumerate()
        .map(|(index, (_, r, _))| r * (index as u64 + 1))
        .sum::<u64>() as usize

    //251136060, 251339174
}
