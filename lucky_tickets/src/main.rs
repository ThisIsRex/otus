//start 19:00

use std::io;

struct Number {
    digits: Vec<u32>,
}

impl Number {
    fn new(n: usize) -> Self {
        Self {
            digits: vec![0; n - 1],
        }
    }

    fn inc(&mut self) {
        let mut addy = 0;

        self.digits
            .iter_mut()
            .enumerate()
            .rev()
            .for_each(|(i, num)| {
                let mut new_num = *num + 1 + addy;
                if new_num > 9 {
                    if i == 0 {
                        panic!("overflow")
                    }
                    addy = 1;
                    new_num = new_num % 10;
                }

                *num = new_num;
            });
    }

    fn get_digit_sum(&self) -> u32 {
        self.digits.iter().fold(0, |sum, num| sum + num)
    }
}

fn get_number_count_for_sum(sum: u32, n: usize, last_digit: u32) -> u32 {
    if last_digit > 9 {
        panic!("up to 9");
    }

    let mut count = 0;

    let mut number = Number::new(n - 1);

    for i in 0..10u64.pow(n as u32 - 1) {
        number.inc();
        let sum = number.get_digit_sum() + 1;
        if sum == 10 {
            count += 1;
        }
    }

    count
}

fn get_lucky_numbers_count(n: usize) -> u64 {
    println!("2N: {}", n * 2);

    let max = 9 * n;

    for i in 0..max {
        println!(
            "nums for sum {}: {}",
            i,
            get_number_count_for_sum(i as u32, n, 1)
        );
    }

    0
}

fn main() {
    get_lucky_numbers_count(3);

    // let mut half_num_str = String::new();

    // io::stdin()
    //     .read_line(&mut half_num_str)
    //     .expect("read error");

    // let n: u32 = half_num_str.parse().expect("parse error");

    // let count = get_lucky_numbers_count(n);

    // println!("{}", count);
}
