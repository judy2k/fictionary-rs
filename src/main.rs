mod model;

use std::{fs, io::{self, BufRead}, path::{Path, PathBuf}};
use clap::Parser;

use directories::ProjectDirs;
use model::{CharCounter, CharkovChain};

#[derive(Parser, Debug)]
#[command(name = "Fictionary")]
#[command(version = "0.0.1")]
#[command(about, long_about = None)]
struct Args {
    #[arg(short = 'c', long, default_value_t = 1)]
    count: usize,
    #[arg(short = 'm', long, default_value_t = 4)]
    min_length: usize,
    #[arg(short = 'x', long, default_value_t = 10)]
    max_length: usize,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let mut filepath: PathBuf = "ispell_wordlist/english.0".into();
    if let Some(project_dirs) = ProjectDirs::from("uk.co", "judy", "fictionary") {
        filepath = project_dirs.data_dir().to_owned();
        filepath.push("english.0");
    }
    
    println!("Reading {filepath:?}");
    let charkov = load_wordfile(&filepath)?;
    for _ in 0..args.count {
        println!("{}", charkov.word(args.min_length, args.max_length)?);
    }
    Ok(())
}

fn load_wordfile(path: &Path) -> io::Result<CharkovChain> {
    let buf = io::BufReader::new(fs::File::open(path)?).lines();
    let mut result = CharCounter::new();
    for word in buf.map_while(Result::ok) {
        if !word.starts_with(|c: char| c.is_uppercase()) && !word.contains('\'') {
            result.feed_word(word);
        }
    }

    Ok(result.into())
}

