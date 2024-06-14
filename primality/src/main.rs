use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_1: u32 = args[1].trim().parse().expect("Must be an integer");
    let primes = mult_prime(arg_1);
    // println!("{:?}", primes);
}

pub fn mult_prime(limit: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    for num in 2..=limit {
        if prime_test(num) {
            primes.push(num);
        }
    }
    primes
}

fn prime_test(candidate: u32) -> bool {
    let sqrt = (candidate as f64).sqrt() as u32;
    let mut is_prime = true;
    for divisor in 2..=sqrt {
        if candidate % divisor == 0 {
            is_prime = false;
            break;
        }
    }
    // println!("{} is prime? {}", candidate, is_prime);
    is_prime
}

// fn main() {
//     println!("Enter a number: ");
//     let mut input = String::new();
//     io::stdin()
//        .read_line(&mut input)
//        .expect("Failed to read line!");
//     let max_num: u32 = input.trim()
//         .parse().expect("Please enter a number!");
//     let max_num = (max_num as f64).sqrt();
//     println!("The square root of your number is: {}", max_num);

//     let max_num = max_num as u32;  // Truncate the fractional part

//     let mut all_factors = Vec::new();

//     for num in 2..=max_num {
//         let mut divisors = Vec::new();
//         for digit in 2..num {
//             if num % digit == 0 {
//                 divisors.push(digit);
//             }
//         }

//         if !divisors.is_empty() {
//             all_factors.push((num, divisors));
//         } else {
//             println!("The number {} is prime!", num);
//         }
//     }

//     for (num, factors) in &all_factors {
//         println!("The number {} has divisors: {:?}", num, factors);
//     }
