use std::sync::{Arc, Mutex};
use std::thread;
use sha2::{Sha256, Digest};
use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Parser, Debug)]
#[command(name = "hash_finder")]
#[command(about = "Finds numbers whose SHA-256 hash ends with N zeros.")]
struct Args {
    #[arg(short = 'N', long)]
    zeros: usize,

    #[arg(short = 'F', long)]
    results: usize,
}

fn main() {
    let args = Args::parse();

    let zeros = args.zeros;
    let results_to_find = args.results;

    let found_hashes = Arc::new(Mutex::new(Vec::new()));
    let counter = Arc::new(Mutex::new(1u64));
    let stop_flag = Arc::new(AtomicBool::new(false));

    let mut handles = vec![];
    let num_threads = num_cpus::get();

    for _ in 0..num_threads {
        let found_hashes = Arc::clone(&found_hashes);
        let counter = Arc::clone(&counter);
        let stop_flag = Arc::clone(&stop_flag);

        let handle = thread::spawn(move || {
            loop {
                if stop_flag.load(Ordering::Relaxed) {
                    break;
                }

                let num = {
                    let mut counter = counter.lock().unwrap();
                    let num = *counter;
                    *counter += 1;
                    num
                };

                let hash = Sha256::digest(num.to_string().as_bytes());
                let hash_hex = format!("{:x}", hash);

                if hash_hex.ends_with(&"0".repeat(zeros)) {
                    let mut found_hashes = found_hashes.lock().unwrap();
                    println!("{}, \"{}\"", num, hash_hex);
                    found_hashes.push((num, hash_hex));

                    if found_hashes.len() >= results_to_find {
                        stop_flag.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
