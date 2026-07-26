use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

/// A license governing the use and or distribution for a product.
///
/// Where available, [`SPDX`] short identifiers are preferred.
///
/// [`SPDX`]: https://spdx.org/licenses/
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
#[repr(transparent)]
pub struct License(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum LicenseError {
    #[error(
        "License must have at least {} characters but has {_0}",
        License::MIN_CHAR_LENGTH
    )]
    TooShort(usize),
    #[error(
        "License must not have more than {} characters but has {_0}",
        License::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl License {
    pub const MIN_CHAR_LENGTH: usize = 3;
    pub const MAX_CHAR_LENGTH: usize = 512;

    /// Represents a proprietary license.
    ///
    /// Use this when the license information for a package is unknown or not publicly available.
    /// Proprietary licenses typically restrict usage, modification, and redistribution.
    pub const PROPRIETARY: Self = Self(CompactString::const_new("Proprietary"));

    /// <https://spdx.org/licenses/Apache-1.0.html>
    pub const APACHE_V1: Self = Self(CompactString::const_new("Apache-1.0"));

    /// <https://spdx.org/licenses/Apache-1.1.html>
    pub const APACHE_V1_1: Self = Self(CompactString::const_new("Apache-1.1"));

    /// <https://spdx.org/licenses/Apache-2.0.html>
    pub const APACHE_V2: Self = Self(CompactString::const_new("Apache-2.0"));

    /// <https://spdx.org/licenses/BSD-3-Clause.html>
    pub const BSD_V3_CLAUSE_NEW_OR_REVISED: Self = Self(CompactString::const_new("BSD-3-Clause"));

    /// <https://spdx.org/licenses/AGPL-1.0-only.html>
    pub const AGPL_V1_ONLY: Self = Self(CompactString::const_new("AGPL-1.0-only"));

    /// <https://spdx.org/licenses/AGPL-1.0-or-later.html>
    pub const AGPL_V1_OR_LATER: Self = Self(CompactString::const_new("AGPL-1.0-or-later"));

    /// <https://spdx.org/licenses/AGPL-3.0-only.html>
    pub const AGPL_V3_ONLY: Self = Self(CompactString::const_new("AGPL-3.0-only"));

    /// <https://spdx.org/licenses/AGPL-3.0-or-later.html>
    pub const AGPL_V3_OR_LATER: Self = Self(CompactString::const_new("AGPL-3.0-or-later"));

    /// <https://spdx.org/licenses/GPL-1.0-only.html>
    pub const GPL_V1_ONLY: Self = Self(CompactString::const_new("GPL-1.0-only"));

    /// <https://spdx.org/licenses/GPL-1.0-or-later.html>
    pub const GPL_V1_OR_LATER: Self = Self(CompactString::const_new("GPL-1.0-or-later"));

    /// <https://spdx.org/licenses/GPL-2.0-only.html>
    pub const GPL_V2_ONLY: Self = Self(CompactString::const_new("GPL-2.0-only"));

    /// <https://spdx.org/licenses/GPL-2.0-or-later.html>
    pub const GPL_V2_OR_LATER: Self = Self(CompactString::const_new("GPL-2.0-or-later"));

    /// <https://spdx.org/licenses/GPL-3.0-only.html>
    pub const GPL_V3_ONLY: Self = Self(CompactString::const_new("GPL-3.0-only"));

    /// <https://spdx.org/licenses/GPL-3.0-or-later.html>
    pub const GPL_V3_OR_LATER: Self = Self(CompactString::const_new("GPL-3.0-or-later"));

    /// <https://spdx.org/licenses/MIT.html>
    pub const MIT: Self = Self(CompactString::const_new("MIT"));

    /// Creates a new `License` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the license is less than 3 characters long or more than 512 characters
    /// long.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::locale::License;
    /// # use winget_types::locale::LicenseError;
    ///
    /// # fn main() -> Result<(), LicenseError>  {
    /// let license = License::new("MIT")?;
    ///
    /// assert_eq!(license.as_str(), "MIT");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str> + Into<CompactString>>(license: T) -> Result<Self, LicenseError> {
        match license.as_ref().chars().count() {
            count if count < Self::MIN_CHAR_LENGTH => Err(LicenseError::TooShort(count)),
            count if count > Self::MAX_CHAR_LENGTH => Err(LicenseError::TooLong(count)),
            _ => Ok(Self(license.into())),
        }
    }

    /// Creates a new `License` from any type that implements `Into<CompactString>` without checking
    /// its validity.
    ///
    /// # Safety
    ///
    /// The license must not be less than 3 characters long or more than 512 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(license: T) -> Self {
        Self(license.into())
    }

    /// Extracts a string slice containing the entire `License`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for License {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Default for License {
    fn default() -> Self {
        Self::PROPRIETARY
    }
}

impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for License {
    type Err = LicenseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for License {
    type Error = LicenseError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
