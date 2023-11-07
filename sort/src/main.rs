use std::time::Instant;

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    bubble::BubbleSort, heap::HeapSort, insertion::InsertionSort, merge::MergeSort,
    quick::QuickSort, selection::SelectionSort, shell::ShellSort,
};

mod bubble;
mod heap;
mod insertion;
mod merge;
mod quick;
mod selection;
mod shell;

const SEED: u64 = 123456789101112;

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

fn measure_heap_sort(size: usize) {
    let mut pool = generate_data(size, SEED);

    let start = Instant::now();

    pool.heap_sort(|item| item.value);

    let duration = start.elapsed();

    println!("heap {}: {:?}", size, duration);
}

fn do_heap_sort() {
    for i in 2..7 {
        measure_heap_sort(10_usize.pow(i));
    }
}

fn measure_selection_sort(size: usize) {
    let mut pool = generate_data(size, SEED);

    let start = Instant::now();

    pool.selection_sort(|item| item.value);

    let duration = start.elapsed();

    println!("selection {}: {:?}", size, duration);
}

fn do_selection_sort() {
    for i in 2..7 {
        measure_selection_sort(10_usize.pow(i));
    }
}

fn measure_quick_sort(size: usize) {
    let mut pool = generate_data(size, SEED);

    let start = Instant::now();

    pool.quick_sort(|item| item.value);

    let duration = start.elapsed();

    println!("quick {}: {:?}", size, duration);
}

fn do_quick_sort() {
    for i in 2..7 {
        measure_quick_sort(10_usize.pow(i));
    }
}

fn measure_merge_sort(size: usize) {
    let mut pool = generate_data(size, SEED);

    let start = Instant::now();

    pool.merge_sort(|item| item.value);

    let duration = start.elapsed();

    println!("merge {}: {:?}", size, duration);
}

fn do_merge_sort() {
    for i in 2..7 {
        measure_merge_sort(10_usize.pow(i));
    }
}

fn measure_native_sort(size: usize) {
    let mut pool = generate_data(size, SEED);

    let start = Instant::now();

    pool.sort_by_key(|item| item.value);

    let duration = start.elapsed();

    println!("native {}: {:?}", size, duration);
}

fn do_native_sort() {
    for i in 2..7 {
        measure_native_sort(10_usize.pow(i));
    }
}

fn main() {
    // do_sort();
    // do_heap_sort();
    // do_selection_sort();
    do_quick_sort();
    // do_merge_sort();
    //do_native_sort()
}
