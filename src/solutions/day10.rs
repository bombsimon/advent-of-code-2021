use crate::input;

pub fn solve() {
    let x = input::file_for_day(10);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let point_map = point_map(1);
    let bracket_map = bracket_map();
    let mut brackets: Vec<char> = vec![];

    input
        .iter()
        .map(|l| l.chars())
        .map(|row| {
            for c in row {
                match c {
                    '<' | '[' | '{' | '(' => brackets.push(c),
                    '>' | ']' | '}' | ')' => {
                        let last_bracket = brackets.pop().unwrap();
                        let must_match = *bracket_map.get(&c).unwrap();

                        if last_bracket != must_match {
                            return *point_map.get(&c).unwrap();
                        }
                    }
                    _ => unreachable!(),
                }
            }

            0
        })
        .sum()
}

fn part_two(input: Vec<String>) -> i64 {
    let bracket_map = bracket_map();
    let mut filter_brackets: Vec<char> = vec![];
    let mut missing_brackets: Vec<char> = vec![];

    let point_map = point_map(2);
    let mut scores: Vec<i64> = vec![];

    input
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        // Filter out invalid ones by checking any missmatching groups.
        .filter(|row| {
            for c in row {
                match c {
                    '<' | '[' | '{' | '(' => filter_brackets.push(*c),
                    '>' | ']' | '}' | ')' => {
                        let last_bracket = filter_brackets.pop().unwrap();
                        let must_match = bracket_map.get(&c).unwrap();

                        if last_bracket != *must_match {
                            return false;
                        }
                    }
                    _ => unreachable!("{}", c),
                }
            }

            true
        })
        // And for each incomplete, add the missing brackets and calculate the score.
        .for_each(|row| {
            for c in row {
                match c {
                    '<' | '[' | '{' | '(' => missing_brackets.push(*bracket_map.get(&c).unwrap()),
                    '>' | ']' | '}' | ')' => {
                        missing_brackets.pop();
                    }
                    _ => unreachable!("{}", c),
                }
            }

            scores.push(
                missing_brackets
                    .iter()
                    .rev()
                    .fold(0, |acc, c| acc * 5 + point_map.get(&c).unwrap()),
            );

            missing_brackets = vec![];
        });

    scores.sort();

    scores[scores.len() / 2 as usize]
}

fn bracket_map() -> std::collections::HashMap<char, char> {
    let mut bracket_map: std::collections::HashMap<char, char> = std::collections::HashMap::new();
    bracket_map.insert('>', '<');
    bracket_map.insert(']', '[');
    bracket_map.insert('}', '{');
    bracket_map.insert(')', '(');
    bracket_map.insert('<', '>');
    bracket_map.insert('[', ']');
    bracket_map.insert('{', '}');
    bracket_map.insert('(', ')');

    bracket_map
}

fn point_map(part: i32) -> std::collections::HashMap<char, i64> {
    let mut point_map: std::collections::HashMap<char, i64> = std::collections::HashMap::new();
    if part == 1 {
        point_map.insert(')', 3);
        point_map.insert(']', 57);
        point_map.insert('}', 1197);
        point_map.insert('>', 25137);
    } else {
        point_map.insert(')', 1);
        point_map.insert(']', 2);
        point_map.insert('}', 3);
        point_map.insert('>', 4);
    }

    point_map
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 26397;
    static SOLUTION_TWO: i64 = 288957;
    static TEST_INPUT: &str = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
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
