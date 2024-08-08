use std::{collections::HashMap, sync::{mpsc::{channel, Sender}, Arc, RwLock}, thread::spawn};

use chashmap::CHashMap;
use num::Integer;
use num_prime::nt_funcs::factorize64;

use crate::{calculate_order_prime, calculate_order_prime_exp};


pub fn threaded_generate_table(lower_limit:u64, upper_limit:u64, num_threads: u64) -> HashMap<u64, u64> {
    let block_size = (upper_limit - lower_limit) / num_threads;

    let mut thread_handles = Vec::new();
    let dynamic_array = Arc::new(RwLock::new(HashMap::new()));
    for i in 0..num_threads {
        let mut thread_min = i * block_size + lower_limit;
        let mut thread_max = (i + 1) * block_size + lower_limit;

        if thread_min % 2 == 0 {
            thread_min += 1;
        }
        if i == num_threads -1 {
            thread_max = upper_limit;
        }
        let mut arr_clone = dynamic_array.clone();
        
        let thread = std::thread::spawn(move || {
            let mut return_hashmap = HashMap::new();

            while thread_min < thread_max {
                return_hashmap.insert(thread_min, calculate_order_composite(thread_min, &mut arr_clone));
                thread_min += 2;
            }
            return_hashmap
        });
        thread_handles.push(thread);
    }

    let mut return_hashmap = HashMap::new();
    // join all of the threads
    for handle in thread_handles {
        return_hashmap.extend(handle.join().unwrap());
    }
    return_hashmap
}

pub fn threaded_table_chashmap(max: u64, num_threads: u64) -> Vec<(u64, u64)> {
    let map = Arc::new(CHashMap::new());
    let mut handles = vec![];
    let (tx, rx) = channel();
    let map_writer = map.clone();
    let writer_handle = spawn(move || {
        for (k, v) in rx {
            map_writer.insert(k, v);
        }
    });
    for i in 0..num_threads {
        let map = map.clone();
        let tx = tx.clone();
        let handle = spawn(move || {
            let mut r = vec![];
            for i in ((2 * i + 3) .. max).step_by(2).step_by(num_threads as usize) {
                let rho = composite_order_2_chashmap(i, tx.clone(), map.clone());
                r.push((i, rho));
            }
            r
        });
        handles.push(handle);
    }

    drop(tx);

    writer_handle.join().unwrap();

    // Arc::into_inner(map).unwrap().into_iter().collect::<Vec<_>>();
    handles.into_iter().map(|handle| handle.join().unwrap()).flatten().collect()
}

pub fn correctness_check(rho_table: &Vec<u64>) { 

    for i in 0..rho_table.len() {
        if !(mod_exp::mod_exp(2, rho_table[i],((i+1)*2+1) as u64) == 1) {
            panic!("n: {}, rho: {}, 2^rho mod n: {}", (i+1)*2+1, rho_table[i], mod_exp::mod_exp(2, rho_table[i],((i+1)*2+1) as u64));
        }
    }

}

pub fn composite_order_2_chashmap(n: u64, update_channel: Sender<(u64, u64)>, table: Arc<CHashMap<u64, u64>>) -> u64 {
    let mut orders = vec![];

    for (prime, exp) in factorize64(n) {
        let rho = match table.get(&prime) {
            Some(rho) => {
                *rho
            },
            None => {
                let rho = calculate_order_prime(prime);
                update_channel.send((prime, rho)).unwrap();
                rho
            },
        };
        orders.push(calculate_order_prime_exp(prime, exp as u32, rho));
    }

    orders.iter().fold(1, |acc, x| acc.lcm(x))
}


pub fn calculate_order_composite (n: u64, dynamic_array: &mut Arc<RwLock<HashMap<u64, u64>>>) -> u64 {
    
    let mut mofs = Vec::new();

    for (prime, exponent) in factorize64(n) {
        let arr_result = {dynamic_array.read().unwrap().get(&prime).cloned()};
        let rho = match arr_result {
            Some(rho) => {
                rho
            },
            None => {
                let rho_value = calculate_order_prime(prime);
                dynamic_array.write().unwrap().insert(prime, rho_value);
                rho_value
            },
        };
        if exponent == 1 {
            mofs.push(rho);
        } else {
            mofs.push(calculate_order_prime_exp(prime, exponent as u32, rho))
        }
        
    }

    return mofs.iter().fold(1, |acc, x| acc.lcm(&x));
}