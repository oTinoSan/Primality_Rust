use primality_testing::*;
use prime_search::threaded_prime_search;
use rug::Integer;
use std::{fs::OpenOptions, io::{BufWriter, Write}};

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    let start = std::time::Instant::now();
    let lower_bound = Integer::from(0);
    let upper_bound = Integer::from(1_000_000_000i64);

    // let array = miller_rabin_array(int as u32);
    // // let array = solovay_strassen(int);

    // let array = baillie_psw_array(upper_bound);
    // // println!("done");
    // println!("total: {:?}", array);
    // println!("total: {:?}", array.len());

    // let result = baillie_wagstaff_lucas_test(&Integer::from(170141183460469231731687303715884105727i128));
    // println!("{} {}", result.0, result.1);

    let array = threaded_prime_search(lower_bound.clone(), upper_bound.clone(), 128);
    let elapsed = start.elapsed();
    println!("time elapsed: {:?}", elapsed);
    println!("searched from {} to {}, found: \n{} primes, \n{} strong base 2 pseudoprimes, \n{} lucas pseudoprimes, \n{} strong lucas pseudoprimes, \n{} baillie psw pseudoprimes", lower_bound, upper_bound, array.0.len(), array.1.len(), array.2.len(), array.3.len(), array.4.len());
    
    // println!("base 2 pseudoprimes: {:?}", array.1);
    // println!("lucas pseudoprimes: {:?}", array.2);
    // println!("strong lucas pseudoprimes: {:?}", array.3);
    println!("bpsw pseudoprimes: {:?}", array.4);


    let f = OpenOptions::new()
        .append(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open("data/spsp2.txt")
        .expect("Unable to open file");
    let mut stream = BufWriter::new(f);
    for prime in array.1 {
        let string = prime.to_string() + "\n";
        stream.write_all(string.as_bytes()).expect("Unable to write data");   
    }
    stream.flush().unwrap();

    let f = OpenOptions::new()
        .append(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open("data/lpsp.txt")
        .expect("Unable to open file");
    let mut stream = BufWriter::new(f);
    for prime in array.2 {
        let string = prime.to_string() + "\n";
        stream.write_all(string.as_bytes()).expect("Unable to write data");   
    }
    stream.flush().unwrap();

    let f = OpenOptions::new()
        .append(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open("data/slpsp.txt")
        .expect("Unable to open file");
    let mut stream = BufWriter::new(f);
    for prime in array.3{
        let string = prime.to_string() + "\n";
        stream.write_all(string.as_bytes()).expect("Unable to write data");   
    }
    stream.flush().unwrap();

    let f = OpenOptions::new()
        .append(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open("data/bpswsp.txt")
        .expect("Unable to open file");
    let mut stream = BufWriter::new(f);
    for prime in array.4{
        let string = prime.to_string() + "\n";
        stream.write_all(string.as_bytes()).expect("Unable to write data");   
    }
    stream.flush().unwrap();
// */
}
