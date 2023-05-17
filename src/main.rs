mod count;

use crate::count::*;

use std::fmt::Display;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::process::exit;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: std::path::PathBuf,

    #[clap(short = 'l', long)]
    lines: bool,

    #[clap(short = 'w', long)]
    words: bool,

    #[clap(short = 'c', long)]
    bytes: bool,

    #[clap(short = 'm', long)]
    chars: bool,

    #[clap(short = 'L', long)]
    max_line_length: bool,
}

#[derive(Debug)]
struct Wc {
    filename: std::path::PathBuf,
    words: bool,
    lines: bool,
    bytes: bool,
    chars: bool,
    max_line_length: bool,
}

impl Wc {
    fn new(
        filename: std::path::PathBuf,
        words: bool,
        lines: bool,
        bytes: bool,
        chars: bool,
        max_line_length: bool,
    ) -> Self {
        Self {
            filename,
            words,
            lines,
            bytes,
            chars,
            max_line_length,
        }
    }
}

impl Display for Wc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\t")?;

        let contents = fs::read_to_string(&self.filename).unwrap();

        if self.lines {
            write!(f, "{}\t", count_lines(&contents))?;
        }

        if self.words {
            write!(f, "{}\t", count_words(&contents))?;
        }

        if self.bytes {
            write!(f, "{}\t", count_bytes(&contents))?;
        }

        if self.chars {
            write!(f, "{}\t", count_chars(&contents))?;
        }

        if self.max_line_length {
            write!(f, "{}\t", count_lines(&contents))?;
        }

        write!(f, "{}", self.filename.display())
    }
}

fn main() {
    let mut args = Args::parse();

    if !args.lines && !args.words && !args.bytes && !args.chars && !args.max_line_length {
        args.words = true;
        args.lines = true;
        args.bytes = true;
    }

    match File::open(&args.filename) {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("No such file: {:?}", args.filename);
                exit(1);
            }
            _ => {
                println!("Error: {:?}", e);
                exit(1);
            }
        },
        Ok(_) => {}
    }

    let wc_output = Wc::new(
        args.filename.clone(),
        args.words,
        args.lines,
        args.bytes,
        args.chars,
        args.max_line_length,
    );

    println!("{}", wc_output);
}
