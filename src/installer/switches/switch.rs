use core::{fmt, str::FromStr};

use compact_str::CompactString;
use smallvec::SmallVec;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InstallerSwitch<const N: usize>(SmallVec<[CompactString; 2]>);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SwitchError<const N: usize> {
    #[error("Switch cannot be empty")]
    Empty,
    #[error("Switch cannot be more than {N} characters long")]
    TooLong,
}

impl<const N: usize> InstallerSwitch<N> {
    pub const MAX_CHAR_LENGTH: usize = N;

    const DELIMITERS: [char; 2] = [',', ' '];

    pub fn push<S: Into<CompactString>>(&mut self, other: S) {
        self.0.push(other.into());
    }

    pub fn contains<S: AsRef<str>>(&self, other: S) -> bool {
        self.0
            .iter()
            .any(|this| this.eq_ignore_ascii_case(other.as_ref()))
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, CompactString> {
        self.0.iter()
    }
}

impl<const N: usize> fmt::Display for InstallerSwitch<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for part in itertools::intersperse(self.0.iter().map(CompactString::as_str), " ") {
            f.write_str(part)?;
        }
        Ok(())
    }
}

impl<const N: usize> FromStr for InstallerSwitch<N> {
    type Err = SwitchError<N>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(SwitchError::Empty)
        } else if s.chars().count() > N {
            Err(SwitchError::TooLong)
        } else {
            Ok(Self(
                s.split(Self::DELIMITERS)
                    .filter(|switch| !switch.is_empty())
                    .map(CompactString::from)
                    .collect::<SmallVec<_>>(),
            ))
        }
    }
}

impl<const N: usize> IntoIterator for InstallerSwitch<N> {
    type Item = CompactString;

    type IntoIter = smallvec::IntoIter<[CompactString; 2]>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'switch, const N: usize> IntoIterator for &'switch InstallerSwitch<N> {
    type Item = &'switch CompactString;

    type IntoIter = core::slice::Iter<'switch, CompactString>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for InstallerSwitch<N>
where
    Self: fmt::Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for InstallerSwitch<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Helper<S>(core::marker::PhantomData<S>);

        impl<S> serde::de::Visitor<'_> for Helper<S>
        where
            S: FromStr,
            <S as FromStr>::Err: fmt::Display,
        {
            type Value = S;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                value.parse::<Self::Value>().map_err(E::custom)
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let utf8 = core::str::from_utf8(value).map_err(E::custom)?;
                self.visit_str(utf8)
            }
        }

        deserializer.deserialize_str(Helper(core::marker::PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use alloc::{borrow::ToOwned, format, string::ToString};

    use smallvec::{SmallVec, smallvec};

    use crate::installer::switches::{log::LogSwitch, switch::SwitchError};

    #[test]
    fn empty_custom_switch() {
        assert_eq!("".parse::<LogSwitch>().err().unwrap(), SwitchError::Empty);
    }

    #[test]
    fn unicode_custom_switch_max_length() {
        let custom_switch = "🦀".repeat(LogSwitch::MAX_CHAR_LENGTH);

        // Ensure that it's character length that's being checked and not byte or UTF-16 length
        assert!(custom_switch.len() > LogSwitch::MAX_CHAR_LENGTH);
        assert!(custom_switch.encode_utf16().count() > LogSwitch::MAX_CHAR_LENGTH);
        assert_eq!(custom_switch.chars().count(), LogSwitch::MAX_CHAR_LENGTH);
        assert!(custom_switch.parse::<LogSwitch>().is_ok());
    }

    #[test]
    fn custom_switch_too_long() {
        let custom_switch = "a".repeat(LogSwitch::MAX_CHAR_LENGTH + 1);

        assert_eq!(
            custom_switch.parse::<LogSwitch>().err(),
            Some(SwitchError::TooLong)
        );
    }

    #[test]
    fn delimited_custom_switches_internal_representation() {
        let switches: SmallVec<[_; 2]> = smallvec!["/ALLUSERS".to_owned(), "/NoRestart".to_owned()];

        assert_eq!(switches.join(" ").parse::<LogSwitch>().unwrap().0, switches);

        assert_eq!(
            switches.join(", ").parse::<LogSwitch>().unwrap().0,
            switches
        );
    }

    #[test]
    fn custom_switch_to_string() {
        const CUSTOM_SWITCH: &str = "/ALLUSERS, /NoRestart, , -NoRestart";

        assert_eq!(
            CUSTOM_SWITCH.parse::<LogSwitch>().unwrap().to_string(),
            "/ALLUSERS /NoRestart -NoRestart"
        );
    }

    #[test]
    fn custom_switch_contains() {
        const ALL_USERS: &str = "/ALLUSERS";

        let all_users_switch = ALL_USERS.parse::<LogSwitch>().unwrap();

        assert!(all_users_switch.contains(ALL_USERS));
        assert!(all_users_switch.contains(&ALL_USERS.to_ascii_lowercase()))
    }

    #[test]
    fn append_custom_switch() {
        const ALL_USERS: &str = "/ALLUSERS";
        const NO_RESTART: &str = "/NoRestart";

        let mut custom_switch = ALL_USERS.parse::<LogSwitch>().unwrap();

        custom_switch.push(NO_RESTART);

        assert!(custom_switch.contains(NO_RESTART));

        assert_eq!(
            custom_switch.to_string(),
            format!("{ALL_USERS} {NO_RESTART}")
        );
    }
}
