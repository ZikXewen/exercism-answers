pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .try_fold(vec![], |mut vec, x| {
            match x {
                '[' => vec.push(']'),
                '{' => vec.push('}'),
                '(' => vec.push(')'),
                ']' | '}' | ')' => {
                    if vec.pop()? != x {
                        return None;
                    }
                }
                _ => (),
            }
            Some(vec)
        })
        .map_or(false, |x| x.is_empty())
}
