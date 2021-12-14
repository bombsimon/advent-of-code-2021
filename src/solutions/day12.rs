use crate::input;

pub fn solve() {
    let x = input::file_for_day(12);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let mut nodes: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();

    input.iter().for_each(|l| {
        let mut parts = l.split("-");
        let lhs = parts.next().unwrap();
        let rhs = parts.next().unwrap();

        let l = nodes.entry(lhs).or_default();
        l.push(rhs);

        let r = nodes.entry(rhs).or_default();
        r.push(lhs);
    });

    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    seen.insert("start");

    traverse(&nodes, &mut seen, "start", true)
}

fn part_two(input: Vec<String>) -> i64 {
    let mut nodes: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();

    input.iter().for_each(|l| {
        let mut parts = l.split("-");
        let lhs = parts.next().unwrap();
        let rhs = parts.next().unwrap();

        let l = nodes.entry(lhs).or_default();
        l.push(rhs);

        let r = nodes.entry(rhs).or_default();
        r.push(lhs);
    });

    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    seen.insert("start");

    traverse(&nodes, &mut seen, "start", false)
}

fn traverse<'a>(
    nodes: &std::collections::HashMap<&str, Vec<&'a str>>,
    seen: &'a mut std::collections::HashSet<&'a str>,
    node: &'a str,
    has_visit_twice: bool,
) -> i64 {
    if node == "end" {
        return 1;
    }

    if !is_uppercase(node) {
        seen.insert(node);
    }

    let neighbors = nodes.get(node).unwrap();
    let mut sum = 0i64;

    for &n in neighbors {
        let mut has_visisted = has_visit_twice;
        if seen.contains(n) {
            if has_visisted || n == "start" {
                continue;
            }

            has_visisted = true;
        }

        let mut seen_this_far = seen.clone();
        sum += traverse(nodes, &mut seen_this_far, n, has_visisted);
    }

    sum
}

fn is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 10;
    static SOLUTION_TWO: i64 = 36;
    static TEST_INPUT: &str = r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end
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
