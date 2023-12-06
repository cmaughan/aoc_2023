use std::collections::{HashMap, HashSet};

fn is_digit(d: &u8) -> bool {
    *d >= b'0' && *d <= b'9'
}

fn is_empty(d: &u8) -> bool {
    *d == b'.' || *d == b'\n'
}

fn extract_num(line: &[u8], begin: i32, end: i32) -> u32 {
    let mut total = 0;
    let mut mul = 1;
    for num_index in (begin..=end).rev() {
        total += (line[num_index as usize] - b'0') as u32 * mul;
        mul *= 10;
    }
    total
}

#[aoc::main(3)]
fn main(part: i32, input: &str) -> usize {
    let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    let stride = lines[0].len() as i32;
    let num_lines = lines.len() as i32;

    let mut gears: HashMap<i32, Vec<_>> = HashMap::new();
    let mut gear_pivots = HashSet::new();

    let mut total_engine = 0;
    for line_index in 0..num_lines {
        let line = lines[line_index as usize];
        let mut start_num: i32 = -1;
        let mut end_num: i32 = -1;
        let mut matched = false;
        for (char_index, ch) in line.iter().enumerate() {
            if is_digit(ch) {
                if start_num < 0 {
                    start_num = char_index as i32;
                    end_num = char_index as i32;
                } else {
                    end_num = char_index as i32;
                }
            }
            if !is_digit(ch) || (char_index == (stride - 1) as usize) {
                if start_num != -1 {
                    for x in start_num-1..=end_num+1 {
                        for y in line_index-1..=line_index+1 {
                            if (x >= start_num && x <= end_num) && (y == line_index) {
                                continue;
                            }

                            if x < 0 || (x > (stride - 1)) {
                                continue;
                            }
                            if y < 0 || (y > (num_lines - 1)) {
                                continue;
                            }
                            let ch = &lines[y as usize][x as usize];
                            if !is_digit(ch) && !is_empty(ch) {
                                matched = true;

                                // For each gear pivot, on this number, store its location
                                if *ch == b'*' {
                                    let key = ((stride as i32) * y) + x;
                                    gear_pivots.insert(key);
                                }
                            }
                        }
                    }

                    let num = extract_num(&line, start_num, end_num);
                    if matched {
                        total_engine += num;

                        // Walk each gear pivot, storing or number in it.
                        // This number matched against this gear
                        for p in &gear_pivots {
                            gears.entry(*p).or_insert(Vec::new()).push(num);
                        }
                    }
                }
                matched = false;
                start_num = -1;
                gear_pivots.clear();
            }
        }
    }

    if part == 0 {
        total_engine as usize
    }
    else {
        gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum::<u32>() as usize
    }
}
