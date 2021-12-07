use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(7);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    let x = input
        .trim_end()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut lowest_total = i64::MAX;
    let min = *x.iter().min().unwrap();
    let max = *x.iter().max().unwrap();

    for n in min..max {
        let total_cost = x.iter().fold(0, |acc, &x| acc + (n - x).abs());
        if total_cost < lowest_total {
            lowest_total = total_cost;
        }
    }

    lowest_total
}

fn part_two(input: String) -> i64 {
    let x = input
        .trim_end()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut lowest_total = i64::MAX;
    let min = *x.iter().min().unwrap();
    let max = *x.iter().max().unwrap();

    for n in min..max {
        let total_cost = x.iter().fold(0, |acc, &x| {
            let diff: i64 = (n - x).abs();
            let cost: i64 = diff * (diff + 1) / 2;
            acc + cost
        });

        if total_cost < lowest_total {
            lowest_total = total_cost;
        }
    }

    lowest_total
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 37;
    static SOLUTION_TWO: i64 = 168;
    static TEST_INPUT: &str = r#"
16,1,2,0,4,2,7,1,2,14
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
