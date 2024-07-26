use num_iter::{range, range_inclusive};
use rug::ops::AssignRound;
use rug::ops::Pow;
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

pub fn BigIntAKS(prime_candidate: Integer) -> bool {
    //check if n is a perfect power
    if prime_candidate.is_perfect_power() {
        return false;
    }
    let mut prime_float = Float::new(prime_candidate.clone().significant_bits());
    prime_float.assign_round(prime_candidate.clone(), Round::Nearest);
    let log2n = prime_float.clone().log2();
    let mut r_true = Integer::from(0);
    //find smallest r such that the multiplicative order of prime modulo r is greater than (log2n)^2
    for mut r in num_iter::range(Integer::from(0), prime_candidate.clone()) {
        r_true = r.clone();
        for k in num_iter::range(Integer::from(0), r.clone()) {
            if prime_candidate.clone().pow_mod(&k, &r.clone()) == Ok(Integer::from(1)) {
                if log2n.clone() * log2n.clone() < r.clone() {
                    if prime_candidate.clone().gcd(&r.clone()) == Integer::from(1) {
                        break;
                    }
                    return false;
                }
            }
        }
    } // checking for all 2<=a>= min(r,prime_candidate-1) that a does not divide n
    if prime_candidate.clone() - Integer::from(0) < r_true.clone() {
        for a in num_iter::range_inclusive(Integer::from(2), prime_candidate.clone()) {
            if prime_candidate.clone() % a == 0 {
                return false;
            }
        }
    } else {
        for a in num_iter::range_inclusive(Integer::from(2), r_true.clone()) {
            if prime_candidate.clone() % a == 0 {
                return false;
            }
        }
    }
    // if n =< r output prime
    if prime_candidate <= r_true {
        return true;
    }
    // check finite ring
    let mut totient = Float::new(53);
    for i in num_iter::range(Integer::from(1), prime_candidate.clone()) {
        if prime_candidate.clone().gcd(&i) == Integer::from(1) {
            totient += 1;
        }
    }
    let roof = (totient.sqrt() * log2n)
        .to_integer_round(Round::Down)
        .unwrap();
    for a in num_iter::range_inclusive(Integer::from(1), roof.0) {
        let mut polynomial: Polynomial = Polynomial::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            Integer::from(1),
        );
        polynomial.set_polynomial_coef(Integer::from(0), a.clone());
        polynomial.set_polynomial_coef(Integer::from(1), Integer::from(1)); // X + a

        let mut p = polynomial.polynomial_clone();
        for _ in num_iter::range_inclusive(Integer::from(1), prime_candidate.clone() - 1) {
            polynomial = polynomial.polynomial_multiplication(&p);
            // (X + a)^n not modular // working
        }
        let mut eq_poly = Polynomial::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            prime_candidate.clone(),
        ); // X^n + a
        eq_poly.set_polynomial_coef(Integer::from(0), a);
        eq_poly.set_polynomial_coef(prime_candidate.clone(), Integer::from(1));
        let mut mod_poly = Polynomial::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            r_true.clone(),
        ); // X^r - 1
        mod_poly.set_polynomial_coef(Integer::from(0), Integer::from(-1));
        mod_poly.set_polynomial_coef(r_true.clone(), Integer::from(1)); // X^r -1
        if !(polynomial
            .clone()
            .polynomial_remainder(&eq_poly, &mod_poly, prime_candidate.clone()))
        {
            return false;
        }
    }
    return true;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial {
    coef: HashMap<Integer, HashMap<Integer, Integer>>,
    deg: Integer,
}

impl Polynomial {
    pub fn initialize_polynomial(
        coef: HashMap<Integer, HashMap<Integer, Integer>>,
        deg: Integer,
    ) -> Polynomial {
        Polynomial { coef, deg }
    }

    pub fn polynomial_clone(&self) -> Polynomial {
        Polynomial {
            coef: self.coef.clone(),
            deg: self.deg.clone(),
        }
    }
    // pub fn is_equal_polynomial(&self, poly_2: &Polynomial) -> bool {
    //     println!("Figuring out if polynomials are equal");
    //     if self.deg != poly_2.deg {
    //         println!("uh oh: {}, {}", self.deg, poly_2.deg);
    //         return false;
    //     }
    //     for i in range_inclusive(Integer::from(0), self.deg.clone()) {
    //         if self.get_polynomial_coef(i.clone()) != poly_2.get_polynomial_coef(i.clone()) {
    //             println!(
    //                 "bang: {}, {}",
    //                 self.get_polynomial_coef(i.clone()),
    //                 poly_2.get_polynomial_coef(i.clone())
    //             );
    //             return false;
    //         }
    //         println!("the polynomials' values at key {} are equal", &i);
    //     }
    //     return true;
    // }

    pub fn set_polynomial_coef(&mut self, order: Integer, coefficient: Integer) {
        if order <= Integer::from(&self.deg) {
            let a: &mut HashMap<Integer, Integer> = self
                .coef
                .get_mut(&(order.clone() / Integer::from(2).pow(64) + 1))
                .unwrap();
            a.insert(order.clone(), coefficient);
            return;
        } else {
            for index in range(Integer::from(self.deg.clone()), order.clone()) {
                if index > Integer::from(2).pow(64) * (self.coef.len() + 1) {
                    self.coef
                        .insert((self.coef.len() + 1).into(), HashMap::new());
                }
                let a: &mut HashMap<Integer, Integer> = self
                    .coef
                    .get_mut(&(index.clone() / Integer::from(2).pow(64) + 1))
                    .unwrap();
                a.insert(index, Integer::from(0));
            }
            let a: &mut HashMap<Integer, Integer> = self
                .coef
                .get_mut(&(order.clone() / Integer::from(2).pow(64) + 1))
                .unwrap();
            a.insert(order.clone(), coefficient);
            self.deg = order;
        }
    }
    pub fn get_polynomial_coef(&self, order: Integer) -> Integer {
        if order > self.deg {
            return Integer::from(0);
        }
        let inner_hash = self
            .coef
            .get(&Integer::from(&(&order / Integer::from(2).pow(64)) + 1))
            .unwrap();
        if inner_hash.contains_key(&order.clone()) {
            return inner_hash.get(&order).unwrap().clone();
        }
        return Integer::from(0);
    }

    pub fn polynomial_modular_multiplication(
        &self,
        poly_2: &Polynomial,
        mod_n: Integer,
    ) -> Polynomial {
        let max_deg = self.deg.clone() + poly_2.deg.clone();
        let mut poly_res = Self::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            max_deg.clone(),
        );
        for i in range_inclusive(
            Integer::from(1),
            Integer::from(((max_deg.clone() + 1) / (Integer::from(2).pow(64))) + 1),
        ) {
            poly_res.coef.insert(i, HashMap::new());
        }
        for i in range_inclusive(Integer::from(0), self.deg.clone()) {
            for j in range_inclusive(Integer::from(0), poly_2.deg.clone()) {
                let mut coef = self.get_polynomial_coef(i.clone())
                    * poly_2.get_polynomial_coef(j.clone())
                    + poly_res.get_polynomial_coef(i.clone() + j.clone());
                coef = coef % mod_n.clone();
                poly_res.set_polynomial_coef(i.clone() + j, coef);
            }
        }
        return poly_res;
    }
    pub fn polynomial_modular_power(&self, exp: Integer) -> Polynomial {
        let mut poly_res = Self::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            Integer::from(0),
        );
        poly_res.set_polynomial_coef(Integer::from(0), Integer::from(1));
        for i in range_inclusive(exp.significant_bits() + 1, 1).rev() {
            poly_res = self.polynomial_modular_multiplication(&poly_res, exp.clone());
            if exp.get_bit(i - 1) {
                poly_res.polynomial_modular_multiplication(&self, exp.clone());
            }
        }
        return poly_res;
    }
    pub fn polynomial_remainder(
        &mut self,
        poly_2: &Polynomial,
        mod_poly: &Polynomial,
        mod_n: Integer,
    ) -> bool {
        for i in range_inclusive(Integer::from(0), Integer::from(self.deg.clone())) {
            self.set_polynomial_coef(
                i.clone(),
                self.get_polynomial_coef(i.clone()) - poly_2.get_polynomial_coef(i.clone()),
            );
        }
        loop {
            for i in range_inclusive(Integer::from(1), self.deg.clone()).rev() {
                if self.get_polynomial_coef(i.clone()) == Integer::from(0) {
                    self.deg = self.deg.clone() - 1;
                } else {
                    break;
                }
            }
            let mut multiple_poly_degree = Integer::from(-1);
            if self.deg < mod_poly.deg {
                break;
            }
            for i in
                range_inclusive(self.deg.clone() - mod_poly.deg.clone(), self.deg.clone()).rev()
            {
                if multiple_poly_degree == !Integer::from(-1) {
                    break;
                }
                if self.get_polynomial_coef(i.clone())
                    > mod_poly.get_polynomial_coef(mod_poly.deg.clone())
                {
                    multiple_poly_degree = i - mod_poly.deg.clone();
                    break;
                } else if self.get_polynomial_coef(i.clone())
                    == mod_poly.get_polynomial_coef(mod_poly.deg.clone())
                {
                    for j in range_inclusive(self.deg.clone() - mod_poly.deg.clone(), i.clone() - 1)
                        .rev()
                    {
                        if self.get_polynomial_coef(j.clone())
                            > poly_2.get_polynomial_coef(mod_poly.deg.clone() - j.clone())
                        {
                            multiple_poly_degree = j - mod_poly.deg.clone();
                            break;
                        }
                    }
                }
            }
            if multiple_poly_degree <= Integer::from(-1) {
                break;
            }
            let mut multiple_poly = Self::initialize_polynomial(
                HashMap::from([(Integer::from(1), HashMap::new())]),
                multiple_poly_degree.clone(),
            );

            multiple_poly.set_polynomial_coef(
                multiple_poly_degree,
                self.get_polynomial_coef(self.deg.clone())
                    / mod_poly.get_polynomial_coef(mod_poly.deg.clone()),
            );
            let mut long_division_poly = mod_poly.polynomial_multiplication(&multiple_poly);
            for i in range_inclusive(
                Integer::from(long_division_poly.deg.clone()),
                Integer::from(0),
            )
            .rev()
            {
                if long_division_poly.get_polynomial_coef(long_division_poly.deg.clone())
                    == Integer::from(0)
                {
                    long_division_poly.deg = long_division_poly.deg.clone() - 1;
                } else {
                    break;
                }
            }
            if long_division_poly.deg < Integer::from(0) {
                break;
            }
            for i in range_inclusive(Integer::from(0), Integer::from(self.deg.clone())) {
                self.set_polynomial_coef(
                    i.clone(),
                    self.get_polynomial_coef(i.clone())
                        - long_division_poly.get_polynomial_coef(i.clone()),
                );
            }
            for i in range_inclusive(Integer::from(1), self.deg.clone()).rev() {
                if self.get_polynomial_coef(i.clone()) == Integer::from(0) {
                    self.deg = self.deg.clone() - 1;
                } else {
                    break;
                }
            }
        }
        for i in range_inclusive(Integer::from(0), self.deg.clone()) {
            if !(self.get_polynomial_coef(i.clone()).is_divisible(&mod_n)) {
                return false;
            }
        }
        return true;
    }
    pub fn polynomial_multiplication(&self, poly_2: &Polynomial) -> Polynomial {
        let max_deg = self.deg.clone() + poly_2.deg.clone();
        let mut poly_res = Self::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            max_deg.clone(),
        );
        for i in range_inclusive(
            Integer::from(1),
            Integer::from(((max_deg.clone() + 1) / (Integer::from(2).pow(64))) + 1),
        ) {
            poly_res.coef.insert(i, HashMap::new());
        }
        for i in range_inclusive(Integer::from(0), self.deg.clone()) {
            for j in range_inclusive(Integer::from(0), poly_2.deg.clone()) {
                let coef = self.get_polynomial_coef(i.clone())
                    * poly_2.get_polynomial_coef(j.clone())
                    + poly_res.get_polynomial_coef(i.clone() + j.clone());
                poly_res.set_polynomial_coef(i.clone() + j, coef);
            }
        }
        return poly_res;
    }
}
