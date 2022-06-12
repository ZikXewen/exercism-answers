pub fn raindrops(n: u32) -> String {
    let ret = format!(
        "{}{}{}",
        if n % 3 == 0 { "Pling" } else { "" },
        if n % 5 == 0 { "Plang" } else { "" },
        if n % 7 == 0 { "Plong" } else { "" }
    );
    if ret.is_empty() {
        n.to_string()
    } else {
        ret
    }
}
