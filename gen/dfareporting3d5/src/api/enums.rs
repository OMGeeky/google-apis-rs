use super::*;



// region CreativeAssetIdTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of asset to upload. This is a required field. FLASH and IMAGE are no longer supported for new uploads. All image assets should use HTML_IMAGE.
pub enum CreativeAssetIdTypeEnum {
    
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    
    /// "FLASH"
    #[serde(rename="FLASH")]
    FLASH,
    
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    
    /// "HTML"
    #[serde(rename="HTML")]
    HTML,
    
    /// "HTML_IMAGE"
    #[serde(rename="HTML_IMAGE")]
    HTMLIMAGE,
    
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
}

impl AsRef<str> for CreativeAssetIdTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetIdTypeEnum::IMAGE => "IMAGE",
            CreativeAssetIdTypeEnum::FLASH => "FLASH",
            CreativeAssetIdTypeEnum::VIDEO => "VIDEO",
            CreativeAssetIdTypeEnum::HTML => "HTML",
            CreativeAssetIdTypeEnum::HTMLIMAGE => "HTML_IMAGE",
            CreativeAssetIdTypeEnum::AUDIO => "AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetIdTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMAGE" => Ok(CreativeAssetIdTypeEnum::IMAGE),
           "FLASH" => Ok(CreativeAssetIdTypeEnum::FLASH),
           "VIDEO" => Ok(CreativeAssetIdTypeEnum::VIDEO),
           "HTML" => Ok(CreativeAssetIdTypeEnum::HTML),
           "HTML_IMAGE" => Ok(CreativeAssetIdTypeEnum::HTMLIMAGE),
           "AUDIO" => Ok(CreativeAssetIdTypeEnum::AUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetIdTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetMetadataDetectedFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of feature dependencies for the creative asset that are detected by Campaign Manager. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative correctly. This is a read-only, auto-generated field.
pub enum CreativeAssetMetadataDetectedFeaturesEnum {
    
    /// "CSS_FONT_FACE"
    #[serde(rename="CSS_FONT_FACE")]
    CSSFONTFACE,
    
    /// "CSS_BACKGROUND_SIZE"
    #[serde(rename="CSS_BACKGROUND_SIZE")]
    CSSBACKGROUNDSIZE,
    
    /// "CSS_BORDER_IMAGE"
    #[serde(rename="CSS_BORDER_IMAGE")]
    CSSBORDERIMAGE,
    
    /// "CSS_BORDER_RADIUS"
    #[serde(rename="CSS_BORDER_RADIUS")]
    CSSBORDERRADIUS,
    
    /// "CSS_BOX_SHADOW"
    #[serde(rename="CSS_BOX_SHADOW")]
    CSSBOXSHADOW,
    
    /// "CSS_FLEX_BOX"
    #[serde(rename="CSS_FLEX_BOX")]
    CSSFLEXBOX,
    
    /// "CSS_HSLA"
    #[serde(rename="CSS_HSLA")]
    CSSHSLA,
    
    /// "CSS_MULTIPLE_BGS"
    #[serde(rename="CSS_MULTIPLE_BGS")]
    CSSMULTIPLEBGS,
    
    /// "CSS_OPACITY"
    #[serde(rename="CSS_OPACITY")]
    CSSOPACITY,
    
    /// "CSS_RGBA"
    #[serde(rename="CSS_RGBA")]
    CSSRGBA,
    
    /// "CSS_TEXT_SHADOW"
    #[serde(rename="CSS_TEXT_SHADOW")]
    CSSTEXTSHADOW,
    
    /// "CSS_ANIMATIONS"
    #[serde(rename="CSS_ANIMATIONS")]
    CSSANIMATIONS,
    
    /// "CSS_COLUMNS"
    #[serde(rename="CSS_COLUMNS")]
    CSSCOLUMNS,
    
    /// "CSS_GENERATED_CONTENT"
    #[serde(rename="CSS_GENERATED_CONTENT")]
    CSSGENERATEDCONTENT,
    
    /// "CSS_GRADIENTS"
    #[serde(rename="CSS_GRADIENTS")]
    CSSGRADIENTS,
    
    /// "CSS_REFLECTIONS"
    #[serde(rename="CSS_REFLECTIONS")]
    CSSREFLECTIONS,
    
    /// "CSS_TRANSFORMS"
    #[serde(rename="CSS_TRANSFORMS")]
    CSSTRANSFORMS,
    
    /// "CSS_TRANSFORMS3D"
    #[serde(rename="CSS_TRANSFORMS3D")]
    CSSTRANSFORMS3D,
    
    /// "CSS_TRANSITIONS"
    #[serde(rename="CSS_TRANSITIONS")]
    CSSTRANSITIONS,
    
    /// "APPLICATION_CACHE"
    #[serde(rename="APPLICATION_CACHE")]
    APPLICATIONCACHE,
    
    /// "CANVAS"
    #[serde(rename="CANVAS")]
    CANVAS,
    
    /// "CANVAS_TEXT"
    #[serde(rename="CANVAS_TEXT")]
    CANVASTEXT,
    
    /// "DRAG_AND_DROP"
    #[serde(rename="DRAG_AND_DROP")]
    DRAGANDDROP,
    
    /// "HASH_CHANGE"
    #[serde(rename="HASH_CHANGE")]
    HASHCHANGE,
    
    /// "HISTORY"
    #[serde(rename="HISTORY")]
    HISTORY,
    
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
    
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    
    /// "INDEXED_DB"
    #[serde(rename="INDEXED_DB")]
    INDEXEDDB,
    
    /// "INPUT_ATTR_AUTOCOMPLETE"
    #[serde(rename="INPUT_ATTR_AUTOCOMPLETE")]
    INPUTATTRAUTOCOMPLETE,
    
    /// "INPUT_ATTR_AUTOFOCUS"
    #[serde(rename="INPUT_ATTR_AUTOFOCUS")]
    INPUTATTRAUTOFOCUS,
    
    /// "INPUT_ATTR_LIST"
    #[serde(rename="INPUT_ATTR_LIST")]
    INPUTATTRLIST,
    
    /// "INPUT_ATTR_PLACEHOLDER"
    #[serde(rename="INPUT_ATTR_PLACEHOLDER")]
    INPUTATTRPLACEHOLDER,
    
    /// "INPUT_ATTR_MAX"
    #[serde(rename="INPUT_ATTR_MAX")]
    INPUTATTRMAX,
    
    /// "INPUT_ATTR_MIN"
    #[serde(rename="INPUT_ATTR_MIN")]
    INPUTATTRMIN,
    
    /// "INPUT_ATTR_MULTIPLE"
    #[serde(rename="INPUT_ATTR_MULTIPLE")]
    INPUTATTRMULTIPLE,
    
    /// "INPUT_ATTR_PATTERN"
    #[serde(rename="INPUT_ATTR_PATTERN")]
    INPUTATTRPATTERN,
    
    /// "INPUT_ATTR_REQUIRED"
    #[serde(rename="INPUT_ATTR_REQUIRED")]
    INPUTATTRREQUIRED,
    
    /// "INPUT_ATTR_STEP"
    #[serde(rename="INPUT_ATTR_STEP")]
    INPUTATTRSTEP,
    
    /// "INPUT_TYPE_SEARCH"
    #[serde(rename="INPUT_TYPE_SEARCH")]
    INPUTTYPESEARCH,
    
    /// "INPUT_TYPE_TEL"
    #[serde(rename="INPUT_TYPE_TEL")]
    INPUTTYPETEL,
    
    /// "INPUT_TYPE_URL"
    #[serde(rename="INPUT_TYPE_URL")]
    INPUTTYPEURL,
    
    /// "INPUT_TYPE_EMAIL"
    #[serde(rename="INPUT_TYPE_EMAIL")]
    INPUTTYPEEMAIL,
    
    /// "INPUT_TYPE_DATETIME"
    #[serde(rename="INPUT_TYPE_DATETIME")]
    INPUTTYPEDATETIME,
    
    /// "INPUT_TYPE_DATE"
    #[serde(rename="INPUT_TYPE_DATE")]
    INPUTTYPEDATE,
    
    /// "INPUT_TYPE_MONTH"
    #[serde(rename="INPUT_TYPE_MONTH")]
    INPUTTYPEMONTH,
    
    /// "INPUT_TYPE_WEEK"
    #[serde(rename="INPUT_TYPE_WEEK")]
    INPUTTYPEWEEK,
    
    /// "INPUT_TYPE_TIME"
    #[serde(rename="INPUT_TYPE_TIME")]
    INPUTTYPETIME,
    
    /// "INPUT_TYPE_DATETIME_LOCAL"
    #[serde(rename="INPUT_TYPE_DATETIME_LOCAL")]
    INPUTTYPEDATETIMELOCAL,
    
    /// "INPUT_TYPE_NUMBER"
    #[serde(rename="INPUT_TYPE_NUMBER")]
    INPUTTYPENUMBER,
    
    /// "INPUT_TYPE_RANGE"
    #[serde(rename="INPUT_TYPE_RANGE")]
    INPUTTYPERANGE,
    
    /// "INPUT_TYPE_COLOR"
    #[serde(rename="INPUT_TYPE_COLOR")]
    INPUTTYPECOLOR,
    
    /// "LOCAL_STORAGE"
    #[serde(rename="LOCAL_STORAGE")]
    LOCALSTORAGE,
    
    /// "POST_MESSAGE"
    #[serde(rename="POST_MESSAGE")]
    POSTMESSAGE,
    
    /// "SESSION_STORAGE"
    #[serde(rename="SESSION_STORAGE")]
    SESSIONSTORAGE,
    
    /// "WEB_SOCKETS"
    #[serde(rename="WEB_SOCKETS")]
    WEBSOCKETS,
    
    /// "WEB_SQL_DATABASE"
    #[serde(rename="WEB_SQL_DATABASE")]
    WEBSQLDATABASE,
    
    /// "WEB_WORKERS"
    #[serde(rename="WEB_WORKERS")]
    WEBWORKERS,
    
    /// "GEO_LOCATION"
    #[serde(rename="GEO_LOCATION")]
    GEOLOCATION,
    
    /// "INLINE_SVG"
    #[serde(rename="INLINE_SVG")]
    INLINESVG,
    
    /// "SMIL"
    #[serde(rename="SMIL")]
    SMIL,
    
    /// "SVG_HREF"
    #[serde(rename="SVG_HREF")]
    SVGHREF,
    
    /// "SVG_CLIP_PATHS"
    #[serde(rename="SVG_CLIP_PATHS")]
    SVGCLIPPATHS,
    
    /// "TOUCH"
    #[serde(rename="TOUCH")]
    TOUCH,
    
    /// "WEBGL"
    #[serde(rename="WEBGL")]
    WEBGL,
    
    /// "SVG_FILTERS"
    #[serde(rename="SVG_FILTERS")]
    SVGFILTERS,
    
    /// "SVG_FE_IMAGE"
    #[serde(rename="SVG_FE_IMAGE")]
    SVGFEIMAGE,
}

impl AsRef<str> for CreativeAssetMetadataDetectedFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetMetadataDetectedFeaturesEnum::CSSFONTFACE => "CSS_FONT_FACE",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSBACKGROUNDSIZE => "CSS_BACKGROUND_SIZE",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSBORDERIMAGE => "CSS_BORDER_IMAGE",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSBORDERRADIUS => "CSS_BORDER_RADIUS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSBOXSHADOW => "CSS_BOX_SHADOW",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSFLEXBOX => "CSS_FLEX_BOX",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSHSLA => "CSS_HSLA",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSMULTIPLEBGS => "CSS_MULTIPLE_BGS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSOPACITY => "CSS_OPACITY",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSRGBA => "CSS_RGBA",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSTEXTSHADOW => "CSS_TEXT_SHADOW",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSANIMATIONS => "CSS_ANIMATIONS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSCOLUMNS => "CSS_COLUMNS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSGENERATEDCONTENT => "CSS_GENERATED_CONTENT",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSGRADIENTS => "CSS_GRADIENTS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSREFLECTIONS => "CSS_REFLECTIONS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSFORMS => "CSS_TRANSFORMS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSFORMS3D => "CSS_TRANSFORMS3D",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSITIONS => "CSS_TRANSITIONS",
            CreativeAssetMetadataDetectedFeaturesEnum::APPLICATIONCACHE => "APPLICATION_CACHE",
            CreativeAssetMetadataDetectedFeaturesEnum::CANVAS => "CANVAS",
            CreativeAssetMetadataDetectedFeaturesEnum::CANVASTEXT => "CANVAS_TEXT",
            CreativeAssetMetadataDetectedFeaturesEnum::DRAGANDDROP => "DRAG_AND_DROP",
            CreativeAssetMetadataDetectedFeaturesEnum::HASHCHANGE => "HASH_CHANGE",
            CreativeAssetMetadataDetectedFeaturesEnum::HISTORY => "HISTORY",
            CreativeAssetMetadataDetectedFeaturesEnum::AUDIO => "AUDIO",
            CreativeAssetMetadataDetectedFeaturesEnum::VIDEO => "VIDEO",
            CreativeAssetMetadataDetectedFeaturesEnum::INDEXEDDB => "INDEXED_DB",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRAUTOCOMPLETE => "INPUT_ATTR_AUTOCOMPLETE",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRAUTOFOCUS => "INPUT_ATTR_AUTOFOCUS",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRLIST => "INPUT_ATTR_LIST",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRPLACEHOLDER => "INPUT_ATTR_PLACEHOLDER",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMAX => "INPUT_ATTR_MAX",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMIN => "INPUT_ATTR_MIN",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMULTIPLE => "INPUT_ATTR_MULTIPLE",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRPATTERN => "INPUT_ATTR_PATTERN",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRREQUIRED => "INPUT_ATTR_REQUIRED",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRSTEP => "INPUT_ATTR_STEP",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPESEARCH => "INPUT_TYPE_SEARCH",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPETEL => "INPUT_TYPE_TEL",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEURL => "INPUT_TYPE_URL",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEEMAIL => "INPUT_TYPE_EMAIL",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATETIME => "INPUT_TYPE_DATETIME",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATE => "INPUT_TYPE_DATE",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEMONTH => "INPUT_TYPE_MONTH",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEWEEK => "INPUT_TYPE_WEEK",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPETIME => "INPUT_TYPE_TIME",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATETIMELOCAL => "INPUT_TYPE_DATETIME_LOCAL",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPENUMBER => "INPUT_TYPE_NUMBER",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPERANGE => "INPUT_TYPE_RANGE",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPECOLOR => "INPUT_TYPE_COLOR",
            CreativeAssetMetadataDetectedFeaturesEnum::LOCALSTORAGE => "LOCAL_STORAGE",
            CreativeAssetMetadataDetectedFeaturesEnum::POSTMESSAGE => "POST_MESSAGE",
            CreativeAssetMetadataDetectedFeaturesEnum::SESSIONSTORAGE => "SESSION_STORAGE",
            CreativeAssetMetadataDetectedFeaturesEnum::WEBSOCKETS => "WEB_SOCKETS",
            CreativeAssetMetadataDetectedFeaturesEnum::WEBSQLDATABASE => "WEB_SQL_DATABASE",
            CreativeAssetMetadataDetectedFeaturesEnum::WEBWORKERS => "WEB_WORKERS",
            CreativeAssetMetadataDetectedFeaturesEnum::GEOLOCATION => "GEO_LOCATION",
            CreativeAssetMetadataDetectedFeaturesEnum::INLINESVG => "INLINE_SVG",
            CreativeAssetMetadataDetectedFeaturesEnum::SMIL => "SMIL",
            CreativeAssetMetadataDetectedFeaturesEnum::SVGHREF => "SVG_HREF",
            CreativeAssetMetadataDetectedFeaturesEnum::SVGCLIPPATHS => "SVG_CLIP_PATHS",
            CreativeAssetMetadataDetectedFeaturesEnum::TOUCH => "TOUCH",
            CreativeAssetMetadataDetectedFeaturesEnum::WEBGL => "WEBGL",
            CreativeAssetMetadataDetectedFeaturesEnum::SVGFILTERS => "SVG_FILTERS",
            CreativeAssetMetadataDetectedFeaturesEnum::SVGFEIMAGE => "SVG_FE_IMAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetMetadataDetectedFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CSS_FONT_FACE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSFONTFACE),
           "CSS_BACKGROUND_SIZE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSBACKGROUNDSIZE),
           "CSS_BORDER_IMAGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSBORDERIMAGE),
           "CSS_BORDER_RADIUS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSBORDERRADIUS),
           "CSS_BOX_SHADOW" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSBOXSHADOW),
           "CSS_FLEX_BOX" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSFLEXBOX),
           "CSS_HSLA" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSHSLA),
           "CSS_MULTIPLE_BGS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSMULTIPLEBGS),
           "CSS_OPACITY" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSOPACITY),
           "CSS_RGBA" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSRGBA),
           "CSS_TEXT_SHADOW" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSTEXTSHADOW),
           "CSS_ANIMATIONS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSANIMATIONS),
           "CSS_COLUMNS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSCOLUMNS),
           "CSS_GENERATED_CONTENT" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSGENERATEDCONTENT),
           "CSS_GRADIENTS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSGRADIENTS),
           "CSS_REFLECTIONS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSREFLECTIONS),
           "CSS_TRANSFORMS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSFORMS),
           "CSS_TRANSFORMS3D" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSFORMS3D),
           "CSS_TRANSITIONS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSITIONS),
           "APPLICATION_CACHE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::APPLICATIONCACHE),
           "CANVAS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CANVAS),
           "CANVAS_TEXT" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CANVASTEXT),
           "DRAG_AND_DROP" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::DRAGANDDROP),
           "HASH_CHANGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::HASHCHANGE),
           "HISTORY" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::HISTORY),
           "AUDIO" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::AUDIO),
           "VIDEO" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::VIDEO),
           "INDEXED_DB" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INDEXEDDB),
           "INPUT_ATTR_AUTOCOMPLETE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRAUTOCOMPLETE),
           "INPUT_ATTR_AUTOFOCUS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRAUTOFOCUS),
           "INPUT_ATTR_LIST" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRLIST),
           "INPUT_ATTR_PLACEHOLDER" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRPLACEHOLDER),
           "INPUT_ATTR_MAX" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMAX),
           "INPUT_ATTR_MIN" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMIN),
           "INPUT_ATTR_MULTIPLE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMULTIPLE),
           "INPUT_ATTR_PATTERN" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRPATTERN),
           "INPUT_ATTR_REQUIRED" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRREQUIRED),
           "INPUT_ATTR_STEP" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRSTEP),
           "INPUT_TYPE_SEARCH" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPESEARCH),
           "INPUT_TYPE_TEL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPETEL),
           "INPUT_TYPE_URL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEURL),
           "INPUT_TYPE_EMAIL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEEMAIL),
           "INPUT_TYPE_DATETIME" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATETIME),
           "INPUT_TYPE_DATE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATE),
           "INPUT_TYPE_MONTH" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEMONTH),
           "INPUT_TYPE_WEEK" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEWEEK),
           "INPUT_TYPE_TIME" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPETIME),
           "INPUT_TYPE_DATETIME_LOCAL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATETIMELOCAL),
           "INPUT_TYPE_NUMBER" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPENUMBER),
           "INPUT_TYPE_RANGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPERANGE),
           "INPUT_TYPE_COLOR" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPECOLOR),
           "LOCAL_STORAGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::LOCALSTORAGE),
           "POST_MESSAGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::POSTMESSAGE),
           "SESSION_STORAGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SESSIONSTORAGE),
           "WEB_SOCKETS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::WEBSOCKETS),
           "WEB_SQL_DATABASE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::WEBSQLDATABASE),
           "WEB_WORKERS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::WEBWORKERS),
           "GEO_LOCATION" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::GEOLOCATION),
           "INLINE_SVG" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INLINESVG),
           "SMIL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SMIL),
           "SVG_HREF" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SVGHREF),
           "SVG_CLIP_PATHS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SVGCLIPPATHS),
           "TOUCH" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::TOUCH),
           "WEBGL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::WEBGL),
           "SVG_FILTERS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SVGFILTERS),
           "SVG_FE_IMAGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SVGFEIMAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetMetadataDetectedFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetMetadataWarnedValidationRulesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rules validated during code generation that generated a warning. This is a read-only, auto-generated field. Possible values are: - "ADMOB_REFERENCED" - "ASSET_FORMAT_UNSUPPORTED_DCM" - "ASSET_INVALID" - "CLICK_TAG_HARD_CODED" - "CLICK_TAG_INVALID" - "CLICK_TAG_IN_GWD" - "CLICK_TAG_MISSING" - "CLICK_TAG_MORE_THAN_ONE" - "CLICK_TAG_NON_TOP_LEVEL" - "COMPONENT_UNSUPPORTED_DCM" - "ENABLER_UNSUPPORTED_METHOD_DCM" - "EXTERNAL_FILE_REFERENCED" - "FILE_DETAIL_EMPTY" - "FILE_TYPE_INVALID" - "GWD_PROPERTIES_INVALID" - "HTML5_FEATURE_UNSUPPORTED" - "LINKED_FILE_NOT_FOUND" - "MAX_FLASH_VERSION_11" - "MRAID_REFERENCED" - "NOT_SSL_COMPLIANT" - "ORPHANED_ASSET" - "PRIMARY_HTML_MISSING" - "SVG_INVALID" - "ZIP_INVALID" 
pub enum CreativeAssetMetadataWarnedValidationRulesEnum {
    
    /// "CLICK_TAG_NON_TOP_LEVEL"
    #[serde(rename="CLICK_TAG_NON_TOP_LEVEL")]
    CLICKTAGNONTOPLEVEL,
    
    /// "CLICK_TAG_MISSING"
    #[serde(rename="CLICK_TAG_MISSING")]
    CLICKTAGMISSING,
    
    /// "CLICK_TAG_MORE_THAN_ONE"
    #[serde(rename="CLICK_TAG_MORE_THAN_ONE")]
    CLICKTAGMORETHANONE,
    
    /// "CLICK_TAG_INVALID"
    #[serde(rename="CLICK_TAG_INVALID")]
    CLICKTAGINVALID,
    
    /// "ORPHANED_ASSET"
    #[serde(rename="ORPHANED_ASSET")]
    ORPHANEDASSET,
    
    /// "PRIMARY_HTML_MISSING"
    #[serde(rename="PRIMARY_HTML_MISSING")]
    PRIMARYHTMLMISSING,
    
    /// "EXTERNAL_FILE_REFERENCED"
    #[serde(rename="EXTERNAL_FILE_REFERENCED")]
    EXTERNALFILEREFERENCED,
    
    /// "MRAID_REFERENCED"
    #[serde(rename="MRAID_REFERENCED")]
    MRAIDREFERENCED,
    
    /// "ADMOB_REFERENCED"
    #[serde(rename="ADMOB_REFERENCED")]
    ADMOBREFERENCED,
    
    /// "FILE_TYPE_INVALID"
    #[serde(rename="FILE_TYPE_INVALID")]
    FILETYPEINVALID,
    
    /// "ZIP_INVALID"
    #[serde(rename="ZIP_INVALID")]
    ZIPINVALID,
    
    /// "LINKED_FILE_NOT_FOUND"
    #[serde(rename="LINKED_FILE_NOT_FOUND")]
    LINKEDFILENOTFOUND,
    
    /// "MAX_FLASH_VERSION_11"
    #[serde(rename="MAX_FLASH_VERSION_11")]
    MAXFLASHVERSION11,
    
    /// "NOT_SSL_COMPLIANT"
    #[serde(rename="NOT_SSL_COMPLIANT")]
    NOTSSLCOMPLIANT,
    
    /// "FILE_DETAIL_EMPTY"
    #[serde(rename="FILE_DETAIL_EMPTY")]
    FILEDETAILEMPTY,
    
    /// "ASSET_INVALID"
    #[serde(rename="ASSET_INVALID")]
    ASSETINVALID,
    
    /// "GWD_PROPERTIES_INVALID"
    #[serde(rename="GWD_PROPERTIES_INVALID")]
    GWDPROPERTIESINVALID,
    
    /// "ENABLER_UNSUPPORTED_METHOD_DCM"
    #[serde(rename="ENABLER_UNSUPPORTED_METHOD_DCM")]
    ENABLERUNSUPPORTEDMETHODDCM,
    
    /// "ASSET_FORMAT_UNSUPPORTED_DCM"
    #[serde(rename="ASSET_FORMAT_UNSUPPORTED_DCM")]
    ASSETFORMATUNSUPPORTEDDCM,
    
    /// "COMPONENT_UNSUPPORTED_DCM"
    #[serde(rename="COMPONENT_UNSUPPORTED_DCM")]
    COMPONENTUNSUPPORTEDDCM,
    
    /// "HTML5_FEATURE_UNSUPPORTED"
    #[serde(rename="HTML5_FEATURE_UNSUPPORTED")]
    HTML5FEATUREUNSUPPORTED,
    
    /// "CLICK_TAG_IN_GWD"
    #[serde(rename="CLICK_TAG_IN_GWD")]
    CLICKTAGINGWD,
    
    /// "CLICK_TAG_HARD_CODED"
    #[serde(rename="CLICK_TAG_HARD_CODED")]
    CLICKTAGHARDCODED,
    
    /// "SVG_INVALID"
    #[serde(rename="SVG_INVALID")]
    SVGINVALID,
    
    /// "CLICK_TAG_IN_RICH_MEDIA"
    #[serde(rename="CLICK_TAG_IN_RICH_MEDIA")]
    CLICKTAGINRICHMEDIA,
    
    /// "MISSING_ENABLER_REFERENCE"
    #[serde(rename="MISSING_ENABLER_REFERENCE")]
    MISSINGENABLERREFERENCE,
}

impl AsRef<str> for CreativeAssetMetadataWarnedValidationRulesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGNONTOPLEVEL => "CLICK_TAG_NON_TOP_LEVEL",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGMISSING => "CLICK_TAG_MISSING",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGMORETHANONE => "CLICK_TAG_MORE_THAN_ONE",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINVALID => "CLICK_TAG_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::ORPHANEDASSET => "ORPHANED_ASSET",
            CreativeAssetMetadataWarnedValidationRulesEnum::PRIMARYHTMLMISSING => "PRIMARY_HTML_MISSING",
            CreativeAssetMetadataWarnedValidationRulesEnum::EXTERNALFILEREFERENCED => "EXTERNAL_FILE_REFERENCED",
            CreativeAssetMetadataWarnedValidationRulesEnum::MRAIDREFERENCED => "MRAID_REFERENCED",
            CreativeAssetMetadataWarnedValidationRulesEnum::ADMOBREFERENCED => "ADMOB_REFERENCED",
            CreativeAssetMetadataWarnedValidationRulesEnum::FILETYPEINVALID => "FILE_TYPE_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::ZIPINVALID => "ZIP_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::LINKEDFILENOTFOUND => "LINKED_FILE_NOT_FOUND",
            CreativeAssetMetadataWarnedValidationRulesEnum::MAXFLASHVERSION11 => "MAX_FLASH_VERSION_11",
            CreativeAssetMetadataWarnedValidationRulesEnum::NOTSSLCOMPLIANT => "NOT_SSL_COMPLIANT",
            CreativeAssetMetadataWarnedValidationRulesEnum::FILEDETAILEMPTY => "FILE_DETAIL_EMPTY",
            CreativeAssetMetadataWarnedValidationRulesEnum::ASSETINVALID => "ASSET_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::GWDPROPERTIESINVALID => "GWD_PROPERTIES_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::ENABLERUNSUPPORTEDMETHODDCM => "ENABLER_UNSUPPORTED_METHOD_DCM",
            CreativeAssetMetadataWarnedValidationRulesEnum::ASSETFORMATUNSUPPORTEDDCM => "ASSET_FORMAT_UNSUPPORTED_DCM",
            CreativeAssetMetadataWarnedValidationRulesEnum::COMPONENTUNSUPPORTEDDCM => "COMPONENT_UNSUPPORTED_DCM",
            CreativeAssetMetadataWarnedValidationRulesEnum::HTML5FEATUREUNSUPPORTED => "HTML5_FEATURE_UNSUPPORTED",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINGWD => "CLICK_TAG_IN_GWD",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGHARDCODED => "CLICK_TAG_HARD_CODED",
            CreativeAssetMetadataWarnedValidationRulesEnum::SVGINVALID => "SVG_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINRICHMEDIA => "CLICK_TAG_IN_RICH_MEDIA",
            CreativeAssetMetadataWarnedValidationRulesEnum::MISSINGENABLERREFERENCE => "MISSING_ENABLER_REFERENCE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetMetadataWarnedValidationRulesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLICK_TAG_NON_TOP_LEVEL" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGNONTOPLEVEL),
           "CLICK_TAG_MISSING" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGMISSING),
           "CLICK_TAG_MORE_THAN_ONE" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGMORETHANONE),
           "CLICK_TAG_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINVALID),
           "ORPHANED_ASSET" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ORPHANEDASSET),
           "PRIMARY_HTML_MISSING" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::PRIMARYHTMLMISSING),
           "EXTERNAL_FILE_REFERENCED" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::EXTERNALFILEREFERENCED),
           "MRAID_REFERENCED" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::MRAIDREFERENCED),
           "ADMOB_REFERENCED" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ADMOBREFERENCED),
           "FILE_TYPE_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::FILETYPEINVALID),
           "ZIP_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ZIPINVALID),
           "LINKED_FILE_NOT_FOUND" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::LINKEDFILENOTFOUND),
           "MAX_FLASH_VERSION_11" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::MAXFLASHVERSION11),
           "NOT_SSL_COMPLIANT" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::NOTSSLCOMPLIANT),
           "FILE_DETAIL_EMPTY" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::FILEDETAILEMPTY),
           "ASSET_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ASSETINVALID),
           "GWD_PROPERTIES_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::GWDPROPERTIESINVALID),
           "ENABLER_UNSUPPORTED_METHOD_DCM" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ENABLERUNSUPPORTEDMETHODDCM),
           "ASSET_FORMAT_UNSUPPORTED_DCM" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ASSETFORMATUNSUPPORTEDDCM),
           "COMPONENT_UNSUPPORTED_DCM" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::COMPONENTUNSUPPORTEDDCM),
           "HTML5_FEATURE_UNSUPPORTED" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::HTML5FEATUREUNSUPPORTED),
           "CLICK_TAG_IN_GWD" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINGWD),
           "CLICK_TAG_HARD_CODED" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGHARDCODED),
           "SVG_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::SVGINVALID),
           "CLICK_TAG_IN_RICH_MEDIA" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINRICHMEDIA),
           "MISSING_ENABLER_REFERENCE" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::MISSINGENABLERREFERENCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetMetadataWarnedValidationRulesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeCustomEventAdvertiserCustomEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the event. This is a read-only field.
pub enum CreativeCustomEventAdvertiserCustomEventTypeEnum {
    
    /// "ADVERTISER_EVENT_TIMER"
    #[serde(rename="ADVERTISER_EVENT_TIMER")]
    ADVERTISEREVENTTIMER,
    
    /// "ADVERTISER_EVENT_EXIT"
    #[serde(rename="ADVERTISER_EVENT_EXIT")]
    ADVERTISEREVENTEXIT,
    
    /// "ADVERTISER_EVENT_COUNTER"
    #[serde(rename="ADVERTISER_EVENT_COUNTER")]
    ADVERTISEREVENTCOUNTER,
}

impl AsRef<str> for CreativeCustomEventAdvertiserCustomEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTTIMER => "ADVERTISER_EVENT_TIMER",
            CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTEXIT => "ADVERTISER_EVENT_EXIT",
            CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTCOUNTER => "ADVERTISER_EVENT_COUNTER",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeCustomEventAdvertiserCustomEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADVERTISER_EVENT_TIMER" => Ok(CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTTIMER),
           "ADVERTISER_EVENT_EXIT" => Ok(CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTEXIT),
           "ADVERTISER_EVENT_COUNTER" => Ok(CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTCOUNTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeCustomEventAdvertiserCustomEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeCustomEventArtworkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Artwork type used by the creative.This is a read-only field.
pub enum CreativeCustomEventArtworkTypeEnum {
    
    /// "ARTWORK_TYPE_FLASH"
    #[serde(rename="ARTWORK_TYPE_FLASH")]
    ARTWORKTYPEFLASH,
    
    /// "ARTWORK_TYPE_HTML5"
    #[serde(rename="ARTWORK_TYPE_HTML5")]
    ARTWORKTYPEHTML5,
    
    /// "ARTWORK_TYPE_MIXED"
    #[serde(rename="ARTWORK_TYPE_MIXED")]
    ARTWORKTYPEMIXED,
    
    /// "ARTWORK_TYPE_IMAGE"
    #[serde(rename="ARTWORK_TYPE_IMAGE")]
    ARTWORKTYPEIMAGE,
}

impl AsRef<str> for CreativeCustomEventArtworkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEFLASH => "ARTWORK_TYPE_FLASH",
            CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEHTML5 => "ARTWORK_TYPE_HTML5",
            CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEMIXED => "ARTWORK_TYPE_MIXED",
            CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEIMAGE => "ARTWORK_TYPE_IMAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeCustomEventArtworkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ARTWORK_TYPE_FLASH" => Ok(CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEFLASH),
           "ARTWORK_TYPE_HTML5" => Ok(CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEHTML5),
           "ARTWORK_TYPE_MIXED" => Ok(CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEMIXED),
           "ARTWORK_TYPE_IMAGE" => Ok(CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEIMAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeCustomEventArtworkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeCustomEventTargetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Target type used by the event.
pub enum CreativeCustomEventTargetTypeEnum {
    
    /// "TARGET_BLANK"
    #[serde(rename="TARGET_BLANK")]
    TARGETBLANK,
    
    /// "TARGET_TOP"
    #[serde(rename="TARGET_TOP")]
    TARGETTOP,
    
    /// "TARGET_SELF"
    #[serde(rename="TARGET_SELF")]
    TARGETSELF,
    
    /// "TARGET_PARENT"
    #[serde(rename="TARGET_PARENT")]
    TARGETPARENT,
    
    /// "TARGET_POPUP"
    #[serde(rename="TARGET_POPUP")]
    TARGETPOPUP,
}

impl AsRef<str> for CreativeCustomEventTargetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeCustomEventTargetTypeEnum::TARGETBLANK => "TARGET_BLANK",
            CreativeCustomEventTargetTypeEnum::TARGETTOP => "TARGET_TOP",
            CreativeCustomEventTargetTypeEnum::TARGETSELF => "TARGET_SELF",
            CreativeCustomEventTargetTypeEnum::TARGETPARENT => "TARGET_PARENT",
            CreativeCustomEventTargetTypeEnum::TARGETPOPUP => "TARGET_POPUP",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeCustomEventTargetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGET_BLANK" => Ok(CreativeCustomEventTargetTypeEnum::TARGETBLANK),
           "TARGET_TOP" => Ok(CreativeCustomEventTargetTypeEnum::TARGETTOP),
           "TARGET_SELF" => Ok(CreativeCustomEventTargetTypeEnum::TARGETSELF),
           "TARGET_PARENT" => Ok(CreativeCustomEventTargetTypeEnum::TARGETPARENT),
           "TARGET_POPUP" => Ok(CreativeCustomEventTargetTypeEnum::TARGETPOPUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeCustomEventTargetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DimensionValueMatchTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how the 'value' field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, '*' is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions ('dfa:paidSearch*') allow a matchType other than EXACT.
pub enum DimensionValueMatchTypeEnum {
    
    /// "EXACT"
    #[serde(rename="EXACT")]
    EXACT,
    
    /// "BEGINS_WITH"
    #[serde(rename="BEGINS_WITH")]
    BEGINSWITH,
    
    /// "CONTAINS"
    #[serde(rename="CONTAINS")]
    CONTAINS,
    
    /// "WILDCARD_EXPRESSION"
    #[serde(rename="WILDCARD_EXPRESSION")]
    WILDCARDEXPRESSION,
}

impl AsRef<str> for DimensionValueMatchTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DimensionValueMatchTypeEnum::EXACT => "EXACT",
            DimensionValueMatchTypeEnum::BEGINSWITH => "BEGINS_WITH",
            DimensionValueMatchTypeEnum::CONTAINS => "CONTAINS",
            DimensionValueMatchTypeEnum::WILDCARDEXPRESSION => "WILDCARD_EXPRESSION",
        }
    }
}

impl std::convert::TryFrom< &str> for DimensionValueMatchTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXACT" => Ok(DimensionValueMatchTypeEnum::EXACT),
           "BEGINS_WITH" => Ok(DimensionValueMatchTypeEnum::BEGINSWITH),
           "CONTAINS" => Ok(DimensionValueMatchTypeEnum::CONTAINS),
           "WILDCARD_EXPRESSION" => Ok(DimensionValueMatchTypeEnum::WILDCARDEXPRESSION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DimensionValueMatchTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediaRequestInfoNotificationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of notification received from Scotty.
pub enum MediaRequestInfoNotificationTypeEnum {
    

    /// Such requests signals the start of a request containing media upload. Only the media field(s) in the inserted/updated resource are set. The response should either return an error or succeed. On success, responses don't need to contain anything.
    ///
    /// "START"
    #[serde(rename="START")]
    START,
    

    /// Such requests signals that the upload has progressed and that the backend might want to access the media file specified in relevant fields in the resource. Only the media field(s) in the inserted/updated resource are set. The response should either return an error or succeed. On success, responses don't need to contain anything.
    ///
    /// "PROGRESS"
    #[serde(rename="PROGRESS")]
    PROGRESS,
    

    /// Such requests signals the end of a request containing media upload. END should be handled just like normal Insert/Upload requests, that is, they should process the request and return a complete resource in the response. Pointers to media data (a GFS path usually) appear in the relevant fields in the inserted/updated resource. See gdata.Media in data.proto.
    ///
    /// "END"
    #[serde(rename="END")]
    END,
    

    /// Such requests occur after an END and signal that the response has been sent back to the client. RESPONSE_SENT is only sent to the backend if it is configured to receive them. The response does not need to contain anything.
    ///
    /// "RESPONSE_SENT"
    #[serde(rename="RESPONSE_SENT")]
    RESPONSESENT,
    

    /// Such requests indicate that an error occurred while processing the request. ERROR is only sent to the backend if it is configured to receive them. It is not guaranteed that all errors will result in this notification to the backend, even if the backend requests them. Since these requests are just for informational purposes, the response does not need to contain anything.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for MediaRequestInfoNotificationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediaRequestInfoNotificationTypeEnum::START => "START",
            MediaRequestInfoNotificationTypeEnum::PROGRESS => "PROGRESS",
            MediaRequestInfoNotificationTypeEnum::END => "END",
            MediaRequestInfoNotificationTypeEnum::RESPONSESENT => "RESPONSE_SENT",
            MediaRequestInfoNotificationTypeEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for MediaRequestInfoNotificationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "START" => Ok(MediaRequestInfoNotificationTypeEnum::START),
           "PROGRESS" => Ok(MediaRequestInfoNotificationTypeEnum::PROGRESS),
           "END" => Ok(MediaRequestInfoNotificationTypeEnum::END),
           "RESPONSE_SENT" => Ok(MediaRequestInfoNotificationTypeEnum::RESPONSESENT),
           "ERROR" => Ok(MediaRequestInfoNotificationTypeEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediaRequestInfoNotificationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediaResponseInfoRequestClassEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Request class to use for all Blobstore operations for this request.
pub enum MediaResponseInfoRequestClassEnum {
    

    /// Unpopulated request_class in log files will be taken as 0 in dremel query. GoogleSQL will try to cast it to enum by default. An unused 0 value is added to avoid GoogleSQL casting error. Please refer to b/69677280.
    ///
    /// "UNKNOWN_REQUEST_CLASS"
    #[serde(rename="UNKNOWN_REQUEST_CLASS")]
    UNKNOWNREQUESTCLASS,
    

    /// A latency-sensitive request.
    ///
    /// "LATENCY_SENSITIVE"
    #[serde(rename="LATENCY_SENSITIVE")]
    LATENCYSENSITIVE,
    

    /// A request generated by a batch process.
    ///
    /// "PRODUCTION_BATCH"
    #[serde(rename="PRODUCTION_BATCH")]
    PRODUCTIONBATCH,
    

    /// A best-effort request.
    ///
    /// "BEST_EFFORT"
    #[serde(rename="BEST_EFFORT")]
    BESTEFFORT,
}

impl AsRef<str> for MediaResponseInfoRequestClassEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediaResponseInfoRequestClassEnum::UNKNOWNREQUESTCLASS => "UNKNOWN_REQUEST_CLASS",
            MediaResponseInfoRequestClassEnum::LATENCYSENSITIVE => "LATENCY_SENSITIVE",
            MediaResponseInfoRequestClassEnum::PRODUCTIONBATCH => "PRODUCTION_BATCH",
            MediaResponseInfoRequestClassEnum::BESTEFFORT => "BEST_EFFORT",
        }
    }
}

impl std::convert::TryFrom< &str> for MediaResponseInfoRequestClassEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_REQUEST_CLASS" => Ok(MediaResponseInfoRequestClassEnum::UNKNOWNREQUESTCLASS),
           "LATENCY_SENSITIVE" => Ok(MediaResponseInfoRequestClassEnum::LATENCYSENSITIVE),
           "PRODUCTION_BATCH" => Ok(MediaResponseInfoRequestClassEnum::PRODUCTIONBATCH),
           "BEST_EFFORT" => Ok(MediaResponseInfoRequestClassEnum::BESTEFFORT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediaResponseInfoRequestClassEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediaResponseInfoTrafficClassFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the TrafficClass that Scotty should use for any RPCs to fetch the response bytes. Will override the traffic class GTOS of the incoming http request. This is a temporary field to facilitate whitelisting and experimentation by the bigstore agent only. For instance, this does not apply to RTMP reads. WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.
pub enum MediaResponseInfoTrafficClassFieldEnum {
    

    /// Application-selectable traffic classes Best effort
    ///
    /// "BE1"
    #[serde(rename="BE1")]
    BE1,
    

    /// Assured forwarding priority 1
    ///
    /// "AF1"
    #[serde(rename="AF1")]
    AF1,
    

    /// Assured forwarding priority 2
    ///
    /// "AF2"
    #[serde(rename="AF2")]
    AF2,
    

    /// Assured forwarding priority 3
    ///
    /// "AF3"
    #[serde(rename="AF3")]
    AF3,
    

    /// Assured forwarding priority 4
    ///
    /// "AF4"
    #[serde(rename="AF4")]
    AF4,
    

    /// Network control
    ///
    /// "NC1"
    #[serde(rename="NC1")]
    NC1,
    

    /// Network control
    ///
    /// "NC0"
    #[serde(rename="NC0")]
    NC0,
    

    /// Best effort at high packet loss
    ///
    /// "BE0"
    #[serde(rename="BE0")]
    BE0,
    

    /// Low-latency queue (LLQ) best effort (go/llq)
    ///
    /// "LLQ"
    #[serde(rename="LLQ")]
    LLQ,
    

    /// LLQ best effort (go/llq2)
    ///
    /// "LLQ1"
    #[serde(rename="LLQ1")]
    LLQ1,
    

    /// LLQ assured forwarding priority 2 (go/llq2)
    ///
    /// "LLQ2"
    #[serde(rename="LLQ2")]
    LLQ2,
}

impl AsRef<str> for MediaResponseInfoTrafficClassFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediaResponseInfoTrafficClassFieldEnum::BE1 => "BE1",
            MediaResponseInfoTrafficClassFieldEnum::AF1 => "AF1",
            MediaResponseInfoTrafficClassFieldEnum::AF2 => "AF2",
            MediaResponseInfoTrafficClassFieldEnum::AF3 => "AF3",
            MediaResponseInfoTrafficClassFieldEnum::AF4 => "AF4",
            MediaResponseInfoTrafficClassFieldEnum::NC1 => "NC1",
            MediaResponseInfoTrafficClassFieldEnum::NC0 => "NC0",
            MediaResponseInfoTrafficClassFieldEnum::BE0 => "BE0",
            MediaResponseInfoTrafficClassFieldEnum::LLQ => "LLQ",
            MediaResponseInfoTrafficClassFieldEnum::LLQ1 => "LLQ1",
            MediaResponseInfoTrafficClassFieldEnum::LLQ2 => "LLQ2",
        }
    }
}

impl std::convert::TryFrom< &str> for MediaResponseInfoTrafficClassFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BE1" => Ok(MediaResponseInfoTrafficClassFieldEnum::BE1),
           "AF1" => Ok(MediaResponseInfoTrafficClassFieldEnum::AF1),
           "AF2" => Ok(MediaResponseInfoTrafficClassFieldEnum::AF2),
           "AF3" => Ok(MediaResponseInfoTrafficClassFieldEnum::AF3),
           "AF4" => Ok(MediaResponseInfoTrafficClassFieldEnum::AF4),
           "NC1" => Ok(MediaResponseInfoTrafficClassFieldEnum::NC1),
           "NC0" => Ok(MediaResponseInfoTrafficClassFieldEnum::NC0),
           "BE0" => Ok(MediaResponseInfoTrafficClassFieldEnum::BE0),
           "LLQ" => Ok(MediaResponseInfoTrafficClassFieldEnum::LLQ),
           "LLQ1" => Ok(MediaResponseInfoTrafficClassFieldEnum::LLQ1),
           "LLQ2" => Ok(MediaResponseInfoTrafficClassFieldEnum::LLQ2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediaResponseInfoTrafficClassFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PopupWindowPropertyPositionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Popup window position either centered or at specific coordinate.
pub enum PopupWindowPropertyPositionTypeEnum {
    
    /// "CENTER"
    #[serde(rename="CENTER")]
    CENTER,
    
    /// "COORDINATES"
    #[serde(rename="COORDINATES")]
    COORDINATES,
}

impl AsRef<str> for PopupWindowPropertyPositionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PopupWindowPropertyPositionTypeEnum::CENTER => "CENTER",
            PopupWindowPropertyPositionTypeEnum::COORDINATES => "COORDINATES",
        }
    }
}

impl std::convert::TryFrom< &str> for PopupWindowPropertyPositionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CENTER" => Ok(PopupWindowPropertyPositionTypeEnum::CENTER),
           "COORDINATES" => Ok(PopupWindowPropertyPositionTypeEnum::COORDINATES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PopupWindowPropertyPositionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


