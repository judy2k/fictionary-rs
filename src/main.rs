mod model;

use clap::Parser;
use std::{fs, io::{self, BufRead, Read, Write}, path::{Path, PathBuf}};

use directories::ProjectDirs;
use model::{CharCounter, CharkovChain};
use postcard::{from_bytes, to_allocvec};
use thiserror::{self, Error};

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

    let mut filepath: PathBuf = "./english.charkov".into();
    if let Some(project_dirs) = ProjectDirs::from("uk.co", "judy", "fictionary") {
        project_dirs.data_dir().clone_into(&mut filepath);
        filepath.push("english.charkov");
    }
    
    let charkov = load_charkov(&filepath)?;
    for _ in 0..args.count {
        println!("{}", charkov.word(args.min_length, args.max_length)?);
    }

    Ok(())
}

/// Take a wordlist file and generate a new charkov file from it.
fn generate_charkov(wordlist_path: &Path, output_path: &Path) -> io::Result<()> {
    save_charkov(&load_wordfile(wordlist_path)?, output_path)
}

fn save_charkov(charkov: &CharkovChain, path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    let mut f = fs::File::create(path)?;
    let buf: Vec<u8> = to_allocvec(&charkov).unwrap();

    f.write_all(&buf)?;

    Ok(())
}

#[derive(Error, Debug)]
enum CharkovFileError {
    #[error("Problem reading file.")]
    IO{
        #[from]
        source: io::Error,
    },
    #[error("Could not parse postcard data.")]
    ParseError{
        #[from]
        source: postcard::Error,
    },
}

fn load_charkov(path: impl AsRef<Path>) -> Result<CharkovChain, CharkovFileError> {
    let mut reader = io::BufReader::new(fs::File::open(path)?);

    let mut buf  = vec![];
    reader.read_to_end(&mut buf)?;
    Ok(from_bytes(&buf)?)
}

fn load_wordfile(path: impl AsRef<Path>) -> io::Result<CharkovChain> {
    let buf = io::BufReader::new(fs::File::open(path)?).lines();
    let mut result = CharCounter::new();
    for word in buf.map_while(Result::ok) {
        if !word.starts_with(|c: char| c.is_uppercase()) && !word.contains('\'') {
            result.feed_word(word);
        }
    }

    Ok(result.into())
}

