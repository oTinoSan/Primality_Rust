use lamellar::array;

pub mod baillie_psw;
pub mod bigint;
pub mod miller_rabin_num;
pub mod miller_rabin_rug;
pub mod prime_search;
pub mod solovay_strassen;

pub fn trial_division(limit: u64) -> Vec<bool> {
    let mut array: Vec<bool> = vec![false; limit as usize];

    'primes: for prime_candidate in 2..limit as usize {
        let sqrt = (f64::sqrt(prime_candidate as f64)) as usize;
        for divisor in 2..=sqrt {
            if prime_candidate % divisor == 0 {
                continue 'primes;
            }
        }
        array[prime_candidate] = true;
        // println!("{}",num);
    }
    return array;
}

pub fn trial_division_odds(limit: u64) -> Vec<bool> {
    let mut array: Vec<bool> = vec![false; limit as usize];

    array[2] = true; // 2 is prime

    'primes: for prime_candidate in (3..limit as usize).step_by(2) {
        let sqrt = (f64::sqrt(prime_candidate as f64)) as usize;
        for divisor in 2..=sqrt {
            if prime_candidate % divisor == 0 {
                continue 'primes;
            }
        }
        array[prime_candidate] = true;
        // println!("{}",num);
    }
    return array;
}

struct Sieve {
    array: Vec<bool>,
    current: u64,
}

impl Iterator for Sieve {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {

        while true {
            if self.array[self.current as usize] {
                for n in (self.current * self.current..u64::MAX).step_by(self.current as usize) {
                    self.array[n as usize] = false;
                }
                return Some(self.current);
            }
            self.current += 1;
        }

        return None;
    }
}

fn sieve_impl() -> Sieve {
    let mut array: Vec<bool> = vec![true; u64::MAX as usize];
    array[0] = false;
    array[1] = false;
    Sieve {array, current: 2}
}

pub fn sieve_of_eratosthenes(int: u128) -> Vec<bool> {
    let mut array: Vec<bool> = vec![true; (int + 1) as usize];

    let mut i: usize = 2;
    array[0] = false;
    array[1] = false;
    while i <= ((f64::sqrt(int as f64)) as u128 + 1) as usize {
        if array[i] {
            for n in (i * i..int as usize).step_by(i) {
                array[n] = false;
            }
        }
        i = i + 1;
    }
    array[int as usize] = false;
    return array;
}

// pub fn bigint_sieve_of_eratosthenes(limit: Integer) -> Vec<Integer> {
//     let mut array: Vec<Integer> = Vec::new();
//     let mut i: Integer = Integer::from(2);
//     while i <= &i.sqrt() + 1 {
//         if
//     }
//     todo!()
// }

// pub fn sieve_of_eratosthenes2(int: u64) -> Vec<bool> {
//     let mut array: Vec<bool> = vec![true; (int + 1) as usize];

//     let mut i: usize = 2;
//     let sqrt = ((f64::sqrt(int as f64)) as u64 + 1).try_into().unwrap();
//     while i <= sqrt {
//         if array[i] {
//             for n in (i * i..int as usize).step_by(i) {
//                 array[n] = false;
//             }
//         }
//         i = i + 1;
//     }
//     return array;
// }

pub fn wheel_factoring_single(prime_candidate: u64) -> bool {
    //first primes to generate the wheel with
    let first_primes = vec![2, 3, 5];
    let wheel = [7, 11, 13, 17, 19, 23, 29, 31];
    // let wheel = generate_wheel(first_primes.clone());
    let wheel_mod = 30; // product of first_primes

    //test the first primes against candidate number
    for p in &first_primes {
        if prime_candidate % p == 0 {
            return false;
        }
    }

    let sqrt = (f64::sqrt(prime_candidate as f64)) as usize;

    for turn_num in (0..=sqrt).step_by(wheel_mod) {
        for spoke in &wheel {
            if prime_candidate % (turn_num as u64 + *spoke as u64) == 0 {
                if prime_candidate == (turn_num as u64 + *spoke as u64) {
                    return true; //edge case for primes in the first turn of the wheel
                }
                return false;
            }
        }
    }

    //if we got this far it must be prime
    return true;
}

pub fn wheel_factoring(limit: u64) -> Vec<bool> {
    let mut array: Vec<bool> = vec![true; limit as usize];

    //first primes to generate the wheel with
    let first_primes = vec![2, 3, 5];
    // let wheel = generate_wheel(first_primes.clone());
    let wheel = vec![7, 11, 13, 17, 19, 23, 29, 31];
    let wheel_mod = 30; // product of first_primes
                        // println!("{:?}",wheel);

    array[0] = false;
    array[1] = false;

    'primes: for candidate_prime in 5..limit {
        //test the first primes against candidate number
        for p in &first_primes {
            if candidate_prime % p == 0 {
                array[candidate_prime as usize] = false;
                continue 'primes; // jump to next candidate prime
            }
        }

        let sqrt = (f64::sqrt(candidate_prime as f64)) as usize;

        for turn_num in (0..=sqrt).step_by(wheel_mod) {
            for spoke in &wheel {
                if candidate_prime % (turn_num as u64 + *spoke as u64) == 0 {
                    if candidate_prime == (turn_num as u64 + *spoke as u64) {
                        continue 'primes; //edge case for primes in the first turn of the wheel
                    }
                    array[candidate_prime as usize] = false;
                    continue 'primes;
                }
            }
        }

        //if we got this far it must be prime
        array[candidate_prime as usize] = true;
    }
    return array;
}

pub fn generate_wheel(first_primes: Vec<u64>) -> Vec<u64> {
    // generate an array of all numbers coprime to the given primes
    // up to the product of the numbers
    let mut product = 1;
    for p in first_primes.iter() {
        product *= p;
    }

    let mut wheel: Vec<u64> = Vec::new();

    //generate list of numbers from 0 to the product and remove
    //the multiples of numbers from the list of primes
    let mut list_of_numbers = vec![true; product as usize];

    list_of_numbers[0] = false;
    list_of_numbers[1] = false;

    for p in first_primes {
        for multples in (p..product).step_by(p as usize) {
            list_of_numbers[multples as usize] = false;
        }
    }

    // add those numbers left over to the wheel
    for n in 0..product {
        if list_of_numbers[n as usize] {
            wheel.push(n);
        }
    }
    return wheel;
}
