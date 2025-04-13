use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

use super::DISALLOWED_CHARACTERS;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
#[repr(transparent)]
pub struct PackageIdentifier(CompactString);

#[derive(Error, Debug, Eq, PartialEq)]
pub enum PackageIdentifierError {
    #[error("Package identifier cannot be empty")]
    Empty,
    #[error("A part of a package identifier cannot be empty")]
    EmptyPart,
    #[error(
        "Package identifier cannot be more than {} characters long",
        PackageIdentifier::MAX_CHAR_LENGTH
    )]
    TooLong,
    #[error("Package identifier contains invalid character {_0:?}")]
    InvalidCharacter(char),
    #[error(
        "The length of a part in a package identifier cannot be more than {} characters long",
        PackageIdentifier::MAX_PART_CHAR_LENGTH
    )]
    PartTooLong,
    #[error(
        "The number of parts in the package identifier must be between {} and {}",
        PackageIdentifier::MIN_PARTS,
        PackageIdentifier::MAX_PARTS
    )]
    InvalidPartCount,
}

impl PackageIdentifier {
    pub const MAX_CHAR_LENGTH: usize = 128;
    pub const MIN_PARTS: usize = 2;
    pub const MAX_PARTS: usize = 8;
    pub const MAX_PART_CHAR_LENGTH: usize = 32;

    /// Creates a new `PackageIdentifier` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the package identifier:
    /// 1. Is empty
    /// 2. Has an empty part
    /// 3. Is more than 128 characters long
    /// 4. Has a part more than 32 characters long
    /// 5. Contains a disallowed character (control, whitespace, or one of [`DISALLOWED_CHARACTERS`])
    pub fn new<T: AsRef<str> + Into<CompactString>>(
        identifier: T,
    ) -> Result<Self, PackageIdentifierError> {
        let identifier_str = identifier.as_ref();

        if identifier_str.is_empty() {
            return Err(PackageIdentifierError::Empty);
        }

        let (char_count, parts_count) = identifier_str.split('.').try_fold(
            (0, 0),
            |(total_char_count, part_count), part| {
                if part.is_empty() {
                    return Err(PackageIdentifierError::EmptyPart);
                }

                let part_char_count = part.chars().try_fold(0, |char_count, char| {
                    if DISALLOWED_CHARACTERS.contains(&char)
                        || char.is_control()
                        || char.is_whitespace()
                    {
                        return Err(PackageIdentifierError::InvalidCharacter(char));
                    }

                    Ok(char_count + 1)
                })?;

                if part_char_count > Self::MAX_PART_CHAR_LENGTH {
                    return Err(PackageIdentifierError::PartTooLong);
                }

                Ok((
                    total_char_count + part_char_count + '.'.len_utf8(),
                    part_count + 1,
                ))
            },
        )?;

        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(PackageIdentifierError::TooLong);
        }

        if !(Self::MIN_PARTS..=Self::MAX_PARTS).contains(&parts_count) {
            return Err(PackageIdentifierError::InvalidPartCount);
        }

        Ok(Self(identifier.into()))
    }

    /// Creates a new `PackageIdentifier` from any type that implements `Into<CompactString>`
    /// without checking its validity.
    ///
    /// # Safety
    ///
    /// The package identifier must not:
    /// 1. Be empty
    /// 2. Have an empty part
    /// 3. Be more than 128 characters long
    /// 4. Have a part more than 32 characters long
    /// 5. Contain a disallowed character (control, whitespace, or one of [`DISALLOWED_CHARACTERS`])
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(identifier: T) -> Self {
        Self(identifier.into())
    }

    /// Extracts a string slice containing the entire `PackageIdentifier`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for PackageIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for PackageIdentifier {
    type Err = PackageIdentifierError;

    fn from_str(s: &str) -> Result<Self, PackageIdentifierError> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for PackageIdentifier {
    type Error = PackageIdentifierError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{format, string::String};
    use core::iter::repeat_n;

    #[cfg(feature = "serde")]
    use indoc::indoc;
    use rstest::rstest;

    use crate::shared::{
        DISALLOWED_CHARACTERS,
        package_identifier::{PackageIdentifier, PackageIdentifierError},
    };

    #[rstest]
    #[case("Package.Identifier")]
    #[case("Microsoft.PowerShell")]
    #[case("Google.Chrome.Canary")]
    #[case("EclipseAdoptium.Temurin.21.JDK")]
    #[case("A.Long.Package.Identifier.With.Exactly.Eight.Parts")]
    fn valid_package_identifier(#[case] package_identifier: &str) {
        assert!(package_identifier.parse::<PackageIdentifier>().is_ok());
    }

    #[test]
    fn too_long_package_identifier() {
        let num_delimiters = PackageIdentifier::MAX_PARTS - 1;
        let part_length = (PackageIdentifier::MAX_CHAR_LENGTH - num_delimiters)
            .div_ceil(PackageIdentifier::MAX_PARTS);

        let part = "a".repeat(part_length);

        let identifier =
            itertools::intersperse(repeat_n(&*part, PackageIdentifier::MAX_PARTS), ".")
                .collect::<String>();

        assert_eq!(
            identifier.parse::<PackageIdentifier>(),
            Err(PackageIdentifierError::TooLong)
        );
    }

    #[test]
    fn too_many_parts_package_identifier() {
        assert_eq!(
            itertools::intersperse(repeat_n('a', PackageIdentifier::MAX_PARTS + 1), '.')
                .collect::<String>()
                .parse::<PackageIdentifier>(),
            Err(PackageIdentifierError::InvalidPartCount)
        );

        assert_eq!(
            "Really.Long.Package.Identifier.Spanning.More.Than.Eight.Parts"
                .parse::<PackageIdentifier>(),
            Err(PackageIdentifierError::InvalidPartCount)
        );
    }

    #[test]
    fn package_identifier_parts_too_long() {
        let part = "a".repeat(PackageIdentifier::MAX_PART_CHAR_LENGTH + 1);

        let identifier =
            itertools::intersperse(repeat_n(&*part, PackageIdentifier::MIN_PARTS), ".")
                .collect::<String>();

        assert_eq!(
            identifier.parse::<PackageIdentifier>(),
            Err(PackageIdentifierError::PartTooLong)
        );
    }

    #[test]
    fn too_few_parts_package_identifier() {
        assert_eq!(
            "a".repeat(PackageIdentifier::MIN_PARTS - 1)
                .parse::<PackageIdentifier>(),
            Err(PackageIdentifierError::InvalidPartCount)
        );

        assert_eq!(
            "OnePart".parse::<PackageIdentifier>(),
            Err(PackageIdentifierError::InvalidPartCount)
        );
    }

    #[test]
    fn whitespace_in_package_identifier() {
        assert_eq!(
            "Publisher.Pack age".parse::<PackageIdentifier>(),
            Err(PackageIdentifierError::InvalidCharacter(' '))
        );
    }

    #[test]
    fn control_chars_in_package_identifier() {
        for char in '\u{0}'..='\u{1F}' {
            assert_eq!(
                format!("Publisher.Pack{char}age").parse::<PackageIdentifier>(),
                Err(PackageIdentifierError::InvalidCharacter(char))
            );
        }
    }

    #[test]
    fn package_identifier_disallowed_characters() {
        for char in DISALLOWED_CHARACTERS {
            let identifier = format!("Publisher.Pack{char}age");

            assert_eq!(
                identifier.parse::<PackageIdentifier>(),
                Err(PackageIdentifierError::InvalidCharacter(char))
            );
        }
    }

    #[test]
    fn package_identifier_part_empty() {
        assert!("a.b".parse::<PackageIdentifier>().is_ok());
        assert_eq!(
            "a.b.".parse::<PackageIdentifier>(),
            Err(PackageIdentifierError::EmptyPart)
        );
        assert_eq!(
            "a..b".parse::<PackageIdentifier>(),
            Err(PackageIdentifierError::EmptyPart)
        );
    }

    #[cfg(feature = "serde")]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Manifest {
        package_identifier: PackageIdentifier,
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_package_identifier() {
        assert_eq!(
            serde_yaml::to_string(&Manifest {
                package_identifier: "Microsoft.PowerShell".parse().unwrap()
            })
            .unwrap(),
            indoc! {"
                PackageIdentifier: Microsoft.PowerShell
            "}
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_package_identifier() {
        assert_eq!(
            serde_yaml::from_str::<Manifest>(indoc! {"
                PackageIdentifier: Microsoft.PowerShell
            "})
            .unwrap()
            .package_identifier,
            "Microsoft.PowerShell".parse::<PackageIdentifier>().unwrap()
        );
    }
}
