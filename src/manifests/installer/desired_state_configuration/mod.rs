mod dsc_v3;
mod dsc_v3_resource;
mod powershell_module;
mod powershell_resource;

use alloc::collections::BTreeSet;

pub use dsc_v3::DscV3;
pub use dsc_v3_resource::DscV3Resource;
pub use powershell_module::PowerShellModule;
pub use powershell_resource::PowerShellResource;

/// References to desired state configuration (DSC) resources related to a package.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DesiredStateConfiguration {
    /// DSC resources contained in PowerShell modules.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "PowerShell",
            skip_serializing_if = "BTreeSet::is_empty",
            default
        )
    )]
    pub powershell_modules: BTreeSet<PowerShellModule>,

    /// DSC resources contained in the package using the DSC v3 specification.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "DSCv3", skip_serializing_if = "DscV3::is_empty", default)
    )]
    pub dsc_v3: DscV3,
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use alloc::vec;

    use indoc::indoc;
    use rstest::rstest;

    use super::DesiredStateConfiguration;
    use crate::{
        Manifest, ManifestVersion, VersionManifest,
        installer::{Installer, InstallerManifest},
        locale::{DefaultLocaleManifest, LocaleManifest},
    };

    const DSC: &str = indoc! {"
        PowerShell:
        - RepositoryUrl: https://www.powershellgallery.com/api/v2
          ModuleName: Example.Module
          Resources:
          - Name: ExampleResource
        DSCv3:
          Resources:
          - Type: Example.Package/Resource
    "};

    #[test]
    fn round_trip_dsc_metadata() {
        let dsc: DesiredStateConfiguration = serde_yaml::from_str(DSC).unwrap();
        assert_eq!(serde_yaml::to_string(&dsc).unwrap(), DSC);

        let manifest = InstallerManifest {
            package_identifier: "Example.Package".parse().unwrap(),
            package_version: "1.0.0".parse().unwrap(),
            desired_state_configuration: Some(dsc.clone()),
            installers: vec![Installer {
                url: "https://example.com/installer.exe".parse().unwrap(),
                desired_state_configuration: Some(dsc),
                ..Installer::default()
            }],
            ..InstallerManifest::default()
        };
        let yaml = serde_yaml::to_string(&manifest).unwrap();
        assert_eq!(
            serde_yaml::from_str::<InstallerManifest>(&yaml).unwrap(),
            manifest
        );
        assert!(yaml.contains("ManifestVersion: 1.28.0"));
    }

    #[rstest]
    #[case("{}")]
    #[case("PowerShell: []\nDSCv3:\n  Resources: []")]
    fn accepts_optional_and_empty_resources(#[case] yaml: &str) {
        let dsc: DesiredStateConfiguration = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            serde_yaml::from_str::<DesiredStateConfiguration>(
                &serde_yaml::to_string(&dsc).unwrap()
            )
            .unwrap(),
            dsc
        );
    }

    #[rstest]
    #[case("PowerShell:\n- ModuleName: Example.Module\n  Resources: []")]
    #[case("PowerShell:\n- RepositoryUrl: https://example.com\n  Resources: []")]
    #[case("PowerShell:\n- RepositoryUrl: https://example.com\n  ModuleName: Example.Module")]
    fn requires_module_fields_and_resource_lists(#[case] yaml: &str) {
        assert!(serde_yaml::from_str::<DesiredStateConfiguration>(yaml).is_err());
    }

    #[test]
    fn manifest_versions_and_schemas_match() {
        fn check<M: Manifest + Default>() {
            assert_eq!(
                M::default().manifest_version(),
                ManifestVersion::new(1, 28, 0)
            );
            assert!(M::SCHEMA.ends_with(".1.28.0.schema.json"));
        }
        check::<InstallerManifest>();
        check::<DefaultLocaleManifest>();
        check::<LocaleManifest>();
        check::<VersionManifest>();
    }
}
