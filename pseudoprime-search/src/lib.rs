use std::{collections::{BTreeMap, HashMap}, hash::Hash, thread};

use num::integer::gcd;

pub fn q_search(mut min: u64, max: u64) -> Vec<(u64, (u64, u64))> {
    let mut result = Vec::new();
    if min % 2 == 0 {
        min += 1;
    }
    for i in (min..=max/3).step_by(2) {
        for j in (i..=max/i).step_by(2) {
            if gcd(i, j) == 1 || gcd(i, j) == i {
                result.push((i * j, (j, i)));
            }
        }
    }
    result
}

pub fn q_search_threaded(max: u64, num_threads: u64) -> HashMap<u64, (u64, u64)> {
    let mut handles = vec![];
    let step = max / num_threads;
    for i in 0..num_threads {
        let thread_min = step * i + 3;
        handles.push(thread::spawn(move || q_search(thread_min, max)));
    }

    let mut res = HashMap::new();

    for handle in handles.into_iter().rev() {
        let r = handle.join().unwrap();
        for (k, v) in r.into_iter() {
            res.insert(k, v);
        }
    }
    res
}