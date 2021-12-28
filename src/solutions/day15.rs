use crate::input;
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self, grid: &[u32]) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        vec![Pos(x + 1, y), Pos(x - 1, y), Pos(x, y + 1), Pos(x, y - 1)]
            .into_iter()
            .filter_map(|p| get(grid, p.0, p.1).map(|weight| (p, weight as usize)))
            .collect()
    }
}

pub fn solve() {
    let x = input::file_for_day(15);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let height = input.len();
    let width = input[0].len();

    let mut grid: Vec<u32> = vec![0; (width * height) + 2];

    // We store the width and height of the grid in the last two positions to be able to boundary
    // check with just the grid.
    grid[width * height] = width as u32;
    grid[width * height + 1] = height as u32;

    input.iter().enumerate().for_each(|(i, line)| {
        for (j, char) in line.chars().enumerate() {
            let v = char.to_digit(10).unwrap();
            let pos = i * width + j;
            grid[pos] = v;
        }
    });

    let goal: Pos = Pos(width as i32 - 1, height as i32 - 1);
    let result = dijkstra(&Pos(0, 0), |p| p.successors(&grid), |p| *p == goal);

    result.expect("no path found").1 as i64
}

fn part_two(input: Vec<String>) -> i64 {
    let height = input.len() * 5;
    let width = input[0].len() * 5;

    let mut grid: Vec<u32> = vec![0; (width * height) + 2];

    // We store the width and height of the grid in the last two positions to be able to boundary
    // check with just the grid.
    grid[width * height] = width as u32;
    grid[width * height + 1] = height as u32;

    input.iter().enumerate().for_each(|(i, line)| {
        for (j, char) in line.chars().enumerate() {
            for xx in 0..5 {
                for yy in 0..5 {
                    let x = i + input.len() * xx;
                    let y = j + input[0].len() * yy;
                    let pos = x * width + y;

                    let v = char.to_digit(10).unwrap();

                    let value = (v + xx as u32 + yy as u32) % 9;
                    grid[pos] = if value == 0 { 9 } else { value };
                }
            }
        }
    });

    let goal: Pos = Pos(width as i32 - 1, height as i32 - 1);
    let result = dijkstra(&Pos(0, 0), |p| p.successors(&grid), |p| *p == goal);

    result.expect("no path found").1 as i64
}

fn get(grid: &[u32], x: i32, y: i32) -> Option<u32> {
    let width = grid[grid.len() - 2];
    let height = grid[grid.len() - 1];

    if x >= width as i32 || y >= height as i32 {
        return None;
    }

    match x * width as i32 + y {
        _ if x < 0 => None,
        _ if y < 0 => None,
        pos if pos > grid.len() as i32 - 1 => None,
        pos => Some(grid[pos as usize]),
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 40;
    static SOLUTION_TWO: i64 = 315;
    static TEST_INPUT: &str = r#"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
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
