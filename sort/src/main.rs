use std::time::Instant;

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{bubble::BubbleSort, insertion::InsertionSort, shell::ShellSort};

mod bubble;
mod insertion;
mod shell;

#[derive(Debug, Clone)]
struct Item {
    value: u32,
}

impl Item {
    fn new(value: u32) -> Self {
        Self { value }
    }
}

fn generate_data(count: usize, seed: u64) -> Vec<Item> {
    let mut rnd: StdRng = SeedableRng::seed_from_u64(seed);

    (0..count).map(|_| Item::new(rnd.gen::<u32>())).collect()
}

fn do_sort() {
    const SEED: u64 = 22813377331;

    let vec100 = generate_data(100, SEED);
    let vec1000 = generate_data(1000, SEED);
    let vec10000 = generate_data(10000, SEED);

    //█bubble
    //100
    let mut v100 = vec100.clone();

    let start = Instant::now();
    v100.bubble_sort(|x| x.value);
    let duration = start.elapsed();

    println!("bubble 100: {:?}", duration);

    //1000
    let mut v1000 = vec1000.clone();

    let start = Instant::now();
    v1000.bubble_sort(|x| x.value);
    let duration = start.elapsed();

    println!("bubble 1000: {:?}", duration);

    //10000
    let mut v10000 = vec10000.clone();

    let start = Instant::now();
    v10000.bubble_sort(|x| x.value);
    let duration = start.elapsed();

    println!("bubble 10000: {:?}", duration);

    //█insert
    //100
    let mut v100 = vec100.clone();

    let start = Instant::now();
    v100.insertion_sort(|x| x.value);
    let duration = start.elapsed();

    println!("insert 100: {:?}", duration);

    //1000
    let mut v1000 = vec1000.clone();

    let start = Instant::now();
    v1000.insertion_sort(|x| x.value);
    let duration = start.elapsed();

    println!("insert 1000: {:?}", duration);

    //10000
    let mut v10000 = vec10000.clone();

    let start = Instant::now();
    v10000.insertion_sort(|x| x.value);
    let duration = start.elapsed();

    println!("insert 10000: {:?}", duration);

    //100
    //█shell
    let mut v100 = vec100.clone();

    let start = Instant::now();
    v100.shell_sort(|x| x.value);
    let duration = start.elapsed();

    println!("shell 100: {:?}", duration);

    //1000
    let mut v1000 = vec1000.clone();

    let start = Instant::now();
    v1000.shell_sort(|x| x.value);
    let duration = start.elapsed();

    println!("shell 1000: {:?}", duration);

    //10000
    let mut v10000 = vec10000.clone();

    let start = Instant::now();
    v10000.shell_sort(|x| x.value);
    let duration = start.elapsed();

    println!("shell 10000: {:?}", duration);
}

fn main() {
    do_sort();
}
