use primality_tests::*;
fn main() {
    let val = 5000;
    let trial_primes = trial_divisions(val);
    let basis = vec![2, 3];
    let wheel_primes = wheel_primes(basis, val);
    let sieve_primes = sieve_primes(val);
    assert_eq!(wheel_primes, sieve_primes);
    assert_eq!(wheel_primes, trial_primes);
}