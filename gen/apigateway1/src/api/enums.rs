use super::*;



// region ApigatewayApiStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the API.
pub enum ApigatewayApiStateEnum {
    

    /// API does not have a state yet.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// API is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// API is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// API creation failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// API is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// API is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for ApigatewayApiStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApigatewayApiStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ApigatewayApiStateEnum::CREATING => "CREATING",
            ApigatewayApiStateEnum::ACTIVE => "ACTIVE",
            ApigatewayApiStateEnum::FAILED => "FAILED",
            ApigatewayApiStateEnum::DELETING => "DELETING",
            ApigatewayApiStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for ApigatewayApiStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ApigatewayApiStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(ApigatewayApiStateEnum::CREATING),
           "ACTIVE" => Ok(ApigatewayApiStateEnum::ACTIVE),
           "FAILED" => Ok(ApigatewayApiStateEnum::FAILED),
           "DELETING" => Ok(ApigatewayApiStateEnum::DELETING),
           "UPDATING" => Ok(ApigatewayApiStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApigatewayApiStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApigatewayApiConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the API Config.
pub enum ApigatewayApiConfigStateEnum {
    

    /// API Config does not have a state yet.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// API Config is being created and deployed to the API Controller.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// API Config is ready for use by Gateways.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// API Config creation failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// API Config is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// API Config is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// API Config settings are being activated in downstream systems. API Configs in this state cannot be used by Gateways.
    ///
    /// "ACTIVATING"
    #[serde(rename="ACTIVATING")]
    ACTIVATING,
}

impl AsRef<str> for ApigatewayApiConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApigatewayApiConfigStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ApigatewayApiConfigStateEnum::CREATING => "CREATING",
            ApigatewayApiConfigStateEnum::ACTIVE => "ACTIVE",
            ApigatewayApiConfigStateEnum::FAILED => "FAILED",
            ApigatewayApiConfigStateEnum::DELETING => "DELETING",
            ApigatewayApiConfigStateEnum::UPDATING => "UPDATING",
            ApigatewayApiConfigStateEnum::ACTIVATING => "ACTIVATING",
        }
    }
}

impl std::convert::TryFrom< &str> for ApigatewayApiConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ApigatewayApiConfigStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(ApigatewayApiConfigStateEnum::CREATING),
           "ACTIVE" => Ok(ApigatewayApiConfigStateEnum::ACTIVE),
           "FAILED" => Ok(ApigatewayApiConfigStateEnum::FAILED),
           "DELETING" => Ok(ApigatewayApiConfigStateEnum::DELETING),
           "UPDATING" => Ok(ApigatewayApiConfigStateEnum::UPDATING),
           "ACTIVATING" => Ok(ApigatewayApiConfigStateEnum::ACTIVATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApigatewayApiConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApigatewayAuditLogConfigLogTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The log type that this config enables.
pub enum ApigatewayAuditLogConfigLogTypeEnum {
    

    /// Default case. Should never be this.
    ///
    /// "LOG_TYPE_UNSPECIFIED"
    #[serde(rename="LOG_TYPE_UNSPECIFIED")]
    LOGTYPEUNSPECIFIED,
    

    /// Admin reads. Example: CloudIAM getIamPolicy
    ///
    /// "ADMIN_READ"
    #[serde(rename="ADMIN_READ")]
    ADMINREAD,
    

    /// Data writes. Example: CloudSQL Users create
    ///
    /// "DATA_WRITE"
    #[serde(rename="DATA_WRITE")]
    DATAWRITE,
    

    /// Data reads. Example: CloudSQL Users list
    ///
    /// "DATA_READ"
    #[serde(rename="DATA_READ")]
    DATAREAD,
}

impl AsRef<str> for ApigatewayAuditLogConfigLogTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApigatewayAuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            ApigatewayAuditLogConfigLogTypeEnum::ADMINREAD => "ADMIN_READ",
            ApigatewayAuditLogConfigLogTypeEnum::DATAWRITE => "DATA_WRITE",
            ApigatewayAuditLogConfigLogTypeEnum::DATAREAD => "DATA_READ",
        }
    }
}

impl std::convert::TryFrom< &str> for ApigatewayAuditLogConfigLogTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(ApigatewayAuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED),
           "ADMIN_READ" => Ok(ApigatewayAuditLogConfigLogTypeEnum::ADMINREAD),
           "DATA_WRITE" => Ok(ApigatewayAuditLogConfigLogTypeEnum::DATAWRITE),
           "DATA_READ" => Ok(ApigatewayAuditLogConfigLogTypeEnum::DATAREAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApigatewayAuditLogConfigLogTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApigatewayGatewayStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the Gateway.
pub enum ApigatewayGatewayStateEnum {
    

    /// Gateway does not have a state yet.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Gateway is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Gateway is running and ready for requests.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Gateway creation failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Gateway is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Gateway is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for ApigatewayGatewayStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApigatewayGatewayStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ApigatewayGatewayStateEnum::CREATING => "CREATING",
            ApigatewayGatewayStateEnum::ACTIVE => "ACTIVE",
            ApigatewayGatewayStateEnum::FAILED => "FAILED",
            ApigatewayGatewayStateEnum::DELETING => "DELETING",
            ApigatewayGatewayStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for ApigatewayGatewayStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ApigatewayGatewayStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(ApigatewayGatewayStateEnum::CREATING),
           "ACTIVE" => Ok(ApigatewayGatewayStateEnum::ACTIVE),
           "FAILED" => Ok(ApigatewayGatewayStateEnum::FAILED),
           "DELETING" => Ok(ApigatewayGatewayStateEnum::DELETING),
           "UPDATING" => Ok(ApigatewayGatewayStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApigatewayGatewayStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies which fields of the API Config are returned in the response. Defaults to `BASIC` view.
pub enum ProjectViewEnum {
    
    /// "CONFIG_VIEW_UNSPECIFIED"
    #[serde(rename="CONFIG_VIEW_UNSPECIFIED")]
    CONFIGVIEWUNSPECIFIED,
    

    /// Do not include configuration source files.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Include configuration source files.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::CONFIGVIEWUNSPECIFIED => "CONFIG_VIEW_UNSPECIFIED",
            ProjectViewEnum::BASIC => "BASIC",
            ProjectViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONFIG_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::CONFIGVIEWUNSPECIFIED),
           "BASIC" => Ok(ProjectViewEnum::BASIC),
           "FULL" => Ok(ProjectViewEnum::FULL),
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


