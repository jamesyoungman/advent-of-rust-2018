use std::str;

fn invert_polarity(ch: char) -> char {
    if ch.is_ascii_uppercase() {
        ch.to_ascii_lowercase()
    } else {
        ch.to_ascii_uppercase()
    }
}

fn have_opposite_polarity(a: char, b: char) -> bool {
    invert_polarity(a) == b
}

fn react_once(letters: &[char]) -> (Vec<char>, bool) {
    let mut result = Vec::with_capacity(letters.len());
    let mut changed = false;
    // Initialise `leftover` in case there are less than 2 chars in
    // `letters`.
    let mut leftover: Option<char> = letters.iter().copied().next();
    let mut skip = false;
    for pair in letters.windows(2) {
        //dbg!(pair);
        //dbg!(&skip);
        if skip {
            skip = false;
            leftover = Some(pair[1]);
            continue;
        }
        match pair {
            &[a, b] if have_opposite_polarity(a, b) => {
                //println!("eating {pair:?}");
                leftover = None;
                changed = true;
                // Make sure we don't consider b as the first letter
                // of a pair next time around the loop.
                skip = true;
            }
            &[a, b] => {
                leftover = Some(b);
                result.push(a);
                //println!(
                //    "passing through {a}, setting leftover={}",
                //    leftover.unwrap()
                //);
            }
            _ => unreachable!(),
        }
    }
    if let Some(ch) = leftover {
        //println!("passing through the leftover {ch}");
        result.push(ch);
    } else {
        //println!("There is no leftover");
    }
    (result, changed)
}

#[test]
fn test_react_once() {
    assert_eq!(react_once(&['a']), (vec!['a'], false));
    assert_eq!(react_once(&['a', 'a']), (vec!['a', 'a'], false));
    assert_eq!(react_once(&['a', 'A']), (vec![], true));
    assert_eq!(react_once(&['b', 'a', 'A']), (vec!['b'], true));
    assert_eq!(
        react_once(&['b', 'a', 'A', 'B']),
        (vec!['b', 'B'], true),
        "baAB should yield bB"
    );
    assert_eq!(react_once(&['b', 'B', 'a', 'A']), (vec![], true));
}

fn react_fully(letters: &str) -> String {
    let mut input: Vec<char> = letters.chars().collect();
    loop {
        let (output, changed) = react_once(&input);
        if !changed {
            return input.iter().collect();
        }
        assert_ne!(input, output);
        input = output;
    }
}

#[test]
fn test_react_fully() {
    assert_eq!(react_fully("baAB"), "");
    assert_eq!(&react_fully("a"), "a");
    assert_eq!(react_fully("aa"), "aa");
    assert_eq!(react_fully("aA"), "");
    assert_eq!(react_fully("baA"), "b");
    assert_eq!(react_fully("aAbB"), "");
}

#[test]
fn test_provided_example() {
    assert_eq!(react_fully("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
}

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt"))
        .expect("input file should be correctly encoded")
        .trim();
    let part1 = react_fully(input);
    println!("Day 05 part 1: {}", part1.len());
}
