pub fn wheel_facts(candidate: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut remainder = candidate;
    let wheel = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    for &prime in wheel.iter() {
        while remainder % prime == 0 {
            factors.push(prime);
            remainder /= prime;
        }
    }

    let mut f = 103; // Start from the next prime after 97
    while f * f <= remainder {
        if remainder % f == 0 {
            factors.push(f);
            remainder /= f;
        } else {
            f += 2; // Skip even numbers
        }
    }

    if remainder != 1 {
        factors.push(remainder);
    }

    factors
}

pub fn wheel(candidate: u64) -> bool {
    let factors = wheel_facts(candidate);
    factors.len() <= 2
}
