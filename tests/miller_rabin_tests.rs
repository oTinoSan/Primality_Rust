//! This is a testing module for the three Miller Rabin tests

#[cfg(test)]
mod miller_rabin_big_test {
    use num::BigUint;
    use primality::bigint_num::miller_rabin_big::miller_rabin_bignum;

    #[test]
    fn t_mr_32761_big() {
        let num = BigUint::from(32_761u32);
        let ans = miller_rabin_bignum(num, 2);
        println!("is prime? {}", ans);
    }
}

#[cfg(test)]
mod miller_rabin_jacob {
    use num::BigUint;
    use primality::bigint_num::big_int_jacob::b_miller_rabin;

    #[test]
    fn t_mr_32761_big() {
        let num = BigUint::from(32_761u32);
        let ans = b_miller_rabin(num);
        println!("is prime? {}", ans);
    }
}

#[cfg(test)]
mod miller_rabin_test {

    use primality::miller_rabin::miller_rabin;

    #[test]
    fn test_known_primes() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for &prime in primes.iter() {
            assert!(miller_rabin(prime, 5), "Failed on prime {}", prime);
        }
    }

    #[test]
    fn test_known_composites() {
        let composites = vec![4, 6, 8, 9, 10, 12, 14, 15, 16, 18];
        for &composite in composites.iter() {
            assert!(
                !miller_rabin(composite, 5),
                "Failed on composite {}",
                composite
            );
        }
    }

    #[test]
    fn test_edge_cases() {
        assert!(!miller_rabin(0, 5), "Failed on 0");
        assert!(!miller_rabin(1, 5), "Failed on 1");
    }
}

// #[cfg(test)]
// mod miller_rabin_threaded_test {

//     use num::BigUint;
//     use num_traits::FromPrimitive;
//     use primality::bigint_num::miller_rabin_threaded::miller_rabin_threaded;

//     #[test]
//     fn test_miller_rabin_under_100() {
//         // Example test case: Check if the function correctly identifies prime numbers
//         // in a given range. Adjust the range, iterations, and threads as needed.
//         let candidate: u32 = 100; // Upper limit of the range to test
//         let iterations = 5; // Number of iterations for the Miller-Rabin test
//         let num_threads = 4u64; // Number of threads to use

//         // Call the function
//         let result = miller_rabin_threaded(BigUint::from(candidate), iterations, num_threads);

//         // Expected output: List of BigInt prime numbers within the range.
//         // This is a simplified example; you should adjust it based on the expected primes.
//         let expected_primes = vec![
//             2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
//             89, 97,
//         ]
//         .into_iter()
//         .map(BigUint::from_u64)
//         .collect::<Option<Vec<BigUint>>>()
//         .unwrap();

//         // Assert that the result contains all expected primes (and only them).
//         // Note: This assumes the function returns a sorted list of primes.
//         assert_eq!(result, expected_primes);
//     }
// }

#[cfg(test)]
mod tests {
    use num::BigUint;
    use primality::bigint_num::miller_rabin_big::miller_rabin_bignum;

    #[test]
    fn test_miller_rabin_bigint_for_bigints() {
        // Known primes and non-primes for testing, represented as BigUint
        let primes = vec![
            BigUint::from(43019u64),
            BigUint::from(108961u64),
            BigUint::from(5u64),
            BigUint::from(7u64),
            BigUint::from(11u64),
            BigUint::from(13u64),
            BigUint::from(17u64),
            BigUint::from(19u64),
            BigUint::from(23u64),
            BigUint::from(29u64),
        ];
        let non_primes = vec![
            BigUint::from(1u64),
            BigUint::from(4u64),
            BigUint::from(6u64),
            BigUint::from(8u64),
            BigUint::from(9u64),
            BigUint::from(10u64),
            BigUint::from(12u64),
            BigUint::from(14u64),
            BigUint::from(15u64),
            BigUint::from(16u64),
        ];

        // Test primes
        for prime in primes {
            assert!(miller_rabin_bignum(prime, 5), "Failed on prime");
        }

        // Test non-primes
        for non_prime in non_primes {
            assert!(!miller_rabin_bignum(non_prime, 5), "Failed on non-prime");
        }
    }
}
