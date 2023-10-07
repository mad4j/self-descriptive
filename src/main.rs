use indicatif::{HumanFloatCount, ProgressBar, ProgressStyle};

fn pow10(n: u64) -> u64 {
    10u64.pow(n as u32)
}

fn is_selfdescriptive(value: u64) -> bool {
    let d = value.ilog10() as u64;

    let mut v = value;
    let mut t = 0;

    while v > 0 {
        t += pow10(d - v % 10);
        v /= 10;
    }

    value == t
}

fn main() {
    let limit = 10_000_000_000u64;

    let pb = ProgressBar::new(limit);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {percent:>3}% ({eta}) {msg}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    println!("Self-descriptive numbers");
    println!("------------------------");

    for k in 1..limit {
        if k % 1_000_000 == 0 {
            pb.set_position(k);
        }

        if is_selfdescriptive(k) {
            pb.suspend(|| println!("> FOUND: {}", k));
        }
    }

    pb.finish_with_message(format!(
        "completed @ {} steps/second",
        HumanFloatCount(pb.per_sec())
    ));
}
