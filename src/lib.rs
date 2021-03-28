//! This crate is for generating random passphrases from characters.
//! The heavy lifting is done by `PasswordGenerator` structs, which needs to be
//! mutable because of the encapsulated
//! [underlying RNG](https://docs.rs/rand/0.8.3/rand/rngs/struct.ThreadRng.html).
//!
//! # Examples
//! ```
//! let mut pwg = yapg::PasswordGenerator::from("ab").length(10);
//! let pass = pwg.generate();
//! assert_eq!(pass.len(), 10);
//! assert_eq!(pass.to_ascii_lowercase(), pass);
//! assert_eq!(pwg.entropy(), 10);
//!
//! let pass_vec = pwg.length(2).generate_n(2);
//! assert_eq!(pass_vec.len(), 2);
//! let permutations = vec![
//!     "aa".to_string(),
//!     "ab".to_string(),
//!     "ba".to_string(),
//!     "bb".to_string(),
//! ];
//! assert!(permutations.contains(&pass_vec[0]));
//! assert!(permutations.contains(&pass_vec[1]));
//! ```
//!
//! # Future ideas
//! - creating passphrases from syllables or words
use rand::Rng;

mod charsets;
pub use charsets::*;

/// Encapsulates RNG and set of characters. See crate documentation for more.
#[derive(Debug)]
pub struct PasswordGenerator {
    charset: Vec<char>,
    length: usize,
    rng: rand::ThreadRng,
}

impl PasswordGenerator {
    /// Creates the `PasswordGenerator` to yield passwords using either
    /// `PasswordGenerator::generate` or `PasswordGenerator::generate_n`
    /// `charset` will not be deduplicated, so that you could (but should not!)
    /// increase the the probability density of the chars in the generated
    /// passwords.
    pub fn new(charset: Vec<char>, length: usize) -> Self {
        PasswordGenerator { charset, length, rng: rand::thread_rng() }
    }

    /// Changes the length of the generated passwords, consumes and returns
    /// itself.
    #[inline]
    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    /// Generates one password, with characters randomly chosen from the
    /// charset.
    #[inline]
    pub fn generate(&mut self) -> String {
        let mut s = String::with_capacity(self.length);
        for _ in (0..self.length).into_iter() {
            s.push(*self.rng.choose(&self.charset).unwrap());
        }
        s
    }

    /// Generates a vector of passwords with length n, calling
    /// `PasswordGenerator::generate` internally.
    /// Cannot return an iterator, because that iterator would need to hold
    /// a mutable reference to the generator.
    #[inline]
    pub fn generate_n(&mut self, n: usize) -> Vec<String> {
        (0..n).into_iter().map(|_| self.generate()).collect()
    }

    /// Number of all possible combinations arising from charset and length.
    #[inline]
    pub fn combinations(&self) -> f64 {
        (self.charset.len() as f64).powf(self.length as f64)
    }

    /// Entropy of the generated passwords in bits.
    #[inline]
    pub fn entropy(&self) -> usize {
        self.combinations().log2().floor() as usize
    }
}

impl std::convert::From<Vec<char>> for PasswordGenerator {
    fn from(charset: Vec<char>) -> PasswordGenerator {
        PasswordGenerator::new(charset, 20)
    }
}

impl std::convert::From<&str> for PasswordGenerator {
    fn from(charset: &str) -> PasswordGenerator {
        PasswordGenerator::new((charset).chars().collect(), 20)
    }
}

// notes for id's:
// target collision probability: 1/1e21
// humans: 1e10 (10 billion)
// items/human: 1e5 (100 thousand)
//      => total items: 1e15
//      => total number of required ids: 1e30
//      => digits in base64: 16.6
// TODO: how to calculate the mean draws until collision?
