use crate::input;

pub fn solve() {
    let x = input::file_for_day(3);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(vec: Vec<String>) -> i32 {
    let most_common = most_common_bits(&vec)
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");

    let least_common = most_common_bits(&vec)
        .iter()
        .map(|x| if *x == 1 { 0 } else { 1 })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");

    let common = i32::from_str_radix(&most_common, 2).unwrap();
    let not_common = i32::from_str_radix(&least_common, 2).unwrap();

    common * not_common
}

fn part_two(vec: Vec<String>) -> i32 {
    let mut oxygen = vec
        .iter()
        .map(|r| {
            r.chars()
                .map(|e| e.to_string().parse::<u16>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let total_cols = oxygen[0].len();
    for i in 0..total_cols {
        oxygen = filter_rows(&oxygen, i, false);
        if oxygen.len() == 1 {
            break;
        }
    }

    let mut co2 = vec
        .iter()
        .map(|r| {
            r.chars()
                .map(|e| e.to_string().parse::<u16>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for i in 0..total_cols {
        co2 = filter_rows(&co2, i, true);
        if co2.len() == 1 {
            break;
        }
    }

    let oxygen_as_string = oxygen[0]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");

    let co2_as_string = co2[0]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");

    let o = i32::from_str_radix(&oxygen_as_string, 2).unwrap();
    let c = i32::from_str_radix(&co2_as_string, 2).unwrap();

    o * c
}

fn most_common_bits(vec: &[String]) -> Vec<u16> {
    let total_rows = vec.len();
    let total_cols = vec[0].len();
    let mut sum_bits: Vec<u16> = vec![Default::default(); total_cols];

    vec.iter().for_each(|e| {
        e.chars().enumerate().for_each(|(i, v)| {
            let v_as_i = v.to_string().parse::<u16>().unwrap();
            sum_bits[i] += v_as_i;
        });
    });

    sum_bits
        .iter()
        .map(|b| if *b as usize >= total_rows / 2 { 1 } else { 0 })
        .collect()
}

fn most_common_bits_2(vec: &[Vec<u16>]) -> Vec<u16> {
    let total_cols = vec[0].len();
    let mut sum_bits: Vec<(u16, u16)> = vec![Default::default(); total_cols];

    vec.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(i, v)| {
            if *v == 0 {
                sum_bits[i] = (sum_bits[i].0 + 1, sum_bits[i].1)
            } else {
                sum_bits[i] = (sum_bits[i].0, sum_bits[i].1 + 1)
            }
        })
    });

    sum_bits
        .iter()
        .map(|(zeroes, ones)| if ones >= zeroes { 1 } else { 0 })
        .collect()
}

fn filter_rows(vec: &[Vec<u16>], col: usize, flip: bool) -> Vec<Vec<u16>> {
    let mut filter = most_common_bits_2(vec);
    if flip {
        filter = filter
            .iter()
            .map(|e| if *e == 1 { 0 } else { 1 })
            .collect::<Vec<_>>();
    }

    let mut keep: Vec<Vec<u16>> = vec![];

    for row in vec {
        if row[col] == filter[col] {
            keep.push(row.clone());
        }
    }

    keep
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 198;
    static SOLUTION_TWO: i32 = 230;
    static TEST_INPUT: &str = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
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
