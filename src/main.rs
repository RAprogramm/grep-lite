use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(mat) => {
                let start = mat.start();
                let end = mat.end();

                let before_match = &line[..start];
                let match_pattern = &line[start..end];
                let after_match = &line[end..];

                print!("{}", before_match);
                print!("{}", match_pattern.red());
                println!("{}",after_match);
            },
            None => (),
        }
    }
}

/// Simple and light grep clone
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pattern to search for
    pattern: String,
    /// File to search
    input: String,
}

fn main() {
    let args = Args::parse();
    let re = Regex::new(&args.pattern).unwrap();
    let input = &args.input;

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
