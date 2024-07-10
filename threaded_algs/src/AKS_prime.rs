use rug::ops::AssignRound;
use rug::{float::Round, Float, Integer};
use rug_polynomial::ModPoly;

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
//     let roof = (totient.sqrt() * log2n).to_integer_round(Round::Down).unwrap();
//     for a in num_iter::range(Integer::from(1), roof.0) {
//         let mut polynomial: Polynomial =
//             Self::initialize_polynomial([a, Integer::from(1)], Integer(1)); // X + a
//         let p = polynomial.clone();
//         for _ in num_iter::range_inclusive(Integer::from(1), prime_candidate) {
//             polynomial.polynomial_modular_multiplication(p, prime_candidate); // (X + a)^n
//         }
//         let poly_no_mod = p.clone();
//         for _ in num_iter::range_inclusive(Integer::from(1), prime_candidate) {
//             poly_no_mod.polynomial_multiplication(p, prime_candidate); // (X + a)^n
//         }
//         let mut eq_poly = Self::initialize_polynomial(vec![a], prime_candidate); // X^n + a
//         eq_poly.set_coefficient_coef(prime_candidate + 1, Integer::from(1));
//         let mut mod_poly = Self::initialize_polynomial(vec![Integer::from(-1)], r); // X^r - 1
//         mod_poly.set_coefficient(r + 1, Integer::from(1));
//         // if (X+a)^n ≠ X^n+a (mod X^r − 1,n), then output composite
//         if (poly_no_mod.polynomial_remainder(&mod_poly) != eq_poly.rem(&mod_poly)
//             || polynomial
//                 != eq_poly.polynomial_modular_multiplication(Self::initialize_polynomial(
//                     vec![Integer::from(1)],
//                     Integer::from(0),
//                 )))
//         {
//             return false;
//         }
//     }

//     return true;
// }

// struct Polynomial {
//     coef: Vec<Integer>,
//     deg: Integer,
// }

// impl Polynomial {
//     fn initialize_polynomial(coef: Vec<Integer>, deg: Integer) -> Polynomial {
//         Polynomial { coef, deg }
//     }
//     fn is_equal_polynomial(&self, poly_2: &Polynomial) -> bool {
//         if self.deg != poly_2.deg {
//             return false;
//         }
//         for i in range_inclusive(Integer::from(0), self.deg) {
//             if self.coef[i] != poly_2.coef[i] {
//                 return false;
//             }
//         }
//         return true;
//     }

//     fn set_polynomial_coef(&self, order: Integer, coefficient: Integer) {
//         if order <= self.deg + 1 {
//             self.coef[order] = coefficient;
//             return;
//         }
//         for index in range(self.deg + 1, order) {
//             self.coef.push(0);
//         }
//         self.coef.push(coefficient);
//         self.deg = order;
//     }

//     fn polynomial_modular_multiplication(&self, poly_2: &Polynomial, mod_n: Integer) -> Polynomial {
//         let max_deg = self.deg * poly_2.deg;
//         let mut poly_res = Self::initialize_polynomial(vec![], max_deg);
//         for i in range(Integer::from(0), max_deg) {
//             let mut coef = 0;
//             let mut c0;
//             let mut c1;
//             let mut jmin = if i > poly_2.deg { i - poly_2.deg } else { 0 };
//             let mut jmax = if i < self.deg {
//                 Integer::from(i)
//             } else {
//                 self.deg
//             };

//             for j in range_inclusive(jmin, jmax) {
//                 c0 = self.coef[j];
//                 c1 = poly_2.coef[i - j];
//                 let c0 = c0 * c1;
//                 coef = c0 + coef;
//             }
//             jmin = i + r - poly_2.deg;
//             jmax = self.deg;
//             for j in range_inclusive(jmin, jmax) {
//                 c0 = self.coef[j];
//                 c1 = poly_2.coef[i + r - j];
//                 c0 = c0 * c1;
//                 coef = c0 + coef;
//             }
//             coef = coef % mod_n;
//             poly_res.coef.push(coef);
//         }
//         return poly_res;
//     }
//     fn polynomial_modular_power(&self, exp: Integer) -> Vec<Integer> {
//         let mut poly_res = Self::initialize_polynomial(vec![], 0);
//         poly_res.set_polynomial_coef(0, 1);
//         for i in range_step_inclusive(exp.significant_bits() + 1, 1) {
//             poly_res = self.polynomial_modular_multiplication(poly_res, poly_res, exp);
//             if exp.get_bit(i - 1) {
//                 poly_res = poly_res.polynomial_modular_multiplication(&self, exp.clone());
//             }
//         }
//         return poly_res;
//     }
//     fn polynomial_remainder(&self, poly_2: &Polynomial) -> &Polynomial {
//         loop {
//             if poly_2.deg >= self.deg {
//                 return self;
//             }
//             let mut multiple_poly = Self::initialize_polynomial(vec![], self.deg - poly_2.deg);
//             multiple_poly.set_coefficient_coef(self.deg - poly_2.deg, 1);
//             let long_division_poly = poly_2.clone().polynomial_multiplication(&multiple_poly);
//             for i in range(1, self.coef.len + 1) {
//                 self.coef[i] = self.coef[i] - long_division_poly.coef[i];
//             }
//             for i in range_step_inclusive(self.coef.len, 1, -1) {
//                 if self.coef[i] == 0 {
//                     self.deg = self.deg - 1;
//                 } else {
//                     break;
//                 }
//             }
//         }
//     }
//     fn polynomial_multiplication(&self, poly_2: &Polynomial) -> Polynomial {
//         let max_deg = self.deg * poly_2.deg;
//         let mut poly_res = Self::initialize_polynomial(vec![], max_deg);
//         for i in range(Integer::from(0), max_deg) {
//             let mut coef = Integer::from(0);
//             let mut c0;
//             let mut c1;
//             let mut jmin = if i > poly_2.deg { i - poly_2.deg } else { Integer::from(0) };
//             let mut jmax = if i < self.deg { i } else { self.deg };

//             for j in range_inclusive(jmin, jmax) {
//                 c0 = self.coef[j];
//                 c1 = poly_2.coef[i - j];
//                 let c0 = c0 * c1;
//                 coef = c0 + coef;
//             }
//             jmin = i + r - poly_2.deg;
//             jmax = self.deg;
//             for j in range_inclusive(jmin, jmax) {
//                 c0 = self.coef[j];
//                 c1 = poly_2.coef[i + r - j];
//                 c0 = c0 * c1;
//                 coef = c0 + coef;
//             }
//             poly_res.coef.push(Integer::from(coef));
//         }
//         return poly_res;
//     }
// }
