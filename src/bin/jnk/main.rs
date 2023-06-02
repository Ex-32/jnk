extern crate pest;
#[macro_use]
extern crate pest_derive;

use color_eyre::eyre::{Result, WrapErr};

mod parser;
mod repl;

const CRATE: &str = env!("CARGO_CRATE_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHROS: &str = env!("CARGO_PKG_AUTHORS");

fn main() -> Result<()> {
    color_eyre::install()?;
    match std::env::var_os("JNK_DEBUG") {
        Some(_) => fern::Dispatch::new()
            .format(|out, msg, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
                    record.level(),
                    record.target(),
                    msg,
                ))
            })
            .level(log::LevelFilter::Debug),
        None => fern::Dispatch::new()
            .format(|out, msg, record| {
                out.finish(format_args!(
                    "[{} {}] {}",
                    humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
                    record.level(),
                    msg,
                ))
            })
            .level(log::LevelFilter::Warn),
    }
    .chain(std::io::stderr())
    .apply()
    .wrap_err("unable to initalize logger")?;

    println!(
        "{} REPL v{} (c) 2023 {}\n(press ctrl-D to exit)",
        CRATE, VERSION, AUTHROS
    );
    repl::run(&mut jnk::context::MathContext::new())?;

    Ok(())
}
