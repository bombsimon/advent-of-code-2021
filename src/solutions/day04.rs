use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(4);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i32 {
    let mut groups = input.split("\n\n");
    let series = groups
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = groups
        .map(|b| {
            b.split("\n")
                .filter(|l| !l.is_empty())
                .map(|r| {
                    r.split_whitespace()
                        .map(|x| (x.parse::<i32>().unwrap(), false))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for n in series {
        update_boards(&mut boards, n);

        for board in &boards {
            if has_bingo(&board) {
                let sum = board
                    .iter()
                    .flat_map(|f| f.iter())
                    .cloned()
                    .fold(0, |acc, (n, drawn)| if !drawn { acc + n } else { acc });

                return n * sum;
            }
        }
    }

    unreachable!();
}

fn part_two(input: String) -> i32 {
    let mut groups = input.split("\n\n");
    let series = groups
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = groups
        .map(|b| {
            b.split("\n")
                .filter(|l| !l.is_empty())
                .map(|r| {
                    r.split_whitespace()
                        .map(|x| (x.parse::<i32>().unwrap(), false))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut last_board: Vec<Vec<(i32, bool)>> = vec![];

    for n in series {
        update_boards(&mut boards, n);

        // If only one day left, store the last board so we can get the sum from it and iterate until it gets bingo!
        if boards.len() == 1 && last_board.len() == 0 {
            last_board = boards[0].clone();
        }

        boards.retain(|b| !has_bingo(b));

        // The last board got bingo, calculate the sum and return the result.
        if boards.len() == 0 {
            let sum = last_board
                .iter()
                .flat_map(|f| f.iter())
                .cloned()
                .fold(0, |acc, (n, drawn)| if !drawn { acc + n } else { acc });

            return n * sum;
        }
    }

    unreachable!();
}

/// Print the board in a human readable way indicating drawn numbers with parentheses.
#[allow(dead_code)]
fn print_board(board: &[Vec<(i32, bool)>]) {
    for row in board {
        for (v, drawn) in row {
            if *drawn {
                print!(" ({:2}) ", v);
            } else {
                print!("  {:2}  ", v);
            }
        }
        println!();
    }
}

///  Mark the number as drawn by checking all rows and columns.
fn update_boards(boards: &mut [Vec<Vec<(i32, bool)>>], number: i32) {
    for board in boards {
        for row in board {
            for col in row {
                if col.0 == number {
                    col.1 = true
                }
            }
        }
    }
}

/// Iterate over the board to see if it has bingo. This is done by iterating
/// column by column and if the column doesn't have bingo, check if the row has
/// bingo.
fn has_bingo(board: &[Vec<(i32, bool)>]) -> bool {
    let columns = board[0].len();

    let mut col_bingo = false;

    for i in 0..columns {
        if col_bingo {
            return true;
        }

        // Reset col_bingo so we can check each column.
        col_bingo = true;

        for row in board {
            // Only check the row the first iteration to avoid duplication.
            if i == 0 {
                let row_bingo = !row.iter().any(|(_, drawn)| *drawn == false);
                if row_bingo {
                    return true;
                }
            }

            col_bingo = col_bingo && row[i].1
        }
    }

    // Return the final result of col_bingo since the last column in the last
    // iteration might have bingo.
    col_bingo
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 4512;
    static SOLUTION_TWO: i32 = 1924;
    static TEST_INPUT: &str = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
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
