use rho_table::*;

fn main() {
    // let start = std::time::Instant::now();
    // let table = threaded_table_chashmap(1000000, 4);
    // println!("CHashMap: {:?}", start.elapsed());

    // let start = std::time::Instant::now();
    // let table = threaded_table_evmap(1000000, 4);
    // println!("evmap: {:?}", start.elapsed());

    let mut factors:Vec<_> = sieve_factorize::sieve_factorize(150).into_iter().collect();
    factors.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("{:?}", factors);
}