use rho_table::{threaded_table_chashmap, threaded_table_evmap};

fn main() {
    divan::main()
}

const THREAD_ARGS: [u64; 6] = [4, 8, 16, 32, 64, 128];

#[divan::bench(sample_count=1, sample_size=1, args=THREAD_ARGS)]
fn chashmap_bench(arg: u64) {
    threaded_table_chashmap(100000000, arg);
}

#[divan::bench(sample_count=1, sample_size=1, args=THREAD_ARGS)]
fn evmap_bench(arg: u64) {
    threaded_table_evmap(100000000, arg);
}