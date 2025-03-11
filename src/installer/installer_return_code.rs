use core::{
    cmp::Ordering,
    fmt,
    num::{NonZero, NonZeroI32, NonZeroU32, ParseIntError, TryFromIntError},
    str::FromStr,
};

pub type InstallerSuccessCode = InstallerReturnCode;

/// An exit code that can be returned by an installer after execution.
///
/// An `InstallerReturnCode` has the possible range of ([`i32::MIN`]..=-1, 1..=[`u32::MAX`]) and
/// cannot be 0.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InstallerReturnCode {
    Positive(NonZeroU32), // Holds values greater than 0
    Negative(NonZeroI32), // Holds values less than 0
}

impl InstallerReturnCode {
    /// The smallest value that can be represented by an `InstallerReturnCode`, equal to
    /// [`i32::MIN`].
    ///
    /// # Examples
    ///
    /// ```
    /// use core::num::NonZeroI32;
    /// use winget_types::installer::InstallerReturnCode;
    ///
    /// assert_eq!(InstallerReturnCode::MIN.i32(), Some(i32::MIN));
    /// assert_eq!(InstallerReturnCode::MIN.i32(), Some(NonZeroI32::MIN.get()));
    /// ```
    pub const MIN: Self = Self::Negative(NonZeroI32::MIN);

    /// The largest value that can be represented by an `InstallerReturnCode`, equal to
    /// [`u32::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// use core::num::NonZeroU32;
    /// use winget_types::installer::InstallerReturnCode;
    ///
    /// assert_eq!(InstallerReturnCode::MAX.u32(), Some(u32::MAX));
    /// assert_eq!(InstallerReturnCode::MAX.u32(), Some(NonZeroU32::MAX.get()));
    /// ```
    pub const MAX: Self = Self::Positive(NonZeroU32::MAX);

    /// Creates a new `InstallerReturnCode` from any type that implements
    /// `TryInto<InstallerReturnCode>` if the given value is not zero.
    ///
    /// This is implemented for [`u8`], [`u16`], [`u32`], [`i8`], [`i16`], [`i32`] and
    /// their [`NonZero`] equivalents.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::InstallerReturnCode;
    ///
    /// assert!(InstallerReturnCode::new(u8::MAX).is_some());
    /// assert!(InstallerReturnCode::new(u16::MAX).is_some());
    /// assert!(InstallerReturnCode::new(u32::MAX).is_some());
    /// assert!(InstallerReturnCode::new(i8::MIN).is_some());
    /// assert!(InstallerReturnCode::new(i16::MIN).is_some());
    /// assert!(InstallerReturnCode::new(i32::MIN).is_some());
    /// ```
    pub fn new<N: TryInto<Self>>(n: N) -> Option<Self> {
        n.try_into().ok()
    }

    /// Creates an `InstallerReturnCode` if the given value is not zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::num::NonZeroU32;
    /// use winget_types::installer::InstallerReturnCode;
    ///
    /// let return_code = InstallerReturnCode::from_u32(10);
    /// assert_eq!(return_code, NonZeroU32::new(10).map(InstallerReturnCode::Positive));
    /// ```
    #[must_use]
    pub const fn from_u32(n: u32) -> Option<Self> {
        match NonZeroU32::new(n) {
            Some(n) => Some(Self::Positive(n)),
            None => None,
        }
    }

    /// Creates an `InstallerReturnCode` if the given value is not zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::num::NonZeroI32;
    /// use winget_types::installer::InstallerReturnCode;
    ///
    /// let return_code = InstallerReturnCode::from_i32(-10);
    /// assert_eq!(return_code, NonZeroI32::new(-10).map(InstallerReturnCode::Negative));
    /// ```
    #[must_use]
    pub const fn from_i32(n: i32) -> Option<Self> {
        match NonZeroI32::new(n) {
            Some(n) => {
                if n.is_positive() {
                    Some(Self::Positive(n.unsigned_abs()))
                } else {
                    Some(Self::Negative(n))
                }
            }
            None => None,
        }
    }

    /// Creates an `InstallerReturnCode` without checking whether the value is non-zero.
    /// This results in undefined behavior if the value is zero.
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[must_use]
    #[inline]
    pub const unsafe fn from_u32_unchecked(n: u32) -> Self {
        Self::Positive(unsafe { NonZeroU32::new_unchecked(n) })
    }

    /// Creates an `InstallerReturnCode` without checking whether the value is non-zero.
    /// This results in undefined behavior if the value is zero.
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[must_use]
    pub const unsafe fn from_i32_unchecked(n: i32) -> Self {
        let non_zero = unsafe { NonZeroI32::new_unchecked(n) };
        if non_zero.is_positive() {
            Self::Positive(non_zero.unsigned_abs())
        } else {
            Self::Negative(non_zero)
        }
    }

    /// Returns the contained value as an `i64` as both a `u32` and `i32` fit into an `i64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::InstallerReturnCode;
    ///
    /// assert_eq!(InstallerReturnCode::new(1).unwrap().get(), 1i64);
    /// assert_eq!(InstallerReturnCode::MAX.get(), 4294967295i64);
    /// assert_eq!(InstallerReturnCode::MIN.get(), -2147483648i64);
    /// ```
    #[must_use]
    #[inline]
    pub const fn get(self) -> i64 {
        match self {
            Self::Positive(n) => n.get() as i64,
            Self::Negative(n) => n.get() as i64,
        }
    }

    /// Returns the contained value as a [`u32`] if it is positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::InstallerReturnCode;
    ///
    /// let return_code = InstallerReturnCode::from_u32(50).unwrap();
    /// assert_eq!(return_code.u32(), Some(50));
    ///
    /// let return_code = InstallerReturnCode::from_i32(-1).unwrap();
    /// assert!(return_code.u32().is_none());
    /// ```
    #[must_use]
    pub const fn u32(self) -> Option<u32> {
        match self {
            Self::Positive(n) => Some(n.get()), // 1..=u32::MAX
            Self::Negative(_) => None,          // Negative values can never convert into a u32
        }
    }

    /// Returns the contained value as an [`i32`] if it is less than or equal to [`i32::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::InstallerReturnCode;
    ///
    /// let return_code = InstallerReturnCode::from_i32(-1).unwrap();
    /// assert_eq!(return_code.i32(), Some(-1));
    ///
    /// let return_code = InstallerReturnCode::from_u32(100).unwrap();
    /// assert_eq!(return_code.i32(), Some(100));
    ///
    /// let return_code = InstallerReturnCode::from_u32((i32::MAX as u32) + 1).unwrap();
    /// assert!(return_code.i32().is_none());
    /// ```
    #[must_use]
    pub fn i32(self) -> Option<i32> {
        match self {
            Self::Positive(n) => n.get().try_into().ok(), // 1..=i32::MAX
            Self::Negative(n) => Some(n.get()),           // i32::MIN..=-1
        }
    }
}

impl fmt::Display for InstallerReturnCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positive(n) => n.fmt(f),
            Self::Negative(n) => n.fmt(f),
        }
    }
}

impl FromStr for InstallerReturnCode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NonZeroU32::from_str(s)
            .map(Self::Positive)
            .or_else(|_| NonZeroI32::from_str(s).map(Self::Negative))
    }
}

impl PartialOrd for InstallerReturnCode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InstallerReturnCode {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Positive(a) => match other {
                Self::Positive(b) => a.cmp(b),
                Self::Negative(_) => Ordering::Greater,
            },
            Self::Negative(a) => match other {
                Self::Positive(_) => Ordering::Less,
                Self::Negative(b) => a.cmp(b),
            },
        }
    }
}

macro_rules! impl_return_code_from_unsigned_int {
    ($($unsigned:ty), +) => {$(
        impl TryFrom<$unsigned> for InstallerReturnCode
        where
            NonZeroU32: From<NonZero<$unsigned>>,
        {
            type Error = TryFromIntError;

            fn try_from(value: $unsigned) -> Result<Self, Self::Error> {
                NonZero::<$unsigned>::try_from(value)
                    .map(NonZeroU32::from)
                    .map(Self::Positive)
            }
        }

        impl From<NonZero<$unsigned>> for InstallerReturnCode
        where
            NonZeroU32: From<NonZero<$unsigned>>,
        {
            #[inline]
            fn from(value: NonZero<$unsigned>) -> Self {
                Self::Positive(NonZeroU32::from(value))
            }
        }
    )*};
}

impl_return_code_from_unsigned_int!(u8, u16, u32);

macro_rules! impl_return_code_from_signed_int {
    ($($signed:ty => $unsigned:ty),+) => {$(
        impl TryFrom<$signed> for InstallerReturnCode
        where
            NonZeroU32: From<NonZero<$unsigned>>,
            NonZeroI32: From<NonZero<$signed>>,
        {
            type Error = TryFromIntError;

            fn try_from(value: $signed) -> Result<Self, Self::Error> {
                if value.is_positive() {
                    NonZero::<$unsigned>::try_from(value.unsigned_abs()).map(NonZeroU32::from).map(Self::Positive)
                } else {
                    NonZero::<$signed>::try_from(value).map(NonZeroI32::from).map(Self::Negative)
                }
            }
        }

        impl From<NonZero<$signed>> for InstallerReturnCode
        where
            NonZeroU32: From<NonZero<$unsigned>>,
            NonZeroI32: From<NonZero<$signed>>,
        {
            fn from(value: NonZero<$signed>) -> Self {
                if value.is_positive() {
                    Self::Positive(NonZeroU32::from(value.unsigned_abs()))
                } else {
                    Self::Negative(NonZeroI32::from(value))
                }
            }
        }
    )*};
}

impl_return_code_from_signed_int!(i8 => u8, i16 => u16, i32 => u32);

#[cfg(test)]
mod tests {
    use core::num::IntErrorKind;

    #[cfg(feature = "serde")]
    use indoc::indoc;
    use rstest::rstest;

    use super::InstallerReturnCode;

    #[rstest]
    #[case("1", Ok(InstallerReturnCode::from_u32(1).unwrap()))]
    #[case("-1", Ok(InstallerReturnCode::from_i32(-1).unwrap()))]
    #[case("0", Err(IntErrorKind::Zero))]
    #[case("4294967295", Ok(InstallerReturnCode::from_u32(u32::MAX).unwrap()))]
    #[case("4294967296", Err(IntErrorKind::PosOverflow))]
    #[case("-2147483648", Ok(InstallerReturnCode::from_i32(i32::MIN).unwrap()))]
    #[case("-2147483649", Err(IntErrorKind::NegOverflow))]
    #[case("", Err(IntErrorKind::Empty))]
    #[case("🦀", Err(IntErrorKind::InvalidDigit))]
    fn from_str(
        #[case] str: &str,
        #[case] int_error_kind: Result<InstallerReturnCode, IntErrorKind>,
    ) {
        assert_eq!(
            str.parse::<InstallerReturnCode>()
                .map_err(|err| err.kind().clone()),
            int_error_kind
        );
    }

    #[rstest]
    #[case(u8::MAX)]
    #[case(u16::MAX)]
    #[case(u32::MAX)]
    #[case(i8::MIN)]
    #[case(i16::MIN)]
    #[case(i32::MIN)]
    fn get<N>(#[case] n: N)
    where
        N: TryInto<InstallerReturnCode> + Into<i64> + Copy,
    {
        assert_eq!(
            InstallerReturnCode::new(n).map(InstallerReturnCode::get),
            Some(n.into())
        );
    }

    #[test]
    fn ordering() {
        use core::cmp::Ordering;

        let positive_return_code = InstallerReturnCode::from_u32(1).unwrap();
        let negative_return_code = InstallerReturnCode::from_i32(-1).unwrap();
        assert_eq!(
            positive_return_code.cmp(&negative_return_code),
            Ordering::Greater
        );
        assert_eq!(
            negative_return_code.cmp(&positive_return_code),
            Ordering::Less
        );
        assert_eq!(
            positive_return_code.cmp(&positive_return_code),
            Ordering::Equal
        );
        assert_eq!(
            negative_return_code.cmp(&negative_return_code),
            Ordering::Equal
        );
    }

    #[cfg(feature = "serde")]
    #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Manifest {
        installer_return_code: InstallerReturnCode,
    }

    #[cfg(feature = "serde")]
    #[rstest]
    #[case(
        Manifest {
            installer_return_code: InstallerReturnCode::from_u32(1).unwrap()
        },
        indoc! {"
            InstallerReturnCode: 1
        "}
    )]
    #[case(
        Manifest {
            installer_return_code: InstallerReturnCode::from_i32(-1).unwrap()
        },
        indoc! {"
            InstallerReturnCode: -1
        "}
    )]
    #[case(
        Manifest {
            installer_return_code: InstallerReturnCode::from_u32(u32::MAX).unwrap()
        },
        indoc! {"
            InstallerReturnCode: 4294967295
        "}
    )]
    #[case(
        Manifest {
            installer_return_code: InstallerReturnCode::from_i32(i32::MIN).unwrap()
        },
        indoc! {"
            InstallerReturnCode: -2147483648
        "}
    )]
    fn serialize(#[case] manifest: Manifest, #[case] manifest_str: &str) {
        assert_eq!(
            serde_yaml::to_string(&manifest).as_deref().map_err(|_| ()),
            Ok(manifest_str)
        );
    }

    #[cfg(feature = "serde")]
    #[rstest]
    #[case(
        indoc! {"
            InstallerReturnCode: 1
        "},
        Ok(Manifest {
            installer_return_code: InstallerReturnCode::from_u32(1).unwrap()
        })
    )]
    #[case(
        indoc! {"
            InstallerReturnCode: -1
        "},
        Ok(Manifest {
            installer_return_code: InstallerReturnCode::from_i32(-1).unwrap()
        })
    )]
    #[case(
        indoc! {"
            InstallerReturnCode: 0
        "},
        Err(())
    )]
    #[case(
        indoc! {"
            InstallerReturnCode: 4294967295
        "},
        Ok(Manifest {
            installer_return_code: InstallerReturnCode::from_u32(u32::MAX).unwrap()
        })
    )]
    #[case(
        indoc! {"
            InstallerReturnCode: 4294967296
        "},
        Err(())
    )]
    #[case(
        indoc! {"
            InstallerReturnCode: -2147483648
        "},
        Ok(Manifest {
            installer_return_code: InstallerReturnCode::from_i32(i32::MIN).unwrap()
        })
    )]
    #[case(
        indoc! {"
            InstallerReturnCode: -2147483649
        "},
        Err(())
    )]
    fn deserialize(#[case] manifest_str: &str, #[case] manifest: Result<Manifest, ()>) {
        assert_eq!(
            serde_yaml::from_str::<Manifest>(manifest_str).map_err(|_| ()),
            manifest,
        );
    }
}
