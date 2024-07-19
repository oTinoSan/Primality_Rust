use std::collections::HashMap;

use rho_table::*;

fn main() {
    // let start = std::time::Instant::now();
    // let table = threaded_table_chashmap(1000000, 4);
    // println!("CHashMap: {:?}", start.elapsed());

    // let start = std::time::Instant::now();
    // let table = threaded_table_evmap(1000000, 4);
    // println!("evmap: {:?}", start.elapsed());

    let factors = sieve_factorize::sieve_factorize(1000000000);
    let writer = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open("factors.bin").unwrap();
    bincode::serialize_into(writer, &factors).unwrap();

    // let reader = std::fs::OpenOptions::new().read(true).open("factors.bin").unwrap();
    // let factors: Vec<(u64, HashMap<u64, u64>)> = bincode::deserialize_from(reader).unwrap();
    // println!("{:?}", factors);
}