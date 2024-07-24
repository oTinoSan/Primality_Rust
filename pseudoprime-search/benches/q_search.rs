use pseudoprime_search::{q_search, q_search_threaded};
use divan::black_box;

const MAX_ARGS: [u64; 4] = [10000, 100000, 1000000, 10000000];

fn main() {
    divan::main()
}

#[divan::bench(args=MAX_ARGS, sample_count=1, sample_size=1)]
fn q_search_test(arg: u64) {
    q_search(black_box(arg));
}

#[divan::bench(args=MAX_ARGS, sample_count=1, sample_size=1)]
fn threaded_q_test(arg: u64) {
    q_search_threaded(black_box(arg), black_box(4));
}