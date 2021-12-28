use crate::input;

pub fn solve() {
    let x = input::file_for_day(11);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let mut x = input
        .iter()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total_flashes = 0i64;

    for _ in 0..100 {
        // Increase each column by one.
        for i in 0..x.len() {
            for j in 0..x[i].len() {
                x[i][j] += 1;
            }
        }

        // Flash if > 9.
        let flashed = flash(&mut x);

        // Increase nearby and flash.
        total_flashes += flashed.len() as i64 + increase_flashing_neigbors(&mut x, &flashed);

        // Reset all flashed octopuses to zero.
        for i in 0..x.len() {
            for j in 0..x[i].len() {
                if x[i][j] > 9 {
                    x[i][j] = 0;
                }
            }
        }
    }

    total_flashes
}

fn part_two(input: Vec<String>) -> i64 {
    let mut x = input
        .iter()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total_flashes = 0i64;

    for step in 1..9999 {
        // Increase each column by one.
        for i in 0..x.len() {
            for j in 0..x[i].len() {
                x[i][j] += 1;
            }
        }

        // Flash if > 9.
        let flashed = flash(&mut x);

        // Increase nearby and flash.
        total_flashes += flashed.len() as i64 + increase_flashing_neigbors(&mut x, &flashed);

        // Reset all flashed octopuses to zero.
        for i in 0..x.len() {
            for j in 0..x[i].len() {
                if x[i][j] > 9 {
                    x[i][j] = 0;
                }
            }
        }

        let sum = x.iter().fold(0, |acc, row| acc + row.iter().sum::<u32>());
        if sum == 0 {
            return step;
        }
    }

    total_flashes
}

#[allow(dead_code)]
fn print_grid(x: &mut Vec<Vec<u32>>) {
    println!("---");
    for i in x {
        for j in i {
            print!("{:<3}", if *j > 9 { *j } else { *j });
        }
        println!();
    }

    println!("---");
}

fn flash(x: &mut Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut flashed: Vec<(usize, usize)> = vec![];

    for i in 0..x.len() {
        for j in 0..x[i].len() {
            if x[i][j] > 9 && x[i][j] < 20 {
                // Increase to not flash again. Since an octopus can be increased at most 9 times
                // by surrouding octopuses we never flash octopuses over 20.
                x[i][j] = 20;

                flashed.push((i, j));
            }
        }
    }

    flashed
}

fn increase_flashing_neigbors(x: &mut Vec<Vec<u32>>, flashed: &[(usize, usize)]) -> i64 {
    if flashed.is_empty() {
        return 0;
    }

    for (i, j) in flashed {
        let i = *i;
        let j = *j;

        if j > 0 {
            x[i][j - 1] += 1;
        }

        if j < x[i].len() - 1 {
            x[i][j + 1] += 1;
        }

        if i > 0 {
            x[i - 1][j] += 1;

            if j > 0 {
                x[i - 1][j - 1] += 1;
            }

            if j < x[i].len() - 1 {
                x[i - 1][j + 1] += 1;
            }
        }

        if i < x.len() - 1 {
            x[i + 1][j] += 1;

            if j < x[i].len() - 1 {
                x[i + 1][j + 1] += 1;
            }

            if j > 0 {
                x[i + 1][j - 1] += 1;
            }
        }
    }

    let neighbor_flashed = flash(x);
    neighbor_flashed.len() as i64 + increase_flashing_neigbors(x, &neighbor_flashed)
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 1656;
    static SOLUTION_TWO: i64 = 195;
    static TEST_INPUT: &str = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

    #[test]
    fn part_one() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
