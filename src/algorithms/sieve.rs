pub fn sieve(candidate: u32) -> bool {
    if candidate <= 1 {
        return false;
    }
    let limit = (candidate as f64).sqrt() as u32;
    let mut primes = vec![true; (candidate + 1) as usize];
    primes[0] = false;
    primes[1] = false;

    for num in 2..=limit {
        if primes[num as usize] {
            let mut multiple = num * num;
            while multiple <= candidate {
                // Change here
                primes[multiple as usize] = false;
                multiple += num;
            }
        }
    }

    primes[candidate as usize]
}

pub fn sieve_vect(limit: u32) -> Vec<u32> {
    (2..=limit).filter(|&num| sieve(num)).collect()
}
