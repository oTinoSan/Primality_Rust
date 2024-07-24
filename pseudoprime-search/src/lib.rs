use std::{collections::{BTreeMap, HashMap}, hash::Hash, thread};

use num::integer::{gcd, Roots};

pub fn q_search(max: u64) -> BTreeMap<u64, (u64, u64)> {
    let mut result = BTreeMap::new();
    for i in (3..=max/3).step_by(2) {
        for j in (i..=max/i).step_by(2) {
            if gcd(i, j) == 1 || gcd(i, j) == i {
                result.insert(i * j, (j, i));
            }
        }
    }
    result
}

fn q_single_thread(mut min: u64, max: u64, global_max: u64) -> Vec<(u64, (u64, u64))> {
    let mut result = vec![];
    if min % 2 == 0 {
        min += 1;
    }
    for i in (min..=max).step_by(2) {
        for j in (i..=global_max/i).step_by(2) {
            if gcd(i, j) == 1 || gcd(i, j) == i {
                result.push((i * j, (j, i)));
            }
        }
    }
    result
}

pub fn q_search_threaded(max: u64, num_threads: u64) -> BTreeMap<u64, (u64, u64)> {
    let mut handles = vec![];
    let step = max.sqrt() / num_threads;
    for i in 0..num_threads {
        let thread_min = step * i + 3;
        let mut thread_max = thread_min + step;
        if i == num_threads - 1 {
            thread_max = max.sqrt() + 1;
        }
        handles.push(thread::spawn(move || q_single_thread(thread_min, thread_max, max)));
    }

    let mut res = BTreeMap::new();

    for handle in handles.into_iter() {
        let r = handle.join().unwrap();
        for (k, v) in r.into_iter() {
            res.insert(k, v);
        }
    }
    res
}