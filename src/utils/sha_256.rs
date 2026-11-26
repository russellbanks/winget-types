use core::{fmt, str::FromStr};

use compact_str::CompactString;
use heapless::String;
use sha2::{Sha256, digest::Output};
use thiserror::Error;

// 256 bits / 4 bits per hex character
const SHA256_LEN: usize = 256 / 0xF_u8.count_ones() as usize;

/// A SHA-256 hash as 64 uppercase hexadecimal characters.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
pub struct Sha256String(String<SHA256_LEN>);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum Sha256StringError {
    #[error("SHA256 hash must be exactly {SHA256_LEN} characters long but has {_0}")]
    InvalidLength(usize),
    #[error("SHA256 hash contains non-hexadecimal character {_0:?}")]
    InvalidCharacter(char),
}

impl Sha256String {
    /// Creates a new `Sha256String` from a string of 64 hexadecimal characters.
    ///
    /// Lowercase hexadecimal characters are accepted and normalized to uppercase.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the value contains a non-hexadecimal character or is not exactly 64
    /// characters long.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::Sha256String;
    /// # use winget_types::Sha256StringError;
    ///
    /// # fn main() -> Result<(), Sha256StringError> {
    /// let hash = Sha256String::new(
    ///     "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
    /// )?;
    ///
    /// assert_eq!(
    ///     hash.as_str(),
    ///     "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str>>(hash: T) -> Result<Self, Sha256StringError> {
        let hash = hash.as_ref();

        if let Some(invalid) = hash.chars().find(|char| !char.is_ascii_hexdigit()) {
            return Err(Sha256StringError::InvalidCharacter(invalid));
        }

        // Every character is ASCII at this point, so the byte length is the character length
        let mut hash = hash
            .parse::<String<SHA256_LEN>>()
            .ok()
            .filter(|hash| hash.len() == SHA256_LEN)
            .ok_or(Sha256StringError::InvalidLength(hash.len()))?;

        hash.make_ascii_uppercase();

        Ok(Self(hash))
    }

    /// Creates a `Sha256String` from a Sha256 digest.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::{sha2::{Digest, Sha256}, Sha256String};
    ///
    /// // Digest some data manually
    /// let sha256_digest = Sha256::digest("abc");
    ///
    /// assert_eq!(
    ///     Sha256String::from_digest(&sha256_digest).as_str(),
    ///     "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD"
    /// );
    /// ```
    #[must_use]
    pub fn from_digest(digest: &Output<Sha256>) -> Self {
        let mut encode_buf = [0; SHA256_LEN];

        Self(
            base16ct::upper::encode_str(digest, &mut encode_buf)
                .unwrap_or_else(|_| unreachable!("SHA256 digests should always be 32 bytes long"))
                .parse::<String<SHA256_LEN>>()
                .unwrap_or_else(|_| {
                    unreachable!("Sha256 hashes should always be {SHA256_LEN} bytes long")
                }),
        )
    }

    /// Creates a `Sha256String` by hashing data from a reader.
    ///
    /// This will repeatedly read the data into a buffer of length 4096.
    ///
    /// # Errors
    ///
    /// Returns the propagated `Err` from [`io::read`].
    ///
    /// # Examples
    ///
    /// [`File`]s implement `Read`:
    ///
    /// [`File`]: std::fs::File
    /// [`io::read`]: std::io::Read::read
    ///
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    ///
    /// use winget_types::Sha256String;
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut f = File::open("foo.txt")?;
    ///
    ///     let sha256_string = Sha256String::hash_from_reader(f)?;
    ///
    ///     println!("File SHA256 hash: {sha256_string}");
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "std")]
    pub fn hash_from_reader<R: std::io::Read>(mut reader: R) -> std::io::Result<Self> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        let mut buffer = [0; 1 << 12];

        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }

        Ok(Self::from_digest(&hasher.finalize()))
    }

    /// Extracts a string slice containing the entire `Sha256String`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Default for Sha256String {
    fn default() -> Self {
        Self(core::iter::repeat_n('0', SHA256_LEN).collect::<_>())
    }
}

impl FromStr for Sha256String {
    type Err = Sha256StringError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for Sha256String {
    type Error = Sha256StringError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for Sha256String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use rstest::rstest;

    use super::Sha256String;

    const VALID: &str = "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD";

    #[rstest]
    #[case::too_short("abc")]
    #[case::empty("''")]
    #[case::non_hex("ZZ7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD")]
    #[case::one_short("A7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD")]
    fn deserialize_rejects_invalid(#[case] input: &str) {
        assert!(serde_yaml::from_str::<Sha256String>(input).is_err());
    }

    #[test]
    fn deserialize_normalizes_lowercase_to_uppercase() {
        let hash = serde_yaml::from_str::<Sha256String>(&VALID.to_ascii_lowercase()).unwrap();
        assert_eq!(hash.as_str(), VALID);
    }

    #[test]
    fn serialize_round_trip() {
        let hash = serde_yaml::from_str::<Sha256String>(VALID).unwrap();
        assert_eq!(serde_yaml::to_string(&hash).unwrap().trim_end(), VALID);
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::{SHA256_LEN, Sha256String, Sha256StringError};

    const VALID: &str = "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD";

    #[test]
    fn parse_valid_uppercase() {
        assert_eq!(VALID.parse::<Sha256String>().unwrap().as_str(), VALID);
    }

    #[test]
    fn parse_normalizes_lowercase() {
        let hash = Sha256String::new(VALID.to_ascii_lowercase()).unwrap();
        assert_eq!(hash.as_str(), VALID);
    }

    #[test]
    fn parse_wrong_length() {
        assert_eq!(
            "ABC".parse::<Sha256String>(),
            Err(Sha256StringError::InvalidLength(3))
        );
        assert_eq!(
            "A".repeat(SHA256_LEN + 1).parse::<Sha256String>(),
            Err(Sha256StringError::InvalidLength(SHA256_LEN + 1))
        );
    }

    #[test]
    fn parse_non_hex_character() {
        let input = VALID.replacen('B', "G", 1);
        assert_eq!(
            input.parse::<Sha256String>(),
            Err(Sha256StringError::InvalidCharacter('G'))
        );
    }

    #[test]
    fn parse_multibyte_character_reports_char_not_byte_length() {
        // 63 ASCII hex chars + one 2-byte char = 65 bytes but 64 chars
        let input = format!("{}é", &VALID[..SHA256_LEN - 1]);
        assert_eq!(
            input.parse::<Sha256String>(),
            Err(Sha256StringError::InvalidCharacter('é'))
        );
    }

    #[test]
    fn default_is_valid() {
        let default = Sha256String::default();
        assert_eq!(Sha256String::new(default.as_str()), Ok(default));
    }
}
