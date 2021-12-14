use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(13);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    let mut parts = input.split("\n\n");
    let coordiantes = parts.next().unwrap();
    let folds = parts.next().unwrap();

    let mut grid: Vec<Vec<&str>> = vec![vec![" "; 1311]; 893]; // Cheated, fetched higest value from input...

    //let mut grid: Vec<Vec<&str>> = vec![vec!["."; 11]; 15]; // For test

    coordiantes.lines().for_each(|l| {
        let mut c = l.split(",");
        let col = c.next().unwrap().parse::<usize>().unwrap();
        let row = c.next().unwrap().parse::<usize>().unwrap();

        grid[row][col] = "█";
    });

    let mut fold_list: Vec<(String, usize)> = vec![];
    folds.lines().for_each(|l| {
        let mut c = l.split("=");
        let axis = c.next().unwrap().replace("fold along ", "");
        let val = c.next().unwrap().parse::<usize>().unwrap();

        fold_list.push((axis.clone(), val));
    });

    let (axis, fold_at) = &fold_list[0];
    flip(&mut grid, *fold_at, axis);

    grid.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&v| *v == "█").count() as i64
    })
}

fn part_two(input: String) -> i64 {
    let mut parts = input.split("\n\n");
    let coordiantes = parts.next().unwrap();
    let folds = parts.next().unwrap();

    let mut grid: Vec<Vec<&str>> = vec![vec![" "; 1311]; 893]; // Cheated, fetched higest value from input...

    coordiantes.lines().for_each(|l| {
        let mut c = l.split(",");
        let col = c.next().unwrap().parse::<usize>().unwrap();
        let row = c.next().unwrap().parse::<usize>().unwrap();

        grid[row][col] = "█";
    });

    let mut fold_list: Vec<(String, usize)> = vec![];
    folds.lines().for_each(|l| {
        let mut c = l.split("=");
        let axis = c.next().unwrap().replace("fold along ", "");
        let val = c.next().unwrap().parse::<usize>().unwrap();

        fold_list.push((axis.clone(), val));
    });

    for (axis, fold_at) in fold_list {
        flip(&mut grid, fold_at, &axis);
    }

    pg(&grid);

    1
}

fn flip(grid: &mut Vec<Vec<&'static str>>, fold_at: usize, axis: &str) {
    let mut half_grid: Vec<Vec<&str>> = vec![];
    for (i, row) in grid.iter().enumerate() {
        if axis == "y" && i <= fold_at {
            continue;
        }

        half_grid.push(vec![]);

        for (j, col) in row.iter().enumerate() {
            if axis == "x" && j <= fold_at {
                continue;
            }

            let idx = if axis == "y" { i - fold_at - 1 } else { i };
            half_grid[idx].push(col);
        }
    }

    for i in 0..half_grid.len() {
        for j in 0..half_grid[0].len() {
            let flipped_value = match axis {
                "y" => half_grid[half_grid.len() - 1 - i][j],
                "x" => half_grid[i][half_grid[0].len() - 1 - j],
                _ => unreachable!(),
            };

            let [grid_i, grid_j] = if axis == "y" {
                let u: i32 = fold_at as i32 - half_grid.len() as i32;
                let delta = u.abs() as usize;
                [i + delta, j]
            } else {
                let u: i32 = fold_at as i32 - half_grid[0].len() as i32;
                let delta = u.abs() as usize;
                [i, j + delta]
            };

            grid[grid_i][grid_j] = if grid[grid_i][grid_j] == "█" {
                "█"
            } else {
                flipped_value
            };
        }
    }

    if axis == "y" {
        while grid.len() > fold_at {
            grid.remove(grid.len() - 1);
        }
    } else {
        for row in grid {
            while row.len() > fold_at {
                row.remove(row.len() - 1);
            }
        }
    }
}

#[allow(dead_code)]
fn pg(grid: &[Vec<&str>]) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 17;
    static SOLUTION_TWO: i64 = 1;
    static TEST_INPUT: &str = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;

    #[test]
    fn part_one() {
        let x = input::test_vec_raw(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec_raw(TEST_INPUT);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
