use std::collections::HashSet;

#[aoc::main(4)]
fn main(input: &str) -> (usize, usize) {

let test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let mut tot_all = 0;
    for line in test.lines() {
        let c = line.split("|").collect::<Vec<_>>();
        
        let card_digits = c[0].trim().split(":").nth(1).unwrap();

        let card = card_digits.trim().split(|c| c == ' ' || c == ':').collect::<HashSet<_>>();
        let rem = c[1].trim().split(|c| c == ' ' || c == ':').collect::<Vec<_>>();

        println!("{:?}", card);
        println!("{:?}", rem);
        let mut tot = 0;
        for c in card {
            if rem.contains(c) {
                tot += 1;
            }
        }
        if tot > 0 {
            tot_all += 1 << (tot - 1);
        }
    }

    (tot_all as usize, 0 as usize)
}
