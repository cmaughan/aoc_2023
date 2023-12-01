
#[aoc::main(1)]
fn main(input: &str) -> (usize, usize) {
    let mut vals = input
        .lines()
        .map(|line| {
            let t = line.chars()
                .filter(|d| *d >= '0' && *d <= '9')
                .collect::<String>();
            format!("{}{}", t.chars().next().unwrap(), t.chars().last().unwrap())
        })
        .collect::<Vec<String>>();

    let p1 = vals.iter().map(|s| s.parse::<u32>().unwrap()).sum::<u32>() as usize;

    vals = input
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
            let t = new_line.chars()
                .filter(|d| *d >= '0' && *d <= '9')
                .collect::<String>();
            format!("{}{}", t.chars().next().unwrap(), t.chars().last().unwrap())
        })
        .collect::<Vec<String>>();

    let p2 = vals.iter().map(|s| s.parse::<u32>().unwrap()).sum::<u32>() as usize;

    (p1 as usize, p2 as usize)
}
