use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.into_iter()
        .flat_map(|(&i, v)| v.into_iter().map(move |c| (c.to_ascii_lowercase(), i)))
        .collect()
}
