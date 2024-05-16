use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View and manage your DoubleClick Campaign Manager's (DCM) display ad campaigns
    Dfatrafficking,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Dfatrafficking => "https://www.googleapis.com/auth/dfatrafficking",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Dfatrafficking
    }
}

