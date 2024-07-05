use num::BigUint;
use num::FromPrimitive;
// no --- use rand::RandBigInt;
use num::bigint::RandBigInt;
// use num_format::ToFormattedString;
// use num_format::Locale;

// cargo test t_mr_max --release --nocapture

#[test]
pub fn t_shift() {
    //let c = 52;
    //let c = 11;
    let c = 63;
    println!("{:b}", c);

    for _i in 0..64 {
        //println!("{:b}, {:b}", (i as u8 + c) % 64, (i as u8 + 1 - c) % 64);
        //println!("{}, {}", (i as u8 + c) % 64, (i as u8 + 1 - c) % 64);
    }

    println!("");
    //for x in 28..35 {
    let mut check: [usize; 64] = [0; 64];
    for x in 0..64 {
        if x < 32 {
            //println!("{:b}, {:b}", x, (2*x + c) % 64);
            println!("{}, {}", x, (2 * x + c) % 64);
            check[x] = 1;
        } else {
            //println!("{:b}, {:b}", x, (2*x + 1 - c) % 64);
            println!("{}, {}", x, (2 * x + 1 - c) % 64);
            check[x] = 1;
        }
    }

    //println!("check? {:?}", check);
    for i in 0..63 {
        if check[i] != 1 {
            assert!(false);
        }
    }
}

#[test]
pub fn t_trial_div_32761() {
    basic_trial_division_determination(32_761);
}

#[test]
fn t_mr_32761() {
    let ans = miller_rabin(32_761);
    println!("is prime? {}", ans);
}

#[test]
fn t_find_k_m() {
    let (k, m) = find_k_m(32);
    assert_eq!((k, m), (5, 1));

    let (k, m) = find_k_m(14);
    assert_eq!((k, m), (1, 7));

    let (k, m) = find_k_m(96);
    assert_eq!((k, m), (5, 3));

    let (k, m) = find_k_m(1000);
    assert_eq!((k, m), (3, 125));
}

#[test]
fn t_trial_div_lt_100() {
    for i in 3..=100 {
        let ans = basic_trial_division_determination(i);
        if ans {
            println!("{} is prime", i);
        }
    }
}

#[test]
fn t_mr_lt_100() {
    for i in 3..=100 {
        let ans = miller_rabin(i);
        if ans {
            println!("{} is prime", i);
        }
    }
}
/*

   running 1 test
checked to 10 and see 4 so far in 177ns seconds.
checked to 100 and see 25 so far in 105.847µs seconds.
checked to 1000 and see 168 so far in 128.783µs seconds.
checked to 10000 and see 1229 so far in 427.544µs seconds.
checked to 100000 and see 9592 so far in 6.682882ms seconds.
checked to 1000000 and see 78498 so far in 146.247152ms seconds.
checked to 10000000 and see 664579 so far in 3.616690894s seconds.
test t_trial_div_max has been running for over 60 seconds
checked to 100000000 and see 5761455 so far in 95.090531983s seconds.


*/

#[test]
fn t_trial_div_max() {
    let mut ten_multiple = 10;
    //let mut count = 1; //start at 1 bc 2 is already counted; we're starting at 3.
    let mut count = 2; //start at 2 bc 2 (and3?)???

    use std::time::Instant;
    let now = Instant::now();

    //   let mut i = 3;
    //    while i < u32::MAX {
    //    for mut i in 3..=u32::MAX {
    for i in 3..=u32::MAX {
        //for mut i in (3..=u32::MAX).step_by(2) {
        //    for i in (3..=u32::MAX).step_by(2) {

        let ans = basic_trial_division_determination(i as u64);
        if ans {
            //println!("{} is prime", i);
            count += 1;
        }
        if i % ten_multiple == 1 {
            let elapsed = now.elapsed();
            println!(
                "checked to {} and see {} so far in {:02?}.",
                i, count, elapsed
            );
            ten_multiple *= 10;
        }
        //i+=2;
    }
    println!(
        "by trial division, i think there are {} primes under {}",
        count,
        u32::MAX
    );
}

#[test]
fn t_mr_max() {
    let mut ten_multiple = 10;
    //let mut count = 1; //start at 1 bc 2 is already counted; we're starting at 3.
    let mut count = 2; //start at 2 bc 2 (and3?)???

    use std::time::Instant;
    let now = Instant::now();

    for i in 5..=u32::MAX {
        let ans = miller_rabin(i as u64);
        if ans {
            //println!("{} is prime", i);
            count += 1;
        }
        if i % ten_multiple == 1 {
            let elapsed = now.elapsed();
            println!(
                "checked to {} and see {} so far in {:02?}.",
                i, count, elapsed
            );
            ten_multiple *= 10;
        }
        //i+=2;
    }
    println!(
        "by miller rabin , i think there are {} primes under {}",
        count,
        u32::MAX
    );
}

// for some sense - see what this generates and test results against wolfram. see
//https://www.wolframalpha.com/input?i=Is+10001+prime%3F

#[test]
pub fn t_largeprimetest_smallinterval() {
    let mut count = 0;
    //let start = BigUint::new(vec![1,1,1,1,1]);
    //let start = BigUint::new(vec![1,1,1,1,1,1,1]);
    //let start = BigUint::new(vec![1,1,1,1,1,1,1,1,1]);
    //let start = BigUint::new(vec![1,1,1,1,1,1,1,1,1,1,1]);
    //let start = BigUint::new(vec![1,1,1,1,1,1,1,1,1,1,1,1]);
    let start = BigUint::new(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    //let start = BigUint::new(vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]);
    //let end = BigUint::new(vec![1,1,1,1,100]);

    println!("big number?\t{}", start);
    //println!("big number? {}", end);

    //let end = 100_000;
    let end = 1_000;
    for input in 0..end {
        //println!("input is: {}", input);
        //let is_prime = b_miller_rabin(BigUint::from_u64(input).unwrap());
        let is_prime = b_miller_rabin(start.clone() + input as u32);
        if is_prime {
            count += 1;
            //println!("\t{}", start.clone() + input as u32 );
        }
    }
    println!("end of range:\t{}", start + end as u32);
    println!("number of primes in range: {}", count);
}

// this is helpful roughly for correctness and for speed.
// i.e., we know there are 78,498 prime numbers less than a million. we're using a probablistic test. it should be close.
#[test]
pub fn t_smallprimetest() {
    let mut count = 0;

    for input in 5..1_000_000 {
        //println!("input is: {}", input);
        let is_prime = b_miller_rabin(BigUint::from_u64(input).unwrap());
        if is_prime {
            count += 1;
        }
    }
    println!("number of primes in range: {}", count);
}

#[test]
pub fn t_small() {
    miller_rabin(2_u64.pow(20) * 5);
    b_miller_rabin(BigUint::from_u64(2_u64.pow(13) * 5).unwrap());
    miller_rabin(221);
    b_miller_rabin(BigUint::from_u64(221).unwrap());
    miller_rabin(32 * 97 + 1);
    b_miller_rabin(BigUint::from_u64(32 * 97 + 1).unwrap());
    miller_rabin(2048 * 97 + 1);
    b_miller_rabin(BigUint::from_u64(2048 * 97 + 1).unwrap());
}

// x^a % n
pub fn modular_exponentiation(mut x: u64, mut a: u64, n: u64) -> u64 {
    let mut ans = 1;
    if a <= 0 {
        return 1;
    }
    loop {
        if a == 1 {
            return ans * x % n;
        }
        if a & 1 == 0 {
            x = (x * x) % n;
            a >>= 1;
            continue;
        } else {
            ans = (ans * x) % n;
            a -= 1;
        }
    }
}

pub fn miller_rabin(n: u64) -> bool {
    /*
    if n < 5 {
        if n == 3 {
            return true;
        }
        else if n == 4 {
            return false;
        }
    }
    */

    let (k, m) = find_k_m(n - 1);
    //println!("2^{} * {} = {}", k, m, n-1);
    //println!("{:?}", (k,m));

    //gens random [2,n-2]
    //let a = rand::thread_rng().gen_range(2..n-1);
    let a = 2;

    let mut b = modular_exponentiation(a, m, n);
    if b == 1 {
        return true;
    }
    for _i in 0..k {
        if b == n - 1 {
            return true;
        }
        b = b * b % n;
    }
    return false;
}

fn find_k_m(n: u64) -> (u64, u64) {
    let mut k = 0;
    let mut m = n;

    while m % 2 == 0 {
        k += 1;
        m /= 2;
    }
    (k, m)
}
/*
fn find_k_m(n: u64) -> (u64, u64) {

    let mut c = 0;
    //let s = n; //copy n so i can mut it without labeling the types.
    let mut i = 1;
    let mut s = 1<<i;
    while s < n {
        let cc = n % 1<<i;   //candidate check
        //println!("{}", cc);
        c = n / (1<<i);        //candidate
        //println!("{}", c);
        if cc == 0 && c % 2 == 1 {  //candidate divides n, and the quotient m is odd
            //println!("{}", c);
            return (i, c);
        }
        //prepare for next loop:
        i += 1;
        s = 1<<i;
    }
    c = n / (1<<i);        //candidate
    (i, c)
}
*/

//// big.....

pub fn b_miller_rabin(n: BigUint) -> bool {
    let n_minus_one = &n - &BigUint::from_u64(1).unwrap();
    let x = b_find_k_m(&n - &BigUint::from_u64(1).unwrap());
    //println!("2^{} * {} = {}", x.0, x.1, n-1);
    //println!("x is {:?}", x);

    let big1 = BigUint::from_u64(1).unwrap();
    let mut rng = rand::thread_rng();
    //let a = RandBigInt::gen_biguint_range(&big1, &n_minus_one);
    let a = rng.gen_biguint_range(&big1, &n_minus_one);
    //println!("a is {}", a);

    //println!("a is {} and x.1 is {}", a, x.1);
    //let mut b = a.pow((x.1).try_into().unwrap()) % n.clone();
    let mut b = a.modpow(&x.1, &n.clone());
    //println!("b is {}", b);

    if b == big1 {
        //println!("{} is prime!", n);
        return true;
    }

    let k = x.0;
    for _i in 0..k {
        if b == n_minus_one {
            //println!("{} is prime!", n);
            return true;
        } else {
            b = (b.clone() * b.clone()) % n.clone();
        }
    }

    //println!("{} is composite!", n);
    return false;
}

pub fn b_find_k_m(n: BigUint) -> (u64, BigUint) {
    //let mut c = 0;
    let big0 = BigUint::from_u64(0).unwrap();
    let big1 = BigUint::from_u64(1).unwrap();
    let big2 = BigUint::from_u64(2).unwrap();
    let mut c = big0.clone();
    //let s = n; //copy n so i can mut it without labeling the types.
    //let mut i : BigUint = BigUint::from_u64(1).unwrap();
    let mut i = 1;
    //let mut s = 1<<i;
    let mut s: BigUint = BigUint::from_u64(2).unwrap();

    while s < n {
        let power = 1 << i;
        let mut big_power = BigUint::from_u64(power).unwrap();
        let cc = n.clone() % big_power.clone(); //candidate check
                                                //println!("{}", cc);
                                                //c = n / (1<<i);        //candidate
        c = n.clone() / big_power.clone(); //candidate
                                           //println!("{}", c);
        if cc == big0.clone() && c.clone() % big2.clone() == big1.clone() {
            //candidate divides n, and the quotient m is odd
            //println!("{}", c);
            return (i, c);
        }
        //prepare for next loop:
        i += 1;
        big_power = BigUint::from_u64(1 << i).unwrap();
        s = big_power;
    }
    c = n / BigUint::from_u64(1 << i).unwrap(); //candidate
    if c == big0 {
        c = big1;
    }
    (i, c)
}

//reading pomerance:
// limitation for u64 is can only check for factors up to 2**32 or ~4B.
pub fn basic_trial_division_determination(n: u64) -> bool {
    let mut i = 2;
    if n % i == 0 {
        return false;
    }
    i = 3;
    if n % i == 0 {
        return false;
    }
    i = 5;

    //    let step_tuple = (2, 4);
    let step_arr = [2, 4];
    let mut bit = 1;

    loop {
        let ii = i * i;
        if ii > n {
            return true;
        }
        // this expected to work for all u64s but nothing larger.
        if ii > u32::MAX as u64 {
            return true;
        }

        if n % i == 0 {
            return false;
        }

        //i += 2;
        bit ^= 1;
        //println!("i {} (prove bit alternates): {}", i, bit);

        //       i += step_tuple.bit; //doesn't work
        i += step_arr[bit];
    }
}

#[test]
pub fn basic_sieve() {
    use std::time::Instant;
    let now = Instant::now();

    //let N : [u64; usize::MAX] = [ 1; usize::MAX ];
    //let n : [u64; usize::MAX] = [ 1; 2_usize.pow(64)-1];
    //const max : usize = 2_usize.pow(43); //biggest i can get on my VM.
    const MAX: usize = 2_usize.pow(30); //biggest i can get on my VM.
                                        //    let n : [u64; usize::MAX] = [ 1; 2_usize.pow(64)];
                                        //let mut n : [u64; MAX] = [ 1; MAX ];
    let mut n: Vec<u64> = vec![1; MAX];

    let elapsed = now.elapsed();
    println!("vector creation in {:02?}.", elapsed);

    //println!("{} {}", usize::MAX, u64::MAX);
    // println!("elems in vector: {} ", MAX.to_formatted_string(&Locale::en));
    n[0] = 0;
    n[1] = 0;
    for i in (4..MAX).step_by(2) {
        n[i] = 0;
    }
    for i in (6..MAX).step_by(3) {
        n[i] = 0;
    }
    for i in (10..MAX).step_by(5) {
        n[i] = 0;
    }
    for i in (14..MAX).step_by(7) {
        n[i] = 0;
    }

    //
    let elapsed = now.elapsed();
    println!("mark off multiples of 2,3,5,7 in total {:02?}.", elapsed);

    //let short = 10_000;
    let _short = 100;

    let mut i = 8;

    // sanity check:
    /*
    for i in 0..short {
        print!("{}", n[i]);
    }
    */

    while i < MAX {
        //print!("{}", i);
        //let mut current = n[i];
        if n[i] == 1 {
            let mut j = i + i;
            let k = i; //step
            while j < MAX {
                n[j] = 0;
                j += k;
            }
        }
        i += 1;
    }

    let elapsed = now.elapsed();
    println!(
        "mark off multiples of everything else in total {:02?}.",
        elapsed
    );

    let mut checkpoint = 10;

    let mut _count = 0;
    for i in 0..MAX {
        if n[i] == 1 {
            //            count += 1;
        }
        if i == checkpoint {
            // println!("count at {}: {}", checkpoint.to_formatted_string(&Locale::en), count.to_formatted_string(&Locale::en));
            checkpoint *= 10;
        }
    }
    // println!("count: {}", count.to_formatted_string(&Locale::en));

    /* // sanity
    for i in 0..short {
        if n[i] == 1 {
            println!("{}", i );
        }
    }
    */
}
