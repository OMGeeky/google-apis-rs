use super::*;



// region AutoTextTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this auto text.
pub enum AutoTextTypeEnum {
    

    /// An unspecified auto text type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Type for auto text that represents the current page number.
    ///
    /// "PAGE_NUMBER"
    #[serde(rename="PAGE_NUMBER")]
    PAGENUMBER,
    

    /// Type for auto text that represents the total number of pages in the document.
    ///
    /// "PAGE_COUNT"
    #[serde(rename="PAGE_COUNT")]
    PAGECOUNT,
}

impl AsRef<str> for AutoTextTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AutoTextTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            AutoTextTypeEnum::PAGENUMBER => "PAGE_NUMBER",
            AutoTextTypeEnum::PAGECOUNT => "PAGE_COUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for AutoTextTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(AutoTextTypeEnum::TYPEUNSPECIFIED),
           "PAGE_NUMBER" => Ok(AutoTextTypeEnum::PAGENUMBER),
           "PAGE_COUNT" => Ok(AutoTextTypeEnum::PAGECOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AutoTextTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreateFooterRequestTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of footer to create.
pub enum CreateFooterRequestTypeEnum {
    

    /// The header/footer type is unspecified.
    ///
    /// "HEADER_FOOTER_TYPE_UNSPECIFIED"
    #[serde(rename="HEADER_FOOTER_TYPE_UNSPECIFIED")]
    HEADERFOOTERTYPEUNSPECIFIED,
    

    /// A default header/footer.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
}

impl AsRef<str> for CreateFooterRequestTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateFooterRequestTypeEnum::HEADERFOOTERTYPEUNSPECIFIED => "HEADER_FOOTER_TYPE_UNSPECIFIED",
            CreateFooterRequestTypeEnum::DEFAULT => "DEFAULT",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateFooterRequestTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEADER_FOOTER_TYPE_UNSPECIFIED" => Ok(CreateFooterRequestTypeEnum::HEADERFOOTERTYPEUNSPECIFIED),
           "DEFAULT" => Ok(CreateFooterRequestTypeEnum::DEFAULT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateFooterRequestTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreateHeaderRequestTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of header to create.
pub enum CreateHeaderRequestTypeEnum {
    

    /// The header/footer type is unspecified.
    ///
    /// "HEADER_FOOTER_TYPE_UNSPECIFIED"
    #[serde(rename="HEADER_FOOTER_TYPE_UNSPECIFIED")]
    HEADERFOOTERTYPEUNSPECIFIED,
    

    /// A default header/footer.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
}

impl AsRef<str> for CreateHeaderRequestTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateHeaderRequestTypeEnum::HEADERFOOTERTYPEUNSPECIFIED => "HEADER_FOOTER_TYPE_UNSPECIFIED",
            CreateHeaderRequestTypeEnum::DEFAULT => "DEFAULT",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateHeaderRequestTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEADER_FOOTER_TYPE_UNSPECIFIED" => Ok(CreateHeaderRequestTypeEnum::HEADERFOOTERTYPEUNSPECIFIED),
           "DEFAULT" => Ok(CreateHeaderRequestTypeEnum::DEFAULT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateHeaderRequestTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreateParagraphBulletsRequestBulletPresetEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The kinds of bullet glyphs to be used.
pub enum CreateParagraphBulletsRequestBulletPresetEnum {
    

    /// The bullet glyph preset is unspecified.
    ///
    /// "BULLET_GLYPH_PRESET_UNSPECIFIED"
    #[serde(rename="BULLET_GLYPH_PRESET_UNSPECIFIED")]
    BULLETGLYPHPRESETUNSPECIFIED,
    

    /// A bulleted list with a `DISC`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels.
    ///
    /// "BULLET_DISC_CIRCLE_SQUARE"
    #[serde(rename="BULLET_DISC_CIRCLE_SQUARE")]
    BULLETDISCCIRCLESQUARE,
    

    /// A bulleted list with a `DIAMONDX`, `ARROW3D` and `SQUARE` bullet glyph for the first 3 list nesting levels.
    ///
    /// "BULLET_DIAMONDX_ARROW3D_SQUARE"
    #[serde(rename="BULLET_DIAMONDX_ARROW3D_SQUARE")]
    BULLETDIAMONDXARROW3DSQUARE,
    

    /// A bulleted list with `CHECKBOX` bullet glyphs for all list nesting levels.
    ///
    /// "BULLET_CHECKBOX"
    #[serde(rename="BULLET_CHECKBOX")]
    BULLETCHECKBOX,
    

    /// A bulleted list with a `ARROW`, `DIAMOND` and `DISC` bullet glyph for the first 3 list nesting levels.
    ///
    /// "BULLET_ARROW_DIAMOND_DISC"
    #[serde(rename="BULLET_ARROW_DIAMOND_DISC")]
    BULLETARROWDIAMONDDISC,
    

    /// A bulleted list with a `STAR`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels.
    ///
    /// "BULLET_STAR_CIRCLE_SQUARE"
    #[serde(rename="BULLET_STAR_CIRCLE_SQUARE")]
    BULLETSTARCIRCLESQUARE,
    

    /// A bulleted list with a `ARROW3D`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels.
    ///
    /// "BULLET_ARROW3D_CIRCLE_SQUARE"
    #[serde(rename="BULLET_ARROW3D_CIRCLE_SQUARE")]
    BULLETARROW3DCIRCLESQUARE,
    

    /// A bulleted list with a `LEFTTRIANGLE`, `DIAMOND` and `DISC` bullet glyph for the first 3 list nesting levels.
    ///
    /// "BULLET_LEFTTRIANGLE_DIAMOND_DISC"
    #[serde(rename="BULLET_LEFTTRIANGLE_DIAMOND_DISC")]
    BULLETLEFTTRIANGLEDIAMONDDISC,
    

    /// A bulleted list with a `DIAMONDX`, `HOLLOWDIAMOND` and `SQUARE` bullet glyph for the first 3 list nesting levels.
    ///
    /// "BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE"
    #[serde(rename="BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE")]
    BULLETDIAMONDXHOLLOWDIAMONDSQUARE,
    

    /// A bulleted list with a `DIAMOND`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels.
    ///
    /// "BULLET_DIAMOND_CIRCLE_SQUARE"
    #[serde(rename="BULLET_DIAMOND_CIRCLE_SQUARE")]
    BULLETDIAMONDCIRCLESQUARE,
    

    /// A numbered list with `DECIMAL`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods.
    ///
    /// "NUMBERED_DECIMAL_ALPHA_ROMAN"
    #[serde(rename="NUMBERED_DECIMAL_ALPHA_ROMAN")]
    NUMBEREDDECIMALALPHAROMAN,
    

    /// A numbered list with `DECIMAL`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by parenthesis.
    ///
    /// "NUMBERED_DECIMAL_ALPHA_ROMAN_PARENS"
    #[serde(rename="NUMBERED_DECIMAL_ALPHA_ROMAN_PARENS")]
    NUMBEREDDECIMALALPHAROMANPARENS,
    

    /// A numbered list with `DECIMAL` numeric glyphs separated by periods, where each nesting level uses the previous nesting level's glyph as a prefix. For example: '1.', '1.1.', '2.', '2.2.'.
    ///
    /// "NUMBERED_DECIMAL_NESTED"
    #[serde(rename="NUMBERED_DECIMAL_NESTED")]
    NUMBEREDDECIMALNESTED,
    

    /// A numbered list with `UPPERALPHA`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods.
    ///
    /// "NUMBERED_UPPERALPHA_ALPHA_ROMAN"
    #[serde(rename="NUMBERED_UPPERALPHA_ALPHA_ROMAN")]
    NUMBEREDUPPERALPHAALPHAROMAN,
    

    /// A numbered list with `UPPERROMAN`, `UPPERALPHA` and `DECIMAL` numeric glyphs for the first 3 list nesting levels, followed by periods.
    ///
    /// "NUMBERED_UPPERROMAN_UPPERALPHA_DECIMAL"
    #[serde(rename="NUMBERED_UPPERROMAN_UPPERALPHA_DECIMAL")]
    NUMBEREDUPPERROMANUPPERALPHADECIMAL,
    

    /// A numbered list with `ZERODECIMAL`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods.
    ///
    /// "NUMBERED_ZERODECIMAL_ALPHA_ROMAN"
    #[serde(rename="NUMBERED_ZERODECIMAL_ALPHA_ROMAN")]
    NUMBEREDZERODECIMALALPHAROMAN,
}

impl AsRef<str> for CreateParagraphBulletsRequestBulletPresetEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateParagraphBulletsRequestBulletPresetEnum::BULLETGLYPHPRESETUNSPECIFIED => "BULLET_GLYPH_PRESET_UNSPECIFIED",
            CreateParagraphBulletsRequestBulletPresetEnum::BULLETDISCCIRCLESQUARE => "BULLET_DISC_CIRCLE_SQUARE",
            CreateParagraphBulletsRequestBulletPresetEnum::BULLETDIAMONDXARROW3DSQUARE => "BULLET_DIAMONDX_ARROW3D_SQUARE",
            CreateParagraphBulletsRequestBulletPresetEnum::BULLETCHECKBOX => "BULLET_CHECKBOX",
            CreateParagraphBulletsRequestBulletPresetEnum::BULLETARROWDIAMONDDISC => "BULLET_ARROW_DIAMOND_DISC",
            CreateParagraphBulletsRequestBulletPresetEnum::BULLETSTARCIRCLESQUARE => "BULLET_STAR_CIRCLE_SQUARE",
            CreateParagraphBulletsRequestBulletPresetEnum::BULLETARROW3DCIRCLESQUARE => "BULLET_ARROW3D_CIRCLE_SQUARE",
            CreateParagraphBulletsRequestBulletPresetEnum::BULLETLEFTTRIANGLEDIAMONDDISC => "BULLET_LEFTTRIANGLE_DIAMOND_DISC",
            CreateParagraphBulletsRequestBulletPresetEnum::BULLETDIAMONDXHOLLOWDIAMONDSQUARE => "BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE",
            CreateParagraphBulletsRequestBulletPresetEnum::BULLETDIAMONDCIRCLESQUARE => "BULLET_DIAMOND_CIRCLE_SQUARE",
            CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDDECIMALALPHAROMAN => "NUMBERED_DECIMAL_ALPHA_ROMAN",
            CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDDECIMALALPHAROMANPARENS => "NUMBERED_DECIMAL_ALPHA_ROMAN_PARENS",
            CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDDECIMALNESTED => "NUMBERED_DECIMAL_NESTED",
            CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDUPPERALPHAALPHAROMAN => "NUMBERED_UPPERALPHA_ALPHA_ROMAN",
            CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDUPPERROMANUPPERALPHADECIMAL => "NUMBERED_UPPERROMAN_UPPERALPHA_DECIMAL",
            CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDZERODECIMALALPHAROMAN => "NUMBERED_ZERODECIMAL_ALPHA_ROMAN",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateParagraphBulletsRequestBulletPresetEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BULLET_GLYPH_PRESET_UNSPECIFIED" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::BULLETGLYPHPRESETUNSPECIFIED),
           "BULLET_DISC_CIRCLE_SQUARE" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::BULLETDISCCIRCLESQUARE),
           "BULLET_DIAMONDX_ARROW3D_SQUARE" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::BULLETDIAMONDXARROW3DSQUARE),
           "BULLET_CHECKBOX" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::BULLETCHECKBOX),
           "BULLET_ARROW_DIAMOND_DISC" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::BULLETARROWDIAMONDDISC),
           "BULLET_STAR_CIRCLE_SQUARE" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::BULLETSTARCIRCLESQUARE),
           "BULLET_ARROW3D_CIRCLE_SQUARE" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::BULLETARROW3DCIRCLESQUARE),
           "BULLET_LEFTTRIANGLE_DIAMOND_DISC" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::BULLETLEFTTRIANGLEDIAMONDDISC),
           "BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::BULLETDIAMONDXHOLLOWDIAMONDSQUARE),
           "BULLET_DIAMOND_CIRCLE_SQUARE" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::BULLETDIAMONDCIRCLESQUARE),
           "NUMBERED_DECIMAL_ALPHA_ROMAN" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDDECIMALALPHAROMAN),
           "NUMBERED_DECIMAL_ALPHA_ROMAN_PARENS" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDDECIMALALPHAROMANPARENS),
           "NUMBERED_DECIMAL_NESTED" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDDECIMALNESTED),
           "NUMBERED_UPPERALPHA_ALPHA_ROMAN" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDUPPERALPHAALPHAROMAN),
           "NUMBERED_UPPERROMAN_UPPERALPHA_DECIMAL" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDUPPERROMANUPPERALPHADECIMAL),
           "NUMBERED_ZERODECIMAL_ALPHA_ROMAN" => Ok(CreateParagraphBulletsRequestBulletPresetEnum::NUMBEREDZERODECIMALALPHAROMAN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateParagraphBulletsRequestBulletPresetEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DimensionUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The units for magnitude.
pub enum DimensionUnitEnum {
    

    /// The units are unknown.
    ///
    /// "UNIT_UNSPECIFIED"
    #[serde(rename="UNIT_UNSPECIFIED")]
    UNITUNSPECIFIED,
    

    /// A point, 1/72 of an inch.
    ///
    /// "PT"
    #[serde(rename="PT")]
    PT,
}

impl AsRef<str> for DimensionUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DimensionUnitEnum::UNITUNSPECIFIED => "UNIT_UNSPECIFIED",
            DimensionUnitEnum::PT => "PT",
        }
    }
}

impl std::convert::TryFrom< &str> for DimensionUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNIT_UNSPECIFIED" => Ok(DimensionUnitEnum::UNITUNSPECIFIED),
           "PT" => Ok(DimensionUnitEnum::PT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DimensionUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DocumentSuggestionsViewModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The suggestions view mode applied to the document. Note: When editing a document, changes must be based on a document with SUGGESTIONS_INLINE.
pub enum DocumentSuggestionsViewModeEnum {
    

    /// The SuggestionsViewMode applied to the returned document depends on the user's current access level. If the user only has view access, PREVIEW_WITHOUT_SUGGESTIONS is applied. Otherwise, SUGGESTIONS_INLINE is applied. This is the default suggestions view mode.
    ///
    /// "DEFAULT_FOR_CURRENT_ACCESS"
    #[serde(rename="DEFAULT_FOR_CURRENT_ACCESS")]
    DEFAULTFORCURRENTACCESS,
    

    /// The returned document has suggestions inline. Suggested changes will be differentiated from base content within the document. Requests to retrieve a document using this mode will return a 403 error if the user does not have permission to view suggested changes.
    ///
    /// "SUGGESTIONS_INLINE"
    #[serde(rename="SUGGESTIONS_INLINE")]
    SUGGESTIONSINLINE,
    

    /// The returned document is a preview with all suggested changes accepted. Requests to retrieve a document using this mode will return a 403 error if the user does not have permission to view suggested changes.
    ///
    /// "PREVIEW_SUGGESTIONS_ACCEPTED"
    #[serde(rename="PREVIEW_SUGGESTIONS_ACCEPTED")]
    PREVIEWSUGGESTIONSACCEPTED,
    

    /// The returned document is a preview with all suggested changes rejected if there are any suggestions in the document.
    ///
    /// "PREVIEW_WITHOUT_SUGGESTIONS"
    #[serde(rename="PREVIEW_WITHOUT_SUGGESTIONS")]
    PREVIEWWITHOUTSUGGESTIONS,
}

impl AsRef<str> for DocumentSuggestionsViewModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DocumentSuggestionsViewModeEnum::DEFAULTFORCURRENTACCESS => "DEFAULT_FOR_CURRENT_ACCESS",
            DocumentSuggestionsViewModeEnum::SUGGESTIONSINLINE => "SUGGESTIONS_INLINE",
            DocumentSuggestionsViewModeEnum::PREVIEWSUGGESTIONSACCEPTED => "PREVIEW_SUGGESTIONS_ACCEPTED",
            DocumentSuggestionsViewModeEnum::PREVIEWWITHOUTSUGGESTIONS => "PREVIEW_WITHOUT_SUGGESTIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for DocumentSuggestionsViewModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT_FOR_CURRENT_ACCESS" => Ok(DocumentSuggestionsViewModeEnum::DEFAULTFORCURRENTACCESS),
           "SUGGESTIONS_INLINE" => Ok(DocumentSuggestionsViewModeEnum::SUGGESTIONSINLINE),
           "PREVIEW_SUGGESTIONS_ACCEPTED" => Ok(DocumentSuggestionsViewModeEnum::PREVIEWSUGGESTIONSACCEPTED),
           "PREVIEW_WITHOUT_SUGGESTIONS" => Ok(DocumentSuggestionsViewModeEnum::PREVIEWWITHOUTSUGGESTIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DocumentSuggestionsViewModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EmbeddedObjectBorderDashStyleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The dash style of the border.
pub enum EmbeddedObjectBorderDashStyleEnum {
    

    /// Unspecified dash style.
    ///
    /// "DASH_STYLE_UNSPECIFIED"
    #[serde(rename="DASH_STYLE_UNSPECIFIED")]
    DASHSTYLEUNSPECIFIED,
    

    /// Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style.
    ///
    /// "SOLID"
    #[serde(rename="SOLID")]
    SOLID,
    

    /// Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'.
    ///
    /// "DOT"
    #[serde(rename="DOT")]
    DOT,
    

    /// Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'.
    ///
    /// "DASH"
    #[serde(rename="DASH")]
    DASH,
}

impl AsRef<str> for EmbeddedObjectBorderDashStyleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EmbeddedObjectBorderDashStyleEnum::DASHSTYLEUNSPECIFIED => "DASH_STYLE_UNSPECIFIED",
            EmbeddedObjectBorderDashStyleEnum::SOLID => "SOLID",
            EmbeddedObjectBorderDashStyleEnum::DOT => "DOT",
            EmbeddedObjectBorderDashStyleEnum::DASH => "DASH",
        }
    }
}

impl std::convert::TryFrom< &str> for EmbeddedObjectBorderDashStyleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DASH_STYLE_UNSPECIFIED" => Ok(EmbeddedObjectBorderDashStyleEnum::DASHSTYLEUNSPECIFIED),
           "SOLID" => Ok(EmbeddedObjectBorderDashStyleEnum::SOLID),
           "DOT" => Ok(EmbeddedObjectBorderDashStyleEnum::DOT),
           "DASH" => Ok(EmbeddedObjectBorderDashStyleEnum::DASH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EmbeddedObjectBorderDashStyleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EmbeddedObjectBorderPropertyStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The property state of the border property.
pub enum EmbeddedObjectBorderPropertyStateEnum {
    

    /// If a property's state is RENDERED, then the element has the corresponding property when rendered in the document. This is the default value.
    ///
    /// "RENDERED"
    #[serde(rename="RENDERED")]
    RENDERED,
    

    /// If a property's state is NOT_RENDERED, then the element does not have the corresponding property when rendered in the document.
    ///
    /// "NOT_RENDERED"
    #[serde(rename="NOT_RENDERED")]
    NOTRENDERED,
}

impl AsRef<str> for EmbeddedObjectBorderPropertyStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EmbeddedObjectBorderPropertyStateEnum::RENDERED => "RENDERED",
            EmbeddedObjectBorderPropertyStateEnum::NOTRENDERED => "NOT_RENDERED",
        }
    }
}

impl std::convert::TryFrom< &str> for EmbeddedObjectBorderPropertyStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RENDERED" => Ok(EmbeddedObjectBorderPropertyStateEnum::RENDERED),
           "NOT_RENDERED" => Ok(EmbeddedObjectBorderPropertyStateEnum::NOTRENDERED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EmbeddedObjectBorderPropertyStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InsertSectionBreakRequestSectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of section to insert.
pub enum InsertSectionBreakRequestSectionTypeEnum {
    

    /// The section type is unspecified.
    ///
    /// "SECTION_TYPE_UNSPECIFIED"
    #[serde(rename="SECTION_TYPE_UNSPECIFIED")]
    SECTIONTYPEUNSPECIFIED,
    

    /// The section starts immediately after the last paragraph of the previous section.
    ///
    /// "CONTINUOUS"
    #[serde(rename="CONTINUOUS")]
    CONTINUOUS,
    

    /// The section starts on the next page.
    ///
    /// "NEXT_PAGE"
    #[serde(rename="NEXT_PAGE")]
    NEXTPAGE,
}

impl AsRef<str> for InsertSectionBreakRequestSectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InsertSectionBreakRequestSectionTypeEnum::SECTIONTYPEUNSPECIFIED => "SECTION_TYPE_UNSPECIFIED",
            InsertSectionBreakRequestSectionTypeEnum::CONTINUOUS => "CONTINUOUS",
            InsertSectionBreakRequestSectionTypeEnum::NEXTPAGE => "NEXT_PAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for InsertSectionBreakRequestSectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_TYPE_UNSPECIFIED" => Ok(InsertSectionBreakRequestSectionTypeEnum::SECTIONTYPEUNSPECIFIED),
           "CONTINUOUS" => Ok(InsertSectionBreakRequestSectionTypeEnum::CONTINUOUS),
           "NEXT_PAGE" => Ok(InsertSectionBreakRequestSectionTypeEnum::NEXTPAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InsertSectionBreakRequestSectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NamedStyleNamedStyleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this named style.
pub enum NamedStyleNamedStyleTypeEnum {
    

    /// The type of named style is unspecified.
    ///
    /// "NAMED_STYLE_TYPE_UNSPECIFIED"
    #[serde(rename="NAMED_STYLE_TYPE_UNSPECIFIED")]
    NAMEDSTYLETYPEUNSPECIFIED,
    

    /// Normal text.
    ///
    /// "NORMAL_TEXT"
    #[serde(rename="NORMAL_TEXT")]
    NORMALTEXT,
    

    /// Title.
    ///
    /// "TITLE"
    #[serde(rename="TITLE")]
    TITLE,
    

    /// Subtitle.
    ///
    /// "SUBTITLE"
    #[serde(rename="SUBTITLE")]
    SUBTITLE,
    

    /// Heading 1.
    ///
    /// "HEADING_1"
    #[serde(rename="HEADING_1")]
    HEADING1,
    

    /// Heading 2.
    ///
    /// "HEADING_2"
    #[serde(rename="HEADING_2")]
    HEADING2,
    

    /// Heading 3.
    ///
    /// "HEADING_3"
    #[serde(rename="HEADING_3")]
    HEADING3,
    

    /// Heading 4.
    ///
    /// "HEADING_4"
    #[serde(rename="HEADING_4")]
    HEADING4,
    

    /// Heading 5.
    ///
    /// "HEADING_5"
    #[serde(rename="HEADING_5")]
    HEADING5,
    

    /// Heading 6.
    ///
    /// "HEADING_6"
    #[serde(rename="HEADING_6")]
    HEADING6,
}

impl AsRef<str> for NamedStyleNamedStyleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NamedStyleNamedStyleTypeEnum::NAMEDSTYLETYPEUNSPECIFIED => "NAMED_STYLE_TYPE_UNSPECIFIED",
            NamedStyleNamedStyleTypeEnum::NORMALTEXT => "NORMAL_TEXT",
            NamedStyleNamedStyleTypeEnum::TITLE => "TITLE",
            NamedStyleNamedStyleTypeEnum::SUBTITLE => "SUBTITLE",
            NamedStyleNamedStyleTypeEnum::HEADING1 => "HEADING_1",
            NamedStyleNamedStyleTypeEnum::HEADING2 => "HEADING_2",
            NamedStyleNamedStyleTypeEnum::HEADING3 => "HEADING_3",
            NamedStyleNamedStyleTypeEnum::HEADING4 => "HEADING_4",
            NamedStyleNamedStyleTypeEnum::HEADING5 => "HEADING_5",
            NamedStyleNamedStyleTypeEnum::HEADING6 => "HEADING_6",
        }
    }
}

impl std::convert::TryFrom< &str> for NamedStyleNamedStyleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NAMED_STYLE_TYPE_UNSPECIFIED" => Ok(NamedStyleNamedStyleTypeEnum::NAMEDSTYLETYPEUNSPECIFIED),
           "NORMAL_TEXT" => Ok(NamedStyleNamedStyleTypeEnum::NORMALTEXT),
           "TITLE" => Ok(NamedStyleNamedStyleTypeEnum::TITLE),
           "SUBTITLE" => Ok(NamedStyleNamedStyleTypeEnum::SUBTITLE),
           "HEADING_1" => Ok(NamedStyleNamedStyleTypeEnum::HEADING1),
           "HEADING_2" => Ok(NamedStyleNamedStyleTypeEnum::HEADING2),
           "HEADING_3" => Ok(NamedStyleNamedStyleTypeEnum::HEADING3),
           "HEADING_4" => Ok(NamedStyleNamedStyleTypeEnum::HEADING4),
           "HEADING_5" => Ok(NamedStyleNamedStyleTypeEnum::HEADING5),
           "HEADING_6" => Ok(NamedStyleNamedStyleTypeEnum::HEADING6),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NamedStyleNamedStyleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NamedStyleSuggestionStateNamedStyleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The named style type that this suggestion state corresponds to. This field is provided as a convenience for matching the NamedStyleSuggestionState with its corresponding NamedStyle.
pub enum NamedStyleSuggestionStateNamedStyleTypeEnum {
    

    /// The type of named style is unspecified.
    ///
    /// "NAMED_STYLE_TYPE_UNSPECIFIED"
    #[serde(rename="NAMED_STYLE_TYPE_UNSPECIFIED")]
    NAMEDSTYLETYPEUNSPECIFIED,
    

    /// Normal text.
    ///
    /// "NORMAL_TEXT"
    #[serde(rename="NORMAL_TEXT")]
    NORMALTEXT,
    

    /// Title.
    ///
    /// "TITLE"
    #[serde(rename="TITLE")]
    TITLE,
    

    /// Subtitle.
    ///
    /// "SUBTITLE"
    #[serde(rename="SUBTITLE")]
    SUBTITLE,
    

    /// Heading 1.
    ///
    /// "HEADING_1"
    #[serde(rename="HEADING_1")]
    HEADING1,
    

    /// Heading 2.
    ///
    /// "HEADING_2"
    #[serde(rename="HEADING_2")]
    HEADING2,
    

    /// Heading 3.
    ///
    /// "HEADING_3"
    #[serde(rename="HEADING_3")]
    HEADING3,
    

    /// Heading 4.
    ///
    /// "HEADING_4"
    #[serde(rename="HEADING_4")]
    HEADING4,
    

    /// Heading 5.
    ///
    /// "HEADING_5"
    #[serde(rename="HEADING_5")]
    HEADING5,
    

    /// Heading 6.
    ///
    /// "HEADING_6"
    #[serde(rename="HEADING_6")]
    HEADING6,
}

impl AsRef<str> for NamedStyleSuggestionStateNamedStyleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NamedStyleSuggestionStateNamedStyleTypeEnum::NAMEDSTYLETYPEUNSPECIFIED => "NAMED_STYLE_TYPE_UNSPECIFIED",
            NamedStyleSuggestionStateNamedStyleTypeEnum::NORMALTEXT => "NORMAL_TEXT",
            NamedStyleSuggestionStateNamedStyleTypeEnum::TITLE => "TITLE",
            NamedStyleSuggestionStateNamedStyleTypeEnum::SUBTITLE => "SUBTITLE",
            NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING1 => "HEADING_1",
            NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING2 => "HEADING_2",
            NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING3 => "HEADING_3",
            NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING4 => "HEADING_4",
            NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING5 => "HEADING_5",
            NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING6 => "HEADING_6",
        }
    }
}

impl std::convert::TryFrom< &str> for NamedStyleSuggestionStateNamedStyleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NAMED_STYLE_TYPE_UNSPECIFIED" => Ok(NamedStyleSuggestionStateNamedStyleTypeEnum::NAMEDSTYLETYPEUNSPECIFIED),
           "NORMAL_TEXT" => Ok(NamedStyleSuggestionStateNamedStyleTypeEnum::NORMALTEXT),
           "TITLE" => Ok(NamedStyleSuggestionStateNamedStyleTypeEnum::TITLE),
           "SUBTITLE" => Ok(NamedStyleSuggestionStateNamedStyleTypeEnum::SUBTITLE),
           "HEADING_1" => Ok(NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING1),
           "HEADING_2" => Ok(NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING2),
           "HEADING_3" => Ok(NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING3),
           "HEADING_4" => Ok(NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING4),
           "HEADING_5" => Ok(NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING5),
           "HEADING_6" => Ok(NamedStyleSuggestionStateNamedStyleTypeEnum::HEADING6),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NamedStyleSuggestionStateNamedStyleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NestingLevelBulletAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The alignment of the bullet within the space allotted for rendering the bullet.
pub enum NestingLevelBulletAlignmentEnum {
    

    /// The bullet alignment is unspecified.
    ///
    /// "BULLET_ALIGNMENT_UNSPECIFIED"
    #[serde(rename="BULLET_ALIGNMENT_UNSPECIFIED")]
    BULLETALIGNMENTUNSPECIFIED,
    

    /// The bullet is aligned to the start of the space allotted for rendering the bullet. Left-aligned for LTR text, right-aligned otherwise.
    ///
    /// "START"
    #[serde(rename="START")]
    START,
    

    /// The bullet is aligned to the center of the space allotted for rendering the bullet.
    ///
    /// "CENTER"
    #[serde(rename="CENTER")]
    CENTER,
    

    /// The bullet is aligned to the end of the space allotted for rendering the bullet. Right-aligned for LTR text, left-aligned otherwise.
    ///
    /// "END"
    #[serde(rename="END")]
    END,
}

impl AsRef<str> for NestingLevelBulletAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NestingLevelBulletAlignmentEnum::BULLETALIGNMENTUNSPECIFIED => "BULLET_ALIGNMENT_UNSPECIFIED",
            NestingLevelBulletAlignmentEnum::START => "START",
            NestingLevelBulletAlignmentEnum::CENTER => "CENTER",
            NestingLevelBulletAlignmentEnum::END => "END",
        }
    }
}

impl std::convert::TryFrom< &str> for NestingLevelBulletAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BULLET_ALIGNMENT_UNSPECIFIED" => Ok(NestingLevelBulletAlignmentEnum::BULLETALIGNMENTUNSPECIFIED),
           "START" => Ok(NestingLevelBulletAlignmentEnum::START),
           "CENTER" => Ok(NestingLevelBulletAlignmentEnum::CENTER),
           "END" => Ok(NestingLevelBulletAlignmentEnum::END),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NestingLevelBulletAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NestingLevelGlyphTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of glyph used by bullets when paragraphs at this level of nesting are ordered. The glyph type determines the type of glyph used to replace placeholders within the glyph_format when paragraphs at this level of nesting are ordered. For example, if the nesting level is 0, the glyph_format is `%0.` and the glyph type is DECIMAL, then the rendered glyph would replace the placeholder `%0` in the glyph format with a number corresponding to list item's order within the list.
pub enum NestingLevelGlyphTypeEnum {
    

    /// The glyph type is unspecified or unsupported.
    ///
    /// "GLYPH_TYPE_UNSPECIFIED"
    #[serde(rename="GLYPH_TYPE_UNSPECIFIED")]
    GLYPHTYPEUNSPECIFIED,
    

    /// An empty string.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// A number, like `1`, `2`, or `3`.
    ///
    /// "DECIMAL"
    #[serde(rename="DECIMAL")]
    DECIMAL,
    

    /// A number where single digit numbers are prefixed with a zero, like `01`, `02`, or `03`. Numbers with more than one digit are not prefixed with a zero.
    ///
    /// "ZERO_DECIMAL"
    #[serde(rename="ZERO_DECIMAL")]
    ZERODECIMAL,
    

    /// An uppercase letter, like `A`, `B`, or `C`.
    ///
    /// "UPPER_ALPHA"
    #[serde(rename="UPPER_ALPHA")]
    UPPERALPHA,
    

    /// A lowercase letter, like `a`, `b`, or `c`.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// An uppercase Roman numeral, like `I`, `II`, or `III`.
    ///
    /// "UPPER_ROMAN"
    #[serde(rename="UPPER_ROMAN")]
    UPPERROMAN,
    

    /// A lowercase Roman numeral, like `i`, `ii`, or `iii`.
    ///
    /// "ROMAN"
    #[serde(rename="ROMAN")]
    ROMAN,
}

impl AsRef<str> for NestingLevelGlyphTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NestingLevelGlyphTypeEnum::GLYPHTYPEUNSPECIFIED => "GLYPH_TYPE_UNSPECIFIED",
            NestingLevelGlyphTypeEnum::NONE => "NONE",
            NestingLevelGlyphTypeEnum::DECIMAL => "DECIMAL",
            NestingLevelGlyphTypeEnum::ZERODECIMAL => "ZERO_DECIMAL",
            NestingLevelGlyphTypeEnum::UPPERALPHA => "UPPER_ALPHA",
            NestingLevelGlyphTypeEnum::ALPHA => "ALPHA",
            NestingLevelGlyphTypeEnum::UPPERROMAN => "UPPER_ROMAN",
            NestingLevelGlyphTypeEnum::ROMAN => "ROMAN",
        }
    }
}

impl std::convert::TryFrom< &str> for NestingLevelGlyphTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GLYPH_TYPE_UNSPECIFIED" => Ok(NestingLevelGlyphTypeEnum::GLYPHTYPEUNSPECIFIED),
           "NONE" => Ok(NestingLevelGlyphTypeEnum::NONE),
           "DECIMAL" => Ok(NestingLevelGlyphTypeEnum::DECIMAL),
           "ZERO_DECIMAL" => Ok(NestingLevelGlyphTypeEnum::ZERODECIMAL),
           "UPPER_ALPHA" => Ok(NestingLevelGlyphTypeEnum::UPPERALPHA),
           "ALPHA" => Ok(NestingLevelGlyphTypeEnum::ALPHA),
           "UPPER_ROMAN" => Ok(NestingLevelGlyphTypeEnum::UPPERROMAN),
           "ROMAN" => Ok(NestingLevelGlyphTypeEnum::ROMAN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NestingLevelGlyphTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParagraphBorderDashStyleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The dash style of the border.
pub enum ParagraphBorderDashStyleEnum {
    

    /// Unspecified dash style.
    ///
    /// "DASH_STYLE_UNSPECIFIED"
    #[serde(rename="DASH_STYLE_UNSPECIFIED")]
    DASHSTYLEUNSPECIFIED,
    

    /// Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style.
    ///
    /// "SOLID"
    #[serde(rename="SOLID")]
    SOLID,
    

    /// Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'.
    ///
    /// "DOT"
    #[serde(rename="DOT")]
    DOT,
    

    /// Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'.
    ///
    /// "DASH"
    #[serde(rename="DASH")]
    DASH,
}

impl AsRef<str> for ParagraphBorderDashStyleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParagraphBorderDashStyleEnum::DASHSTYLEUNSPECIFIED => "DASH_STYLE_UNSPECIFIED",
            ParagraphBorderDashStyleEnum::SOLID => "SOLID",
            ParagraphBorderDashStyleEnum::DOT => "DOT",
            ParagraphBorderDashStyleEnum::DASH => "DASH",
        }
    }
}

impl std::convert::TryFrom< &str> for ParagraphBorderDashStyleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DASH_STYLE_UNSPECIFIED" => Ok(ParagraphBorderDashStyleEnum::DASHSTYLEUNSPECIFIED),
           "SOLID" => Ok(ParagraphBorderDashStyleEnum::SOLID),
           "DOT" => Ok(ParagraphBorderDashStyleEnum::DOT),
           "DASH" => Ok(ParagraphBorderDashStyleEnum::DASH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParagraphBorderDashStyleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParagraphStyleAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The text alignment for this paragraph.
pub enum ParagraphStyleAlignmentEnum {
    

    /// The paragraph alignment is inherited from the parent.
    ///
    /// "ALIGNMENT_UNSPECIFIED"
    #[serde(rename="ALIGNMENT_UNSPECIFIED")]
    ALIGNMENTUNSPECIFIED,
    

    /// The paragraph is aligned to the start of the line. Left-aligned for LTR text, right-aligned otherwise.
    ///
    /// "START"
    #[serde(rename="START")]
    START,
    

    /// The paragraph is centered.
    ///
    /// "CENTER"
    #[serde(rename="CENTER")]
    CENTER,
    

    /// The paragraph is aligned to the end of the line. Right-aligned for LTR text, left-aligned otherwise.
    ///
    /// "END"
    #[serde(rename="END")]
    END,
    

    /// The paragraph is justified.
    ///
    /// "JUSTIFIED"
    #[serde(rename="JUSTIFIED")]
    JUSTIFIED,
}

impl AsRef<str> for ParagraphStyleAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParagraphStyleAlignmentEnum::ALIGNMENTUNSPECIFIED => "ALIGNMENT_UNSPECIFIED",
            ParagraphStyleAlignmentEnum::START => "START",
            ParagraphStyleAlignmentEnum::CENTER => "CENTER",
            ParagraphStyleAlignmentEnum::END => "END",
            ParagraphStyleAlignmentEnum::JUSTIFIED => "JUSTIFIED",
        }
    }
}

impl std::convert::TryFrom< &str> for ParagraphStyleAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALIGNMENT_UNSPECIFIED" => Ok(ParagraphStyleAlignmentEnum::ALIGNMENTUNSPECIFIED),
           "START" => Ok(ParagraphStyleAlignmentEnum::START),
           "CENTER" => Ok(ParagraphStyleAlignmentEnum::CENTER),
           "END" => Ok(ParagraphStyleAlignmentEnum::END),
           "JUSTIFIED" => Ok(ParagraphStyleAlignmentEnum::JUSTIFIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParagraphStyleAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParagraphStyleDirectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The text direction of this paragraph. If unset, the value defaults to LEFT_TO_RIGHT since paragraph direction is not inherited.
pub enum ParagraphStyleDirectionEnum {
    

    /// The content direction is unspecified.
    ///
    /// "CONTENT_DIRECTION_UNSPECIFIED"
    #[serde(rename="CONTENT_DIRECTION_UNSPECIFIED")]
    CONTENTDIRECTIONUNSPECIFIED,
    

    /// The content goes from left to right.
    ///
    /// "LEFT_TO_RIGHT"
    #[serde(rename="LEFT_TO_RIGHT")]
    LEFTTORIGHT,
    

    /// The content goes from right to left.
    ///
    /// "RIGHT_TO_LEFT"
    #[serde(rename="RIGHT_TO_LEFT")]
    RIGHTTOLEFT,
}

impl AsRef<str> for ParagraphStyleDirectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParagraphStyleDirectionEnum::CONTENTDIRECTIONUNSPECIFIED => "CONTENT_DIRECTION_UNSPECIFIED",
            ParagraphStyleDirectionEnum::LEFTTORIGHT => "LEFT_TO_RIGHT",
            ParagraphStyleDirectionEnum::RIGHTTOLEFT => "RIGHT_TO_LEFT",
        }
    }
}

impl std::convert::TryFrom< &str> for ParagraphStyleDirectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_DIRECTION_UNSPECIFIED" => Ok(ParagraphStyleDirectionEnum::CONTENTDIRECTIONUNSPECIFIED),
           "LEFT_TO_RIGHT" => Ok(ParagraphStyleDirectionEnum::LEFTTORIGHT),
           "RIGHT_TO_LEFT" => Ok(ParagraphStyleDirectionEnum::RIGHTTOLEFT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParagraphStyleDirectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParagraphStyleNamedStyleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The named style type of the paragraph. Since updating the named style type affects other properties within ParagraphStyle, the named style type is applied before the other properties are updated.
pub enum ParagraphStyleNamedStyleTypeEnum {
    

    /// The type of named style is unspecified.
    ///
    /// "NAMED_STYLE_TYPE_UNSPECIFIED"
    #[serde(rename="NAMED_STYLE_TYPE_UNSPECIFIED")]
    NAMEDSTYLETYPEUNSPECIFIED,
    

    /// Normal text.
    ///
    /// "NORMAL_TEXT"
    #[serde(rename="NORMAL_TEXT")]
    NORMALTEXT,
    

    /// Title.
    ///
    /// "TITLE"
    #[serde(rename="TITLE")]
    TITLE,
    

    /// Subtitle.
    ///
    /// "SUBTITLE"
    #[serde(rename="SUBTITLE")]
    SUBTITLE,
    

    /// Heading 1.
    ///
    /// "HEADING_1"
    #[serde(rename="HEADING_1")]
    HEADING1,
    

    /// Heading 2.
    ///
    /// "HEADING_2"
    #[serde(rename="HEADING_2")]
    HEADING2,
    

    /// Heading 3.
    ///
    /// "HEADING_3"
    #[serde(rename="HEADING_3")]
    HEADING3,
    

    /// Heading 4.
    ///
    /// "HEADING_4"
    #[serde(rename="HEADING_4")]
    HEADING4,
    

    /// Heading 5.
    ///
    /// "HEADING_5"
    #[serde(rename="HEADING_5")]
    HEADING5,
    

    /// Heading 6.
    ///
    /// "HEADING_6"
    #[serde(rename="HEADING_6")]
    HEADING6,
}

impl AsRef<str> for ParagraphStyleNamedStyleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParagraphStyleNamedStyleTypeEnum::NAMEDSTYLETYPEUNSPECIFIED => "NAMED_STYLE_TYPE_UNSPECIFIED",
            ParagraphStyleNamedStyleTypeEnum::NORMALTEXT => "NORMAL_TEXT",
            ParagraphStyleNamedStyleTypeEnum::TITLE => "TITLE",
            ParagraphStyleNamedStyleTypeEnum::SUBTITLE => "SUBTITLE",
            ParagraphStyleNamedStyleTypeEnum::HEADING1 => "HEADING_1",
            ParagraphStyleNamedStyleTypeEnum::HEADING2 => "HEADING_2",
            ParagraphStyleNamedStyleTypeEnum::HEADING3 => "HEADING_3",
            ParagraphStyleNamedStyleTypeEnum::HEADING4 => "HEADING_4",
            ParagraphStyleNamedStyleTypeEnum::HEADING5 => "HEADING_5",
            ParagraphStyleNamedStyleTypeEnum::HEADING6 => "HEADING_6",
        }
    }
}

impl std::convert::TryFrom< &str> for ParagraphStyleNamedStyleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NAMED_STYLE_TYPE_UNSPECIFIED" => Ok(ParagraphStyleNamedStyleTypeEnum::NAMEDSTYLETYPEUNSPECIFIED),
           "NORMAL_TEXT" => Ok(ParagraphStyleNamedStyleTypeEnum::NORMALTEXT),
           "TITLE" => Ok(ParagraphStyleNamedStyleTypeEnum::TITLE),
           "SUBTITLE" => Ok(ParagraphStyleNamedStyleTypeEnum::SUBTITLE),
           "HEADING_1" => Ok(ParagraphStyleNamedStyleTypeEnum::HEADING1),
           "HEADING_2" => Ok(ParagraphStyleNamedStyleTypeEnum::HEADING2),
           "HEADING_3" => Ok(ParagraphStyleNamedStyleTypeEnum::HEADING3),
           "HEADING_4" => Ok(ParagraphStyleNamedStyleTypeEnum::HEADING4),
           "HEADING_5" => Ok(ParagraphStyleNamedStyleTypeEnum::HEADING5),
           "HEADING_6" => Ok(ParagraphStyleNamedStyleTypeEnum::HEADING6),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParagraphStyleNamedStyleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParagraphStyleSpacingModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The spacing mode for the paragraph.
pub enum ParagraphStyleSpacingModeEnum {
    

    /// The spacing mode is inherited from the parent.
    ///
    /// "SPACING_MODE_UNSPECIFIED"
    #[serde(rename="SPACING_MODE_UNSPECIFIED")]
    SPACINGMODEUNSPECIFIED,
    

    /// Paragraph spacing is always rendered.
    ///
    /// "NEVER_COLLAPSE"
    #[serde(rename="NEVER_COLLAPSE")]
    NEVERCOLLAPSE,
    

    /// Paragraph spacing is skipped between list elements.
    ///
    /// "COLLAPSE_LISTS"
    #[serde(rename="COLLAPSE_LISTS")]
    COLLAPSELISTS,
}

impl AsRef<str> for ParagraphStyleSpacingModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParagraphStyleSpacingModeEnum::SPACINGMODEUNSPECIFIED => "SPACING_MODE_UNSPECIFIED",
            ParagraphStyleSpacingModeEnum::NEVERCOLLAPSE => "NEVER_COLLAPSE",
            ParagraphStyleSpacingModeEnum::COLLAPSELISTS => "COLLAPSE_LISTS",
        }
    }
}

impl std::convert::TryFrom< &str> for ParagraphStyleSpacingModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPACING_MODE_UNSPECIFIED" => Ok(ParagraphStyleSpacingModeEnum::SPACINGMODEUNSPECIFIED),
           "NEVER_COLLAPSE" => Ok(ParagraphStyleSpacingModeEnum::NEVERCOLLAPSE),
           "COLLAPSE_LISTS" => Ok(ParagraphStyleSpacingModeEnum::COLLAPSELISTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParagraphStyleSpacingModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PositionedObjectPositioningLayoutEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The layout of this positioned object.
pub enum PositionedObjectPositioningLayoutEnum {
    

    /// The layout is unspecified.
    ///
    /// "POSITIONED_OBJECT_LAYOUT_UNSPECIFIED"
    #[serde(rename="POSITIONED_OBJECT_LAYOUT_UNSPECIFIED")]
    POSITIONEDOBJECTLAYOUTUNSPECIFIED,
    

    /// The text wraps around the positioned object.
    ///
    /// "WRAP_TEXT"
    #[serde(rename="WRAP_TEXT")]
    WRAPTEXT,
    

    /// Breaks text such that the positioned object is on the left and text is on the right.
    ///
    /// "BREAK_LEFT"
    #[serde(rename="BREAK_LEFT")]
    BREAKLEFT,
    

    /// Breaks text such that the positioned object is on the right and text is on the left.
    ///
    /// "BREAK_RIGHT"
    #[serde(rename="BREAK_RIGHT")]
    BREAKRIGHT,
    

    /// Breaks text such that there's no text on the left or right of the positioned object.
    ///
    /// "BREAK_LEFT_RIGHT"
    #[serde(rename="BREAK_LEFT_RIGHT")]
    BREAKLEFTRIGHT,
    

    /// The positioned object is in front of the text.
    ///
    /// "IN_FRONT_OF_TEXT"
    #[serde(rename="IN_FRONT_OF_TEXT")]
    INFRONTOFTEXT,
    

    /// The positioned object is behind the text.
    ///
    /// "BEHIND_TEXT"
    #[serde(rename="BEHIND_TEXT")]
    BEHINDTEXT,
}

impl AsRef<str> for PositionedObjectPositioningLayoutEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PositionedObjectPositioningLayoutEnum::POSITIONEDOBJECTLAYOUTUNSPECIFIED => "POSITIONED_OBJECT_LAYOUT_UNSPECIFIED",
            PositionedObjectPositioningLayoutEnum::WRAPTEXT => "WRAP_TEXT",
            PositionedObjectPositioningLayoutEnum::BREAKLEFT => "BREAK_LEFT",
            PositionedObjectPositioningLayoutEnum::BREAKRIGHT => "BREAK_RIGHT",
            PositionedObjectPositioningLayoutEnum::BREAKLEFTRIGHT => "BREAK_LEFT_RIGHT",
            PositionedObjectPositioningLayoutEnum::INFRONTOFTEXT => "IN_FRONT_OF_TEXT",
            PositionedObjectPositioningLayoutEnum::BEHINDTEXT => "BEHIND_TEXT",
        }
    }
}

impl std::convert::TryFrom< &str> for PositionedObjectPositioningLayoutEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POSITIONED_OBJECT_LAYOUT_UNSPECIFIED" => Ok(PositionedObjectPositioningLayoutEnum::POSITIONEDOBJECTLAYOUTUNSPECIFIED),
           "WRAP_TEXT" => Ok(PositionedObjectPositioningLayoutEnum::WRAPTEXT),
           "BREAK_LEFT" => Ok(PositionedObjectPositioningLayoutEnum::BREAKLEFT),
           "BREAK_RIGHT" => Ok(PositionedObjectPositioningLayoutEnum::BREAKRIGHT),
           "BREAK_LEFT_RIGHT" => Ok(PositionedObjectPositioningLayoutEnum::BREAKLEFTRIGHT),
           "IN_FRONT_OF_TEXT" => Ok(PositionedObjectPositioningLayoutEnum::INFRONTOFTEXT),
           "BEHIND_TEXT" => Ok(PositionedObjectPositioningLayoutEnum::BEHINDTEXT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PositionedObjectPositioningLayoutEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReplaceImageRequestImageReplaceMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The replacement method.
pub enum ReplaceImageRequestImageReplaceMethodEnum {
    

    /// Unspecified image replace method. This value must not be used.
    ///
    /// "IMAGE_REPLACE_METHOD_UNSPECIFIED"
    #[serde(rename="IMAGE_REPLACE_METHOD_UNSPECIFIED")]
    IMAGEREPLACEMETHODUNSPECIFIED,
    

    /// Scales and centers the image to fill the bounds of the original image. The image may be cropped in order to fill the original image's bounds. The rendered size of the image will be the same as the original image.
    ///
    /// "CENTER_CROP"
    #[serde(rename="CENTER_CROP")]
    CENTERCROP,
}

impl AsRef<str> for ReplaceImageRequestImageReplaceMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReplaceImageRequestImageReplaceMethodEnum::IMAGEREPLACEMETHODUNSPECIFIED => "IMAGE_REPLACE_METHOD_UNSPECIFIED",
            ReplaceImageRequestImageReplaceMethodEnum::CENTERCROP => "CENTER_CROP",
        }
    }
}

impl std::convert::TryFrom< &str> for ReplaceImageRequestImageReplaceMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMAGE_REPLACE_METHOD_UNSPECIFIED" => Ok(ReplaceImageRequestImageReplaceMethodEnum::IMAGEREPLACEMETHODUNSPECIFIED),
           "CENTER_CROP" => Ok(ReplaceImageRequestImageReplaceMethodEnum::CENTERCROP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReplaceImageRequestImageReplaceMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SectionStyleColumnSeparatorStyleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The style of column separators. This style can be set even when there's one column in the section. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
pub enum SectionStyleColumnSeparatorStyleEnum {
    

    /// An unspecified column separator style.
    ///
    /// "COLUMN_SEPARATOR_STYLE_UNSPECIFIED"
    #[serde(rename="COLUMN_SEPARATOR_STYLE_UNSPECIFIED")]
    COLUMNSEPARATORSTYLEUNSPECIFIED,
    

    /// No column separator lines between columns.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Renders a column separator line between each column.
    ///
    /// "BETWEEN_EACH_COLUMN"
    #[serde(rename="BETWEEN_EACH_COLUMN")]
    BETWEENEACHCOLUMN,
}

impl AsRef<str> for SectionStyleColumnSeparatorStyleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SectionStyleColumnSeparatorStyleEnum::COLUMNSEPARATORSTYLEUNSPECIFIED => "COLUMN_SEPARATOR_STYLE_UNSPECIFIED",
            SectionStyleColumnSeparatorStyleEnum::NONE => "NONE",
            SectionStyleColumnSeparatorStyleEnum::BETWEENEACHCOLUMN => "BETWEEN_EACH_COLUMN",
        }
    }
}

impl std::convert::TryFrom< &str> for SectionStyleColumnSeparatorStyleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COLUMN_SEPARATOR_STYLE_UNSPECIFIED" => Ok(SectionStyleColumnSeparatorStyleEnum::COLUMNSEPARATORSTYLEUNSPECIFIED),
           "NONE" => Ok(SectionStyleColumnSeparatorStyleEnum::NONE),
           "BETWEEN_EACH_COLUMN" => Ok(SectionStyleColumnSeparatorStyleEnum::BETWEENEACHCOLUMN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SectionStyleColumnSeparatorStyleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SectionStyleContentDirectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The content direction of this section. If unset, the value defaults to LEFT_TO_RIGHT. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
pub enum SectionStyleContentDirectionEnum {
    

    /// The content direction is unspecified.
    ///
    /// "CONTENT_DIRECTION_UNSPECIFIED"
    #[serde(rename="CONTENT_DIRECTION_UNSPECIFIED")]
    CONTENTDIRECTIONUNSPECIFIED,
    

    /// The content goes from left to right.
    ///
    /// "LEFT_TO_RIGHT"
    #[serde(rename="LEFT_TO_RIGHT")]
    LEFTTORIGHT,
    

    /// The content goes from right to left.
    ///
    /// "RIGHT_TO_LEFT"
    #[serde(rename="RIGHT_TO_LEFT")]
    RIGHTTOLEFT,
}

impl AsRef<str> for SectionStyleContentDirectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SectionStyleContentDirectionEnum::CONTENTDIRECTIONUNSPECIFIED => "CONTENT_DIRECTION_UNSPECIFIED",
            SectionStyleContentDirectionEnum::LEFTTORIGHT => "LEFT_TO_RIGHT",
            SectionStyleContentDirectionEnum::RIGHTTOLEFT => "RIGHT_TO_LEFT",
        }
    }
}

impl std::convert::TryFrom< &str> for SectionStyleContentDirectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_DIRECTION_UNSPECIFIED" => Ok(SectionStyleContentDirectionEnum::CONTENTDIRECTIONUNSPECIFIED),
           "LEFT_TO_RIGHT" => Ok(SectionStyleContentDirectionEnum::LEFTTORIGHT),
           "RIGHT_TO_LEFT" => Ok(SectionStyleContentDirectionEnum::RIGHTTOLEFT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SectionStyleContentDirectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SectionStyleSectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of section.
pub enum SectionStyleSectionTypeEnum {
    

    /// The section type is unspecified.
    ///
    /// "SECTION_TYPE_UNSPECIFIED"
    #[serde(rename="SECTION_TYPE_UNSPECIFIED")]
    SECTIONTYPEUNSPECIFIED,
    

    /// The section starts immediately after the last paragraph of the previous section.
    ///
    /// "CONTINUOUS"
    #[serde(rename="CONTINUOUS")]
    CONTINUOUS,
    

    /// The section starts on the next page.
    ///
    /// "NEXT_PAGE"
    #[serde(rename="NEXT_PAGE")]
    NEXTPAGE,
}

impl AsRef<str> for SectionStyleSectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SectionStyleSectionTypeEnum::SECTIONTYPEUNSPECIFIED => "SECTION_TYPE_UNSPECIFIED",
            SectionStyleSectionTypeEnum::CONTINUOUS => "CONTINUOUS",
            SectionStyleSectionTypeEnum::NEXTPAGE => "NEXT_PAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for SectionStyleSectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_TYPE_UNSPECIFIED" => Ok(SectionStyleSectionTypeEnum::SECTIONTYPEUNSPECIFIED),
           "CONTINUOUS" => Ok(SectionStyleSectionTypeEnum::CONTINUOUS),
           "NEXT_PAGE" => Ok(SectionStyleSectionTypeEnum::NEXTPAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SectionStyleSectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TabStopAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The alignment of this tab stop. If unset, the value defaults to START.
pub enum TabStopAlignmentEnum {
    

    /// The tab stop alignment is unspecified.
    ///
    /// "TAB_STOP_ALIGNMENT_UNSPECIFIED"
    #[serde(rename="TAB_STOP_ALIGNMENT_UNSPECIFIED")]
    TABSTOPALIGNMENTUNSPECIFIED,
    

    /// The tab stop is aligned to the start of the line. This is the default.
    ///
    /// "START"
    #[serde(rename="START")]
    START,
    

    /// The tab stop is aligned to the center of the line.
    ///
    /// "CENTER"
    #[serde(rename="CENTER")]
    CENTER,
    

    /// The tab stop is aligned to the end of the line.
    ///
    /// "END"
    #[serde(rename="END")]
    END,
}

impl AsRef<str> for TabStopAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TabStopAlignmentEnum::TABSTOPALIGNMENTUNSPECIFIED => "TAB_STOP_ALIGNMENT_UNSPECIFIED",
            TabStopAlignmentEnum::START => "START",
            TabStopAlignmentEnum::CENTER => "CENTER",
            TabStopAlignmentEnum::END => "END",
        }
    }
}

impl std::convert::TryFrom< &str> for TabStopAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TAB_STOP_ALIGNMENT_UNSPECIFIED" => Ok(TabStopAlignmentEnum::TABSTOPALIGNMENTUNSPECIFIED),
           "START" => Ok(TabStopAlignmentEnum::START),
           "CENTER" => Ok(TabStopAlignmentEnum::CENTER),
           "END" => Ok(TabStopAlignmentEnum::END),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TabStopAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TableCellBorderDashStyleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The dash style of the border.
pub enum TableCellBorderDashStyleEnum {
    

    /// Unspecified dash style.
    ///
    /// "DASH_STYLE_UNSPECIFIED"
    #[serde(rename="DASH_STYLE_UNSPECIFIED")]
    DASHSTYLEUNSPECIFIED,
    

    /// Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style.
    ///
    /// "SOLID"
    #[serde(rename="SOLID")]
    SOLID,
    

    /// Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'.
    ///
    /// "DOT"
    #[serde(rename="DOT")]
    DOT,
    

    /// Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'.
    ///
    /// "DASH"
    #[serde(rename="DASH")]
    DASH,
}

impl AsRef<str> for TableCellBorderDashStyleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TableCellBorderDashStyleEnum::DASHSTYLEUNSPECIFIED => "DASH_STYLE_UNSPECIFIED",
            TableCellBorderDashStyleEnum::SOLID => "SOLID",
            TableCellBorderDashStyleEnum::DOT => "DOT",
            TableCellBorderDashStyleEnum::DASH => "DASH",
        }
    }
}

impl std::convert::TryFrom< &str> for TableCellBorderDashStyleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DASH_STYLE_UNSPECIFIED" => Ok(TableCellBorderDashStyleEnum::DASHSTYLEUNSPECIFIED),
           "SOLID" => Ok(TableCellBorderDashStyleEnum::SOLID),
           "DOT" => Ok(TableCellBorderDashStyleEnum::DOT),
           "DASH" => Ok(TableCellBorderDashStyleEnum::DASH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TableCellBorderDashStyleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TableCellStyleContentAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The alignment of the content in the table cell. The default alignment matches the alignment for newly created table cells in the Docs editor.
pub enum TableCellStyleContentAlignmentEnum {
    

    /// An unspecified content alignment. The content alignment is inherited from the parent if one exists.
    ///
    /// "CONTENT_ALIGNMENT_UNSPECIFIED"
    #[serde(rename="CONTENT_ALIGNMENT_UNSPECIFIED")]
    CONTENTALIGNMENTUNSPECIFIED,
    

    /// An unsupported content alignment.
    ///
    /// "CONTENT_ALIGNMENT_UNSUPPORTED"
    #[serde(rename="CONTENT_ALIGNMENT_UNSUPPORTED")]
    CONTENTALIGNMENTUNSUPPORTED,
    

    /// An alignment that aligns the content to the top of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 't'.
    ///
    /// "TOP"
    #[serde(rename="TOP")]
    TOP,
    

    /// An alignment that aligns the content to the middle of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 'ctr'.
    ///
    /// "MIDDLE"
    #[serde(rename="MIDDLE")]
    MIDDLE,
    

    /// An alignment that aligns the content to the bottom of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 'b'.
    ///
    /// "BOTTOM"
    #[serde(rename="BOTTOM")]
    BOTTOM,
}

impl AsRef<str> for TableCellStyleContentAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TableCellStyleContentAlignmentEnum::CONTENTALIGNMENTUNSPECIFIED => "CONTENT_ALIGNMENT_UNSPECIFIED",
            TableCellStyleContentAlignmentEnum::CONTENTALIGNMENTUNSUPPORTED => "CONTENT_ALIGNMENT_UNSUPPORTED",
            TableCellStyleContentAlignmentEnum::TOP => "TOP",
            TableCellStyleContentAlignmentEnum::MIDDLE => "MIDDLE",
            TableCellStyleContentAlignmentEnum::BOTTOM => "BOTTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for TableCellStyleContentAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_ALIGNMENT_UNSPECIFIED" => Ok(TableCellStyleContentAlignmentEnum::CONTENTALIGNMENTUNSPECIFIED),
           "CONTENT_ALIGNMENT_UNSUPPORTED" => Ok(TableCellStyleContentAlignmentEnum::CONTENTALIGNMENTUNSUPPORTED),
           "TOP" => Ok(TableCellStyleContentAlignmentEnum::TOP),
           "MIDDLE" => Ok(TableCellStyleContentAlignmentEnum::MIDDLE),
           "BOTTOM" => Ok(TableCellStyleContentAlignmentEnum::BOTTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TableCellStyleContentAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TableColumnPropertyWidthTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The width type of the column.
pub enum TableColumnPropertyWidthTypeEnum {
    

    /// The column width type is unspecified.
    ///
    /// "WIDTH_TYPE_UNSPECIFIED"
    #[serde(rename="WIDTH_TYPE_UNSPECIFIED")]
    WIDTHTYPEUNSPECIFIED,
    

    /// The column width is evenly distributed among the other evenly distributed columns. The width of the column is automatically determined and will have an equal portion of the width remaining for the table after accounting for all columns with specified widths.
    ///
    /// "EVENLY_DISTRIBUTED"
    #[serde(rename="EVENLY_DISTRIBUTED")]
    EVENLYDISTRIBUTED,
    

    /// A fixed column width. The width property contains the column's width.
    ///
    /// "FIXED_WIDTH"
    #[serde(rename="FIXED_WIDTH")]
    FIXEDWIDTH,
}

impl AsRef<str> for TableColumnPropertyWidthTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TableColumnPropertyWidthTypeEnum::WIDTHTYPEUNSPECIFIED => "WIDTH_TYPE_UNSPECIFIED",
            TableColumnPropertyWidthTypeEnum::EVENLYDISTRIBUTED => "EVENLY_DISTRIBUTED",
            TableColumnPropertyWidthTypeEnum::FIXEDWIDTH => "FIXED_WIDTH",
        }
    }
}

impl std::convert::TryFrom< &str> for TableColumnPropertyWidthTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WIDTH_TYPE_UNSPECIFIED" => Ok(TableColumnPropertyWidthTypeEnum::WIDTHTYPEUNSPECIFIED),
           "EVENLY_DISTRIBUTED" => Ok(TableColumnPropertyWidthTypeEnum::EVENLYDISTRIBUTED),
           "FIXED_WIDTH" => Ok(TableColumnPropertyWidthTypeEnum::FIXEDWIDTH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TableColumnPropertyWidthTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TextStyleBaselineOffsetEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The text's vertical offset from its normal position. Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically rendered in a smaller font size, computed based on the `font_size` field. Changes in this field don't affect the `font_size`.
pub enum TextStyleBaselineOffsetEnum {
    

    /// The text's baseline offset is inherited from the parent.
    ///
    /// "BASELINE_OFFSET_UNSPECIFIED"
    #[serde(rename="BASELINE_OFFSET_UNSPECIFIED")]
    BASELINEOFFSETUNSPECIFIED,
    

    /// The text is not vertically offset.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// The text is vertically offset upwards (superscript).
    ///
    /// "SUPERSCRIPT"
    #[serde(rename="SUPERSCRIPT")]
    SUPERSCRIPT,
    

    /// The text is vertically offset downwards (subscript).
    ///
    /// "SUBSCRIPT"
    #[serde(rename="SUBSCRIPT")]
    SUBSCRIPT,
}

impl AsRef<str> for TextStyleBaselineOffsetEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TextStyleBaselineOffsetEnum::BASELINEOFFSETUNSPECIFIED => "BASELINE_OFFSET_UNSPECIFIED",
            TextStyleBaselineOffsetEnum::NONE => "NONE",
            TextStyleBaselineOffsetEnum::SUPERSCRIPT => "SUPERSCRIPT",
            TextStyleBaselineOffsetEnum::SUBSCRIPT => "SUBSCRIPT",
        }
    }
}

impl std::convert::TryFrom< &str> for TextStyleBaselineOffsetEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASELINE_OFFSET_UNSPECIFIED" => Ok(TextStyleBaselineOffsetEnum::BASELINEOFFSETUNSPECIFIED),
           "NONE" => Ok(TextStyleBaselineOffsetEnum::NONE),
           "SUPERSCRIPT" => Ok(TextStyleBaselineOffsetEnum::SUPERSCRIPT),
           "SUBSCRIPT" => Ok(TextStyleBaselineOffsetEnum::SUBSCRIPT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TextStyleBaselineOffsetEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DocumentSuggestionsViewModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The suggestions view mode to apply to the document. This allows viewing the document with all suggestions inline, accepted or rejected. If one is not specified, DEFAULT_FOR_CURRENT_ACCESS is used.
pub enum DocumentSuggestionsViewModeEnum {
    

    /// The SuggestionsViewMode applied to the returned document depends on the user's current access level. If the user only has view access, PREVIEW_WITHOUT_SUGGESTIONS is applied. Otherwise, SUGGESTIONS_INLINE is applied. This is the default suggestions view mode.
    ///
    /// "DEFAULT_FOR_CURRENT_ACCESS"
    #[serde(rename="DEFAULT_FOR_CURRENT_ACCESS")]
    DEFAULTFORCURRENTACCESS,
    

    /// The returned document has suggestions inline. Suggested changes will be differentiated from base content within the document. Requests to retrieve a document using this mode will return a 403 error if the user does not have permission to view suggested changes.
    ///
    /// "SUGGESTIONS_INLINE"
    #[serde(rename="SUGGESTIONS_INLINE")]
    SUGGESTIONSINLINE,
    

    /// The returned document is a preview with all suggested changes accepted. Requests to retrieve a document using this mode will return a 403 error if the user does not have permission to view suggested changes.
    ///
    /// "PREVIEW_SUGGESTIONS_ACCEPTED"
    #[serde(rename="PREVIEW_SUGGESTIONS_ACCEPTED")]
    PREVIEWSUGGESTIONSACCEPTED,
    

    /// The returned document is a preview with all suggested changes rejected if there are any suggestions in the document.
    ///
    /// "PREVIEW_WITHOUT_SUGGESTIONS"
    #[serde(rename="PREVIEW_WITHOUT_SUGGESTIONS")]
    PREVIEWWITHOUTSUGGESTIONS,
}

impl AsRef<str> for DocumentSuggestionsViewModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DocumentSuggestionsViewModeEnum::DEFAULTFORCURRENTACCESS => "DEFAULT_FOR_CURRENT_ACCESS",
            DocumentSuggestionsViewModeEnum::SUGGESTIONSINLINE => "SUGGESTIONS_INLINE",
            DocumentSuggestionsViewModeEnum::PREVIEWSUGGESTIONSACCEPTED => "PREVIEW_SUGGESTIONS_ACCEPTED",
            DocumentSuggestionsViewModeEnum::PREVIEWWITHOUTSUGGESTIONS => "PREVIEW_WITHOUT_SUGGESTIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for DocumentSuggestionsViewModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT_FOR_CURRENT_ACCESS" => Ok(DocumentSuggestionsViewModeEnum::DEFAULTFORCURRENTACCESS),
           "SUGGESTIONS_INLINE" => Ok(DocumentSuggestionsViewModeEnum::SUGGESTIONSINLINE),
           "PREVIEW_SUGGESTIONS_ACCEPTED" => Ok(DocumentSuggestionsViewModeEnum::PREVIEWSUGGESTIONSACCEPTED),
           "PREVIEW_WITHOUT_SUGGESTIONS" => Ok(DocumentSuggestionsViewModeEnum::PREVIEWWITHOUTSUGGESTIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DocumentSuggestionsViewModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


