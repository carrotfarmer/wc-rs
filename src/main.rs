mod wc;
mod count;

use crate::wc::*;

use std::fs::File;
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
