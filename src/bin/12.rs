use itertools::Itertools;

#[aoc::main(12)]
fn main(part: i32, input: &str) -> usize {

    let springs = input.split_whitespace().tuples::<(_, _)>().collect::<Vec<_>>();
    for (spring, counts) in springs.iter() {
        let len = spring.len() as u64;
        let or_mask = spring.chars().enumerate().fold(0,|acc, (index, c)| {
            let shift = len - 1 - index as u64;
            let ret = match c {
                '#' => (1 as u64) << shift,
                _ => 0 as u64
            };
            acc | ret
        });
        let and_mask = spring.chars().enumerate().fold(u64::MAX,|acc, (index, c)| {
            let shift = len - 1 - index as u64;
            match c {
                '.' => acc & !((1 as u64) << shift),
                _ => acc
            }
        });


        println!("Or: {:b}, And: {:b}", or_mask, and_mask);
    }

    println!("{:?}", springs);
    //251136060, 251339174
    0
}
