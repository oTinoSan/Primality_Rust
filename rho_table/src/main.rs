use rho_table::*;

fn main() {
    let start = std::time::Instant::now();
    let table = threaded_table_chashmap(1000000, 4);
    println!("CHashMap: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let table = threaded_table_evmap(1000000, 4);
    println!("evmap: {:?}", start.elapsed());
}