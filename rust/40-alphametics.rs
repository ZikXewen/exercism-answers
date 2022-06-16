use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const TEN_POWS: [i64; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = input.split_once(" == ")?;
    let mut counts: HashMap<char, i64> = HashMap::new();
    let mut first_chars: HashSet<char> = HashSet::new();
    for x in left.split(" + ") {
        first_chars.insert(x.chars().next()?);
        for (i, x) in x.chars().rev().enumerate() {
            *counts.entry(x).or_default() += TEN_POWS[i];
        }
    }
    for x in right.split(" + ") {
        first_chars.insert(x.chars().next()?);
        for (i, x) in x.chars().rev().enumerate() {
            *counts.entry(x).or_default() -= TEN_POWS[i];
        }
    }
    let mut vec = vec![];
    for c in &first_chars {
        vec.push((*c, counts[c]));
    }
    for (c, x) in counts {
        if !first_chars.contains(&c) {
            vec.push((c, x));
        }
    }
    (0..10u8)
        .permutations(vec.len())
        .find(|perm| {
            perm.iter().take(first_chars.len()).all(|&x| x != 0)
                && vec
                    .iter()
                    .zip(perm.iter())
                    .map(|(&(_, a), &b)| a * b as i64)
                    .sum::<i64>()
                    == 0
        })
        .map(|perm| {
            perm.into_iter()
                .zip(vec.into_iter())
                .map(|(x, (c, _))| (c, x))
                .collect()
        })
}
