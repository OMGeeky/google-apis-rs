use super::*;



// region AttributePropagationSettingOutputCredentialsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Which output credentials attributes selected by the CEL expression should be propagated in. All attributes will be fully duplicated in each selected output credential.
pub enum AttributePropagationSettingOutputCredentialsEnum {
    

    /// An output credential is required.
    ///
    /// "OUTPUT_CREDENTIALS_UNSPECIFIED"
    #[serde(rename="OUTPUT_CREDENTIALS_UNSPECIFIED")]
    OUTPUTCREDENTIALSUNSPECIFIED,
    

    /// Propagate attributes in the headers with "x-goog-iap-attr-" prefix.
    ///
    /// "HEADER"
    #[serde(rename="HEADER")]
    HEADER,
    

    /// Propagate attributes in the JWT of the form: `"additional_claims": { "my_attribute": ["value1", "value2"] }`
    ///
    /// "JWT"
    #[serde(rename="JWT")]
    JWT,
    

    /// Propagate attributes in the RCToken of the form: `"additional_claims": { "my_attribute": ["value1", "value2"] }`
    ///
    /// "RCTOKEN"
    #[serde(rename="RCTOKEN")]
    RCTOKEN,
}

impl AsRef<str> for AttributePropagationSettingOutputCredentialsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AttributePropagationSettingOutputCredentialsEnum::OUTPUTCREDENTIALSUNSPECIFIED => "OUTPUT_CREDENTIALS_UNSPECIFIED",
            AttributePropagationSettingOutputCredentialsEnum::HEADER => "HEADER",
            AttributePropagationSettingOutputCredentialsEnum::JWT => "JWT",
            AttributePropagationSettingOutputCredentialsEnum::RCTOKEN => "RCTOKEN",
        }
    }
}

impl std::convert::TryFrom< &str> for AttributePropagationSettingOutputCredentialsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OUTPUT_CREDENTIALS_UNSPECIFIED" => Ok(AttributePropagationSettingOutputCredentialsEnum::OUTPUTCREDENTIALSUNSPECIFIED),
           "HEADER" => Ok(AttributePropagationSettingOutputCredentialsEnum::HEADER),
           "JWT" => Ok(AttributePropagationSettingOutputCredentialsEnum::JWT),
           "RCTOKEN" => Ok(AttributePropagationSettingOutputCredentialsEnum::RCTOKEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AttributePropagationSettingOutputCredentialsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReauthSettingMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Reauth method requested.
pub enum ReauthSettingMethodEnum {
    

    /// Reauthentication disabled.
    ///
    /// "METHOD_UNSPECIFIED"
    #[serde(rename="METHOD_UNSPECIFIED")]
    METHODUNSPECIFIED,
    

    /// Prompts the user to log in again.
    ///
    /// "LOGIN"
    #[serde(rename="LOGIN")]
    LOGIN,
    
    /// "PASSWORD"
    #[serde(rename="PASSWORD")]
    PASSWORD,
    

    /// User must use their secure key 2nd factor device.
    ///
    /// "SECURE_KEY"
    #[serde(rename="SECURE_KEY")]
    SECUREKEY,
    

    /// User can use any enabled 2nd factor.
    ///
    /// "ENROLLED_SECOND_FACTORS"
    #[serde(rename="ENROLLED_SECOND_FACTORS")]
    ENROLLEDSECONDFACTORS,
}

impl AsRef<str> for ReauthSettingMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReauthSettingMethodEnum::METHODUNSPECIFIED => "METHOD_UNSPECIFIED",
            ReauthSettingMethodEnum::LOGIN => "LOGIN",
            ReauthSettingMethodEnum::PASSWORD => "PASSWORD",
            ReauthSettingMethodEnum::SECUREKEY => "SECURE_KEY",
            ReauthSettingMethodEnum::ENROLLEDSECONDFACTORS => "ENROLLED_SECOND_FACTORS",
        }
    }
}

impl std::convert::TryFrom< &str> for ReauthSettingMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METHOD_UNSPECIFIED" => Ok(ReauthSettingMethodEnum::METHODUNSPECIFIED),
           "LOGIN" => Ok(ReauthSettingMethodEnum::LOGIN),
           "PASSWORD" => Ok(ReauthSettingMethodEnum::PASSWORD),
           "SECURE_KEY" => Ok(ReauthSettingMethodEnum::SECUREKEY),
           "ENROLLED_SECOND_FACTORS" => Ok(ReauthSettingMethodEnum::ENROLLEDSECONDFACTORS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReauthSettingMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReauthSettingPolicyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How IAP determines the effective policy in cases of hierarchical policies. Policies are merged from higher in the hierarchy to lower in the hierarchy.
pub enum ReauthSettingPolicyTypeEnum {
    

    /// Default value. This value is unused.
    ///
    /// "POLICY_TYPE_UNSPECIFIED"
    #[serde(rename="POLICY_TYPE_UNSPECIFIED")]
    POLICYTYPEUNSPECIFIED,
    

    /// This policy acts as a minimum to other policies, lower in the hierarchy. Effective policy may only be the same or stricter.
    ///
    /// "MINIMUM"
    #[serde(rename="MINIMUM")]
    MINIMUM,
    

    /// This policy acts as a default if no other reauth policy is set.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
}

impl AsRef<str> for ReauthSettingPolicyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReauthSettingPolicyTypeEnum::POLICYTYPEUNSPECIFIED => "POLICY_TYPE_UNSPECIFIED",
            ReauthSettingPolicyTypeEnum::MINIMUM => "MINIMUM",
            ReauthSettingPolicyTypeEnum::DEFAULT => "DEFAULT",
        }
    }
}

impl std::convert::TryFrom< &str> for ReauthSettingPolicyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POLICY_TYPE_UNSPECIFIED" => Ok(ReauthSettingPolicyTypeEnum::POLICYTYPEUNSPECIFIED),
           "MINIMUM" => Ok(ReauthSettingPolicyTypeEnum::MINIMUM),
           "DEFAULT" => Ok(ReauthSettingPolicyTypeEnum::DEFAULT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReauthSettingPolicyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


