use core::fmt;

use bitflags::bitflags;

bitflags! {
    /// A list of unsupported arguments internally represented as bit flags
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct UnsupportedArguments: u8 {
        const LOG = 1;
        const LOCATION = 1 << 1;
    }
}

const LOG: &str = "Log";
const LOCATION: &str = "Location";

impl fmt::Display for UnsupportedArguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LOG => f.write_str(LOG),
            Self::LOCATION => f.write_str(LOCATION),
            _ => bitflags::parser::to_writer(self, f),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for UnsupportedArguments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.iter().count()))?;
        for unsupported_argument in self.iter() {
            match unsupported_argument {
                Self::LOG => seq.serialize_element(LOG)?,
                Self::LOCATION => seq.serialize_element(LOCATION)?,
                _ => {}
            }
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for UnsupportedArguments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UnsupportedArgumentsVisitor;

        impl<'de> serde::de::Visitor<'de> for UnsupportedArgumentsVisitor {
            type Value = UnsupportedArguments;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of unsupported arguments")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let mut unsupported_arguments = UnsupportedArguments::empty();

                while let Some(value) = seq.next_element::<&str>()? {
                    match value {
                        LOG => unsupported_arguments |= UnsupportedArguments::LOG,
                        LOCATION => unsupported_arguments |= UnsupportedArguments::LOCATION,
                        _ => {
                            return Err(serde::de::Error::unknown_variant(value, &[LOG, LOCATION]));
                        }
                    }
                }

                Ok(unsupported_arguments)
            }
        }

        deserializer.deserialize_seq(UnsupportedArgumentsVisitor)
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use indoc::indoc;
    use rstest::rstest;

    use super::UnsupportedArguments;

    #[rstest]
    #[case(
        UnsupportedArguments::all(),
        indoc! {"
            - Log
            - Location
        "}
    )]
    #[case(
        UnsupportedArguments::empty(),
        indoc! {"
            []
        "}
    )]
    #[case(
        UnsupportedArguments::LOG,
        indoc! {"
            - Log
        "}
    )]
    #[case(
        UnsupportedArguments::LOCATION,
        indoc! {"
            - Location
        "}
    )]
    fn serialize_unsupported_arguments(
        #[case] unsupported_args: UnsupportedArguments,
        #[case] expected: &str,
    ) {
        assert_eq!(serde_yaml::to_string(&unsupported_args).unwrap(), expected);
    }

    #[rstest]
    #[case(
        indoc! {"
            - Log
            - Location
        "},
        UnsupportedArguments::all(),
    )]
    #[case(
        indoc! {"
            []
        "},
        UnsupportedArguments::empty()
    )]
    #[case(
        indoc! {"
            - Log
        "},
        UnsupportedArguments::LOG,
    )]
    #[case(
        indoc! {"
            - Location
        "},
        UnsupportedArguments::LOCATION
    )]
    fn deserialize_unsupported_arguments(
        #[case] input: &str,
        #[case] expected: UnsupportedArguments,
    ) {
        assert_eq!(
            serde_yaml::from_str::<UnsupportedArguments>(input).unwrap(),
            expected
        );
    }

    #[test]
    fn unsupported_arguments_serialize_ordered() {
        let input = indoc! {"
            - Location
            - Log
        "};

        let deserialized = serde_yaml::from_str::<UnsupportedArguments>(input).unwrap();

        assert_eq!(
            serde_yaml::to_string(&deserialized).unwrap(),
            indoc! {"
                - Log
                - Location
            "}
        );
    }
}
