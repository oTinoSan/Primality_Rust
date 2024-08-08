// use primality_testing::bigint::b_miller_rabin;
use primality_testing::{
    miller_rabin_rug::miller_rabin_array, solovay_strassen::{solovay_strassen}
};
use rug::Integer;

fn main() {
    divan::main();
}

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000])]
// fn advanced_trial_division (arg: u64) {
//     trial_division_odds(arg);
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000])]
// fn sieve_of_eratosthenes_test (arg: u64) {
//     sieve_of_eratosthenes(arg);
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000])]
// fn wheel_factoring_test (arg: u64) {
//     wheel_factoring(arg);
// }

// #[divan::bench(args = [1193, 3779, 11939, 19937, 193939, 199933, 479001599, 2147483647, 2147483649])]
// fn wheel_factoring_single_test (arg: u64) {
//     wheel_factoring_single(arg);
// }



// #[divan::bench(args = [1193, 3779, 11939, 19937, 193939, 199933, 479001599, 2147483647, 2147483649])]
// fn miller_single_test (arg: u64) {
//     miller_rabin_probabilistic(arg, 10);
// }

// #[divan::bench(args = [BigInt::from(170141183460469231731687303715884105727i128),
//         BigInt::from(2).pow(1279)-1,
//         BigInt::from(2).pow(2203)-1,
//         BigInt::from(2).pow(3217)-1])]
// fn my_bigint_miller_rabin_single_test(arg: &BigInt) {
//     bigint_miller_rabin(arg.clone(), 10);
// }

#[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000, 64000])]
fn miller_rabin_array_test (arg: u32) {
    miller_rabin_array(Integer::from(arg));
}

#[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000, 64000])]
fn solovay_strassen_array_test (arg: u64) {
    solovay_strassen(arg);
}

// #[divan::bench(args = [BigInt::from(170141183460469231731687303715884105727i128),
//         BigInt::from(2).pow(1279)-1,
//         BigInt::from(2).pow(2203)-1,
//         BigInt::from(2).pow(3217)-1])]
// fn solovay_strassen_single_test(arg: &BigInt) {
//     bigint_solovay_strassen_test(arg.clone());
// }

// #[divan::bench(args = [
//     &BigInt::from(1193),
//     &BigInt::from(3779),
//     &BigInt::from(11939),
//     &BigInt::from(19937),
//     &BigInt::from(193939),
//     &BigInt::from(199933),
//     &BigInt::from(479001599),
//     &BigInt::from(2147483647), ])]
// fn solovay_strassen_single_test (arg: BigInt) {
//     solovay_strassen_test(&arg);
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000, 64000])]
// fn built_in_miller_rabin_array_test (arg: u32) {
//     built_in_miller_rabin_array(arg);
// }
