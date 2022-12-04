use std::collections::HashMap;
use std::str;

use lib::error::Fail;

fn first_repeat(c: &[i64]) -> Option<i64> {
    let mut seen: HashMap<i64, usize> = HashMap::new();
    seen.insert(0, 1);
    let mut current = 0;
    for n in c.iter().cycle() {
        current += n;
        seen.entry(current)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        match seen.get(&current) {
            Some(2) => {
                return Some(current);
            }
            _ => (),
        }
    }
    dbg!(&seen);
    None
}

fn get_input(text: &str) -> Result<Vec<i64>, Fail> {
    text.split_terminator('\n')
        .map(|line| {
            line.parse::<i64>()
                .map_err(|e| Fail(format!("invalid input [{line}]: {e}")))
        })
        .collect::<Result<Vec<i64>, _>>()
}

#[test]
fn test_part2() {
    assert_eq!(
        first_repeat(&get_input("+1\n-1\n").expect("wanted valid input")),
        Some(0)
    );
    assert_eq!(
        first_repeat(&get_input("+3\n+3\n+4\n-2\n-4\n").expect("wanted valid input")),
        Some(10)
    );
}

fn main() {
    let text = str::from_utf8(include_bytes!("input.txt")).unwrap();
    let frequencies = get_input(&text).expect("wanted valid input");
    let first_repeat: Option<i64> = first_repeat(&frequencies);
    println!("Day 01 part 1: {}", frequencies.iter().sum::<i64>());
    println!("Day 01 part 2: {}", first_repeat.expect("expected repeat"));
}
