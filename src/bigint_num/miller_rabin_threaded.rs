use super::miller_rabin_big::miller_rabin_bignum;
use crate::miller_rabin::miller_rabin;
use num_bigint::BigUint;
use num_traits::FromPrimitive; // Removed ToPrimitive
use std::sync::{Arc, Mutex};
use std::thread;

pub fn miller_rabin_array(limit: u32) -> Vec<bool> {
    let mut array = vec![false; limit as usize];
    for i in (5..limit).step_by(2) {
        if miller_rabin(i as u64, 10) {
            array[i as usize] = true;
        }
    }
    array
}

pub fn miller_rabin_threaded(
    candidate: BigUint,
    iterations: usize,
    num_threads: u64,
) -> Result<Vec<BigUint>, &'static str> {
    if num_threads == 0 {
        return Err("num_threads must be greater than 0");
    }

    let block_size = &candidate / BigUint::from_u64(num_threads).ok_or("Conversion error")?;
    let return_vector = Arc::new(Mutex::new(Vec::new()));

    let mut thread_handles = Vec::with_capacity(num_threads as usize);

    for i in 0..num_threads {
        let thread_min = BigUint::from_u64(i).ok_or("Conversion error")? * &block_size;
        let thread_max = BigUint::from_u64(i + 1).ok_or("Conversion error")? * &block_size;
        let return_vector_clone = Arc::clone(&return_vector);

        let thread = thread::spawn(move || {
            let mut local_vector = Vec::new();
            let start = if i == 0 {
                BigUint::from_u32(2).unwrap()
            } else {
                thread_min
            };
            let mut current = start.clone();
            while current < thread_max {
                if miller_rabin_bignum(current.clone(), iterations) {
                    local_vector.push(current.clone());
                }
                current += BigUint::from_u32(1).unwrap();
            }
            let mut return_vector = return_vector_clone.lock().expect("Lock poisoned");
            return_vector.extend(local_vector);
        });
        thread_handles.push(thread);
    }

    for handle in thread_handles {
        handle.join().expect("Thread panicked");
    }

    // Clone the data inside the mutex into a local variable before the Arc<Mutex<...>> is dropped
    let cloned_data = {
        let locked_data = return_vector.lock().expect("Lock poisoned");
        locked_data.clone()
    };

    Ok(cloned_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use std::str::FromStr;

    #[test]
    fn test_miller_rabin_threaded_with_prime() {
        let prime_candidate = BigUint::from_str("5").unwrap();
        let iterations = 5;
        let num_threads = 4;
        let result = miller_rabin_threaded(prime_candidate.clone(), iterations, num_threads);

        // Define expected_primes with the correct type and value
        // Assuming the function returns a vector of BigUint indicating prime candidates
        let expected_primes = vec![BigUint::from_str("5").unwrap()]; // Adjust according to the expected output

        match result {
            Ok(primes) => assert_eq!(
                primes, expected_primes,
                "The prime candidate was not correctly identified."
            ),
            Err(_) => panic!("Function returned an error."),
        }
    }
}
