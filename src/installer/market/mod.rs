use core::{borrow::Borrow, fmt, str::FromStr};

use compact_str::CompactString;
use heapless::String;
pub use markets::{Markets, MarketsError};
use thiserror::Error;

mod markets;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
#[repr(transparent)]
pub struct Market(String<2>);

#[derive(Error, Debug, Eq, PartialEq)]
#[error(
    "Market must be exactly {} ASCII uppercase characters long",
    Market::LEN
)]
pub enum MarketError {
    InvalidLength,
    InvalidCharacter,
}

impl Market {
    const LEN: usize = 2;

    /// Creates a new `Market` if the value has exactly 2 ASCII uppercase characters.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the value is not exactly 2 ASCII characters.
    pub fn new<T: AsRef<str>>(market: T) -> Result<Self, MarketError> {
        let market = market
            .as_ref()
            .parse::<String<{ Self::LEN }>>()
            .map_err(|()| MarketError::InvalidLength)?;

        if market.len() != Self::LEN {
            return Err(MarketError::InvalidLength);
        }

        if !market.as_bytes().iter().all(u8::is_ascii_uppercase) {
            return Err(MarketError::InvalidCharacter);
        }

        Ok(Self(market))
    }

    /// Create a new `Market` without checking whether the value has exactly 2 ASCII uppercase
    /// characters. This results in undefined behaviour if the value is not exactly 2 ASCII
    /// uppercase characters.
    ///
    /// # Safety
    ///
    /// The value must be exactly 2 ASCII uppercase characters.
    pub unsafe fn new_unchecked<T: AsRef<str>>(market: T) -> Self {
        Self(unsafe {
            market
                .as_ref()
                .parse::<String<{ Self::LEN }>>()
                .unwrap_unchecked()
        })
    }

    /// Extracts a string slice containing the entire `Market`.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::Market;
    ///
    /// let market = Market::new("US").unwrap();
    ///
    /// assert_eq!("US", market.as_str())
    /// ```
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Market {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for Market {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Market {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Market {
    type Err = MarketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for Market {
    type Error = MarketError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
