mod apps_and_features_entry;

use alloc::{vec, vec::Vec};
use core::slice;

pub use apps_and_features_entry::AppsAndFeaturesEntry;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct AppsAndFeaturesEntries(Vec<AppsAndFeaturesEntry>);

impl AppsAndFeaturesEntries {
    /// Constructs a new, empty `AppsAndFeaturesEntries`.
    ///
    /// The collection will not allocate until elements are pushed onto it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use winget_types::installer::AppsAndFeaturesEntries;
    /// let mut apps_and_features_entries = AppsAndFeaturesEntries::new();
    /// ```
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Consumes `self`, returning the inner `Vec<AppsAndFeaturesEntry>`.
    #[must_use]
    #[inline]
    pub fn into_inner(self) -> Vec<AppsAndFeaturesEntry> {
        self.0
    }

    /// Appends an [`AppsAndFeaturesEntry`] to the back of the apps and features entries.
    #[inline]
    pub fn push(&mut self, entry: AppsAndFeaturesEntry) {
        self.0.push(entry);
    }

    /// Clears the apps and features entries, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity of the apps and features
    /// entries.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns the number of non-empty [`AppsAndFeaturesEntry`] in the apps and features entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0
            .iter()
            .filter(|apps_and_features_entry| !apps_and_features_entry.is_empty())
            .count()
    }

    /// Returns `true` if there are no apps and features entries or all of the apps and features
    /// entries are empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use winget_types::installer::{AppsAndFeaturesEntries, AppsAndFeaturesEntry};
    /// let mut apps_and_features_entries = AppsAndFeaturesEntries::new();
    /// assert!(apps_and_features_entries.is_empty());
    ///
    /// apps_and_features_entries.push(AppsAndFeaturesEntry::default());
    /// assert!(apps_and_features_entries.is_empty());
    ///
    /// apps_and_features_entries.push(
    ///     AppsAndFeaturesEntry::builder()
    ///         .publisher("Microsoft")
    ///         .build()
    /// );
    /// assert!(!apps_and_features_entries.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(AppsAndFeaturesEntry::is_empty)
    }

    /// Returns an iterator over the apps and features entries.
    ///
    /// The iterator yields all items from start to end.
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, AppsAndFeaturesEntry> {
        self.into_iter()
    }

    /// Returns an iterator that allows modifying each apps and features entry.
    ///
    /// The iterator yields all items from start to end.
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, AppsAndFeaturesEntry> {
        self.into_iter()
    }
}

impl<T> From<T> for AppsAndFeaturesEntries
where
    T: Into<Vec<AppsAndFeaturesEntry>>,
{
    /// Creates a new `AppsAndFeaturesEntries` from any type that implements
    /// `Into<Vec<AppsAndFeaturesEntry>>`.
    #[inline]
    fn from(apps_and_features_entries: T) -> Self {
        Self(apps_and_features_entries.into())
    }
}

impl From<AppsAndFeaturesEntry> for AppsAndFeaturesEntries {
    /// Creates a new `AppsAndFeaturesEntries` from a single `AppsAndFeaturesEntry`.
    #[inline]
    fn from(apps_and_features_entries: AppsAndFeaturesEntry) -> Self {
        Self(vec![apps_and_features_entries])
    }
}

impl FromIterator<AppsAndFeaturesEntry> for AppsAndFeaturesEntries {
    fn from_iter<I: IntoIterator<Item = AppsAndFeaturesEntry>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for AppsAndFeaturesEntries {
    type Item = AppsAndFeaturesEntry;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a AppsAndFeaturesEntries {
    type Item = &'a AppsAndFeaturesEntry;

    type IntoIter = slice::Iter<'a, AppsAndFeaturesEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut AppsAndFeaturesEntries {
    type Item = &'a mut AppsAndFeaturesEntry;

    type IntoIter = slice::IterMut<'a, AppsAndFeaturesEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
