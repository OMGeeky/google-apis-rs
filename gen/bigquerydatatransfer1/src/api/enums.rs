use super::*;



// region DataSourceAuthorizationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the type of authorization.
pub enum DataSourceAuthorizationTypeEnum {
    

    /// Type unspecified.
    ///
    /// "AUTHORIZATION_TYPE_UNSPECIFIED"
    #[serde(rename="AUTHORIZATION_TYPE_UNSPECIFIED")]
    AUTHORIZATIONTYPEUNSPECIFIED,
    

    /// Use OAuth 2 authorization codes that can be exchanged for a refresh token on the backend.
    ///
    /// "AUTHORIZATION_CODE"
    #[serde(rename="AUTHORIZATION_CODE")]
    AUTHORIZATIONCODE,
    

    /// Return an authorization code for a given Google+ page that can then be exchanged for a refresh token on the backend.
    ///
    /// "GOOGLE_PLUS_AUTHORIZATION_CODE"
    #[serde(rename="GOOGLE_PLUS_AUTHORIZATION_CODE")]
    GOOGLEPLUSAUTHORIZATIONCODE,
    

    /// Use First Party OAuth.
    ///
    /// "FIRST_PARTY_OAUTH"
    #[serde(rename="FIRST_PARTY_OAUTH")]
    FIRSTPARTYOAUTH,
}

impl AsRef<str> for DataSourceAuthorizationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataSourceAuthorizationTypeEnum::AUTHORIZATIONTYPEUNSPECIFIED => "AUTHORIZATION_TYPE_UNSPECIFIED",
            DataSourceAuthorizationTypeEnum::AUTHORIZATIONCODE => "AUTHORIZATION_CODE",
            DataSourceAuthorizationTypeEnum::GOOGLEPLUSAUTHORIZATIONCODE => "GOOGLE_PLUS_AUTHORIZATION_CODE",
            DataSourceAuthorizationTypeEnum::FIRSTPARTYOAUTH => "FIRST_PARTY_OAUTH",
        }
    }
}

impl std::convert::TryFrom< &str> for DataSourceAuthorizationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTHORIZATION_TYPE_UNSPECIFIED" => Ok(DataSourceAuthorizationTypeEnum::AUTHORIZATIONTYPEUNSPECIFIED),
           "AUTHORIZATION_CODE" => Ok(DataSourceAuthorizationTypeEnum::AUTHORIZATIONCODE),
           "GOOGLE_PLUS_AUTHORIZATION_CODE" => Ok(DataSourceAuthorizationTypeEnum::GOOGLEPLUSAUTHORIZATIONCODE),
           "FIRST_PARTY_OAUTH" => Ok(DataSourceAuthorizationTypeEnum::FIRSTPARTYOAUTH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataSourceAuthorizationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataSourceDataRefreshTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether the data source supports automatic data refresh for the past few days, and how it's supported. For some data sources, data might not be complete until a few days later, so it's useful to refresh data automatically.
pub enum DataSourceDataRefreshTypeEnum {
    

    /// The data source won't support data auto refresh, which is default value.
    ///
    /// "DATA_REFRESH_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_REFRESH_TYPE_UNSPECIFIED")]
    DATAREFRESHTYPEUNSPECIFIED,
    

    /// The data source supports data auto refresh, and runs will be scheduled for the past few days. Does not allow custom values to be set for each transfer config.
    ///
    /// "SLIDING_WINDOW"
    #[serde(rename="SLIDING_WINDOW")]
    SLIDINGWINDOW,
    

    /// The data source supports data auto refresh, and runs will be scheduled for the past few days. Allows custom values to be set for each transfer config.
    ///
    /// "CUSTOM_SLIDING_WINDOW"
    #[serde(rename="CUSTOM_SLIDING_WINDOW")]
    CUSTOMSLIDINGWINDOW,
}

impl AsRef<str> for DataSourceDataRefreshTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataSourceDataRefreshTypeEnum::DATAREFRESHTYPEUNSPECIFIED => "DATA_REFRESH_TYPE_UNSPECIFIED",
            DataSourceDataRefreshTypeEnum::SLIDINGWINDOW => "SLIDING_WINDOW",
            DataSourceDataRefreshTypeEnum::CUSTOMSLIDINGWINDOW => "CUSTOM_SLIDING_WINDOW",
        }
    }
}

impl std::convert::TryFrom< &str> for DataSourceDataRefreshTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_REFRESH_TYPE_UNSPECIFIED" => Ok(DataSourceDataRefreshTypeEnum::DATAREFRESHTYPEUNSPECIFIED),
           "SLIDING_WINDOW" => Ok(DataSourceDataRefreshTypeEnum::SLIDINGWINDOW),
           "CUSTOM_SLIDING_WINDOW" => Ok(DataSourceDataRefreshTypeEnum::CUSTOMSLIDINGWINDOW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataSourceDataRefreshTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataSourceTransferTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated. This field has no effect.
pub enum DataSourceTransferTypeEnum {
    

    /// Invalid or Unknown transfer type placeholder.
    ///
    /// "TRANSFER_TYPE_UNSPECIFIED"
    #[serde(rename="TRANSFER_TYPE_UNSPECIFIED")]
    TRANSFERTYPEUNSPECIFIED,
    

    /// Batch data transfer.
    ///
    /// "BATCH"
    #[serde(rename="BATCH")]
    BATCH,
    

    /// Streaming data transfer. Streaming data source currently doesn't support multiple transfer configs per project.
    ///
    /// "STREAMING"
    #[serde(rename="STREAMING")]
    STREAMING,
}

impl AsRef<str> for DataSourceTransferTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataSourceTransferTypeEnum::TRANSFERTYPEUNSPECIFIED => "TRANSFER_TYPE_UNSPECIFIED",
            DataSourceTransferTypeEnum::BATCH => "BATCH",
            DataSourceTransferTypeEnum::STREAMING => "STREAMING",
        }
    }
}

impl std::convert::TryFrom< &str> for DataSourceTransferTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSFER_TYPE_UNSPECIFIED" => Ok(DataSourceTransferTypeEnum::TRANSFERTYPEUNSPECIFIED),
           "BATCH" => Ok(DataSourceTransferTypeEnum::BATCH),
           "STREAMING" => Ok(DataSourceTransferTypeEnum::STREAMING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataSourceTransferTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataSourceParameterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Parameter type.
pub enum DataSourceParameterTypeEnum {
    

    /// Type unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// String parameter.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// Integer parameter (64-bits). Will be serialized to json as string.
    ///
    /// "INTEGER"
    #[serde(rename="INTEGER")]
    INTEGER,
    

    /// Double precision floating point parameter.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
    

    /// Boolean parameter.
    ///
    /// "BOOLEAN"
    #[serde(rename="BOOLEAN")]
    BOOLEAN,
    

    /// Deprecated. This field has no effect.
    ///
    /// "RECORD"
    #[serde(rename="RECORD")]
    RECORD,
    

    /// Page ID for a Google+ Page.
    ///
    /// "PLUS_PAGE"
    #[serde(rename="PLUS_PAGE")]
    PLUSPAGE,
    

    /// List of strings parameter.
    ///
    /// "LIST"
    #[serde(rename="LIST")]
    LIST,
}

impl AsRef<str> for DataSourceParameterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataSourceParameterTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            DataSourceParameterTypeEnum::STRING => "STRING",
            DataSourceParameterTypeEnum::INTEGER => "INTEGER",
            DataSourceParameterTypeEnum::DOUBLE => "DOUBLE",
            DataSourceParameterTypeEnum::BOOLEAN => "BOOLEAN",
            DataSourceParameterTypeEnum::RECORD => "RECORD",
            DataSourceParameterTypeEnum::PLUSPAGE => "PLUS_PAGE",
            DataSourceParameterTypeEnum::LIST => "LIST",
        }
    }
}

impl std::convert::TryFrom< &str> for DataSourceParameterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(DataSourceParameterTypeEnum::TYPEUNSPECIFIED),
           "STRING" => Ok(DataSourceParameterTypeEnum::STRING),
           "INTEGER" => Ok(DataSourceParameterTypeEnum::INTEGER),
           "DOUBLE" => Ok(DataSourceParameterTypeEnum::DOUBLE),
           "BOOLEAN" => Ok(DataSourceParameterTypeEnum::BOOLEAN),
           "RECORD" => Ok(DataSourceParameterTypeEnum::RECORD),
           "PLUS_PAGE" => Ok(DataSourceParameterTypeEnum::PLUSPAGE),
           "LIST" => Ok(DataSourceParameterTypeEnum::LIST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataSourceParameterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransferConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the most recently updated transfer run.
pub enum TransferConfigStateEnum {
    

    /// State placeholder (0).
    ///
    /// "TRANSFER_STATE_UNSPECIFIED"
    #[serde(rename="TRANSFER_STATE_UNSPECIFIED")]
    TRANSFERSTATEUNSPECIFIED,
    

    /// Data transfer is scheduled and is waiting to be picked up by data transfer backend (2).
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Data transfer is in progress (3).
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// Data transfer completed successfully (4).
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// Data transfer failed (5).
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Data transfer is cancelled (6).
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
}

impl AsRef<str> for TransferConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransferConfigStateEnum::TRANSFERSTATEUNSPECIFIED => "TRANSFER_STATE_UNSPECIFIED",
            TransferConfigStateEnum::PENDING => "PENDING",
            TransferConfigStateEnum::RUNNING => "RUNNING",
            TransferConfigStateEnum::SUCCEEDED => "SUCCEEDED",
            TransferConfigStateEnum::FAILED => "FAILED",
            TransferConfigStateEnum::CANCELLED => "CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for TransferConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSFER_STATE_UNSPECIFIED" => Ok(TransferConfigStateEnum::TRANSFERSTATEUNSPECIFIED),
           "PENDING" => Ok(TransferConfigStateEnum::PENDING),
           "RUNNING" => Ok(TransferConfigStateEnum::RUNNING),
           "SUCCEEDED" => Ok(TransferConfigStateEnum::SUCCEEDED),
           "FAILED" => Ok(TransferConfigStateEnum::FAILED),
           "CANCELLED" => Ok(TransferConfigStateEnum::CANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransferConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransferMessageSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Message severity.
pub enum TransferMessageSeverityEnum {
    

    /// No severity specified.
    ///
    /// "MESSAGE_SEVERITY_UNSPECIFIED"
    #[serde(rename="MESSAGE_SEVERITY_UNSPECIFIED")]
    MESSAGESEVERITYUNSPECIFIED,
    

    /// Informational message.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// Warning message.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Error message.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for TransferMessageSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransferMessageSeverityEnum::MESSAGESEVERITYUNSPECIFIED => "MESSAGE_SEVERITY_UNSPECIFIED",
            TransferMessageSeverityEnum::INFO => "INFO",
            TransferMessageSeverityEnum::WARNING => "WARNING",
            TransferMessageSeverityEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for TransferMessageSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MESSAGE_SEVERITY_UNSPECIFIED" => Ok(TransferMessageSeverityEnum::MESSAGESEVERITYUNSPECIFIED),
           "INFO" => Ok(TransferMessageSeverityEnum::INFO),
           "WARNING" => Ok(TransferMessageSeverityEnum::WARNING),
           "ERROR" => Ok(TransferMessageSeverityEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransferMessageSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransferRunStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Data transfer run state. Ignored for input requests.
pub enum TransferRunStateEnum {
    

    /// State placeholder (0).
    ///
    /// "TRANSFER_STATE_UNSPECIFIED"
    #[serde(rename="TRANSFER_STATE_UNSPECIFIED")]
    TRANSFERSTATEUNSPECIFIED,
    

    /// Data transfer is scheduled and is waiting to be picked up by data transfer backend (2).
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Data transfer is in progress (3).
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// Data transfer completed successfully (4).
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// Data transfer failed (5).
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Data transfer is cancelled (6).
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
}

impl AsRef<str> for TransferRunStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransferRunStateEnum::TRANSFERSTATEUNSPECIFIED => "TRANSFER_STATE_UNSPECIFIED",
            TransferRunStateEnum::PENDING => "PENDING",
            TransferRunStateEnum::RUNNING => "RUNNING",
            TransferRunStateEnum::SUCCEEDED => "SUCCEEDED",
            TransferRunStateEnum::FAILED => "FAILED",
            TransferRunStateEnum::CANCELLED => "CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for TransferRunStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSFER_STATE_UNSPECIFIED" => Ok(TransferRunStateEnum::TRANSFERSTATEUNSPECIFIED),
           "PENDING" => Ok(TransferRunStateEnum::PENDING),
           "RUNNING" => Ok(TransferRunStateEnum::RUNNING),
           "SUCCEEDED" => Ok(TransferRunStateEnum::SUCCEEDED),
           "FAILED" => Ok(TransferRunStateEnum::FAILED),
           "CANCELLED" => Ok(TransferRunStateEnum::CANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransferRunStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectMessageTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Message types to return. If not populated - INFO, WARNING and ERROR messages are returned.
pub enum ProjectMessageTypesEnum {
    

    /// No severity specified.
    ///
    /// "MESSAGE_SEVERITY_UNSPECIFIED"
    #[serde(rename="MESSAGE_SEVERITY_UNSPECIFIED")]
    MESSAGESEVERITYUNSPECIFIED,
    

    /// Informational message.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// Warning message.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Error message.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for ProjectMessageTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectMessageTypesEnum::MESSAGESEVERITYUNSPECIFIED => "MESSAGE_SEVERITY_UNSPECIFIED",
            ProjectMessageTypesEnum::INFO => "INFO",
            ProjectMessageTypesEnum::WARNING => "WARNING",
            ProjectMessageTypesEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectMessageTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MESSAGE_SEVERITY_UNSPECIFIED" => Ok(ProjectMessageTypesEnum::MESSAGESEVERITYUNSPECIFIED),
           "INFO" => Ok(ProjectMessageTypesEnum::INFO),
           "WARNING" => Ok(ProjectMessageTypesEnum::WARNING),
           "ERROR" => Ok(ProjectMessageTypesEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectMessageTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectRunAttemptEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates how run attempts are to be pulled.
pub enum ProjectRunAttemptEnum {
    

    /// All runs should be returned.
    ///
    /// "RUN_ATTEMPT_UNSPECIFIED"
    #[serde(rename="RUN_ATTEMPT_UNSPECIFIED")]
    RUNATTEMPTUNSPECIFIED,
    

    /// Only latest run per day should be returned.
    ///
    /// "LATEST"
    #[serde(rename="LATEST")]
    LATEST,
}

impl AsRef<str> for ProjectRunAttemptEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectRunAttemptEnum::RUNATTEMPTUNSPECIFIED => "RUN_ATTEMPT_UNSPECIFIED",
            ProjectRunAttemptEnum::LATEST => "LATEST",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectRunAttemptEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RUN_ATTEMPT_UNSPECIFIED" => Ok(ProjectRunAttemptEnum::RUNATTEMPTUNSPECIFIED),
           "LATEST" => Ok(ProjectRunAttemptEnum::LATEST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectRunAttemptEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectStatesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// When specified, only transfer runs with requested states are returned.
pub enum ProjectStatesEnum {
    

    /// State placeholder (0).
    ///
    /// "TRANSFER_STATE_UNSPECIFIED"
    #[serde(rename="TRANSFER_STATE_UNSPECIFIED")]
    TRANSFERSTATEUNSPECIFIED,
    

    /// Data transfer is scheduled and is waiting to be picked up by data transfer backend (2).
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Data transfer is in progress (3).
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// Data transfer completed successfully (4).
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// Data transfer failed (5).
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Data transfer is cancelled (6).
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
}

impl AsRef<str> for ProjectStatesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectStatesEnum::TRANSFERSTATEUNSPECIFIED => "TRANSFER_STATE_UNSPECIFIED",
            ProjectStatesEnum::PENDING => "PENDING",
            ProjectStatesEnum::RUNNING => "RUNNING",
            ProjectStatesEnum::SUCCEEDED => "SUCCEEDED",
            ProjectStatesEnum::FAILED => "FAILED",
            ProjectStatesEnum::CANCELLED => "CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectStatesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSFER_STATE_UNSPECIFIED" => Ok(ProjectStatesEnum::TRANSFERSTATEUNSPECIFIED),
           "PENDING" => Ok(ProjectStatesEnum::PENDING),
           "RUNNING" => Ok(ProjectStatesEnum::RUNNING),
           "SUCCEEDED" => Ok(ProjectStatesEnum::SUCCEEDED),
           "FAILED" => Ok(ProjectStatesEnum::FAILED),
           "CANCELLED" => Ok(ProjectStatesEnum::CANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectStatesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


