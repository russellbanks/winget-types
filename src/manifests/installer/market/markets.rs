use alloc::collections::BTreeSet;
use core::{borrow::Borrow, fmt::Debug};

use thiserror::Error;

use super::{Market, MarketError};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Markets {
    /// Any markets a package may be installed in.
    Allowed(BTreeSet<Market>),

    /// Any markets a package may not be installed in.
    Excluded(BTreeSet<Market>),
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum MarketsError {
    #[error("Markets may not contain more than {} markets", Markets::MAX_ITEMS)]
    TooManyMarkets,

    #[error(transparent)]
    Market(#[from] MarketError),
}

impl Markets {
    pub const MAX_ITEMS: usize = 256;

    /// Makes a new, empty, allowed `Markets`.
    ///
    /// Does not allocate anything on its own.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use winget_types::installer::Markets;
    ///
    /// let mut markets = Markets::new_allowed();
    /// ```
    #[must_use]
    pub const fn new_allowed() -> Self {
        Self::Allowed(BTreeSet::new())
    }

    /// Creates a new allowed `Markets` from an iterator of any type that implements `AsRef<str`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if any of the iterator items are not exactly 2 ASCII uppercase characters
    /// or if the number of unique markets exceeds 256.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::{Market, Markets};
    ///
    /// let markets = Markets::allowed_from_iter(["US", "UK"]).unwrap();
    ///
    /// let v: Vec<_> = markets.into_iter().collect();
    /// assert_eq!(v, [Market::new("UK").unwrap(), Market::new("US").unwrap()]);
    /// ```
    pub fn allowed_from_iter<I, T>(markets: I) -> Result<Self, MarketsError>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let markets = markets
            .into_iter()
            .map(|market| Market::new(market.as_ref()))
            .collect::<Result<BTreeSet<_>, MarketError>>()?;

        if markets.len() > Self::MAX_ITEMS {
            return Err(MarketsError::TooManyMarkets);
        }

        Ok(Self::Allowed(markets))
    }

    /// Creates a new allowed `Markets` from an iterator of any type that implements `AsRef<str`
    /// without checking whether each item is exactly 2 ASCII uppercase characters or if the number
    /// of unique markets exceeds 256. This results in undefined behaviour if any item is not
    /// exactly 2 ASCII uppercase characters or the number of unique markets exceeds 256.
    ///
    /// # Safety
    ///
    /// Each item must be exactly 2 ASCII uppercase characters and the number of unique markets must
    /// not exceed 256.
    pub unsafe fn allowed_from_iter_unchecked<I, T>(markets: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let markets = markets
            .into_iter()
            .map(|market| unsafe { Market::new_unchecked(market.as_ref()) })
            .collect::<BTreeSet<_>>();

        Self::Allowed(markets)
    }

    /// Makes a new, empty excluded `Markets`.
    ///
    /// Does not allocate anything on its own.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use winget_types::installer::Markets;
    ///
    /// let mut markets = Markets::new_excluded();
    /// ```
    #[must_use]
    pub const fn new_excluded() -> Self {
        Self::Excluded(BTreeSet::new())
    }

    /// Creates a new excluded `Markets` from an iterator of any type that implements `AsRef<str`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if any of the iterator items are not exactly 2 ASCII uppercase characters
    /// or if the number of unique markets exceeds 256.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::{Market, Markets};
    ///
    /// let markets = Markets::excluded_from_iter(["US", "UK"]).unwrap();
    ///
    /// let v: Vec<_> = markets.into_iter().collect();
    /// assert_eq!(v, [Market::new("UK").unwrap(), Market::new("US").unwrap()]);
    /// ```
    pub fn excluded_from_iter<I, T>(markets: I) -> Result<Self, MarketsError>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let markets = markets
            .into_iter()
            .map(|market| Market::new(market.as_ref()))
            .collect::<Result<BTreeSet<_>, MarketError>>()?;

        if markets.len() > Self::MAX_ITEMS {
            return Err(MarketsError::TooManyMarkets);
        }

        Ok(Self::Excluded(markets))
    }

    /// Creates a new excluded `Markets` from an iterator of any type that implements `AsRef<str`
    /// without checking whether each item is exactly 2 ASCII uppercase characters or if the number
    /// of unique markets exceeds 256. This results in undefined behaviour if any item is not
    /// exactly 2 ASCII uppercase characters or the number of unique markets exceeds 256.
    ///
    /// # Safety
    ///
    /// Each item must be exactly 2 ASCII uppercase characters and the number of unique markets must
    /// not exceed 256.
    pub unsafe fn excluded_from_iter_unchecked<I, T>(markets: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let markets = markets
            .into_iter()
            .map(|market| unsafe { Market::new_unchecked(market.as_ref()) })
            .collect::<BTreeSet<_>>();

        Self::Excluded(markets)
    }

    /// Adds a market to the set.
    ///
    /// Returns whether the market was newly inserted. That is:
    ///
    /// - If the markets did not previously contain an equal market, `true` is returned.
    /// - If the markets already contained an equal market, `false` is returned, and the markets are
    ///   not updated.
    ///
    /// # Errors
    ///
    /// Returns an error if adding the market would result in more than 256 markets.
    pub fn add<T: AsRef<str>>(&mut self, market: T) -> Result<bool, MarketsError> {
        let markets = match self {
            Self::Allowed(markets) | Self::Excluded(markets) => markets,
        };

        if markets.len() == Self::MAX_ITEMS {
            return Err(MarketsError::TooManyMarkets);
        }

        Ok(markets.insert(Market::new(market)?))
    }

    /// Returns `true` if the set contains a market equal to the value.
    ///
    /// The value may be any borrowed form of `Market`, but the ordering on the borrowed form *must*
    /// match the ordering on the element type.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::Markets;
    ///
    /// let markets = Markets::allowed_from_iter(["US", "UK"]).unwrap();
    /// assert_eq!(markets.contains("US"), true);
    /// assert_eq!(markets.contains("DE"), false);
    /// ```
    pub fn contains<Q>(&self, market: &Q) -> bool
    where
        Market: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        match self {
            Self::Allowed(markets) | Self::Excluded(markets) => markets.contains(market),
        }
    }

    /// If the set contains a market equal to the value, removes it from the set and drops it.
    /// Returns whether such a market was present.
    ///
    /// The value may be any borrowed form of the set's element type, but the ordering on the
    /// borrowed form *must* match the ordering on the element type.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::Markets;
    ///
    /// let mut markets = Markets::new_allowed();
    ///
    /// markets.add("US").unwrap();
    /// assert_eq!(markets.remove("US"), true);
    /// assert_eq!(markets.remove("US"), false);
    /// ```
    pub fn remove<Q>(&mut self, market: &Q) -> bool
    where
        Market: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        match self {
            Self::Allowed(markets) | Self::Excluded(markets) => markets.remove(market),
        }
    }

    /// Clears the set, removing all markets.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::Markets;
    ///
    /// let mut markets = Markets::new_allowed();
    /// markets.add("US").unwrap();
    /// markets.clear();
    /// assert!(markets.is_empty());
    /// ```
    pub fn clear(&mut self) {
        match self {
            Self::Allowed(markets) | Self::Excluded(markets) => markets.clear(),
        }
    }

    /// Returns the number of markets in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::Markets;
    ///
    /// let mut markets = Markets::new_allowed();
    /// assert_eq!(markets.len(), 0);
    /// markets.add("US").unwrap();
    /// assert_eq!(markets.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Allowed(markets) | Self::Excluded(markets) => markets.len(),
        }
    }

    /// Returns `true` if the set contains no markets.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::Markets;
    ///
    /// let mut markets = Markets::new_allowed();
    /// assert!(markets.is_empty());
    /// markets.add("US").unwrap();
    /// assert!(!markets.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Allowed(markets) | Self::Excluded(markets) => markets.is_empty(),
        }
    }

    /// Gets an iterator that visits the elements in `Markets` in ascending order.
    #[inline]
    pub fn iter(&self) -> alloc::collections::btree_set::Iter<'_, Market> {
        self.into_iter()
    }
}

impl IntoIterator for Markets {
    type Item = Market;

    type IntoIter = alloc::collections::btree_set::IntoIter<Market>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Allowed(markets) | Self::Excluded(markets) => markets.into_iter(),
        }
    }
}

impl<'market> IntoIterator for &'market Markets {
    type Item = &'market Market;

    type IntoIter = alloc::collections::btree_set::Iter<'market, Market>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Markets::Allowed(markets) | Markets::Excluded(markets) => markets.iter(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Markets {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serde::Serializer::serialize_struct(serializer, "Markets", 1)?;
        match self {
            Self::Allowed(markets) => {
                serde::ser::SerializeStruct::serialize_field(
                    &mut state,
                    "AllowedMarkets",
                    markets,
                )?;
            }
            Self::Excluded(markets) => {
                serde::ser::SerializeStruct::serialize_field(
                    &mut state,
                    "ExcludedMarkets",
                    markets,
                )?;
            }
        }
        serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Markets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MarketsVisitor;

        impl<'de> serde::de::Visitor<'de> for MarketsVisitor {
            type Value = Markets;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a map with either 'AllowedMarkets' or 'ExcludedMarkets'")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Markets, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut allowed: Option<BTreeSet<Market>> = None;
                let mut excluded: Option<BTreeSet<Market>> = None;

                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "AllowedMarkets" => {
                            if allowed.is_some() {
                                return Err(serde::de::Error::duplicate_field("AllowedMarkets"));
                            }
                            allowed = Some(map.next_value()?);
                        }
                        "ExcludedMarkets" => {
                            if excluded.is_some() {
                                return Err(serde::de::Error::duplicate_field("ExcludedMarkets"));
                            }
                            excluded = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                key,
                                &["AllowedMarkets", "ExcludedMarkets"],
                            ));
                        }
                    }

                    if allowed
                        .as_ref()
                        .is_some_and(|markets| markets.len() > Self::Value::MAX_ITEMS)
                        || excluded
                            .as_ref()
                            .is_some_and(|markets| markets.len() > Self::Value::MAX_ITEMS)
                    {
                        return Err(serde::de::Error::custom(MarketsError::TooManyMarkets));
                    }
                }

                match (allowed, excluded) {
                    (Some(markets), None) => Ok(Markets::Allowed(markets)),
                    (None, Some(markets)) => Ok(Markets::Excluded(markets)),
                    (Some(_), Some(_)) => Err(serde::de::Error::custom(
                        "Expected either 'AllowedMarkets' or 'ExcludedMarkets', but found both",
                    )),
                    (None, None) => Err(serde::de::Error::custom(
                        "Expected either 'AllowedMarkets' or 'ExcludedMarkets', but found neither",
                    )),
                }
            }
        }

        deserializer.deserialize_map(MarketsVisitor)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use indoc::indoc;
    use itertools::iproduct;

    use super::{Markets, MarketsError};

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_markets() {
        assert_eq!(
            serde_yaml::to_string(&Markets::allowed_from_iter(["US", "UK"]).unwrap()).unwrap(),
            indoc! {"
                AllowedMarkets:
                - UK
                - US
            "}
        );

        assert_eq!(
            serde_yaml::to_string(&Markets::excluded_from_iter(["US", "UK"]).unwrap()).unwrap(),
            indoc! {"
                ExcludedMarkets:
                - UK
                - US
            "}
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_valid_markets() {
        assert_eq!(
            serde_yaml::from_str::<Markets>(indoc! {"
                AllowedMarkets:
                - US
                - UK
            "})
            .unwrap(),
            Markets::allowed_from_iter(["UK", "US"]).unwrap()
        );

        assert_eq!(
            serde_yaml::from_str::<Markets>(indoc! {"
                ExcludedMarkets:
                - US
                - UK
            "})
            .unwrap(),
            Markets::excluded_from_iter(["UK", "US"]).unwrap()
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_invalid_markets() {
        assert!(
            serde_yaml::from_str::<Markets>(indoc! {"
                AllowedMarkets:
                - ABC
            "})
            .is_err(),
        );

        assert!(
            serde_yaml::from_str::<Markets>(indoc! {"
                ExcludedMarkets:
                - ab
            "})
            .is_err(),
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_too_many_markets() {
        use alloc::string::String;
        use core::fmt::Write;

        let mut many_markets = String::from("AllowedMarkets:\n");

        let mut markets = iproduct!('A'..='Z', 'A'..='Z');

        for _ in 0..Markets::MAX_ITEMS {
            let (first, second) = markets.next().unwrap();
            let _ = writeln!(many_markets, "- {first}{second}");
        }

        // Check that the maximum number of markets is valid
        assert!(serde_yaml::from_str::<Markets>(&many_markets).is_ok());

        // Add one more market to overflow the maximum number of markets
        let (first, second) = markets.next().unwrap();
        let _ = writeln!(many_markets, "- {first}{second}");

        assert!(serde_yaml::from_str::<Markets>(&many_markets).is_err());
    }

    #[test]
    fn too_many_markets() {
        use compact_str::format_compact;

        let mut markets = heapless::Vec::<_, { Markets::MAX_ITEMS + 1 }>::new();

        for (first, second) in iproduct!('A'..='Z', 'A'..='Z') {
            if markets.push(format_compact!("{first}{second}")).is_err() {
                break;
            }
        }

        assert_eq!(
            Markets::allowed_from_iter(&markets),
            Err(MarketsError::TooManyMarkets)
        );
        assert_eq!(
            Markets::excluded_from_iter(&markets),
            Err(MarketsError::TooManyMarkets)
        );
    }
}
