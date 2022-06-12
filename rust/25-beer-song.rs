fn get_bottles(n: u32, cap: bool) -> String {
    if n == 0 && cap {
        "No more bottles of beer".to_owned()
    } else {
        match n {
            0 => "no more bottles of beer".to_owned(),
            1 => "1 bottle of beer".to_owned(),
            _ => format!("{} bottles of beer", n),
        }
    }
}

pub fn verse(n: u32) -> String {
    let v1 = get_bottles(n, true);
    let v2 = get_bottles(n, false);
    let v3 = match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    };
    let v4 = get_bottles((n + 99) % 100, false);
    format!("{v1} on the wall, {v2}.\n{v3}, {v4} on the wall.\n")
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .fold("".to_string(), |cr, x| format!("{}\n{}", cr, verse(x)))
        .trim_start()
        .to_owned()
}
