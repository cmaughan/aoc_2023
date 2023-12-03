use std::cmp;

#[aoc::main(2)]
fn main(input: &str) -> (usize, usize) {
    let games = input
        .lines()
        .map(|l| l.split(":").nth(1).unwrap())
        .collect::<Vec<_>>();
    let mut possibles: usize = 0;
    let mut mins: usize = 0;
    for (_id, g) in games.iter().enumerate() {
        let mut valid = true;
        let mut max: (usize, usize, usize) = (0, 0, 0);
        let draws = g.split(";").collect::<Vec<_>>();
        for draw in draws {
            let dice = draw.trim().split(",").map(|a| a.trim()).collect::<Vec<_>>();
            for pairs in dice {
                let (num, col) = (
                    pairs.split(" ").nth(0).unwrap().parse::<usize>().unwrap(),
                    pairs.split(" ").nth(1).unwrap(),
                );
                match col.chars().next().unwrap() {
                    'r' => {
                        if num > 12 {
                            valid = false;
                        }
                        max.0 = cmp::max(num, max.0)
                    }
                    'g' => {
                        if num > 13 {
                            valid = false;
                        }
                        max.1 = cmp::max(num, max.1)
                    }
                    'b' => {
                        if num > 14 {
                            valid = false;
                        }
                        max.2 = cmp::max(num, max.2)
                    }
                    _ => panic!(),
                }
            }
        }
        if valid {
            possibles += _id + 1;
        }
        mins += max.0 * max.1 * max.2;
    }

    // 2563, 70768
    (possibles as usize, mins as usize)
}
