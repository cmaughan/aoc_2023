use itertools::Itertools;

#[aoc::main(11)]
fn main(part: i32, input: &str) -> usize {
    let data = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let empty_rows = data
        .iter()
        .enumerate()
        .filter(|(index, chars)| chars.iter().position(|&v| v == b'#').is_none())
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let mut empty_columns = Vec::new();
    for x in 0..data[0].len() {
        let mut found = false;
        for y in 0..data.len() {
            if data[y][x] == b'#' {
                found = true;
                break;
            }
        }
        if !found {
            empty_columns.push(x);
        }
    }

    let mut loc = Vec::new();
    let mut yy: i64 = 0;
    let mut current_y = empty_rows.iter().peekable();

    for y in 0..data.len() {
        let mut current_x = empty_columns.iter().peekable();
        let mut xx :i64 = 0;
        for x in 0..data[0].len() {
            if data[y][x] == b'#' {
                loc.push((yy, xx));
            }

            if let Some(&next_x) = current_x.peek() {
                if *next_x <= x {
                    if part == 0 {
                        xx += 2;
                    } else {
                        xx += 1000000;
                    }
                    current_x.next();
                    continue;
                } 
            }
            xx += 1;
        }

        if let Some(&next_y) = current_y.peek() {
            if *next_y <= y {
                if part == 0 {
                    yy += 2;
                }
                else {
                    yy += 1000000;
                }
                current_y.next();
                continue;
            }
        }
        yy += 1;
    }

    let mut total = 0;
    for indices in (0..loc.len()).combinations(2) {
        let a = loc[indices[0]];
        let b = loc[indices[1]];
        let dist = ((a.0 - b.0) as i64).abs() + ((a.1 - b.1) as i64).abs();
        total += dist;
    }

    // 9312968, 597714117556
    total as usize
}
