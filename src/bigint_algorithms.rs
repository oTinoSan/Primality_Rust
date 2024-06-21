use rug::{Integer, rand};

pub fn solovay_strassen(num_tests: u64, candidate: Integer) -> bool {
    let mut rand = rand::RandState::new();
    for _ in 0..num_tests {
        let test_base = Integer::from(candidate.random_below_ref(&mut rand));
        let jacobi = test_base.jacobi(&candidate);
        let powmod_result = test_base.pow_mod(&(Integer::from(&candidate - 1) / 2), &candidate).unwrap();
        if !((powmod_result == jacobi) || (powmod_result == Integer::from(&candidate - 1) && jacobi == -1)) {
            return false;
        }
    }
    return true;
}