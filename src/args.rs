use super::errors::{Result, missing_argument_error};

use std::{env, io::Write};


pub fn usage(mut out: impl Write) -> Result<()> {
    let arg0 = arg0();
    writeln!(out, "Usage:")?;
    writeln!(out, "  {} [-l|-p]", arg0)?;
    writeln!(out, "  {} [-P|-b] DIRS...+", arg0)?;
    Ok(())
}

pub fn get_args() -> impl Iterator<Item = String> {
    let args = env::args().skip(1);

    let mut extra = Vec::new();
    if args.len() == 0 {
        extra.push("-l".to_owned());
    }

    args.chain(extra)
}

pub fn arg0() -> String {
    env::args().next()
        .unwrap_or_else(|| "cwdring".to_owned())
}

pub fn req_param<T>(option: &str, maybe: Option<T>) -> T {
    maybe.unwrap_or_else(|| missing_argument_error(option))
}
