use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// Administer your Spanner databases
    Admin,

    /// View and manage the contents of your Spanner databases
    Data,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::Admin => "https://www.googleapis.com/auth/spanner.admin",
            Scope::Data => "https://www.googleapis.com/auth/spanner.data",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Data
    }
}

