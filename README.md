# Fictionary (in Rust)

This is a port of my [Fictionary tool](https://github.com/judy2k/fictionary) to
Rust. It currently works, just about, but it's still in the early stages.

## To-Do

Not in any particular order...

* Add the ability to serialize the CharkovChain to an appropriate file format,
  so that it doesn't need to be generated from a word list each time.
* Add the ability to find and load the wordlist/charkov file from a list of
  known locations on disk, not just the current working directory.
* Publish to crates.io?
* Package up for installation using Homebrew?
* Add the ability to choose a wordlist by name, such as american/british/etc
* Add the ability to choose a wordlist by path,
  so that custom local wordlists can be used.
* Distribute as a library for re-use, as well as a binary?
* Make the tests deterministic by setting a random seed.