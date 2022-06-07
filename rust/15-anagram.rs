use std::collections::HashSet;

fn sort_string(word: &str) -> String {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort_unstable();
    sorted.iter().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let sorted = sort_string(&word);
    possible_anagrams
        .iter()
        .filter(|&&x| {
            let x = x.to_lowercase();
            x != word && sort_string(&x) == sorted
        })
        .copied()
        .collect()
}
