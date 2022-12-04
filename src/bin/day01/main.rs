use std::str;

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt")).unwrap();
    let numbers: Vec<i64> = input
        .split_terminator('\n')
        .map(|line| line.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()
        .expect("parse error");
    println!("Day 01 part 1: {}", numbers.iter().sum::<i64>());
}
