
#[aoc::main(1)]
fn main(input: &str) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|line| {
            let t = line
                .chars()
                .filter(|d| *d >= '0' && *d <= '9')
                .map(|d| (d as u32) - '0' as u32)
                .collect::<Vec<_>>();
            t.first().unwrap() * 10 + t.last().unwrap()
        })
        .sum::<u32>();

    let p2 = input
        .lines()
        .map(|line| {
            let mut new_line = line.replace("one", "o1e");
            new_line = new_line.replace("two", "t2o");
            new_line = new_line.replace("three", "t3e");
            new_line = new_line.replace("four", "f4r");
            new_line = new_line.replace("five", "f5e");
            new_line = new_line.replace("six", "s6x");
            new_line = new_line.replace("seven", "s7n");
            new_line = new_line.replace("eight", "e8t");
            new_line = new_line.replace("nine", "n9e");
            let t = new_line
                .chars()
                .filter(|d| *d >= '0' && *d <= '9')
                .map(|d| (d as u32) - '0' as u32)
                .collect::<Vec<_>>();
            t.first().unwrap() * 10 + t.last().unwrap()
        })
        .sum::<u32>();

    (p1 as usize, p2 as usize)
}
