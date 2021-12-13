use crate::input;

pub fn solve() {
    let x = input::file_for_day(12);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(_input: Vec<String>) -> i64 {
    1
}

fn part_two(_input: Vec<String>) -> i64 {
    1
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 10;
    static SOLUTION_TWO: i64 = 1;
    static TEST_INPUT_SMALL: &str = r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;
    static _TEST_INPUT: &str = r#"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;

    #[test]
    fn part_one() {
        let x = input::test_vec(TEST_INPUT_SMALL);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec(TEST_INPUT_SMALL);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
