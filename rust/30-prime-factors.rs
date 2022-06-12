pub fn factors(mut n: u64) -> Vec<u64> {
    let mut v = vec![];
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            v.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    if n != 1 {
        v.push(n);
    }
    v
}
