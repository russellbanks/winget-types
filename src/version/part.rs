use core::{cmp::Ordering, convert::Infallible, str::FromStr};

use compact_str::CompactString;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VersionPart {
    pub number: u64,
    pub supplement: CompactString,
}

impl VersionPart {
    pub const DEFAULT: Self = Self {
        number: 0,
        supplement: CompactString::const_new(""),
    };

    #[inline]
    pub fn new<T: Into<CompactString>>(number: u64, supplement: T) -> Self {
        Self {
            number,
            supplement: supplement.into(),
        }
    }

    /// Returns `true` if this version part has a numeric value of `0` and no supplemental value.
    /// In other words, it represents just `.0`.
    ///
    /// `WinGet` ignores trailing `.0` parts in versions.
    #[inline]
    pub fn is_droppable(&self) -> bool {
        self.number == 0 && self.supplement.is_empty()
    }
}

impl Default for VersionPart {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl From<&str> for VersionPart {
    fn from(value: &str) -> Self {
        let part = value.trim();

        let supplement_start_index = part
            .find(|char: char| !char.is_ascii_digit())
            .unwrap_or(part.len());

        let (number_str, supplement) = part.split_at(supplement_start_index);

        let number = number_str.parse().unwrap_or_default();

        Self::new(number, supplement)
    }
}

impl FromStr for VersionPart {
    type Err = Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl PartialOrd for VersionPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionPart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number).then_with(|| {
            match (self.supplement.as_str(), other.supplement.as_str()) {
                ("", "") => Ordering::Equal,
                ("", _) => Ordering::Greater,
                (_, "") => Ordering::Less,
                (supplement, other_supplement) => supplement.cmp(other_supplement),
            }
        })
    }
}
