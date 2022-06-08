pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|&x| !x.is_whitespace())
        .try_fold((0, 0), |(id, sum), cr| {
            let num = cr.to_digit(10)? << (id & 1);
            Some((id + 1, sum + num - if num >= 10 { 9 } else { 0 }))
        })
        .map_or(false, |(id, sum)| id > 1 && sum % 10 == 0)
}
