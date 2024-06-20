# Fictionary (in Rust)

This is a port of my [Fictionary tool](https://github.com/judy2k/fictionary) to
Rust. It currently works, just about, but it's still in the early stages.

The wordlist file is currently loaded (on MacOS) from `$HOME/Library/Application Support/uk.co.judy.fictionary/english.0` 

## To-Do

In rough priority order:

* Add the ability to serialize the CharkovChain to an appropriate file format,
  so that it doesn't need to be generated from a word list each time.
* Make wordlist/charkov file loading a bit more idiomatic and cascading from shared to user directories.
* Publish to crates.io?

* Add the ability to choose a wordlist by name, such as american/british/etc
* Add the ability to choose a wordlist by path,
  so that custom local wordlists can be used.
* Make the tests deterministic by setting a random seed.
* Validate that --min-length &lt;= --max-length
* Document the code.
* Different output depending on stdout being a TTY
* Capital-letter words should not be added to the chain, but *should* be converted to lower-case and added to the known wordset, so that these words aren't accidentally generated.
* Add a CLI param to specify a seed to the generator so that the same wordlist can be generated more than once.
* Implement an infinite iterator of generated words.

* CI/CD
* More comprehensive tests
* Distribute as a library for re-use, as well as a binary?
* Package up for installation using Homebrew?