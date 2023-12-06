
fn run_trial(nums: Vec<u64>) -> u64 {
    let num_trials = nums.len() / 2;
    let mut tot = 1;
    for boat in 0..num_trials {
        let win_distance = nums[boat + num_trials];
        let total_time = nums[boat];
        
        let mut ways: u64 = 0;
        for time in 0..total_time {
            let speed = time;
            let travel_time = total_time - time;
            let distance = travel_time * speed;
            if distance > win_distance {
                ways += 1;
            }
        }
        tot *= ways;
    }
    tot
}

#[aoc::main(6)]
fn main(input: &str) -> (usize, usize) {

    let p1 = run_trial(input
        .split_whitespace()
        .filter_map(|d| d.parse::<u64>().ok())
        .collect::<Vec<u64>>());

    let p2_chars = input
        .lines()
        .map(|s| 
            s
                .chars()
                .filter(|c| {
                    c.is_digit(10)
                })
                .collect::<String>()
        )
        .map(|a| a.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let p2 = run_trial(p2_chars);


    //503424, 32607562
    (p1 as usize,  p2 as usize)
}
