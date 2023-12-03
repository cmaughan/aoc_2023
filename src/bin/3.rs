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
fn main(input: &str) -> (usize, usize) {

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

                for x in -1..=1 {
                    for y in -1..=1 {
                        if (x == 0) && (y == 0) {
                            continue;
                        }
                        let grid_indices = (char_index as i32 + x, line_index as i32 + y);
                        if grid_indices.0 < 0 || (grid_indices.0 > (stride - 1)) {
                            continue;
                        }
                        if grid_indices.1 < 0 || (grid_indices.1 > (num_lines - 1)) {
                            continue;
                        }
                        let ch = &lines[grid_indices.1 as usize][grid_indices.0 as usize];
                        if !is_digit(ch) && !is_empty(ch) {
                            matched = true;

                            // For each gear pivot, on this number, store its location
                            if *ch == b'*' {
                                let key = ((stride as i32) * grid_indices.1) + grid_indices.0;
                                gear_pivots.insert(key);
                            }
                        }
                    }
                }
            }
            if !is_digit(ch) || (char_index == (stride - 1) as usize) {
                if start_num != -1 {
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

    let total_gears: u32 = gears.iter().filter(|(_, v)| v.len() == 2).map(|(_, v)| v[0] * v[1]).sum();

    (total_engine as usize, total_gears as usize)
}
