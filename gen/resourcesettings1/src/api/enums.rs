use super::*;



// region GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The data type for this setting.
pub enum GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum {
    

    /// Unspecified data type.
    ///
    /// "DATA_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_TYPE_UNSPECIFIED")]
    DATATYPEUNSPECIFIED,
    

    /// A boolean setting.
    ///
    /// "BOOLEAN"
    #[serde(rename="BOOLEAN")]
    BOOLEAN,
    

    /// A string setting.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// A string set setting.
    ///
    /// "STRING_SET"
    #[serde(rename="STRING_SET")]
    STRINGSET,
    

    /// A Enum setting
    ///
    /// "ENUM_VALUE"
    #[serde(rename="ENUM_VALUE")]
    ENUMVALUE,
    

    /// A Duration setting
    ///
    /// "DURATION_VALUE"
    #[serde(rename="DURATION_VALUE")]
    DURATIONVALUE,
    

    /// A string->string map setting
    ///
    /// "STRING_MAP"
    #[serde(rename="STRING_MAP")]
    STRINGMAP,
}

impl AsRef<str> for GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::DATATYPEUNSPECIFIED => "DATA_TYPE_UNSPECIFIED",
            GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::BOOLEAN => "BOOLEAN",
            GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::STRING => "STRING",
            GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::STRINGSET => "STRING_SET",
            GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::ENUMVALUE => "ENUM_VALUE",
            GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::DURATIONVALUE => "DURATION_VALUE",
            GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::STRINGMAP => "STRING_MAP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_TYPE_UNSPECIFIED" => Ok(GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::DATATYPEUNSPECIFIED),
           "BOOLEAN" => Ok(GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::BOOLEAN),
           "STRING" => Ok(GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::STRING),
           "STRING_SET" => Ok(GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::STRINGSET),
           "ENUM_VALUE" => Ok(GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::ENUMVALUE),
           "DURATION_VALUE" => Ok(GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::DURATIONVALUE),
           "STRING_MAP" => Ok(GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum::STRINGMAP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FolderViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The SettingView for this request.
pub enum FolderViewEnum {
    

    /// The default / unset value. The API will default to the SETTING_VIEW_BASIC view.
    ///
    /// "SETTING_VIEW_UNSPECIFIED"
    #[serde(rename="SETTING_VIEW_UNSPECIFIED")]
    SETTINGVIEWUNSPECIFIED,
    

    /// Include Setting.metadata, but nothing else. This is the default value (for both ListSettings and GetSetting).
    ///
    /// "SETTING_VIEW_BASIC"
    #[serde(rename="SETTING_VIEW_BASIC")]
    SETTINGVIEWBASIC,
    

    /// Include Setting.effective_value, but nothing else.
    ///
    /// "SETTING_VIEW_EFFECTIVE_VALUE"
    #[serde(rename="SETTING_VIEW_EFFECTIVE_VALUE")]
    SETTINGVIEWEFFECTIVEVALUE,
    

    /// Include Setting.local_value, but nothing else.
    ///
    /// "SETTING_VIEW_LOCAL_VALUE"
    #[serde(rename="SETTING_VIEW_LOCAL_VALUE")]
    SETTINGVIEWLOCALVALUE,
}

impl AsRef<str> for FolderViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FolderViewEnum::SETTINGVIEWUNSPECIFIED => "SETTING_VIEW_UNSPECIFIED",
            FolderViewEnum::SETTINGVIEWBASIC => "SETTING_VIEW_BASIC",
            FolderViewEnum::SETTINGVIEWEFFECTIVEVALUE => "SETTING_VIEW_EFFECTIVE_VALUE",
            FolderViewEnum::SETTINGVIEWLOCALVALUE => "SETTING_VIEW_LOCAL_VALUE",
        }
    }
}

impl std::convert::TryFrom< &str> for FolderViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SETTING_VIEW_UNSPECIFIED" => Ok(FolderViewEnum::SETTINGVIEWUNSPECIFIED),
           "SETTING_VIEW_BASIC" => Ok(FolderViewEnum::SETTINGVIEWBASIC),
           "SETTING_VIEW_EFFECTIVE_VALUE" => Ok(FolderViewEnum::SETTINGVIEWEFFECTIVEVALUE),
           "SETTING_VIEW_LOCAL_VALUE" => Ok(FolderViewEnum::SETTINGVIEWLOCALVALUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FolderViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The SettingView for this request.
pub enum OrganizationViewEnum {
    

    /// The default / unset value. The API will default to the SETTING_VIEW_BASIC view.
    ///
    /// "SETTING_VIEW_UNSPECIFIED"
    #[serde(rename="SETTING_VIEW_UNSPECIFIED")]
    SETTINGVIEWUNSPECIFIED,
    

    /// Include Setting.metadata, but nothing else. This is the default value (for both ListSettings and GetSetting).
    ///
    /// "SETTING_VIEW_BASIC"
    #[serde(rename="SETTING_VIEW_BASIC")]
    SETTINGVIEWBASIC,
    

    /// Include Setting.effective_value, but nothing else.
    ///
    /// "SETTING_VIEW_EFFECTIVE_VALUE"
    #[serde(rename="SETTING_VIEW_EFFECTIVE_VALUE")]
    SETTINGVIEWEFFECTIVEVALUE,
    

    /// Include Setting.local_value, but nothing else.
    ///
    /// "SETTING_VIEW_LOCAL_VALUE"
    #[serde(rename="SETTING_VIEW_LOCAL_VALUE")]
    SETTINGVIEWLOCALVALUE,
}

impl AsRef<str> for OrganizationViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationViewEnum::SETTINGVIEWUNSPECIFIED => "SETTING_VIEW_UNSPECIFIED",
            OrganizationViewEnum::SETTINGVIEWBASIC => "SETTING_VIEW_BASIC",
            OrganizationViewEnum::SETTINGVIEWEFFECTIVEVALUE => "SETTING_VIEW_EFFECTIVE_VALUE",
            OrganizationViewEnum::SETTINGVIEWLOCALVALUE => "SETTING_VIEW_LOCAL_VALUE",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SETTING_VIEW_UNSPECIFIED" => Ok(OrganizationViewEnum::SETTINGVIEWUNSPECIFIED),
           "SETTING_VIEW_BASIC" => Ok(OrganizationViewEnum::SETTINGVIEWBASIC),
           "SETTING_VIEW_EFFECTIVE_VALUE" => Ok(OrganizationViewEnum::SETTINGVIEWEFFECTIVEVALUE),
           "SETTING_VIEW_LOCAL_VALUE" => Ok(OrganizationViewEnum::SETTINGVIEWLOCALVALUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The SettingView for this request.
pub enum ProjectViewEnum {
    

    /// The default / unset value. The API will default to the SETTING_VIEW_BASIC view.
    ///
    /// "SETTING_VIEW_UNSPECIFIED"
    #[serde(rename="SETTING_VIEW_UNSPECIFIED")]
    SETTINGVIEWUNSPECIFIED,
    

    /// Include Setting.metadata, but nothing else. This is the default value (for both ListSettings and GetSetting).
    ///
    /// "SETTING_VIEW_BASIC"
    #[serde(rename="SETTING_VIEW_BASIC")]
    SETTINGVIEWBASIC,
    

    /// Include Setting.effective_value, but nothing else.
    ///
    /// "SETTING_VIEW_EFFECTIVE_VALUE"
    #[serde(rename="SETTING_VIEW_EFFECTIVE_VALUE")]
    SETTINGVIEWEFFECTIVEVALUE,
    

    /// Include Setting.local_value, but nothing else.
    ///
    /// "SETTING_VIEW_LOCAL_VALUE"
    #[serde(rename="SETTING_VIEW_LOCAL_VALUE")]
    SETTINGVIEWLOCALVALUE,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::SETTINGVIEWUNSPECIFIED => "SETTING_VIEW_UNSPECIFIED",
            ProjectViewEnum::SETTINGVIEWBASIC => "SETTING_VIEW_BASIC",
            ProjectViewEnum::SETTINGVIEWEFFECTIVEVALUE => "SETTING_VIEW_EFFECTIVE_VALUE",
            ProjectViewEnum::SETTINGVIEWLOCALVALUE => "SETTING_VIEW_LOCAL_VALUE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SETTING_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::SETTINGVIEWUNSPECIFIED),
           "SETTING_VIEW_BASIC" => Ok(ProjectViewEnum::SETTINGVIEWBASIC),
           "SETTING_VIEW_EFFECTIVE_VALUE" => Ok(ProjectViewEnum::SETTINGVIEWEFFECTIVEVALUE),
           "SETTING_VIEW_LOCAL_VALUE" => Ok(ProjectViewEnum::SETTINGVIEWLOCALVALUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


