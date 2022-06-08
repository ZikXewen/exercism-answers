use std::ops::RangeInclusive;

fn get_range(num: usize, max: usize) -> RangeInclusive<usize> {
    num.saturating_sub(1)..=(num + 1).min(max - 1)
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, str)| {
            str.bytes()
                .enumerate()
                .map(|(j, x)| {
                    if '*' == x.into() {
                        '*'
                    } else {
                        let count = minefield
                            .get(get_range(i, minefield.len()))
                            .unwrap()
                            .iter()
                            .fold(0, |count, row| {
                                count
                                    + row
                                        .get(get_range(j, row.len()))
                                        .unwrap()
                                        .bytes()
                                        .filter(|&ch| '*' == ch.into())
                                        .count()
                            });
                        if count == 0 {
                            ' '
                        } else {
                            char::from_digit(count as u32, 10).unwrap()
                        }
                    }
                })
                .collect()
        })
        .collect()
}
