pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|&x| !(2..=((x as f32).sqrt() as u32)).any(|y| x % y == 0))
        .nth(n as usize)
        .unwrap()
}
