use std::{sync::{mpsc::{channel, Sender}, Arc}, thread::spawn};

use chashmap::CHashMap;
use evmap::ReadHandle;
use num::Integer;
use num_prime::nt_funcs::factorize64;

pub mod sieve_factorize;

/// find the multiplicative order of 2 mod a prime p
pub fn prime_order_2(p: u64) -> u64 {
    let factors = factorize64(p - 1);

    let mut potential_orders = vec![1];

    // generate the set of all combinations of all powers of the factors of p-1
    for (prime, exp) in factors {
        let mut new_potentials = Vec::new();
        for e in 0..=exp {
            for order in potential_orders.iter() {
                new_potentials.push(*order * prime.pow(e as u32));
            }
        }
        potential_orders = new_potentials;
    }

    potential_orders.sort_unstable();

    for order in potential_orders.iter() {
        // the lowest k in the set satisfying 2^k%p==1 is the order of 2
        if mod_exp::mod_exp(2, *order, p) == 1 {
            return *order;
        }
    }

    potential_orders.pop().unwrap()
}

/// find the multiplicative order of 2 mod p^a where p is prime
pub fn prime_exp_order_rho(p: u64, e: u32, rho: u64) -> u64 {
    // rho is the multiplicative order of 2 mod p^1
    if e == 1 {
        return rho;
    }

    // otherwise, rho(p) | rho(p^a), so we find the value by dumb iteration
    let mut potential = rho;
    let p_e = p.pow(e);
    loop {
        if mod_exp::mod_exp(2, potential, p_e) == 1 {
            return potential;
        }
        potential += rho;
    }
}

/// find the multiplicative order of 2 mod a composite number
pub fn composite_order_2_chashmap(n: u64, update_channel: Sender<(u64, u64)>, table: Arc<CHashMap<u64, u64>>) -> u64 {
    let mut orders = vec![];

    // find the multiplicative order of 2 mod all the individual prime powers of its factorization
    for (prime, exp) in factorize64(n) {
        let rho = match table.get(&prime) {
            Some(rho) => {
                *rho
            },
            None => {
                let rho = prime_order_2(prime);
                update_channel.send((prime, rho)).unwrap();
                rho
            },
        };
        orders.push(prime_exp_order_rho(prime, exp as u32, rho));
    }

    // the multiplicative order of 2 mod a composite n is the lcm of the multiplicative order of 2 mod each of its factors
    orders.iter().fold(1, |acc, x| acc.lcm(x))
}

pub fn composite_order_2_evmap<'rh>(n: u64, update_channel: Sender<(u64, u64)>, table: &ReadHandle<u64, u64>) -> u64 {
    let mut orders = vec![];

    for (prime, exp) in factorize64(n) {
        let rho = match table.get_one(&prime) {
            Some(rho) => {
                *rho
            },
            None => {
                let rho = prime_order_2(prime);
                update_channel.send((prime, rho)).unwrap();
                rho
            },
        };
        orders.push(prime_exp_order_rho(prime, exp as u32, rho));
    }

    orders.iter().fold(1, |acc, x| acc.lcm(x))
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

pub fn threaded_table_evmap(max: u64, num_threads: u64) -> Vec<(u64, u64)> {
    let (map_reader, mut map_writer) = evmap::new();
    let (tx, rx) = channel();
    let writer_handle = spawn(move || {
        let mut update_counter = 0;
        for (k, v) in rx {
            map_writer.update(k, v);
            update_counter += 1;
            if update_counter % 20 == 0 {
                map_writer.flush();
            }
        }
    });
    let mut handles = vec![];
    
    for i in 0..num_threads {
        let map_reader = map_reader.clone();
        let tx = tx.clone();
        let handle = spawn(move || {
            let mut r = vec![];
            for i in ((2 * i + 3)..max).step_by(2).step_by(num_threads as usize) {
                let rho = composite_order_2_evmap(i, tx.clone(), &map_reader);
                r.push((i, rho));
            }
            r
        });
        handles.push(handle);
    }

    drop(tx);

    writer_handle.join().unwrap();

    // let _: Vec<_> = map_reader.map_into(|k, v| (*k, *v.get_one().unwrap()));
    handles.into_iter().map(|handle| handle.join().unwrap()).flatten().collect()
}