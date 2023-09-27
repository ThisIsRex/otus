use array::{ArrayOps, SingleArray};

use crate::array::ExtendMode;

mod array;
mod priority_queue;

fn main() {
    let mut single_array = SingleArray::new(ExtendMode::Mul2);
    dbg!(&single_array);
    single_array.add(1);
    dbg!(&single_array);
    single_array.add(2);
    dbg!(&single_array);
    single_array.add(3);
    dbg!(&single_array);
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::array::{ArrayOps, ExtendMode, SingleArray};

    #[test]
    fn test_vec() {
        let start = Instant::now();

        let mut vector = vec![];

        (0..100000).for_each(|i| {
            vector.push(i);
        });

        let duration = start.elapsed();

        println!("native vec, time: {:?}", duration);
    }

    #[test]
    fn test_single_add() {
        let start = Instant::now();

        let mut array = SingleArray::new(ExtendMode::Add(1));
        (0..100000).for_each(|i| {
            array.add(i);
        });

        let duration = start.elapsed();

        println!("add 1, time: {:?}", duration);
    }

    #[test]
    fn test_add() {
        let start = Instant::now();

        let mut array = SingleArray::new(ExtendMode::Add(10));
        (0..100000).for_each(|i| {
            array.add(i);
        });

        let duration = start.elapsed();

        println!("add 10, time: {:?}", duration);
    }

    #[test]
    fn test_factor() {
        let start = Instant::now();

        let mut array = SingleArray::new(ExtendMode::Mul(2));
        (0..100000).for_each(|i| {
            array.add(i);
        });

        let duration = start.elapsed();

        println!("mul x2, time: {:?}", duration);
    }
}
