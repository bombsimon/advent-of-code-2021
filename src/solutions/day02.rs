use crate::input;

pub fn solve() {
    let x = input::file_for_day(2);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(vec: Vec<String>) -> i32 {
    let t = vec.iter().fold((0, 0), |(x, y), v| {
        let mut s = v.split_whitespace();
        let direction = s.next().unwrap();
        let step = s.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => (x + step, y),
            "down" => (x, y + step),
            "up" => (x, y - step),
            x => unreachable!(x),
        }
    });

    t.0 * t.1
}

fn part_two(vec: Vec<String>) -> i32 {
    let t = vec.iter().fold((0, 0, 0), |(x, y, aim), v| {
        let mut s = v.split_whitespace();
        let direction = s.next().unwrap();
        let step = s.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => (x + step, y + (aim * step), aim),
            "down" => (x, y, aim + step),
            "up" => (x, y, aim - step),
            x => unreachable!(x),
        }
    });

    t.0 * t.1
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 150;
    static SOLUTION_TWO: i32 = 900;
    static TEST_INPUT: &str = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
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
