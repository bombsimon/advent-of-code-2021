use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

pub fn file_for_day(day: i32) -> Vec<String> {
    let input_file = format!("input/day{:02}", day);
    let f = File::open(input_file).unwrap();
    let r = BufReader::new(f);

    r.lines().filter_map(|l| l.ok()).collect()
}

pub fn test_vec(s: &str) -> Vec<String> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect::<Vec<_>>()
}

#[allow(unused)]
pub fn print_and_wait(print: String) {
    let mut s = String::new();

    println!("{}", print);
    print!("❯ PRESS ENTER TO CONTINUE...");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
}
