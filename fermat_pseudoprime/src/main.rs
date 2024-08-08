use std::sync::Arc;

use fermat_pseudoprime::*;
use num::integer::Roots;
use num_prime::nt_funcs::factorize128;
use search_e::e_search_threaded;
// use rug::ops::Pow;
use search_s::s_search_threaded;

pub mod search_e;
pub mod search_s;

fn main() {

    
    println!("{:?}", factorize128(121021));

    // let start = std::time::Instant::now();
    
    // let file = std::fs::read("rho_table.bin").unwrap();
    // let rho_table: Arc<Vec<u64>> = Arc::new(bincode::deserialize(&file[..]).unwrap());
    
    // println!("deserialized rho table, took {:?}", start.elapsed());
    
    // let start = std::time::Instant::now();
    // let array = e_search_threaded(3, 10_000_000,2u128.pow(24), rho_table.clone(), 4);
    // println!("e size: {:?}, took: {:?}",array.len(), start.elapsed());
    // // println!("{}", array.get(array.len() - 1).unwrap());


    // let start = std::time::Instant::now();
    // let threaded_qs = s_search_threaded(3, 10_000_000, 2u128.pow(24), 2u128.pow(24).sqrt(), rho_table,  4);


    // println!("s size: {:?}, time: {:?}", threaded_qs.len(), start.elapsed());

}
