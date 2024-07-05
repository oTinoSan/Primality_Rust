/// Executes Miller-Rabin primality test in parallel using multiple threads.
///
/// Function divides range from 1 to `max_num` into `num_threads` parts,
/// and spawns separate thread for each part to perform primality test on
/// each number within its assigned range.
///
/// # Arguments
///
/// `max_num` - upper limit of range (inclusive) to test for primality.
/// `num_threads` - number of threads to use for parallel computation.
/// ** step is range between each each new thread
/// # Example
/// 
/// Test numbers from 1 to 100 for primality using 4 threads.
/// threaded_bignum(100, 4);

use num::BigInt;
use std::thread;
use super::miller_rabin_big::miller_rabin_bignum;

pub fn threaded_bignum(max_num: i64, num_threads: i64) {
    // divides max_number by num_threads to find the step
    let num_per_thread = max_num / num_threads;
    // creates empty vector for threads
    let mut threads = Vec::new();

    // for loop through the number of threads
    for t in 0..num_threads {
        // initiates start of range to index of t multiplied by the current step plus one
        let start = t * num_per_thread + 1;
        // calculates end of range; if last thread, go up to max_num + 1, if not, calculate else based on num_per_thread
        let end = if t == num_threads - 1 {
            max_num + 1
        } else {
            start + num_per_thread
        };

        // spawn thread for range of numbers
        let handle = thread::spawn(move || {
            // for loop over range from start to end
            for number in start..end {
                //// println!("hi number {number} from the spawned thread!");
                let candidate_num = BigInt::from(number);
                miller_rabin_bignum(candidate_num, 5);
                //// println!("Is {} prime? {}", number, result);
            }
        });

        // add thread's join handle to thread's vector
        threads.push(handle);
    }

    // wait for all spawned threads to complete
    for handle in threads {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod threaded_tests {
    use super::*;

    #[test]
    fn test_threaded_bignum_basic() {
        // test with basic values
        threaded_bignum(10, 2);
    }

    // #[test]
    // #[should_panic(expected = "attempt to divide by zero")]
    // fn test_threaded_bigrug_divide_by_zero() {
    //     // test is expected to panic due to division by zero
    //     threaded_bigrug(10, 0);
    // }

    #[test]
    fn test_threaded_bignum_large_numbers() {
        // test with larger numbers
        threaded_bignum(1000, 10);
    }

    #[test]
    fn test_threaded_bignum_single_thread() {
        // test with single thread
        threaded_bignum(10, 1);
    }

    // #[test]
    // fn test_threaded_bigrug_negative_numbers() {
    //     // test with negative numbers, expecting it to handle gracefully or panic
    //     threaded_bigrug(-10, -2);
}
