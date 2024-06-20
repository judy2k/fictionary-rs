mod model;

use std::{io, fs, io::BufRead};
use clap::Parser;

use model::{CharCounter, CharkovChain};

#[derive(Parser, Debug)]
#[command(name = "Fictionary")]
#[command(version = "0.0.1")]
#[command(about, long_about = None)]
struct Args {
    #[arg(short = 'c', long, default_value_t = 1)]
    count: usize,
    #[arg(short = 'm', long, default_value_t = 4)]
    min_len: usize,
    #[arg(short = 'x', long, default_value_t = 10)]
    max_len: usize,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let filepath = "ispell_wordlist/english.0";
    println!("Reading {filepath}");
    let charkov = load_wordfile(filepath)?;
    for _ in 0..args.count {
        println!("{}", charkov.word(args.min_len, args.max_len)?);
    }
    Ok(())
}

fn load_wordfile(path: &str) -> io::Result<CharkovChain> {
    let buf = io::BufReader::new(fs::File::open(path)?).lines();
    let mut result = CharCounter::new();
    for maybe_line in buf {
        if let Ok(word) = maybe_line {
            if !word.starts_with(|c: char| c.is_uppercase()) && !word.contains('\'') {
                result.feed_word(word);
            }
        }
    }

    return Ok(result.into());
}

