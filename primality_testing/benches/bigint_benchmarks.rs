use primality_testing::baillie_psw::{baillie_psw_test, baillie_wagstaff_lucas_test, crandall_pomerance_lucas_test};
use primality_testing::miller_rabin_rug::bigint_miller_rabin;
use rug::Integer;

fn main() {
    divan::main();
}

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000])]
// fn num_miller_rabin_array_test(arg: u32) {
//     miller_rabin_array(BigInt::from(arg));
// }

// #[divan::bench(args = [100000, 200000, 400000], sample_count=10, sample_size=10)]
// fn threaded_miller_rabin_array_test_8(arg: u32) {
//     threaded_miller_rabin(Integer::from(arg), 8);
// }

// #[divan::bench(args = [100000, 200000, 400000], sample_count=10, sample_size=10)]
// fn threaded_miller_rabin_array_test_16(arg: u32) {
//     threaded_miller_rabin(Integer::from(arg), 16);
// }

// #[divan::bench(args = [100000, 200000, 400000], sample_count=10, sample_size=10)]
// fn threaded_miller_rabin_array_test_64(arg: u32) {
//     threaded_miller_rabin(Integer::from(arg), 64);
// }

// #[divan::bench(args = [100000, 200000, 400000], sample_count=10, sample_size=10)]
// fn threaded_baillie_psw_test_8(arg: u32) {
//     threaded_baillie_psw(Integer::from(0), Integer::from(arg), 8);
// }

// #[divan::bench(args = [100000, 200000, 400000], sample_count=10, sample_size=10)]
// fn threaded_baillie_psw_test_16(arg: u32) {
//     threaded_baillie_psw(Integer::from(0), Integer::from(arg), 16);
// }

// #[divan::bench(args = [100000, 200000, 400000], sample_count=10, sample_size=10)]
// fn threaded_baillie_psw_test_64(arg: u32) {
//     threaded_baillie_psw(Integer::from(0), Integer::from(arg), 64);
// }

// #[divan::bench(args = [Integer::from(170141183460469231731687303715884105727i128),
//         Integer::from(2).pow(1279)-1,
//         Integer::from(2).pow(2203)-1,
//         Integer::from(2).pow(3217)-1])]

// #[divan::bench(args = [Integer::from(87178291199i128),
//         Integer::from(3318308475676071413i128),
//         Integer::from(10888869450418352160768000001i128),
//         Integer::from(170141183460469231731687303715884105727i128)])]

// #[divan::bench(args = [100_000, 1_000_000, 10_000_000], sample_count=1, sample_size=1)]
// fn threaded_1 (arg: u32) {
//     threaded_prime_search(Integer::from(arg.clone() / 10), Integer::from(arg), 64);
// }

// #[divan::bench(args = [100_000, 1_000_000, 10_000_000], sample_count=1, sample_size=1)]
// fn threaded_4 (arg: u32) {
//     threaded_prime_search(Integer::from(arg.clone() / 10), Integer::from(arg), 4);
// }

// #[divan::bench(args = [100_000, 1_000_000, 10_000_000], sample_count=1, sample_size=1)]
// fn threaded_16 (arg: u32) {
//     threaded_prime_search(Integer::from(arg.clone() / 10), Integer::from(arg), 16);
// }

// #[divan::bench(args = [100_000, 1_000_000, 10_000_000], sample_count=1, sample_size=1)]
// fn threaded_64 (arg: u32) {
//     threaded_prime_search(Integer::from(arg.clone() / 10), Integer::from(arg), 64);
// }

#[divan::bench(args = [Integer::from(87178291199i128),
        Integer::from(3318308475676071413i128),
        Integer::from(10888869450418352160768000001i128),
        Integer::from(170141183460469231731687303715884105727i128)])]
fn bpsw_test(arg: &Integer) {
    baillie_psw_test(&arg);
}

#[divan::bench(args = [Integer::from(87178291199i128),
        Integer::from(3318308475676071413i128),
        Integer::from(10888869450418352160768000001i128),
        Integer::from(170141183460469231731687303715884105727i128)])]
fn miller_rabin_test(arg: &Integer) {
    bigint_miller_rabin(&arg, 10);
}

#[divan::bench(args = [Integer::from(87178291199i128),
        Integer::from(3318308475676071413i128),
        Integer::from(10888869450418352160768000001i128),
        Integer::from(170141183460469231731687303715884105727i128)])]
fn lucas_test(arg: &Integer) {
    crandall_pomerance_lucas_test(&arg);
}

#[divan::bench(args = [Integer::from(87178291199i128),
        Integer::from(3318308475676071413i128),
        Integer::from(10888869450418352160768000001i128),
        Integer::from(170141183460469231731687303715884105727i128)])]
fn strong_lucas_test(arg: &Integer) {
    baillie_wagstaff_lucas_test(&arg);
}