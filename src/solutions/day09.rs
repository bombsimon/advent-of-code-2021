use crate::input;

pub fn solve() {
    let x = input::file_for_day(9);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let mut grid: Vec<Vec<i32>> = vec![vec![0; input[0].len()]; input.len()];

    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|col| col.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .enumerate()
        .for_each(|(row, columns)| {
            for (col, &val) in columns.iter().enumerate() {
                grid[row][col] = val as i32;
            }
        });

    let mut low_points: Vec<i32> = vec![];

    for (i, columns) in grid.iter().enumerate() {
        for (j, &column) in columns.iter().enumerate() {
            let left = if j == 0 { 9 } else { grid[i][j - 1] };
            let right = if j == columns.len() - 1 {
                9
            } else {
                grid[i][j + 1]
            };
            let up = if i == 0 { 9 } else { grid[i - 1][j] };
            let down = if i == grid.len() - 1 {
                9
            } else {
                grid[i + 1][j]
            };

            if left > column && right > column && up > column && down > column {
                low_points.push(column);
            }
        }
    }

    low_points.iter().fold(0, |acc, v| acc + *v as i64 + 1)
}

fn part_two(input: Vec<String>) -> i64 {
    let mut grid: Vec<Vec<i32>> = vec![vec![0; input[0].len()]; input.len()];

    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|col| col.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .enumerate()
        .for_each(|(row, columns)| {
            for (col, &val) in columns.iter().enumerate() {
                grid[row][col] = val as i32;
            }
        });

    let mut low_points: Vec<(usize, usize, i32)> = vec![];

    let height = grid.len();
    let width = grid[0].len();

    for (i, columns) in grid.iter().enumerate() {
        for (j, &column) in columns.iter().enumerate() {
            let left = if j == 0 { 9 } else { grid[i][j - 1] };
            let right = if j == width - 1 { 9 } else { grid[i][j + 1] };
            let up = if i == 0 { 9 } else { grid[i - 1][j] };
            let down = if i == height - 1 { 9 } else { grid[i + 1][j] };

            if left > column && right > column && up > column && down > column {
                low_points.push((i, j, column));
            }
        }
    }

    // println!("{:#?}", low_points);

    let mut sum: Vec<i64> = vec![];
    for (i, j, v) in low_points {
        let mut h: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        h.insert((i, j));
        check_adjecent(&grid, (i, j, v), &mut h);

        sum.push(h.len() as i64);
    }

    sum.sort_unstable();

    sum.iter()
        .rev()
        .take(3)
        .fold(0, |acc, &v| if acc == 0 { v } else { acc * v })
}

fn check_adjecent(
    grid: &[Vec<i32>],
    (i, j, x): (usize, usize, i32),
    h: &mut std::collections::HashSet<(usize, usize)>,
) {
    let height = grid.len();
    let width = grid[0].len();

    let left = if j == 0 { 0 } else { grid[i][j - 1] };
    let right = if j == width - 1 { 0 } else { grid[i][j + 1] };
    let up = if i == 0 { 0 } else { grid[i - 1][j] };
    let down = if i == height - 1 { 0 } else { grid[i + 1][j] };

    if left > x && left != 9 {
        h.insert((i, j - 1));
        check_adjecent(grid, (i, j - 1, left), h);
    }

    if right > x && right != 9 {
        h.insert((i, j + 1));
        check_adjecent(grid, (i, j + 1, right), h);
    }

    if up > x && up != 9 {
        h.insert((i - 1, j));
        check_adjecent(grid, (i - 1, j, up), h);
    }

    if down > x && down != 9 {
        h.insert((i + 1, j));
        check_adjecent(grid, (i + 1, j, down), h);
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 15;
    static SOLUTION_TWO: i64 = 1134;
    static TEST_INPUT: &str = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
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
