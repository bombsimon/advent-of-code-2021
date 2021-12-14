use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(14);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    let mut parts = input.split("\n\n");
    let polymer_template = parts.next().unwrap();
    let par_insertion = parts.next().unwrap();

    let mut polymer_list = polymer_template.chars().collect::<Vec<_>>();
    let mut par_map: std::collections::HashMap<&str, char> = std::collections::HashMap::new();

    par_insertion.lines().for_each(|l| {
        let mut parts = l.split(" -> ");
        let ins = parts.next().unwrap();
        let val = parts.next().unwrap();

        par_map.insert(ins, val.chars().next().unwrap());
    });

    let mut f = vec![];

    for _ in 0..10 {
        let mut polymer = polymer_list.clone();
        for i in 0..polymer_list.len() - 1 {
            let [a, b] = [polymer_list[i], polymer_list[i + 1]];
            let group = format!("{}{}", a, b);
            let c = par_map.get(group.as_str()).unwrap();

            polymer.insert(i + i + 1, *c);
        }

        polymer_list = polymer.clone();
        f = polymer;
    }

    let mut count: std::collections::HashMap<char, i64> = std::collections::HashMap::new();
    for c in f {
        *count.entry(c).or_default() += 1;
    }

    let mut count_vec = count.iter().map(|(_, v)| *v).collect::<Vec<_>>();
    count_vec.sort();

    count_vec[count_vec.len() - 1] - count_vec[0]
}

fn part_two(_input: String) -> i64 {
    1
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 1588;
    static SOLUTION_TWO: i64 = 1;
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
