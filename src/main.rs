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
use std::process::Command;
use walkdir::WalkDir;

// quick_main!(run);
fn main() {
    match run() {
        Err(ref e) => {
            eprintln!("{}", error_chain::ChainedError::display_chain(e));
            if let Some(backtrace) = e.backtrace() {
                let frames = backtrace.frames();
                for frame in frames.iter() {
                    for symbol in frame.symbols().iter() {
                        if let (Some(file), Some(lineno)) = (symbol.filename(), symbol.lineno()) {
                            if file.display().to_string()[0..3] == "src".to_string() {
                                println!("{}:{}", file.display().to_string(), lineno);
                            }
                        }
                    }
                }
            }
        }
        Ok(code) => std::process::exit(error_chain::ExitCode::code(code)),
    };
}

fn run() -> Result<()> {
    let mut it = WalkDir::new(env::current_dir()?)
        .into_iter()
        .filter_entry(|e| {
            e.file_name()
                .to_str()
                .map(|s| !s.starts_with(".") && s != "target")
                .unwrap_or(false)
        });
    loop {
        let entry = match it.next() {
            Some(e) => e?,
            None => break,
        };
        if entry.file_name().to_string_lossy() == "Cargo.toml" {
            let workdir = entry.path().parent().unwrap();
            println!("Cargo clean in {:?}", workdir);
            if !Command::new("cargo")
                .arg("clean")
                .current_dir(workdir)
                .status()
                .map_err(|e| match e.kind() {
                    std::io::ErrorKind::NotFound => Error::with_chain(e, "cargo is not installed!"),
                    _ => Error::with_chain(e, "Errors happened during cargo clean!"),
                })?
                .success()
            {
                bail!("cargo clean failed!");
            }
        }
    }
    Ok(())
}
