use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(14);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    count_after_insertion(input, 10)
}

fn part_two(input: String) -> i64 {
    count_after_insertion(input, 40)
}

fn count_after_insertion(input: String, iterations: usize) -> i64 {
    let mut parts = input.split("\n\n");
    let polymer_template = parts.next().unwrap();
    let par_insertion = parts.next().unwrap();

    let mut par_map: std::collections::HashMap<&str, char> = std::collections::HashMap::new();
    par_insertion.lines().for_each(|l| {
        let mut parts = l.split(" -> ");
        let ins = parts.next().unwrap();
        let val = parts.next().unwrap();

        par_map.insert(ins, val.chars().next().unwrap());
    });

    let polymer_list = polymer_template.chars().collect::<Vec<_>>();
    let mut m: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
    for i in 0..polymer_list.len() - 1 {
        let [a, b] = [polymer_list[i], polymer_list[i + 1]];
        *m.entry(format!("{}{}", a, b)).or_default() += 1;
    }

    for _ in 0..iterations {
        let mut new_groups: Vec<(String, i64)> = vec![];

        for (group, v) in m.iter_mut() {
            // This group doesn't exist anymore, nothign to split.
            if *v == 0 {
                continue;
            }

            let mut parts = group.chars();
            let a = parts.next().unwrap();
            let b = parts.next().unwrap();

            // The char to be inserted.
            let c = par_map.get(group.as_str()).unwrap();

            // When C is inserted between A and B it creates
            // AC
            // CB
            let new_a = format!("{}{}", a, c);
            let new_b = format!("{}{}", c, b);

            new_groups.push((new_a, *v));
            new_groups.push((new_b, *v));

            // This group will be split up with a letter inserted so reduce once to the count.
            *v = 0;
        }

        // For each group created in the iteration, increment the group count.
        for (group, count) in &new_groups {
            *m.entry(group.into()).or_default() += count;
        }
    }

    let mut count: std::collections::HashMap<char, i64> = std::collections::HashMap::new();
    *count.entry('N').or_default() += 1;

    for (group, c) in m {
        if c < 1 {
            continue;
        }

        let mut parts = group.chars();
        let _ = parts.next().unwrap();
        let b = parts.next().unwrap();

        *count.entry(b).or_default() += c;
    }

    let mut count_vec = count.iter().map(|(_, v)| *v).collect::<Vec<_>>();
    count_vec.sort();

    count_vec[count_vec.len() - 1] - count_vec[0]
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 1588;
    static SOLUTION_TWO: i64 = 2188189693529;
    static TEST_INPUT: &str = r#"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;

    #[test]
    fn part_one() {
        let x = input::test_vec_raw(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec_raw(TEST_INPUT);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
