use core::{
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use compact_str::CompactString;

use super::switch::{InstallerSwitch, SwitchError};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CustomSwitch(InstallerSwitch<2048>);

impl CustomSwitch {
    #[must_use]
    pub fn all_users() -> Self {
        "/ALLUSERS".parse().unwrap_or_else(|_| unreachable!())
    }

    #[must_use]
    pub fn current_user() -> Self {
        "/CURRENTUSER".parse().unwrap_or_else(|_| unreachable!())
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, CompactString> {
        self.0.iter()
    }
}

impl Deref for CustomSwitch {
    type Target = InstallerSwitch<2048>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CustomSwitch {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for CustomSwitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for CustomSwitch {
    type Err = SwitchError<2048>;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        InstallerSwitch::<2048>::from_str(src).map(Self)
    }
}

impl IntoIterator for CustomSwitch {
    type Item = CompactString;

    type IntoIter = smallvec::IntoIter<[CompactString; 2]>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'switch> IntoIterator for &'switch CustomSwitch {
    type Item = &'switch CompactString;

    type IntoIter = core::slice::Iter<'switch, CompactString>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
