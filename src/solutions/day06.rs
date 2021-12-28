use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(6);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    let mut fish = input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..80 {
        let mut eights: Vec<i32> = vec![];

        for age in fish.iter_mut() {
            *age = if *age == 0 {
                eights.push(8);
                6
            } else {
                *age - 1
            };
        }

        fish.append(&mut eights);
    }

    fish.len() as i64
}

fn part_two(input: String) -> i64 {
    let mut fish_by_age: std::collections::HashMap<i8, i64> = std::collections::HashMap::new();

    input
        .split(',')
        .map(|x| x.parse::<i8>().unwrap())
        .for_each(|x| {
            let count = fish_by_age.entry(x).or_default();
            *count += 1;
        });

    for _ in 0..256 {
        let mut daily_fishes: std::collections::HashMap<i8, i64> = std::collections::HashMap::new();

        for age in 0..=8 {
            let &mut count = fish_by_age.entry(age).or_default();

            let new_age = if age == 0 {
                let fish_of_age_eight = daily_fishes.entry(8).or_default();
                *fish_of_age_eight += count;

                6
            } else {
                age - 1
            };

            let fish_of_new_age = daily_fishes.entry(new_age).or_default();
            *fish_of_new_age += count;
        }

        fish_by_age = daily_fishes;
    }

    fish_by_age.iter().fold(0, |acc, (_, &count)| acc + count)
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 5934;
    static SOLUTION_TWO: i64 = 26984457539;
    static TEST_INPUT: &str = r#"
3,4,3,1,2
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
