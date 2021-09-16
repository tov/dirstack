#![allow(unused)]

use std::{
    env, fmt, fs,
    io::{self, BufRead, Write},
    iter::{once, FromIterator},
    path, process,
    result,
};

mod args;

mod errors;
use errors::{Error, ErrorExit, Result};

mod getcwd;
use getcwd::getcwd;

mod line;
use line::{AsLine, Line, LineBuf, load_lines, store_lines};

const CWD_FILE_BASE: &str = ".dirstack";

fn main() {
    let mut args = args::get_args();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-l" => look(),
            "-L" => look_all(),
            "-t" => take(),
            "-c" => clear(),
            "-w" => push_cwd(),

            _ if !arg.starts_with('-') => push(&[arg]),
            "-p" => push(&[args::req_param("-p", args.next())]),
            "-b" => bury(&[args::req_param("-b", args.next())]),
            "-P" => { push(Vec::from_iter(args)); break }
            "-B" => { bury(Vec::from_iter(args)); break }

            "-h" | "--help" => {
                let _ = args::usage(io::stdout().lock());
                process::exit(0);
            }

            _ => errors::unrecognized_option_error(&arg),
        }
    }
}

fn look() {
    let cwd_file = find_cwd_file();
    if let Some(first) = load_lines(&cwd_file).get(0) {
        println!("{}", first.display());
    } else {
        process::exit(1);
    }
}

fn look_all() {
    let cwd_file = find_cwd_file();
    for line in load_lines(&cwd_file) {
        println!("{}", line.display());
    }
}

fn take() {
    let cwd_file = find_cwd_file();
    let mut lines = load_lines(&cwd_file).into_iter();
    if let Some(first) = lines.next() {
        store_lines(&cwd_file, lines);
        println!("{}", first.display());
    } else {
        process::exit(1);
    }
}

fn clear() {
    let cwd_file = find_cwd_file();
    store_lines(&cwd_file, vec![].into_iter());
}

fn push_cwd() {
    push(&[getcwd::getcwd()]);
}

fn push<S: AsLine>(new: impl AsRef<[S]>) {
    put_append(new, false);
}

fn bury<S: AsLine>(new: impl AsRef<[S]>) {
    put_append(new, true);
}

fn put_append<S: AsLine>(new: impl AsRef<[S]>, back: bool) {
    let cwd_file = find_cwd_file();
    let old = Vec::from_iter(load_lines(&cwd_file)).into_iter();
    let new = new.as_ref().into_iter().map(|s| s.as_line().to_owned());
    if back {
        store_lines(&cwd_file, old.chain(new));
    } else {
        store_lines(&cwd_file, new.chain(old));
    }
}

fn find_cwd_file() -> path::PathBuf {
    let home = env::var("HOME").unwrap_or_exit("$HOME");
    let mut result = path::PathBuf::from(home);
    result.push(CWD_FILE_BASE);
    result
}
