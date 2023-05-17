mod count;

use crate::count::*;

use std::fmt::Display;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::process::exit;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
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

    #[clap(num_args=1.., value_delimiter=' ')]
    filenames: Vec<std::path::PathBuf>,
}

#[derive(Debug)]
struct Wc {
    filenames: Vec<std::path::PathBuf>,
    words: bool,
    lines: bool,
    bytes: bool,
    chars: bool,
    max_line_length: bool,
}

impl Wc {
    fn new(
        filenames: Vec<std::path::PathBuf>,
        words: bool,
        lines: bool,
        bytes: bool,
        chars: bool,
        max_line_length: bool,
    ) -> Self {
        Self {
            filenames,
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

        for filename in &self.filenames {
            let contents = fs::read_to_string(&filename).unwrap();

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

            write!(f, "{}\n\t", &filename.display())?
        }

        Ok(())
    }
}

fn main() {
    let mut args = Args::parse();

    if !args.lines && !args.words && !args.bytes && !args.chars && !args.max_line_length {
        args.words = true;
        args.lines = true;
        args.bytes = true;
    }

    for filename in &args.filenames {
        match File::open(filename) {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    println!("No such file: {:?}", filename);
                    exit(1);
                }
                _ => {
                    println!("Error: {:?}", e);
                    exit(1);
                }
            },
            Ok(_) => {}
        }
    }

    let wc_output = Wc::new(
        args.filenames.clone(),
        args.words,
        args.lines,
        args.bytes,
        args.chars,
        args.max_line_length,
    );

    println!("{}", wc_output);
}
