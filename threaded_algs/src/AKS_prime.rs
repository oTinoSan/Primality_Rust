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
    let r = Integer::from(0);
    //find smallest r such that the multiplicative order of prime modulo r is greater than (log2n)^2
    for mut r in num_iter::range(Integer::from(0), prime_candidate.clone()) {
        r = Integer::from(r);
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
    if prime_candidate.clone() - Integer::from(0) < r.clone() {
        for a in num_iter::range(Integer::from(2), prime_candidate.clone()) {
            if prime_candidate.clone() % a == 0 {
                return false;
            }
        }
    } else {
        for a in num_iter::range_inclusive(Integer::from(2), r.clone()) {
            if prime_candidate.clone() % a == 0 {
                return false;
            }
        }
    }
    // if n =< r output prime
    if prime_candidate <= r {
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
    for a in num_iter::range(Integer::from(1), roof.0) {
        let mut polynomial: Polynomial = Polynomial::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            Integer::from(1),
        );
        polynomial.set_polynomial_coef(Integer::from(0), a.clone());
        polynomial.set_polynomial_coef(Integer::from(1), Integer::from(1)); // X + a
        let p = polynomial.polynomial_clone();
        for _ in num_iter::range_inclusive(Integer::from(1), prime_candidate.clone()) {
            polynomial.polynomial_modular_multiplication(&p, prime_candidate.clone());
            // (X + a)^n modular
        }
        let mut poly_no_mod = p.polynomial_clone();
        for _ in num_iter::range_inclusive(Integer::from(1), prime_candidate.clone()) {
            poly_no_mod.polynomial_multiplication(&p); // (X + a)^n not modular
        }
        let mut eq_poly = Polynomial::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            prime_candidate.clone(),
        ); // X^n + a
        eq_poly.set_polynomial_coef(Integer::from(0), a);
        eq_poly.set_polynomial_coef(prime_candidate.clone(), Integer::from(1));
        let mut mod_poly = Polynomial::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            r.clone(),
        ); // X^r - 1
        mod_poly.set_polynomial_coef(Integer::from(1), Integer::from(-1));
        mod_poly.set_polynomial_coef(r.clone() + 1, Integer::from(1));
        let mut n_poly = Polynomial::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            Integer::from(0),
        );
        n_poly.set_polynomial_coef(Integer::from(0), Integer::from(prime_candidate.clone()));
        println!("Calculating... ");
        // if (X+a)^n ≠ X^n+a (mod X^r − 1,n), then output composite
        if !((poly_no_mod.polynomial_remainder(&mod_poly))
            .is_equal_polynomial(&eq_poly.polynomial_remainder(&mod_poly)))
            || !(polynomial.is_equal_polynomial(&eq_poly.polynomial_remainder(&n_poly)))
        {
            return false;
        }
    }

    return true;
}

#[derive(Clone, Debug)]
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
    pub fn is_equal_polynomial(&self, poly_2: &Polynomial) -> bool {
        println!("Figuring out if polynomials are equal");
        if self.deg != poly_2.deg {
            return false;
        }
        for i in range_inclusive(Integer::from(0), self.deg.clone()) {
            if self.get_polynomial_coef(i.clone()) != poly_2.get_polynomial_coef(i.clone()) {
                return false;
            }
            println!("the polynomials' values at key {} are equal", &i);
        }
        return true;
    }

    pub fn set_polynomial_coef(&mut self, order: Integer, coefficient: Integer) {
        if order <= Integer::from(&self.deg + 1) {
            let mut a = self.get_polynomial_coef(order.clone());
            a = coefficient;
            return;
        } else {
            for index in range(Integer::from(self.deg.clone() + 1), order.clone()) {
                if index > 2 ^ 64 * (self.coef.len() + 1) {
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
        let max_deg = self.deg.clone() * poly_2.deg.clone();
        let mut poly_res = Self::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            max_deg.clone(),
        );
        for i in range(Integer::from(0), max_deg) {
            let mut coef = Integer::from(0);
            let mut c0;
            let mut c1;
            let mut jmin = if i > poly_2.deg {
                Integer::from(i.clone() - poly_2.deg.clone())
            } else {
                Integer::from(0)
            };
            let mut jmax = if i < self.deg {
                Integer::from(i.clone())
            } else {
                self.deg.clone()
            };

            for j in range_inclusive(jmin, jmax) {
                c0 = self.get_polynomial_coef(j.clone());
                c1 = poly_2.get_polynomial_coef(i.clone() - j.clone());
                let c0 = Integer::from(c0 * c1);
                coef = c0 + coef;
            }
            jmin = Integer::from(i.clone() - poly_2.deg.clone());
            jmax = self.deg.clone();
            for j in range_inclusive(jmin, jmax) {
                c0 = self.get_polynomial_coef(j.clone());
                c1 = poly_2.get_polynomial_coef(i.clone() - j.clone());
                c0 = Integer::from(c0 * c1);
                coef = c0 + coef;
            }
            coef = coef % mod_n.clone();
            poly_res.set_polynomial_coef(i, coef);
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
                poly_res = poly_res.polynomial_modular_multiplication(&self, exp.clone());
            }
        }
        return poly_res;
    }
    pub fn polynomial_remainder(&mut self, poly_2: &Polynomial) -> &Polynomial {
        println!("Entering loop...");
        loop {
            if poly_2.deg >= self.deg {
                return self;
            }
            let mut multiple_poly = Self::initialize_polynomial(
                HashMap::from([(Integer::from(1), HashMap::new())]),
                self.deg.clone() - poly_2.deg.clone(),
            );
            multiple_poly
                .set_polynomial_coef(self.deg.clone() - poly_2.deg.clone(), Integer::from(1));
            println!("multiple_poly: {:?}", &multiple_poly);
            let long_division_poly = poly_2.polynomial_multiplication(&multiple_poly);
            println!("long_division_poly: {:?}", &long_division_poly);
            for i in range_inclusive(
                Integer::from(0),
                Integer::from(self.coef.len() * Integer::from(2).pow(64) - 1),
            ) {
                self.set_polynomial_coef(
                    i.clone(),
                    self.get_polynomial_coef(i.clone())
                        - long_division_poly.get_polynomial_coef(i.clone()),
                );
            }
            for i in range_inclusive(
                Integer::from(self.coef.len() * Integer::from(2).pow(64)),
                Integer::from(1),
            )
            .rev()
            {
                if self.get_polynomial_coef(i.clone()) == Integer::from(0) {
                    self.deg = self.deg.clone() - 1;
                } else {
                    break;
                }
            }
        }
    }
    pub fn polynomial_multiplication(&self, poly_2: &Polynomial) -> Polynomial {
        let max_deg = self.deg.clone() * poly_2.deg.clone();
        let mut poly_res = Self::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            max_deg.clone(),
        );
        for i in range(
            Integer::from(1),
            Integer::from(max_deg.clone() + 1 / Integer::from(2).pow(64) + 1),
        ) {
            poly_res.coef.insert(i, HashMap::new());
        }
        for i in range(Integer::from(0), max_deg) {
            let mut coef = Integer::from(0);
            let mut c0;
            let mut c1;
            let mut jmin = if i > poly_2.deg {
                i.clone() - poly_2.deg.clone()
            } else {
                Integer::from(0)
            };
            let mut jmax = if i < self.deg {
                i.clone()
            } else {
                self.deg.clone()
            };

            for j in range_inclusive(jmin, jmax) {
                c0 = self.get_polynomial_coef(j.clone());
                c1 = poly_2.get_polynomial_coef(i.clone() - j.clone());
                let c0 = Integer::from(c0 * c1);
                coef = c0 + coef;
            }
            jmin = i.clone() - poly_2.deg.clone();
            jmax = self.deg.clone();
            for j in range_inclusive(jmin, jmax) {
                c0 = self.get_polynomial_coef(j.clone());
                c1 = self.get_polynomial_coef(i.clone() - j.clone());
                c0 = Integer::from(c0 * c1);
                coef = c0 + coef;
            }
            poly_res.set_polynomial_coef(i, Integer::from(coef));
        }
        return poly_res;
    }
}
