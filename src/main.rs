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


   // 2, 3, 5, 7, 11, 13, 17, 19, 23, 29
    let result = miller_rabin::miller_rabin(3, 5);
    println!("{:?}", result);

    // let result = solovay_strassen::solovay_strassen(479001599);
    // println!("{:?}", result);
}
