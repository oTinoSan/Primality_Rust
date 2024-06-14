#[cfg(test)]
mod tests;

pub fn trial_division(num: u64) -> bool {
    for i in 2..=f64::sqrt(num as f64) as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub fn trial_divisions(num: u64) -> Vec<u64> {
    let mut primes = vec![];
    for i in 2..=num {
        if trial_division(i) {
            primes.push(i);
        }
    }
    primes
}

pub fn wheel_primes(mut basis: Vec<u64>, max_val: u64) -> Vec<u64> {
    let mut product = basis.iter().product();
    let mut wheel: Vec<u64> = (0..product).collect();
    let mut new_wheel = wheel.clone();
    for n in wheel.iter() {
        for v in basis.iter() {
            if n % v == 0 {
                new_wheel.retain(|x| x != n);
                break;
            }
        }
    }
    wheel = new_wheel.clone();
    while max_val > product {
        new_wheel = wheel.clone();
        let p = wheel[1];
        for i in 1..p {
            new_wheel.extend(wheel.iter().map(|x| x + i * product))
        }
        let filter: Vec<_> = wheel.iter().map(|x| x * p).collect();
        wheel = new_wheel
            .into_iter()
            .filter(|x| !filter.contains(x))
            .collect();
        basis.push(p);
        product = basis.iter().product();
    }

    let mut p = wheel[1];
    while p < f64::sqrt(*wheel.last().unwrap() as f64) as u64 {
        let filter: Vec<_> = wheel.iter().map(|x| x * p).collect();
        wheel.retain(|x| !filter.contains(x));
        basis.push(p);
        p = wheel[1];
    }

    basis.extend_from_slice(&wheel[1..]);
    basis.retain(|x| *x <= max_val);
    basis
}

pub fn sieve_primes(max_val: u64) -> Vec<u64> {
    let mut is_prime = vec![true; (max_val + 1) as usize];
    is_prime[0] = false;
    if max_val > 0 {
        is_prime[1] = false;
    }
    let mut p = 2;
    while p < max_val {
        for i in 2..=max_val / p {
            is_prime[(i * p) as usize] = false;
        }
        let old_p = p;
        for i in (p + 1)..max_val {
            if is_prime[i as usize] {
                p = i;
                break;
            }
        }
        if old_p == p {
            break;
        }
    }
    let mut primes: Vec<_> = (0..=max_val).collect();
    let mut iter = is_prime.iter();
    primes.retain(|_| *iter.next().unwrap());
    primes
}
