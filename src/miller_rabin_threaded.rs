use num_bigint::BigUint;
use num_traits::FromPrimitive;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::miller_rabin::miller_rabin;
use crate::miller_rabin_big::miller_rabin_bigint;

pub fn miller_rabin_array(limit: u32) -> Vec<bool> {
    let mut array = vec![false; limit as usize]; // Fixed syntax error
    for i in (5..limit).step_by(2) {
        // Assuming miller_rabin function exists and takes a u64 and returns a bool
        // Fixed the if condition and function call syntax
        if miller_rabin(i as u64, 10) {
            array[i as usize] = true; // Corrected logic to mark primes as true
        }
    }
    array
}

pub fn miller_rabin_threaded(candidate: BigUint, iterations: i32, num_threads: u64) -> Vec<BigUint> {
    let block_size = &candidate / BigUint::from_u64(num_threads).unwrap();
    let mut thread_handles = Vec::new();
    let return_vector = Arc::new(Mutex::new(Vec::new()));

    for i in 0..num_threads {
        let thread_min = BigUint::from_u64(i).unwrap() * &block_size;
        let thread_max = BigUint::from_u64(i + 1).unwrap() * &block_size;
        let return_vector_clone = Arc::clone(&return_vector);

        let thread = thread::spawn(move || {
            let mut local_vector = Vec::new();
            let mut current = if thread_min == BigUint::from_u64(0).unwrap() {
                BigUint::from_u64(2).unwrap()
            } else {
                thread_min
            };
            while current < thread_max {
                if miller_rabin_bigint(current.clone(), iterations.try_into().unwrap()) { 
                    local_vector.push(current.clone());
                }
                current += BigUint::from_u64(1).unwrap();
            }
            let mut return_vector = return_vector_clone.lock().unwrap();
            return_vector.extend(local_vector);
        });
        thread_handles.push(thread);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    let locked_vector = return_vector.lock().unwrap();
    locked_vector.clone()
}