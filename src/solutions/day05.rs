use crate::input;

pub fn solve() {
    let x = input::file_for_day(5);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i32 {
    let mut coordinates: std::collections::HashMap<(i32, i32), i32> =
        std::collections::HashMap::new();

    input
        .iter()
        .map(|l| line_to_coordinates(l))
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .for_each(|((x1, y1), (x2, y2))| {
            let x_range = make_range(x1, x2, y1, y2);
            let y_range = make_range(y1, y2, x1, x2);

            x_range.iter().zip(&y_range).for_each(|(x, y)| {
                let c = (*x, *y);
                let current_seen = coordinates.entry(c).or_default();
                *current_seen += 1;
            });
        });

    coordinates.iter().filter(|(_, &v)| v > 1).count() as i32
}

fn part_two(input: Vec<String>) -> i32 {
    let mut coordinates: std::collections::HashMap<(i32, i32), i32> =
        std::collections::HashMap::new();

    input
        .iter()
        .map(|l| line_to_coordinates(l))
        .for_each(|((x1, y1), (x2, y2))| {
            let x_range = make_range(x1, x2, y1, y2);
            let y_range = make_range(y1, y2, x1, x2);

            x_range.iter().zip(&y_range).for_each(|(x, y)| {
                let c = (*x, *y);
                let current_seen = coordinates.entry(c).or_default();
                *current_seen += 1;
            });
        });

    coordinates.iter().filter(|(_, &v)| v > 1).count() as i32
}

fn line_to_coordinates(l: &str) -> ((i32, i32), (i32, i32)) {
    let mut sides = l.split(" -> ");
    let left = sides.next().unwrap();
    let right = sides.next().unwrap();

    (str_to_tuple(left), str_to_tuple(right))
}

fn str_to_tuple(s: &str) -> (i32, i32) {
    let mut numbers = s.split(',');
    let x = numbers.next().unwrap().parse::<i32>().unwrap();
    let y = numbers.next().unwrap().parse::<i32>().unwrap();

    (x, y)
}

fn make_range(start: i32, stop: i32, other_start: i32, other_stop: i32) -> Vec<i32> {
    if start == stop {
        let diff = ((other_start - other_stop).abs() + 1) as usize;
        vec![start; diff]
    } else if start > stop {
        (stop..=start).rev().collect::<Vec<_>>()
    } else {
        (start..=stop).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 5;
    static SOLUTION_TWO: i32 = 12;
    static TEST_INPUT: &str = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
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
