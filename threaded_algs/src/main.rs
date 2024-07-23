use crate::lamellar_prime_tests::{bigint_miller_rabin, lamellar_baillie_psw};
use crate::AKS_prime::BigIntAKS;
use rug::{Complete, Integer}; //integer::MiniInteger,

pub mod AKS_prime;
pub mod baille_psw;
pub mod lamellar_prime_tests;
pub mod threaded_solovay;
pub mod wheel_algos;

fn main() {
    // threaded_miller_rabin(Integer::from(1000000000), 8);
    // let limit = "100000000";
    // let limit_int = limit.parse::<Integer>().unwrap();
    // lamellar_wheel_miller();
    // lamellar_wheel_solovay();
    // lamellar_baillie_psw();
    let limit = "13";
    let limit_int = limit.parse::<Integer>().unwrap();
    println!("Number is prime: {:?}", BigIntAKS(limit_int));
    // lamellar_wheel_miller_solovay();
}

pub fn threaded_miller_rabin(num_threads: u64, limit: Integer) -> Vec<Integer> {
    let block_size = (&limit / num_threads).complete();

    let mut thread_handles = Vec::new();

    for i in 0..num_threads {
        let mut thread_min: Integer = (i * &block_size).complete() + 1;
        let thread_max: Integer = ((i + 1) * &block_size).complete() + 1;

        if Integer::from(&thread_min % 2) == 0 {
            thread_min += 1;
        }
        thread_handles.push(std::thread::spawn(move || {
            let mut return_vector = Vec::new();
            while thread_min < thread_max {
                if bigint_miller_rabin(10, thread_min.clone()) {
                    return_vector.push(thread_min.clone());
                }
                thread_min += 2;
            }
            return_vector
        }));
    }
    let mut results = vec![];
    for handle in thread_handles {
        let mut thread_results = handle.join().unwrap();
        results.append(&mut thread_results);
    }

    results
}
