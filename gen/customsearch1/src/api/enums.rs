use super::*;



// region CseImgColorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Returns black and white, grayscale, transparent, or color images. Acceptable values are: * `"color"` * `"gray"` * `"mono"`: black and white * `"trans"`: transparent background
pub enum CseImgColorTypeEnum {
    

    /// No image color type specified.
    ///
    /// "imgColorTypeUndefined"
    #[serde(rename="imgColorTypeUndefined")]
    ImgColorTypeUndefined,
    

    /// Black and white images only.
    ///
    /// "mono"
    #[serde(rename="mono")]
    Mono,
    

    /// Grayscale images only.
    ///
    /// "gray"
    #[serde(rename="gray")]
    Gray,
    

    /// Color images only.
    ///
    /// "color"
    #[serde(rename="color")]
    Color,
    

    /// Images with transparent background
    ///
    /// "trans"
    #[serde(rename="trans")]
    Trans,
}

impl AsRef<str> for CseImgColorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CseImgColorTypeEnum::ImgColorTypeUndefined => "imgColorTypeUndefined",
            CseImgColorTypeEnum::Mono => "mono",
            CseImgColorTypeEnum::Gray => "gray",
            CseImgColorTypeEnum::Color => "color",
            CseImgColorTypeEnum::Trans => "trans",
        }
    }
}

impl std::convert::TryFrom< &str> for CseImgColorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "imgColorTypeUndefined" => Ok(CseImgColorTypeEnum::ImgColorTypeUndefined),
           "mono" => Ok(CseImgColorTypeEnum::Mono),
           "gray" => Ok(CseImgColorTypeEnum::Gray),
           "color" => Ok(CseImgColorTypeEnum::Color),
           "trans" => Ok(CseImgColorTypeEnum::Trans),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CseImgColorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CseImgDominantColorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Returns images of a specific dominant color. Acceptable values are: * `"black"` * `"blue"` * `"brown"` * `"gray"` * `"green"` * `"orange"` * `"pink"` * `"purple"` * `"red"` * `"teal"` * `"white"` * `"yellow"`
pub enum CseImgDominantColorEnum {
    

    /// No dominant color specified.
    ///
    /// "imgDominantColorUndefined"
    #[serde(rename="imgDominantColorUndefined")]
    ImgDominantColorUndefined,
    

    /// Predominantly black images only.
    ///
    /// "black"
    #[serde(rename="black")]
    Black,
    

    /// Predominantly blue images only.
    ///
    /// "blue"
    #[serde(rename="blue")]
    Blue,
    

    /// Predominantly brown images only.
    ///
    /// "brown"
    #[serde(rename="brown")]
    Brown,
    

    /// Predominantly gray images only.
    ///
    /// "gray"
    #[serde(rename="gray")]
    Gray,
    

    /// Predominantly green images only.
    ///
    /// "green"
    #[serde(rename="green")]
    Green,
    

    /// Predominantly orange images only.
    ///
    /// "orange"
    #[serde(rename="orange")]
    Orange,
    

    /// Predominantly pink images only.
    ///
    /// "pink"
    #[serde(rename="pink")]
    Pink,
    

    /// Predominantly purple images only.
    ///
    /// "purple"
    #[serde(rename="purple")]
    Purple,
    

    /// Predominantly red images only.
    ///
    /// "red"
    #[serde(rename="red")]
    Red,
    

    /// Predominantly teal images only.
    ///
    /// "teal"
    #[serde(rename="teal")]
    Teal,
    

    /// Predominantly white images only.
    ///
    /// "white"
    #[serde(rename="white")]
    White,
    

    /// Predominantly yellow images only.
    ///
    /// "yellow"
    #[serde(rename="yellow")]
    Yellow,
}

impl AsRef<str> for CseImgDominantColorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CseImgDominantColorEnum::ImgDominantColorUndefined => "imgDominantColorUndefined",
            CseImgDominantColorEnum::Black => "black",
            CseImgDominantColorEnum::Blue => "blue",
            CseImgDominantColorEnum::Brown => "brown",
            CseImgDominantColorEnum::Gray => "gray",
            CseImgDominantColorEnum::Green => "green",
            CseImgDominantColorEnum::Orange => "orange",
            CseImgDominantColorEnum::Pink => "pink",
            CseImgDominantColorEnum::Purple => "purple",
            CseImgDominantColorEnum::Red => "red",
            CseImgDominantColorEnum::Teal => "teal",
            CseImgDominantColorEnum::White => "white",
            CseImgDominantColorEnum::Yellow => "yellow",
        }
    }
}

impl std::convert::TryFrom< &str> for CseImgDominantColorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "imgDominantColorUndefined" => Ok(CseImgDominantColorEnum::ImgDominantColorUndefined),
           "black" => Ok(CseImgDominantColorEnum::Black),
           "blue" => Ok(CseImgDominantColorEnum::Blue),
           "brown" => Ok(CseImgDominantColorEnum::Brown),
           "gray" => Ok(CseImgDominantColorEnum::Gray),
           "green" => Ok(CseImgDominantColorEnum::Green),
           "orange" => Ok(CseImgDominantColorEnum::Orange),
           "pink" => Ok(CseImgDominantColorEnum::Pink),
           "purple" => Ok(CseImgDominantColorEnum::Purple),
           "red" => Ok(CseImgDominantColorEnum::Red),
           "teal" => Ok(CseImgDominantColorEnum::Teal),
           "white" => Ok(CseImgDominantColorEnum::White),
           "yellow" => Ok(CseImgDominantColorEnum::Yellow),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CseImgDominantColorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CseImgSizeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Returns images of a specified size. Acceptable values are: * `"huge"` * `"icon"` * `"large"` * `"medium"` * `"small"` * `"xlarge"` * `"xxlarge"`
pub enum CseImgSizeEnum {
    

    /// No image size specified.
    ///
    /// "imgSizeUndefined"
    #[serde(rename="imgSizeUndefined")]
    ImgSizeUndefined,
    

    /// Only the largest possible images.
    ///
    /// "HUGE"
    #[serde(rename="HUGE")]
    HUGE,
    

    /// Only very small icon-sized images.
    ///
    /// "ICON"
    #[serde(rename="ICON")]
    ICON,
    

    /// Only large images.
    ///
    /// "LARGE"
    #[serde(rename="LARGE")]
    LARGE,
    

    /// Only medium images.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Only small images.
    ///
    /// "SMALL"
    #[serde(rename="SMALL")]
    SMALL,
    

    /// Only very large images.
    ///
    /// "XLARGE"
    #[serde(rename="XLARGE")]
    XLARGE,
    

    /// Only extremely large images.
    ///
    /// "XXLARGE"
    #[serde(rename="XXLARGE")]
    XXLARGE,
}

impl AsRef<str> for CseImgSizeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CseImgSizeEnum::ImgSizeUndefined => "imgSizeUndefined",
            CseImgSizeEnum::HUGE => "HUGE",
            CseImgSizeEnum::ICON => "ICON",
            CseImgSizeEnum::LARGE => "LARGE",
            CseImgSizeEnum::MEDIUM => "MEDIUM",
            CseImgSizeEnum::SMALL => "SMALL",
            CseImgSizeEnum::XLARGE => "XLARGE",
            CseImgSizeEnum::XXLARGE => "XXLARGE",
        }
    }
}

impl std::convert::TryFrom< &str> for CseImgSizeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "imgSizeUndefined" => Ok(CseImgSizeEnum::ImgSizeUndefined),
           "HUGE" => Ok(CseImgSizeEnum::HUGE),
           "ICON" => Ok(CseImgSizeEnum::ICON),
           "LARGE" => Ok(CseImgSizeEnum::LARGE),
           "MEDIUM" => Ok(CseImgSizeEnum::MEDIUM),
           "SMALL" => Ok(CseImgSizeEnum::SMALL),
           "XLARGE" => Ok(CseImgSizeEnum::XLARGE),
           "XXLARGE" => Ok(CseImgSizeEnum::XXLARGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CseImgSizeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CseImgTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Returns images of a type. Acceptable values are: * `"clipart"` * `"face"` * `"lineart"` * `"stock"` * `"photo"` * `"animated"`
pub enum CseImgTypeEnum {
    

    /// No image type specified.
    ///
    /// "imgTypeUndefined"
    #[serde(rename="imgTypeUndefined")]
    ImgTypeUndefined,
    

    /// Clipart-style images only.
    ///
    /// "clipart"
    #[serde(rename="clipart")]
    Clipart,
    

    /// Images of faces only.
    ///
    /// "face"
    #[serde(rename="face")]
    Face,
    

    /// Line art images only.
    ///
    /// "lineart"
    #[serde(rename="lineart")]
    Lineart,
    

    /// Stock images only.
    ///
    /// "stock"
    #[serde(rename="stock")]
    Stock,
    

    /// Photo images only.
    ///
    /// "photo"
    #[serde(rename="photo")]
    Photo,
    

    /// Animated images only.
    ///
    /// "animated"
    #[serde(rename="animated")]
    Animated,
}

impl AsRef<str> for CseImgTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CseImgTypeEnum::ImgTypeUndefined => "imgTypeUndefined",
            CseImgTypeEnum::Clipart => "clipart",
            CseImgTypeEnum::Face => "face",
            CseImgTypeEnum::Lineart => "lineart",
            CseImgTypeEnum::Stock => "stock",
            CseImgTypeEnum::Photo => "photo",
            CseImgTypeEnum::Animated => "animated",
        }
    }
}

impl std::convert::TryFrom< &str> for CseImgTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "imgTypeUndefined" => Ok(CseImgTypeEnum::ImgTypeUndefined),
           "clipart" => Ok(CseImgTypeEnum::Clipart),
           "face" => Ok(CseImgTypeEnum::Face),
           "lineart" => Ok(CseImgTypeEnum::Lineart),
           "stock" => Ok(CseImgTypeEnum::Stock),
           "photo" => Ok(CseImgTypeEnum::Photo),
           "animated" => Ok(CseImgTypeEnum::Animated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CseImgTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CseSafeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Search safety level. Acceptable values are: * `"active"`: Enables SafeSearch filtering. * `"off"`: Disables SafeSearch filtering. (default)
pub enum CseSafeEnum {
    

    /// SafeSearch mode unspecified. (Falls back to engine's configuration.)
    ///
    /// "safeUndefined"
    #[serde(rename="safeUndefined")]
    SafeUndefined,
    

    /// Turn SafeSearch on.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    

    /// Deprecated, equivalent to "active".
    ///
    /// "high"
    #[serde(rename="high")]
    High,
    

    /// Deprecated, equivalent to "active".
    ///
    /// "medium"
    #[serde(rename="medium")]
    Medium,
    

    /// Turn SafeSearch off.
    ///
    /// "off"
    #[serde(rename="off")]
    Off,
}

impl AsRef<str> for CseSafeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CseSafeEnum::SafeUndefined => "safeUndefined",
            CseSafeEnum::Active => "active",
            CseSafeEnum::High => "high",
            CseSafeEnum::Medium => "medium",
            CseSafeEnum::Off => "off",
        }
    }
}

impl std::convert::TryFrom< &str> for CseSafeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "safeUndefined" => Ok(CseSafeEnum::SafeUndefined),
           "active" => Ok(CseSafeEnum::Active),
           "high" => Ok(CseSafeEnum::High),
           "medium" => Ok(CseSafeEnum::Medium),
           "off" => Ok(CseSafeEnum::Off),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CseSafeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CseSearchTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the search type: `image`. If unspecified, results are limited to webpages. Acceptable values are: * `"image"`: custom image search.
pub enum CseSearchTypeEnum {
    

    /// Search type unspecified (defaults to web search).
    ///
    /// "searchTypeUndefined"
    #[serde(rename="searchTypeUndefined")]
    SearchTypeUndefined,
    

    /// Image search.
    ///
    /// "image"
    #[serde(rename="image")]
    Image,
}

impl AsRef<str> for CseSearchTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CseSearchTypeEnum::SearchTypeUndefined => "searchTypeUndefined",
            CseSearchTypeEnum::Image => "image",
        }
    }
}

impl std::convert::TryFrom< &str> for CseSearchTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "searchTypeUndefined" => Ok(CseSearchTypeEnum::SearchTypeUndefined),
           "image" => Ok(CseSearchTypeEnum::Image),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CseSearchTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CseSiteSearchFilterEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls whether to include or exclude results from the site named in the `siteSearch` parameter. Acceptable values are: * `"e"`: exclude * `"i"`: include
pub enum CseSiteSearchFilterEnum {
    

    /// Filter mode unspecified.
    ///
    /// "siteSearchFilterUndefined"
    #[serde(rename="siteSearchFilterUndefined")]
    SiteSearchFilterUndefined,
    

    /// Exclude results from the listed sites.
    ///
    /// "e"
    #[serde(rename="e")]
    E,
    

    /// Include only results from the listed sites.
    ///
    /// "i"
    #[serde(rename="i")]
    I,
}

impl AsRef<str> for CseSiteSearchFilterEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CseSiteSearchFilterEnum::SiteSearchFilterUndefined => "siteSearchFilterUndefined",
            CseSiteSearchFilterEnum::E => "e",
            CseSiteSearchFilterEnum::I => "i",
        }
    }
}

impl std::convert::TryFrom< &str> for CseSiteSearchFilterEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "siteSearchFilterUndefined" => Ok(CseSiteSearchFilterEnum::SiteSearchFilterUndefined),
           "e" => Ok(CseSiteSearchFilterEnum::E),
           "i" => Ok(CseSiteSearchFilterEnum::I),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CseSiteSearchFilterEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


