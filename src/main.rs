use primality_tests::*;
fn main() {
    println!("{:?}", miller_rabin::miller_rabin_list(8, 100000));
}
