pub fn abbreviate(phrase: &str) -> String {
    let ch: Vec<char> = phrase.chars().collect();
    ch.iter()
        .enumerate()
        .fold("".to_string(), |mut cr, (i, &x)| {
            if (ch[i].is_alphabetic()
                && (i == 0 || (!ch[i - 1].is_alphabetic() && ch[i - 1] != '\'')))
                || (ch[i].is_ascii_uppercase() && !ch[i - 1].is_ascii_uppercase())
            {
                cr.push(x);
            }
            cr
        })
        .to_uppercase()
}
