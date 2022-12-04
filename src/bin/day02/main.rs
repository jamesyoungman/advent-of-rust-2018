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
            .or_insert(vec![*letter]);
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

fn checksum(s: &str) -> usize {
    let (count_two, count_three) = s
        .split_terminator('\n')
        .map(|line| two_or_three(line))
        .fold((0, 0), |(total_two, total_three), (two, three)| {
            (
                total_two + if two.is_some() { 1 } else { 0 },
                total_three + if three.is_some() { 1 } else { 0 },
            )
        });
    count_two * count_three
}

#[test]
fn test_checksum() {
    let cs = checksum("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n");
    assert_eq!(cs, 12);
}

fn main() {
    let text = str::from_utf8(include_bytes!("input.txt")).expect("valid input file");
    println!("Day 02 part 1: {}", checksum(text));
}
