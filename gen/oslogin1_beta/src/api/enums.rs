use super::*;



// region PosixAccountOperatingSystemTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operating system type where this account applies.
pub enum PosixAccountOperatingSystemTypeEnum {
    

    /// The operating system type associated with the user account information is unspecified.
    ///
    /// "OPERATING_SYSTEM_TYPE_UNSPECIFIED"
    #[serde(rename="OPERATING_SYSTEM_TYPE_UNSPECIFIED")]
    OPERATINGSYSTEMTYPEUNSPECIFIED,
    

    /// Linux user account information.
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Windows user account information.
    ///
    /// "WINDOWS"
    #[serde(rename="WINDOWS")]
    WINDOWS,
}

impl AsRef<str> for PosixAccountOperatingSystemTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PosixAccountOperatingSystemTypeEnum::OPERATINGSYSTEMTYPEUNSPECIFIED => "OPERATING_SYSTEM_TYPE_UNSPECIFIED",
            PosixAccountOperatingSystemTypeEnum::LINUX => "LINUX",
            PosixAccountOperatingSystemTypeEnum::WINDOWS => "WINDOWS",
        }
    }
}

impl std::convert::TryFrom< &str> for PosixAccountOperatingSystemTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATING_SYSTEM_TYPE_UNSPECIFIED" => Ok(PosixAccountOperatingSystemTypeEnum::OPERATINGSYSTEMTYPEUNSPECIFIED),
           "LINUX" => Ok(PosixAccountOperatingSystemTypeEnum::LINUX),
           "WINDOWS" => Ok(PosixAccountOperatingSystemTypeEnum::WINDOWS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PosixAccountOperatingSystemTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The view configures whether to retrieve security keys information.
pub enum UserViewEnum {
    

    /// The default login profile view. The API defaults to the BASIC view.
    ///
    /// "LOGIN_PROFILE_VIEW_UNSPECIFIED"
    #[serde(rename="LOGIN_PROFILE_VIEW_UNSPECIFIED")]
    LOGINPROFILEVIEWUNSPECIFIED,
    

    /// Includes POSIX and SSH key information.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Include security key information for the user.
    ///
    /// "SECURITY_KEY"
    #[serde(rename="SECURITY_KEY")]
    SECURITYKEY,
}

impl AsRef<str> for UserViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserViewEnum::LOGINPROFILEVIEWUNSPECIFIED => "LOGIN_PROFILE_VIEW_UNSPECIFIED",
            UserViewEnum::BASIC => "BASIC",
            UserViewEnum::SECURITYKEY => "SECURITY_KEY",
        }
    }
}

impl std::convert::TryFrom< &str> for UserViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOGIN_PROFILE_VIEW_UNSPECIFIED" => Ok(UserViewEnum::LOGINPROFILEVIEWUNSPECIFIED),
           "BASIC" => Ok(UserViewEnum::BASIC),
           "SECURITY_KEY" => Ok(UserViewEnum::SECURITYKEY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


