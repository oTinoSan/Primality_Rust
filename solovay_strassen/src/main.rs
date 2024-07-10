use mod_exp::mod_exp;
use rand::{thread_rng, Rng};
use std::env;
use rug::{Assign, Integer};


fn main() {
    let args: Vec<String> = env::args().collect();
    let number: u64 = args[1].trim().parse().expect("Must be an integer");
    let solovay_primes = solovay_strassen_list(10, number);
    println!("Primes : {:?}", solovay_primes);
    // let primes = solovay_list(number);
    // println!("{:?}", primes);
}

// fn solovay_list(limit: u32) -> Vec<u32> {
//     let mut primes: Vec<u32> = Vec::new();
//     // let sqrt = (limit as f64).sqrt() as u32;
//     for num in 5..=limit {
//         if solovay_test(num) {
//             primes.push(num);
//         }
//     }
//     primes
// }

// fn solovay_test(candidate: u32) -> bool {
//     // while n<=number{
//     let a = thread_rng().gen_range(1..=(candidate - 1));
//     if mod_exp(a, (candidate - 1) / 2, candidate) as i32
//         == legendre_simp(a as i32, candidate as i32)
//     {
//         return false;
//     } else {
//         return true;
//     }
// }

// fn legendre_simp(random: i32, odd: i32) -> i32 {
//     let mut a = random;
//     println!("{a}");
//     let mut n = odd;
//     loop {
//         a = a % n;
//         // step 1 done
//         if a == 0 {
//             return a;
//         }
//         let mut exp = 0;
//         loop {
//             if a % 2 == 0 {
//                 exp += 1;
//                 a = a / 2;
//             } else {
//                 break;
//             }
//         }
//         // step 3 done
//         let n_mod = n % 8;
//         let mut k = 0;
//         if (n_mod == 1) ^ (n_mod == 7) {
//             k = 1;
//         } else if (n_mod == 3) ^ (n_mod == 5) {
//             k = -1;
//             for _i in (1..exp).rev() {
//                 k *= -1;
//             }
//         }
//         a = k * a;
//         // step 2 done
//         let m = n;
//         if (a % 4 == 3) & (n % 4 == 3) {
//             n = -a;
//             a = m;
//         } else {
//             n = a;
//             a = m;
//         }
//         println!("{}", a);
//         // step 4 done
//         if a == 1 || a == -1 || a == 0 {
//             return a;
//         }
        
//     }
// }

#[derive(Debug, Clone)]
struct Jacobi {
    a: u64,
    n: u64,
    sign: bool,
}

impl Jacobi {
    fn new(a:u64, n: u64) -> Jacobi {
        Jacobi {
            a,
            n,
            sign: false,
        }
    }

    fn mod_reduce(&mut self) {
        self.a = self.a % self.n;
    }

    fn remove_twos(&mut self) {
        let pow = self.a.trailing_zeros();
        self.a = self.a >> pow;
        let mod_8 = self.n % 8;
        if !(pow % 2 == 0 || mod_8 == 1 || mod_8 == 7){
            self.sign = !self.sign;
        }
    }

    fn invert(&mut self){
        if self.a % 4 == 3 && self.n % 4 == 3 {
            self.sign = !self.sign;
        }
        let temp = self.a;
        self.a = self.n;
        self.n = temp;
    }

    fn eval(&mut self) -> i32 {
        if self.a % 2 == 0{
            self.remove_twos();
        }
        while self.a > 1{
            self.invert();
            self.mod_reduce();
            if self.a == 0{
                return 0;
            }
            self.remove_twos();
        }
        if self.sign {
            return -1;
        } else {
            return 1;
        }
    }
}

fn solovay_strassen(num_tests: u64, candidate: u64) -> bool {
    for _ in 0..num_tests{
        let a = thread_rng().gen_range(2..=(candidate-2));
        let mut jacobi = Jacobi::new(a, candidate);
        let jacobi_result = jacobi.eval();
        let mod_result = mod_exp(a, candidate -1, candidate);
        if mod_result == 0 {
            return false;
        }
        if (mod_result == 1 && jacobi_result == 1) || (mod_result == candidate -1 && jacobi_result == -1) {
            return true;
        }
    }
    return false;
    }

fn solovay_strassen_list(num_tests: u64, max_val: u64) -> Vec<u64>{
    let mut primes= vec![];

    for i in (5..=max_val).step_by(2){
        if solovay_strassen(num_tests, i){
            primes.push(i);
        }
    }

    primes
}
