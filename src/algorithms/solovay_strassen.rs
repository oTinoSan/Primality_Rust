use super::mod_pow::mod_pow;
use rand::prelude::*;

pub fn jacobi(a: i64, n: i64) -> i64 {
    let mut j = 1i64;
    let mut nn = n;
    let mut aa = a % n;
    while nn > 1 {
        if aa == 0 {
            return 0;
        }
        let mut t = 0;
        while aa % 2 == 0 {
            t += 1;
            aa /= 2;
        }
        if t % 2 == 1 {
            j = j * i64::pow(-1, ((nn * nn - 1) / 8) as u32);
        }
        if aa % 4 == 3 && nn % 4 == 3 {
            j = -j;
        }
        let temp = nn;
        nn = aa;
        aa = temp % aa;
    }
    return j;
}

pub fn solovay_strassen(candidate: i64) -> bool {
    if candidate < 4 {
        return candidate == 2 || candidate == 3;
    }
    if candidate % 2 == 0 {
        return false;
    }
    let mut rng = rand::thread_rng();
    let iterations = 3;
    let max_range = (candidate as f64).sqrt() as i64;
    for _ in 0..iterations {
        let a = rng.gen_range(2..max_range);
        let x = jacobi(a, candidate);
        let y = mod_pow(a as u64, ((candidate - 1) / 2) as u64, candidate as u64) as i64;
        if x == 0 || y != (x + candidate) % candidate {
            return false;
        }
    }
    return true;
}
