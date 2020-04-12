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
        errors {
            NoTre(t: String) {
                description("No Tre Message")
                display("No Tre Message: {}", t)
            }
        }
    }
}

use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    use std::fs::File;

    // This operation will fail
    // File::open("tretrete").chain_err(|| ErrorKind::NoTre(String::from("no t")))?;
    File::open("tretrete")
        .map_err(|e| Error::with_chain(e, ErrorKind::NoTre(String::from("no t"))))?;

    Ok(())
}
