use rug::{Complete, Integer};
use std::time::{Duration, Instant};

fn fib_luc(n: &Integer) -> (Integer, Integer) {
    if *n == 0 {
        return (Integer::from(0), Integer::from(2));
    }

    if (n & Integer::from(1)) == 1 {
        let n_minus_one = Integer::from(n - Integer::from(1));
        let (fib, luc) = fib_luc(&n_minus_one);
        return (
            Integer::from((&fib + &luc).complete() >> 1),
            Integer::from((Integer::from(5) * &fib + &luc) >> 1),
        );
    }

    let n_half = Integer::from(n >> 1);
    let (fib, luc) = fib_luc(&n_half);
    let luc_squared = Integer::from(luc.square_ref());
    let adjustment = Integer::from(((n & Integer::from(2)) << 1) - Integer::from(2));
    return (
        (&fib * &luc).complete(),
        Integer::from(luc_squared + adjustment),
    );
}

fn main() -> anyhow::Result<()> {
    let duration = Duration::from_secs(1);
    let mut low_number = Integer::from(0);
    let mut high_number = Integer::from(1);

    loop {
        let start = Instant::now();
        let _ = fib_luc(&high_number);
        if start.elapsed() >= duration {
            break;
        }
        low_number = high_number.clone();
        high_number *= 2;
    }

    let mut fibonacci = Integer::new();
    while low_number <= high_number {
        let sum = (&low_number + &high_number).complete();
        let mid = Integer::from(sum / 2);
        let start = Instant::now();
        let (fib_temp, _) = fib_luc(&mid);
        if start.elapsed() >= duration {
            high_number = Integer::from(&mid - 1);
        } else {
            low_number = Integer::from(&mid + 1);
            fibonacci = fib_temp;
        }
    }

    println!("Index: {}", &high_number);
    println!("Fibonacci digits: {}", fibonacci.to_string().len());

    Ok(())
}
