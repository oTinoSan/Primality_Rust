use primality::*;

fn main() {

    // let result = trial_vect(1000);
    // println!("{:?}", result);
    // let result = trial(1000);
    // println!("{:?}", result);

    // let result = trial_vect_2(1000);
    // println!("{:?}", result);
    // let result = trial_2(1000);
    // println!("{:?}", result);

    // let result = sieve(1000);
    // println!("{:?}", result);
    // let result = sieve_vect(1000);
    // println!("{:?}", result);

    // let result = wheel_facts(1000);
    // println!("{:?}", result);
    // let result = wheel(1000);
    // println!("{:?}", result);

    // let result = miller_rabin(1000);
    // println!("{:?}", result);

    let result = solovay_strassen(1000);
    println!("{:?}", result);

}