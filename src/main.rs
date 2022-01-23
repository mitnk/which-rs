use std::env;
use std::process;
use which::which_all;

use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        process::exit(1);
    }

    let mut idx_start = 1;
    let mut find_all = false;
    if args[1].starts_with("-") {
        if args[1] != "-a" {
            println!("Illegal option {}", args[1]);
            println!("Usage: {} [-a] args", args[0]);
            process::exit(1);
        }
        if args.len() == 2 {
            process::exit(1);
        }
        idx_start = 2;
        find_all = true;
    }

    let mut missed = false;
    for bin_name in &args[idx_start..] {
        match which_all(bin_name) {
            Ok(x) => {
                let results: Vec<PathBuf> = x.collect();
                if results.len() == 0 {
                    missed = true;
                }
                for item in results {
                    println!("{}", item.to_string_lossy());
                    if !find_all {
                        break;
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
                missed = true
            }
        }
    }

    if missed {
        process::exit(1);
    }
}
