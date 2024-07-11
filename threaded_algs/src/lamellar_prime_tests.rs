use lamellar::array::prelude::*;
use rug::{Integer, rand};
use crate::threaded_solovay::bigint_solovay_strassen;

pub fn lamellar_wheel_miller(){
    let world = lamellar::LamellarWorldBuilder::new().build();
    let my_pe = world.my_pe();
    let num_pes = world.num_pes();

    let start = std::time::Instant::now();

    let results = AtomicArray::<u64>::new(&world, num_pes, Distribution::Block);
    let limit = "10000000000";
    let search_max = limit.parse::<Integer>().unwrap();

    let step = Integer::from(&search_max / num_pes);

    let local_min = Integer::from(&step * my_pe);
    let mut local_max = Integer::from(&step * (my_pe + 1));
    if my_pe == num_pes - 1 {
        local_max = search_max.clone();
    }

    let local_results = super::wheel_algos::general_wheel_threaded(
        10,
        local_min,
        local_max,
        128,
        bigint_miller_rabin,
        vec![2, 3, 5],
        vec![1, 7, 11, 13, 17, 19, 23, 29],
    );
    results
        .mut_local_data()
        .at(0)
        .store(local_results.len() as u64);

    world.barrier();

    if my_pe == 0 {
        let sum = world.block_on(results.sum());
        let elapsed = start.elapsed();
        println!("Found {} primes under {}", sum, &search_max);
        println!("Time elapsed: {:?}", elapsed);
    }

}

pub fn lamellar_wheel_solovay(){
    let world = lamellar::LamellarWorldBuilder::new().build();
    let my_pe = world.my_pe();
    let num_pes = world.num_pes();

    let start = std::time::Instant::now();

    let results = AtomicArray::<u64>::new(&world, num_pes, Distribution::Block);
    let limit = "10000000000";
    let search_max = limit.parse::<Integer>().unwrap();

    let step = Integer::from(&search_max / num_pes);

    let local_min = Integer::from(&step * my_pe);
    let mut local_max = Integer::from(&step * (my_pe + 1));
    if my_pe == num_pes - 1 {
        local_max = search_max.clone();
    }

    let local_results = super::wheel_algos::general_wheel_threaded(
        10,
        local_min,
        local_max,
        128,
        bigint_solovay_strassen,
        vec![2, 3, 5, 7],
        vec![1, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 107, 109, 113, 127, 131, 137, 139, 149 ],
    );
    results
        .mut_local_data()
        .at(0)
        .store(local_results.len() as u64);

    world.barrier();

    if my_pe == 0 {
        let sum = world.block_on(results.sum());
        let elapsed = start.elapsed();
        println!("Found {} primes under {}", sum, &search_max);
        println!("Time elapsed: {:?}", elapsed);
    }

}

pub fn bigint_miller_rabin(loop_amount: u64, n: Integer) -> bool {
    let mut rand = rand::RandState::new();
    let minus_one = Integer::from(&n - 1);
    let s = minus_one.find_one(0).unwrap();
    let d = Integer::from(&minus_one >> s);
    'outer: for _ in 0..loop_amount {
        let mut a =
            Integer::from(Integer::from(Integer::from(&n - 3).random_below_ref(&mut rand)) + 1);
        a = a.pow_mod(&d, &n).unwrap();
        if a == 1 {
            continue;
        }
        for _ in 0..s {
            if a == Integer::from(&n - 1) {
                continue 'outer;
            }
            a = a.pow_mod(&Integer::from(2), &n).unwrap();
        }
        if a != minus_one {
            return false;
        }
    }
    true
}