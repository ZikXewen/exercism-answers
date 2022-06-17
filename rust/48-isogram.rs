use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    for c in candidate.to_ascii_lowercase().chars() {
        if c.is_alphabetic() && set.contains(&c) {
            return false;
        }
        set.insert(c);
    }
    true
}
