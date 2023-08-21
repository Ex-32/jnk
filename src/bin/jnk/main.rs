extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;
use once_cell::sync::Lazy;
use owo_colors::{OwoColorize, Stream::Stdout};

mod parser;
mod repl;
mod scripts;

const CRATE: &str = env!("CARGO_CRATE_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHROS: &str = env!("CARGO_PKG_AUTHORS");

static ARGS: Lazy<Args> = Lazy::new(|| Args::parse());

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
   
    /// disable prompt prompt characters
    #[arg(short, long)]  
    quiet: bool,

    /// path to script to evaluate (`-' for stdin)
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    if let Some(path) = &ARGS.file {
        scripts::MathScript::from_file(path)?.eval()?;
    } else {
        if !ARGS.quiet {
            println!(
                "{} REPL {}{} (c) 2023 {}\n{}",
                CRATE.bold(),
                "v".bold(),
                VERSION.bold(),
                AUTHROS,
                "(press ctrl-d to exit)".if_supports_color(Stdout, |x| x.dimmed()),
            );
        }
        repl::run(&mut jnk::context::MathContext::new())?;
    }

    Ok(())
}
