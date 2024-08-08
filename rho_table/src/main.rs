use std::{fs::OpenOptions, io::{BufWriter, Write}, time::Instant};

use rho_table:: *;
use threaded_rho::threaded_table_chashmap;

fn main() {

    let limit = 2u64.pow(24);

    let start = Instant::now();
    let mut array = threaded_table_chashmap(limit, 4);
    println!("time: {:?}, {:?}",start.elapsed(), array.len());

    let start = Instant::now();
    array.sort_unstable();
    println!("done sorting, time: {:?}", start.elapsed());

    let mut rho_table = vec![0; (limit/2) as usize];

    for (odd_number, rho_value) in array {
        rho_table[((odd_number/2)-1) as usize] = rho_value; // index 3, 5, 7, 9,... to 0, 1, 2, 3,...
    }

    // correctness_check(&rho_table);

    let f = OpenOptions::new()
        .write(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .truncate(true)
        .open("rho_table.bin")
        .expect("Unable to open file");
    let mut stream = BufWriter::new(f);
    let serialized = bincode::serialize(&rho_table).unwrap();
    stream.write_all(&serialized).unwrap();
    stream.flush().unwrap();

}
