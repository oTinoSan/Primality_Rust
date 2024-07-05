use rug::{integer::MiniInteger, rand, Integer};

pub fn miller_rabin_bigrug(candidate: Integer, iterations: u64) -> bool {
    if candidate == Integer::from(2u8) || candidate == Integer::from(3u8) {
        return true;
    }
    if Integer::from(&candidate % &Integer::from(2u8)) == Integer::from(0u8)
        || candidate == Integer::from(1u8)
    {
        return false;
    }
    let mut rand = rand::RandState::new();
    let minus_one = Integer::from(&candidate - 1);
    let s = minus_one.find_one(0).unwrap();
    let d = Integer::from(&minus_one >> s);

    'outer: for _ in 0..iterations {
        let mut a = Integer::from(
            Integer::from(Integer::from(&candidate - 3).random_below_ref(&mut rand)) + 1,
        );
        a = a.pow_mod(&d, &candidate).unwrap();
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
