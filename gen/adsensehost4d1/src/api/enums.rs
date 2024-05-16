use super::*;



// region AssociationsessionProductCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Products to associate with the user.
pub enum AssociationsessionProductCodeEnum {
    

    /// AdSense For Content
    ///
    /// "AFC"
    #[serde(rename="AFC")]
    AFC,
    

    /// AdSense For Games
    ///
    /// "AFG"
    #[serde(rename="AFG")]
    AFG,
    

    /// AdSense For Mobile Content - deprecated
    ///
    /// "AFMC"
    #[serde(rename="AFMC")]
    AFMC,
    

    /// AdSense For Search - deprecated
    ///
    /// "AFS"
    #[serde(rename="AFS")]
    AFS,
    

    /// AdSense For Video
    ///
    /// "AFV"
    #[serde(rename="AFV")]
    AFV,
}

impl AsRef<str> for AssociationsessionProductCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssociationsessionProductCodeEnum::AFC => "AFC",
            AssociationsessionProductCodeEnum::AFG => "AFG",
            AssociationsessionProductCodeEnum::AFMC => "AFMC",
            AssociationsessionProductCodeEnum::AFS => "AFS",
            AssociationsessionProductCodeEnum::AFV => "AFV",
        }
    }
}

impl std::convert::TryFrom< &str> for AssociationsessionProductCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AFC" => Ok(AssociationsessionProductCodeEnum::AFC),
           "AFG" => Ok(AssociationsessionProductCodeEnum::AFG),
           "AFMC" => Ok(AssociationsessionProductCodeEnum::AFMC),
           "AFS" => Ok(AssociationsessionProductCodeEnum::AFS),
           "AFV" => Ok(AssociationsessionProductCodeEnum::AFV),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssociationsessionProductCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


