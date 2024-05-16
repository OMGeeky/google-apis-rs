use super::*;



// region CreativeStatusFilterEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// When specified, only creatives having the given status are returned.
pub enum CreativeStatusFilterEnum {
    

    /// Creatives which have been approved.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    

    /// Creatives which have been disapproved.
    ///
    /// "disapproved"
    #[serde(rename="disapproved")]
    Disapproved,
    

    /// Creatives whose status is not yet checked.
    ///
    /// "not_checked"
    #[serde(rename="not_checked")]
    NotChecked,
}

impl AsRef<str> for CreativeStatusFilterEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeStatusFilterEnum::Approved => "approved",
            CreativeStatusFilterEnum::Disapproved => "disapproved",
            CreativeStatusFilterEnum::NotChecked => "not_checked",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeStatusFilterEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "approved" => Ok(CreativeStatusFilterEnum::Approved),
           "disapproved" => Ok(CreativeStatusFilterEnum::Disapproved),
           "not_checked" => Ok(CreativeStatusFilterEnum::NotChecked),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeStatusFilterEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


