pub fn find_iter(n: u64) -> u128 {
    match n {
        0..=1 => n as u128,
        _ => {
            let mut f1 = 0;
            let mut f2 = 1;

            for _ in 2..=n {
                let next = f1 + f2;
                f1 = f2;
                f2 = next;
            }

            f2
        }
    }
}

pub fn find_rec(n: u64) -> u128 {
    match n {
        0..=1 => n as u128,
        _ => find_rec(n - 1) + find_rec(n - 2),
    }
}

pub fn find_golden(n: u32) -> u128 {
    let sqrt_5 = 5.0f64.sqrt();
    let phi = (1.0 + sqrt_5) / 2.0;
    let psi = (1.0 - sqrt_5) / 2.0;

    let fib = ((phi.powi(n as i32) - psi.powi(n as i32)) / sqrt_5).round();

    fib as u128
}
