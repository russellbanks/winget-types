use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ElevationRequirement {
    /// Must be run from a shell that is running in an administrative context (e.g - Admin user
    /// using powershell/terminal/cmd with "Run as Administrator")
    ElevationRequired,
    /// Must be run from a shell that is not running in an administrative context.
    ElevationProhibited,
    /// If called from a non-administrative context, will request elevation. If called from an
    /// administrative context, may or may not request elevation.
    ElevatesSelf,
}

impl fmt::Display for ElevationRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElevationRequired => f.write_str("Elevation required"),
            Self::ElevationProhibited => f.write_str("Elevation prohibited"),
            Self::ElevatesSelf => f.write_str("Elevate self"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use indoc::indoc;
    #[cfg(feature = "serde")]
    use rstest::rstest;

    #[cfg(feature = "serde")]
    use super::ElevationRequirement;

    #[cfg(feature = "serde")]
    #[rstest]
    #[case(
        ElevationRequirement::ElevationRequired,
        indoc! {"
            ElevationRequirement: elevationRequired
        "}
    )]
    #[case(
        ElevationRequirement::ElevationProhibited,
        indoc! {"
            ElevationRequirement: elevationProhibited
        "}
    )]
    #[case(
        ElevationRequirement::ElevatesSelf,
        indoc! {"
            ElevationRequirement: elevatesSelf
        "}
    )]
    fn serialize_elevation_requirement(
        #[case] elevation_requirement: ElevationRequirement,
        #[case] expected: &str,
    ) {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Manifest {
            elevation_requirement: ElevationRequirement,
        }

        assert_eq!(
            serde_yaml::to_string(&Manifest {
                elevation_requirement,
            })
            .unwrap(),
            expected,
        );
    }
}
