fn run_trial(nums: Vec<u64>) -> u64 {
    let num_trials = nums.len() / 2;
    let mut tot = 1;
    for boat in 0..num_trials {
        let win_distance = nums[boat + num_trials] as f64;
        let total_time = nums[boat] as f64;

        let mut end = (total_time - (total_time * total_time - 4.0 * win_distance).sqrt()) / -2.0;
        let mut beg = (total_time + (total_time * total_time - 4.0 * win_distance).sqrt()) / -2.0;

        if beg == beg.ceil() {
            beg += 1.0;
        }
        if end == end.ceil() {
            end -= 1.0;
        }
        tot *= (end.floor() - beg.ceil()) as u64 + 1;
    }
    tot
}

#[aoc::main(6)]
fn main(part: i32, input: &str) -> usize {
    if part == 0 {
        run_trial(
            input
                .split_whitespace()
                .filter_map(|d| d.parse::<u64>().ok())
                .collect::<Vec<u64>>(),
        ) as usize
    } else {
        let p2_chars = input
            .lines()
            .map(|s| s.chars().filter(|c| c.is_digit(10)).collect::<String>())
            .map(|a| a.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        run_trial(p2_chars) as usize
    }

    //503424, 32607562
}
