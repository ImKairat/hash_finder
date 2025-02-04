use sha2::{Sha256, Digest};
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "hash_finder")]
#[command(about = "Finds numbers whose SHA-256 hash ends with N zeros.")]
struct Args {
    #[arg(short = 'N', long)]
    zeros: usize,

    #[arg(short = 'F', long)]
    results: usize,
}

pub fn find_hashes(zeros: usize, results_to_find: usize) -> Vec<(u64, String)> {
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

    Arc::try_unwrap(found_hashes).unwrap().into_inner().unwrap()
}

fn main() {
    let args = Args::parse();
    let zeros = args.zeros;
    let results_to_find = args.results;

    let found_hashes = find_hashes(zeros, results_to_find);

    for (num, hash) in found_hashes {
        println!("{}, \"{}\"", num, hash);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_ends_with_zeros() {
        let zeros = 3;
        let results_to_find = 2;
        let found_hashes = find_hashes(zeros, results_to_find);

        for (_, hash) in found_hashes {
            let trailing_zeros = hash.chars().rev().take_while(|&c| c == '0').count();
            assert_eq!(trailing_zeros, zeros);
        }
    }

    #[test]
    fn test_results_count() {
        let zeros = 3;
        let results_to_find = 5;
        let found_hashes = find_hashes(zeros, results_to_find);

        assert_eq!(found_hashes.len(), results_to_find);
    }
}
