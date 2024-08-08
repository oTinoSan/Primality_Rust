use mod_exp::mod_exp;
use num::{bigint::RandBigInt, BigInt, FromPrimitive};
use rand::Rng;

pub fn solovay_strassen(limit: u64) -> Vec<bool> {
    let mut array = vec![false; limit as usize];
    for i in (5..limit as usize).step_by(2) {
        array[i] = bigint_solovay_strassen_test(BigInt::from_u64(i as u64).expect("failed"));
    }
    return array;
}

pub fn bigint_solovay_strassen_test(prime_candidate: BigInt) -> bool {
    for _ in 0..10 {
        let random_num = rand::thread_rng().gen_bigint_range(&BigInt::from(2), &prime_candidate);

        //evaluate jacobi symbol using wikipedia algorithm

        let mut a = random_num.clone();
        let mut n = prime_candidate.clone();
        a = a % &n; // step 1
        let mut result = 1;
        let mut r: BigInt;

        //step 3
        while a != BigInt::from(0) {
            //step 2
            while &a % 2 == BigInt::from(0) {
                a /= 2;
                r = &n % 8;
                if r == BigInt::from(3) || r == BigInt::from(5) {
                    result = -result;
                }
            }

            //step 4
            r = n;
            n = a;
            a = r;
            if &a % 4 == BigInt::from(3) && &n % 4 == BigInt::from(3) {
                result = -result;
            }
            a %= &n;
        }
        if n != BigInt::from(1) {
            result = 0;
        }
        let modpow_result = random_num.modpow(&((&prime_candidate - 1) / 2), &prime_candidate);
        if result == 0
            || !(modpow_result == BigInt::from(result)
                || (modpow_result == &prime_candidate - 1 && result == -1))
        {
            // println!("candidate: {}, rand: {}, result: {}, false, a^n-1/2: {}", prime_candidate, random_num, result, random_num.modpow(&((prime_candidate.clone()-1) /2), &prime_candidate));
            return false; //composite
        }
        // println!("candidate: {}, rand: {}, result: {}, true", prime_candidate, random_num, result);
    }

    return true; //probably prime
}

pub fn solovay_strassen_test(prime_candidate: u64) -> bool {
    for _ in 0..10 {
        let random_num: u64 = rand::thread_rng()
            .gen_range(2..=(prime_candidate - 2))
            .try_into()
            .unwrap();

        let mut jacobi: JacobiSymbol = JacobiSymbol::new(random_num, prime_candidate);

        // println!("jacobi = {:?}", jacobi);

        let result = jacobi.evaluate();
        let x: u64;
        if result == -1 {
            x = prime_candidate - 1;
        } else {
            x = result as u64;
        }
        if x == 0 {
            return false;
        }
        let y: u64 = mod_exp(random_num, (prime_candidate - 1) / 2, prime_candidate);
        if x != y {
            return false;
        }
    }
    return true;
}

#[derive(Debug, Clone)]
pub struct JacobiSymbol {
    top: u64,
    bottom: u64,
    sign: bool,
    simplified: bool,
}

impl JacobiSymbol {
    pub fn new(top: u64, bottom: u64) -> Self {
        JacobiSymbol {
            top,
            bottom,
            sign: true,
            simplified: false,
        }
    }

    pub fn evaluate(&mut self) -> i32 {
        // check whether the given random a value is even and pull it out if it is:
        if self.top % 2 == 0 {
            self.separate();
        }

        while !self.simplified {
            self.flip();
            self.simplify(); // the only step where a zero can show up

            //check for zeros
            if self.top == 0 {
                return 0;
            }
            self.separate();
        }
        if self.sign {
            return 1;
        } else {
            return -1;
        }
    }
    fn flip(&mut self) {
        if self.bottom % 4 == 3 && self.top % 4 == 3 {
            let temp = self.top;
            self.top = self.bottom;
            self.bottom = temp;
            self.sign = !self.sign;
        } else {
            let temp = self.top;
            self.top = self.bottom;
            self.bottom = temp;
        }
    }

    fn simplify(&mut self) {
        self.top = self.top % self.bottom;
    }

    fn separate(&mut self) {
        let k = self.top.trailing_zeros();
        let t = self.top / (2u64.pow(k));
        self.top = t;
        if !JacobiSymbol::evaluate_twos_jacobi(self.bottom, k as u64) {
            self.sign = !self.sign;
        }
        if t == 1 {
            self.simplified = true;
        }
    }

    pub fn evaluate_twos_jacobi(bottom: u64, exp: u64) -> bool {
        // uses property 2 to evaluate a given jacobi with a two in the numerator
        // raised to a power, into either a +1 or -1, returns it as a boolean
        if bottom % 8 == 1 || bottom % 8 == 7 || exp % 2 == 0 {
            return true;
        } else {
            return false;
        }
    }
}
