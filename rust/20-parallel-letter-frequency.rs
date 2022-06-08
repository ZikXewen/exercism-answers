use std::{collections::HashMap, thread};

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let mut threads = Vec::with_capacity(worker_count);
    for i in input.chunks(input.len() / worker_count + 1) {
        let chunk = Vec::from(i);
        let thread = thread::spawn(move || {
            let mut res = HashMap::new();
            for str in chunk {
                for c in str.to_lowercase().chars() {
                    if c.is_alphabetic() {
                        res.entry(c).and_modify(|x| *x += 1).or_insert(1);
                    }
                }
            }
            res
        });
        threads.push(thread);
    }

    let mut map = HashMap::new();
    for thread in threads {
        for (&c, &n) in thread.join().unwrap().iter() {
            map.entry(c).and_modify(|x| *x += n).or_insert(n);
        }
    }
    map
}
