use std::convert::{Into, TryFrom};
use std::io;

/// Contains all lower-case latin letters
pub static CHARSET_ALPHA_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Contains all upper-case latin letters.
pub static CHARSET_ALPHA_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Contains all digits.
pub static CHARSET_NUMERIC: [char; 10] =
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Contains '.', ':', ',', ';', '!', '?', ' ', '\'', and '"'.
pub static CHARSET_PROSE: [char; 9] =
    ['.', ':', ',', ';', '!', '?', ' ', '\'', '"'];

/// Contains '+', '-', '*', '/', '=', '<', and '>'.
pub static CHARSET_MATHOPS: [char; 7] = ['+', '-', '*', '/', '=', '<', '>'];

/// Contains '(', ')', '[', ']', '{', and '}'.
pub static CHARSET_DELIM: [char; 6] = ['(', ')', '[', ']', '{', '}'];

/// Contains '#', '@', '$', '%', '&', '|', '\\', '~',
/// '^', '_', and '`'.
pub static CHARSET_MISC_SPECIAL: [char; 11] =
    ['#', '@', '$', '%', '&', '|', '\\', '~', '^', '_', '`'];

// total specials: 9 + 7 + 6 + 11 = 33
// ----------------------- intermediaries for user IO ----------------------- //
/// Translation layer between chars (e.g. for cli flags) and the actual
/// character sets.
///
/// Especially, you can do `CharsetName::from::<char>(c)`. Translations are:
///
/// | CharsetName   | associated char | contained chars                                       |
/// | ------------- | --------------- | ----------------------------------------------------- |
/// | `AlphaLower`  | `'L'`           | matching regex `[a-z]`                                |
/// | `AlphaUpper`  | `'U'`           | matching regex `[A-Z]`                                |
/// | `Numeric`     | `'N'`           | matching regex `[0-9]`                                |
/// | `Mathops`     | `'M'`           | `+`, `-`, `*`, `/`, `=`, `<`, `>`                     |
/// | `Prose`       | `'P'`           | `.`, `,`, `:`, `;`, `!`, `?`, `'`, `"`, ` `           |
/// | `Delim`       | `'D'`           | `(`, `)`, `{`, `}`, `[`, `]`                          |
/// | `MiscSpecial` | `'X'`           | `#`, `@`, `$`, `%`, `&`, `|`, `\`, `~`, `^`, `_`, ``` |
///
/// For convenience, there are also some charsets built from the "atomic"
/// charsets shown above:
///
/// | CharsetName | associated char | Contained Charsets                                           |
/// | ----------- | --------------- | ------------------------------------------------------------ |
/// | `Alpha`     | `'A'`           | `AlphaLower`, `AlphaUpper`                                   |
/// | `Special`   | `'S'`           | `Mathops`, `Punct`, `Delim`, `Quote`, `Blank`, `MiscSpecial` |
#[derive(Debug, PartialEq)]
pub enum CharsetName {
    // atomic
    AlphaLower,
    AlphaUpper,
    Numeric,
    Mathops,
    Prose,
    Delim,
    MiscSpecial,
    // compound
    Alpha,
    Special,
}

impl TryFrom<char> for CharsetName {
    type Error = io::Error;

    fn try_from(c: char) -> io::Result<Self> {
        match c {
            // atomic
            'U' => Ok(Self::AlphaUpper),
            'L' => Ok(Self::AlphaLower),
            'N' => Ok(Self::Numeric),
            'M' => Ok(Self::Mathops),
            'P' => Ok(Self::Prose),
            'D' => Ok(Self::Delim),
            'X' => Ok(Self::MiscSpecial),
            // compound
            'A' => Ok(Self::Alpha),
            'S' => Ok(Self::Special),
            // invalid input
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid character set abbreviation: {}", c),
            )),
        }
    }
}

// TODO: impl as bitflags with: method for AND/OR
/// Represents a specification for a charset
///
/// Any of the predefined `CharsetName`s can be toggled and additional
/// characters may be included.
/// For this purpose, `CharsetSpec` implements `AddAssign<CharsetName>` and
/// `SubAssign<CharsetName>`.
/// Alternatively, you can parse a string containing the corresponding chars.
///
/// # Example
///
/// ```
/// let mut spec = yapg::CharsetSpec::empty();
/// spec += yapg::CharsetName::Numeric; // Adding a named charset
/// spec += "+-*"; // Adding a string
/// spec += '/'; // Adding a single char
///
/// assert_eq!(spec.construct().as_slice(), [
///     '*', '+', '-', '/', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
/// ]);
/// ```
#[derive(Debug)]
pub struct CharsetSpec {
    alpha_lower: bool,
    alpha_upper: bool,
    numeric: bool,
    mathops: bool,
    prose: bool,
    delim: bool,
    misc_special: bool,
    additions: Vec<char>,
}

impl CharsetSpec {
    /// Builds the actual character set in form of a `Vec<char>`, which is
    /// sorted and deduplicated.
    pub fn construct(mut self) -> Vec<char> {
        let mut set = vec![];
        if self.alpha_lower {
            set.append(&mut CHARSET_ALPHA_LOWER.to_vec());
        }
        if self.alpha_upper {
            set.append(&mut CHARSET_ALPHA_UPPER.to_vec());
        }
        if self.numeric {
            set.append(&mut CHARSET_NUMERIC.to_vec());
        }
        if self.mathops {
            set.append(&mut CHARSET_MATHOPS.to_vec());
        }
        if self.prose {
            set.append(&mut CHARSET_PROSE.to_vec());
        }
        if self.delim {
            set.append(&mut CHARSET_DELIM.to_vec());
        }
        if self.misc_special {
            set.append(&mut CHARSET_MISC_SPECIAL.to_vec());
        }
        set.append(&mut self.additions);
        set.sort();
        set.dedup();
        set
    }

    /// Creates the specification for an empty charset.
    ///
    /// # Example
    /// ```
    /// let charset = yapg::CharsetSpec::empty().construct();
    /// assert_eq!(charset.len(), 0);
    /// ```
    pub fn empty() -> Self {
        Self {
            alpha_lower: false,
            alpha_upper: false,
            numeric: false,
            mathops: false,
            prose: false,
            delim: false,
            misc_special: false,
            additions: vec![],
        }
    }

    /// Creates the specification for a standard charset, including all
    /// alphanumerics, `-` and `_`.
    /// Should be safe to use in most places, except for strict
    /// "no-special-characters"-policies or where neither `-` nor `_` are
    /// considered to be special, yet special chars are required.
    ///
    /// # Example
    /// ```
    /// let charset = yapg::CharsetSpec::std64().construct();
    /// assert_eq!(charset.len(), 64);
    /// ```
    pub fn std64() -> Self {
        Self {
            alpha_lower: true,
            alpha_upper: true,
            numeric: true,
            mathops: false,
            prose: false,
            delim: false,
            misc_special: false,
            additions: vec!['-', '_'],
        }
    }

    /// Creates the specification for charset that contains all printable ASCII
    /// characters.
    ///
    /// # Example
    /// ```
    /// let charset = yapg::CharsetSpec::printable_ascii().construct();
    /// assert_eq!(charset.len(), 95);
    /// ```
    pub fn printable_ascii() -> Self {
        Self {
            alpha_lower: true,
            alpha_upper: true,
            numeric: true,
            mathops: true,
            prose: true,
            delim: true,
            misc_special: true,
            additions: vec![],
        }
    }
}

impl std::str::FromStr for CharsetSpec {
    type Err = io::Error;

    fn from_str(s: &str) -> io::Result<Self> {
        let mut spec = Self::empty();
        for c in s.chars() {
            let name = CharsetName::try_from(c)?;
            spec += name;
        }
        Ok(spec)
    }
}

impl Into<Vec<char>> for CharsetSpec {
    #[inline]
    fn into(self) -> Vec<char> { self.construct() }
}

impl std::ops::AddAssign<&str> for CharsetSpec {
    #[inline]
    fn add_assign(&mut self, more: &str) {
        for c in more.chars() {
            (*self) += c;
        }
    }
}

impl std::ops::AddAssign<char> for CharsetSpec {
    #[inline]
    fn add_assign(&mut self, c: char) { self.additions.push(c); }
}

impl std::ops::AddAssign<CharsetName> for CharsetSpec {
    fn add_assign(&mut self, name: CharsetName) {
        match name {
            // atomic
            CharsetName::AlphaLower => self.alpha_lower = true,
            CharsetName::AlphaUpper => self.alpha_upper = true,
            CharsetName::Numeric => self.numeric = true,
            CharsetName::Mathops => self.mathops = true,
            CharsetName::Prose => self.prose = true,
            CharsetName::Delim => self.delim = true,
            CharsetName::MiscSpecial => self.misc_special = true,
            // compound
            CharsetName::Alpha => {
                self.alpha_lower = true;
                self.alpha_upper = true;
            },
            CharsetName::Special => {
                self.mathops = true;
                self.prose = true;
                self.delim = true;
                self.misc_special = true;
            },
        }
    }
}

impl std::ops::SubAssign<CharsetName> for CharsetSpec {
    fn sub_assign(&mut self, name: CharsetName) {
        match name {
            // atomic
            CharsetName::AlphaLower => self.alpha_lower = false,
            CharsetName::AlphaUpper => self.alpha_upper = false,
            CharsetName::Numeric => self.numeric = false,
            CharsetName::Mathops => self.mathops = false,
            CharsetName::Prose => self.prose = false,
            CharsetName::Delim => self.delim = false,
            CharsetName::MiscSpecial => self.misc_special = false,
            // compound
            CharsetName::Alpha => {
                self.alpha_lower = false;
                self.alpha_upper = false;
            },
            CharsetName::Special => {
                self.mathops = false;
                self.prose = false;
                self.delim = false;
                self.misc_special = false;
            },
        }
    }
}

// ------------------------------- unit tests ------------------------------- //
#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::CharsetName::*;
    use super::{CharsetName, CharsetSpec};

    #[test]
    fn parsing_charset_names() {
        // atomic
        assert_eq!(CharsetName::try_from('U').unwrap(), AlphaUpper);
        assert_eq!(CharsetName::try_from('L').unwrap(), AlphaLower);
        assert_eq!(CharsetName::try_from('N').unwrap(), Numeric);
        assert_eq!(CharsetName::try_from('M').unwrap(), Mathops);
        assert_eq!(CharsetName::try_from('P').unwrap(), Prose);
        assert_eq!(CharsetName::try_from('D').unwrap(), Delim);
        assert_eq!(CharsetName::try_from('X').unwrap(), MiscSpecial);
        // compound
        assert_eq!(CharsetName::try_from('A').unwrap(), Alpha);
        assert_eq!(CharsetName::try_from('S').unwrap(), Special);
        // invalid input
        assert!(CharsetName::try_from('Z').is_err());
    }

    #[test]
    fn parsing_charset_specs() {
        let (alpha, alnum) = {
            let mut alpha = [
                super::CHARSET_ALPHA_UPPER.to_vec(),
                super::CHARSET_ALPHA_LOWER.to_vec(),
            ]
            .concat();
            let mut alnum = [
                super::CHARSET_ALPHA_UPPER.to_vec(),
                super::CHARSET_ALPHA_LOWER.to_vec(),
                super::CHARSET_NUMERIC.to_vec(),
            ]
            .concat();
            alpha.sort();
            alnum.sort();
            (alpha, alnum)
        };

        assert_eq!("LU".parse::<CharsetSpec>().unwrap().construct(), alpha);
        assert_eq!("LUN".parse::<CharsetSpec>().unwrap().construct(), alnum);
    }

    #[test]
    fn adding_charset_to_spec() {
        let mut spec = CharsetSpec::empty();
        spec += Mathops;
        assert_eq!(spec.construct(), vec!['*', '+', '-', '/', '<', '=', '>'])
    }

    #[test]
    fn subtracting_charset_from_spec() {
        let mut spec = CharsetSpec::std64();
        spec -= Alpha;
        spec -= Numeric;
        assert_eq!(spec.construct(), vec!['-', '_'])
    }

    #[test]
    fn adding_chars_to_spec() {
        let mut spec = CharsetSpec::empty();
        spec += 'a';
        spec += 'b';
        spec += 'c';
        spec += 'd';
        assert_eq!(spec.construct(), vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn adding_strings_to_spec() {
        let mut spec = CharsetSpec::empty();
        spec += "abcd";
        assert_eq!(spec.construct(), vec!['a', 'b', 'c', 'd']);
    }
}
