use rug::Integer;
use std::thread;
// use std::time::Duration;

use super::super::bigint_rug::miller_rabin_rug;

// Assume miller_rabin_rug is a module you've defined or imported
// use miller_rabin_rug;

pub fn threaded_bigrug(max_num: i64, num_threads: i64) {
    let num_per_thread = max_num / num_threads;
    let mut threads = Vec::new();

    for t in 0..num_threads {
        let start = t * num_per_thread + 1;
        let end = if t == num_threads - 1 {
            max_num + 1
        } else {
            start + num_per_thread
        };

        let handle = thread::spawn(move || {
            for number in start..end {
                // println!("hi number {number} from the spawned thread!");
                let candidate_num = Integer::from(number);
                // Assuming miller_rabin_bigrug is a function that returns a boolean
                miller_rabin_rug::miller_rabin_bigrug(candidate_num, 5);
                // println!("Is {} prime? {}", number, result);
            }
        });

        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod threaded_tests {
    use super::*;

    #[test]
    fn test_threaded_bigrug_basic() {
        // Test with basic values
        threaded_bigrug(10, 2);
    }

    // #[test]
    // #[should_panic(expected = "attempt to divide by zero")]
    // fn test_threaded_bigrug_divide_by_zero() {
    //     // This test is expected to panic due to division by zero
    //     threaded_bigrug(10, 0);
    // }

    #[test]
    fn test_threaded_bigrug_large_numbers() {
        // Test with larger numbers
        threaded_bigrug(1000, 10);
    }

    #[test]
    fn test_threaded_bigrug_single_thread() {
        // Test with a single thread
        threaded_bigrug(10, 1);
    }

    // #[test]
    // fn test_threaded_bigrug_negative_numbers() {
    //     // Test with negative numbers, expecting it to handle gracefully or panic
    //     threaded_bigrug(-10, -2);
}
