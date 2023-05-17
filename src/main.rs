use std::fs::{File, self};
use std::io::ErrorKind;
use std::process::exit;
use std::fmt::Display;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: std::path::PathBuf,
}

#[derive(Debug)]
struct WcOutput {
    lines_count: u32,
    word_count: u32,
    bytes_count: u32,
    filename: std::path::PathBuf,
}

impl WcOutput {
    fn new(word_count: u32, lines_count: u32, bytes_count: u32, filename: std::path::PathBuf) -> Self {
        Self {
            lines_count,
            word_count,
            bytes_count,
            filename
        }
    }
}

impl Display for WcOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}\t{} {}", self.lines_count, self.word_count, self.bytes_count, self.filename.display())
    }
}

fn main() {
    let args = Args::parse();

    match File::open(&args.filename) {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("No such file: {:?}", args.filename);
                exit(1);
            },
            _ => {
                println!("Error: {:?}", e);
                exit(1);
            }
        },
        Ok(_) => {}
    }

    let file_contents = fs::read_to_string(&args.filename).unwrap();

    let wc_output = WcOutput::new(count_words(&file_contents), count_lines(&file_contents), count_bytes(&file_contents), args.filename.clone());

    println!("{}", wc_output);
}

fn count_words(contents: &str) -> u32 {
    let mut count = 0;

    for _ in contents.split_whitespace() {
        count += 1;
    }

    count
} 

fn count_lines(contents: &str) -> u32 {
    let mut lines = 0;

    for _ in contents.lines() {
        lines += 1;
    }

    lines
}

fn count_bytes(contents: &str) -> u32 {
    let mut count = 0;

    for _ in contents.as_bytes() {
        count += 1;
    }

    count
}
