use std::collections::HashSet;

#[aoc::main(4)]
fn main(input: &str) -> (usize, usize) {

    let mut tot_all = 0;
    let mut mult =  vec![1 as usize; input.lines().count()];
    for (index, line) in input.lines().enumerate() {

        let c = line.split("|").collect::<Vec<_>>();
        let card_digits = c[0].trim().split(":").nth(1).unwrap();
        let card = card_digits
            .trim()
            .split(|c| c == ' ' || c == ':')
            .filter(|c| !c.is_empty())
            .collect::<Vec<_>>();
        let rem = c[1]
            .trim()
            .split(|c| c == ' ' || c == ':')
            .filter(|c| !c.is_empty())
            .collect::<HashSet<_>>();

        let mut tot = 0;
        for c in card {
            if rem.contains(c) {
                tot += 1;
            }
        }
            
        let current = mult[index];
        if tot > 0 {
            tot_all += 1 << (tot - 1);

            for _ in 0..current {
                for slot in (index + 1)..(index + 1 + tot) {
                    mult[slot] += 1;
                }
            }
        }
    }

    (tot_all as usize, mult.iter().sum())
}
