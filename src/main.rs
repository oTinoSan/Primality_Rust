use primality::*;
use num_bigint::BigUint;
use num_traits::FromPrimitive;
fn main() {
    // let result = trial_vect(1000);
    // println!("{:?}", result);
    // let result = trial(1000);
    // println!("{:?}", result);

    // let result = trials::trial_vect_2(1000);
    // println!("{:?}", result);
    // let result = trials::trial_2(1000);
    // println!("{:?}", result);

    // let result = sieve::sieve(1000);
    // println!("{:?}", result);
    // let result = seive::sieve_vect(1000);
    // println!("{:?}", result);

    // let result = wheel::wheel_facts(1000);
    // println!("{:?}", result);
    // let result = wheel::wheel(1000);
    // println!("{:?}", result);

    // let result = miller_rabin::miller_rabin(3, 5);
    // println!("{:?}", result);
    // let result = solovay_strassen::solovay_strassen(479001599);
    // println!("{:?}", result);
    
    let n = BigUint::from_u32(3*324).unwrap();
    let result = bigint_num::miller_rabin_big::miller_rabin_bigint(n, 5);
    println!("{}", result);

    // let n = Integer::from(3*324);
    // let result = miller_rabin_rug::miller_rabin_bigrug(n, 5);
    // println!("{}", result);


}

