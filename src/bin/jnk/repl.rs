use color_eyre::eyre::{eyre, Result};
use owo_colors::{OwoColorize, Stream::Stdout};
use rustyline::error::ReadlineError;

pub(crate) fn run(ctx: &mut jnk::context::MathContext) -> Result<()> {
    let mut rl = rustyline::DefaultEditor::new()?;
    let prompt = if !crate::ARGS.quiet {
        format!("jnk {} ", ">>".bold())
    } else {
        String::new()
    };
    loop {
        let input = match rl.readline(&prompt) {
            Ok(x) => x,
            Err(e) => match e {
                ReadlineError::Eof => break Ok(()),
                ReadlineError::Interrupted => continue,
                _ => return Err(eyre!(e).wrap_err("unexpcted error reading user input")),
            },
        };
        match ctx.eval(&input) {
            Ok(x) => {
                if x.var.is_none() {
                    if crate::ARGS.quiet {
                        println!("{}", x.value)
                    } else {
                        println!("-> {}", x.value.bold())
                    }
                }
            },
            Err(e) => match e {
                jnk::error::Error::VarNotFound(var) => println!(
                    "{} {} '{}'",
                    "ERROR".if_supports_color(Stdout, |x| x.red()).bold(),
                    "variable not found:".bold(),
                    var,
                ),
                jnk::error::Error::NotValidVar(var) => println!(
                    "{} {} '{}'",
                    "ERROR".if_supports_color(Stdout, |x| x.red()).bold(),
                    "invalid variable name:".bold(),
                    var,
                ),
                jnk::error::Error::ParseError { parse_failure, .. } => println!(
                    "{} {}\n{}",
                    "ERROR".if_supports_color(Stdout, |x| x.red()).bold(),
                    "failed to parse expression:".bold(),
                    parse_failure,
                ),
                _ => println!(
                    "{} {}\n{}",
                    "ERROR".if_supports_color(Stdout, |x| x.red()).bold(),
                    "unexpected error:".bold(),
                    e,
                ),
            },
        }
    }
}

