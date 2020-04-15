// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {
        foreign_links {
            Io(std::io::Error);
            Wd(walkdir::Error);
        }
    }
}

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use errors::*;
use std::env;
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    match run() {
        Err(ref e) => {
            error!("{}", error_chain::ChainedError::display_chain(e));
            if let Some(backtrace) = e.backtrace() {
                let frames = backtrace.frames();
                for frame in frames.iter() {
                    for symbol in frame.symbols().iter() {
                        if let (Some(file), Some(lineno)) = (symbol.filename(), symbol.lineno()) {
                            if file.display().to_string()[0..3] == "src".to_string() {
                                info!("{}:{}", file.display().to_string(), lineno);
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
    let mut cmd = Command::new("cargo");
    cmd.arg("clean");
    let it = WalkDir::new(env::current_dir()?)
        .into_iter()
        .filter_entry(|e| {
            e.file_name()
                .to_str()
                .map(|s| !s.starts_with(".") && s != "target")
                .unwrap_or(false)
        })
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_string_lossy() == "Cargo.toml");
    let mut error_dirs = Vec::new();
    for entry in it {
        let workdir = entry.path().parent().unwrap();
        info!("Cargo clean in {:?}", workdir);
        if !cmd
            .current_dir(workdir)
            .status()
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => Error::with_chain(e, "cargo is not installed!"),
                _ => Error::with_chain(e, "Errors happened during cargo clean!"),
            })?
            .success()
        {
            error_dirs.push(workdir.to_path_buf());
        }
    }
    if !error_dirs.is_empty() {
        bail!("Cargo clean failed in those directories: {:#?}", error_dirs);
    }
    Ok(())
}
