pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut ct = 10;
    let mut sm = 0;
    for c in isbn.chars() {
        match c {
            '0'..='9' => {
                sm += c.to_digit(10).unwrap() as i32 * ct;
                ct -= 1;
            }
            'X' if ct == 1 => {
                sm += 10 * ct;
                ct -= 1;
            }
            '-' => (),
            _ => return false,
        }
    }
    return ct == 0 && sm % 11 == 0;
}
