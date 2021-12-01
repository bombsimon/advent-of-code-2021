use crate::input;

use itertools::Itertools;

pub fn solve() {
    let x = input::file_for_day(1);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(vec: Vec<String>) -> i32 {
    vec.iter()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((-1, 0), |(previous, increases), v| {
            if previous < 1 || previous > v {
                (v, increases)
            } else {
                (v, increases + 1)
            }
        })
        .1
}

fn part_two(vec: Vec<String>) -> i32 {
    vec.iter()
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows::<(_, _, _)>()
        .map(|(x, y, z)| x + y + z)
        .fold((-1, 0), |(previous, increases), v| {
            if previous < 1 || v <= previous {
                (v, increases)
            } else {
                (v, increases + 1)
            }
        })
        .1
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 7;
    static SOLUTION_TWO: i32 = 5;
    static TEST_INPUT: &str = r#"
199
200
208
210
200
207
240
269
260
263
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
