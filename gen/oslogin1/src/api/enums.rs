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


