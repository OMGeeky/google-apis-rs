use super::*;



// region PermissionRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role granted by this permission. The role determines the entity’s ability to read, write, and share notes.
pub enum PermissionRoleEnum {
    

    /// An undefined role.
    ///
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    

    /// A role granting full access. This role cannot be added or removed. Defined by the creator of the note.
    ///
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
    

    /// A role granting the ability to contribute content and modify note permissions.
    ///
    /// "WRITER"
    #[serde(rename="WRITER")]
    WRITER,
}

impl AsRef<str> for PermissionRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PermissionRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            PermissionRoleEnum::OWNER => "OWNER",
            PermissionRoleEnum::WRITER => "WRITER",
        }
    }
}

impl std::convert::TryFrom< &str> for PermissionRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(PermissionRoleEnum::ROLEUNSPECIFIED),
           "OWNER" => Ok(PermissionRoleEnum::OWNER),
           "WRITER" => Ok(PermissionRoleEnum::WRITER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PermissionRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


