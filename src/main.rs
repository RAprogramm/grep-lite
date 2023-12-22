use clap::Parser;
use regex::Regex;

/// Simple and light grep clone
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pattern to search for
    pattern: String,
}

fn main() {
    let args = Args::parse();

    let re = Regex::new(&args.pattern).unwrap();

    let quote = "Every face, every shop, bedroom window, puЬlic-house, and dark square 
is а picture feverishly turned--in search of what? 
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
