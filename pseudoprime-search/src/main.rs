use pseudoprime_search::*;

fn main() {
    let qs = q_search(1000000);
    // println!("{:?}", qs);

    let threaded_qs = q_search_threaded(1000000, 4);

    assert_eq!(qs.keys().into_iter().collect::<Vec<_>>(), threaded_qs.keys().into_iter().collect::<Vec<_>>());
}