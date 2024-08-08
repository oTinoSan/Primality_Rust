use rug::Integer;

use crate::{
    baillie_psw::{base_2_strong_probable_prime_test, baillie_wagstaff_lucas_test},
    miller_rabin_rug::bigint_miller_rabin,
};

pub fn threaded_prime_search(
    lower_limit: Integer,
    upper_limit: Integer,
    num_threads: u64,
) -> (Vec<Integer>, Vec<Integer>, Vec<Integer>, Vec<Integer>, Vec<Integer>) {
    let block_size = Integer::from(Integer::from(&upper_limit - &lower_limit) / num_threads);

    let mut thread_handles = Vec::new();

    for i in 0..num_threads {
        let mut thread_min: Integer = i * Integer::from(&block_size) + &lower_limit + 5;
        let thread_max: Integer = (i + 1) * Integer::from(&block_size) + &lower_limit + 5;
        if Integer::from(&thread_min) % 2 == Integer::ZERO {
            thread_min += 1;
        }
        let thread = std::thread::spawn(move || {
            let mut primes_vector = Vec::new();
            let mut strong_base_2_pseudoprimes_vector = Vec::new();
            let mut lucas_pseudoprimes_vector = Vec::new();
            let mut strong_lucas_pseudoprimes_vector = Vec::new();
            let mut baillie_psw_pseudoprimes_vector = Vec::new();
            
            while thread_min < thread_max {
                // step 1: run miller rabin first for performance. if prime, continue.
                let miller_rabin_primality = bigint_miller_rabin(&thread_min, 10);
                if miller_rabin_primality {
                    primes_vector.push(thread_min.clone()); //optionally add to prime vector to make a list of primes
                    thread_min += 2;
                    continue;
                }
                // step 2: check if it is a base 2 pseudoprime
                let base_2_strong_probable: bool = base_2_strong_probable_prime_test(&thread_min);
                // step 3: check if it is a lucas probable prime
                let lucas_probable = baillie_wagstaff_lucas_test(&thread_min);

                if base_2_strong_probable {
                    strong_base_2_pseudoprimes_vector.push(thread_min.clone());
                } 
                if lucas_probable.0 {
                    lucas_pseudoprimes_vector.push(thread_min.clone());
                }
                if lucas_probable.1 {
                    strong_lucas_pseudoprimes_vector.push(thread_min.clone());
                }
                if lucas_probable.0 && base_2_strong_probable {
                    baillie_psw_pseudoprimes_vector.push(thread_min.clone());
                }
                thread_min += 2;
            }
            (
                primes_vector,
                strong_base_2_pseudoprimes_vector,
                lucas_pseudoprimes_vector,
                strong_lucas_pseudoprimes_vector,
                baillie_psw_pseudoprimes_vector,
            )
        });
        thread_handles.push(thread);

    }
    let mut vector_tuple: (Vec<Integer>, Vec<Integer>, Vec<Integer>, Vec<Integer>, Vec<Integer>);
    let mut primes_vector = Vec::new();
    let mut strong_base_2_pseudoprimes_vector = Vec::new();
    let mut lucas_pseudoprimes_vector = Vec::new();
    let mut strong_lucas_pseudoprimes_vector = Vec::new();
    let mut baillie_psw_pseudoprimes_vector = Vec::new();
    // join all of the threads
    for handle in thread_handles {
        vector_tuple = handle.join().unwrap();
        primes_vector.append(&mut vector_tuple.0);
        strong_base_2_pseudoprimes_vector.append(&mut vector_tuple.1);
        lucas_pseudoprimes_vector.append(&mut vector_tuple.2);
        strong_lucas_pseudoprimes_vector.append(&mut vector_tuple.3);
        baillie_psw_pseudoprimes_vector.append(&mut vector_tuple.4);
    }    
    (
        primes_vector,
        strong_base_2_pseudoprimes_vector,
        lucas_pseudoprimes_vector,
        strong_lucas_pseudoprimes_vector,
        baillie_psw_pseudoprimes_vector,
    )
}
