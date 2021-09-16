use super::args;

use std::{fmt, io, process, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub io: io::Error,
    pub reason: Option<String>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}\n", self.io)?;
        if let Some(reason) = &self.reason {
            writeln!(f, "  ({})\n", reason)?;
        }
        Ok(())
    }
}

impl From<io::Error> for Error {
    fn from(io: io::Error) -> Self {
        Error {
            io,
            reason: None,
        }
    }
}

impl Error {
    pub fn because(args: fmt::Arguments<'_>) -> impl FnOnce(io::Error) -> Self {
        let reason = Some(format!("{}", args));
        move |io| Error { io, reason, }
    }
}

pub fn missing_argument_error(option: &str) -> ! {
    usage_error(&format!("option requires an argument: ‘{}’", option));
}

pub fn unrecognized_option_error(option: &str) -> ! {
    usage_error(&format!("unrecognized option: ‘{}’", option));
}

pub fn usage_error(message: &str) -> ! {
    eprintln!("{}: {}", args::arg0(), message);
    let _ = args::usage(io::stderr().lock());
    process::exit(10);
}

pub trait ErrorExit<T>: Sized {
    type Error: fmt::Display;

    fn unwrap_or_fail(self, fail: impl FnOnce(Option<Self::Error>) -> Never) -> T;

    fn unwrap_or_exit(self, what: impl fmt::Display) -> T {
        self.unwrap_or_exit_code(what, 11)
    }

    fn unwrap_or_exit_code(self, what: impl fmt::Display, code: i32) -> T {
        self.unwrap_or_fail(|e| {
            if let Some(e) = e {
                eprintln!("{}: {}: {}", args::arg0(), e, what);
            } else {
                eprintln!("{}: {}", args::arg0(), what);
            }

            process::exit(code);
        })
    }
}

impl<T, E> ErrorExit<T> for result::Result<T, E>
where
    E: fmt::Display,
{
    type Error = E;

    fn unwrap_or_fail(self, fail: impl FnOnce(Option<Self::Error>) -> Never) -> T {
        self.unwrap_or_else(|e| fail(Some(e)).elim())
    }
}

impl<T> ErrorExit<T> for Option<T> {
    type Error = Never;

    fn unwrap_or_fail(self, fail: impl FnOnce(Option<Self::Error>) -> Never) -> T {
        self.unwrap_or_else(|| fail(None).elim())
    }
}

//
// An uninhabited type, for things that can never happen
//

pub enum Never { }

impl Never {
    pub fn elim(&self) -> ! {
        match *self { }
    }
}

impl fmt::Display for Never {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        self.elim()
    }
}

