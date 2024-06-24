use primality::*;

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

    let result = miller_rabin::miller_rabin(1000);
    println!("{:?}", result);

    // let result = solovay_strassen::solovay_strassen(479001599);
    // println!("{:?}", result);
}
