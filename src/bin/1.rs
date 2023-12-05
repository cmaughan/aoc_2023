const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_begin(line: &str) -> Option<u32> {
    line.chars().find_map(|a| a.to_digit(10))
}

fn find_begin_end_digit(line: &str) -> u32 {
    let first = find_begin(line).unwrap();
    let second = line.chars().rev().find_map(|a| a.to_digit(10)).unwrap();
    first * 10 + second
}

fn find_begin_digit_or_word(line: &str) -> Option<u32> {
    (0..line.len()).find_map(|a| {
        let search = &line[a..];
        NUMS.iter()
            .enumerate()
            .find(|(_, num_text)| search.starts_with(*num_text))
            .map(|(index, _)| index as u32 + 1)
            .or_else(|| find_begin(search))
    })
}

fn find_end_digit_or_word(line: &str) -> Option<u32> {
    (0..line.len()).rev().find_map(|a| {
        let search = &line[a..];
        NUMS.iter()
            .enumerate()
            .find(|(_, num_text)| search.starts_with(*num_text))
            .map(|(index, _)| index as u32 + 1)
            .or_else(|| find_begin(search))
    })
}

fn find_begin_end_digit_or_word(line: &str) -> u32 {
    let begin = find_begin_digit_or_word(line).unwrap();
    let end = find_end_digit_or_word(line).unwrap();
    begin * 10 + end
}

#[aoc::main(1)]
fn main(input: &str) -> (usize, usize) {
    let p1 = input.lines().map(find_begin_end_digit).sum::<u32>();
    let p2 = input.lines().map(find_begin_end_digit_or_word).sum::<u32>();
    
    (p1 as usize, p2 as usize)
}
