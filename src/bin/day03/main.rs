use std::collections::BTreeMap;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::str;

use itertools::Itertools;
use regex::Regex;

use lib::error::Fail;

//#1 @ 1,3: 4x4
//#2 @ 3,1: 4x4
//#3 @ 5,5: 2x2

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Square {
    left: u32,
    top: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Claim {
    id: usize,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn squares(&self) -> impl Iterator<Item = Square> {
        let columns = self.left..(self.left + self.width);
        let rows = self.top..(self.top + self.height);
        columns
            .cartesian_product(rows)
            .map(|(c, r)| Square { left: c, top: r })
    }
}

const CLAIM_REGEX: &str = r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$";

struct ClaimParser {
    rx: Regex,
}

impl ClaimParser {
    fn new() -> Result<ClaimParser, Fail> {
        // example: "#1 @ 1,3: 4x4"
        let rx = Regex::new(CLAIM_REGEX).map_err(|e| Fail(e.to_string()))?;
        Ok(ClaimParser { rx })
    }
}

impl ClaimParser {
    fn parse(&self, claim: &str) -> Result<Claim, Fail> {
        let invalid = |why: &str| format!("invalid claim {claim}: {why}");
        match self.rx.captures(claim) {
            None => Err(Fail(format!(
                "claim {claim} does not match regular expression {CLAIM_REGEX}"
            ))),
            Some(captures) => {
                match (
                    captures.get(1),
                    captures.get(2),
                    captures.get(3),
                    captures.get(4),
                    captures.get(5),
                ) {
                    (Some(id), Some(left), Some(top), Some(width), Some(height)) => {
                        match (
                            id.as_str().parse(),
                            left.as_str().parse(),
                            top.as_str().parse(),
                            width.as_str().parse(),
                            height.as_str().parse(),
                        ) {
                            (Ok(id), Ok(left), Ok(top), Ok(width), Ok(height)) => Ok(Claim {
                                id,
                                left,
                                top,
                                width,
                                height,
                            }),
                            _ => Err(Fail(invalid("non-numeric field"))),
                        }
                    }
                    _ => Err(Fail(invalid("regex did not match"))),
                }
            }
        }
    }
}

#[test]
fn test_claim_parser() {
    let parser = ClaimParser::new().expect("should be able to instantiate a ClaimParser");
    assert!(parser.parse("#1 @ 1,3: 4x4").is_ok());
    assert!(parser.parse("1 @ 1,3: 4x4").is_err());
    assert!(parser.parse("#1@ 1,3: 4x4").is_err());
    assert!(parser.parse("#1 @ a,3: 4x4").is_err());
    assert!(parser.parse("#1 @ 3,a: 4x4").is_err());
    assert!(parser.parse("#1 @ 3,: 4x4").is_err());
    assert!(parser.parse("#1 @ ,3: 4x4").is_err());
    assert!(parser.parse("#1 @ 1,3: 4y4").is_err());
    assert!(parser.parse("#1 @ 1,3: x4").is_err());
    assert!(parser.parse("#1 @ 1,3: 4x").is_err());
    assert!(!parser.parse("#1 @ 1,3: 4x4z").is_ok());

    assert_eq!(
        parser.parse("#1 @ 1,3: 4x4").expect("test data is valid"),
        Claim {
            id: 1,
            left: 1,
            top: 3,
            width: 4,
            height: 4,
        }
    );
}

fn get_claims(text: &str) -> Result<Vec<Claim>, Fail> {
    let parser = ClaimParser::new()?;
    text.split_terminator('\n')
        .map(|line| parser.parse(line))
        .collect()
}

fn count_overlap_squares(claims: &[Claim]) -> usize {
    let mut taken = HashSet::new();
    let mut overlaps: HashSet<Square> = HashSet::new();
    for square in claims.iter().flat_map(|claim| claim.squares()) {
        if !taken.insert(square.clone()) {
            overlaps.insert(square);
        }
    }
    overlaps.len()
}

#[test]
fn test_count_overlap_squares() {
    let claims = get_claims(concat!(
        "#1 @ 1,3: 4x4\n",
        "#2 @ 3,1: 4x4\n",
        "#3 @ 5,5: 2x2\n"
    ))
    .expect("valid test input");
    assert_eq!(count_overlap_squares(&claims), 4);
}

fn find_nonoverlapping_claim(claims: &[Claim]) -> Option<usize> {
    let mut taken_by: BTreeMap<Square, HashSet<usize>> = BTreeMap::new();
    for claim in claims.iter() {
        for square in claim.squares() {
            taken_by
                .entry(square)
                .and_modify(|v| {
                    v.insert(claim.id);
                })
                .or_insert_with(|| {
                    let mut v = HashSet::new();
                    v.insert(claim.id);
                    v
                });
        }
    }
    let mut non_overlapping: HashSet<usize> = claims.iter().map(|claim| claim.id).collect();
    for (_, takers) in taken_by.iter() {
        if takers.len() > 1 {
            for claim_id in takers.iter() {
                non_overlapping.remove(claim_id);
            }
        }
    }
    if non_overlapping.len() > 1 {
        None
    } else {
        non_overlapping.iter().copied().next()
    }
}

#[test]
fn test_find_nonoverlapping_claim() {
    let claims = get_claims(concat!(
        "#1 @ 1,3: 4x4\n",
        "#2 @ 3,1: 4x4\n",
        "#3 @ 5,5: 2x2\n"
    ))
    .expect("valid test input");
    match find_nonoverlapping_claim(&claims) {
        Some(3) => (),
        Some(n) => {
            panic!("wrong non-overlap id, expected 3 but got {n}");
        }
        None => {
            panic!("failed to find non-overlap");
        }
    }
}

fn main() {
    let claims = get_claims(str::from_utf8(include_bytes!("input.txt")).expect("valid input file"))
        .expect("valid claim lines");
    // 327761 is too high
    println!("Day 03 part 1: {}", count_overlap_squares(&claims));
    println!(
        "Day 03 part 2: {}",
        find_nonoverlapping_claim(&claims).expect("at least one overlap")
    );
}
