use lamellar::array::prelude::*;
use crate::wheel_algos::general_wheel_rayon;
use crate::main::bigint_miller_rabin;
use rug::Integer;

pub fn lamellar(){
    let world = lamellar::LamellarWorldBuilder::new().build();
    let my_pe = world.my_pe();
    let num_pes = world.num_pes();

    let start = std::time::Instant::now();

    let results = AtomicArray::<u64>::new(&world, num_pes, Distribution::Block);

    let search_max = Integer::from(u32::MAX);

    let step = Integer::from(&search_max / num_pes);

    let local_min = Integer::from(&step * my_pe);
    let mut local_max = Integer::from(&step * (my_pe + 1));
    if my_pe == num_pes - 1 {
        local_max = search_max.clone();
    }

    let local_results = super::wheel_algos::general_wheel_rayon(
        10,
        local_min,
        local_max,
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