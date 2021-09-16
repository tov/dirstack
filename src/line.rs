use crate::errors::{Error, ErrorExit, Result};

use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader, BufWriter, Result as IoResult, Write};

pub type Line    = Path;
pub type LineBuf = PathBuf;

pub trait AsLine {
    fn as_line(&self) -> &Line;

    fn into_line_buf(self) -> LineBuf where Self: Sized {
        self.as_line().to_owned()
    }
}

impl AsLine for Line {
    fn as_line(&self) -> &Line {
        self
    }
}

impl AsLine for LineBuf {
    fn as_line(&self) -> &Line {
        self.as_ref()
    }

    fn into_line_buf(self) -> LineBuf {
        self
    }
}

impl AsLine for str {
    fn as_line(&self) -> &Line {
        Path::new(self)
    }
}

impl AsLine for String {
    fn as_line(&self) -> &Line {
        self.as_str().as_line()
    }

    fn into_line_buf(self) -> LineBuf {
        self.into()
    }
}

pub fn load_lines<'a>(path: &'a Path) -> Vec<LineBuf> {
    let file = fs::File::open(path)
        .unwrap_or_exit(path.display());

    BufReader::new(file)
        .lines()
        .map(|r| r.map(|s| s.into()))
        .collect::<IoResult<Vec<LineBuf>>>()
        .unwrap_or_exit(path.display())
}

pub fn store_lines(path: &Path, lines: impl Iterator<Item = LineBuf>) {
    let go = || -> Result<()> {
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|io| Error {
                io,
                reason: Some(format!("Opening {}", path.display())),
            })?;
        let file = &mut BufWriter::new(file);

        for line in lines {
            writeln!(file, "{}", line.display())
                .map_err(|io| Error {
                    io,
                    reason: Some(format!("Writing to {}", path.display())),
                })?;
        }

        Ok(())
    };

    go().unwrap_or_exit(path.display())
}
