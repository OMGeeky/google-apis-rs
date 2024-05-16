use super::*;



// region AuditLogConfigLogTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The log type that this config enables.
pub enum AuditLogConfigLogTypeEnum {
    

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

impl AsRef<str> for AuditLogConfigLogTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            AuditLogConfigLogTypeEnum::ADMINREAD => "ADMIN_READ",
            AuditLogConfigLogTypeEnum::DATAWRITE => "DATA_WRITE",
            AuditLogConfigLogTypeEnum::DATAREAD => "DATA_READ",
        }
    }
}

impl std::convert::TryFrom< &str> for AuditLogConfigLogTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED),
           "ADMIN_READ" => Ok(AuditLogConfigLogTypeEnum::ADMINREAD),
           "DATA_WRITE" => Ok(AuditLogConfigLogTypeEnum::DATAWRITE),
           "DATA_READ" => Ok(AuditLogConfigLogTypeEnum::DATAREAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuditLogConfigLogTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuthConfigAuthTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of authentication configured.
pub enum AuthConfigAuthTypeEnum {
    

    /// Authentication type not specified.
    ///
    /// "AUTH_TYPE_UNSPECIFIED"
    #[serde(rename="AUTH_TYPE_UNSPECIFIED")]
    AUTHTYPEUNSPECIFIED,
    

    /// Username and Password Authentication.
    ///
    /// "USER_PASSWORD"
    #[serde(rename="USER_PASSWORD")]
    USERPASSWORD,
    

    /// JSON Web Token (JWT) Profile for Oauth 2.0 Authorization Grant based authentication
    ///
    /// "OAUTH2_JWT_BEARER"
    #[serde(rename="OAUTH2_JWT_BEARER")]
    OAUTH2JWTBEARER,
    

    /// Oauth 2.0 Client Credentials Grant Authentication
    ///
    /// "OAUTH2_CLIENT_CREDENTIALS"
    #[serde(rename="OAUTH2_CLIENT_CREDENTIALS")]
    OAUTH2CLIENTCREDENTIALS,
    

    /// SSH Public Key Authentication
    ///
    /// "SSH_PUBLIC_KEY"
    #[serde(rename="SSH_PUBLIC_KEY")]
    SSHPUBLICKEY,
    

    /// Oauth 2.0 Authorization Code Flow
    ///
    /// "OAUTH2_AUTH_CODE_FLOW"
    #[serde(rename="OAUTH2_AUTH_CODE_FLOW")]
    OAUTH2AUTHCODEFLOW,
    

    /// Google authentication
    ///
    /// "GOOGLE_AUTHENTICATION"
    #[serde(rename="GOOGLE_AUTHENTICATION")]
    GOOGLEAUTHENTICATION,
}

impl AsRef<str> for AuthConfigAuthTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthConfigAuthTypeEnum::AUTHTYPEUNSPECIFIED => "AUTH_TYPE_UNSPECIFIED",
            AuthConfigAuthTypeEnum::USERPASSWORD => "USER_PASSWORD",
            AuthConfigAuthTypeEnum::OAUTH2JWTBEARER => "OAUTH2_JWT_BEARER",
            AuthConfigAuthTypeEnum::OAUTH2CLIENTCREDENTIALS => "OAUTH2_CLIENT_CREDENTIALS",
            AuthConfigAuthTypeEnum::SSHPUBLICKEY => "SSH_PUBLIC_KEY",
            AuthConfigAuthTypeEnum::OAUTH2AUTHCODEFLOW => "OAUTH2_AUTH_CODE_FLOW",
            AuthConfigAuthTypeEnum::GOOGLEAUTHENTICATION => "GOOGLE_AUTHENTICATION",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthConfigAuthTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTH_TYPE_UNSPECIFIED" => Ok(AuthConfigAuthTypeEnum::AUTHTYPEUNSPECIFIED),
           "USER_PASSWORD" => Ok(AuthConfigAuthTypeEnum::USERPASSWORD),
           "OAUTH2_JWT_BEARER" => Ok(AuthConfigAuthTypeEnum::OAUTH2JWTBEARER),
           "OAUTH2_CLIENT_CREDENTIALS" => Ok(AuthConfigAuthTypeEnum::OAUTH2CLIENTCREDENTIALS),
           "SSH_PUBLIC_KEY" => Ok(AuthConfigAuthTypeEnum::SSHPUBLICKEY),
           "OAUTH2_AUTH_CODE_FLOW" => Ok(AuthConfigAuthTypeEnum::OAUTH2AUTHCODEFLOW),
           "GOOGLE_AUTHENTICATION" => Ok(AuthConfigAuthTypeEnum::GOOGLEAUTHENTICATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthConfigAuthTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuthConfigTemplateAuthTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of authentication configured.
pub enum AuthConfigTemplateAuthTypeEnum {
    

    /// Authentication type not specified.
    ///
    /// "AUTH_TYPE_UNSPECIFIED"
    #[serde(rename="AUTH_TYPE_UNSPECIFIED")]
    AUTHTYPEUNSPECIFIED,
    

    /// Username and Password Authentication.
    ///
    /// "USER_PASSWORD"
    #[serde(rename="USER_PASSWORD")]
    USERPASSWORD,
    

    /// JSON Web Token (JWT) Profile for Oauth 2.0 Authorization Grant based authentication
    ///
    /// "OAUTH2_JWT_BEARER"
    #[serde(rename="OAUTH2_JWT_BEARER")]
    OAUTH2JWTBEARER,
    

    /// Oauth 2.0 Client Credentials Grant Authentication
    ///
    /// "OAUTH2_CLIENT_CREDENTIALS"
    #[serde(rename="OAUTH2_CLIENT_CREDENTIALS")]
    OAUTH2CLIENTCREDENTIALS,
    

    /// SSH Public Key Authentication
    ///
    /// "SSH_PUBLIC_KEY"
    #[serde(rename="SSH_PUBLIC_KEY")]
    SSHPUBLICKEY,
    

    /// Oauth 2.0 Authorization Code Flow
    ///
    /// "OAUTH2_AUTH_CODE_FLOW"
    #[serde(rename="OAUTH2_AUTH_CODE_FLOW")]
    OAUTH2AUTHCODEFLOW,
    

    /// Google authentication
    ///
    /// "GOOGLE_AUTHENTICATION"
    #[serde(rename="GOOGLE_AUTHENTICATION")]
    GOOGLEAUTHENTICATION,
}

impl AsRef<str> for AuthConfigTemplateAuthTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthConfigTemplateAuthTypeEnum::AUTHTYPEUNSPECIFIED => "AUTH_TYPE_UNSPECIFIED",
            AuthConfigTemplateAuthTypeEnum::USERPASSWORD => "USER_PASSWORD",
            AuthConfigTemplateAuthTypeEnum::OAUTH2JWTBEARER => "OAUTH2_JWT_BEARER",
            AuthConfigTemplateAuthTypeEnum::OAUTH2CLIENTCREDENTIALS => "OAUTH2_CLIENT_CREDENTIALS",
            AuthConfigTemplateAuthTypeEnum::SSHPUBLICKEY => "SSH_PUBLIC_KEY",
            AuthConfigTemplateAuthTypeEnum::OAUTH2AUTHCODEFLOW => "OAUTH2_AUTH_CODE_FLOW",
            AuthConfigTemplateAuthTypeEnum::GOOGLEAUTHENTICATION => "GOOGLE_AUTHENTICATION",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthConfigTemplateAuthTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTH_TYPE_UNSPECIFIED" => Ok(AuthConfigTemplateAuthTypeEnum::AUTHTYPEUNSPECIFIED),
           "USER_PASSWORD" => Ok(AuthConfigTemplateAuthTypeEnum::USERPASSWORD),
           "OAUTH2_JWT_BEARER" => Ok(AuthConfigTemplateAuthTypeEnum::OAUTH2JWTBEARER),
           "OAUTH2_CLIENT_CREDENTIALS" => Ok(AuthConfigTemplateAuthTypeEnum::OAUTH2CLIENTCREDENTIALS),
           "SSH_PUBLIC_KEY" => Ok(AuthConfigTemplateAuthTypeEnum::SSHPUBLICKEY),
           "OAUTH2_AUTH_CODE_FLOW" => Ok(AuthConfigTemplateAuthTypeEnum::OAUTH2AUTHCODEFLOW),
           "GOOGLE_AUTHENTICATION" => Ok(AuthConfigTemplateAuthTypeEnum::GOOGLEAUTHENTICATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthConfigTemplateAuthTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BillingConfigBillingCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Billing category for the connector.
pub enum BillingConfigBillingCategoryEnum {
    

    /// Billing category is not specified.
    ///
    /// "BILLING_CATEGORY_UNSPECIFIED"
    #[serde(rename="BILLING_CATEGORY_UNSPECIFIED")]
    BILLINGCATEGORYUNSPECIFIED,
    

    /// GCP/Technical connector.
    ///
    /// "GCP_AND_TECHNICAL_CONNECTOR"
    #[serde(rename="GCP_AND_TECHNICAL_CONNECTOR")]
    GCPANDTECHNICALCONNECTOR,
    

    /// Non-GCP connector.
    ///
    /// "NON_GCP_CONNECTOR"
    #[serde(rename="NON_GCP_CONNECTOR")]
    NONGCPCONNECTOR,
}

impl AsRef<str> for BillingConfigBillingCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BillingConfigBillingCategoryEnum::BILLINGCATEGORYUNSPECIFIED => "BILLING_CATEGORY_UNSPECIFIED",
            BillingConfigBillingCategoryEnum::GCPANDTECHNICALCONNECTOR => "GCP_AND_TECHNICAL_CONNECTOR",
            BillingConfigBillingCategoryEnum::NONGCPCONNECTOR => "NON_GCP_CONNECTOR",
        }
    }
}

impl std::convert::TryFrom< &str> for BillingConfigBillingCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BILLING_CATEGORY_UNSPECIFIED" => Ok(BillingConfigBillingCategoryEnum::BILLINGCATEGORYUNSPECIFIED),
           "GCP_AND_TECHNICAL_CONNECTOR" => Ok(BillingConfigBillingCategoryEnum::GCPANDTECHNICALCONNECTOR),
           "NON_GCP_CONNECTOR" => Ok(BillingConfigBillingCategoryEnum::NONGCPCONNECTOR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BillingConfigBillingCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConfigVariableTemplateEnumSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. enum source denotes the source of api to fill the enum options
pub enum ConfigVariableTemplateEnumSourceEnum {
    

    /// Api type unspecified.
    ///
    /// "ENUM_SOURCE_UNSPECIFIED"
    #[serde(rename="ENUM_SOURCE_UNSPECIFIED")]
    ENUMSOURCEUNSPECIFIED,
    

    /// list event types.
    ///
    /// "EVENT_TYPES_API"
    #[serde(rename="EVENT_TYPES_API")]
    EVENTTYPESAPI,
}

impl AsRef<str> for ConfigVariableTemplateEnumSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConfigVariableTemplateEnumSourceEnum::ENUMSOURCEUNSPECIFIED => "ENUM_SOURCE_UNSPECIFIED",
            ConfigVariableTemplateEnumSourceEnum::EVENTTYPESAPI => "EVENT_TYPES_API",
        }
    }
}

impl std::convert::TryFrom< &str> for ConfigVariableTemplateEnumSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENUM_SOURCE_UNSPECIFIED" => Ok(ConfigVariableTemplateEnumSourceEnum::ENUMSOURCEUNSPECIFIED),
           "EVENT_TYPES_API" => Ok(ConfigVariableTemplateEnumSourceEnum::EVENTTYPESAPI),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConfigVariableTemplateEnumSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConfigVariableTemplateLocationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Location Tyep denotes where this value should be sent in BYOC connections.
pub enum ConfigVariableTemplateLocationTypeEnum {
    

    /// Location type unspecified.
    ///
    /// "LOCATION_TYPE_UNSPECIFIED"
    #[serde(rename="LOCATION_TYPE_UNSPECIFIED")]
    LOCATIONTYPEUNSPECIFIED,
    

    /// Request header.
    ///
    /// "HEADER"
    #[serde(rename="HEADER")]
    HEADER,
    

    /// Request Payload.
    ///
    /// "PAYLOAD"
    #[serde(rename="PAYLOAD")]
    PAYLOAD,
    

    /// Request query param.
    ///
    /// "QUERY_PARAM"
    #[serde(rename="QUERY_PARAM")]
    QUERYPARAM,
    

    /// Request path param.
    ///
    /// "PATH_PARAM"
    #[serde(rename="PATH_PARAM")]
    PATHPARAM,
}

impl AsRef<str> for ConfigVariableTemplateLocationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConfigVariableTemplateLocationTypeEnum::LOCATIONTYPEUNSPECIFIED => "LOCATION_TYPE_UNSPECIFIED",
            ConfigVariableTemplateLocationTypeEnum::HEADER => "HEADER",
            ConfigVariableTemplateLocationTypeEnum::PAYLOAD => "PAYLOAD",
            ConfigVariableTemplateLocationTypeEnum::QUERYPARAM => "QUERY_PARAM",
            ConfigVariableTemplateLocationTypeEnum::PATHPARAM => "PATH_PARAM",
        }
    }
}

impl std::convert::TryFrom< &str> for ConfigVariableTemplateLocationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOCATION_TYPE_UNSPECIFIED" => Ok(ConfigVariableTemplateLocationTypeEnum::LOCATIONTYPEUNSPECIFIED),
           "HEADER" => Ok(ConfigVariableTemplateLocationTypeEnum::HEADER),
           "PAYLOAD" => Ok(ConfigVariableTemplateLocationTypeEnum::PAYLOAD),
           "QUERY_PARAM" => Ok(ConfigVariableTemplateLocationTypeEnum::QUERYPARAM),
           "PATH_PARAM" => Ok(ConfigVariableTemplateLocationTypeEnum::PATHPARAM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConfigVariableTemplateLocationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConfigVariableTemplateStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the config variable.
pub enum ConfigVariableTemplateStateEnum {
    

    /// Status is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Config variable is active
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Config variable is deprecated.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for ConfigVariableTemplateStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConfigVariableTemplateStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ConfigVariableTemplateStateEnum::ACTIVE => "ACTIVE",
            ConfigVariableTemplateStateEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for ConfigVariableTemplateStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ConfigVariableTemplateStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(ConfigVariableTemplateStateEnum::ACTIVE),
           "DEPRECATED" => Ok(ConfigVariableTemplateStateEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConfigVariableTemplateStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConfigVariableTemplateValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the parameter: string, int, bool etc. consider custom type for the benefit for the validation.
pub enum ConfigVariableTemplateValueTypeEnum {
    

    /// Value type is not specified.
    ///
    /// "VALUE_TYPE_UNSPECIFIED"
    #[serde(rename="VALUE_TYPE_UNSPECIFIED")]
    VALUETYPEUNSPECIFIED,
    

    /// Value type is string.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// Value type is integer.
    ///
    /// "INT"
    #[serde(rename="INT")]
    INT,
    

    /// Value type is boolean.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// Value type is secret.
    ///
    /// "SECRET"
    #[serde(rename="SECRET")]
    SECRET,
    

    /// Value type is enum.
    ///
    /// "ENUM"
    #[serde(rename="ENUM")]
    ENUM,
    

    /// Value type is authorization code.
    ///
    /// "AUTHORIZATION_CODE"
    #[serde(rename="AUTHORIZATION_CODE")]
    AUTHORIZATIONCODE,
    

    /// Encryption Key.
    ///
    /// "ENCRYPTION_KEY"
    #[serde(rename="ENCRYPTION_KEY")]
    ENCRYPTIONKEY,
}

impl AsRef<str> for ConfigVariableTemplateValueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConfigVariableTemplateValueTypeEnum::VALUETYPEUNSPECIFIED => "VALUE_TYPE_UNSPECIFIED",
            ConfigVariableTemplateValueTypeEnum::STRING => "STRING",
            ConfigVariableTemplateValueTypeEnum::INT => "INT",
            ConfigVariableTemplateValueTypeEnum::BOOL => "BOOL",
            ConfigVariableTemplateValueTypeEnum::SECRET => "SECRET",
            ConfigVariableTemplateValueTypeEnum::ENUM => "ENUM",
            ConfigVariableTemplateValueTypeEnum::AUTHORIZATIONCODE => "AUTHORIZATION_CODE",
            ConfigVariableTemplateValueTypeEnum::ENCRYPTIONKEY => "ENCRYPTION_KEY",
        }
    }
}

impl std::convert::TryFrom< &str> for ConfigVariableTemplateValueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VALUE_TYPE_UNSPECIFIED" => Ok(ConfigVariableTemplateValueTypeEnum::VALUETYPEUNSPECIFIED),
           "STRING" => Ok(ConfigVariableTemplateValueTypeEnum::STRING),
           "INT" => Ok(ConfigVariableTemplateValueTypeEnum::INT),
           "BOOL" => Ok(ConfigVariableTemplateValueTypeEnum::BOOL),
           "SECRET" => Ok(ConfigVariableTemplateValueTypeEnum::SECRET),
           "ENUM" => Ok(ConfigVariableTemplateValueTypeEnum::ENUM),
           "AUTHORIZATION_CODE" => Ok(ConfigVariableTemplateValueTypeEnum::AUTHORIZATIONCODE),
           "ENCRYPTION_KEY" => Ok(ConfigVariableTemplateValueTypeEnum::ENCRYPTIONKEY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConfigVariableTemplateValueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectionConnectorVersionLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Flag to mark the version indicating the launch stage.
pub enum ConnectionConnectorVersionLaunchStageEnum {
    

    /// LAUNCH_STAGE_UNSPECIFIED.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// PREVIEW.
    ///
    /// "PREVIEW"
    #[serde(rename="PREVIEW")]
    PREVIEW,
    

    /// GA.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// DEPRECATED.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
    

    /// PRIVATE_PREVIEW.
    ///
    /// "PRIVATE_PREVIEW"
    #[serde(rename="PRIVATE_PREVIEW")]
    PRIVATEPREVIEW,
}

impl AsRef<str> for ConnectionConnectorVersionLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectionConnectorVersionLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            ConnectionConnectorVersionLaunchStageEnum::PREVIEW => "PREVIEW",
            ConnectionConnectorVersionLaunchStageEnum::GA => "GA",
            ConnectionConnectorVersionLaunchStageEnum::DEPRECATED => "DEPRECATED",
            ConnectionConnectorVersionLaunchStageEnum::PRIVATEPREVIEW => "PRIVATE_PREVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectionConnectorVersionLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(ConnectionConnectorVersionLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "PREVIEW" => Ok(ConnectionConnectorVersionLaunchStageEnum::PREVIEW),
           "GA" => Ok(ConnectionConnectorVersionLaunchStageEnum::GA),
           "DEPRECATED" => Ok(ConnectionConnectorVersionLaunchStageEnum::DEPRECATED),
           "PRIVATE_PREVIEW" => Ok(ConnectionConnectorVersionLaunchStageEnum::PRIVATEPREVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectionConnectorVersionLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectionEventingEnablementTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Eventing enablement type. Will be nil if eventing is not enabled.
pub enum ConnectionEventingEnablementTypeEnum {
    

    /// Eventing Enablement Type Unspecifeied.
    ///
    /// "EVENTING_ENABLEMENT_TYPE_UNSPECIFIED"
    #[serde(rename="EVENTING_ENABLEMENT_TYPE_UNSPECIFIED")]
    EVENTINGENABLEMENTTYPEUNSPECIFIED,
    

    /// Both connection and eventing.
    ///
    /// "EVENTING_AND_CONNECTION"
    #[serde(rename="EVENTING_AND_CONNECTION")]
    EVENTINGANDCONNECTION,
    

    /// Only Eventing.
    ///
    /// "ONLY_EVENTING"
    #[serde(rename="ONLY_EVENTING")]
    ONLYEVENTING,
}

impl AsRef<str> for ConnectionEventingEnablementTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectionEventingEnablementTypeEnum::EVENTINGENABLEMENTTYPEUNSPECIFIED => "EVENTING_ENABLEMENT_TYPE_UNSPECIFIED",
            ConnectionEventingEnablementTypeEnum::EVENTINGANDCONNECTION => "EVENTING_AND_CONNECTION",
            ConnectionEventingEnablementTypeEnum::ONLYEVENTING => "ONLY_EVENTING",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectionEventingEnablementTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENTING_ENABLEMENT_TYPE_UNSPECIFIED" => Ok(ConnectionEventingEnablementTypeEnum::EVENTINGENABLEMENTTYPEUNSPECIFIED),
           "EVENTING_AND_CONNECTION" => Ok(ConnectionEventingEnablementTypeEnum::EVENTINGANDCONNECTION),
           "ONLY_EVENTING" => Ok(ConnectionEventingEnablementTypeEnum::ONLYEVENTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectionEventingEnablementTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectionSubscriptionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. This subscription type enum states the subscription type of the project.
pub enum ConnectionSubscriptionTypeEnum {
    

    /// Unspecified subscription type.
    ///
    /// "SUBSCRIPTION_TYPE_UNSPECIFIED"
    #[serde(rename="SUBSCRIPTION_TYPE_UNSPECIFIED")]
    SUBSCRIPTIONTYPEUNSPECIFIED,
    

    /// PayG subscription.
    ///
    /// "PAY_G"
    #[serde(rename="PAY_G")]
    PAYG,
    

    /// Paid Subscription.
    ///
    /// "PAID"
    #[serde(rename="PAID")]
    PAID,
}

impl AsRef<str> for ConnectionSubscriptionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectionSubscriptionTypeEnum::SUBSCRIPTIONTYPEUNSPECIFIED => "SUBSCRIPTION_TYPE_UNSPECIFIED",
            ConnectionSubscriptionTypeEnum::PAYG => "PAY_G",
            ConnectionSubscriptionTypeEnum::PAID => "PAID",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectionSubscriptionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBSCRIPTION_TYPE_UNSPECIFIED" => Ok(ConnectionSubscriptionTypeEnum::SUBSCRIPTIONTYPEUNSPECIFIED),
           "PAY_G" => Ok(ConnectionSubscriptionTypeEnum::PAYG),
           "PAID" => Ok(ConnectionSubscriptionTypeEnum::PAID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectionSubscriptionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectionSchemaMetadataStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of runtime schema.
pub enum ConnectionSchemaMetadataStateEnum {
    

    /// Default state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Schema refresh is in progress.
    ///
    /// "REFRESHING"
    #[serde(rename="REFRESHING")]
    REFRESHING,
    

    /// Schema has been updated.
    ///
    /// "UPDATED"
    #[serde(rename="UPDATED")]
    UPDATED,
    

    /// Schema refresh for metadata is in progress.
    ///
    /// "REFRESHING_SCHEMA_METADATA"
    #[serde(rename="REFRESHING_SCHEMA_METADATA")]
    REFRESHINGSCHEMAMETADATA,
    

    /// Schema metadata has been updated.
    ///
    /// "UPDATED_SCHEMA_METADATA"
    #[serde(rename="UPDATED_SCHEMA_METADATA")]
    UPDATEDSCHEMAMETADATA,
    

    /// Failed to refresh schema metadata
    ///
    /// "REFRESH_SCHEMA_METADATA_FAILED"
    #[serde(rename="REFRESH_SCHEMA_METADATA_FAILED")]
    REFRESHSCHEMAMETADATAFAILED,
    

    /// Triggered full schema refresh
    ///
    /// "REFRESHING_FULL_SCHEMA"
    #[serde(rename="REFRESHING_FULL_SCHEMA")]
    REFRESHINGFULLSCHEMA,
    

    /// Updated full schema
    ///
    /// "UPDATED_FULL_SCHEMA"
    #[serde(rename="UPDATED_FULL_SCHEMA")]
    UPDATEDFULLSCHEMA,
}

impl AsRef<str> for ConnectionSchemaMetadataStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectionSchemaMetadataStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ConnectionSchemaMetadataStateEnum::REFRESHING => "REFRESHING",
            ConnectionSchemaMetadataStateEnum::UPDATED => "UPDATED",
            ConnectionSchemaMetadataStateEnum::REFRESHINGSCHEMAMETADATA => "REFRESHING_SCHEMA_METADATA",
            ConnectionSchemaMetadataStateEnum::UPDATEDSCHEMAMETADATA => "UPDATED_SCHEMA_METADATA",
            ConnectionSchemaMetadataStateEnum::REFRESHSCHEMAMETADATAFAILED => "REFRESH_SCHEMA_METADATA_FAILED",
            ConnectionSchemaMetadataStateEnum::REFRESHINGFULLSCHEMA => "REFRESHING_FULL_SCHEMA",
            ConnectionSchemaMetadataStateEnum::UPDATEDFULLSCHEMA => "UPDATED_FULL_SCHEMA",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectionSchemaMetadataStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ConnectionSchemaMetadataStateEnum::STATEUNSPECIFIED),
           "REFRESHING" => Ok(ConnectionSchemaMetadataStateEnum::REFRESHING),
           "UPDATED" => Ok(ConnectionSchemaMetadataStateEnum::UPDATED),
           "REFRESHING_SCHEMA_METADATA" => Ok(ConnectionSchemaMetadataStateEnum::REFRESHINGSCHEMAMETADATA),
           "UPDATED_SCHEMA_METADATA" => Ok(ConnectionSchemaMetadataStateEnum::UPDATEDSCHEMAMETADATA),
           "REFRESH_SCHEMA_METADATA_FAILED" => Ok(ConnectionSchemaMetadataStateEnum::REFRESHSCHEMAMETADATAFAILED),
           "REFRESHING_FULL_SCHEMA" => Ok(ConnectionSchemaMetadataStateEnum::REFRESHINGFULLSCHEMA),
           "UPDATED_FULL_SCHEMA" => Ok(ConnectionSchemaMetadataStateEnum::UPDATEDFULLSCHEMA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectionSchemaMetadataStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectionStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State.
pub enum ConnectionStatusStateEnum {
    

    /// Connection does not have a state yet.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Connection is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Connection is running and ready for requests.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Connection is stopped.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// Connection is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Connection is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// Connection is not running due to an error.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Connection is not running because the authorization configuration is not complete.
    ///
    /// "AUTHORIZATION_REQUIRED"
    #[serde(rename="AUTHORIZATION_REQUIRED")]
    AUTHORIZATIONREQUIRED,
}

impl AsRef<str> for ConnectionStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectionStatusStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ConnectionStatusStateEnum::CREATING => "CREATING",
            ConnectionStatusStateEnum::ACTIVE => "ACTIVE",
            ConnectionStatusStateEnum::INACTIVE => "INACTIVE",
            ConnectionStatusStateEnum::DELETING => "DELETING",
            ConnectionStatusStateEnum::UPDATING => "UPDATING",
            ConnectionStatusStateEnum::ERROR => "ERROR",
            ConnectionStatusStateEnum::AUTHORIZATIONREQUIRED => "AUTHORIZATION_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectionStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ConnectionStatusStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(ConnectionStatusStateEnum::CREATING),
           "ACTIVE" => Ok(ConnectionStatusStateEnum::ACTIVE),
           "INACTIVE" => Ok(ConnectionStatusStateEnum::INACTIVE),
           "DELETING" => Ok(ConnectionStatusStateEnum::DELETING),
           "UPDATING" => Ok(ConnectionStatusStateEnum::UPDATING),
           "ERROR" => Ok(ConnectionStatusStateEnum::ERROR),
           "AUTHORIZATION_REQUIRED" => Ok(ConnectionStatusStateEnum::AUTHORIZATIONREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectionStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectorLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Flag to mark the version indicating the launch stage.
pub enum ConnectorLaunchStageEnum {
    

    /// LAUNCH_STAGE_UNSPECIFIED.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// PREVIEW.
    ///
    /// "PREVIEW"
    #[serde(rename="PREVIEW")]
    PREVIEW,
    

    /// GA.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// DEPRECATED.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
    

    /// PRIVATE_PREVIEW.
    ///
    /// "PRIVATE_PREVIEW"
    #[serde(rename="PRIVATE_PREVIEW")]
    PRIVATEPREVIEW,
}

impl AsRef<str> for ConnectorLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectorLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            ConnectorLaunchStageEnum::PREVIEW => "PREVIEW",
            ConnectorLaunchStageEnum::GA => "GA",
            ConnectorLaunchStageEnum::DEPRECATED => "DEPRECATED",
            ConnectorLaunchStageEnum::PRIVATEPREVIEW => "PRIVATE_PREVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectorLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(ConnectorLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "PREVIEW" => Ok(ConnectorLaunchStageEnum::PREVIEW),
           "GA" => Ok(ConnectorLaunchStageEnum::GA),
           "DEPRECATED" => Ok(ConnectorLaunchStageEnum::DEPRECATED),
           "PRIVATE_PREVIEW" => Ok(ConnectorLaunchStageEnum::PRIVATEPREVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectorLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectorInfraConfigDeploymentModelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicate whether connector is deployed on GKE/CloudRun
pub enum ConnectorInfraConfigDeploymentModelEnum {
    

    /// Deployment model is not specified.
    ///
    /// "DEPLOYMENT_MODEL_UNSPECIFIED"
    #[serde(rename="DEPLOYMENT_MODEL_UNSPECIFIED")]
    DEPLOYMENTMODELUNSPECIFIED,
    

    /// Default model gke mst.
    ///
    /// "GKE_MST"
    #[serde(rename="GKE_MST")]
    GKEMST,
    

    /// Cloud run mst.
    ///
    /// "CLOUD_RUN_MST"
    #[serde(rename="CLOUD_RUN_MST")]
    CLOUDRUNMST,
}

impl AsRef<str> for ConnectorInfraConfigDeploymentModelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectorInfraConfigDeploymentModelEnum::DEPLOYMENTMODELUNSPECIFIED => "DEPLOYMENT_MODEL_UNSPECIFIED",
            ConnectorInfraConfigDeploymentModelEnum::GKEMST => "GKE_MST",
            ConnectorInfraConfigDeploymentModelEnum::CLOUDRUNMST => "CLOUD_RUN_MST",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectorInfraConfigDeploymentModelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEPLOYMENT_MODEL_UNSPECIFIED" => Ok(ConnectorInfraConfigDeploymentModelEnum::DEPLOYMENTMODELUNSPECIFIED),
           "GKE_MST" => Ok(ConnectorInfraConfigDeploymentModelEnum::GKEMST),
           "CLOUD_RUN_MST" => Ok(ConnectorInfraConfigDeploymentModelEnum::CLOUDRUNMST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectorInfraConfigDeploymentModelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectorVersionLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Flag to mark the version indicating the launch stage.
pub enum ConnectorVersionLaunchStageEnum {
    

    /// LAUNCH_STAGE_UNSPECIFIED.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// PREVIEW.
    ///
    /// "PREVIEW"
    #[serde(rename="PREVIEW")]
    PREVIEW,
    

    /// GA.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// DEPRECATED.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
    

    /// PRIVATE_PREVIEW.
    ///
    /// "PRIVATE_PREVIEW"
    #[serde(rename="PRIVATE_PREVIEW")]
    PRIVATEPREVIEW,
}

impl AsRef<str> for ConnectorVersionLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectorVersionLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            ConnectorVersionLaunchStageEnum::PREVIEW => "PREVIEW",
            ConnectorVersionLaunchStageEnum::GA => "GA",
            ConnectorVersionLaunchStageEnum::DEPRECATED => "DEPRECATED",
            ConnectorVersionLaunchStageEnum::PRIVATEPREVIEW => "PRIVATE_PREVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectorVersionLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(ConnectorVersionLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "PREVIEW" => Ok(ConnectorVersionLaunchStageEnum::PREVIEW),
           "GA" => Ok(ConnectorVersionLaunchStageEnum::GA),
           "DEPRECATED" => Ok(ConnectorVersionLaunchStageEnum::DEPRECATED),
           "PRIVATE_PREVIEW" => Ok(ConnectorVersionLaunchStageEnum::PRIVATEPREVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectorVersionLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectorVersionUnsupportedConnectionTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Unsupported connection types.
pub enum ConnectorVersionUnsupportedConnectionTypesEnum {
    

    /// Connection type is unspecified.
    ///
    /// "CONNECTION_TYPE_UNSPECIFIED"
    #[serde(rename="CONNECTION_TYPE_UNSPECIFIED")]
    CONNECTIONTYPEUNSPECIFIED,
    

    /// Connection with eventing.
    ///
    /// "CONNECTION_WITH_EVENTING"
    #[serde(rename="CONNECTION_WITH_EVENTING")]
    CONNECTIONWITHEVENTING,
    

    /// Only connection.
    ///
    /// "ONLY_CONNECTION"
    #[serde(rename="ONLY_CONNECTION")]
    ONLYCONNECTION,
    

    /// Only eventing.
    ///
    /// "ONLY_EVENTING"
    #[serde(rename="ONLY_EVENTING")]
    ONLYEVENTING,
}

impl AsRef<str> for ConnectorVersionUnsupportedConnectionTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectorVersionUnsupportedConnectionTypesEnum::CONNECTIONTYPEUNSPECIFIED => "CONNECTION_TYPE_UNSPECIFIED",
            ConnectorVersionUnsupportedConnectionTypesEnum::CONNECTIONWITHEVENTING => "CONNECTION_WITH_EVENTING",
            ConnectorVersionUnsupportedConnectionTypesEnum::ONLYCONNECTION => "ONLY_CONNECTION",
            ConnectorVersionUnsupportedConnectionTypesEnum::ONLYEVENTING => "ONLY_EVENTING",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectorVersionUnsupportedConnectionTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTION_TYPE_UNSPECIFIED" => Ok(ConnectorVersionUnsupportedConnectionTypesEnum::CONNECTIONTYPEUNSPECIFIED),
           "CONNECTION_WITH_EVENTING" => Ok(ConnectorVersionUnsupportedConnectionTypesEnum::CONNECTIONWITHEVENTING),
           "ONLY_CONNECTION" => Ok(ConnectorVersionUnsupportedConnectionTypesEnum::ONLYCONNECTION),
           "ONLY_EVENTING" => Ok(ConnectorVersionUnsupportedConnectionTypesEnum::ONLYEVENTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectorVersionUnsupportedConnectionTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectorVersionInfraConfigDeploymentModelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Indicates whether connector is deployed on GKE/CloudRun
pub enum ConnectorVersionInfraConfigDeploymentModelEnum {
    

    /// Deployment model is not specified.
    ///
    /// "DEPLOYMENT_MODEL_UNSPECIFIED"
    #[serde(rename="DEPLOYMENT_MODEL_UNSPECIFIED")]
    DEPLOYMENTMODELUNSPECIFIED,
    

    /// Default model gke mst.
    ///
    /// "GKE_MST"
    #[serde(rename="GKE_MST")]
    GKEMST,
    

    /// Cloud run mst.
    ///
    /// "CLOUD_RUN_MST"
    #[serde(rename="CLOUD_RUN_MST")]
    CLOUDRUNMST,
}

impl AsRef<str> for ConnectorVersionInfraConfigDeploymentModelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectorVersionInfraConfigDeploymentModelEnum::DEPLOYMENTMODELUNSPECIFIED => "DEPLOYMENT_MODEL_UNSPECIFIED",
            ConnectorVersionInfraConfigDeploymentModelEnum::GKEMST => "GKE_MST",
            ConnectorVersionInfraConfigDeploymentModelEnum::CLOUDRUNMST => "CLOUD_RUN_MST",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectorVersionInfraConfigDeploymentModelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEPLOYMENT_MODEL_UNSPECIFIED" => Ok(ConnectorVersionInfraConfigDeploymentModelEnum::DEPLOYMENTMODELUNSPECIFIED),
           "GKE_MST" => Ok(ConnectorVersionInfraConfigDeploymentModelEnum::GKEMST),
           "CLOUD_RUN_MST" => Ok(ConnectorVersionInfraConfigDeploymentModelEnum::CLOUDRUNMST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectorVersionInfraConfigDeploymentModelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomConnectorCustomConnectorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Type of the custom connector.
pub enum CustomConnectorCustomConnectorTypeEnum {
    

    /// Connector type is not specified.
    ///
    /// "CUSTOM_CONNECTOR_TYPE_UNSPECIFIED"
    #[serde(rename="CUSTOM_CONNECTOR_TYPE_UNSPECIFIED")]
    CUSTOMCONNECTORTYPEUNSPECIFIED,
    

    /// OpenAPI connector.
    ///
    /// "OPEN_API"
    #[serde(rename="OPEN_API")]
    OPENAPI,
    

    /// Proto connector.
    ///
    /// "PROTO"
    #[serde(rename="PROTO")]
    PROTO,
}

impl AsRef<str> for CustomConnectorCustomConnectorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomConnectorCustomConnectorTypeEnum::CUSTOMCONNECTORTYPEUNSPECIFIED => "CUSTOM_CONNECTOR_TYPE_UNSPECIFIED",
            CustomConnectorCustomConnectorTypeEnum::OPENAPI => "OPEN_API",
            CustomConnectorCustomConnectorTypeEnum::PROTO => "PROTO",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomConnectorCustomConnectorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CUSTOM_CONNECTOR_TYPE_UNSPECIFIED" => Ok(CustomConnectorCustomConnectorTypeEnum::CUSTOMCONNECTORTYPEUNSPECIFIED),
           "OPEN_API" => Ok(CustomConnectorCustomConnectorTypeEnum::OPENAPI),
           "PROTO" => Ok(CustomConnectorCustomConnectorTypeEnum::PROTO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomConnectorCustomConnectorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomConnectorVersionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the custom connector version.
pub enum CustomConnectorVersionStateEnum {
    

    /// State Unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Active state. By default we set the state to Active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Deprecated state.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for CustomConnectorVersionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomConnectorVersionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CustomConnectorVersionStateEnum::ACTIVE => "ACTIVE",
            CustomConnectorVersionStateEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomConnectorVersionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CustomConnectorVersionStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(CustomConnectorVersionStateEnum::ACTIVE),
           "DEPRECATED" => Ok(CustomConnectorVersionStateEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomConnectorVersionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DestinationConfigTemplatePortFieldTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether port number should be provided by customers.
pub enum DestinationConfigTemplatePortFieldTypeEnum {
    
    /// "FIELD_TYPE_UNSPECIFIED"
    #[serde(rename="FIELD_TYPE_UNSPECIFIED")]
    FIELDTYPEUNSPECIFIED,
    
    /// "REQUIRED"
    #[serde(rename="REQUIRED")]
    REQUIRED,
    
    /// "OPTIONAL"
    #[serde(rename="OPTIONAL")]
    OPTIONAL,
    
    /// "NOT_USED"
    #[serde(rename="NOT_USED")]
    NOTUSED,
}

impl AsRef<str> for DestinationConfigTemplatePortFieldTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DestinationConfigTemplatePortFieldTypeEnum::FIELDTYPEUNSPECIFIED => "FIELD_TYPE_UNSPECIFIED",
            DestinationConfigTemplatePortFieldTypeEnum::REQUIRED => "REQUIRED",
            DestinationConfigTemplatePortFieldTypeEnum::OPTIONAL => "OPTIONAL",
            DestinationConfigTemplatePortFieldTypeEnum::NOTUSED => "NOT_USED",
        }
    }
}

impl std::convert::TryFrom< &str> for DestinationConfigTemplatePortFieldTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FIELD_TYPE_UNSPECIFIED" => Ok(DestinationConfigTemplatePortFieldTypeEnum::FIELDTYPEUNSPECIFIED),
           "REQUIRED" => Ok(DestinationConfigTemplatePortFieldTypeEnum::REQUIRED),
           "OPTIONAL" => Ok(DestinationConfigTemplatePortFieldTypeEnum::OPTIONAL),
           "NOT_USED" => Ok(DestinationConfigTemplatePortFieldTypeEnum::NOTUSED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DestinationConfigTemplatePortFieldTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EncryptionConfigEncryptionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Encryption type for the region.
pub enum EncryptionConfigEncryptionTypeEnum {
    

    /// Encryption type unspecified.
    ///
    /// "ENCRYPTION_TYPE_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_TYPE_UNSPECIFIED")]
    ENCRYPTIONTYPEUNSPECIFIED,
    

    /// Google managed encryption keys
    ///
    /// "GMEK"
    #[serde(rename="GMEK")]
    GMEK,
    

    /// Customer managed encryption keys.
    ///
    /// "CMEK"
    #[serde(rename="CMEK")]
    CMEK,
}

impl AsRef<str> for EncryptionConfigEncryptionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EncryptionConfigEncryptionTypeEnum::ENCRYPTIONTYPEUNSPECIFIED => "ENCRYPTION_TYPE_UNSPECIFIED",
            EncryptionConfigEncryptionTypeEnum::GMEK => "GMEK",
            EncryptionConfigEncryptionTypeEnum::CMEK => "CMEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EncryptionConfigEncryptionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_TYPE_UNSPECIFIED" => Ok(EncryptionConfigEncryptionTypeEnum::ENCRYPTIONTYPEUNSPECIFIED),
           "GMEK" => Ok(EncryptionConfigEncryptionTypeEnum::GMEK),
           "CMEK" => Ok(EncryptionConfigEncryptionTypeEnum::CMEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EncryptionConfigEncryptionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EncryptionKeyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type.
pub enum EncryptionKeyTypeEnum {
    

    /// Value type is not specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Google Managed.
    ///
    /// "GOOGLE_MANAGED"
    #[serde(rename="GOOGLE_MANAGED")]
    GOOGLEMANAGED,
    

    /// Customer Managed.
    ///
    /// "CUSTOMER_MANAGED"
    #[serde(rename="CUSTOMER_MANAGED")]
    CUSTOMERMANAGED,
}

impl AsRef<str> for EncryptionKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EncryptionKeyTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            EncryptionKeyTypeEnum::GOOGLEMANAGED => "GOOGLE_MANAGED",
            EncryptionKeyTypeEnum::CUSTOMERMANAGED => "CUSTOMER_MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for EncryptionKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(EncryptionKeyTypeEnum::TYPEUNSPECIFIED),
           "GOOGLE_MANAGED" => Ok(EncryptionKeyTypeEnum::GOOGLEMANAGED),
           "CUSTOMER_MANAGED" => Ok(EncryptionKeyTypeEnum::CUSTOMERMANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EncryptionKeyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventSubscriptionDestinationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// type of the destination
pub enum EventSubscriptionDestinationTypeEnum {
    

    /// Default state.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Endpoint - Hit the value of endpoint when event is received
    ///
    /// "ENDPOINT"
    #[serde(rename="ENDPOINT")]
    ENDPOINT,
}

impl AsRef<str> for EventSubscriptionDestinationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventSubscriptionDestinationTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            EventSubscriptionDestinationTypeEnum::ENDPOINT => "ENDPOINT",
        }
    }
}

impl std::convert::TryFrom< &str> for EventSubscriptionDestinationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(EventSubscriptionDestinationTypeEnum::TYPEUNSPECIFIED),
           "ENDPOINT" => Ok(EventSubscriptionDestinationTypeEnum::ENDPOINT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventSubscriptionDestinationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventSubscriptionStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of Event Subscription resource.
pub enum EventSubscriptionStatusStateEnum {
    

    /// Default state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// EventSubscription creation is in progress.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// EventSubscription is in Updating status.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// EventSubscription is in Active state and is ready to receive events.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// EventSubscription is currently suspended.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
    

    /// EventSubscription is in Error state.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for EventSubscriptionStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventSubscriptionStatusStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            EventSubscriptionStatusStateEnum::CREATING => "CREATING",
            EventSubscriptionStatusStateEnum::UPDATING => "UPDATING",
            EventSubscriptionStatusStateEnum::ACTIVE => "ACTIVE",
            EventSubscriptionStatusStateEnum::SUSPENDED => "SUSPENDED",
            EventSubscriptionStatusStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for EventSubscriptionStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(EventSubscriptionStatusStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(EventSubscriptionStatusStateEnum::CREATING),
           "UPDATING" => Ok(EventSubscriptionStatusStateEnum::UPDATING),
           "ACTIVE" => Ok(EventSubscriptionStatusStateEnum::ACTIVE),
           "SUSPENDED" => Ok(EventSubscriptionStatusStateEnum::SUSPENDED),
           "ERROR" => Ok(EventSubscriptionStatusStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventSubscriptionStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventingConfigTemplateEventListenerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the event listener for a specific connector.
pub enum EventingConfigTemplateEventListenerTypeEnum {
    

    /// Default value.
    ///
    /// "EVENT_LISTENER_TYPE_UNSPECIFIED"
    #[serde(rename="EVENT_LISTENER_TYPE_UNSPECIFIED")]
    EVENTLISTENERTYPEUNSPECIFIED,
    

    /// Webhook listener. e.g. Jira, Zendesk, Servicenow etc.,
    ///
    /// "WEBHOOK_LISTENER"
    #[serde(rename="WEBHOOK_LISTENER")]
    WEBHOOKLISTENER,
    

    /// JMS Listener. e.g. IBM MQ, Rabbit MQ etc.,
    ///
    /// "JMS_LISTENER"
    #[serde(rename="JMS_LISTENER")]
    JMSLISTENER,
}

impl AsRef<str> for EventingConfigTemplateEventListenerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventingConfigTemplateEventListenerTypeEnum::EVENTLISTENERTYPEUNSPECIFIED => "EVENT_LISTENER_TYPE_UNSPECIFIED",
            EventingConfigTemplateEventListenerTypeEnum::WEBHOOKLISTENER => "WEBHOOK_LISTENER",
            EventingConfigTemplateEventListenerTypeEnum::JMSLISTENER => "JMS_LISTENER",
        }
    }
}

impl std::convert::TryFrom< &str> for EventingConfigTemplateEventListenerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_LISTENER_TYPE_UNSPECIFIED" => Ok(EventingConfigTemplateEventListenerTypeEnum::EVENTLISTENERTYPEUNSPECIFIED),
           "WEBHOOK_LISTENER" => Ok(EventingConfigTemplateEventListenerTypeEnum::WEBHOOKLISTENER),
           "JMS_LISTENER" => Ok(EventingConfigTemplateEventListenerTypeEnum::JMSLISTENER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventingConfigTemplateEventListenerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventingDetailLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Eventing Launch Stage.
pub enum EventingDetailLaunchStageEnum {
    

    /// LAUNCH_STAGE_UNSPECIFIED.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// PREVIEW.
    ///
    /// "PREVIEW"
    #[serde(rename="PREVIEW")]
    PREVIEW,
    

    /// GA.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// DEPRECATED.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
    

    /// PRIVATE_PREVIEW.
    ///
    /// "PRIVATE_PREVIEW"
    #[serde(rename="PRIVATE_PREVIEW")]
    PRIVATEPREVIEW,
}

impl AsRef<str> for EventingDetailLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventingDetailLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            EventingDetailLaunchStageEnum::PREVIEW => "PREVIEW",
            EventingDetailLaunchStageEnum::GA => "GA",
            EventingDetailLaunchStageEnum::DEPRECATED => "DEPRECATED",
            EventingDetailLaunchStageEnum::PRIVATEPREVIEW => "PRIVATE_PREVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for EventingDetailLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(EventingDetailLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "PREVIEW" => Ok(EventingDetailLaunchStageEnum::PREVIEW),
           "GA" => Ok(EventingDetailLaunchStageEnum::GA),
           "DEPRECATED" => Ok(EventingDetailLaunchStageEnum::DEPRECATED),
           "PRIVATE_PREVIEW" => Ok(EventingDetailLaunchStageEnum::PRIVATEPREVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventingDetailLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventingDetailTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the event listener for a specific connector.
pub enum EventingDetailTypeEnum {
    

    /// Default value.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Webhook listener. e.g. Jira, Zendesk, Servicenow etc.,
    ///
    /// "WEBHOOK"
    #[serde(rename="WEBHOOK")]
    WEBHOOK,
    

    /// JMS Listener. e.g. IBM MQ, Rabbit MQ etc.,
    ///
    /// "JMS"
    #[serde(rename="JMS")]
    JMS,
}

impl AsRef<str> for EventingDetailTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventingDetailTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            EventingDetailTypeEnum::WEBHOOK => "WEBHOOK",
            EventingDetailTypeEnum::JMS => "JMS",
        }
    }
}

impl std::convert::TryFrom< &str> for EventingDetailTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(EventingDetailTypeEnum::TYPEUNSPECIFIED),
           "WEBHOOK" => Ok(EventingDetailTypeEnum::WEBHOOK),
           "JMS" => Ok(EventingDetailTypeEnum::JMS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventingDetailTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventingStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State.
pub enum EventingStatusStateEnum {
    

    /// Default state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Eventing is enabled and ready to receive events.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Eventing is not active due to an error.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Ingress endpoint required.
    ///
    /// "INGRESS_ENDPOINT_REQUIRED"
    #[serde(rename="INGRESS_ENDPOINT_REQUIRED")]
    INGRESSENDPOINTREQUIRED,
}

impl AsRef<str> for EventingStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventingStatusStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            EventingStatusStateEnum::ACTIVE => "ACTIVE",
            EventingStatusStateEnum::ERROR => "ERROR",
            EventingStatusStateEnum::INGRESSENDPOINTREQUIRED => "INGRESS_ENDPOINT_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for EventingStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(EventingStatusStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(EventingStatusStateEnum::ACTIVE),
           "ERROR" => Ok(EventingStatusStateEnum::ERROR),
           "INGRESS_ENDPOINT_REQUIRED" => Ok(EventingStatusStateEnum::INGRESSENDPOINTREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventingStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FieldDataTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The data type of the Field.
pub enum FieldDataTypeEnum {
    

    /// Data type is not specified.
    ///
    /// "DATA_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_TYPE_UNSPECIFIED")]
    DATATYPEUNSPECIFIED,
    

    /// DEPRECATED! Use DATA_TYPE_INTEGER.
    ///
    /// "DATA_TYPE_INT"
    #[serde(rename="DATA_TYPE_INT")]
    DATATYPEINT,
    

    /// Short integer(int16) data type.
    ///
    /// "DATA_TYPE_SMALLINT"
    #[serde(rename="DATA_TYPE_SMALLINT")]
    DATATYPESMALLINT,
    

    /// Double data type.
    ///
    /// "DATA_TYPE_DOUBLE"
    #[serde(rename="DATA_TYPE_DOUBLE")]
    DATATYPEDOUBLE,
    

    /// Date data type.
    ///
    /// "DATA_TYPE_DATE"
    #[serde(rename="DATA_TYPE_DATE")]
    DATATYPEDATE,
    

    /// DEPRECATED! Use DATA_TYPE_TIMESTAMP.
    ///
    /// "DATA_TYPE_DATETIME"
    #[serde(rename="DATA_TYPE_DATETIME")]
    DATATYPEDATETIME,
    

    /// Time data type.
    ///
    /// "DATA_TYPE_TIME"
    #[serde(rename="DATA_TYPE_TIME")]
    DATATYPETIME,
    

    /// DEPRECATED! Use DATA_TYPE_VARCHAR.
    ///
    /// "DATA_TYPE_STRING"
    #[serde(rename="DATA_TYPE_STRING")]
    DATATYPESTRING,
    

    /// DEPRECATED! Use DATA_TYPE_BIGINT.
    ///
    /// "DATA_TYPE_LONG"
    #[serde(rename="DATA_TYPE_LONG")]
    DATATYPELONG,
    

    /// Boolean data type.
    ///
    /// "DATA_TYPE_BOOLEAN"
    #[serde(rename="DATA_TYPE_BOOLEAN")]
    DATATYPEBOOLEAN,
    

    /// Decimal data type.
    ///
    /// "DATA_TYPE_DECIMAL"
    #[serde(rename="DATA_TYPE_DECIMAL")]
    DATATYPEDECIMAL,
    

    /// DEPRECATED! Use DATA_TYPE_VARCHAR.
    ///
    /// "DATA_TYPE_UUID"
    #[serde(rename="DATA_TYPE_UUID")]
    DATATYPEUUID,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_BLOB"
    #[serde(rename="DATA_TYPE_BLOB")]
    DATATYPEBLOB,
    

    /// Bit data type.
    ///
    /// "DATA_TYPE_BIT"
    #[serde(rename="DATA_TYPE_BIT")]
    DATATYPEBIT,
    

    /// Small integer(int8) data type.
    ///
    /// "DATA_TYPE_TINYINT"
    #[serde(rename="DATA_TYPE_TINYINT")]
    DATATYPETINYINT,
    

    /// Integer(int32) data type.
    ///
    /// "DATA_TYPE_INTEGER"
    #[serde(rename="DATA_TYPE_INTEGER")]
    DATATYPEINTEGER,
    

    /// Long integer(int64) data type.
    ///
    /// "DATA_TYPE_BIGINT"
    #[serde(rename="DATA_TYPE_BIGINT")]
    DATATYPEBIGINT,
    

    /// Float data type.
    ///
    /// "DATA_TYPE_FLOAT"
    #[serde(rename="DATA_TYPE_FLOAT")]
    DATATYPEFLOAT,
    

    /// Real data type.
    ///
    /// "DATA_TYPE_REAL"
    #[serde(rename="DATA_TYPE_REAL")]
    DATATYPEREAL,
    

    /// Numeric data type.
    ///
    /// "DATA_TYPE_NUMERIC"
    #[serde(rename="DATA_TYPE_NUMERIC")]
    DATATYPENUMERIC,
    

    /// Char data type.
    ///
    /// "DATA_TYPE_CHAR"
    #[serde(rename="DATA_TYPE_CHAR")]
    DATATYPECHAR,
    

    /// Varchar data type.
    ///
    /// "DATA_TYPE_VARCHAR"
    #[serde(rename="DATA_TYPE_VARCHAR")]
    DATATYPEVARCHAR,
    

    /// Longvarchar data type.
    ///
    /// "DATA_TYPE_LONGVARCHAR"
    #[serde(rename="DATA_TYPE_LONGVARCHAR")]
    DATATYPELONGVARCHAR,
    

    /// Timestamp data type.
    ///
    /// "DATA_TYPE_TIMESTAMP"
    #[serde(rename="DATA_TYPE_TIMESTAMP")]
    DATATYPETIMESTAMP,
    

    /// Nchar data type.
    ///
    /// "DATA_TYPE_NCHAR"
    #[serde(rename="DATA_TYPE_NCHAR")]
    DATATYPENCHAR,
    

    /// Nvarchar data type.
    ///
    /// "DATA_TYPE_NVARCHAR"
    #[serde(rename="DATA_TYPE_NVARCHAR")]
    DATATYPENVARCHAR,
    

    /// Longnvarchar data type.
    ///
    /// "DATA_TYPE_LONGNVARCHAR"
    #[serde(rename="DATA_TYPE_LONGNVARCHAR")]
    DATATYPELONGNVARCHAR,
    

    /// Null data type.
    ///
    /// "DATA_TYPE_NULL"
    #[serde(rename="DATA_TYPE_NULL")]
    DATATYPENULL,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_OTHER"
    #[serde(rename="DATA_TYPE_OTHER")]
    DATATYPEOTHER,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_JAVA_OBJECT"
    #[serde(rename="DATA_TYPE_JAVA_OBJECT")]
    DATATYPEJAVAOBJECT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_DISTINCT"
    #[serde(rename="DATA_TYPE_DISTINCT")]
    DATATYPEDISTINCT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_STRUCT"
    #[serde(rename="DATA_TYPE_STRUCT")]
    DATATYPESTRUCT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_ARRAY"
    #[serde(rename="DATA_TYPE_ARRAY")]
    DATATYPEARRAY,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_CLOB"
    #[serde(rename="DATA_TYPE_CLOB")]
    DATATYPECLOB,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_REF"
    #[serde(rename="DATA_TYPE_REF")]
    DATATYPEREF,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_DATALINK"
    #[serde(rename="DATA_TYPE_DATALINK")]
    DATATYPEDATALINK,
    

    /// UNSUPPORTED! Row id data type.
    ///
    /// "DATA_TYPE_ROWID"
    #[serde(rename="DATA_TYPE_ROWID")]
    DATATYPEROWID,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_BINARY"
    #[serde(rename="DATA_TYPE_BINARY")]
    DATATYPEBINARY,
    

    /// UNSUPPORTED! Variable binary data type.
    ///
    /// "DATA_TYPE_VARBINARY"
    #[serde(rename="DATA_TYPE_VARBINARY")]
    DATATYPEVARBINARY,
    

    /// UNSUPPORTED! Long variable binary data type.
    ///
    /// "DATA_TYPE_LONGVARBINARY"
    #[serde(rename="DATA_TYPE_LONGVARBINARY")]
    DATATYPELONGVARBINARY,
    

    /// UNSUPPORTED! NCLOB data type.
    ///
    /// "DATA_TYPE_NCLOB"
    #[serde(rename="DATA_TYPE_NCLOB")]
    DATATYPENCLOB,
    

    /// UNSUPPORTED! SQL XML data type is not supported.
    ///
    /// "DATA_TYPE_SQLXML"
    #[serde(rename="DATA_TYPE_SQLXML")]
    DATATYPESQLXML,
    

    /// UNSUPPORTED! Cursor reference type is not supported.
    ///
    /// "DATA_TYPE_REF_CURSOR"
    #[serde(rename="DATA_TYPE_REF_CURSOR")]
    DATATYPEREFCURSOR,
    

    /// UNSUPPORTED! Use TIME or TIMESTAMP instead.
    ///
    /// "DATA_TYPE_TIME_WITH_TIMEZONE"
    #[serde(rename="DATA_TYPE_TIME_WITH_TIMEZONE")]
    DATATYPETIMEWITHTIMEZONE,
    

    /// UNSUPPORTED! Use TIMESTAMP instead.
    ///
    /// "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE"
    #[serde(rename="DATA_TYPE_TIMESTAMP_WITH_TIMEZONE")]
    DATATYPETIMESTAMPWITHTIMEZONE,
}

impl AsRef<str> for FieldDataTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FieldDataTypeEnum::DATATYPEUNSPECIFIED => "DATA_TYPE_UNSPECIFIED",
            FieldDataTypeEnum::DATATYPEINT => "DATA_TYPE_INT",
            FieldDataTypeEnum::DATATYPESMALLINT => "DATA_TYPE_SMALLINT",
            FieldDataTypeEnum::DATATYPEDOUBLE => "DATA_TYPE_DOUBLE",
            FieldDataTypeEnum::DATATYPEDATE => "DATA_TYPE_DATE",
            FieldDataTypeEnum::DATATYPEDATETIME => "DATA_TYPE_DATETIME",
            FieldDataTypeEnum::DATATYPETIME => "DATA_TYPE_TIME",
            FieldDataTypeEnum::DATATYPESTRING => "DATA_TYPE_STRING",
            FieldDataTypeEnum::DATATYPELONG => "DATA_TYPE_LONG",
            FieldDataTypeEnum::DATATYPEBOOLEAN => "DATA_TYPE_BOOLEAN",
            FieldDataTypeEnum::DATATYPEDECIMAL => "DATA_TYPE_DECIMAL",
            FieldDataTypeEnum::DATATYPEUUID => "DATA_TYPE_UUID",
            FieldDataTypeEnum::DATATYPEBLOB => "DATA_TYPE_BLOB",
            FieldDataTypeEnum::DATATYPEBIT => "DATA_TYPE_BIT",
            FieldDataTypeEnum::DATATYPETINYINT => "DATA_TYPE_TINYINT",
            FieldDataTypeEnum::DATATYPEINTEGER => "DATA_TYPE_INTEGER",
            FieldDataTypeEnum::DATATYPEBIGINT => "DATA_TYPE_BIGINT",
            FieldDataTypeEnum::DATATYPEFLOAT => "DATA_TYPE_FLOAT",
            FieldDataTypeEnum::DATATYPEREAL => "DATA_TYPE_REAL",
            FieldDataTypeEnum::DATATYPENUMERIC => "DATA_TYPE_NUMERIC",
            FieldDataTypeEnum::DATATYPECHAR => "DATA_TYPE_CHAR",
            FieldDataTypeEnum::DATATYPEVARCHAR => "DATA_TYPE_VARCHAR",
            FieldDataTypeEnum::DATATYPELONGVARCHAR => "DATA_TYPE_LONGVARCHAR",
            FieldDataTypeEnum::DATATYPETIMESTAMP => "DATA_TYPE_TIMESTAMP",
            FieldDataTypeEnum::DATATYPENCHAR => "DATA_TYPE_NCHAR",
            FieldDataTypeEnum::DATATYPENVARCHAR => "DATA_TYPE_NVARCHAR",
            FieldDataTypeEnum::DATATYPELONGNVARCHAR => "DATA_TYPE_LONGNVARCHAR",
            FieldDataTypeEnum::DATATYPENULL => "DATA_TYPE_NULL",
            FieldDataTypeEnum::DATATYPEOTHER => "DATA_TYPE_OTHER",
            FieldDataTypeEnum::DATATYPEJAVAOBJECT => "DATA_TYPE_JAVA_OBJECT",
            FieldDataTypeEnum::DATATYPEDISTINCT => "DATA_TYPE_DISTINCT",
            FieldDataTypeEnum::DATATYPESTRUCT => "DATA_TYPE_STRUCT",
            FieldDataTypeEnum::DATATYPEARRAY => "DATA_TYPE_ARRAY",
            FieldDataTypeEnum::DATATYPECLOB => "DATA_TYPE_CLOB",
            FieldDataTypeEnum::DATATYPEREF => "DATA_TYPE_REF",
            FieldDataTypeEnum::DATATYPEDATALINK => "DATA_TYPE_DATALINK",
            FieldDataTypeEnum::DATATYPEROWID => "DATA_TYPE_ROWID",
            FieldDataTypeEnum::DATATYPEBINARY => "DATA_TYPE_BINARY",
            FieldDataTypeEnum::DATATYPEVARBINARY => "DATA_TYPE_VARBINARY",
            FieldDataTypeEnum::DATATYPELONGVARBINARY => "DATA_TYPE_LONGVARBINARY",
            FieldDataTypeEnum::DATATYPENCLOB => "DATA_TYPE_NCLOB",
            FieldDataTypeEnum::DATATYPESQLXML => "DATA_TYPE_SQLXML",
            FieldDataTypeEnum::DATATYPEREFCURSOR => "DATA_TYPE_REF_CURSOR",
            FieldDataTypeEnum::DATATYPETIMEWITHTIMEZONE => "DATA_TYPE_TIME_WITH_TIMEZONE",
            FieldDataTypeEnum::DATATYPETIMESTAMPWITHTIMEZONE => "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE",
        }
    }
}

impl std::convert::TryFrom< &str> for FieldDataTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_TYPE_UNSPECIFIED" => Ok(FieldDataTypeEnum::DATATYPEUNSPECIFIED),
           "DATA_TYPE_INT" => Ok(FieldDataTypeEnum::DATATYPEINT),
           "DATA_TYPE_SMALLINT" => Ok(FieldDataTypeEnum::DATATYPESMALLINT),
           "DATA_TYPE_DOUBLE" => Ok(FieldDataTypeEnum::DATATYPEDOUBLE),
           "DATA_TYPE_DATE" => Ok(FieldDataTypeEnum::DATATYPEDATE),
           "DATA_TYPE_DATETIME" => Ok(FieldDataTypeEnum::DATATYPEDATETIME),
           "DATA_TYPE_TIME" => Ok(FieldDataTypeEnum::DATATYPETIME),
           "DATA_TYPE_STRING" => Ok(FieldDataTypeEnum::DATATYPESTRING),
           "DATA_TYPE_LONG" => Ok(FieldDataTypeEnum::DATATYPELONG),
           "DATA_TYPE_BOOLEAN" => Ok(FieldDataTypeEnum::DATATYPEBOOLEAN),
           "DATA_TYPE_DECIMAL" => Ok(FieldDataTypeEnum::DATATYPEDECIMAL),
           "DATA_TYPE_UUID" => Ok(FieldDataTypeEnum::DATATYPEUUID),
           "DATA_TYPE_BLOB" => Ok(FieldDataTypeEnum::DATATYPEBLOB),
           "DATA_TYPE_BIT" => Ok(FieldDataTypeEnum::DATATYPEBIT),
           "DATA_TYPE_TINYINT" => Ok(FieldDataTypeEnum::DATATYPETINYINT),
           "DATA_TYPE_INTEGER" => Ok(FieldDataTypeEnum::DATATYPEINTEGER),
           "DATA_TYPE_BIGINT" => Ok(FieldDataTypeEnum::DATATYPEBIGINT),
           "DATA_TYPE_FLOAT" => Ok(FieldDataTypeEnum::DATATYPEFLOAT),
           "DATA_TYPE_REAL" => Ok(FieldDataTypeEnum::DATATYPEREAL),
           "DATA_TYPE_NUMERIC" => Ok(FieldDataTypeEnum::DATATYPENUMERIC),
           "DATA_TYPE_CHAR" => Ok(FieldDataTypeEnum::DATATYPECHAR),
           "DATA_TYPE_VARCHAR" => Ok(FieldDataTypeEnum::DATATYPEVARCHAR),
           "DATA_TYPE_LONGVARCHAR" => Ok(FieldDataTypeEnum::DATATYPELONGVARCHAR),
           "DATA_TYPE_TIMESTAMP" => Ok(FieldDataTypeEnum::DATATYPETIMESTAMP),
           "DATA_TYPE_NCHAR" => Ok(FieldDataTypeEnum::DATATYPENCHAR),
           "DATA_TYPE_NVARCHAR" => Ok(FieldDataTypeEnum::DATATYPENVARCHAR),
           "DATA_TYPE_LONGNVARCHAR" => Ok(FieldDataTypeEnum::DATATYPELONGNVARCHAR),
           "DATA_TYPE_NULL" => Ok(FieldDataTypeEnum::DATATYPENULL),
           "DATA_TYPE_OTHER" => Ok(FieldDataTypeEnum::DATATYPEOTHER),
           "DATA_TYPE_JAVA_OBJECT" => Ok(FieldDataTypeEnum::DATATYPEJAVAOBJECT),
           "DATA_TYPE_DISTINCT" => Ok(FieldDataTypeEnum::DATATYPEDISTINCT),
           "DATA_TYPE_STRUCT" => Ok(FieldDataTypeEnum::DATATYPESTRUCT),
           "DATA_TYPE_ARRAY" => Ok(FieldDataTypeEnum::DATATYPEARRAY),
           "DATA_TYPE_CLOB" => Ok(FieldDataTypeEnum::DATATYPECLOB),
           "DATA_TYPE_REF" => Ok(FieldDataTypeEnum::DATATYPEREF),
           "DATA_TYPE_DATALINK" => Ok(FieldDataTypeEnum::DATATYPEDATALINK),
           "DATA_TYPE_ROWID" => Ok(FieldDataTypeEnum::DATATYPEROWID),
           "DATA_TYPE_BINARY" => Ok(FieldDataTypeEnum::DATATYPEBINARY),
           "DATA_TYPE_VARBINARY" => Ok(FieldDataTypeEnum::DATATYPEVARBINARY),
           "DATA_TYPE_LONGVARBINARY" => Ok(FieldDataTypeEnum::DATATYPELONGVARBINARY),
           "DATA_TYPE_NCLOB" => Ok(FieldDataTypeEnum::DATATYPENCLOB),
           "DATA_TYPE_SQLXML" => Ok(FieldDataTypeEnum::DATATYPESQLXML),
           "DATA_TYPE_REF_CURSOR" => Ok(FieldDataTypeEnum::DATATYPEREFCURSOR),
           "DATA_TYPE_TIME_WITH_TIMEZONE" => Ok(FieldDataTypeEnum::DATATYPETIMEWITHTIMEZONE),
           "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE" => Ok(FieldDataTypeEnum::DATATYPETIMESTAMPWITHTIMEZONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FieldDataTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FieldComparisonComparatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Comparator to use for comparing the field value.
pub enum FieldComparisonComparatorEnum {
    

    /// The default value.
    ///
    /// "COMPARATOR_UNSPECIFIED"
    #[serde(rename="COMPARATOR_UNSPECIFIED")]
    COMPARATORUNSPECIFIED,
    

    /// The field value must be equal to the specified value.
    ///
    /// "EQUALS"
    #[serde(rename="EQUALS")]
    EQUALS,
    

    /// The field value must not be equal to the specified value.
    ///
    /// "NOT_EQUALS"
    #[serde(rename="NOT_EQUALS")]
    NOTEQUALS,
}

impl AsRef<str> for FieldComparisonComparatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FieldComparisonComparatorEnum::COMPARATORUNSPECIFIED => "COMPARATOR_UNSPECIFIED",
            FieldComparisonComparatorEnum::EQUALS => "EQUALS",
            FieldComparisonComparatorEnum::NOTEQUALS => "NOT_EQUALS",
        }
    }
}

impl std::convert::TryFrom< &str> for FieldComparisonComparatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPARATOR_UNSPECIFIED" => Ok(FieldComparisonComparatorEnum::COMPARATORUNSPECIFIED),
           "EQUALS" => Ok(FieldComparisonComparatorEnum::EQUALS),
           "NOT_EQUALS" => Ok(FieldComparisonComparatorEnum::NOTEQUALS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FieldComparisonComparatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InputParameterDataTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The data type of the Parameter.
pub enum InputParameterDataTypeEnum {
    

    /// Data type is not specified.
    ///
    /// "DATA_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_TYPE_UNSPECIFIED")]
    DATATYPEUNSPECIFIED,
    

    /// DEPRECATED! Use DATA_TYPE_INTEGER.
    ///
    /// "DATA_TYPE_INT"
    #[serde(rename="DATA_TYPE_INT")]
    DATATYPEINT,
    

    /// Short integer(int16) data type.
    ///
    /// "DATA_TYPE_SMALLINT"
    #[serde(rename="DATA_TYPE_SMALLINT")]
    DATATYPESMALLINT,
    

    /// Double data type.
    ///
    /// "DATA_TYPE_DOUBLE"
    #[serde(rename="DATA_TYPE_DOUBLE")]
    DATATYPEDOUBLE,
    

    /// Date data type.
    ///
    /// "DATA_TYPE_DATE"
    #[serde(rename="DATA_TYPE_DATE")]
    DATATYPEDATE,
    

    /// DEPRECATED! Use DATA_TYPE_TIMESTAMP.
    ///
    /// "DATA_TYPE_DATETIME"
    #[serde(rename="DATA_TYPE_DATETIME")]
    DATATYPEDATETIME,
    

    /// Time data type.
    ///
    /// "DATA_TYPE_TIME"
    #[serde(rename="DATA_TYPE_TIME")]
    DATATYPETIME,
    

    /// DEPRECATED! Use DATA_TYPE_VARCHAR.
    ///
    /// "DATA_TYPE_STRING"
    #[serde(rename="DATA_TYPE_STRING")]
    DATATYPESTRING,
    

    /// DEPRECATED! Use DATA_TYPE_BIGINT.
    ///
    /// "DATA_TYPE_LONG"
    #[serde(rename="DATA_TYPE_LONG")]
    DATATYPELONG,
    

    /// Boolean data type.
    ///
    /// "DATA_TYPE_BOOLEAN"
    #[serde(rename="DATA_TYPE_BOOLEAN")]
    DATATYPEBOOLEAN,
    

    /// Decimal data type.
    ///
    /// "DATA_TYPE_DECIMAL"
    #[serde(rename="DATA_TYPE_DECIMAL")]
    DATATYPEDECIMAL,
    

    /// DEPRECATED! Use DATA_TYPE_VARCHAR.
    ///
    /// "DATA_TYPE_UUID"
    #[serde(rename="DATA_TYPE_UUID")]
    DATATYPEUUID,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_BLOB"
    #[serde(rename="DATA_TYPE_BLOB")]
    DATATYPEBLOB,
    

    /// Bit data type.
    ///
    /// "DATA_TYPE_BIT"
    #[serde(rename="DATA_TYPE_BIT")]
    DATATYPEBIT,
    

    /// Small integer(int8) data type.
    ///
    /// "DATA_TYPE_TINYINT"
    #[serde(rename="DATA_TYPE_TINYINT")]
    DATATYPETINYINT,
    

    /// Integer(int32) data type.
    ///
    /// "DATA_TYPE_INTEGER"
    #[serde(rename="DATA_TYPE_INTEGER")]
    DATATYPEINTEGER,
    

    /// Long integer(int64) data type.
    ///
    /// "DATA_TYPE_BIGINT"
    #[serde(rename="DATA_TYPE_BIGINT")]
    DATATYPEBIGINT,
    

    /// Float data type.
    ///
    /// "DATA_TYPE_FLOAT"
    #[serde(rename="DATA_TYPE_FLOAT")]
    DATATYPEFLOAT,
    

    /// Real data type.
    ///
    /// "DATA_TYPE_REAL"
    #[serde(rename="DATA_TYPE_REAL")]
    DATATYPEREAL,
    

    /// Numeric data type.
    ///
    /// "DATA_TYPE_NUMERIC"
    #[serde(rename="DATA_TYPE_NUMERIC")]
    DATATYPENUMERIC,
    

    /// Char data type.
    ///
    /// "DATA_TYPE_CHAR"
    #[serde(rename="DATA_TYPE_CHAR")]
    DATATYPECHAR,
    

    /// Varchar data type.
    ///
    /// "DATA_TYPE_VARCHAR"
    #[serde(rename="DATA_TYPE_VARCHAR")]
    DATATYPEVARCHAR,
    

    /// Longvarchar data type.
    ///
    /// "DATA_TYPE_LONGVARCHAR"
    #[serde(rename="DATA_TYPE_LONGVARCHAR")]
    DATATYPELONGVARCHAR,
    

    /// Timestamp data type.
    ///
    /// "DATA_TYPE_TIMESTAMP"
    #[serde(rename="DATA_TYPE_TIMESTAMP")]
    DATATYPETIMESTAMP,
    

    /// Nchar data type.
    ///
    /// "DATA_TYPE_NCHAR"
    #[serde(rename="DATA_TYPE_NCHAR")]
    DATATYPENCHAR,
    

    /// Nvarchar data type.
    ///
    /// "DATA_TYPE_NVARCHAR"
    #[serde(rename="DATA_TYPE_NVARCHAR")]
    DATATYPENVARCHAR,
    

    /// Longnvarchar data type.
    ///
    /// "DATA_TYPE_LONGNVARCHAR"
    #[serde(rename="DATA_TYPE_LONGNVARCHAR")]
    DATATYPELONGNVARCHAR,
    

    /// Null data type.
    ///
    /// "DATA_TYPE_NULL"
    #[serde(rename="DATA_TYPE_NULL")]
    DATATYPENULL,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_OTHER"
    #[serde(rename="DATA_TYPE_OTHER")]
    DATATYPEOTHER,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_JAVA_OBJECT"
    #[serde(rename="DATA_TYPE_JAVA_OBJECT")]
    DATATYPEJAVAOBJECT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_DISTINCT"
    #[serde(rename="DATA_TYPE_DISTINCT")]
    DATATYPEDISTINCT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_STRUCT"
    #[serde(rename="DATA_TYPE_STRUCT")]
    DATATYPESTRUCT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_ARRAY"
    #[serde(rename="DATA_TYPE_ARRAY")]
    DATATYPEARRAY,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_CLOB"
    #[serde(rename="DATA_TYPE_CLOB")]
    DATATYPECLOB,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_REF"
    #[serde(rename="DATA_TYPE_REF")]
    DATATYPEREF,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_DATALINK"
    #[serde(rename="DATA_TYPE_DATALINK")]
    DATATYPEDATALINK,
    

    /// UNSUPPORTED! Row id data type.
    ///
    /// "DATA_TYPE_ROWID"
    #[serde(rename="DATA_TYPE_ROWID")]
    DATATYPEROWID,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_BINARY"
    #[serde(rename="DATA_TYPE_BINARY")]
    DATATYPEBINARY,
    

    /// UNSUPPORTED! Variable binary data type.
    ///
    /// "DATA_TYPE_VARBINARY"
    #[serde(rename="DATA_TYPE_VARBINARY")]
    DATATYPEVARBINARY,
    

    /// UNSUPPORTED! Long variable binary data type.
    ///
    /// "DATA_TYPE_LONGVARBINARY"
    #[serde(rename="DATA_TYPE_LONGVARBINARY")]
    DATATYPELONGVARBINARY,
    

    /// UNSUPPORTED! NCLOB data type.
    ///
    /// "DATA_TYPE_NCLOB"
    #[serde(rename="DATA_TYPE_NCLOB")]
    DATATYPENCLOB,
    

    /// UNSUPPORTED! SQL XML data type is not supported.
    ///
    /// "DATA_TYPE_SQLXML"
    #[serde(rename="DATA_TYPE_SQLXML")]
    DATATYPESQLXML,
    

    /// UNSUPPORTED! Cursor reference type is not supported.
    ///
    /// "DATA_TYPE_REF_CURSOR"
    #[serde(rename="DATA_TYPE_REF_CURSOR")]
    DATATYPEREFCURSOR,
    

    /// UNSUPPORTED! Use TIME or TIMESTAMP instead.
    ///
    /// "DATA_TYPE_TIME_WITH_TIMEZONE"
    #[serde(rename="DATA_TYPE_TIME_WITH_TIMEZONE")]
    DATATYPETIMEWITHTIMEZONE,
    

    /// UNSUPPORTED! Use TIMESTAMP instead.
    ///
    /// "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE"
    #[serde(rename="DATA_TYPE_TIMESTAMP_WITH_TIMEZONE")]
    DATATYPETIMESTAMPWITHTIMEZONE,
}

impl AsRef<str> for InputParameterDataTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InputParameterDataTypeEnum::DATATYPEUNSPECIFIED => "DATA_TYPE_UNSPECIFIED",
            InputParameterDataTypeEnum::DATATYPEINT => "DATA_TYPE_INT",
            InputParameterDataTypeEnum::DATATYPESMALLINT => "DATA_TYPE_SMALLINT",
            InputParameterDataTypeEnum::DATATYPEDOUBLE => "DATA_TYPE_DOUBLE",
            InputParameterDataTypeEnum::DATATYPEDATE => "DATA_TYPE_DATE",
            InputParameterDataTypeEnum::DATATYPEDATETIME => "DATA_TYPE_DATETIME",
            InputParameterDataTypeEnum::DATATYPETIME => "DATA_TYPE_TIME",
            InputParameterDataTypeEnum::DATATYPESTRING => "DATA_TYPE_STRING",
            InputParameterDataTypeEnum::DATATYPELONG => "DATA_TYPE_LONG",
            InputParameterDataTypeEnum::DATATYPEBOOLEAN => "DATA_TYPE_BOOLEAN",
            InputParameterDataTypeEnum::DATATYPEDECIMAL => "DATA_TYPE_DECIMAL",
            InputParameterDataTypeEnum::DATATYPEUUID => "DATA_TYPE_UUID",
            InputParameterDataTypeEnum::DATATYPEBLOB => "DATA_TYPE_BLOB",
            InputParameterDataTypeEnum::DATATYPEBIT => "DATA_TYPE_BIT",
            InputParameterDataTypeEnum::DATATYPETINYINT => "DATA_TYPE_TINYINT",
            InputParameterDataTypeEnum::DATATYPEINTEGER => "DATA_TYPE_INTEGER",
            InputParameterDataTypeEnum::DATATYPEBIGINT => "DATA_TYPE_BIGINT",
            InputParameterDataTypeEnum::DATATYPEFLOAT => "DATA_TYPE_FLOAT",
            InputParameterDataTypeEnum::DATATYPEREAL => "DATA_TYPE_REAL",
            InputParameterDataTypeEnum::DATATYPENUMERIC => "DATA_TYPE_NUMERIC",
            InputParameterDataTypeEnum::DATATYPECHAR => "DATA_TYPE_CHAR",
            InputParameterDataTypeEnum::DATATYPEVARCHAR => "DATA_TYPE_VARCHAR",
            InputParameterDataTypeEnum::DATATYPELONGVARCHAR => "DATA_TYPE_LONGVARCHAR",
            InputParameterDataTypeEnum::DATATYPETIMESTAMP => "DATA_TYPE_TIMESTAMP",
            InputParameterDataTypeEnum::DATATYPENCHAR => "DATA_TYPE_NCHAR",
            InputParameterDataTypeEnum::DATATYPENVARCHAR => "DATA_TYPE_NVARCHAR",
            InputParameterDataTypeEnum::DATATYPELONGNVARCHAR => "DATA_TYPE_LONGNVARCHAR",
            InputParameterDataTypeEnum::DATATYPENULL => "DATA_TYPE_NULL",
            InputParameterDataTypeEnum::DATATYPEOTHER => "DATA_TYPE_OTHER",
            InputParameterDataTypeEnum::DATATYPEJAVAOBJECT => "DATA_TYPE_JAVA_OBJECT",
            InputParameterDataTypeEnum::DATATYPEDISTINCT => "DATA_TYPE_DISTINCT",
            InputParameterDataTypeEnum::DATATYPESTRUCT => "DATA_TYPE_STRUCT",
            InputParameterDataTypeEnum::DATATYPEARRAY => "DATA_TYPE_ARRAY",
            InputParameterDataTypeEnum::DATATYPECLOB => "DATA_TYPE_CLOB",
            InputParameterDataTypeEnum::DATATYPEREF => "DATA_TYPE_REF",
            InputParameterDataTypeEnum::DATATYPEDATALINK => "DATA_TYPE_DATALINK",
            InputParameterDataTypeEnum::DATATYPEROWID => "DATA_TYPE_ROWID",
            InputParameterDataTypeEnum::DATATYPEBINARY => "DATA_TYPE_BINARY",
            InputParameterDataTypeEnum::DATATYPEVARBINARY => "DATA_TYPE_VARBINARY",
            InputParameterDataTypeEnum::DATATYPELONGVARBINARY => "DATA_TYPE_LONGVARBINARY",
            InputParameterDataTypeEnum::DATATYPENCLOB => "DATA_TYPE_NCLOB",
            InputParameterDataTypeEnum::DATATYPESQLXML => "DATA_TYPE_SQLXML",
            InputParameterDataTypeEnum::DATATYPEREFCURSOR => "DATA_TYPE_REF_CURSOR",
            InputParameterDataTypeEnum::DATATYPETIMEWITHTIMEZONE => "DATA_TYPE_TIME_WITH_TIMEZONE",
            InputParameterDataTypeEnum::DATATYPETIMESTAMPWITHTIMEZONE => "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE",
        }
    }
}

impl std::convert::TryFrom< &str> for InputParameterDataTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_TYPE_UNSPECIFIED" => Ok(InputParameterDataTypeEnum::DATATYPEUNSPECIFIED),
           "DATA_TYPE_INT" => Ok(InputParameterDataTypeEnum::DATATYPEINT),
           "DATA_TYPE_SMALLINT" => Ok(InputParameterDataTypeEnum::DATATYPESMALLINT),
           "DATA_TYPE_DOUBLE" => Ok(InputParameterDataTypeEnum::DATATYPEDOUBLE),
           "DATA_TYPE_DATE" => Ok(InputParameterDataTypeEnum::DATATYPEDATE),
           "DATA_TYPE_DATETIME" => Ok(InputParameterDataTypeEnum::DATATYPEDATETIME),
           "DATA_TYPE_TIME" => Ok(InputParameterDataTypeEnum::DATATYPETIME),
           "DATA_TYPE_STRING" => Ok(InputParameterDataTypeEnum::DATATYPESTRING),
           "DATA_TYPE_LONG" => Ok(InputParameterDataTypeEnum::DATATYPELONG),
           "DATA_TYPE_BOOLEAN" => Ok(InputParameterDataTypeEnum::DATATYPEBOOLEAN),
           "DATA_TYPE_DECIMAL" => Ok(InputParameterDataTypeEnum::DATATYPEDECIMAL),
           "DATA_TYPE_UUID" => Ok(InputParameterDataTypeEnum::DATATYPEUUID),
           "DATA_TYPE_BLOB" => Ok(InputParameterDataTypeEnum::DATATYPEBLOB),
           "DATA_TYPE_BIT" => Ok(InputParameterDataTypeEnum::DATATYPEBIT),
           "DATA_TYPE_TINYINT" => Ok(InputParameterDataTypeEnum::DATATYPETINYINT),
           "DATA_TYPE_INTEGER" => Ok(InputParameterDataTypeEnum::DATATYPEINTEGER),
           "DATA_TYPE_BIGINT" => Ok(InputParameterDataTypeEnum::DATATYPEBIGINT),
           "DATA_TYPE_FLOAT" => Ok(InputParameterDataTypeEnum::DATATYPEFLOAT),
           "DATA_TYPE_REAL" => Ok(InputParameterDataTypeEnum::DATATYPEREAL),
           "DATA_TYPE_NUMERIC" => Ok(InputParameterDataTypeEnum::DATATYPENUMERIC),
           "DATA_TYPE_CHAR" => Ok(InputParameterDataTypeEnum::DATATYPECHAR),
           "DATA_TYPE_VARCHAR" => Ok(InputParameterDataTypeEnum::DATATYPEVARCHAR),
           "DATA_TYPE_LONGVARCHAR" => Ok(InputParameterDataTypeEnum::DATATYPELONGVARCHAR),
           "DATA_TYPE_TIMESTAMP" => Ok(InputParameterDataTypeEnum::DATATYPETIMESTAMP),
           "DATA_TYPE_NCHAR" => Ok(InputParameterDataTypeEnum::DATATYPENCHAR),
           "DATA_TYPE_NVARCHAR" => Ok(InputParameterDataTypeEnum::DATATYPENVARCHAR),
           "DATA_TYPE_LONGNVARCHAR" => Ok(InputParameterDataTypeEnum::DATATYPELONGNVARCHAR),
           "DATA_TYPE_NULL" => Ok(InputParameterDataTypeEnum::DATATYPENULL),
           "DATA_TYPE_OTHER" => Ok(InputParameterDataTypeEnum::DATATYPEOTHER),
           "DATA_TYPE_JAVA_OBJECT" => Ok(InputParameterDataTypeEnum::DATATYPEJAVAOBJECT),
           "DATA_TYPE_DISTINCT" => Ok(InputParameterDataTypeEnum::DATATYPEDISTINCT),
           "DATA_TYPE_STRUCT" => Ok(InputParameterDataTypeEnum::DATATYPESTRUCT),
           "DATA_TYPE_ARRAY" => Ok(InputParameterDataTypeEnum::DATATYPEARRAY),
           "DATA_TYPE_CLOB" => Ok(InputParameterDataTypeEnum::DATATYPECLOB),
           "DATA_TYPE_REF" => Ok(InputParameterDataTypeEnum::DATATYPEREF),
           "DATA_TYPE_DATALINK" => Ok(InputParameterDataTypeEnum::DATATYPEDATALINK),
           "DATA_TYPE_ROWID" => Ok(InputParameterDataTypeEnum::DATATYPEROWID),
           "DATA_TYPE_BINARY" => Ok(InputParameterDataTypeEnum::DATATYPEBINARY),
           "DATA_TYPE_VARBINARY" => Ok(InputParameterDataTypeEnum::DATATYPEVARBINARY),
           "DATA_TYPE_LONGVARBINARY" => Ok(InputParameterDataTypeEnum::DATATYPELONGVARBINARY),
           "DATA_TYPE_NCLOB" => Ok(InputParameterDataTypeEnum::DATATYPENCLOB),
           "DATA_TYPE_SQLXML" => Ok(InputParameterDataTypeEnum::DATATYPESQLXML),
           "DATA_TYPE_REF_CURSOR" => Ok(InputParameterDataTypeEnum::DATATYPEREFCURSOR),
           "DATA_TYPE_TIME_WITH_TIMEZONE" => Ok(InputParameterDataTypeEnum::DATATYPETIMEWITHTIMEZONE),
           "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE" => Ok(InputParameterDataTypeEnum::DATATYPETIMESTAMPWITHTIMEZONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InputParameterDataTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JMTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Type of the JMS Source. i.e. Queue or Topic
pub enum JMTypeEnum {
    

    /// Default state.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// JMS Queue.
    ///
    /// "QUEUE"
    #[serde(rename="QUEUE")]
    QUEUE,
    

    /// JMS Topic.
    ///
    /// "TOPIC"
    #[serde(rename="TOPIC")]
    TOPIC,
}

impl AsRef<str> for JMTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JMTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            JMTypeEnum::QUEUE => "QUEUE",
            JMTypeEnum::TOPIC => "TOPIC",
        }
    }
}

impl std::convert::TryFrom< &str> for JMTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(JMTypeEnum::TYPEUNSPECIFIED),
           "QUEUE" => Ok(JMTypeEnum::QUEUE),
           "TOPIC" => Ok(JMTypeEnum::TOPIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JMTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JsonSchemaJdbcTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// JDBC datatype of the field.
pub enum JsonSchemaJdbcTypeEnum {
    

    /// Data type is not specified.
    ///
    /// "DATA_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_TYPE_UNSPECIFIED")]
    DATATYPEUNSPECIFIED,
    

    /// DEPRECATED! Use DATA_TYPE_INTEGER.
    ///
    /// "DATA_TYPE_INT"
    #[serde(rename="DATA_TYPE_INT")]
    DATATYPEINT,
    

    /// Short integer(int16) data type.
    ///
    /// "DATA_TYPE_SMALLINT"
    #[serde(rename="DATA_TYPE_SMALLINT")]
    DATATYPESMALLINT,
    

    /// Double data type.
    ///
    /// "DATA_TYPE_DOUBLE"
    #[serde(rename="DATA_TYPE_DOUBLE")]
    DATATYPEDOUBLE,
    

    /// Date data type.
    ///
    /// "DATA_TYPE_DATE"
    #[serde(rename="DATA_TYPE_DATE")]
    DATATYPEDATE,
    

    /// DEPRECATED! Use DATA_TYPE_TIMESTAMP.
    ///
    /// "DATA_TYPE_DATETIME"
    #[serde(rename="DATA_TYPE_DATETIME")]
    DATATYPEDATETIME,
    

    /// Time data type.
    ///
    /// "DATA_TYPE_TIME"
    #[serde(rename="DATA_TYPE_TIME")]
    DATATYPETIME,
    

    /// DEPRECATED! Use DATA_TYPE_VARCHAR.
    ///
    /// "DATA_TYPE_STRING"
    #[serde(rename="DATA_TYPE_STRING")]
    DATATYPESTRING,
    

    /// DEPRECATED! Use DATA_TYPE_BIGINT.
    ///
    /// "DATA_TYPE_LONG"
    #[serde(rename="DATA_TYPE_LONG")]
    DATATYPELONG,
    

    /// Boolean data type.
    ///
    /// "DATA_TYPE_BOOLEAN"
    #[serde(rename="DATA_TYPE_BOOLEAN")]
    DATATYPEBOOLEAN,
    

    /// Decimal data type.
    ///
    /// "DATA_TYPE_DECIMAL"
    #[serde(rename="DATA_TYPE_DECIMAL")]
    DATATYPEDECIMAL,
    

    /// DEPRECATED! Use DATA_TYPE_VARCHAR.
    ///
    /// "DATA_TYPE_UUID"
    #[serde(rename="DATA_TYPE_UUID")]
    DATATYPEUUID,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_BLOB"
    #[serde(rename="DATA_TYPE_BLOB")]
    DATATYPEBLOB,
    

    /// Bit data type.
    ///
    /// "DATA_TYPE_BIT"
    #[serde(rename="DATA_TYPE_BIT")]
    DATATYPEBIT,
    

    /// Small integer(int8) data type.
    ///
    /// "DATA_TYPE_TINYINT"
    #[serde(rename="DATA_TYPE_TINYINT")]
    DATATYPETINYINT,
    

    /// Integer(int32) data type.
    ///
    /// "DATA_TYPE_INTEGER"
    #[serde(rename="DATA_TYPE_INTEGER")]
    DATATYPEINTEGER,
    

    /// Long integer(int64) data type.
    ///
    /// "DATA_TYPE_BIGINT"
    #[serde(rename="DATA_TYPE_BIGINT")]
    DATATYPEBIGINT,
    

    /// Float data type.
    ///
    /// "DATA_TYPE_FLOAT"
    #[serde(rename="DATA_TYPE_FLOAT")]
    DATATYPEFLOAT,
    

    /// Real data type.
    ///
    /// "DATA_TYPE_REAL"
    #[serde(rename="DATA_TYPE_REAL")]
    DATATYPEREAL,
    

    /// Numeric data type.
    ///
    /// "DATA_TYPE_NUMERIC"
    #[serde(rename="DATA_TYPE_NUMERIC")]
    DATATYPENUMERIC,
    

    /// Char data type.
    ///
    /// "DATA_TYPE_CHAR"
    #[serde(rename="DATA_TYPE_CHAR")]
    DATATYPECHAR,
    

    /// Varchar data type.
    ///
    /// "DATA_TYPE_VARCHAR"
    #[serde(rename="DATA_TYPE_VARCHAR")]
    DATATYPEVARCHAR,
    

    /// Longvarchar data type.
    ///
    /// "DATA_TYPE_LONGVARCHAR"
    #[serde(rename="DATA_TYPE_LONGVARCHAR")]
    DATATYPELONGVARCHAR,
    

    /// Timestamp data type.
    ///
    /// "DATA_TYPE_TIMESTAMP"
    #[serde(rename="DATA_TYPE_TIMESTAMP")]
    DATATYPETIMESTAMP,
    

    /// Nchar data type.
    ///
    /// "DATA_TYPE_NCHAR"
    #[serde(rename="DATA_TYPE_NCHAR")]
    DATATYPENCHAR,
    

    /// Nvarchar data type.
    ///
    /// "DATA_TYPE_NVARCHAR"
    #[serde(rename="DATA_TYPE_NVARCHAR")]
    DATATYPENVARCHAR,
    

    /// Longnvarchar data type.
    ///
    /// "DATA_TYPE_LONGNVARCHAR"
    #[serde(rename="DATA_TYPE_LONGNVARCHAR")]
    DATATYPELONGNVARCHAR,
    

    /// Null data type.
    ///
    /// "DATA_TYPE_NULL"
    #[serde(rename="DATA_TYPE_NULL")]
    DATATYPENULL,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_OTHER"
    #[serde(rename="DATA_TYPE_OTHER")]
    DATATYPEOTHER,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_JAVA_OBJECT"
    #[serde(rename="DATA_TYPE_JAVA_OBJECT")]
    DATATYPEJAVAOBJECT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_DISTINCT"
    #[serde(rename="DATA_TYPE_DISTINCT")]
    DATATYPEDISTINCT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_STRUCT"
    #[serde(rename="DATA_TYPE_STRUCT")]
    DATATYPESTRUCT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_ARRAY"
    #[serde(rename="DATA_TYPE_ARRAY")]
    DATATYPEARRAY,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_CLOB"
    #[serde(rename="DATA_TYPE_CLOB")]
    DATATYPECLOB,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_REF"
    #[serde(rename="DATA_TYPE_REF")]
    DATATYPEREF,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_DATALINK"
    #[serde(rename="DATA_TYPE_DATALINK")]
    DATATYPEDATALINK,
    

    /// UNSUPPORTED! Row id data type.
    ///
    /// "DATA_TYPE_ROWID"
    #[serde(rename="DATA_TYPE_ROWID")]
    DATATYPEROWID,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_BINARY"
    #[serde(rename="DATA_TYPE_BINARY")]
    DATATYPEBINARY,
    

    /// UNSUPPORTED! Variable binary data type.
    ///
    /// "DATA_TYPE_VARBINARY"
    #[serde(rename="DATA_TYPE_VARBINARY")]
    DATATYPEVARBINARY,
    

    /// UNSUPPORTED! Long variable binary data type.
    ///
    /// "DATA_TYPE_LONGVARBINARY"
    #[serde(rename="DATA_TYPE_LONGVARBINARY")]
    DATATYPELONGVARBINARY,
    

    /// UNSUPPORTED! NCLOB data type.
    ///
    /// "DATA_TYPE_NCLOB"
    #[serde(rename="DATA_TYPE_NCLOB")]
    DATATYPENCLOB,
    

    /// UNSUPPORTED! SQL XML data type is not supported.
    ///
    /// "DATA_TYPE_SQLXML"
    #[serde(rename="DATA_TYPE_SQLXML")]
    DATATYPESQLXML,
    

    /// UNSUPPORTED! Cursor reference type is not supported.
    ///
    /// "DATA_TYPE_REF_CURSOR"
    #[serde(rename="DATA_TYPE_REF_CURSOR")]
    DATATYPEREFCURSOR,
    

    /// UNSUPPORTED! Use TIME or TIMESTAMP instead.
    ///
    /// "DATA_TYPE_TIME_WITH_TIMEZONE"
    #[serde(rename="DATA_TYPE_TIME_WITH_TIMEZONE")]
    DATATYPETIMEWITHTIMEZONE,
    

    /// UNSUPPORTED! Use TIMESTAMP instead.
    ///
    /// "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE"
    #[serde(rename="DATA_TYPE_TIMESTAMP_WITH_TIMEZONE")]
    DATATYPETIMESTAMPWITHTIMEZONE,
}

impl AsRef<str> for JsonSchemaJdbcTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JsonSchemaJdbcTypeEnum::DATATYPEUNSPECIFIED => "DATA_TYPE_UNSPECIFIED",
            JsonSchemaJdbcTypeEnum::DATATYPEINT => "DATA_TYPE_INT",
            JsonSchemaJdbcTypeEnum::DATATYPESMALLINT => "DATA_TYPE_SMALLINT",
            JsonSchemaJdbcTypeEnum::DATATYPEDOUBLE => "DATA_TYPE_DOUBLE",
            JsonSchemaJdbcTypeEnum::DATATYPEDATE => "DATA_TYPE_DATE",
            JsonSchemaJdbcTypeEnum::DATATYPEDATETIME => "DATA_TYPE_DATETIME",
            JsonSchemaJdbcTypeEnum::DATATYPETIME => "DATA_TYPE_TIME",
            JsonSchemaJdbcTypeEnum::DATATYPESTRING => "DATA_TYPE_STRING",
            JsonSchemaJdbcTypeEnum::DATATYPELONG => "DATA_TYPE_LONG",
            JsonSchemaJdbcTypeEnum::DATATYPEBOOLEAN => "DATA_TYPE_BOOLEAN",
            JsonSchemaJdbcTypeEnum::DATATYPEDECIMAL => "DATA_TYPE_DECIMAL",
            JsonSchemaJdbcTypeEnum::DATATYPEUUID => "DATA_TYPE_UUID",
            JsonSchemaJdbcTypeEnum::DATATYPEBLOB => "DATA_TYPE_BLOB",
            JsonSchemaJdbcTypeEnum::DATATYPEBIT => "DATA_TYPE_BIT",
            JsonSchemaJdbcTypeEnum::DATATYPETINYINT => "DATA_TYPE_TINYINT",
            JsonSchemaJdbcTypeEnum::DATATYPEINTEGER => "DATA_TYPE_INTEGER",
            JsonSchemaJdbcTypeEnum::DATATYPEBIGINT => "DATA_TYPE_BIGINT",
            JsonSchemaJdbcTypeEnum::DATATYPEFLOAT => "DATA_TYPE_FLOAT",
            JsonSchemaJdbcTypeEnum::DATATYPEREAL => "DATA_TYPE_REAL",
            JsonSchemaJdbcTypeEnum::DATATYPENUMERIC => "DATA_TYPE_NUMERIC",
            JsonSchemaJdbcTypeEnum::DATATYPECHAR => "DATA_TYPE_CHAR",
            JsonSchemaJdbcTypeEnum::DATATYPEVARCHAR => "DATA_TYPE_VARCHAR",
            JsonSchemaJdbcTypeEnum::DATATYPELONGVARCHAR => "DATA_TYPE_LONGVARCHAR",
            JsonSchemaJdbcTypeEnum::DATATYPETIMESTAMP => "DATA_TYPE_TIMESTAMP",
            JsonSchemaJdbcTypeEnum::DATATYPENCHAR => "DATA_TYPE_NCHAR",
            JsonSchemaJdbcTypeEnum::DATATYPENVARCHAR => "DATA_TYPE_NVARCHAR",
            JsonSchemaJdbcTypeEnum::DATATYPELONGNVARCHAR => "DATA_TYPE_LONGNVARCHAR",
            JsonSchemaJdbcTypeEnum::DATATYPENULL => "DATA_TYPE_NULL",
            JsonSchemaJdbcTypeEnum::DATATYPEOTHER => "DATA_TYPE_OTHER",
            JsonSchemaJdbcTypeEnum::DATATYPEJAVAOBJECT => "DATA_TYPE_JAVA_OBJECT",
            JsonSchemaJdbcTypeEnum::DATATYPEDISTINCT => "DATA_TYPE_DISTINCT",
            JsonSchemaJdbcTypeEnum::DATATYPESTRUCT => "DATA_TYPE_STRUCT",
            JsonSchemaJdbcTypeEnum::DATATYPEARRAY => "DATA_TYPE_ARRAY",
            JsonSchemaJdbcTypeEnum::DATATYPECLOB => "DATA_TYPE_CLOB",
            JsonSchemaJdbcTypeEnum::DATATYPEREF => "DATA_TYPE_REF",
            JsonSchemaJdbcTypeEnum::DATATYPEDATALINK => "DATA_TYPE_DATALINK",
            JsonSchemaJdbcTypeEnum::DATATYPEROWID => "DATA_TYPE_ROWID",
            JsonSchemaJdbcTypeEnum::DATATYPEBINARY => "DATA_TYPE_BINARY",
            JsonSchemaJdbcTypeEnum::DATATYPEVARBINARY => "DATA_TYPE_VARBINARY",
            JsonSchemaJdbcTypeEnum::DATATYPELONGVARBINARY => "DATA_TYPE_LONGVARBINARY",
            JsonSchemaJdbcTypeEnum::DATATYPENCLOB => "DATA_TYPE_NCLOB",
            JsonSchemaJdbcTypeEnum::DATATYPESQLXML => "DATA_TYPE_SQLXML",
            JsonSchemaJdbcTypeEnum::DATATYPEREFCURSOR => "DATA_TYPE_REF_CURSOR",
            JsonSchemaJdbcTypeEnum::DATATYPETIMEWITHTIMEZONE => "DATA_TYPE_TIME_WITH_TIMEZONE",
            JsonSchemaJdbcTypeEnum::DATATYPETIMESTAMPWITHTIMEZONE => "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE",
        }
    }
}

impl std::convert::TryFrom< &str> for JsonSchemaJdbcTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_TYPE_UNSPECIFIED" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEUNSPECIFIED),
           "DATA_TYPE_INT" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEINT),
           "DATA_TYPE_SMALLINT" => Ok(JsonSchemaJdbcTypeEnum::DATATYPESMALLINT),
           "DATA_TYPE_DOUBLE" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEDOUBLE),
           "DATA_TYPE_DATE" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEDATE),
           "DATA_TYPE_DATETIME" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEDATETIME),
           "DATA_TYPE_TIME" => Ok(JsonSchemaJdbcTypeEnum::DATATYPETIME),
           "DATA_TYPE_STRING" => Ok(JsonSchemaJdbcTypeEnum::DATATYPESTRING),
           "DATA_TYPE_LONG" => Ok(JsonSchemaJdbcTypeEnum::DATATYPELONG),
           "DATA_TYPE_BOOLEAN" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEBOOLEAN),
           "DATA_TYPE_DECIMAL" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEDECIMAL),
           "DATA_TYPE_UUID" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEUUID),
           "DATA_TYPE_BLOB" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEBLOB),
           "DATA_TYPE_BIT" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEBIT),
           "DATA_TYPE_TINYINT" => Ok(JsonSchemaJdbcTypeEnum::DATATYPETINYINT),
           "DATA_TYPE_INTEGER" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEINTEGER),
           "DATA_TYPE_BIGINT" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEBIGINT),
           "DATA_TYPE_FLOAT" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEFLOAT),
           "DATA_TYPE_REAL" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEREAL),
           "DATA_TYPE_NUMERIC" => Ok(JsonSchemaJdbcTypeEnum::DATATYPENUMERIC),
           "DATA_TYPE_CHAR" => Ok(JsonSchemaJdbcTypeEnum::DATATYPECHAR),
           "DATA_TYPE_VARCHAR" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEVARCHAR),
           "DATA_TYPE_LONGVARCHAR" => Ok(JsonSchemaJdbcTypeEnum::DATATYPELONGVARCHAR),
           "DATA_TYPE_TIMESTAMP" => Ok(JsonSchemaJdbcTypeEnum::DATATYPETIMESTAMP),
           "DATA_TYPE_NCHAR" => Ok(JsonSchemaJdbcTypeEnum::DATATYPENCHAR),
           "DATA_TYPE_NVARCHAR" => Ok(JsonSchemaJdbcTypeEnum::DATATYPENVARCHAR),
           "DATA_TYPE_LONGNVARCHAR" => Ok(JsonSchemaJdbcTypeEnum::DATATYPELONGNVARCHAR),
           "DATA_TYPE_NULL" => Ok(JsonSchemaJdbcTypeEnum::DATATYPENULL),
           "DATA_TYPE_OTHER" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEOTHER),
           "DATA_TYPE_JAVA_OBJECT" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEJAVAOBJECT),
           "DATA_TYPE_DISTINCT" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEDISTINCT),
           "DATA_TYPE_STRUCT" => Ok(JsonSchemaJdbcTypeEnum::DATATYPESTRUCT),
           "DATA_TYPE_ARRAY" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEARRAY),
           "DATA_TYPE_CLOB" => Ok(JsonSchemaJdbcTypeEnum::DATATYPECLOB),
           "DATA_TYPE_REF" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEREF),
           "DATA_TYPE_DATALINK" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEDATALINK),
           "DATA_TYPE_ROWID" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEROWID),
           "DATA_TYPE_BINARY" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEBINARY),
           "DATA_TYPE_VARBINARY" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEVARBINARY),
           "DATA_TYPE_LONGVARBINARY" => Ok(JsonSchemaJdbcTypeEnum::DATATYPELONGVARBINARY),
           "DATA_TYPE_NCLOB" => Ok(JsonSchemaJdbcTypeEnum::DATATYPENCLOB),
           "DATA_TYPE_SQLXML" => Ok(JsonSchemaJdbcTypeEnum::DATATYPESQLXML),
           "DATA_TYPE_REF_CURSOR" => Ok(JsonSchemaJdbcTypeEnum::DATATYPEREFCURSOR),
           "DATA_TYPE_TIME_WITH_TIMEZONE" => Ok(JsonSchemaJdbcTypeEnum::DATATYPETIMEWITHTIMEZONE),
           "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE" => Ok(JsonSchemaJdbcTypeEnum::DATATYPETIMESTAMPWITHTIMEZONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JsonSchemaJdbcTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LogicalExpressionLogicalOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The logical operator to use between the fields and conditions.
pub enum LogicalExpressionLogicalOperatorEnum {
    

    /// The default value.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// AND operator; The conditions must all be true.
    ///
    /// "AND"
    #[serde(rename="AND")]
    AND,
    

    /// OR operator; At least one of the conditions must be true.
    ///
    /// "OR"
    #[serde(rename="OR")]
    OR,
}

impl AsRef<str> for LogicalExpressionLogicalOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LogicalExpressionLogicalOperatorEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            LogicalExpressionLogicalOperatorEnum::AND => "AND",
            LogicalExpressionLogicalOperatorEnum::OR => "OR",
        }
    }
}

impl std::convert::TryFrom< &str> for LogicalExpressionLogicalOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(LogicalExpressionLogicalOperatorEnum::OPERATORUNSPECIFIED),
           "AND" => Ok(LogicalExpressionLogicalOperatorEnum::AND),
           "OR" => Ok(LogicalExpressionLogicalOperatorEnum::OR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LogicalExpressionLogicalOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkConfigEgressModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Egress mode for the network.
pub enum NetworkConfigEgressModeEnum {
    

    /// Egress mode unspecified.
    ///
    /// "NETWORK_EGRESS_MODE_UNSPECIFIED"
    #[serde(rename="NETWORK_EGRESS_MODE_UNSPECIFIED")]
    NETWORKEGRESSMODEUNSPECIFIED,
    

    /// Network egress through auto assigned IPs.
    ///
    /// "AUTO_IP"
    #[serde(rename="AUTO_IP")]
    AUTOIP,
    

    /// Network egress through static IPs.
    ///
    /// "STATIC_IP"
    #[serde(rename="STATIC_IP")]
    STATICIP,
}

impl AsRef<str> for NetworkConfigEgressModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkConfigEgressModeEnum::NETWORKEGRESSMODEUNSPECIFIED => "NETWORK_EGRESS_MODE_UNSPECIFIED",
            NetworkConfigEgressModeEnum::AUTOIP => "AUTO_IP",
            NetworkConfigEgressModeEnum::STATICIP => "STATIC_IP",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkConfigEgressModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NETWORK_EGRESS_MODE_UNSPECIFIED" => Ok(NetworkConfigEgressModeEnum::NETWORKEGRESSMODEUNSPECIFIED),
           "AUTO_IP" => Ok(NetworkConfigEgressModeEnum::AUTOIP),
           "STATIC_IP" => Ok(NetworkConfigEgressModeEnum::STATICIP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkConfigEgressModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProviderLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Flag to mark the version indicating the launch stage.
pub enum ProviderLaunchStageEnum {
    

    /// LAUNCH_STAGE_UNSPECIFIED.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// PREVIEW.
    ///
    /// "PREVIEW"
    #[serde(rename="PREVIEW")]
    PREVIEW,
    

    /// GA.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// DEPRECATED.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
    

    /// PRIVATE_PREVIEW.
    ///
    /// "PRIVATE_PREVIEW"
    #[serde(rename="PRIVATE_PREVIEW")]
    PRIVATEPREVIEW,
}

impl AsRef<str> for ProviderLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProviderLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            ProviderLaunchStageEnum::PREVIEW => "PREVIEW",
            ProviderLaunchStageEnum::GA => "GA",
            ProviderLaunchStageEnum::DEPRECATED => "DEPRECATED",
            ProviderLaunchStageEnum::PRIVATEPREVIEW => "PRIVATE_PREVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for ProviderLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(ProviderLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "PREVIEW" => Ok(ProviderLaunchStageEnum::PREVIEW),
           "GA" => Ok(ProviderLaunchStageEnum::GA),
           "DEPRECATED" => Ok(ProviderLaunchStageEnum::DEPRECATED),
           "PRIVATE_PREVIEW" => Ok(ProviderLaunchStageEnum::PRIVATEPREVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProviderLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Different types of resource supported.
pub enum ResourceTypeEnum {
    

    /// Value type is not specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Google Cloud Project Resource.
    ///
    /// "GCP_PROJECT"
    #[serde(rename="GCP_PROJECT")]
    GCPPROJECT,
    

    /// Any Google Cloud Resource which is identified uniquely by IAM.
    ///
    /// "GCP_RESOURCE"
    #[serde(rename="GCP_RESOURCE")]
    GCPRESOURCE,
    

    /// Google Cloud Secret Resource.
    ///
    /// "GCP_SECRETMANAGER_SECRET"
    #[serde(rename="GCP_SECRETMANAGER_SECRET")]
    GCPSECRETMANAGERSECRET,
    

    /// Google Cloud Secret Version Resource.
    ///
    /// "GCP_SECRETMANAGER_SECRET_VERSION"
    #[serde(rename="GCP_SECRETMANAGER_SECRET_VERSION")]
    GCPSECRETMANAGERSECRETVERSION,
}

impl AsRef<str> for ResourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResourceTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            ResourceTypeEnum::GCPPROJECT => "GCP_PROJECT",
            ResourceTypeEnum::GCPRESOURCE => "GCP_RESOURCE",
            ResourceTypeEnum::GCPSECRETMANAGERSECRET => "GCP_SECRETMANAGER_SECRET",
            ResourceTypeEnum::GCPSECRETMANAGERSECRETVERSION => "GCP_SECRETMANAGER_SECRET_VERSION",
        }
    }
}

impl std::convert::TryFrom< &str> for ResourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(ResourceTypeEnum::TYPEUNSPECIFIED),
           "GCP_PROJECT" => Ok(ResourceTypeEnum::GCPPROJECT),
           "GCP_RESOURCE" => Ok(ResourceTypeEnum::GCPRESOURCE),
           "GCP_SECRETMANAGER_SECRET" => Ok(ResourceTypeEnum::GCPSECRETMANAGERSECRET),
           "GCP_SECRETMANAGER_SECRET_VERSION" => Ok(ResourceTypeEnum::GCPSECRETMANAGERSECRETVERSION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResultMetadataDataTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The data type of the field.
pub enum ResultMetadataDataTypeEnum {
    

    /// Data type is not specified.
    ///
    /// "DATA_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_TYPE_UNSPECIFIED")]
    DATATYPEUNSPECIFIED,
    

    /// DEPRECATED! Use DATA_TYPE_INTEGER.
    ///
    /// "DATA_TYPE_INT"
    #[serde(rename="DATA_TYPE_INT")]
    DATATYPEINT,
    

    /// Short integer(int16) data type.
    ///
    /// "DATA_TYPE_SMALLINT"
    #[serde(rename="DATA_TYPE_SMALLINT")]
    DATATYPESMALLINT,
    

    /// Double data type.
    ///
    /// "DATA_TYPE_DOUBLE"
    #[serde(rename="DATA_TYPE_DOUBLE")]
    DATATYPEDOUBLE,
    

    /// Date data type.
    ///
    /// "DATA_TYPE_DATE"
    #[serde(rename="DATA_TYPE_DATE")]
    DATATYPEDATE,
    

    /// DEPRECATED! Use DATA_TYPE_TIMESTAMP.
    ///
    /// "DATA_TYPE_DATETIME"
    #[serde(rename="DATA_TYPE_DATETIME")]
    DATATYPEDATETIME,
    

    /// Time data type.
    ///
    /// "DATA_TYPE_TIME"
    #[serde(rename="DATA_TYPE_TIME")]
    DATATYPETIME,
    

    /// DEPRECATED! Use DATA_TYPE_VARCHAR.
    ///
    /// "DATA_TYPE_STRING"
    #[serde(rename="DATA_TYPE_STRING")]
    DATATYPESTRING,
    

    /// DEPRECATED! Use DATA_TYPE_BIGINT.
    ///
    /// "DATA_TYPE_LONG"
    #[serde(rename="DATA_TYPE_LONG")]
    DATATYPELONG,
    

    /// Boolean data type.
    ///
    /// "DATA_TYPE_BOOLEAN"
    #[serde(rename="DATA_TYPE_BOOLEAN")]
    DATATYPEBOOLEAN,
    

    /// Decimal data type.
    ///
    /// "DATA_TYPE_DECIMAL"
    #[serde(rename="DATA_TYPE_DECIMAL")]
    DATATYPEDECIMAL,
    

    /// DEPRECATED! Use DATA_TYPE_VARCHAR.
    ///
    /// "DATA_TYPE_UUID"
    #[serde(rename="DATA_TYPE_UUID")]
    DATATYPEUUID,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_BLOB"
    #[serde(rename="DATA_TYPE_BLOB")]
    DATATYPEBLOB,
    

    /// Bit data type.
    ///
    /// "DATA_TYPE_BIT"
    #[serde(rename="DATA_TYPE_BIT")]
    DATATYPEBIT,
    

    /// Small integer(int8) data type.
    ///
    /// "DATA_TYPE_TINYINT"
    #[serde(rename="DATA_TYPE_TINYINT")]
    DATATYPETINYINT,
    

    /// Integer(int32) data type.
    ///
    /// "DATA_TYPE_INTEGER"
    #[serde(rename="DATA_TYPE_INTEGER")]
    DATATYPEINTEGER,
    

    /// Long integer(int64) data type.
    ///
    /// "DATA_TYPE_BIGINT"
    #[serde(rename="DATA_TYPE_BIGINT")]
    DATATYPEBIGINT,
    

    /// Float data type.
    ///
    /// "DATA_TYPE_FLOAT"
    #[serde(rename="DATA_TYPE_FLOAT")]
    DATATYPEFLOAT,
    

    /// Real data type.
    ///
    /// "DATA_TYPE_REAL"
    #[serde(rename="DATA_TYPE_REAL")]
    DATATYPEREAL,
    

    /// Numeric data type.
    ///
    /// "DATA_TYPE_NUMERIC"
    #[serde(rename="DATA_TYPE_NUMERIC")]
    DATATYPENUMERIC,
    

    /// Char data type.
    ///
    /// "DATA_TYPE_CHAR"
    #[serde(rename="DATA_TYPE_CHAR")]
    DATATYPECHAR,
    

    /// Varchar data type.
    ///
    /// "DATA_TYPE_VARCHAR"
    #[serde(rename="DATA_TYPE_VARCHAR")]
    DATATYPEVARCHAR,
    

    /// Longvarchar data type.
    ///
    /// "DATA_TYPE_LONGVARCHAR"
    #[serde(rename="DATA_TYPE_LONGVARCHAR")]
    DATATYPELONGVARCHAR,
    

    /// Timestamp data type.
    ///
    /// "DATA_TYPE_TIMESTAMP"
    #[serde(rename="DATA_TYPE_TIMESTAMP")]
    DATATYPETIMESTAMP,
    

    /// Nchar data type.
    ///
    /// "DATA_TYPE_NCHAR"
    #[serde(rename="DATA_TYPE_NCHAR")]
    DATATYPENCHAR,
    

    /// Nvarchar data type.
    ///
    /// "DATA_TYPE_NVARCHAR"
    #[serde(rename="DATA_TYPE_NVARCHAR")]
    DATATYPENVARCHAR,
    

    /// Longnvarchar data type.
    ///
    /// "DATA_TYPE_LONGNVARCHAR"
    #[serde(rename="DATA_TYPE_LONGNVARCHAR")]
    DATATYPELONGNVARCHAR,
    

    /// Null data type.
    ///
    /// "DATA_TYPE_NULL"
    #[serde(rename="DATA_TYPE_NULL")]
    DATATYPENULL,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_OTHER"
    #[serde(rename="DATA_TYPE_OTHER")]
    DATATYPEOTHER,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_JAVA_OBJECT"
    #[serde(rename="DATA_TYPE_JAVA_OBJECT")]
    DATATYPEJAVAOBJECT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_DISTINCT"
    #[serde(rename="DATA_TYPE_DISTINCT")]
    DATATYPEDISTINCT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_STRUCT"
    #[serde(rename="DATA_TYPE_STRUCT")]
    DATATYPESTRUCT,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_ARRAY"
    #[serde(rename="DATA_TYPE_ARRAY")]
    DATATYPEARRAY,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_CLOB"
    #[serde(rename="DATA_TYPE_CLOB")]
    DATATYPECLOB,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_REF"
    #[serde(rename="DATA_TYPE_REF")]
    DATATYPEREF,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_DATALINK"
    #[serde(rename="DATA_TYPE_DATALINK")]
    DATATYPEDATALINK,
    

    /// UNSUPPORTED! Row id data type.
    ///
    /// "DATA_TYPE_ROWID"
    #[serde(rename="DATA_TYPE_ROWID")]
    DATATYPEROWID,
    

    /// UNSUPPORTED! Binary data type.
    ///
    /// "DATA_TYPE_BINARY"
    #[serde(rename="DATA_TYPE_BINARY")]
    DATATYPEBINARY,
    

    /// UNSUPPORTED! Variable binary data type.
    ///
    /// "DATA_TYPE_VARBINARY"
    #[serde(rename="DATA_TYPE_VARBINARY")]
    DATATYPEVARBINARY,
    

    /// UNSUPPORTED! Long variable binary data type.
    ///
    /// "DATA_TYPE_LONGVARBINARY"
    #[serde(rename="DATA_TYPE_LONGVARBINARY")]
    DATATYPELONGVARBINARY,
    

    /// UNSUPPORTED! NCLOB data type.
    ///
    /// "DATA_TYPE_NCLOB"
    #[serde(rename="DATA_TYPE_NCLOB")]
    DATATYPENCLOB,
    

    /// UNSUPPORTED! SQL XML data type is not supported.
    ///
    /// "DATA_TYPE_SQLXML"
    #[serde(rename="DATA_TYPE_SQLXML")]
    DATATYPESQLXML,
    

    /// UNSUPPORTED! Cursor reference type is not supported.
    ///
    /// "DATA_TYPE_REF_CURSOR"
    #[serde(rename="DATA_TYPE_REF_CURSOR")]
    DATATYPEREFCURSOR,
    

    /// UNSUPPORTED! Use TIME or TIMESTAMP instead.
    ///
    /// "DATA_TYPE_TIME_WITH_TIMEZONE"
    #[serde(rename="DATA_TYPE_TIME_WITH_TIMEZONE")]
    DATATYPETIMEWITHTIMEZONE,
    

    /// UNSUPPORTED! Use TIMESTAMP instead.
    ///
    /// "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE"
    #[serde(rename="DATA_TYPE_TIMESTAMP_WITH_TIMEZONE")]
    DATATYPETIMESTAMPWITHTIMEZONE,
}

impl AsRef<str> for ResultMetadataDataTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResultMetadataDataTypeEnum::DATATYPEUNSPECIFIED => "DATA_TYPE_UNSPECIFIED",
            ResultMetadataDataTypeEnum::DATATYPEINT => "DATA_TYPE_INT",
            ResultMetadataDataTypeEnum::DATATYPESMALLINT => "DATA_TYPE_SMALLINT",
            ResultMetadataDataTypeEnum::DATATYPEDOUBLE => "DATA_TYPE_DOUBLE",
            ResultMetadataDataTypeEnum::DATATYPEDATE => "DATA_TYPE_DATE",
            ResultMetadataDataTypeEnum::DATATYPEDATETIME => "DATA_TYPE_DATETIME",
            ResultMetadataDataTypeEnum::DATATYPETIME => "DATA_TYPE_TIME",
            ResultMetadataDataTypeEnum::DATATYPESTRING => "DATA_TYPE_STRING",
            ResultMetadataDataTypeEnum::DATATYPELONG => "DATA_TYPE_LONG",
            ResultMetadataDataTypeEnum::DATATYPEBOOLEAN => "DATA_TYPE_BOOLEAN",
            ResultMetadataDataTypeEnum::DATATYPEDECIMAL => "DATA_TYPE_DECIMAL",
            ResultMetadataDataTypeEnum::DATATYPEUUID => "DATA_TYPE_UUID",
            ResultMetadataDataTypeEnum::DATATYPEBLOB => "DATA_TYPE_BLOB",
            ResultMetadataDataTypeEnum::DATATYPEBIT => "DATA_TYPE_BIT",
            ResultMetadataDataTypeEnum::DATATYPETINYINT => "DATA_TYPE_TINYINT",
            ResultMetadataDataTypeEnum::DATATYPEINTEGER => "DATA_TYPE_INTEGER",
            ResultMetadataDataTypeEnum::DATATYPEBIGINT => "DATA_TYPE_BIGINT",
            ResultMetadataDataTypeEnum::DATATYPEFLOAT => "DATA_TYPE_FLOAT",
            ResultMetadataDataTypeEnum::DATATYPEREAL => "DATA_TYPE_REAL",
            ResultMetadataDataTypeEnum::DATATYPENUMERIC => "DATA_TYPE_NUMERIC",
            ResultMetadataDataTypeEnum::DATATYPECHAR => "DATA_TYPE_CHAR",
            ResultMetadataDataTypeEnum::DATATYPEVARCHAR => "DATA_TYPE_VARCHAR",
            ResultMetadataDataTypeEnum::DATATYPELONGVARCHAR => "DATA_TYPE_LONGVARCHAR",
            ResultMetadataDataTypeEnum::DATATYPETIMESTAMP => "DATA_TYPE_TIMESTAMP",
            ResultMetadataDataTypeEnum::DATATYPENCHAR => "DATA_TYPE_NCHAR",
            ResultMetadataDataTypeEnum::DATATYPENVARCHAR => "DATA_TYPE_NVARCHAR",
            ResultMetadataDataTypeEnum::DATATYPELONGNVARCHAR => "DATA_TYPE_LONGNVARCHAR",
            ResultMetadataDataTypeEnum::DATATYPENULL => "DATA_TYPE_NULL",
            ResultMetadataDataTypeEnum::DATATYPEOTHER => "DATA_TYPE_OTHER",
            ResultMetadataDataTypeEnum::DATATYPEJAVAOBJECT => "DATA_TYPE_JAVA_OBJECT",
            ResultMetadataDataTypeEnum::DATATYPEDISTINCT => "DATA_TYPE_DISTINCT",
            ResultMetadataDataTypeEnum::DATATYPESTRUCT => "DATA_TYPE_STRUCT",
            ResultMetadataDataTypeEnum::DATATYPEARRAY => "DATA_TYPE_ARRAY",
            ResultMetadataDataTypeEnum::DATATYPECLOB => "DATA_TYPE_CLOB",
            ResultMetadataDataTypeEnum::DATATYPEREF => "DATA_TYPE_REF",
            ResultMetadataDataTypeEnum::DATATYPEDATALINK => "DATA_TYPE_DATALINK",
            ResultMetadataDataTypeEnum::DATATYPEROWID => "DATA_TYPE_ROWID",
            ResultMetadataDataTypeEnum::DATATYPEBINARY => "DATA_TYPE_BINARY",
            ResultMetadataDataTypeEnum::DATATYPEVARBINARY => "DATA_TYPE_VARBINARY",
            ResultMetadataDataTypeEnum::DATATYPELONGVARBINARY => "DATA_TYPE_LONGVARBINARY",
            ResultMetadataDataTypeEnum::DATATYPENCLOB => "DATA_TYPE_NCLOB",
            ResultMetadataDataTypeEnum::DATATYPESQLXML => "DATA_TYPE_SQLXML",
            ResultMetadataDataTypeEnum::DATATYPEREFCURSOR => "DATA_TYPE_REF_CURSOR",
            ResultMetadataDataTypeEnum::DATATYPETIMEWITHTIMEZONE => "DATA_TYPE_TIME_WITH_TIMEZONE",
            ResultMetadataDataTypeEnum::DATATYPETIMESTAMPWITHTIMEZONE => "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE",
        }
    }
}

impl std::convert::TryFrom< &str> for ResultMetadataDataTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_TYPE_UNSPECIFIED" => Ok(ResultMetadataDataTypeEnum::DATATYPEUNSPECIFIED),
           "DATA_TYPE_INT" => Ok(ResultMetadataDataTypeEnum::DATATYPEINT),
           "DATA_TYPE_SMALLINT" => Ok(ResultMetadataDataTypeEnum::DATATYPESMALLINT),
           "DATA_TYPE_DOUBLE" => Ok(ResultMetadataDataTypeEnum::DATATYPEDOUBLE),
           "DATA_TYPE_DATE" => Ok(ResultMetadataDataTypeEnum::DATATYPEDATE),
           "DATA_TYPE_DATETIME" => Ok(ResultMetadataDataTypeEnum::DATATYPEDATETIME),
           "DATA_TYPE_TIME" => Ok(ResultMetadataDataTypeEnum::DATATYPETIME),
           "DATA_TYPE_STRING" => Ok(ResultMetadataDataTypeEnum::DATATYPESTRING),
           "DATA_TYPE_LONG" => Ok(ResultMetadataDataTypeEnum::DATATYPELONG),
           "DATA_TYPE_BOOLEAN" => Ok(ResultMetadataDataTypeEnum::DATATYPEBOOLEAN),
           "DATA_TYPE_DECIMAL" => Ok(ResultMetadataDataTypeEnum::DATATYPEDECIMAL),
           "DATA_TYPE_UUID" => Ok(ResultMetadataDataTypeEnum::DATATYPEUUID),
           "DATA_TYPE_BLOB" => Ok(ResultMetadataDataTypeEnum::DATATYPEBLOB),
           "DATA_TYPE_BIT" => Ok(ResultMetadataDataTypeEnum::DATATYPEBIT),
           "DATA_TYPE_TINYINT" => Ok(ResultMetadataDataTypeEnum::DATATYPETINYINT),
           "DATA_TYPE_INTEGER" => Ok(ResultMetadataDataTypeEnum::DATATYPEINTEGER),
           "DATA_TYPE_BIGINT" => Ok(ResultMetadataDataTypeEnum::DATATYPEBIGINT),
           "DATA_TYPE_FLOAT" => Ok(ResultMetadataDataTypeEnum::DATATYPEFLOAT),
           "DATA_TYPE_REAL" => Ok(ResultMetadataDataTypeEnum::DATATYPEREAL),
           "DATA_TYPE_NUMERIC" => Ok(ResultMetadataDataTypeEnum::DATATYPENUMERIC),
           "DATA_TYPE_CHAR" => Ok(ResultMetadataDataTypeEnum::DATATYPECHAR),
           "DATA_TYPE_VARCHAR" => Ok(ResultMetadataDataTypeEnum::DATATYPEVARCHAR),
           "DATA_TYPE_LONGVARCHAR" => Ok(ResultMetadataDataTypeEnum::DATATYPELONGVARCHAR),
           "DATA_TYPE_TIMESTAMP" => Ok(ResultMetadataDataTypeEnum::DATATYPETIMESTAMP),
           "DATA_TYPE_NCHAR" => Ok(ResultMetadataDataTypeEnum::DATATYPENCHAR),
           "DATA_TYPE_NVARCHAR" => Ok(ResultMetadataDataTypeEnum::DATATYPENVARCHAR),
           "DATA_TYPE_LONGNVARCHAR" => Ok(ResultMetadataDataTypeEnum::DATATYPELONGNVARCHAR),
           "DATA_TYPE_NULL" => Ok(ResultMetadataDataTypeEnum::DATATYPENULL),
           "DATA_TYPE_OTHER" => Ok(ResultMetadataDataTypeEnum::DATATYPEOTHER),
           "DATA_TYPE_JAVA_OBJECT" => Ok(ResultMetadataDataTypeEnum::DATATYPEJAVAOBJECT),
           "DATA_TYPE_DISTINCT" => Ok(ResultMetadataDataTypeEnum::DATATYPEDISTINCT),
           "DATA_TYPE_STRUCT" => Ok(ResultMetadataDataTypeEnum::DATATYPESTRUCT),
           "DATA_TYPE_ARRAY" => Ok(ResultMetadataDataTypeEnum::DATATYPEARRAY),
           "DATA_TYPE_CLOB" => Ok(ResultMetadataDataTypeEnum::DATATYPECLOB),
           "DATA_TYPE_REF" => Ok(ResultMetadataDataTypeEnum::DATATYPEREF),
           "DATA_TYPE_DATALINK" => Ok(ResultMetadataDataTypeEnum::DATATYPEDATALINK),
           "DATA_TYPE_ROWID" => Ok(ResultMetadataDataTypeEnum::DATATYPEROWID),
           "DATA_TYPE_BINARY" => Ok(ResultMetadataDataTypeEnum::DATATYPEBINARY),
           "DATA_TYPE_VARBINARY" => Ok(ResultMetadataDataTypeEnum::DATATYPEVARBINARY),
           "DATA_TYPE_LONGVARBINARY" => Ok(ResultMetadataDataTypeEnum::DATATYPELONGVARBINARY),
           "DATA_TYPE_NCLOB" => Ok(ResultMetadataDataTypeEnum::DATATYPENCLOB),
           "DATA_TYPE_SQLXML" => Ok(ResultMetadataDataTypeEnum::DATATYPESQLXML),
           "DATA_TYPE_REF_CURSOR" => Ok(ResultMetadataDataTypeEnum::DATATYPEREFCURSOR),
           "DATA_TYPE_TIME_WITH_TIMEZONE" => Ok(ResultMetadataDataTypeEnum::DATATYPETIMEWITHTIMEZONE),
           "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE" => Ok(ResultMetadataDataTypeEnum::DATATYPETIMESTAMPWITHTIMEZONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResultMetadataDataTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RoleGrantPrincipalEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Prinicipal/Identity for whom the role need to assigned.
pub enum RoleGrantPrincipalEnum {
    

    /// Value type is not specified.
    ///
    /// "PRINCIPAL_UNSPECIFIED"
    #[serde(rename="PRINCIPAL_UNSPECIFIED")]
    PRINCIPALUNSPECIFIED,
    

    /// Service Account used for Connector workload identity This is either the default service account if unspecified or Service Account provided by Customers through BYOSA.
    ///
    /// "CONNECTOR_SA"
    #[serde(rename="CONNECTOR_SA")]
    CONNECTORSA,
}

impl AsRef<str> for RoleGrantPrincipalEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RoleGrantPrincipalEnum::PRINCIPALUNSPECIFIED => "PRINCIPAL_UNSPECIFIED",
            RoleGrantPrincipalEnum::CONNECTORSA => "CONNECTOR_SA",
        }
    }
}

impl std::convert::TryFrom< &str> for RoleGrantPrincipalEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRINCIPAL_UNSPECIFIED" => Ok(RoleGrantPrincipalEnum::PRINCIPALUNSPECIFIED),
           "CONNECTOR_SA" => Ok(RoleGrantPrincipalEnum::CONNECTORSA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RoleGrantPrincipalEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuntimeConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the location.
pub enum RuntimeConfigStateEnum {
    

    /// STATE_UNSPECIFIED.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// INACTIVE.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// ACTIVATING.
    ///
    /// "ACTIVATING"
    #[serde(rename="ACTIVATING")]
    ACTIVATING,
    

    /// ACTIVE.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// CREATING.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// DELETING.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// UPDATING.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for RuntimeConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuntimeConfigStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            RuntimeConfigStateEnum::INACTIVE => "INACTIVE",
            RuntimeConfigStateEnum::ACTIVATING => "ACTIVATING",
            RuntimeConfigStateEnum::ACTIVE => "ACTIVE",
            RuntimeConfigStateEnum::CREATING => "CREATING",
            RuntimeConfigStateEnum::DELETING => "DELETING",
            RuntimeConfigStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for RuntimeConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(RuntimeConfigStateEnum::STATEUNSPECIFIED),
           "INACTIVE" => Ok(RuntimeConfigStateEnum::INACTIVE),
           "ACTIVATING" => Ok(RuntimeConfigStateEnum::ACTIVATING),
           "ACTIVE" => Ok(RuntimeConfigStateEnum::ACTIVE),
           "CREATING" => Ok(RuntimeConfigStateEnum::CREATING),
           "DELETING" => Ok(RuntimeConfigStateEnum::DELETING),
           "UPDATING" => Ok(RuntimeConfigStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuntimeConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuntimeEntitySchemaOperationsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of operations supported by this entity
pub enum RuntimeEntitySchemaOperationsEnum {
    

    /// Operation unspecified.
    ///
    /// "OPERATION_UNSPECIFIED"
    #[serde(rename="OPERATION_UNSPECIFIED")]
    OPERATIONUNSPECIFIED,
    

    /// This operation means entity type supports LIST entities.
    ///
    /// "LIST"
    #[serde(rename="LIST")]
    LIST,
    

    /// This operation means entity type supports GET entity.
    ///
    /// "GET"
    #[serde(rename="GET")]
    GET,
    

    /// This operation means entity type supports CREATE entity.
    ///
    /// "CREATE"
    #[serde(rename="CREATE")]
    CREATE,
    

    /// This operation means entity type supports UPDATE entity.
    ///
    /// "UPDATE"
    #[serde(rename="UPDATE")]
    UPDATE,
    

    /// This operation means entity type supports DELETE entity.
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
}

impl AsRef<str> for RuntimeEntitySchemaOperationsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuntimeEntitySchemaOperationsEnum::OPERATIONUNSPECIFIED => "OPERATION_UNSPECIFIED",
            RuntimeEntitySchemaOperationsEnum::LIST => "LIST",
            RuntimeEntitySchemaOperationsEnum::GET => "GET",
            RuntimeEntitySchemaOperationsEnum::CREATE => "CREATE",
            RuntimeEntitySchemaOperationsEnum::UPDATE => "UPDATE",
            RuntimeEntitySchemaOperationsEnum::DELETE => "DELETE",
        }
    }
}

impl std::convert::TryFrom< &str> for RuntimeEntitySchemaOperationsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATION_UNSPECIFIED" => Ok(RuntimeEntitySchemaOperationsEnum::OPERATIONUNSPECIFIED),
           "LIST" => Ok(RuntimeEntitySchemaOperationsEnum::LIST),
           "GET" => Ok(RuntimeEntitySchemaOperationsEnum::GET),
           "CREATE" => Ok(RuntimeEntitySchemaOperationsEnum::CREATE),
           "UPDATE" => Ok(RuntimeEntitySchemaOperationsEnum::UPDATE),
           "DELETE" => Ok(RuntimeEntitySchemaOperationsEnum::DELETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuntimeEntitySchemaOperationsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SourceSourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the source.
pub enum SourceSourceTypeEnum {
    

    /// Default SOURCE.
    ///
    /// "SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="SOURCE_TYPE_UNSPECIFIED")]
    SOURCETYPEUNSPECIFIED,
    

    /// Config Variable source type.
    ///
    /// "CONFIG_VARIABLE"
    #[serde(rename="CONFIG_VARIABLE")]
    CONFIGVARIABLE,
}

impl AsRef<str> for SourceSourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SourceSourceTypeEnum::SOURCETYPEUNSPECIFIED => "SOURCE_TYPE_UNSPECIFIED",
            SourceSourceTypeEnum::CONFIGVARIABLE => "CONFIG_VARIABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for SourceSourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SOURCE_TYPE_UNSPECIFIED" => Ok(SourceSourceTypeEnum::SOURCETYPEUNSPECIFIED),
           "CONFIG_VARIABLE" => Ok(SourceSourceTypeEnum::CONFIGVARIABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SourceSourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SslConfigClientCertTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of Client Cert (PEM/JKS/.. etc.)
pub enum SslConfigClientCertTypeEnum {
    

    /// Cert type unspecified.
    ///
    /// "CERT_TYPE_UNSPECIFIED"
    #[serde(rename="CERT_TYPE_UNSPECIFIED")]
    CERTTYPEUNSPECIFIED,
    

    /// Privacy Enhanced Mail (PEM) Type
    ///
    /// "PEM"
    #[serde(rename="PEM")]
    PEM,
}

impl AsRef<str> for SslConfigClientCertTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SslConfigClientCertTypeEnum::CERTTYPEUNSPECIFIED => "CERT_TYPE_UNSPECIFIED",
            SslConfigClientCertTypeEnum::PEM => "PEM",
        }
    }
}

impl std::convert::TryFrom< &str> for SslConfigClientCertTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERT_TYPE_UNSPECIFIED" => Ok(SslConfigClientCertTypeEnum::CERTTYPEUNSPECIFIED),
           "PEM" => Ok(SslConfigClientCertTypeEnum::PEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SslConfigClientCertTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SslConfigServerCertTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of Server Cert (PEM/JKS/.. etc.)
pub enum SslConfigServerCertTypeEnum {
    

    /// Cert type unspecified.
    ///
    /// "CERT_TYPE_UNSPECIFIED"
    #[serde(rename="CERT_TYPE_UNSPECIFIED")]
    CERTTYPEUNSPECIFIED,
    

    /// Privacy Enhanced Mail (PEM) Type
    ///
    /// "PEM"
    #[serde(rename="PEM")]
    PEM,
}

impl AsRef<str> for SslConfigServerCertTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SslConfigServerCertTypeEnum::CERTTYPEUNSPECIFIED => "CERT_TYPE_UNSPECIFIED",
            SslConfigServerCertTypeEnum::PEM => "PEM",
        }
    }
}

impl std::convert::TryFrom< &str> for SslConfigServerCertTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERT_TYPE_UNSPECIFIED" => Ok(SslConfigServerCertTypeEnum::CERTTYPEUNSPECIFIED),
           "PEM" => Ok(SslConfigServerCertTypeEnum::PEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SslConfigServerCertTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SslConfigTrustModelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Trust Model of the SSL connection
pub enum SslConfigTrustModelEnum {
    

    /// Public Trust Model. Takes the Default Java trust store.
    ///
    /// "PUBLIC"
    #[serde(rename="PUBLIC")]
    PUBLIC,
    

    /// Private Trust Model. Takes custom/private trust store.
    ///
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
    

    /// Insecure Trust Model. Accept all certificates.
    ///
    /// "INSECURE"
    #[serde(rename="INSECURE")]
    INSECURE,
}

impl AsRef<str> for SslConfigTrustModelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SslConfigTrustModelEnum::PUBLIC => "PUBLIC",
            SslConfigTrustModelEnum::PRIVATE => "PRIVATE",
            SslConfigTrustModelEnum::INSECURE => "INSECURE",
        }
    }
}

impl std::convert::TryFrom< &str> for SslConfigTrustModelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PUBLIC" => Ok(SslConfigTrustModelEnum::PUBLIC),
           "PRIVATE" => Ok(SslConfigTrustModelEnum::PRIVATE),
           "INSECURE" => Ok(SslConfigTrustModelEnum::INSECURE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SslConfigTrustModelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SslConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the ssl type for the given connector version.
pub enum SslConfigTypeEnum {
    

    /// No SSL configuration required.
    ///
    /// "SSL_TYPE_UNSPECIFIED"
    #[serde(rename="SSL_TYPE_UNSPECIFIED")]
    SSLTYPEUNSPECIFIED,
    

    /// TLS Handshake
    ///
    /// "TLS"
    #[serde(rename="TLS")]
    TLS,
    

    /// mutual TLS (MTLS) Handshake
    ///
    /// "MTLS"
    #[serde(rename="MTLS")]
    MTLS,
}

impl AsRef<str> for SslConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SslConfigTypeEnum::SSLTYPEUNSPECIFIED => "SSL_TYPE_UNSPECIFIED",
            SslConfigTypeEnum::TLS => "TLS",
            SslConfigTypeEnum::MTLS => "MTLS",
        }
    }
}

impl std::convert::TryFrom< &str> for SslConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SSL_TYPE_UNSPECIFIED" => Ok(SslConfigTypeEnum::SSLTYPEUNSPECIFIED),
           "TLS" => Ok(SslConfigTypeEnum::TLS),
           "MTLS" => Ok(SslConfigTypeEnum::MTLS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SslConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SslConfigTemplateClientCertTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of supported Client Cert Types
pub enum SslConfigTemplateClientCertTypeEnum {
    

    /// Cert type unspecified.
    ///
    /// "CERT_TYPE_UNSPECIFIED"
    #[serde(rename="CERT_TYPE_UNSPECIFIED")]
    CERTTYPEUNSPECIFIED,
    

    /// Privacy Enhanced Mail (PEM) Type
    ///
    /// "PEM"
    #[serde(rename="PEM")]
    PEM,
}

impl AsRef<str> for SslConfigTemplateClientCertTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SslConfigTemplateClientCertTypeEnum::CERTTYPEUNSPECIFIED => "CERT_TYPE_UNSPECIFIED",
            SslConfigTemplateClientCertTypeEnum::PEM => "PEM",
        }
    }
}

impl std::convert::TryFrom< &str> for SslConfigTemplateClientCertTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERT_TYPE_UNSPECIFIED" => Ok(SslConfigTemplateClientCertTypeEnum::CERTTYPEUNSPECIFIED),
           "PEM" => Ok(SslConfigTemplateClientCertTypeEnum::PEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SslConfigTemplateClientCertTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SslConfigTemplateServerCertTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of supported Server Cert Types
pub enum SslConfigTemplateServerCertTypeEnum {
    

    /// Cert type unspecified.
    ///
    /// "CERT_TYPE_UNSPECIFIED"
    #[serde(rename="CERT_TYPE_UNSPECIFIED")]
    CERTTYPEUNSPECIFIED,
    

    /// Privacy Enhanced Mail (PEM) Type
    ///
    /// "PEM"
    #[serde(rename="PEM")]
    PEM,
}

impl AsRef<str> for SslConfigTemplateServerCertTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SslConfigTemplateServerCertTypeEnum::CERTTYPEUNSPECIFIED => "CERT_TYPE_UNSPECIFIED",
            SslConfigTemplateServerCertTypeEnum::PEM => "PEM",
        }
    }
}

impl std::convert::TryFrom< &str> for SslConfigTemplateServerCertTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERT_TYPE_UNSPECIFIED" => Ok(SslConfigTemplateServerCertTypeEnum::CERTTYPEUNSPECIFIED),
           "PEM" => Ok(SslConfigTemplateServerCertTypeEnum::PEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SslConfigTemplateServerCertTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SslConfigTemplateSslTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the ssl type for the given connector version
pub enum SslConfigTemplateSslTypeEnum {
    

    /// No SSL configuration required.
    ///
    /// "SSL_TYPE_UNSPECIFIED"
    #[serde(rename="SSL_TYPE_UNSPECIFIED")]
    SSLTYPEUNSPECIFIED,
    

    /// TLS Handshake
    ///
    /// "TLS"
    #[serde(rename="TLS")]
    TLS,
    

    /// mutual TLS (MTLS) Handshake
    ///
    /// "MTLS"
    #[serde(rename="MTLS")]
    MTLS,
}

impl AsRef<str> for SslConfigTemplateSslTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SslConfigTemplateSslTypeEnum::SSLTYPEUNSPECIFIED => "SSL_TYPE_UNSPECIFIED",
            SslConfigTemplateSslTypeEnum::TLS => "TLS",
            SslConfigTemplateSslTypeEnum::MTLS => "MTLS",
        }
    }
}

impl std::convert::TryFrom< &str> for SslConfigTemplateSslTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SSL_TYPE_UNSPECIFIED" => Ok(SslConfigTemplateSslTypeEnum::SSLTYPEUNSPECIFIED),
           "TLS" => Ok(SslConfigTemplateSslTypeEnum::TLS),
           "MTLS" => Ok(SslConfigTemplateSslTypeEnum::MTLS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SslConfigTemplateSslTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ValidateCustomConnectorSpecRequestSpecTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Spec type of the custom connector spec.
pub enum ValidateCustomConnectorSpecRequestSpecTypeEnum {
    

    /// Connector type is not specified.
    ///
    /// "CUSTOM_CONNECTOR_TYPE_UNSPECIFIED"
    #[serde(rename="CUSTOM_CONNECTOR_TYPE_UNSPECIFIED")]
    CUSTOMCONNECTORTYPEUNSPECIFIED,
    

    /// OpenAPI connector.
    ///
    /// "OPEN_API"
    #[serde(rename="OPEN_API")]
    OPENAPI,
    

    /// Proto connector.
    ///
    /// "PROTO"
    #[serde(rename="PROTO")]
    PROTO,
}

impl AsRef<str> for ValidateCustomConnectorSpecRequestSpecTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ValidateCustomConnectorSpecRequestSpecTypeEnum::CUSTOMCONNECTORTYPEUNSPECIFIED => "CUSTOM_CONNECTOR_TYPE_UNSPECIFIED",
            ValidateCustomConnectorSpecRequestSpecTypeEnum::OPENAPI => "OPEN_API",
            ValidateCustomConnectorSpecRequestSpecTypeEnum::PROTO => "PROTO",
        }
    }
}

impl std::convert::TryFrom< &str> for ValidateCustomConnectorSpecRequestSpecTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CUSTOM_CONNECTOR_TYPE_UNSPECIFIED" => Ok(ValidateCustomConnectorSpecRequestSpecTypeEnum::CUSTOMCONNECTORTYPEUNSPECIFIED),
           "OPEN_API" => Ok(ValidateCustomConnectorSpecRequestSpecTypeEnum::OPENAPI),
           "PROTO" => Ok(ValidateCustomConnectorSpecRequestSpecTypeEnum::PROTO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ValidateCustomConnectorSpecRequestSpecTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies which fields of the ConnectorVersion are returned in the response. Defaults to `BASIC` view.
pub enum ProjectViewEnum {
    

    /// CONNECTOR_VERSION_VIEW_UNSPECIFIED.
    ///
    /// "CONNECTOR_VERSION_VIEW_UNSPECIFIED"
    #[serde(rename="CONNECTOR_VERSION_VIEW_UNSPECIFIED")]
    CONNECTORVERSIONVIEWUNSPECIFIED,
    

    /// Do not include role grant configs.
    ///
    /// "CONNECTOR_VERSION_VIEW_BASIC"
    #[serde(rename="CONNECTOR_VERSION_VIEW_BASIC")]
    CONNECTORVERSIONVIEWBASIC,
    

    /// Include role grant configs.
    ///
    /// "CONNECTOR_VERSION_VIEW_FULL"
    #[serde(rename="CONNECTOR_VERSION_VIEW_FULL")]
    CONNECTORVERSIONVIEWFULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::CONNECTORVERSIONVIEWUNSPECIFIED => "CONNECTOR_VERSION_VIEW_UNSPECIFIED",
            ProjectViewEnum::CONNECTORVERSIONVIEWBASIC => "CONNECTOR_VERSION_VIEW_BASIC",
            ProjectViewEnum::CONNECTORVERSIONVIEWFULL => "CONNECTOR_VERSION_VIEW_FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTOR_VERSION_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::CONNECTORVERSIONVIEWUNSPECIFIED),
           "CONNECTOR_VERSION_VIEW_BASIC" => Ok(ProjectViewEnum::CONNECTORVERSIONVIEWBASIC),
           "CONNECTOR_VERSION_VIEW_FULL" => Ok(ProjectViewEnum::CONNECTORVERSIONVIEWFULL),
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


