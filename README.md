# Yet another password generator!

[![docs.rs badge](https://docs.rs/yapg/badge.svg)](https://docs.rs/yapg/)
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)
[![license](https://img.shields.io/github/license/tillyboy/yapg)](https://github.com/tillyboy/yapg/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/yapg)](https://crates.io/crates/yapg)

Not because the world needed it, but for me to learn about the rust ecosystem.
I have some ideas to improve it in the future, but having learned something
about rust tooling, it's already done its job for me.

## Implemented functionality

- generating random passwords from characters
  - configurable character set, length and amount of passwords

## Possible future functionality (unlikely)

- generating random passwords from syllables
  - allowed [syllables read from file](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases)
  - configurable intercalation with special chars
  - configurable capitalization rules
- generating random passphrases from words
  - basically the same as from syllables

## Used tooling/crates (+ notes)

- testing and documentation with `cargo`
  - test coverage with [`grcov`](https://crates.io/crates/grcov)
    - source-based coverage appears bugged (e.g. comments marked as missing
      coverage)
- automation using [`cargo-make`](https://crates.io/crates/cargo-make)
  - seems a bit clunky when compared to GNU's `make`, ymmv.
- source tracking with `git` and `hub` (obviously...)
- publishing to `crates.io` with `cargo` (no shit, Sherlock!)

## Possible future tooling (somewhat likely)

- benchmarking with `cargo`
  - profiling with [`inferno`](https://crates.io/crates/inferno) and
    [`flamegraph`]()
- integration tests for the binary
- publish docs (docs.rs?)
- link git repo
