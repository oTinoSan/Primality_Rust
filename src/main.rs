use primality_tests::*;
use rug::{integer::Order, Integer};
fn main() {
    let max = Integer::from(1000);
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

    // let threaded_primes = bigint_algorithms::miller_rabin_wheel(10, max.clone(), 4);
    let threaded_primes =
        primality_tests::bigint_algorithms::wheel_threaded::general_wheel_threaded(
            10,
            Integer::ZERO,
            max.clone(),
            4,
            primality_tests::bigint_algorithms::miller_rabin,
            vec![2, 3, 5],
            vec![1, 7, 11, 13, 17, 19, 23, 29],
        );

    // println!("{:?}", threaded_primes);

    println!("Found {} primes under {}", threaded_primes.len(), &max);
    println!("{:?}", threaded_primes);
}
