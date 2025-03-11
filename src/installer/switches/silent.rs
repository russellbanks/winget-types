use core::{
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use compact_str::CompactString;

use super::switch::{InstallerSwitch, SwitchError};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SilentSwitch(InstallerSwitch<512>);

impl SilentSwitch {
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<CompactString> {
        self.0.iter()
    }
}

impl Deref for SilentSwitch {
    type Target = InstallerSwitch<512>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SilentSwitch {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for SilentSwitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for SilentSwitch {
    type Err = SwitchError<512>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InstallerSwitch::<512>::from_str(s).map(Self)
    }
}

impl IntoIterator for SilentSwitch {
    type Item = CompactString;

    type IntoIter = smallvec::IntoIter<[CompactString; 2]>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'switch> IntoIterator for &'switch SilentSwitch {
    type Item = &'switch CompactString;

    type IntoIter = core::slice::Iter<'switch, CompactString>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
