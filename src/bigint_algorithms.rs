use rug::{integer::MiniInteger, rand, Integer};

pub fn solovay_strassen(num_tests: u64, candidate: Integer) -> bool {
    let mut rand = rand::RandState::new();
    for _ in 0..num_tests {
        let test_base = Integer::from(candidate.random_below_ref(&mut rand));
        let jacobi = test_base.jacobi(&candidate);
        let powmod_result = test_base
            .pow_mod(&(Integer::from(&candidate - 1) / 2), &candidate)
            .unwrap();
        if !((powmod_result == jacobi)
            || (powmod_result == Integer::from(&candidate - 1) && jacobi == -1))
        {
            return false;
        }
    }
    return true;
}

// Not working for candidate < 10
pub fn miller_rabin(num_tests: u64, candidate: Integer) -> bool {
    let mut rand = rand::RandState::new();
    let minus_one = Integer::from(&candidate - 1);
    let s = minus_one.find_one(0).unwrap();
    let d = Integer::from(&minus_one >> s);
    let exp = Integer::from(&minus_one / 2);
    'outer: for _ in 0..num_tests {
        let mut a = Integer::from(candidate.random_below_ref(&mut rand));
        a = a.pow_mod(&exp, &candidate).unwrap();
        if a == 1 {
            continue;
        }
        for _ in 0..s {
            if a == Integer::from(&candidate - 1) {
                continue 'outer;
            }
            a = a
                .pow_mod(&MiniInteger::from(2).borrow(), &candidate)
                .unwrap();
        }
        if a != minus_one {
            return false;
        }
    }
    true
}
