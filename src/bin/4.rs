#[aoc::main(4)]
fn main(input: &str) -> (usize, usize) {
    let mut tot_all = 0;
    let mut mult = vec![1 as usize; input.lines().count()];
    let mut nums = vec![0; 100];
    for (index, line) in input.lines().enumerate() {

        nums.iter_mut().for_each(|a| *a = 0);

        let card_matches = line.split("|").collect::<Vec<_>>();

        let card = card_matches[0].trim().split(":").nth(1).unwrap();

        card_matches[1]
            .trim()
            .split(|c| c == ' ' || c == ':')
            .filter_map(|c| c.parse::<u32>().ok())
            .for_each(|m| nums[m as usize] = 1 );

        let tot = card
            .trim()
            .split(|c| c == ' ' || c == ':')
            .filter_map(|c| c.parse::<u32>().ok())
            .filter(|v| nums[*v as usize] != 0)
            .count();

        if tot > 0 {
            tot_all += 1 << (tot - 1);

            let current = mult[index];
            for slot in (index + 1)..(index + 1 + tot) {
                mult[slot] += current;
            }
        }
    }

    (tot_all as usize, mult.iter().sum())
}
