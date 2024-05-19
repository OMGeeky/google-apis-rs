use super::*;



// region EditDeobfuscationFileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum EditDeobfuscationFileTypeEnum {
    
    /// "proguard"
    #[serde(rename="proguard")]
    Proguard,
}

impl AsRef<str> for EditDeobfuscationFileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EditDeobfuscationFileTypeEnum::Proguard => "proguard",
        }
    }
}

impl std::convert::TryFrom< &str> for EditDeobfuscationFileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "proguard" => Ok(EditDeobfuscationFileTypeEnum::Proguard),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EditDeobfuscationFileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EditExpansionFileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum EditExpansionFileTypeEnum {
    
    /// "main"
    #[serde(rename="main")]
    Main,
    
    /// "patch"
    #[serde(rename="patch")]
    Patch,
}

impl AsRef<str> for EditExpansionFileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EditExpansionFileTypeEnum::Main => "main",
            EditExpansionFileTypeEnum::Patch => "patch",
        }
    }
}

impl std::convert::TryFrom< &str> for EditExpansionFileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "main" => Ok(EditExpansionFileTypeEnum::Main),
           "patch" => Ok(EditExpansionFileTypeEnum::Patch),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EditExpansionFileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EditImageTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum EditImageTypeEnum {
    
    /// "featureGraphic"
    #[serde(rename="featureGraphic")]
    FeatureGraphic,
    
    /// "icon"
    #[serde(rename="icon")]
    Icon,
    
    /// "phoneScreenshots"
    #[serde(rename="phoneScreenshots")]
    PhoneScreenshots,
    
    /// "promoGraphic"
    #[serde(rename="promoGraphic")]
    PromoGraphic,
    
    /// "sevenInchScreenshots"
    #[serde(rename="sevenInchScreenshots")]
    SevenInchScreenshots,
    
    /// "tenInchScreenshots"
    #[serde(rename="tenInchScreenshots")]
    TenInchScreenshots,
    
    /// "tvBanner"
    #[serde(rename="tvBanner")]
    TvBanner,
    
    /// "tvScreenshots"
    #[serde(rename="tvScreenshots")]
    TvScreenshots,
    
    /// "wearScreenshots"
    #[serde(rename="wearScreenshots")]
    WearScreenshots,
}

impl AsRef<str> for EditImageTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EditImageTypeEnum::FeatureGraphic => "featureGraphic",
            EditImageTypeEnum::Icon => "icon",
            EditImageTypeEnum::PhoneScreenshots => "phoneScreenshots",
            EditImageTypeEnum::PromoGraphic => "promoGraphic",
            EditImageTypeEnum::SevenInchScreenshots => "sevenInchScreenshots",
            EditImageTypeEnum::TenInchScreenshots => "tenInchScreenshots",
            EditImageTypeEnum::TvBanner => "tvBanner",
            EditImageTypeEnum::TvScreenshots => "tvScreenshots",
            EditImageTypeEnum::WearScreenshots => "wearScreenshots",
        }
    }
}

impl std::convert::TryFrom< &str> for EditImageTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "featureGraphic" => Ok(EditImageTypeEnum::FeatureGraphic),
           "icon" => Ok(EditImageTypeEnum::Icon),
           "phoneScreenshots" => Ok(EditImageTypeEnum::PhoneScreenshots),
           "promoGraphic" => Ok(EditImageTypeEnum::PromoGraphic),
           "sevenInchScreenshots" => Ok(EditImageTypeEnum::SevenInchScreenshots),
           "tenInchScreenshots" => Ok(EditImageTypeEnum::TenInchScreenshots),
           "tvBanner" => Ok(EditImageTypeEnum::TvBanner),
           "tvScreenshots" => Ok(EditImageTypeEnum::TvScreenshots),
           "wearScreenshots" => Ok(EditImageTypeEnum::WearScreenshots),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EditImageTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


