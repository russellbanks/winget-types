pub use resource::{Resource, ResourceError};
pub use scope::{Scope, ScopeError};

mod resource;
mod scope;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct MicrosoftEntraIdAuthenticationInfo {
    /// This field controls the resource which will be used when using Entra Id for downloading or
    /// installing packages.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub resource: Option<Resource>,

    /// This field controls the scope which will be used when using Entra Id for downloading or
    /// installing packages.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub scope: Option<Scope>,
}

impl MicrosoftEntraIdAuthenticationInfo {
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.resource.is_none() && self.scope.is_none()
    }
}
