mod model;

use clap::{Args, Parser, Subcommand};
use std::{
    fs,
    io::{self, BufRead, Read, Write},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use model::{CharCounter, CharkovChain};
use postcard::{from_bytes, to_allocvec};
use thiserror::{self, Error};

#[derive(Parser, Debug)]
#[command(name = "Fictionary")]
#[command(version = "0.0.1")]
#[command(about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    words: WordsArgs,
}

#[derive(Args, Debug)]
struct WordsArgs {
    #[arg(short = 'c', long, default_value_t = 1)]
    count: usize,
    #[arg(short = 'm', long, default_value_t = 4)]
    min_length: usize,
    #[arg(short = 'x', long, default_value_t = 10)]
    max_length: usize,
    #[arg(short = 'f', long)]
    fictionary_file: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Words(WordsArgs),
    Compile {
        wordlist_path: PathBuf,
        output_path: PathBuf,
    },
    DataDir,
}

fn main() -> eyre::Result<()> {
    let args = Cli::parse();

    match &args.command {
        Some(command) => match command {
            Commands::Words(ref words_args) => command_words(&args, words_args),
            Commands::Compile {
                wordlist_path,
                output_path,
            } => command_compile(&wordlist_path as &Path, &output_path),
            Commands::DataDir => command_datadir(&args),
        },
        None => command_words(&args, &args.words),
    }
}

fn command_words(_args: &Cli, words_args: &WordsArgs) -> eyre::Result<()> {
    let mut filepath: PathBuf = "./american.fictionary".into();
    if let Some(ref provided_path) = words_args.fictionary_file {
        filepath.clone_from(provided_path);
    } else if let Some(project_dirs) = ProjectDirs::from("uk.co", "judy", "fictionary") {
        project_dirs.data_dir().clone_into(&mut filepath);
        filepath.push("american.fictionary");
    }

    let charkov = load_charkov(&filepath)?;
    for _ in 0..words_args.count {
        println!(
            "{}",
            charkov.word(words_args.min_length, words_args.max_length)?
        );
    }

    Ok(())
}

fn command_compile(wordlist_path: &Path, output_path: &Path) -> eyre::Result<()> {
    save_charkov(&load_wordfile(wordlist_path)?, output_path)?;
    Ok(())
}

fn command_datadir(_args: &Cli) -> eyre::Result<()> {
    if let Some(project_dirs) = ProjectDirs::from("uk.co", "judy", "fictionary") {
        println!("{}", project_dirs.data_dir().to_string_lossy());
    }
    Ok(())
}

fn save_charkov(charkov: &CharkovChain, path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    let mut f = fs::File::create(path)?;
    let buf: Vec<u8> = to_allocvec(&charkov).unwrap();

    f.write_all(&buf)
}

#[derive(Error, Debug)]
enum CharkovFileError {
    #[error("Problem reading file.")]
    IO {
        #[from]
        source: io::Error,
    },
    #[error("Could not parse postcard data.")]
    ParseError {
        #[from]
        source: postcard::Error,
    },
}

fn load_charkov(path: impl AsRef<Path>) -> Result<CharkovChain, CharkovFileError> {
    let mut reader = io::BufReader::new(fs::File::open(path)?);

    let mut buf = vec![];
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
