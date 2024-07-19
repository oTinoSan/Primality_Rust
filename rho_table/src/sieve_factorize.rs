use std::collections::HashMap;

use num::integer::Roots;

/// use an implementation of the sieve of eratosthenes to generate the prime factorization of every number up to max
pub fn sieve_factorize(max: u64) -> HashMap<u64, HashMap<u64, u64>> {
    let mut factors = HashMap::new();
    for cur in 2 ..= (max.sqrt() + 1) {
        if !factors.contains_key(&cur) {
            factors.insert(cur, HashMap::from([(cur, 1)]));
            for mult in cur ..= max / cur {
                match factors.get(&mult) {
                    None => {
                        factors.insert(mult * cur, HashMap::from([(cur, 1), (mult, 1)]));
                    },
                    Some(factorization) => {
                        let mut factorization = factorization.clone();
                        *factorization.entry(cur).or_insert(0) += 1;
                        factors.insert(mult * cur, factorization);
                    }
                }
            }
        }

    }
    for i in max.sqrt()..=max {
        factors.entry(i).or_insert(HashMap::from([(i, 1)]));
    }
    factors
}