use std::time::Instant;

use bucket::bucket_sort;
use count::count_sort;
use radix::radix_sort;
use rand::{rngs::StdRng, Rng, SeedableRng};

mod bucket;
mod count;
mod gen;
mod radix;

const SEED: u64 = 1234;

fn generate_data(count: usize, seed: u64) -> Vec<u16> {
    let mut rnd: StdRng = SeedableRng::seed_from_u64(seed);

    (0..count).map(|_| rnd.gen::<u16>()).collect()
}

fn measure(name: &str, mut func: impl FnMut() -> ()) {
    let start = Instant::now();
    func();
    println!("{} time: {:?}", name, start.elapsed());
}

fn do_bucket() {
    for i in 2..=8 {
        let count = 10usize.pow(i);

        let mut arr = generate_data(count, SEED);
        measure(&format!("bucket {}", count), || bucket_sort(&mut arr));
    }
}

fn do_radix() {
    for i in 2..=9 {
        let count = 10usize.pow(i);

        let mut arr = generate_data(count, SEED);
        measure(&format!("radix {}", count), || radix_sort(&mut arr));
    }
}

fn do_count() {
    for i in 2..=9 {
        let count = 10usize.pow(i);

        let mut arr = generate_data(count, SEED);
        measure(&format!("count {}", count), || count_sort(&mut arr));
    }
}

fn main() {
    do_bucket();
    do_radix();
    do_count();
}
