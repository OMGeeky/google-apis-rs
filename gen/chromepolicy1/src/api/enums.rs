use super::*;



// region GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates current life cycle stage of the policy API.
pub enum GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum {
    

    /// Policy Api Lifecycle is Unspecified.
    ///
    /// "API_UNSPECIFIED"
    #[serde(rename="API_UNSPECIFIED")]
    APIUNSPECIFIED,
    

    /// Policy is not working yet, but giving developers heads up on format. This stage can transfer to API_DEVELOPEMNT or API_CURRENT.
    ///
    /// "API_PREVIEW"
    #[serde(rename="API_PREVIEW")]
    APIPREVIEW,
    

    /// Policy can change format in backward incompatible way (breaking change). This stage can transfer to API_CURRENT or API_DEPRECATED. This could be used for policies launched only to TTs or launched to selected customers for emergency usage.
    ///
    /// "API_DEVELOPMENT"
    #[serde(rename="API_DEVELOPMENT")]
    APIDEVELOPMENT,
    

    /// Policy in official format. Policy can change format in backward compatible way (non-breaking change). Example: this policy can introduce a new field, which is considered non-breaking change, when field masks are properly utilized. This stage can transfer to API_DEPRECATED.
    ///
    /// "API_CURRENT"
    #[serde(rename="API_CURRENT")]
    APICURRENT,
    

    /// Please stop using this policy. This policy is deprecated and may/will be removed in the future. Most likely a new policy was introduced to replace this one.
    ///
    /// "API_DEPRECATED"
    #[serde(rename="API_DEPRECATED")]
    APIDEPRECATED,
}

impl AsRef<str> for GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum::APIUNSPECIFIED => "API_UNSPECIFIED",
            GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum::APIPREVIEW => "API_PREVIEW",
            GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum::APIDEVELOPMENT => "API_DEVELOPMENT",
            GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum::APICURRENT => "API_CURRENT",
            GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum::APIDEPRECATED => "API_DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "API_UNSPECIFIED" => Ok(GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum::APIUNSPECIFIED),
           "API_PREVIEW" => Ok(GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum::APIPREVIEW),
           "API_DEVELOPMENT" => Ok(GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum::APIDEVELOPMENT),
           "API_CURRENT" => Ok(GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum::APICURRENT),
           "API_DEPRECATED" => Ok(GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum::APIDEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromePolicyVersionsV1PolicyApiLifecyclePolicyApiLifecycleStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. List indicates that the policy will only apply to devices/users on these platforms.
pub enum GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum {
    

    /// Unspecified platform.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// ChromeOS.
    ///
    /// "CHROME_OS"
    #[serde(rename="CHROME_OS")]
    CHROMEOS,
    

    /// Chrome Browser for OSX/Windows/Linux.
    ///
    /// "CHROME_BROWSER"
    #[serde(rename="CHROME_BROWSER")]
    CHROMEBROWSER,
    

    /// Chrome Browser for Android.
    ///
    /// "CHROME_BROWSER_FOR_ANDROID"
    #[serde(rename="CHROME_BROWSER_FOR_ANDROID")]
    CHROMEBROWSERFORANDROID,
    

    /// Chrome Browser for iOS.
    ///
    /// "CHROME_BROWSER_FOR_IOS"
    #[serde(rename="CHROME_BROWSER_FOR_IOS")]
    CHROMEBROWSERFORIOS,
}

impl AsRef<str> for GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum::CHROMEOS => "CHROME_OS",
            GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum::CHROMEBROWSER => "CHROME_BROWSER",
            GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum::CHROMEBROWSERFORANDROID => "CHROME_BROWSER_FOR_ANDROID",
            GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum::CHROMEBROWSERFORIOS => "CHROME_BROWSER_FOR_IOS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum::PLATFORMUNSPECIFIED),
           "CHROME_OS" => Ok(GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum::CHROMEOS),
           "CHROME_BROWSER" => Ok(GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum::CHROMEBROWSER),
           "CHROME_BROWSER_FOR_ANDROID" => Ok(GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum::CHROMEBROWSERFORANDROID),
           "CHROME_BROWSER_FOR_IOS" => Ok(GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum::CHROMEBROWSERFORIOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromePolicyVersionsV1PolicySchemaSupportedPlatformsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Information about applicable target resources for the policy.
pub enum GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum {
    

    /// Unspecified target resource.
    ///
    /// "TARGET_RESOURCE_UNSPECIFIED"
    #[serde(rename="TARGET_RESOURCE_UNSPECIFIED")]
    TARGETRESOURCEUNSPECIFIED,
    

    /// Organizational Unit target resource.
    ///
    /// "ORG_UNIT"
    #[serde(rename="ORG_UNIT")]
    ORGUNIT,
    

    /// Group target resource.
    ///
    /// "GROUP"
    #[serde(rename="GROUP")]
    GROUP,
}

impl AsRef<str> for GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum::TARGETRESOURCEUNSPECIFIED => "TARGET_RESOURCE_UNSPECIFIED",
            GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum::ORGUNIT => "ORG_UNIT",
            GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum::GROUP => "GROUP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGET_RESOURCE_UNSPECIFIED" => Ok(GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum::TARGETRESOURCEUNSPECIFIED),
           "ORG_UNIT" => Ok(GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum::ORGUNIT),
           "GROUP" => Ok(GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum::GROUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// File types that can be uploaded for a setting.
pub enum GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum {
    

    /// Unspecified content type.
    ///
    /// "CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_TYPE_UNSPECIFIED")]
    CONTENTTYPEUNSPECIFIED,
    

    /// Plain text.
    ///
    /// "CONTENT_TYPE_PLAIN_TEXT"
    #[serde(rename="CONTENT_TYPE_PLAIN_TEXT")]
    CONTENTTYPEPLAINTEXT,
    

    /// HTML.
    ///
    /// "CONTENT_TYPE_HTML"
    #[serde(rename="CONTENT_TYPE_HTML")]
    CONTENTTYPEHTML,
    

    /// JPEG.
    ///
    /// "CONTENT_TYPE_IMAGE_JPEG"
    #[serde(rename="CONTENT_TYPE_IMAGE_JPEG")]
    CONTENTTYPEIMAGEJPEG,
    

    /// GIF.
    ///
    /// "CONTENT_TYPE_IMAGE_GIF"
    #[serde(rename="CONTENT_TYPE_IMAGE_GIF")]
    CONTENTTYPEIMAGEGIF,
    

    /// PNG.
    ///
    /// "CONTENT_TYPE_IMAGE_PNG"
    #[serde(rename="CONTENT_TYPE_IMAGE_PNG")]
    CONTENTTYPEIMAGEPNG,
    

    /// JSON.
    ///
    /// "CONTENT_TYPE_JSON"
    #[serde(rename="CONTENT_TYPE_JSON")]
    CONTENTTYPEJSON,
    

    /// ZIP.
    ///
    /// "CONTENT_TYPE_ZIP"
    #[serde(rename="CONTENT_TYPE_ZIP")]
    CONTENTTYPEZIP,
    

    /// GZIP.
    ///
    /// "CONTENT_TYPE_GZIP"
    #[serde(rename="CONTENT_TYPE_GZIP")]
    CONTENTTYPEGZIP,
    

    /// CSV.
    ///
    /// "CONTENT_TYPE_CSV"
    #[serde(rename="CONTENT_TYPE_CSV")]
    CONTENTTYPECSV,
    

    /// YAML.
    ///
    /// "CONTENT_TYPE_YAML"
    #[serde(rename="CONTENT_TYPE_YAML")]
    CONTENTTYPEYAML,
    

    /// WEBP.
    ///
    /// "CONTENT_TYPE_IMAGE_WEBP"
    #[serde(rename="CONTENT_TYPE_IMAGE_WEBP")]
    CONTENTTYPEIMAGEWEBP,
}

impl AsRef<str> for GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEUNSPECIFIED => "CONTENT_TYPE_UNSPECIFIED",
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEPLAINTEXT => "CONTENT_TYPE_PLAIN_TEXT",
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEHTML => "CONTENT_TYPE_HTML",
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEIMAGEJPEG => "CONTENT_TYPE_IMAGE_JPEG",
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEIMAGEGIF => "CONTENT_TYPE_IMAGE_GIF",
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEIMAGEPNG => "CONTENT_TYPE_IMAGE_PNG",
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEJSON => "CONTENT_TYPE_JSON",
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEZIP => "CONTENT_TYPE_ZIP",
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEGZIP => "CONTENT_TYPE_GZIP",
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPECSV => "CONTENT_TYPE_CSV",
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEYAML => "CONTENT_TYPE_YAML",
            GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEIMAGEWEBP => "CONTENT_TYPE_IMAGE_WEBP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_TYPE_UNSPECIFIED" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEUNSPECIFIED),
           "CONTENT_TYPE_PLAIN_TEXT" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEPLAINTEXT),
           "CONTENT_TYPE_HTML" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEHTML),
           "CONTENT_TYPE_IMAGE_JPEG" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEIMAGEJPEG),
           "CONTENT_TYPE_IMAGE_GIF" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEIMAGEGIF),
           "CONTENT_TYPE_IMAGE_PNG" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEIMAGEPNG),
           "CONTENT_TYPE_JSON" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEJSON),
           "CONTENT_TYPE_ZIP" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEZIP),
           "CONTENT_TYPE_GZIP" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEGZIP),
           "CONTENT_TYPE_CSV" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPECSV),
           "CONTENT_TYPE_YAML" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEYAML),
           "CONTENT_TYPE_IMAGE_WEBP" => Ok(GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum::CONTENTTYPEIMAGEWEBP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromePolicyVersionsV1UploadedFileConstraintSupportedContentTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region Proto2FieldDescriptorProtoLabelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum Proto2FieldDescriptorProtoLabelEnum {
    

    /// 0 is reserved for errors
    ///
    /// "LABEL_OPTIONAL"
    #[serde(rename="LABEL_OPTIONAL")]
    LABELOPTIONAL,
    
    /// "LABEL_REPEATED"
    #[serde(rename="LABEL_REPEATED")]
    LABELREPEATED,
    

    /// The required label is only allowed in proto2. In proto3 and Editions it's explicitly prohibited. In Editions, the `field_presence` feature can be used to get this behavior.
    ///
    /// "LABEL_REQUIRED"
    #[serde(rename="LABEL_REQUIRED")]
    LABELREQUIRED,
}

impl AsRef<str> for Proto2FieldDescriptorProtoLabelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            Proto2FieldDescriptorProtoLabelEnum::LABELOPTIONAL => "LABEL_OPTIONAL",
            Proto2FieldDescriptorProtoLabelEnum::LABELREPEATED => "LABEL_REPEATED",
            Proto2FieldDescriptorProtoLabelEnum::LABELREQUIRED => "LABEL_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for Proto2FieldDescriptorProtoLabelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LABEL_OPTIONAL" => Ok(Proto2FieldDescriptorProtoLabelEnum::LABELOPTIONAL),
           "LABEL_REPEATED" => Ok(Proto2FieldDescriptorProtoLabelEnum::LABELREPEATED),
           "LABEL_REQUIRED" => Ok(Proto2FieldDescriptorProtoLabelEnum::LABELREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a Proto2FieldDescriptorProtoLabelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region Proto2FieldDescriptorProtoTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If type_name is set, this need not be set. If both this and type_name are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
pub enum Proto2FieldDescriptorProtoTypeEnum {
    

    /// 0 is reserved for errors. Order is weird for historical reasons.
    ///
    /// "TYPE_DOUBLE"
    #[serde(rename="TYPE_DOUBLE")]
    TYPEDOUBLE,
    
    /// "TYPE_FLOAT"
    #[serde(rename="TYPE_FLOAT")]
    TYPEFLOAT,
    

    /// Not ZigZag encoded. Negative numbers take 10 bytes. Use TYPE_SINT64 if negative values are likely.
    ///
    /// "TYPE_INT64"
    #[serde(rename="TYPE_INT64")]
    TYPEINT64,
    
    /// "TYPE_UINT64"
    #[serde(rename="TYPE_UINT64")]
    TYPEUINT64,
    

    /// Not ZigZag encoded. Negative numbers take 10 bytes. Use TYPE_SINT32 if negative values are likely.
    ///
    /// "TYPE_INT32"
    #[serde(rename="TYPE_INT32")]
    TYPEINT32,
    
    /// "TYPE_FIXED64"
    #[serde(rename="TYPE_FIXED64")]
    TYPEFIXED64,
    
    /// "TYPE_FIXED32"
    #[serde(rename="TYPE_FIXED32")]
    TYPEFIXED32,
    
    /// "TYPE_BOOL"
    #[serde(rename="TYPE_BOOL")]
    TYPEBOOL,
    
    /// "TYPE_STRING"
    #[serde(rename="TYPE_STRING")]
    TYPESTRING,
    

    /// Tag-delimited aggregate. Group type is deprecated and not supported after proto2. However, Proto3 implementations should still be able to parse the group wire format and treat group fields as unknown fields. In Editions, the group wire format can be enabled via the `message_encoding` feature.
    ///
    /// "TYPE_GROUP"
    #[serde(rename="TYPE_GROUP")]
    TYPEGROUP,
    

    /// Length-delimited aggregate.
    ///
    /// "TYPE_MESSAGE"
    #[serde(rename="TYPE_MESSAGE")]
    TYPEMESSAGE,
    

    /// New in version 2.
    ///
    /// "TYPE_BYTES"
    #[serde(rename="TYPE_BYTES")]
    TYPEBYTES,
    
    /// "TYPE_UINT32"
    #[serde(rename="TYPE_UINT32")]
    TYPEUINT32,
    
    /// "TYPE_ENUM"
    #[serde(rename="TYPE_ENUM")]
    TYPEENUM,
    
    /// "TYPE_SFIXED32"
    #[serde(rename="TYPE_SFIXED32")]
    TYPESFIXED32,
    
    /// "TYPE_SFIXED64"
    #[serde(rename="TYPE_SFIXED64")]
    TYPESFIXED64,
    

    /// Uses ZigZag encoding.
    ///
    /// "TYPE_SINT32"
    #[serde(rename="TYPE_SINT32")]
    TYPESINT32,
    

    /// Uses ZigZag encoding.
    ///
    /// "TYPE_SINT64"
    #[serde(rename="TYPE_SINT64")]
    TYPESINT64,
}

impl AsRef<str> for Proto2FieldDescriptorProtoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            Proto2FieldDescriptorProtoTypeEnum::TYPEDOUBLE => "TYPE_DOUBLE",
            Proto2FieldDescriptorProtoTypeEnum::TYPEFLOAT => "TYPE_FLOAT",
            Proto2FieldDescriptorProtoTypeEnum::TYPEINT64 => "TYPE_INT64",
            Proto2FieldDescriptorProtoTypeEnum::TYPEUINT64 => "TYPE_UINT64",
            Proto2FieldDescriptorProtoTypeEnum::TYPEINT32 => "TYPE_INT32",
            Proto2FieldDescriptorProtoTypeEnum::TYPEFIXED64 => "TYPE_FIXED64",
            Proto2FieldDescriptorProtoTypeEnum::TYPEFIXED32 => "TYPE_FIXED32",
            Proto2FieldDescriptorProtoTypeEnum::TYPEBOOL => "TYPE_BOOL",
            Proto2FieldDescriptorProtoTypeEnum::TYPESTRING => "TYPE_STRING",
            Proto2FieldDescriptorProtoTypeEnum::TYPEGROUP => "TYPE_GROUP",
            Proto2FieldDescriptorProtoTypeEnum::TYPEMESSAGE => "TYPE_MESSAGE",
            Proto2FieldDescriptorProtoTypeEnum::TYPEBYTES => "TYPE_BYTES",
            Proto2FieldDescriptorProtoTypeEnum::TYPEUINT32 => "TYPE_UINT32",
            Proto2FieldDescriptorProtoTypeEnum::TYPEENUM => "TYPE_ENUM",
            Proto2FieldDescriptorProtoTypeEnum::TYPESFIXED32 => "TYPE_SFIXED32",
            Proto2FieldDescriptorProtoTypeEnum::TYPESFIXED64 => "TYPE_SFIXED64",
            Proto2FieldDescriptorProtoTypeEnum::TYPESINT32 => "TYPE_SINT32",
            Proto2FieldDescriptorProtoTypeEnum::TYPESINT64 => "TYPE_SINT64",
        }
    }
}

impl std::convert::TryFrom< &str> for Proto2FieldDescriptorProtoTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_DOUBLE" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEDOUBLE),
           "TYPE_FLOAT" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEFLOAT),
           "TYPE_INT64" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEINT64),
           "TYPE_UINT64" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEUINT64),
           "TYPE_INT32" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEINT32),
           "TYPE_FIXED64" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEFIXED64),
           "TYPE_FIXED32" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEFIXED32),
           "TYPE_BOOL" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEBOOL),
           "TYPE_STRING" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPESTRING),
           "TYPE_GROUP" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEGROUP),
           "TYPE_MESSAGE" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEMESSAGE),
           "TYPE_BYTES" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEBYTES),
           "TYPE_UINT32" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEUINT32),
           "TYPE_ENUM" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPEENUM),
           "TYPE_SFIXED32" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPESFIXED32),
           "TYPE_SFIXED64" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPESFIXED64),
           "TYPE_SINT32" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPESINT32),
           "TYPE_SINT64" => Ok(Proto2FieldDescriptorProtoTypeEnum::TYPESINT64),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a Proto2FieldDescriptorProtoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


