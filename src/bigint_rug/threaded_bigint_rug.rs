use rug::Integer;
use std::thread;
use std::time::Duration;

use super::super::bigint_rug::miller_rabin_rug;

pub fn threaded_bigrug(max_num: i64, num_threads: i64) {
    // divides max_numb by num_threads to decide the distribution step
    let num_per_thread = max_num / num_threads;
    // sets new_thread to 0
    let mut new_thread = 0;
    // updates end_thread to distribution step
    let mut end_thread: i64 = num_per_thread;
    // begins for loop over each step
    for number in new_thread..end_thread {
        thread::spawn(move || {
            let candidate_num = Integer::from(number);
            println!("hi number {number} from the spawned thread!");
            let result = miller_rabin_rug::miller_rabin_bigrug(candidate_num, 5);
            println!("Is {} prime? {}", number, result);
            thread::sleep(Duration::from_millis(1));
        });

        for i in new_thread..end_thread {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
        if end_thread >= max_num {
            break;
        } else {
            new_thread += &num_per_thread;
            end_thread += &num_per_thread;
        }
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
