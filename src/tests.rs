use super::*;

const PRIMES_31: [u64; 11] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];

#[test]
fn division_31() {
    let result = trial_divisions(31);
    assert_eq!(result, PRIMES_31.to_vec());
}

#[test]
fn sieve_31() {
    let result = sieve_primes(31);
    assert_eq!(result, PRIMES_31.to_vec());
}

#[test]
fn wheel_31() {
    let result = wheel_primes(vec![2, 3], 31);
    assert_eq!(result, PRIMES_31.to_vec());
}