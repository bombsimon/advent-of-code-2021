use itertools::Itertools;

use crate::input;

pub fn solve() {
    let x = input::file_for_day(8);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    input
        .iter()
        .map(|x| {
            let mut sides = x.split(" | ");
            (sides.next().unwrap(), sides.next().unwrap())
        })
        .map(|(_, x)| x.split_whitespace().collect::<Vec<_>>())
        .map(|x| {
            x.iter()
                .filter(|&&x| match x.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .collect::<Vec<_>>()
                .len()
        })
        .fold(0, |acc, v| acc + v as i64)
}

fn part_two(input: Vec<String>) -> i64 {
    // lines is a list of tuples containing the left and right side of the pipe where both sides in
    // turn is a vector of chars.
    let lines = &input
        .iter()
        .map(|x| {
            let mut sides = x.split(" | ");
            (sides.next().unwrap(), sides.next().unwrap())
        })
        .map(|(l, r)| {
            (
                l.split_whitespace()
                    .map(|x| x.chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                r.split_whitespace()
                    .map(|x| x.chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // The valid chars that can exist in groups to create digits.
    let chars = vec!["a", "b", "c", "d", "e", "f", "g"];
    let mut valid_combinations: std::collections::HashMap<i32, Vec<usize>> =
        std::collections::HashMap::new();

    // The indexes in the grid counting top -> down, left -> right for each digit.
    valid_combinations.insert(0, vec![0, 1, 2, 4, 5, 6]);
    valid_combinations.insert(1, vec![2, 5]);
    valid_combinations.insert(2, vec![0, 2, 3, 4, 6]);
    valid_combinations.insert(3, vec![0, 2, 3, 5, 6]);
    valid_combinations.insert(4, vec![1, 2, 3, 5]);
    valid_combinations.insert(5, vec![0, 1, 3, 5, 6]);
    valid_combinations.insert(6, vec![0, 1, 3, 4, 5, 6]);
    valid_combinations.insert(7, vec![0, 2, 5]);
    valid_combinations.insert(8, vec![0, 1, 2, 3, 4, 5, 6]);
    valid_combinations.insert(9, vec![0, 1, 2, 3, 5, 6]);

    // numbers is the vector with each number from each group on the right side of the pipe.
    let mut numbers: Vec<i64> = vec![];

    for x in lines {
        let mut valid_permutatin: Vec<&&str> = vec![];
        // Check each permutation that our letters may be in and brute force or way by checking
        // that all groups represent valid digits. This is slow and will take around 10 seconds for
        // the full input.
        'perm: for perm in chars.iter().permutations(chars.len()).unique() {
            'group: for group in &x.0 {
                // Given this permutation, give us the indexes that would be lit up for this group.
                let mut idxs: Vec<usize> = vec![];
                indexes_for_group_in_permutation(&mut idxs, &perm, group);

                // See if the current group is a valid digit with the current permutation.
                for (_, v) in &valid_combinations {
                    if idxs == *v {
                        // This group is valid, check next
                        continue 'group;
                    }
                }

                // This group is not valid with current permutation meaning it's not the right one.
                // Try another one.
                continue 'perm;
            }

            // We found the valid permutation, all groups are valid digit with this one. Just store
            // it and break out of the loop.
            valid_permutatin = perm;
            break;
        }

        // digits is the vector for all digits extracted frin the groups on the right side.
        let mut digits: Vec<i32> = vec![];

        'digit: for x in &x.1 {
            // Get the vector with all indexes for this digit.
            let mut idxs: Vec<usize> = vec![];
            indexes_for_group_in_permutation(&mut idxs, &valid_permutatin, x);

            // Loop over our valid combinations to figure out which digit is represented by this
            // index vector.
            for (&n, v) in &valid_combinations {
                if idxs == *v {
                    digits.push(n);

                    continue 'digit;
                }
            }
        }

        // Now just join our digits to a number and add it to our list of nubmers to sum.
        let number = digits.iter().join("").parse::<i64>().unwrap();
        numbers.push(number);
    }

    numbers.iter().sum()
}

fn indexes_for_group_in_permutation(idxs: &mut Vec<usize>, perm: &Vec<&&str>, group: &Vec<char>) {
    for c in group {
        let idx = perm
            .iter()
            .position(|&&x| x == c.to_string().as_str())
            .unwrap();

        idxs.push(idx);
    }

    // Sort our list so it matches the expected output.
    idxs.sort_unstable();
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 26;
    static SOLUTION_TWO: i64 = 61229;
    static TEST_INPUT: &str = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
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
