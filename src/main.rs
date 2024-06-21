use primality_tests::*;
use rug::Integer;
fn main() {
    let mut miller_primes = vec![];
    for i in (5..100).step_by(2) {
        if bigint_algorithms::miller_rabin(10, Integer::from(i)) {
            miller_primes.push(i);
        }
    }
    println!("{:?}", miller_primes);
}
