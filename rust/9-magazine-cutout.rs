use std::collections::{hash_map::Entry, HashMap};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut dict: HashMap<&str, i32> = HashMap::new();
    for word in magazine {
        dict.entry(word).and_modify(|x| *x += 1).or_insert(1);
    }
    for word in note {
        dict.entry(word).and_modify(|x| *x -= 1).or_insert(-1);
    }
    dict.iter().all(|(_, x)| x >= &0)
}
