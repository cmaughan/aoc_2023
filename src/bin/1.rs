
#[aoc::main(1)]
fn main(input: &str) -> (usize, usize) {
    let vals = input
        .lines()
        .map(|line| {
            let t = line.chars()
                .filter(|d| *d >= '0' && *d <= '9')
                .collect::<String>();
            format!("{}{}", t.chars().next().unwrap(), t.chars().last().unwrap())
        })
        .collect::<Vec<String>>();

    (vals.iter().map(|s| s.parse::<u32>().unwrap()).sum::<u32>() as usize, 0)
}
