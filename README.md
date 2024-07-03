# Fictionary (in Rust)

[![Rust Build][ci-badge]][ci-url]
[![Crate Version][version-badge]][version-url]

This is a port of my [Fictionary tool](https://github.com/judy2k/fictionary) to
Rust. It currently works, but it's still in the early stages.

The wordlist file is currently loaded (on MacOS) from `$HOME/Library/Application Support/uk.co.judy.fictionary/american.fictionary` 

## To-Do

In rough priority order:

* Test fictionary file locations on Linux and Windows!
* Write a decent README.
* Publish to crates.io?

* Make the tests deterministic by setting a random seed.
  * Add a CLI param to specify a seed to the generator so that the same wordlist can be generated more than once.
* Document the code.
* Different output depending on stdout being a TTY

* Continuous Delivery
* More comprehensive tests
* Distribute as a library for re-use, as well as a binary?
* Implement an infinite iterator of generated words.
* Package up for installation using Homebrew?

[ci-url]: https://github.com/judy2k/fictionary-rs/actions?query=branch%3Amain+workflow%3ARust
[ci-badge]: https://img.shields.io/github/actions/workflow/status/judy2k/fictionary-rs/rust.yml?branch=main&style=for-the-badge
[version-badge]: https://img.shields.io/crates/v/fictionary?style=for-the-badge&logo=rust
[version-url]: https://crates.io/crates/fictionary