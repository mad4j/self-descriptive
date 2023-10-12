use rayon::prelude::*;

use indicatif::{
    HumanFloatCount, ProgressBar, ProgressStyle,
};

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
    let delta = 1_000_000u64;

    let pb = ProgressBar::new(limit)
        .with_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {percent:>3}% ({eta}) {msg}",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

    println!();
    println!("Self-descriptive numbers");
    println!("----------------------------------");
    
    println!("[num_cpus] Logical CPUs   : {:>3}", num_cpus::get());
    println!("[num_cpus] Physical CPUs  : {:>3}", num_cpus::get_physical());
    println!("[rayon]    Current Threads: {:>3}", rayon::current_num_threads());
    println!("----------------------------------");

    let solutions: Vec<u64> = (1..limit)
        .into_par_iter()
        .inspect(|&k| if k%delta == 0 { pb.inc(delta) })
        .filter(|&k| is_selfdescriptive(k))
        .inspect(|&k| pb.suspend(|| println!("+ Found: {}", k)))
        .collect();

    pb.finish();

    println!(
        "- Completed: found {} solutions @ {} steps/secs",
        solutions.len(),
        HumanFloatCount(pb.per_sec())
    );
}
