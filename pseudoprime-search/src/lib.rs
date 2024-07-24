use std::{collections::{BTreeMap, HashMap}, sync::mpsc::Sender, thread};

use rayon::prelude::*;

use num::integer::{gcd, Roots};

pub fn q_search(max: u64) -> BTreeMap<u64, (u64, u64)> {
    let mut result = BTreeMap::new();
    for i in (3..=max.sqrt() + 1).step_by(2) {
        for j in (i..=max/i).step_by(2) {
            let gcd = gcd(i, j);
            if gcd == 1 {
                result.insert(i * j, (j, i));
            }
        }
    }
    result
}

pub fn q_search_rayon(max: u64) -> BTreeMap<u64, (u64, u64)> {
    let is: Vec<_> = (3..=max.sqrt() + 1).step_by(2).collect();
    is.par_iter().map(|i| {
        let mut r = BTreeMap::new();
        for j in *i..=max / *i {
            let gcd = gcd(*i, j);
            if gcd == 1 {
                r.insert(*i * j, (j, *i));
            }
        }
        r
    }).reduce(|| BTreeMap::new(), |mut acc, x| {acc.extend(x); acc})
}

fn q_single_thread(mut min: u64, max: u64) -> BTreeMap<u64, (u64, u64)>{
    let mut r = BTreeMap::new();
    if min % 2 == 0 {
        min += 1;
    }
    for i in (3..=max.sqrt()).step_by(2) {
        let mut min = min / i;
        if min % 2 == 0 {
            min += 1;
        }
        for j in (i.max(min)..=max/i).step_by(2) {
            let gcd = gcd(i, j);
            if gcd == 1 {
                r.insert(i*j, (i, j));
            }
        }
    }
    r
}

pub fn q_search_threaded(min: u64, max: u64, num_threads: u64) -> BTreeMap<u64, (u64, u64)> {
    let mut handles = vec![];
    let step = (max - min) / num_threads;
    for i in 0..num_threads {
        let thread_min = min + step * i;
        let mut thread_max = thread_min + step;
        if i == num_threads - 1 {
            thread_max = max;
        }
        handles.push(thread::spawn(move || q_single_thread(thread_min, thread_max)));
    }

    let mut res = BTreeMap::new();

    for handle in handles {
        res.extend(handle.join().unwrap());
    }

    res
}