use primality_tests::*;
fn main() {
    let max_val = 1000;
    let solovay_primes = solovay_strassen::solovay_strassen_list(10, max_val);
    let sieve_primes = sieve_primes(max_val)[2..].to_vec();
    assert_eq!(solovay_primes, sieve_primes);
    println!("{:?}", sieve_primes);
}
