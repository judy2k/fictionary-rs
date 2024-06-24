mod model;

#[cfg(target_os = "windows")]
mod sys_windows;
#[cfg(target_os = "windows")]
use sys_windows as sys;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod sys_mac;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use sys_mac as sys;

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "ios",)))]
mod sys_linux;
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "ios",)))]
use sys_linux as sys;

use clap::{Args, Parser, Subcommand};
use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, Read, Write},
};

use camino::{Utf8Path, Utf8PathBuf};
use eyre::{eyre, Result};
use model::{CharCounter, CharkovChain};
use postcard::{from_bytes, to_allocvec};
use thiserror::{self, Error};

const QUALIFIER: &str = "uk.co";
const ORG: &str = "judy";
const APP: &str = "fictionary";
const VERSION: &str = "0.1.1";
const DEFAULT_FICTIONARY: &str = "american";

#[derive(Parser, Debug)]
#[command(name = APP)]
#[command(version = VERSION)]
#[command(about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    words: WordsArgs,
}

#[derive(Args, Debug)]
struct WordsArgs {
    /// The number of fictionary words to be generated.
    #[arg(short = 'c', long, default_value_t = 1)]
    count: usize,
    /// The minimum word length
    #[arg(short = 'm', long, default_value_t = 4)]
    min_length: usize,
    /// The maximum word length
    #[arg(short = 'x', long, default_value_t = 10)]
    max_length: usize,
    /// The path to a fictionary file which will be used to generate words.
    #[arg(short = 'p', long, value_name = "FILE")]
    fictionary_file: Option<Utf8PathBuf>,
    /// The name of a fictionary to be used to generate words.
    #[arg(short = 'f', long, value_name = "NAME")]
    fictionary: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate one or more fictionary words. This is the default command.
    Words(WordsArgs),
    /// Compile a fictionary from a wordlist file.
    Compile {
        /// The path to a wordlist file. (A text file with one word per line)
        #[arg(value_name = "WORDLIST")]
        wordlist_path: Utf8PathBuf,
        /// The path to write the output fictionary file.
        #[arg(value_name = "OUTPUT-PATH")]
        output_path: Utf8PathBuf,
    },
    /// Print out a (probably) writeable location of fictionary files.
    DataDir,
    /// Print out the locations to be searched for fictionary files.
    DataDirs,
    /// Print out the available fictionary names.
    Names,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();

    match &args.command {
        Some(command) => match command {
            Commands::Words(ref words_args) => command_words(&args, words_args),
            Commands::Compile {
                wordlist_path,
                output_path,
            } => command_compile(wordlist_path as &Utf8Path, output_path),
            Commands::DataDirs => command_datadirs(),
            Commands::DataDir => command_datadir(),
            Commands::Names => command_fictionaries(),
        },
        None => command_words(&args, &args.words),
    }
}

fn available_fictionary_files(data_dirs: Vec<Utf8PathBuf>) -> Result<HashMap<String, Utf8PathBuf>> {
    let mut result = HashMap::new();

    for dir in data_dirs.into_iter().filter(|p| p.exists()) {
        for entry in dir.read_dir_utf8()? {
            let entry = entry?;
            if let Some(name) = entry.file_name().strip_suffix(".fictionary") {
                result.insert(name.to_string(), entry.into_path());
            }
        }
    }

    Ok(result)
}

fn command_words(_args: &Cli, words_args: &WordsArgs) -> Result<()> {
    if words_args.min_length < 3 {
        return Err(eyre!("--min-length must be at least 3."));
    }
    if words_args.max_length < 5 {
        return Err(eyre!("--min-length must be at least 5."));
    }
    if words_args.min_length > words_args.max_length {
        return Err(eyre!("--min-length cannot be bigger than --max-length"));
    }

    let mut filepath: Utf8PathBuf = "./american.fictionary".into();
    if let Some(ref provided_path) = words_args.fictionary_file {
        filepath.clone_from(provided_path);
    } else {
        let fictionary_files = available_fictionary_files(data_dirs())?;

        let fictionary_name = words_args
            .fictionary
            .clone()
            .unwrap_or(DEFAULT_FICTIONARY.into());

        if let Some(default_path) = fictionary_files.get(&fictionary_name) {
            filepath.clone_from(default_path);
        } else {
            // TODO: List paths searched in error message.
            return Err(eyre!(
                "Could not find {fictionary_name}.fictionary in data dirs!"
            ));
        }
    }

    // TODO: Implement verbose logging and fix next line.
    // println!("Loading from path {filepath}");
    let charkov = load_charkov(&filepath)?;
    for _ in 0..words_args.count {
        println!(
            "{}",
            charkov.word(words_args.min_length, words_args.max_length)?
        );
    }

    Ok(())
}

fn command_compile(wordlist_path: &Utf8Path, output_path: &Utf8Path) -> Result<()> {
    save_charkov(&load_wordfile(wordlist_path)?, output_path)?;
    Ok(())
}

fn command_datadirs() -> Result<()> {
    for dir in sys::data_dirs(QUALIFIER, ORG, APP) {
        println!("{dir}")
    }
    Ok(())
}

fn command_datadir() -> Result<()> {
    let dirs = sys::data_dirs(QUALIFIER, ORG, APP);
    println!("{}", dirs[dirs.len() - 1]);
    Ok(())
}

fn data_dirs() -> Vec<Utf8PathBuf> {
    sys::data_dirs(QUALIFIER, ORG, APP)
}

fn command_fictionaries() -> Result<()> {
    let file_map = available_fictionary_files(data_dirs())?;
    let mut fictionary_files: Vec<_> = file_map.keys().collect();
    fictionary_files.sort();

    for name in fictionary_files {
        println!("{name}");
    }

    Ok(())
}

fn save_charkov(charkov: &CharkovChain, path: impl AsRef<Utf8Path>) -> io::Result<()> {
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

fn load_charkov(path: impl AsRef<Utf8Path>) -> Result<CharkovChain, CharkovFileError> {
    let mut reader = io::BufReader::new(fs::File::open(path.as_ref())?);

    let mut buf = vec![];
    reader.read_to_end(&mut buf)?;
    Ok(from_bytes(&buf)?)
}

fn load_wordfile(path: impl AsRef<Utf8Path>) -> io::Result<CharkovChain> {
    let buf = io::BufReader::new(fs::File::open(path.as_ref())?).lines();
    let mut result = CharCounter::new();
    for word in buf.map_while(Result::ok) {
        let word = word.trim();
        if word.len() >= 3 {
            result.feed_word(word);
        }
    }

    Ok(result.into())
}
