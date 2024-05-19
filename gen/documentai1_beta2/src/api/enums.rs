use super::*;



// region GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of the layout element that is being referenced if any.
pub enum GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum {
    

    /// Layout Unspecified.
    ///
    /// "LAYOUT_TYPE_UNSPECIFIED"
    #[serde(rename="LAYOUT_TYPE_UNSPECIFIED")]
    LAYOUTTYPEUNSPECIFIED,
    

    /// References a Page.blocks element.
    ///
    /// "BLOCK"
    #[serde(rename="BLOCK")]
    BLOCK,
    

    /// References a Page.paragraphs element.
    ///
    /// "PARAGRAPH"
    #[serde(rename="PARAGRAPH")]
    PARAGRAPH,
    

    /// References a Page.lines element.
    ///
    /// "LINE"
    #[serde(rename="LINE")]
    LINE,
    

    /// References a Page.tokens element.
    ///
    /// "TOKEN"
    #[serde(rename="TOKEN")]
    TOKEN,
    

    /// References a Page.visual_elements element.
    ///
    /// "VISUAL_ELEMENT"
    #[serde(rename="VISUAL_ELEMENT")]
    VISUALELEMENT,
    

    /// Refrrences a Page.tables element.
    ///
    /// "TABLE"
    #[serde(rename="TABLE")]
    TABLE,
    

    /// References a Page.form_fields element.
    ///
    /// "FORM_FIELD"
    #[serde(rename="FORM_FIELD")]
    FORMFIELD,
}

impl AsRef<str> for GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::LAYOUTTYPEUNSPECIFIED => "LAYOUT_TYPE_UNSPECIFIED",
            GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::BLOCK => "BLOCK",
            GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::PARAGRAPH => "PARAGRAPH",
            GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::LINE => "LINE",
            GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::TOKEN => "TOKEN",
            GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::VISUALELEMENT => "VISUAL_ELEMENT",
            GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::TABLE => "TABLE",
            GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::FORMFIELD => "FORM_FIELD",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAYOUT_TYPE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::LAYOUTTYPEUNSPECIFIED),
           "BLOCK" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::BLOCK),
           "PARAGRAPH" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::PARAGRAPH),
           "LINE" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::LINE),
           "TOKEN" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::TOKEN),
           "VISUAL_ELEMENT" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::VISUALELEMENT),
           "TABLE" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::TABLE),
           "FORM_FIELD" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum::FORMFIELD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Detected orientation for the Layout.
pub enum GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum {
    

    /// Unspecified orientation.
    ///
    /// "ORIENTATION_UNSPECIFIED"
    #[serde(rename="ORIENTATION_UNSPECIFIED")]
    ORIENTATIONUNSPECIFIED,
    

    /// Orientation is aligned with page up.
    ///
    /// "PAGE_UP"
    #[serde(rename="PAGE_UP")]
    PAGEUP,
    

    /// Orientation is aligned with page right. Turn the head 90 degrees clockwise from upright to read.
    ///
    /// "PAGE_RIGHT"
    #[serde(rename="PAGE_RIGHT")]
    PAGERIGHT,
    

    /// Orientation is aligned with page down. Turn the head 180 degrees from upright to read.
    ///
    /// "PAGE_DOWN"
    #[serde(rename="PAGE_DOWN")]
    PAGEDOWN,
    

    /// Orientation is aligned with page left. Turn the head 90 degrees counterclockwise from upright to read.
    ///
    /// "PAGE_LEFT"
    #[serde(rename="PAGE_LEFT")]
    PAGELEFT,
}

impl AsRef<str> for GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum::ORIENTATIONUNSPECIFIED => "ORIENTATION_UNSPECIFIED",
            GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum::PAGEUP => "PAGE_UP",
            GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum::PAGERIGHT => "PAGE_RIGHT",
            GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum::PAGEDOWN => "PAGE_DOWN",
            GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum::PAGELEFT => "PAGE_LEFT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORIENTATION_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum::ORIENTATIONUNSPECIFIED),
           "PAGE_UP" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum::PAGEUP),
           "PAGE_RIGHT" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum::PAGERIGHT),
           "PAGE_DOWN" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum::PAGEDOWN),
           "PAGE_LEFT" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum::PAGELEFT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Detected break type.
pub enum GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum {
    

    /// Unspecified break type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// A single whitespace.
    ///
    /// "SPACE"
    #[serde(rename="SPACE")]
    SPACE,
    

    /// A wider whitespace.
    ///
    /// "WIDE_SPACE"
    #[serde(rename="WIDE_SPACE")]
    WIDESPACE,
    

    /// A hyphen that indicates that a token has been split across lines.
    ///
    /// "HYPHEN"
    #[serde(rename="HYPHEN")]
    HYPHEN,
}

impl AsRef<str> for GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum::SPACE => "SPACE",
            GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum::WIDESPACE => "WIDE_SPACE",
            GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum::HYPHEN => "HYPHEN",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum::TYPEUNSPECIFIED),
           "SPACE" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum::SPACE),
           "WIDE_SPACE" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum::WIDESPACE),
           "HYPHEN" => Ok(GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum::HYPHEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of provenance operation.
pub enum GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum {
    

    /// Operation type unspecified. If no operation is specified a provenance entry is simply used to match against a `parent`.
    ///
    /// "OPERATION_TYPE_UNSPECIFIED"
    #[serde(rename="OPERATION_TYPE_UNSPECIFIED")]
    OPERATIONTYPEUNSPECIFIED,
    

    /// Add an element.
    ///
    /// "ADD"
    #[serde(rename="ADD")]
    ADD,
    

    /// Remove an element identified by `parent`.
    ///
    /// "REMOVE"
    #[serde(rename="REMOVE")]
    REMOVE,
    

    /// Updates any fields within the given provenance scope of the message. It overwrites the fields rather than replacing them. Use this when you want to update a field value of an entity without also updating all the child properties.
    ///
    /// "UPDATE"
    #[serde(rename="UPDATE")]
    UPDATE,
    

    /// Currently unused. Replace an element identified by `parent`.
    ///
    /// "REPLACE"
    #[serde(rename="REPLACE")]
    REPLACE,
    

    /// Deprecated. Request human review for the element identified by `parent`.
    ///
    /// "EVAL_REQUESTED"
    #[serde(rename="EVAL_REQUESTED")]
    EVALREQUESTED,
    

    /// Deprecated. Element is reviewed and approved at human review, confidence will be set to 1.0.
    ///
    /// "EVAL_APPROVED"
    #[serde(rename="EVAL_APPROVED")]
    EVALAPPROVED,
    

    /// Deprecated. Element is skipped in the validation process.
    ///
    /// "EVAL_SKIPPED"
    #[serde(rename="EVAL_SKIPPED")]
    EVALSKIPPED,
}

impl AsRef<str> for GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::OPERATIONTYPEUNSPECIFIED => "OPERATION_TYPE_UNSPECIFIED",
            GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::ADD => "ADD",
            GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::REMOVE => "REMOVE",
            GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::UPDATE => "UPDATE",
            GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::REPLACE => "REPLACE",
            GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::EVALREQUESTED => "EVAL_REQUESTED",
            GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::EVALAPPROVED => "EVAL_APPROVED",
            GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::EVALSKIPPED => "EVAL_SKIPPED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::OPERATIONTYPEUNSPECIFIED),
           "ADD" => Ok(GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::ADD),
           "REMOVE" => Ok(GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::REMOVE),
           "UPDATE" => Ok(GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::UPDATE),
           "REPLACE" => Ok(GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::REPLACE),
           "EVAL_REQUESTED" => Ok(GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::EVALREQUESTED),
           "EVAL_APPROVED" => Ok(GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::EVALAPPROVED),
           "EVAL_SKIPPED" => Ok(GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum::EVALSKIPPED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


