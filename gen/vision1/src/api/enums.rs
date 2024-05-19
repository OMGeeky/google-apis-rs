use super::*;



// region BlockBlockTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Detected block type (text, image etc) for this block.
pub enum BlockBlockTypeEnum {
    

    /// Unknown block type.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Regular text block.
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// Table block.
    ///
    /// "TABLE"
    #[serde(rename="TABLE")]
    TABLE,
    

    /// Image block.
    ///
    /// "PICTURE"
    #[serde(rename="PICTURE")]
    PICTURE,
    

    /// Horizontal/vertical line box.
    ///
    /// "RULER"
    #[serde(rename="RULER")]
    RULER,
    

    /// Barcode block.
    ///
    /// "BARCODE"
    #[serde(rename="BARCODE")]
    BARCODE,
}

impl AsRef<str> for BlockBlockTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BlockBlockTypeEnum::UNKNOWN => "UNKNOWN",
            BlockBlockTypeEnum::TEXT => "TEXT",
            BlockBlockTypeEnum::TABLE => "TABLE",
            BlockBlockTypeEnum::PICTURE => "PICTURE",
            BlockBlockTypeEnum::RULER => "RULER",
            BlockBlockTypeEnum::BARCODE => "BARCODE",
        }
    }
}

impl std::convert::TryFrom< &str> for BlockBlockTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(BlockBlockTypeEnum::UNKNOWN),
           "TEXT" => Ok(BlockBlockTypeEnum::TEXT),
           "TABLE" => Ok(BlockBlockTypeEnum::TABLE),
           "PICTURE" => Ok(BlockBlockTypeEnum::PICTURE),
           "RULER" => Ok(BlockBlockTypeEnum::RULER),
           "BARCODE" => Ok(BlockBlockTypeEnum::BARCODE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BlockBlockTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DetectedBreakTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Detected break type.
pub enum DetectedBreakTypeEnum {
    

    /// Unknown break label type.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Regular space.
    ///
    /// "SPACE"
    #[serde(rename="SPACE")]
    SPACE,
    

    /// Sure space (very wide).
    ///
    /// "SURE_SPACE"
    #[serde(rename="SURE_SPACE")]
    SURESPACE,
    

    /// Line-wrapping break.
    ///
    /// "EOL_SURE_SPACE"
    #[serde(rename="EOL_SURE_SPACE")]
    EOLSURESPACE,
    

    /// End-line hyphen that is not present in text; does not co-occur with `SPACE`, `LEADER_SPACE`, or `LINE_BREAK`.
    ///
    /// "HYPHEN"
    #[serde(rename="HYPHEN")]
    HYPHEN,
    

    /// Line break that ends a paragraph.
    ///
    /// "LINE_BREAK"
    #[serde(rename="LINE_BREAK")]
    LINEBREAK,
}

impl AsRef<str> for DetectedBreakTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DetectedBreakTypeEnum::UNKNOWN => "UNKNOWN",
            DetectedBreakTypeEnum::SPACE => "SPACE",
            DetectedBreakTypeEnum::SURESPACE => "SURE_SPACE",
            DetectedBreakTypeEnum::EOLSURESPACE => "EOL_SURE_SPACE",
            DetectedBreakTypeEnum::HYPHEN => "HYPHEN",
            DetectedBreakTypeEnum::LINEBREAK => "LINE_BREAK",
        }
    }
}

impl std::convert::TryFrom< &str> for DetectedBreakTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(DetectedBreakTypeEnum::UNKNOWN),
           "SPACE" => Ok(DetectedBreakTypeEnum::SPACE),
           "SURE_SPACE" => Ok(DetectedBreakTypeEnum::SURESPACE),
           "EOL_SURE_SPACE" => Ok(DetectedBreakTypeEnum::EOLSURESPACE),
           "HYPHEN" => Ok(DetectedBreakTypeEnum::HYPHEN),
           "LINE_BREAK" => Ok(DetectedBreakTypeEnum::LINEBREAK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DetectedBreakTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FaceAnnotationAngerLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Anger likelihood.
pub enum FaceAnnotationAngerLikelihoodEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for FaceAnnotationAngerLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FaceAnnotationAngerLikelihoodEnum::UNKNOWN => "UNKNOWN",
            FaceAnnotationAngerLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            FaceAnnotationAngerLikelihoodEnum::UNLIKELY => "UNLIKELY",
            FaceAnnotationAngerLikelihoodEnum::POSSIBLE => "POSSIBLE",
            FaceAnnotationAngerLikelihoodEnum::LIKELY => "LIKELY",
            FaceAnnotationAngerLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for FaceAnnotationAngerLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(FaceAnnotationAngerLikelihoodEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(FaceAnnotationAngerLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(FaceAnnotationAngerLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(FaceAnnotationAngerLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(FaceAnnotationAngerLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(FaceAnnotationAngerLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FaceAnnotationAngerLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FaceAnnotationBlurredLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Blurred likelihood.
pub enum FaceAnnotationBlurredLikelihoodEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for FaceAnnotationBlurredLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FaceAnnotationBlurredLikelihoodEnum::UNKNOWN => "UNKNOWN",
            FaceAnnotationBlurredLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            FaceAnnotationBlurredLikelihoodEnum::UNLIKELY => "UNLIKELY",
            FaceAnnotationBlurredLikelihoodEnum::POSSIBLE => "POSSIBLE",
            FaceAnnotationBlurredLikelihoodEnum::LIKELY => "LIKELY",
            FaceAnnotationBlurredLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for FaceAnnotationBlurredLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(FaceAnnotationBlurredLikelihoodEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(FaceAnnotationBlurredLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(FaceAnnotationBlurredLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(FaceAnnotationBlurredLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(FaceAnnotationBlurredLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(FaceAnnotationBlurredLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FaceAnnotationBlurredLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FaceAnnotationHeadwearLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Headwear likelihood.
pub enum FaceAnnotationHeadwearLikelihoodEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for FaceAnnotationHeadwearLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FaceAnnotationHeadwearLikelihoodEnum::UNKNOWN => "UNKNOWN",
            FaceAnnotationHeadwearLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            FaceAnnotationHeadwearLikelihoodEnum::UNLIKELY => "UNLIKELY",
            FaceAnnotationHeadwearLikelihoodEnum::POSSIBLE => "POSSIBLE",
            FaceAnnotationHeadwearLikelihoodEnum::LIKELY => "LIKELY",
            FaceAnnotationHeadwearLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for FaceAnnotationHeadwearLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(FaceAnnotationHeadwearLikelihoodEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(FaceAnnotationHeadwearLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(FaceAnnotationHeadwearLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(FaceAnnotationHeadwearLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(FaceAnnotationHeadwearLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(FaceAnnotationHeadwearLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FaceAnnotationHeadwearLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FaceAnnotationJoyLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Joy likelihood.
pub enum FaceAnnotationJoyLikelihoodEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for FaceAnnotationJoyLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FaceAnnotationJoyLikelihoodEnum::UNKNOWN => "UNKNOWN",
            FaceAnnotationJoyLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            FaceAnnotationJoyLikelihoodEnum::UNLIKELY => "UNLIKELY",
            FaceAnnotationJoyLikelihoodEnum::POSSIBLE => "POSSIBLE",
            FaceAnnotationJoyLikelihoodEnum::LIKELY => "LIKELY",
            FaceAnnotationJoyLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for FaceAnnotationJoyLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(FaceAnnotationJoyLikelihoodEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(FaceAnnotationJoyLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(FaceAnnotationJoyLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(FaceAnnotationJoyLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(FaceAnnotationJoyLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(FaceAnnotationJoyLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FaceAnnotationJoyLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FaceAnnotationSorrowLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sorrow likelihood.
pub enum FaceAnnotationSorrowLikelihoodEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for FaceAnnotationSorrowLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FaceAnnotationSorrowLikelihoodEnum::UNKNOWN => "UNKNOWN",
            FaceAnnotationSorrowLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            FaceAnnotationSorrowLikelihoodEnum::UNLIKELY => "UNLIKELY",
            FaceAnnotationSorrowLikelihoodEnum::POSSIBLE => "POSSIBLE",
            FaceAnnotationSorrowLikelihoodEnum::LIKELY => "LIKELY",
            FaceAnnotationSorrowLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for FaceAnnotationSorrowLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(FaceAnnotationSorrowLikelihoodEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(FaceAnnotationSorrowLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(FaceAnnotationSorrowLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(FaceAnnotationSorrowLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(FaceAnnotationSorrowLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(FaceAnnotationSorrowLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FaceAnnotationSorrowLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FaceAnnotationSurpriseLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Surprise likelihood.
pub enum FaceAnnotationSurpriseLikelihoodEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for FaceAnnotationSurpriseLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FaceAnnotationSurpriseLikelihoodEnum::UNKNOWN => "UNKNOWN",
            FaceAnnotationSurpriseLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            FaceAnnotationSurpriseLikelihoodEnum::UNLIKELY => "UNLIKELY",
            FaceAnnotationSurpriseLikelihoodEnum::POSSIBLE => "POSSIBLE",
            FaceAnnotationSurpriseLikelihoodEnum::LIKELY => "LIKELY",
            FaceAnnotationSurpriseLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for FaceAnnotationSurpriseLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(FaceAnnotationSurpriseLikelihoodEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(FaceAnnotationSurpriseLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(FaceAnnotationSurpriseLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(FaceAnnotationSurpriseLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(FaceAnnotationSurpriseLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(FaceAnnotationSurpriseLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FaceAnnotationSurpriseLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FaceAnnotationUnderExposedLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Under-exposed likelihood.
pub enum FaceAnnotationUnderExposedLikelihoodEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for FaceAnnotationUnderExposedLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FaceAnnotationUnderExposedLikelihoodEnum::UNKNOWN => "UNKNOWN",
            FaceAnnotationUnderExposedLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            FaceAnnotationUnderExposedLikelihoodEnum::UNLIKELY => "UNLIKELY",
            FaceAnnotationUnderExposedLikelihoodEnum::POSSIBLE => "POSSIBLE",
            FaceAnnotationUnderExposedLikelihoodEnum::LIKELY => "LIKELY",
            FaceAnnotationUnderExposedLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for FaceAnnotationUnderExposedLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(FaceAnnotationUnderExposedLikelihoodEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(FaceAnnotationUnderExposedLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(FaceAnnotationUnderExposedLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(FaceAnnotationUnderExposedLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(FaceAnnotationUnderExposedLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(FaceAnnotationUnderExposedLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FaceAnnotationUnderExposedLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FeatureTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The feature type.
pub enum FeatureTypeEnum {
    

    /// Unspecified feature type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Run face detection.
    ///
    /// "FACE_DETECTION"
    #[serde(rename="FACE_DETECTION")]
    FACEDETECTION,
    

    /// Run landmark detection.
    ///
    /// "LANDMARK_DETECTION"
    #[serde(rename="LANDMARK_DETECTION")]
    LANDMARKDETECTION,
    

    /// Run logo detection.
    ///
    /// "LOGO_DETECTION"
    #[serde(rename="LOGO_DETECTION")]
    LOGODETECTION,
    

    /// Run label detection.
    ///
    /// "LABEL_DETECTION"
    #[serde(rename="LABEL_DETECTION")]
    LABELDETECTION,
    

    /// Run text detection / optical character recognition (OCR). Text detection is optimized for areas of text within a larger image; if the image is a document, use `DOCUMENT_TEXT_DETECTION` instead.
    ///
    /// "TEXT_DETECTION"
    #[serde(rename="TEXT_DETECTION")]
    TEXTDETECTION,
    

    /// Run dense text document OCR. Takes precedence when both `DOCUMENT_TEXT_DETECTION` and `TEXT_DETECTION` are present.
    ///
    /// "DOCUMENT_TEXT_DETECTION"
    #[serde(rename="DOCUMENT_TEXT_DETECTION")]
    DOCUMENTTEXTDETECTION,
    

    /// Run Safe Search to detect potentially unsafe or undesirable content.
    ///
    /// "SAFE_SEARCH_DETECTION"
    #[serde(rename="SAFE_SEARCH_DETECTION")]
    SAFESEARCHDETECTION,
    

    /// Compute a set of image properties, such as the image's dominant colors.
    ///
    /// "IMAGE_PROPERTIES"
    #[serde(rename="IMAGE_PROPERTIES")]
    IMAGEPROPERTIES,
    

    /// Run crop hints.
    ///
    /// "CROP_HINTS"
    #[serde(rename="CROP_HINTS")]
    CROPHINTS,
    

    /// Run web detection.
    ///
    /// "WEB_DETECTION"
    #[serde(rename="WEB_DETECTION")]
    WEBDETECTION,
    

    /// Run Product Search.
    ///
    /// "PRODUCT_SEARCH"
    #[serde(rename="PRODUCT_SEARCH")]
    PRODUCTSEARCH,
    

    /// Run localizer for object detection.
    ///
    /// "OBJECT_LOCALIZATION"
    #[serde(rename="OBJECT_LOCALIZATION")]
    OBJECTLOCALIZATION,
}

impl AsRef<str> for FeatureTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FeatureTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            FeatureTypeEnum::FACEDETECTION => "FACE_DETECTION",
            FeatureTypeEnum::LANDMARKDETECTION => "LANDMARK_DETECTION",
            FeatureTypeEnum::LOGODETECTION => "LOGO_DETECTION",
            FeatureTypeEnum::LABELDETECTION => "LABEL_DETECTION",
            FeatureTypeEnum::TEXTDETECTION => "TEXT_DETECTION",
            FeatureTypeEnum::DOCUMENTTEXTDETECTION => "DOCUMENT_TEXT_DETECTION",
            FeatureTypeEnum::SAFESEARCHDETECTION => "SAFE_SEARCH_DETECTION",
            FeatureTypeEnum::IMAGEPROPERTIES => "IMAGE_PROPERTIES",
            FeatureTypeEnum::CROPHINTS => "CROP_HINTS",
            FeatureTypeEnum::WEBDETECTION => "WEB_DETECTION",
            FeatureTypeEnum::PRODUCTSEARCH => "PRODUCT_SEARCH",
            FeatureTypeEnum::OBJECTLOCALIZATION => "OBJECT_LOCALIZATION",
        }
    }
}

impl std::convert::TryFrom< &str> for FeatureTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(FeatureTypeEnum::TYPEUNSPECIFIED),
           "FACE_DETECTION" => Ok(FeatureTypeEnum::FACEDETECTION),
           "LANDMARK_DETECTION" => Ok(FeatureTypeEnum::LANDMARKDETECTION),
           "LOGO_DETECTION" => Ok(FeatureTypeEnum::LOGODETECTION),
           "LABEL_DETECTION" => Ok(FeatureTypeEnum::LABELDETECTION),
           "TEXT_DETECTION" => Ok(FeatureTypeEnum::TEXTDETECTION),
           "DOCUMENT_TEXT_DETECTION" => Ok(FeatureTypeEnum::DOCUMENTTEXTDETECTION),
           "SAFE_SEARCH_DETECTION" => Ok(FeatureTypeEnum::SAFESEARCHDETECTION),
           "IMAGE_PROPERTIES" => Ok(FeatureTypeEnum::IMAGEPROPERTIES),
           "CROP_HINTS" => Ok(FeatureTypeEnum::CROPHINTS),
           "WEB_DETECTION" => Ok(FeatureTypeEnum::WEBDETECTION),
           "PRODUCT_SEARCH" => Ok(FeatureTypeEnum::PRODUCTSEARCH),
           "OBJECT_LOCALIZATION" => Ok(FeatureTypeEnum::OBJECTLOCALIZATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FeatureTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LandmarkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Face landmark type.
pub enum LandmarkTypeEnum {
    

    /// Unknown face landmark detected. Should not be filled.
    ///
    /// "UNKNOWN_LANDMARK"
    #[serde(rename="UNKNOWN_LANDMARK")]
    UNKNOWNLANDMARK,
    

    /// Left eye.
    ///
    /// "LEFT_EYE"
    #[serde(rename="LEFT_EYE")]
    LEFTEYE,
    

    /// Right eye.
    ///
    /// "RIGHT_EYE"
    #[serde(rename="RIGHT_EYE")]
    RIGHTEYE,
    

    /// Left of left eyebrow.
    ///
    /// "LEFT_OF_LEFT_EYEBROW"
    #[serde(rename="LEFT_OF_LEFT_EYEBROW")]
    LEFTOFLEFTEYEBROW,
    

    /// Right of left eyebrow.
    ///
    /// "RIGHT_OF_LEFT_EYEBROW"
    #[serde(rename="RIGHT_OF_LEFT_EYEBROW")]
    RIGHTOFLEFTEYEBROW,
    

    /// Left of right eyebrow.
    ///
    /// "LEFT_OF_RIGHT_EYEBROW"
    #[serde(rename="LEFT_OF_RIGHT_EYEBROW")]
    LEFTOFRIGHTEYEBROW,
    

    /// Right of right eyebrow.
    ///
    /// "RIGHT_OF_RIGHT_EYEBROW"
    #[serde(rename="RIGHT_OF_RIGHT_EYEBROW")]
    RIGHTOFRIGHTEYEBROW,
    

    /// Midpoint between eyes.
    ///
    /// "MIDPOINT_BETWEEN_EYES"
    #[serde(rename="MIDPOINT_BETWEEN_EYES")]
    MIDPOINTBETWEENEYES,
    

    /// Nose tip.
    ///
    /// "NOSE_TIP"
    #[serde(rename="NOSE_TIP")]
    NOSETIP,
    

    /// Upper lip.
    ///
    /// "UPPER_LIP"
    #[serde(rename="UPPER_LIP")]
    UPPERLIP,
    

    /// Lower lip.
    ///
    /// "LOWER_LIP"
    #[serde(rename="LOWER_LIP")]
    LOWERLIP,
    

    /// Mouth left.
    ///
    /// "MOUTH_LEFT"
    #[serde(rename="MOUTH_LEFT")]
    MOUTHLEFT,
    

    /// Mouth right.
    ///
    /// "MOUTH_RIGHT"
    #[serde(rename="MOUTH_RIGHT")]
    MOUTHRIGHT,
    

    /// Mouth center.
    ///
    /// "MOUTH_CENTER"
    #[serde(rename="MOUTH_CENTER")]
    MOUTHCENTER,
    

    /// Nose, bottom right.
    ///
    /// "NOSE_BOTTOM_RIGHT"
    #[serde(rename="NOSE_BOTTOM_RIGHT")]
    NOSEBOTTOMRIGHT,
    

    /// Nose, bottom left.
    ///
    /// "NOSE_BOTTOM_LEFT"
    #[serde(rename="NOSE_BOTTOM_LEFT")]
    NOSEBOTTOMLEFT,
    

    /// Nose, bottom center.
    ///
    /// "NOSE_BOTTOM_CENTER"
    #[serde(rename="NOSE_BOTTOM_CENTER")]
    NOSEBOTTOMCENTER,
    

    /// Left eye, top boundary.
    ///
    /// "LEFT_EYE_TOP_BOUNDARY"
    #[serde(rename="LEFT_EYE_TOP_BOUNDARY")]
    LEFTEYETOPBOUNDARY,
    

    /// Left eye, right corner.
    ///
    /// "LEFT_EYE_RIGHT_CORNER"
    #[serde(rename="LEFT_EYE_RIGHT_CORNER")]
    LEFTEYERIGHTCORNER,
    

    /// Left eye, bottom boundary.
    ///
    /// "LEFT_EYE_BOTTOM_BOUNDARY"
    #[serde(rename="LEFT_EYE_BOTTOM_BOUNDARY")]
    LEFTEYEBOTTOMBOUNDARY,
    

    /// Left eye, left corner.
    ///
    /// "LEFT_EYE_LEFT_CORNER"
    #[serde(rename="LEFT_EYE_LEFT_CORNER")]
    LEFTEYELEFTCORNER,
    

    /// Right eye, top boundary.
    ///
    /// "RIGHT_EYE_TOP_BOUNDARY"
    #[serde(rename="RIGHT_EYE_TOP_BOUNDARY")]
    RIGHTEYETOPBOUNDARY,
    

    /// Right eye, right corner.
    ///
    /// "RIGHT_EYE_RIGHT_CORNER"
    #[serde(rename="RIGHT_EYE_RIGHT_CORNER")]
    RIGHTEYERIGHTCORNER,
    

    /// Right eye, bottom boundary.
    ///
    /// "RIGHT_EYE_BOTTOM_BOUNDARY"
    #[serde(rename="RIGHT_EYE_BOTTOM_BOUNDARY")]
    RIGHTEYEBOTTOMBOUNDARY,
    

    /// Right eye, left corner.
    ///
    /// "RIGHT_EYE_LEFT_CORNER"
    #[serde(rename="RIGHT_EYE_LEFT_CORNER")]
    RIGHTEYELEFTCORNER,
    

    /// Left eyebrow, upper midpoint.
    ///
    /// "LEFT_EYEBROW_UPPER_MIDPOINT"
    #[serde(rename="LEFT_EYEBROW_UPPER_MIDPOINT")]
    LEFTEYEBROWUPPERMIDPOINT,
    

    /// Right eyebrow, upper midpoint.
    ///
    /// "RIGHT_EYEBROW_UPPER_MIDPOINT"
    #[serde(rename="RIGHT_EYEBROW_UPPER_MIDPOINT")]
    RIGHTEYEBROWUPPERMIDPOINT,
    

    /// Left ear tragion.
    ///
    /// "LEFT_EAR_TRAGION"
    #[serde(rename="LEFT_EAR_TRAGION")]
    LEFTEARTRAGION,
    

    /// Right ear tragion.
    ///
    /// "RIGHT_EAR_TRAGION"
    #[serde(rename="RIGHT_EAR_TRAGION")]
    RIGHTEARTRAGION,
    

    /// Left eye pupil.
    ///
    /// "LEFT_EYE_PUPIL"
    #[serde(rename="LEFT_EYE_PUPIL")]
    LEFTEYEPUPIL,
    

    /// Right eye pupil.
    ///
    /// "RIGHT_EYE_PUPIL"
    #[serde(rename="RIGHT_EYE_PUPIL")]
    RIGHTEYEPUPIL,
    

    /// Forehead glabella.
    ///
    /// "FOREHEAD_GLABELLA"
    #[serde(rename="FOREHEAD_GLABELLA")]
    FOREHEADGLABELLA,
    

    /// Chin gnathion.
    ///
    /// "CHIN_GNATHION"
    #[serde(rename="CHIN_GNATHION")]
    CHINGNATHION,
    

    /// Chin left gonion.
    ///
    /// "CHIN_LEFT_GONION"
    #[serde(rename="CHIN_LEFT_GONION")]
    CHINLEFTGONION,
    

    /// Chin right gonion.
    ///
    /// "CHIN_RIGHT_GONION"
    #[serde(rename="CHIN_RIGHT_GONION")]
    CHINRIGHTGONION,
    

    /// Left cheek center.
    ///
    /// "LEFT_CHEEK_CENTER"
    #[serde(rename="LEFT_CHEEK_CENTER")]
    LEFTCHEEKCENTER,
    

    /// Right cheek center.
    ///
    /// "RIGHT_CHEEK_CENTER"
    #[serde(rename="RIGHT_CHEEK_CENTER")]
    RIGHTCHEEKCENTER,
}

impl AsRef<str> for LandmarkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LandmarkTypeEnum::UNKNOWNLANDMARK => "UNKNOWN_LANDMARK",
            LandmarkTypeEnum::LEFTEYE => "LEFT_EYE",
            LandmarkTypeEnum::RIGHTEYE => "RIGHT_EYE",
            LandmarkTypeEnum::LEFTOFLEFTEYEBROW => "LEFT_OF_LEFT_EYEBROW",
            LandmarkTypeEnum::RIGHTOFLEFTEYEBROW => "RIGHT_OF_LEFT_EYEBROW",
            LandmarkTypeEnum::LEFTOFRIGHTEYEBROW => "LEFT_OF_RIGHT_EYEBROW",
            LandmarkTypeEnum::RIGHTOFRIGHTEYEBROW => "RIGHT_OF_RIGHT_EYEBROW",
            LandmarkTypeEnum::MIDPOINTBETWEENEYES => "MIDPOINT_BETWEEN_EYES",
            LandmarkTypeEnum::NOSETIP => "NOSE_TIP",
            LandmarkTypeEnum::UPPERLIP => "UPPER_LIP",
            LandmarkTypeEnum::LOWERLIP => "LOWER_LIP",
            LandmarkTypeEnum::MOUTHLEFT => "MOUTH_LEFT",
            LandmarkTypeEnum::MOUTHRIGHT => "MOUTH_RIGHT",
            LandmarkTypeEnum::MOUTHCENTER => "MOUTH_CENTER",
            LandmarkTypeEnum::NOSEBOTTOMRIGHT => "NOSE_BOTTOM_RIGHT",
            LandmarkTypeEnum::NOSEBOTTOMLEFT => "NOSE_BOTTOM_LEFT",
            LandmarkTypeEnum::NOSEBOTTOMCENTER => "NOSE_BOTTOM_CENTER",
            LandmarkTypeEnum::LEFTEYETOPBOUNDARY => "LEFT_EYE_TOP_BOUNDARY",
            LandmarkTypeEnum::LEFTEYERIGHTCORNER => "LEFT_EYE_RIGHT_CORNER",
            LandmarkTypeEnum::LEFTEYEBOTTOMBOUNDARY => "LEFT_EYE_BOTTOM_BOUNDARY",
            LandmarkTypeEnum::LEFTEYELEFTCORNER => "LEFT_EYE_LEFT_CORNER",
            LandmarkTypeEnum::RIGHTEYETOPBOUNDARY => "RIGHT_EYE_TOP_BOUNDARY",
            LandmarkTypeEnum::RIGHTEYERIGHTCORNER => "RIGHT_EYE_RIGHT_CORNER",
            LandmarkTypeEnum::RIGHTEYEBOTTOMBOUNDARY => "RIGHT_EYE_BOTTOM_BOUNDARY",
            LandmarkTypeEnum::RIGHTEYELEFTCORNER => "RIGHT_EYE_LEFT_CORNER",
            LandmarkTypeEnum::LEFTEYEBROWUPPERMIDPOINT => "LEFT_EYEBROW_UPPER_MIDPOINT",
            LandmarkTypeEnum::RIGHTEYEBROWUPPERMIDPOINT => "RIGHT_EYEBROW_UPPER_MIDPOINT",
            LandmarkTypeEnum::LEFTEARTRAGION => "LEFT_EAR_TRAGION",
            LandmarkTypeEnum::RIGHTEARTRAGION => "RIGHT_EAR_TRAGION",
            LandmarkTypeEnum::LEFTEYEPUPIL => "LEFT_EYE_PUPIL",
            LandmarkTypeEnum::RIGHTEYEPUPIL => "RIGHT_EYE_PUPIL",
            LandmarkTypeEnum::FOREHEADGLABELLA => "FOREHEAD_GLABELLA",
            LandmarkTypeEnum::CHINGNATHION => "CHIN_GNATHION",
            LandmarkTypeEnum::CHINLEFTGONION => "CHIN_LEFT_GONION",
            LandmarkTypeEnum::CHINRIGHTGONION => "CHIN_RIGHT_GONION",
            LandmarkTypeEnum::LEFTCHEEKCENTER => "LEFT_CHEEK_CENTER",
            LandmarkTypeEnum::RIGHTCHEEKCENTER => "RIGHT_CHEEK_CENTER",
        }
    }
}

impl std::convert::TryFrom< &str> for LandmarkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_LANDMARK" => Ok(LandmarkTypeEnum::UNKNOWNLANDMARK),
           "LEFT_EYE" => Ok(LandmarkTypeEnum::LEFTEYE),
           "RIGHT_EYE" => Ok(LandmarkTypeEnum::RIGHTEYE),
           "LEFT_OF_LEFT_EYEBROW" => Ok(LandmarkTypeEnum::LEFTOFLEFTEYEBROW),
           "RIGHT_OF_LEFT_EYEBROW" => Ok(LandmarkTypeEnum::RIGHTOFLEFTEYEBROW),
           "LEFT_OF_RIGHT_EYEBROW" => Ok(LandmarkTypeEnum::LEFTOFRIGHTEYEBROW),
           "RIGHT_OF_RIGHT_EYEBROW" => Ok(LandmarkTypeEnum::RIGHTOFRIGHTEYEBROW),
           "MIDPOINT_BETWEEN_EYES" => Ok(LandmarkTypeEnum::MIDPOINTBETWEENEYES),
           "NOSE_TIP" => Ok(LandmarkTypeEnum::NOSETIP),
           "UPPER_LIP" => Ok(LandmarkTypeEnum::UPPERLIP),
           "LOWER_LIP" => Ok(LandmarkTypeEnum::LOWERLIP),
           "MOUTH_LEFT" => Ok(LandmarkTypeEnum::MOUTHLEFT),
           "MOUTH_RIGHT" => Ok(LandmarkTypeEnum::MOUTHRIGHT),
           "MOUTH_CENTER" => Ok(LandmarkTypeEnum::MOUTHCENTER),
           "NOSE_BOTTOM_RIGHT" => Ok(LandmarkTypeEnum::NOSEBOTTOMRIGHT),
           "NOSE_BOTTOM_LEFT" => Ok(LandmarkTypeEnum::NOSEBOTTOMLEFT),
           "NOSE_BOTTOM_CENTER" => Ok(LandmarkTypeEnum::NOSEBOTTOMCENTER),
           "LEFT_EYE_TOP_BOUNDARY" => Ok(LandmarkTypeEnum::LEFTEYETOPBOUNDARY),
           "LEFT_EYE_RIGHT_CORNER" => Ok(LandmarkTypeEnum::LEFTEYERIGHTCORNER),
           "LEFT_EYE_BOTTOM_BOUNDARY" => Ok(LandmarkTypeEnum::LEFTEYEBOTTOMBOUNDARY),
           "LEFT_EYE_LEFT_CORNER" => Ok(LandmarkTypeEnum::LEFTEYELEFTCORNER),
           "RIGHT_EYE_TOP_BOUNDARY" => Ok(LandmarkTypeEnum::RIGHTEYETOPBOUNDARY),
           "RIGHT_EYE_RIGHT_CORNER" => Ok(LandmarkTypeEnum::RIGHTEYERIGHTCORNER),
           "RIGHT_EYE_BOTTOM_BOUNDARY" => Ok(LandmarkTypeEnum::RIGHTEYEBOTTOMBOUNDARY),
           "RIGHT_EYE_LEFT_CORNER" => Ok(LandmarkTypeEnum::RIGHTEYELEFTCORNER),
           "LEFT_EYEBROW_UPPER_MIDPOINT" => Ok(LandmarkTypeEnum::LEFTEYEBROWUPPERMIDPOINT),
           "RIGHT_EYEBROW_UPPER_MIDPOINT" => Ok(LandmarkTypeEnum::RIGHTEYEBROWUPPERMIDPOINT),
           "LEFT_EAR_TRAGION" => Ok(LandmarkTypeEnum::LEFTEARTRAGION),
           "RIGHT_EAR_TRAGION" => Ok(LandmarkTypeEnum::RIGHTEARTRAGION),
           "LEFT_EYE_PUPIL" => Ok(LandmarkTypeEnum::LEFTEYEPUPIL),
           "RIGHT_EYE_PUPIL" => Ok(LandmarkTypeEnum::RIGHTEYEPUPIL),
           "FOREHEAD_GLABELLA" => Ok(LandmarkTypeEnum::FOREHEADGLABELLA),
           "CHIN_GNATHION" => Ok(LandmarkTypeEnum::CHINGNATHION),
           "CHIN_LEFT_GONION" => Ok(LandmarkTypeEnum::CHINLEFTGONION),
           "CHIN_RIGHT_GONION" => Ok(LandmarkTypeEnum::CHINRIGHTGONION),
           "LEFT_CHEEK_CENTER" => Ok(LandmarkTypeEnum::LEFTCHEEKCENTER),
           "RIGHT_CHEEK_CENTER" => Ok(LandmarkTypeEnum::RIGHTCHEEKCENTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LandmarkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SafeSearchAnnotationAdultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities.
pub enum SafeSearchAnnotationAdultEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for SafeSearchAnnotationAdultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SafeSearchAnnotationAdultEnum::UNKNOWN => "UNKNOWN",
            SafeSearchAnnotationAdultEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            SafeSearchAnnotationAdultEnum::UNLIKELY => "UNLIKELY",
            SafeSearchAnnotationAdultEnum::POSSIBLE => "POSSIBLE",
            SafeSearchAnnotationAdultEnum::LIKELY => "LIKELY",
            SafeSearchAnnotationAdultEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for SafeSearchAnnotationAdultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(SafeSearchAnnotationAdultEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(SafeSearchAnnotationAdultEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(SafeSearchAnnotationAdultEnum::UNLIKELY),
           "POSSIBLE" => Ok(SafeSearchAnnotationAdultEnum::POSSIBLE),
           "LIKELY" => Ok(SafeSearchAnnotationAdultEnum::LIKELY),
           "VERY_LIKELY" => Ok(SafeSearchAnnotationAdultEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SafeSearchAnnotationAdultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SafeSearchAnnotationMedicalEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Likelihood that this is a medical image.
pub enum SafeSearchAnnotationMedicalEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for SafeSearchAnnotationMedicalEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SafeSearchAnnotationMedicalEnum::UNKNOWN => "UNKNOWN",
            SafeSearchAnnotationMedicalEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            SafeSearchAnnotationMedicalEnum::UNLIKELY => "UNLIKELY",
            SafeSearchAnnotationMedicalEnum::POSSIBLE => "POSSIBLE",
            SafeSearchAnnotationMedicalEnum::LIKELY => "LIKELY",
            SafeSearchAnnotationMedicalEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for SafeSearchAnnotationMedicalEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(SafeSearchAnnotationMedicalEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(SafeSearchAnnotationMedicalEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(SafeSearchAnnotationMedicalEnum::UNLIKELY),
           "POSSIBLE" => Ok(SafeSearchAnnotationMedicalEnum::POSSIBLE),
           "LIKELY" => Ok(SafeSearchAnnotationMedicalEnum::LIKELY),
           "VERY_LIKELY" => Ok(SafeSearchAnnotationMedicalEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SafeSearchAnnotationMedicalEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SafeSearchAnnotationRacyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas.
pub enum SafeSearchAnnotationRacyEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for SafeSearchAnnotationRacyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SafeSearchAnnotationRacyEnum::UNKNOWN => "UNKNOWN",
            SafeSearchAnnotationRacyEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            SafeSearchAnnotationRacyEnum::UNLIKELY => "UNLIKELY",
            SafeSearchAnnotationRacyEnum::POSSIBLE => "POSSIBLE",
            SafeSearchAnnotationRacyEnum::LIKELY => "LIKELY",
            SafeSearchAnnotationRacyEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for SafeSearchAnnotationRacyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(SafeSearchAnnotationRacyEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(SafeSearchAnnotationRacyEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(SafeSearchAnnotationRacyEnum::UNLIKELY),
           "POSSIBLE" => Ok(SafeSearchAnnotationRacyEnum::POSSIBLE),
           "LIKELY" => Ok(SafeSearchAnnotationRacyEnum::LIKELY),
           "VERY_LIKELY" => Ok(SafeSearchAnnotationRacyEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SafeSearchAnnotationRacyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SafeSearchAnnotationSpoofEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive.
pub enum SafeSearchAnnotationSpoofEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for SafeSearchAnnotationSpoofEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SafeSearchAnnotationSpoofEnum::UNKNOWN => "UNKNOWN",
            SafeSearchAnnotationSpoofEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            SafeSearchAnnotationSpoofEnum::UNLIKELY => "UNLIKELY",
            SafeSearchAnnotationSpoofEnum::POSSIBLE => "POSSIBLE",
            SafeSearchAnnotationSpoofEnum::LIKELY => "LIKELY",
            SafeSearchAnnotationSpoofEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for SafeSearchAnnotationSpoofEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(SafeSearchAnnotationSpoofEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(SafeSearchAnnotationSpoofEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(SafeSearchAnnotationSpoofEnum::UNLIKELY),
           "POSSIBLE" => Ok(SafeSearchAnnotationSpoofEnum::POSSIBLE),
           "LIKELY" => Ok(SafeSearchAnnotationSpoofEnum::LIKELY),
           "VERY_LIKELY" => Ok(SafeSearchAnnotationSpoofEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SafeSearchAnnotationSpoofEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SafeSearchAnnotationViolenceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Likelihood that this image contains violent content. Violent content may include death, serious harm, or injury to individuals or groups of individuals.
pub enum SafeSearchAnnotationViolenceEnum {
    

    /// Unknown likelihood.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// It is very unlikely.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// It is unlikely.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// It is possible.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// It is likely.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// It is very likely.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for SafeSearchAnnotationViolenceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SafeSearchAnnotationViolenceEnum::UNKNOWN => "UNKNOWN",
            SafeSearchAnnotationViolenceEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            SafeSearchAnnotationViolenceEnum::UNLIKELY => "UNLIKELY",
            SafeSearchAnnotationViolenceEnum::POSSIBLE => "POSSIBLE",
            SafeSearchAnnotationViolenceEnum::LIKELY => "LIKELY",
            SafeSearchAnnotationViolenceEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for SafeSearchAnnotationViolenceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(SafeSearchAnnotationViolenceEnum::UNKNOWN),
           "VERY_UNLIKELY" => Ok(SafeSearchAnnotationViolenceEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(SafeSearchAnnotationViolenceEnum::UNLIKELY),
           "POSSIBLE" => Ok(SafeSearchAnnotationViolenceEnum::POSSIBLE),
           "LIKELY" => Ok(SafeSearchAnnotationViolenceEnum::LIKELY),
           "VERY_LIKELY" => Ok(SafeSearchAnnotationViolenceEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SafeSearchAnnotationViolenceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


