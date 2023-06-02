use color_eyre::eyre::{eyre, Result};
use rustyline::error::ReadlineError;

pub(crate) fn run(ctx: &mut jnk::context::MathContext) -> Result<()> {
    let mut rl = rustyline::DefaultEditor::new()?;
    loop {
        match ctx.eval(&match rl.readline("jnk >> ") {
            Ok(x) => x,
            Err(e) => match e {
                ReadlineError::Eof => break Ok(()),
                ReadlineError::Interrupted => continue,
                _ => return Err(eyre!(e).wrap_err("unexpcted error reading user input")),
            },
        }) {
            Ok(x) => println!("-> {}", x),
            Err(e) => match e {
                jnk::error::Error::VarNotFound(_) => {
                    log::warn!("{}", e);
                    continue;
                }
                _ => return Err(eyre!(e).wrap_err("unexpcted error parsing expression")),
            },
        }
    }
}
