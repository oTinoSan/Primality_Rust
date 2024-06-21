use primality_tests::*;
use rug::Integer;
fn main() {
    let mut solovay_primes = vec![];
    for i in (5..100).step_by(2) {
        if bigint_algorithms::solovay_strassen(10, Integer::from(i)) {
            solovay_primes.push(i);
        }
    }
    println!("{:?}", solovay_primes);
}
