use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,

    /// View your data across Google Cloud Platform services
    CloudPlatformReadOnly,

    /// View and manage your Google Compute Engine resources
    Compute,

    /// View your Google Compute Engine resources
    ComputeReadonly,

    /// View and manage your Google Cloud Platform management resources and deployment status information
    NdevCloudman,

    /// View your Google Cloud Platform management resources and deployment status information
    NdevCloudmanReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            Scope::Compute => "https://www.googleapis.com/auth/compute",
            Scope::ComputeReadonly => "https://www.googleapis.com/auth/compute.readonly",
            Scope::NdevCloudman => "https://www.googleapis.com/auth/ndev.cloudman",
            Scope::NdevCloudmanReadonly => "https://www.googleapis.com/auth/ndev.cloudman.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ComputeReadonly
    }
}

