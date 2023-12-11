use itertools::Itertools;

#[aoc::main(11)]
fn main(part: i32, input: &str) -> usize {
    let mut data: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect_vec()).collect::<Vec<_>>();

    for x in 0..data[0].len() {
        let mut found = false;
        for y in 0..data.len() {
            if data[y][x] == b'#' {
                found = true;
                break;
            }
        }
        if !found {
            data[0][x] = 0;
        }
    }

    let mut loc = Vec::new();
    let mut yy: i64 = 0;

    for y in 0..data.len() {
        let mut xx: i64 = 0;

        let mut empty_row = true;
        for x in 0..data[0].len() {
            if data[y][x] == b'#' {
                loc.push((yy, xx));
                empty_row = false;
            }

            if data[0][x] == 0 {
                if part == 0 {
                    xx += 2;
                } else {
                    xx += 1000000;
                }
            } else {
                xx += 1;
            }
        }

        if empty_row {
            if part == 0 {
                yy += 2;
            } else {
                yy += 1000000;
            }
        } else {
            yy += 1;
        }
    }

    let mut total: i64 = 0;
    for (index_1, loc_1) in loc.iter().enumerate() {
        for (x, y) in loc.iter().skip(index_1 + 1) {
            let dist = ((loc_1.0 - x) as i64).abs() + ((loc_1.1 - y) as i64).abs();
            total += dist;
        }
    }

    // 9312968, 597714117556
    total as usize
}
