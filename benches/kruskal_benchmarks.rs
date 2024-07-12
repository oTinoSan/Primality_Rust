use divan::black_box;
use primality::wheel_algos::*;
use rug::Integer;

const THREAD_ARGS: [u64; 3] = [32, 64, 128];

fn main() {
    divan::main();
}

#[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
fn solovay_strassen_bench(num_threads: u64){
    black_box( solovay_strassen(10, Integer::from(100000),black_box(num_threads)));  
}

#[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
fn miller_rabin_bench(num_threads: u64){
    black_box( miller_rabin(10, Integer::from(100000),black_box(num_threads)));  
}

#[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
fn solovay_strassen_general_bench(num_threads: u64) {
    black_box( solovay_strassen_general(10, Integer::from(100000),black_box(num_threads)));
}

#[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
fn miller_rabin_general_bench(num_threads: u64) {
    black_box( miller_rabin_general(10, Integer::from(100000),black_box(num_threads)));
}