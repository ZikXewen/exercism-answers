pub fn is_armstrong_number(num: u32) -> bool {
    let dig = num.to_string().len() as u32;
    num == num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap().pow(dig))
        .sum()
}
