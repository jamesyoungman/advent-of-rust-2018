use itertools::Itertools;
use lib::error::Fail;
use std::collections::HashMap;
use std::str;

fn letters_by_freq(s: &str) -> HashMap<usize, Vec<char>> {
    let mut freqs_by_letter: HashMap<char, usize> = HashMap::new();
    for ch in s.chars() {
        freqs_by_letter
            .entry(ch)
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
    }
    let mut letters_by_freq: HashMap<usize, Vec<char>> = HashMap::new();
    for (letter, freq) in freqs_by_letter.iter() {
        letters_by_freq
            .entry(*freq)
            .and_modify(|letters| letters.push(*letter))
            .or_insert_with(|| vec![*letter]);
    }
    letters_by_freq
}

fn two_or_three(s: &str) -> (Option<usize>, Option<usize>) {
    let lbf = letters_by_freq(s);
    let two = lbf.get(&2).map(|letters| letters.len());
    let three = lbf.get(&3).map(|letters| letters.len());
    (two, three)
}

#[test]
fn test_two_or_three() {
    assert_eq!(two_or_three("abcdef"), (None, None));
    assert_eq!(two_or_three("bababc"), (Some(1), Some(1)));
    assert_eq!(two_or_three("abbcde"), (Some(1), None));
    assert_eq!(two_or_three("abcccd"), (None, Some(1)));
    assert_eq!(two_or_three("aabcdd"), (Some(2), None));
    assert_eq!(two_or_three("abcdee"), (Some(1), None));
    assert_eq!(two_or_three("ababab"), (None, Some(2)));
}

fn get_ids(s: &str) -> Vec<&str> {
    s.split_terminator('\n').collect()
}

fn checksum(ids: &[&str]) -> usize {
    let (count_two, count_three) = ids.iter().map(|line| two_or_three(line)).fold(
        (0, 0),
        |(total_two, total_three), (two, three)| {
            (
                total_two + if two.is_some() { 1 } else { 0 },
                total_three + if three.is_some() { 1 } else { 0 },
            )
        },
    );
    count_two * count_three
}

#[test]
fn test_checksum() {
    let ids = get_ids("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n");
    assert_eq!(checksum(&ids), 12);
}

fn diffcount(left: &str, right: &str) -> usize {
    left.chars()
        .zip(right.chars())
        .filter(|(l, r)| l != r)
        .count()
}

#[test]
fn test_diffcount() {
    assert_eq!(diffcount("abcde", "axcye"), 2);
    assert_eq!(diffcount("fghij", "fguij"), 1);
}

fn find_id_pair<'a, 'b>(ids: &'a [&'b str]) -> Option<(&'b str, &'b str)> {
    for (left, right) in ids
        .iter()
        .cartesian_product(ids.iter())
        .filter(|(l, r)| l != r)
    {
        if diffcount(left, right) == 1 {
            return Some((left, right));
        }
    }
    None
}

#[test]
fn test_id_pair() {
    let ids = vec![
        "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
    ];
    assert_eq!(find_id_pair(&ids), Some(("fghij", "fguij")));
}

fn solve_part2(ids: &[&str]) -> Result<String, Fail> {
    match find_id_pair(ids) {
        None => Err(Fail("no suitable pair".to_string())),
        Some((left, right)) => {
            let rest: String = left
                .chars()
                .zip(right.chars())
                .filter_map(|(l, r)| if l == r { Some(r) } else { None })
                .collect();
            Ok(rest)
        }
    }
}

#[test]
fn test_solve_part2() {
    let ids = vec![
        "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
    ];
    match solve_part2(&ids) {
        Ok(s) if &s == "fgij" => (),
        Ok(other) => {
            panic!("wrong solution: {other}");
        }
        Err(e) => {
            panic!("solution failed: {e}");
        }
    }
}

fn main() {
    let ids = get_ids(str::from_utf8(include_bytes!("input.txt")).expect("valid input file"));
    println!("Day 02 part 1: {}", checksum(&ids));
    println!("Day 02 part 2: {}", solve_part2(&ids).expect("solution"));
}
