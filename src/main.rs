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
// use walkdir::DirEntry;
use walkdir::WalkDir;

quick_main!(run);

fn run() -> Result<()> {
    for entry in WalkDir::new(env::current_dir()?)
        .into_iter()
        .filter_entry(|e| {
            e.file_name()
                .to_str()
                .map(|s| !s.starts_with(".") && s != "target")
                .unwrap_or(false)
        })
    {
        let entry = entry?;
        println!("{}", entry.path().display());
        if entry.file_name().to_string_lossy() == "Cargo.toml" {
            println!("Cargo!");
        }
    }
    Ok(())
}
