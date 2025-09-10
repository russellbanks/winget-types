use core::fmt;

use heapless::String;
use sha2::{Sha256, digest::Output};

// 256 bits / 4 bits per hex character
const SHA256_LEN: usize = 256 / 0xF_u8.count_ones() as usize;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sha256String(String<SHA256_LEN>);

impl Sha256String {
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

impl fmt::Display for Sha256String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
