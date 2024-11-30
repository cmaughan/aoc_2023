use itertools::Itertools;

#[aoc::main(11)]
fn main(part: i32, input: &str) -> usize {
    let mut data: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect_vec()).collect::<Vec<_>>();

    let x_len = data[0].len();
    let y_len = data.len();

    for x in 0..x_len {
        let mut found = false;
        for y in 0..y_len {
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

    for y in 0..y_len {
        let mut xx: i64 = 0;

        let mut empty_row = true;
        for x in 0..x_len {
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
        for (y, x) in loc.iter().skip(index_1 + 1) {
            let dist = ((y - loc_1.0) as i64) + ((loc_1.1 - x).abs() as i64);
            total += dist;
        }
    }

    // 9312968, 597714117556
    total as usize
}
