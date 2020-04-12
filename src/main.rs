//! Loop directories
//!
//! If it contains Cargo.toml, run cargo clean, and go to up directory
//!
//! If it doesn't contain Cargo.toml, loop it.
//!
// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links {
            Io(std::io::Error);
            Wd(walkdir::Error);
        }
    }
}

use errors::*;
use std::env;
use walkdir::DirEntry;
use walkdir::WalkDir;

quick_main!(run);

fn run() -> Result<()> {
    for entry in WalkDir::new(env::current_dir()?)
        .into_iter()
        // .filter_entry(|_e: &DirEntry| true)
        .filter_map(|e| e.ok())
    {
        println!("{}", entry.path().display());
    }
    Ok(())
}
