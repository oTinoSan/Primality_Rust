use primality_tests::*;
use rug::{integer::Order, Integer};
fn main() {
    let max = Integer::from(100000000);
    // let solovay_primes =
    //     primality_tests::bigint_algorithms::solovay_strassen_range(10, max.clone());
    // let miller_rabin_primes =
    //     primality_tests::bigint_algorithms::miller_rabin_range(10, max.clone());
    // println!(
    //     "Solovay-Strassen found {} primes under {}",
    //     solovay_primes.len(),
    //     max
    // );
    // println!(
    //     "Miller-Rabin found {} primes under {}",
    //     miller_rabin_primes.len(),
    //     max
    // );

    let threaded_primes = bigint_algorithms::solovay_strassen_threaded(10, max.clone(), 64);

    println!(
        "Found {} primes under {}",
        threaded_primes.len(),
        &max
    );
}
