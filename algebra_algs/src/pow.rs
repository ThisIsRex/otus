pub trait MyPow {
    fn pow_iter(&self, n: u64) -> Self;
    fn pow_rec(&self, n: u64) -> Self;
    fn pow_sq_rec(&self, n: u64) -> Self;
    fn pow_sq_iter(&self, n: u64) -> Self;
    fn pow_mat(&self, n: u64) -> Self;
}

impl MyPow for f64 {
    fn pow_iter(&self, n: u64) -> Self {
        match n {
            0 => 1.,
            1 => *self,
            _ => {
                let mut result = *self;

                for _ in 0..(n - 1) {
                    result *= *self;
                }

                result
            }
        }
    }

    fn pow_rec(&self, n: u64) -> Self {
        match n {
            0 => 1.,
            1 => *self,
            _ => *self * self.pow_rec(n - 1),
        }
    }

    fn pow_sq_rec(&self, n: u64) -> Self {
        match n {
            0 => 1.,
            1 => *self,
            _ => {
                if n & 1 == 0 {
                    let x = self.pow_sq_rec(n / 2);
                    x * x
                } else {
                    *self * self.pow_sq_rec(n - 1)
                }
            }
        }
    }

    fn pow_sq_iter(&self, mut n: u64) -> f64 {
        let mut result = 1.0;
        let mut base = *self;

        while n > 0 {
            if n & 1 == 1 {
                result *= base;
            }

            base *= base;
            n >>= 1;
        }

        result
    }

    fn pow_mat(&self, n: u64) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{self, BufRead, BufReader},
    };

    use crate::pow::MyPow;

    const TESTS_COUNT: usize = 10;

    fn read_in(n: usize) -> io::Result<(f64, u64)> {
        let file = File::open(format!("data/test.{}.in", n))?;
        let mut reader = BufReader::new(file).lines();

        let num_line = reader.next().ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Expected first line",
        ))??;
        let num_val: f64 = num_line
            .trim()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse f64"))?;

        let pow_line = reader.next().ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Expected second line",
        ))??;
        let pow_val: u64 = pow_line
            .trim()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse u64"))?;

        Ok((num_val, pow_val))
    }

    fn read_out(n: usize) -> io::Result<f64> {
        let file = File::open(format!("data/test.{}.out", n))?;
        let mut reader = BufReader::new(file).lines();

        let answer_line = reader.next().ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Expected first line",
        ))??;

        let answer: f64 = answer_line
            .trim()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse f64"))?;

        Ok(answer)
    }

    const PRECISION: f64 = 100000000000.;

    #[test]
    fn test_pow_iter() {
        for i in 0..TESTS_COUNT {
            let (num, pow) = read_in(i).unwrap();
            let answer = read_out(i).unwrap();

            let result = (num.pow_iter(pow) * PRECISION).round();

            assert_eq!(result, (answer * PRECISION).round());
        }
    }

    #[test]
    fn test_pow_rec() {
        for i in 0..TESTS_COUNT {
            let (num, pow) = read_in(i).unwrap();
            let answer = read_out(i).unwrap();

            let result = (num.pow_rec(pow) * PRECISION).round();

            assert_eq!(result, (answer * PRECISION).round());
        }
    }

    #[test]
    fn test_pow_sq() {
        for i in 0..TESTS_COUNT {
            let (num, pow) = read_in(i).unwrap();
            let answer = read_out(i).unwrap();

            let result = (num.pow_sq_rec(pow) * PRECISION).round();

            assert_eq!(result, (answer * PRECISION).round());
        }
    }
}
