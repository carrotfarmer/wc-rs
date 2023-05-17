use std::fs::{File, self};
use std::io::ErrorKind;
use std::process::exit;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: std::path::PathBuf,
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

    println!("{}", file_contents);
}
