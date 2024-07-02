use lamellar::ActiveMessaging;
use rug::Integer;

#[lamellar::AmData(Debug, Clone)]
struct PrimeTest {
    start: Integer,
    end: Integer,
}

#[lamellar::am]
impl LamellarAM for PrimeTest {
    async fn exec(self) -> usize {
        primality_tests::bigint_algorithms::wheel_threaded::general_wheel_threaded(
            10,
            self.start.clone(),
            self.end.clone(),
            128,
            primality_tests::bigint_algorithms::miller_rabin,
            vec![2, 3, 5],
            vec![1, 7, 11, 13, 17, 19, 23, 29],
        )
        .len()
    }
}

fn main() {
    let world = lamellar::LamellarWorldBuilder::new().build();
    let my_pe = world.my_pe();
    let num_pes = world.num_pes();

    if my_pe == 0 {
        
    }
}
