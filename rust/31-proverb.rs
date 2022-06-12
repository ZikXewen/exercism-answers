pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        "".to_owned()
    } else {
        format!(
            "{}\nAnd all for the want of a {}.",
            list.windows(2).fold("".to_string(), |cr, x| {
                format!("{}\nFor want of a {} the {} was lost.", cr, x[0], x[1])
            }),
            list[0]
        )
        .trim()
        .to_owned()
    }
}
