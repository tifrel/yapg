#[macro_use]
extern crate clap;

use std::io;

// TODO:
//  [x] print warnings in highlighted coloring (auto-detect terminal)
//  [x] document library
//  [] testing
//      [x] doc tests (library)
//      [x] unit tests (library)
//      [] integration tests (binary)
//      [x] test coverage
//  [] benchmark + profiling
//  [] add git
//  [] publish on github
//  [] publish on crates.io
//  [] add functionality for syllables and words
//  [] merge the two `PasswordGenerator::from` `impl`s by using `AsRef<str>`
//  [] refactor `CharsetSpec` into bitflag + additions

const DEFAULT_LENGTH: usize = 24;
const DEFAULT_NUMBER: usize = 20;
const ENTROPY_THRESHOLD: usize = 100;

struct Args {
    length: usize,
    number: usize,
    charset: Vec<char>,
    quiet: bool,
}

fn parse_arg_or_exit<T>(code: i32) -> impl Fn(&str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    move |arg: &str| match arg.parse::<T>() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing argument: {}", e);
            std::process::exit(code);
        },
    }
}

impl Args {
    fn get_matches() -> clap::ArgMatches<'static> {
        clap_app!(yapg =>
            (version: "0.1")
            (author: "tillyboy (https://github.com/tillyboy)")
            (about: "Generate random passphrases")
            (@arg number: -n --number +takes_value "Number (count) of passwords to print")
            (@arg length: -l --length +takes_value "Length of each password")
            (@arg added_chars: -a --add +takes_value "Additional characters to use")
            (@arg quiet: -q --quiet "Don't print debug/safety information")
            (@arg charsets: "Selection of charsets to use")
        )
        .get_matches()
    }

    pub fn get() -> io::Result<Self> {
        let matches = Self::get_matches();

        // length and number of passwords
        let length = matches
            .value_of("length")
            .map(parse_arg_or_exit(1))
            .unwrap_or(DEFAULT_LENGTH);
        let number = matches
            .value_of("number")
            .map(parse_arg_or_exit(1))
            .unwrap_or(DEFAULT_NUMBER);

        // charset
        let mut charset = match matches.value_of("charsets") {
            None => yapg::CharsetSpec::std64(),
            Some(inits) => inits.parse::<yapg::CharsetSpec>()?,
        };
        if let Some(additions) = matches.value_of("added_chars") {
            charset += additions;
        }

        // misc
        let quiet = matches.is_present("quiet");

        Ok(Args { number, length, charset: charset.into(), quiet })
    }
}

fn main() {
    let args = match Args::get() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Encountered error while parsing arguments: {}", e);
            std::process::exit(1)
        },
    };

    let mut pwg = yapg::PasswordGenerator::new(args.charset, args.length);

    // print eavesdropper warning
    if !args.quiet && args.number < 10 {
        eprintln!(
            "Any eavesdropper will have an easy time trying one of your {} \
             passphrases!",
            args.number
        );
    }

    // print low entropy warning
    if !args.quiet && pwg.entropy() < ENTROPY_THRESHOLD {
        eprintln!("Low password entropy of {} bits!", pwg.entropy() as i32);
    }

    // generate and print the passwords
    for pw in pwg.generate_n(args.number).iter() {
        println!("{}", pw);
    }

    // println!("Entropy: {} bits", pwg.entropy() as i32);
}
