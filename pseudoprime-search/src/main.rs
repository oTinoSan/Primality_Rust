use pseudoprime_search::*;

fn main() {
    let qs = q_search(3, 1000000000);
    println!("{:?}", qs);

    let threaded_qs = q_search_threaded(1000, 4);
}