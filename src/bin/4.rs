#[aoc::main(4)]
fn main(part: i32, input: &str) -> usize {

    let mut tot_all = 0;
    let mut mult = vec![1 as usize; input.lines().count()];
    let mut nums = vec![-1; 100];
    for (index, line) in input.lines().enumerate() {

        let card_matches = line.split("|").collect::<Vec<_>>();

        let card = card_matches[0].trim().split(":").nth(1).unwrap();

        card
            .trim()
            .split(|c| c == ' ')
            .filter_map(|c| c.parse::<u32>().ok())
            .for_each(|m| nums[m as usize] = index as i32);

        let tot = card_matches[1]
            .trim()
            .split(|c| c == ' ')
            .filter_map(|c| c.parse::<u32>().ok())
            .filter(|v| nums[*v as usize] == index as i32)
            .count();

        if tot > 0 {
            tot_all += 1 << (tot - 1);

            let current = mult[index];
            for slot in (index + 1)..=(index + tot) {
                mult[slot] += current;
            }
        }
    }

    if part == 0 {
        tot_all as usize
    }
    else {
        mult.iter().sum()
    }
}
