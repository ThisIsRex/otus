pub trait PrimeFind {
    fn is_prime_iter(&self) -> bool;
}

impl PrimeFind for u64 {
    fn is_prime_iter(&self) -> bool {
        match *self {
            0 => false,
            1..=2 => true,
            _ => {
                if *self & 0x01 == 0 {
                    return false;
                }

                for i in (3..=((*self as f64).sqrt() as u64)).step_by(2) {
                    if *self % i == 0 {
                        return false;
                    }
                }

                true
            }
        }
    }
}

pub fn find_prime_count(n: u64) -> u64 {
    let mut count = 0;

    for i in 2..=n {
        if i.is_prime_iter() {
            count += 1;
        }
    }

    count
}

pub fn find_eratosthenes_linear(n: usize) -> Vec<usize> {
    let mut lp = vec![0; n + 1];
    let mut primes = Vec::new();

    for i in 2..=n {
        if lp[i] == 0 {
            lp[i] = i;
            primes.push(i);
        }
        for prime in &primes {
            if *prime * i > n || *prime > lp[i] {
                break;
            }
            lp[*prime * i] = *prime;
        }
    }

    primes
}
