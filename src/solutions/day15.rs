use crate::input;

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

    let mut stack: Vec<(u32, u32, Vec<u32>)> = vec![(0u32, 0u32, vec![])];
    let mut paths: Vec<Vec<u32>> = vec![];
    let mut lowest = u32::MAX;
    let mut finished = 0;

    while stack.len() > 0 {
        let (x, y, path) = stack.pop().unwrap();

        // We're at the end, push the path this far and continue.
        if x >= width as u32 - 1 && y >= height as u32 - 1 {
            finished += 1;
            let value = path.iter().sum::<u32>();
            if value < lowest {
                println!("Found new low: {}", value);
                lowest = value;
            }

            continue;
        }

        // Might be out of bounds so just continue if we are.
        for (new_x, new_y) in [(x + 1, y), (x, y + 1)] {
            match get(&grid, new_x, new_y) {
                Some(v) => {
                    let mut p = path.clone();
                    p.push(v);
                    stack.push((new_x, new_y, p));
                }
                None => continue,
            }
        }
    }

    return lowest as i64;
    paths.iter().map(|l| l.iter().sum::<u32>()).min().unwrap() as i64
}

fn part_two(_input: Vec<String>) -> i64 {
    1
}

fn get(grid: &[u32], x: u32, y: u32) -> Option<u32> {
    let width = grid[grid.len() - 2];
    let height = grid[grid.len() - 1];

    if x >= width || y >= height {
        return None;
    }

    match x * width + y {
        pos if pos > grid.len() as u32 - 1 => None,
        pos => Some(grid[pos as usize]),
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 40;
    static SOLUTION_TWO: i64 = 1;
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
