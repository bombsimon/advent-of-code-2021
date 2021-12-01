# Advent of Code 2021

Let's go, [Advent of Code](https://adventofcode.com) 2021!

## Test

```sh
# Test all days so far
% cargo test

# Test a single day example
% cargo test day03
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running target/debug/deps/advent_of_code_2020-5c178449a4178058

running 2 tests
test solutions::day03::tests::part_one ... ok
test solutions::day03::tests::part_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out
```

## Execute

```sh
# Run a specific day
% carog run <day: i32>

# Example
% cargo run 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/advent-of-code-2020 1`
Showing solution for day 1

Solution part 1: 197451
Solution part 2: 138233720
```
