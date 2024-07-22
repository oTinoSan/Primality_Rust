use num_iter::{range, range_inclusive, range_step_inclusive};
use rug::ops::AssignRound;
use rug::{float::Round, Float, Integer};
use rug_polynomial::ModPoly;
use std::collections::HashMap;

pub fn u64AKS(prime_candidate: u64) -> bool {
    let prime_int = Integer::from(prime_candidate.clone());
    let mut prime_float: Float = Float::with_val(31, 0);
    prime_float.assign_round(prime_int.clone(), Round::Nearest);
    let log2n = prime_float.clone().log2();
    //check if n is a perfect power
    if prime_int.clone().is_perfect_power() {
        return false;
    }
    let r = 0;
    //find smallest r such that the multiplicative order of prime modulo r is greater than (log2n)^2
    for mut r in num_iter::range(Integer::from(0), prime_int.clone()) {
        r = Integer::from(r);
        for k in num_iter::range(Integer::from(0), r.clone()) {
            if prime_int.clone().pow_mod(&k, &r.clone()) == Ok(Integer::from(1)) {
                if log2n.clone() * log2n.clone() < r.clone() {
                    if prime_int.clone().gcd(&r.clone()) == Integer::from(1) {
                        break;
                    }
                    return false;
                }
            }
        }
    } // checking for all 2<=a>= min(r,prime_candidate-1) that a does not divide n
    if prime_int.clone() - Integer::from(0) < r.clone() {
        for a in num_iter::range(Integer::from(2), prime_int.clone()) {
            if prime_int.clone() % a == 0 {
                return false;
            }
        }
    } else {
        for a in num_iter::range_inclusive(Integer::from(2), Integer::from(r)) {
            if prime_int.clone() % a == 0 {
                return false;
            }
        }
    }
    // if n =< r output prime
    if prime_int.clone() <= r {
        return true;
    }
    // check finite ring
    let mut totient = Float::new(53);
    for i in num_iter::range(Integer::from(1), prime_int.clone()) {
        if prime_int.clone().gcd(&i) == Integer::from(1) {
            totient += 1;
        }
    }
    let roof = (totient.sqrt() * log2n)
        .to_integer_round(Round::Down)
        .unwrap();
    for a in num_iter::range(Integer::from(1), roof.0) {
        let mut polynomial: ModPoly =
            ModPoly::with_roots(vec![-a.clone()], &Integer::from(u64::MAX)); // X + a
        let p = polynomial.clone();
        for _ in num_iter::range_inclusive(Integer::from(1), prime_int.clone()) {
            polynomial.mul(&p); // (X + a)^n
        }

        let mut eq_poly = ModPoly::from_int(Integer::from(u64::MAX), a.clone()); // X^n + a
        eq_poly.set_coefficient((prime_candidate + 1).try_into().unwrap(), &Integer::from(1));
        let mut mod_poly = ModPoly::from_int(Integer::from(u64::MAX), Integer::from(-1)); // X^r - 1
        mod_poly.set_coefficient(r + 1, &Integer::from(1));
        // if (X+a)^n ≠ X^n+a (mod X^r − 1,n), then output composite
        if polynomial.rem(&mod_poly) != eq_poly.rem(&mod_poly)
            || polynomial.rem(&ModPoly::from_int(
                Integer::from(u64::MAX),
                prime_int.clone(),
            )) != eq_poly.rem(&ModPoly::from_int(
                Integer::from(u64::MAX),
                prime_int.clone(),
            ))
        {
            return false;
        }
    }

    return true;
}

// pub fn BigIntAKS(prime_candidate: Integer) -> bool {
//     let mut prime_float = Float::new(0);
//     prime_float.assign_round(prime_candidate, Round::Nearest);
//     let log2n = prime_float.clone().log2();
//     //check if n is a perfect power
//     if prime_candidate.is_perfect_power() {
//         return false;
//     }
//     let mut r = Integer::from(0);
//     //find smallest r such that the multiplicative order of prime modulo r is greater than (log2n)^2
//     for r in num_iter::range(Integer::from(0), prime_candidate) {
//         r = Integer::from(r);
//         for k in num_iter::range(Integer::from(0), r) {
//             if prime_candidate.pow_mod(&k, &r.clone()) == Ok(Integer::from(1)) {
//                 if log2n.clone() * log2n.clone() < r.clone() {
//                     if prime_candidate.gcd(&r.clone()) == Integer::from(1) {
//                         break;
//                     }
//                     return false;
//                 }
//             }
//         }
//     } // checking for all 2<=a>= min(r,prime_candidate-1) that a does not divide n
//     if prime_candidate.clone() - Integer::from(0) < r.clone() {
//         for a in num_iter::range(Integer::from(2), prime_candidate) {
//             if prime_candidate % a == 0 {
//                 return false;
//             }
//         }
//     } else {
//         for a in num_iter::range_inclusive(Integer::from(2), r) {
//             if prime_candidate % a == 0 {
//                 return false;
//             }
//         }
//     }
//     // if n =< r output prime
//     if prime_candidate <= r {
//         return true;
//     }
//     // check finite ring
//     let mut totient = Float::new(53);
//     for i in num_iter::range(Integer::from(1), prime_candidate) {
//         if prime_candidate.gcd(&i) == Integer::from(1) {
//             totient += 1;
//         }
//     }
//     let roof = (totient.sqrt() * log2n)
//         .to_integer_round(Round::Down)
//         .unwrap();
//     for a in num_iter::range(Integer::from(1), roof.0) {
//         let mut polynomial: Polynomial = Polynomial::initialize_polynomial(HashMap::new(), Integer::from(1));
//         polynomial.set_polynomial_coef(Integer::from(0), a);
//         polynomial.set_polynomial_coef(Integer::from(1), Integer::from(1)); // X + a
//         let p = polynomial.clone();
//         for _ in num_iter::range_inclusive(Integer::from(1), prime_candidate) {
//             polynomial.polynomial_modular_multiplication(p, prime_candidate); // (X + a)^n
//         }
//         let poly_no_mod = p.clone();
//         for _ in num_iter::range_inclusive(Integer::from(1), prime_candidate) {
//             poly_no_mod.polynomial_multiplication(p, prime_candidate); // (X + a)^n
//         }
//         let mut eq_poly = Polynomial::initialize_polynomial(HashMap::new(), prime_candidate); // X^n + a
//         eq_poly.set_polynomial_coef(Integer::from(0), a);
//         eq_poly.set_polynomial_coef(prime_candidate, Integer::from(1));
//         let mut mod_poly = Polynomial::initialize_polynomial(HashMap::new(), r); // X^r - 1
//         mod_poly.set_polynomial_coef(Integer::from(1), Integer::from(-1));
//         mod_poly.set_polynomial_coef(r + 1, Integer::from(1));
//         let mut r_poly = Polynomial::initialize_polynomial(HashMap::new(), Integer::from(0));
//         r_poly.set_polynomial_coef(Integer::from(0), Integer::from(prime_candidate));
//         // if (X+a)^n ≠ X^n+a (mod X^r − 1,n), then output composite
//         if (poly_no_mod.polynomial_remainder(&mod_poly) != eq_poly.rem(&mod_poly)
//             || polynomial != eq_poly.polynomial_modular_multiplication())
//         {
//             return false;
//         }
//     }

//     return true;
// }

// pub struct Polynomial {
//     coef: HashMap<Integer, HashMap<Integer, Integer>>,
//     deg: Integer,
// }

// impl Polynomial {
//     pub fn initialize_polynomial(
//         coef: HashMap<Integer, HashMap<Integer, Integer>>,
//         deg: Integer,
//     ) -> Polynomial {
//         Polynomial { coef, deg }
//     }
//     pub fn is_equal_polynomial(&self, poly_2: &Polynomial) -> bool {
//         if self.deg != poly_2.deg {
//             return false;
//         }
//         for i in range_inclusive(Integer::from(0), self.deg) {
//             if self.coef.get(i) != poly_2.coef.get(i) {
//                 return false;
//             }
//         }
//         return true;
//     }

//     pub fn set_polynomial_coef(&self, order: Integer, coefficient: Integer) {
//         if order <= self.deg + 1 {
//             self.coef.insert(order, coefficient);
//             return;
//         } else {
//             for index in range(Integer::from(self.deg + 1), order) {
//                 if index > 2^64 * (self.coef.len() + 1) {
//                     self.coef.insert(self.coef.len()+1, HashMap::new());
//                 }
//                 let a = self.coef.get(&(index / 2^64 + 1));
//                 a.insert(index, 0);
//             }
//             let b = self.coef.get(&Integer::from(self.coef.len()));
//             b.insert(order, coefficient);
//             self.deg = order;
//         }
//     }

//     pub fn polynomial_modular_multiplication(
//         &self,
//         poly_2: &Polynomial,
//         mod_n: Integer,
//     ) -> Polynomial {
//         let max_deg = self.deg * poly_2.deg;
//         let mut poly_res = Self::initialize_polynomial(HashMap::new(), max_deg);
//         for i in range(Integer::from(0), max_deg) {
//             let mut coef = 0;
//             let mut home_1;
//             let mut c0;
//             let mut home_2;
//             let mut c1;
//             let mut jmin = if i > poly_2.deg {
//                 Integer::from(i - poly_2.deg)
//             } else {
//                 Integer::from(0)
//             };
//             let mut jmax = if i < self.deg {
//                 Integer::from(i)
//             } else {
//                 self.deg
//             };

//             for j in range_inclusive(jmin, jmax) {
//                 home_1 = self.coef.get(&j/ 2^64 +1).unwrap();
//                 c0 = home_1.get(&j);
//                 home_2 = poly_2.coef.get(&(&i - &j) / 2^64 +1).unwrap();
//                 c1 = home_2.get(&(i - j));
//                 let c0 = &Integer::from(c0 * c1);
//                 coef = c0 + coef;
//             }
//             jmin = Integer::from(i - poly_2.deg);
//             jmax = self.deg;
//             for j in range_inclusive(jmin, jmax) {
//                 home_1 = self.coef.get(&j/ 2^64 +1).unwrap();
//                 c0 = self.coef.get(j.clone());
//                 home_2 = poly_2.coef.get((&(&i-&j))/ 2^64 +1).unwrap();
//                 c1 = poly_2.coef.get(&(i - j));
//                 c0 = &Integer::from(c0 * c1);
//                 coef = c0 + coef;
//             }
//             coef = coef % mod_n;
//             let b = self.coef.get(&Integer::from(self.coef.len()));
//             poly_res.coef.insert(i, coef);
//         }
//         return poly_res;
//     }
//     pub fn polynomial_modular_power(&self, exp: Integer) -> Polynomial {
//         let mut poly_res = Self::initialize_polynomial(HashMap::new(), Integer::from(0));
//         poly_res.set_polynomial_coef(Integer::from(0), Integer::from(1));
//         for i in range_inclusive(exp.significant_bits() + 1, 1).rev() {
//             poly_res = self.polynomial_modular_multiplication(&poly_res, exp.clone());
//             if exp.get_bit(i - 1) {
//                 poly_res = poly_res.polynomial_modular_multiplication(&self, exp.clone());
//             }
//         }
//         return poly_res;
//     }
//     pub fn polynomial_remainder(&self, poly_2: &Polynomial) -> &Polynomial {
//         loop {
//             if poly_2.deg >= self.deg {
//                 return self;
//             }
//             let mut multiple_poly =
//                 Self::initialize_polynomial(HashMap::new(), self.deg - poly_2.deg);
//             multiple_poly.set_polynomial_coef(self.deg - poly_2.deg, Integer::from(1));
//             let long_division_poly = poly_2.clone().polynomial_multiplication(&multiple_poly);
//             for i in range(Integer::from(1), Integer::from(self.coef.len() + 1)) {
//                 self.set_polynomial_coef(i, &(self.coef.get(&i) - long_division_poly.coef.get(&i)));
//             }
//             for i in range_inclusive(self.coef.len(), 1).rev() {
//                 if self.coef.get(&i) == Integer::from(0) {
//                     self.deg = self.deg - 1;
//                 } else {
//                     break;
//                 }
//             }
//         }
//     }
//     pub fn polynomial_multiplication(&self, poly_2: &Polynomial) -> Polynomial {
//         let max_deg = self.deg * poly_2.deg;
//         for i in range(Integer::from(1), Integer::from(max_deg + 1 / 2^64 + 1)){
//             for j in range_inclusive(Integer::from(0), Integer::from(2^64-1)){
//                 let tuple_list
//                 poly_res.coef.insert(i, );
//             }

//         }
//         let mut poly_res = Self::initialize_polynomial(HashMap::new(), max_deg);
//         for i in range(Integer::from(0), max_deg) {
//             let mut coef = Integer::from(0);
//             let mut home_1;
//             let mut c0;
//             let mut home_2;
//             let mut c1;
//             let mut jmin = if i > poly_2.deg {
//                 i - poly_2.deg
//             } else {
//                 Integer::from(0)
//             };
//             let mut jmax = if i < self.deg { i } else { self.deg };

//             for j in range_inclusive(jmin, jmax) {
//                 home_1 = self.coef.get(j.clone()/2^64).unwrap();
//                 c0 = home_1.get(&j).unwrap();
//                 home_2 = poly_2.coef.get(j.clone()/2^64).unwrap();
//                 c1 = home_2.get(&(i - j)).unwrap();
//                 let c0 = &Integer::from(c0 * c1);
//                 coef = c0 + coef;
//             }
//             jmin = i - poly_2.deg;
//             jmax = self.deg;
//             for j in range_inclusive(jmin, jmax) {
//                 home_1 = self.coef.get(&j / 2^64).unwrap();
//                 c0 = home_1.get(&j).unwrap();
//                 home_2 = self.coef.get(&(i - j) / 2^64).unwrap();
//                 c1 = home_2.get(&(i - j)).unwrap();
//                 c0 = &Integer::from(c0 * c1);
//                 coef = c0 + coef;
//             }
//             let b = poly_res.coef.get(i/ 2^64 + 1);
//             b.insert(i, Integer::from(coef));
//         }
//         return poly_res;
//     }
// }
