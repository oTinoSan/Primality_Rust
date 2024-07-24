use std::{collections::{BTreeMap, HashMap}, hash::Hash, sync::mpsc::Sender, thread};

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

fn q_single_thread(mut min: u64, max: u64, global_max: u64, tx: Sender<(u64, (u64, u64))>){
    if min % 2 == 0 {
        min += 1;
    }
    for i in (min..=max).step_by(2) {
        for j in (i..=global_max/i).step_by(2) {
            let gcd = gcd(i, j);
            if gcd == 1 {
                tx.send((i * j, (j, i)));
            }
        }
    }
}

pub fn q_search_threaded(max: u64, num_threads: u64) -> BTreeMap<u64, (u64, u64)> {
    let (tx, rx) = std::sync::mpsc::channel();
    let combiner = thread::spawn(move || {
        let mut r = BTreeMap::new();
        for (k, v) in rx {
            r.insert(k, v);
        }
        r
    });
    let mut handles = vec![];
    let step = max.sqrt() / num_threads;
    for i in 0..num_threads {
        let thread_min = step * i + 3;
        let mut thread_max = thread_min + step;
        if i == num_threads - 1 {
            thread_max = max.sqrt() + 1;
        }
        let tx = tx.clone();
        handles.push(thread::spawn(move || q_single_thread(thread_min, thread_max, max, tx)));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    drop(tx);
    
    combiner.join().unwrap()
}