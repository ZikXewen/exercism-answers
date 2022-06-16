pub fn find<T: AsRef<[S]>, S: Ord>(array: T, key: S) -> Option<usize> {
    use std::cmp::Ordering::*;
    let array = array.as_ref();
    let m = array.len() / 2;
    match array.get(m)?.cmp(&key) {
        Equal => Some(m),
        Greater => find(&array[..m], key),
        Less => Some(find(&array[(m + 1)..], key)? + m + 1),
    }
}
