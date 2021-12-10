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
        .map(|l| l.chars().collect::<Vec<_>>())
        .map(|row| {
            for c in row {
                match c {
                    '<' | '[' | '{' | '(' => {
                        brackets.push(c);
                    }
                    '>' | ']' | '}' | ')' => {
                        let last_bracket = brackets[brackets.len() - 1];
                        let must_match = *bracket_map.get(&c).unwrap();

                        if last_bracket != must_match {
                            return *point_map.get(&c).unwrap();
                        } else {
                            brackets.remove(brackets.len() - 1);
                        }
                    }
                    _ => unreachable!("{}", c),
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
        .filter(|row| {
            for c in row {
                match c {
                    '<' | '[' | '{' | '(' => {
                        filter_brackets.push(*c);
                    }
                    '>' | ']' | '}' | ')' => {
                        let last_bracket = filter_brackets[filter_brackets.len() - 1];
                        let must_match = bracket_map.get(&c).unwrap();

                        if last_bracket != *must_match {
                            return false;
                        } else {
                            filter_brackets.remove(filter_brackets.len() - 1);
                        }
                    }
                    _ => unreachable!("{}", c),
                }
            }

            true
        })
        .for_each(|row| {
            for c in row {
                match c {
                    '<' | '[' | '{' | '(' => {
                        missing_brackets.push(c);
                    }
                    '>' | ']' | '}' | ')' => {
                        missing_brackets.remove(missing_brackets.len() - 1);
                    }
                    _ => unreachable!("{}", c),
                }
            }

            let mut matching_missing_brackets: Vec<char> = vec![];
            for c in missing_brackets.iter().rev() {
                let matching = bracket_map.get(c).unwrap();
                matching_missing_brackets.push(*matching);
            }

            let mut score: i64 = 0;
            for c in matching_missing_brackets {
                let bracket_worth = point_map.get(&c).unwrap();
                score = score * 5 + bracket_worth;
            }

            scores.push(score);

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
