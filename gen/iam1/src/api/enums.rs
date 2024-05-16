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


// region CreateServiceAccountKeyRequestKeyAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Which type of key and algorithm to use for the key. The default is currently a 2K RSA key. However this may change in the future.
pub enum CreateServiceAccountKeyRequestKeyAlgorithmEnum {
    

    /// An unspecified key algorithm.
    ///
    /// "KEY_ALG_UNSPECIFIED"
    #[serde(rename="KEY_ALG_UNSPECIFIED")]
    KEYALGUNSPECIFIED,
    

    /// 1k RSA Key.
    ///
    /// "KEY_ALG_RSA_1024"
    #[serde(rename="KEY_ALG_RSA_1024")]
    KEYALGRSA1024,
    

    /// 2k RSA Key.
    ///
    /// "KEY_ALG_RSA_2048"
    #[serde(rename="KEY_ALG_RSA_2048")]
    KEYALGRSA2048,
}

impl AsRef<str> for CreateServiceAccountKeyRequestKeyAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateServiceAccountKeyRequestKeyAlgorithmEnum::KEYALGUNSPECIFIED => "KEY_ALG_UNSPECIFIED",
            CreateServiceAccountKeyRequestKeyAlgorithmEnum::KEYALGRSA1024 => "KEY_ALG_RSA_1024",
            CreateServiceAccountKeyRequestKeyAlgorithmEnum::KEYALGRSA2048 => "KEY_ALG_RSA_2048",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateServiceAccountKeyRequestKeyAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_ALG_UNSPECIFIED" => Ok(CreateServiceAccountKeyRequestKeyAlgorithmEnum::KEYALGUNSPECIFIED),
           "KEY_ALG_RSA_1024" => Ok(CreateServiceAccountKeyRequestKeyAlgorithmEnum::KEYALGRSA1024),
           "KEY_ALG_RSA_2048" => Ok(CreateServiceAccountKeyRequestKeyAlgorithmEnum::KEYALGRSA2048),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateServiceAccountKeyRequestKeyAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreateServiceAccountKeyRequestPrivateKeyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The output format of the private key. The default value is `TYPE_GOOGLE_CREDENTIALS_FILE`, which is the Google Credentials File format.
pub enum CreateServiceAccountKeyRequestPrivateKeyTypeEnum {
    

    /// Unspecified. Equivalent to `TYPE_GOOGLE_CREDENTIALS_FILE`.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// PKCS12 format. The password for the PKCS12 file is `notasecret`. For more information, see https://tools.ietf.org/html/rfc7292.
    ///
    /// "TYPE_PKCS12_FILE"
    #[serde(rename="TYPE_PKCS12_FILE")]
    TYPEPKCS12FILE,
    

    /// Google Credentials File format.
    ///
    /// "TYPE_GOOGLE_CREDENTIALS_FILE"
    #[serde(rename="TYPE_GOOGLE_CREDENTIALS_FILE")]
    TYPEGOOGLECREDENTIALSFILE,
}

impl AsRef<str> for CreateServiceAccountKeyRequestPrivateKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateServiceAccountKeyRequestPrivateKeyTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            CreateServiceAccountKeyRequestPrivateKeyTypeEnum::TYPEPKCS12FILE => "TYPE_PKCS12_FILE",
            CreateServiceAccountKeyRequestPrivateKeyTypeEnum::TYPEGOOGLECREDENTIALSFILE => "TYPE_GOOGLE_CREDENTIALS_FILE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateServiceAccountKeyRequestPrivateKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(CreateServiceAccountKeyRequestPrivateKeyTypeEnum::TYPEUNSPECIFIED),
           "TYPE_PKCS12_FILE" => Ok(CreateServiceAccountKeyRequestPrivateKeyTypeEnum::TYPEPKCS12FILE),
           "TYPE_GOOGLE_CREDENTIALS_FILE" => Ok(CreateServiceAccountKeyRequestPrivateKeyTypeEnum::TYPEGOOGLECREDENTIALSFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateServiceAccountKeyRequestPrivateKeyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Describes the reason this key is being disabled. If unspecified, the default value of SERVICE_ACCOUNT_KEY_DISABLE_REASON_USER_INITIATED will be used.
pub enum DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum {
    

    /// Unspecified disable reason
    ///
    /// "SERVICE_ACCOUNT_KEY_DISABLE_REASON_UNSPECIFIED"
    #[serde(rename="SERVICE_ACCOUNT_KEY_DISABLE_REASON_UNSPECIFIED")]
    SERVICEACCOUNTKEYDISABLEREASONUNSPECIFIED,
    

    /// Disabled by the user
    ///
    /// "SERVICE_ACCOUNT_KEY_DISABLE_REASON_USER_INITIATED"
    #[serde(rename="SERVICE_ACCOUNT_KEY_DISABLE_REASON_USER_INITIATED")]
    SERVICEACCOUNTKEYDISABLEREASONUSERINITIATED,
    

    /// Google detected this Service Account external key's private key data as exposed, typically in a public repository on GitHub or similar.
    ///
    /// "SERVICE_ACCOUNT_KEY_DISABLE_REASON_EXPOSED"
    #[serde(rename="SERVICE_ACCOUNT_KEY_DISABLE_REASON_EXPOSED")]
    SERVICEACCOUNTKEYDISABLEREASONEXPOSED,
    

    /// This service account external key was detected as compromised and used by an attacker.
    ///
    /// "SERVICE_ACCOUNT_KEY_DISABLE_REASON_COMPROMISE_DETECTED"
    #[serde(rename="SERVICE_ACCOUNT_KEY_DISABLE_REASON_COMPROMISE_DETECTED")]
    SERVICEACCOUNTKEYDISABLEREASONCOMPROMISEDETECTED,
}

impl AsRef<str> for DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONUNSPECIFIED => "SERVICE_ACCOUNT_KEY_DISABLE_REASON_UNSPECIFIED",
            DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONUSERINITIATED => "SERVICE_ACCOUNT_KEY_DISABLE_REASON_USER_INITIATED",
            DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONEXPOSED => "SERVICE_ACCOUNT_KEY_DISABLE_REASON_EXPOSED",
            DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONCOMPROMISEDETECTED => "SERVICE_ACCOUNT_KEY_DISABLE_REASON_COMPROMISE_DETECTED",
        }
    }
}

impl std::convert::TryFrom< &str> for DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVICE_ACCOUNT_KEY_DISABLE_REASON_UNSPECIFIED" => Ok(DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONUNSPECIFIED),
           "SERVICE_ACCOUNT_KEY_DISABLE_REASON_USER_INITIATED" => Ok(DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONUSERINITIATED),
           "SERVICE_ACCOUNT_KEY_DISABLE_REASON_EXPOSED" => Ok(DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONEXPOSED),
           "SERVICE_ACCOUNT_KEY_DISABLE_REASON_COMPROMISE_DETECTED" => Ok(DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONCOMPROMISEDETECTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExtendedStatusKeyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The key for this extended status.
pub enum ExtendedStatusKeyEnum {
    

    /// Unspecified extended status, should not be used.
    ///
    /// "SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_UNSPECIFIED"
    #[serde(rename="SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_UNSPECIFIED")]
    SERVICEACCOUNTKEYEXTENDEDSTATUSKEYUNSPECIFIED,
    

    /// This key has been detected as exposed. extended_status_value may contain information about the exposure (public GitHub repo, open internet, etc.)
    ///
    /// "SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_EXPOSED"
    #[serde(rename="SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_EXPOSED")]
    SERVICEACCOUNTKEYEXTENDEDSTATUSKEYEXPOSED,
    

    /// This key was implicated in a compromise or other attack. extended_status_value may contain information about the abuse perpetrated.
    ///
    /// "SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_COMPROMISE_DETECTED"
    #[serde(rename="SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_COMPROMISE_DETECTED")]
    SERVICEACCOUNTKEYEXTENDEDSTATUSKEYCOMPROMISEDETECTED,
}

impl AsRef<str> for ExtendedStatusKeyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExtendedStatusKeyEnum::SERVICEACCOUNTKEYEXTENDEDSTATUSKEYUNSPECIFIED => "SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_UNSPECIFIED",
            ExtendedStatusKeyEnum::SERVICEACCOUNTKEYEXTENDEDSTATUSKEYEXPOSED => "SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_EXPOSED",
            ExtendedStatusKeyEnum::SERVICEACCOUNTKEYEXTENDEDSTATUSKEYCOMPROMISEDETECTED => "SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_COMPROMISE_DETECTED",
        }
    }
}

impl std::convert::TryFrom< &str> for ExtendedStatusKeyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_UNSPECIFIED" => Ok(ExtendedStatusKeyEnum::SERVICEACCOUNTKEYEXTENDEDSTATUSKEYUNSPECIFIED),
           "SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_EXPOSED" => Ok(ExtendedStatusKeyEnum::SERVICEACCOUNTKEYEXTENDEDSTATUSKEYEXPOSED),
           "SERVICE_ACCOUNT_KEY_EXTENDED_STATUS_KEY_COMPROMISE_DETECTED" => Ok(ExtendedStatusKeyEnum::SERVICEACCOUNTKEYEXTENDEDSTATUSKEYCOMPROMISEDETECTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExtendedStatusKeyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientAttributesTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Represents the IdP and type of claims that should be fetched.
pub enum GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientAttributesTypeEnum {
    

    /// No AttributesType specified.
    ///
    /// "ATTRIBUTES_TYPE_UNSPECIFIED"
    #[serde(rename="ATTRIBUTES_TYPE_UNSPECIFIED")]
    ATTRIBUTESTYPEUNSPECIFIED,
    

    /// Used to get the user's group claims from the Azure AD identity provider using configuration provided in ExtraAttributesOAuth2Client and `mail` property of the `microsoft.graph.group` object is used for claim mapping. See https://learn.microsoft.com/en-us/graph/api/resources/group?view=graph-rest-1.0#properties for more details on `microsoft.graph.group` properties. The attributes obtained from idntity provider are mapped to `assertion.groups`.
    ///
    /// "AZURE_AD_GROUPS_MAIL"
    #[serde(rename="AZURE_AD_GROUPS_MAIL")]
    AZUREADGROUPSMAIL,
}

impl AsRef<str> for GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientAttributesTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientAttributesTypeEnum::ATTRIBUTESTYPEUNSPECIFIED => "ATTRIBUTES_TYPE_UNSPECIFIED",
            GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientAttributesTypeEnum::AZUREADGROUPSMAIL => "AZURE_AD_GROUPS_MAIL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientAttributesTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTRIBUTES_TYPE_UNSPECIFIED" => Ok(GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientAttributesTypeEnum::ATTRIBUTESTYPEUNSPECIFIED),
           "AZURE_AD_GROUPS_MAIL" => Ok(GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientAttributesTypeEnum::AZUREADGROUPSMAIL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientAttributesTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The behavior for how OIDC Claims are included in the `assertion` object used for attribute mapping and attribute condition.
pub enum GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum {
    

    /// No assertion claims behavior specified.
    ///
    /// "ASSERTION_CLAIMS_BEHAVIOR_UNSPECIFIED"
    #[serde(rename="ASSERTION_CLAIMS_BEHAVIOR_UNSPECIFIED")]
    ASSERTIONCLAIMSBEHAVIORUNSPECIFIED,
    

    /// Merge the UserInfo Endpoint Claims with ID Token Claims, preferring UserInfo Claim Values for the same Claim Name. This option is available only for the Authorization Code Flow.
    ///
    /// "MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS"
    #[serde(rename="MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS")]
    MERGEUSERINFOOVERIDTOKENCLAIMS,
    

    /// Only include ID Token Claims.
    ///
    /// "ONLY_ID_TOKEN_CLAIMS"
    #[serde(rename="ONLY_ID_TOKEN_CLAIMS")]
    ONLYIDTOKENCLAIMS,
}

impl AsRef<str> for GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum::ASSERTIONCLAIMSBEHAVIORUNSPECIFIED => "ASSERTION_CLAIMS_BEHAVIOR_UNSPECIFIED",
            GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum::MERGEUSERINFOOVERIDTOKENCLAIMS => "MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS",
            GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum::ONLYIDTOKENCLAIMS => "ONLY_ID_TOKEN_CLAIMS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSERTION_CLAIMS_BEHAVIOR_UNSPECIFIED" => Ok(GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum::ASSERTIONCLAIMSBEHAVIORUNSPECIFIED),
           "MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS" => Ok(GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum::MERGEUSERINFOOVERIDTOKENCLAIMS),
           "ONLY_ID_TOKEN_CLAIMS" => Ok(GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum::ONLYIDTOKENCLAIMS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The Response Type to request for in the OIDC Authorization Request for web sign-in. The `CODE` Response Type is recommended to avoid the Implicit Flow, for security reasons.
pub enum GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum {
    

    /// No Response Type specified.
    ///
    /// "RESPONSE_TYPE_UNSPECIFIED"
    #[serde(rename="RESPONSE_TYPE_UNSPECIFIED")]
    RESPONSETYPEUNSPECIFIED,
    

    /// The `response_type=code` selection uses the Authorization Code Flow for web sign-in. Requires a configured client secret.
    ///
    /// "CODE"
    #[serde(rename="CODE")]
    CODE,
    

    /// The `response_type=id_token` selection uses the Implicit Flow for web sign-in.
    ///
    /// "ID_TOKEN"
    #[serde(rename="ID_TOKEN")]
    IDTOKEN,
}

impl AsRef<str> for GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum::RESPONSETYPEUNSPECIFIED => "RESPONSE_TYPE_UNSPECIFIED",
            GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum::CODE => "CODE",
            GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum::IDTOKEN => "ID_TOKEN",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESPONSE_TYPE_UNSPECIFIED" => Ok(GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum::RESPONSETYPEUNSPECIFIED),
           "CODE" => Ok(GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum::CODE),
           "ID_TOKEN" => Ok(GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum::IDTOKEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region KeyDataFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The format of the key.
pub enum KeyDataFormatEnum {
    

    /// No format has been specified. This is an invalid format and must not be used.
    ///
    /// "KEY_FORMAT_UNSPECIFIED"
    #[serde(rename="KEY_FORMAT_UNSPECIFIED")]
    KEYFORMATUNSPECIFIED,
    

    /// A RSA public key wrapped in an X.509v3 certificate ([RFC5280] ( https://www.ietf.org/rfc/rfc5280.txt)), encoded in base64, and wrapped in [public certificate label](https://datatracker.ietf.org/doc/html/rfc7468#section-5.1).
    ///
    /// "RSA_X509_PEM"
    #[serde(rename="RSA_X509_PEM")]
    RSAX509PEM,
}

impl AsRef<str> for KeyDataFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KeyDataFormatEnum::KEYFORMATUNSPECIFIED => "KEY_FORMAT_UNSPECIFIED",
            KeyDataFormatEnum::RSAX509PEM => "RSA_X509_PEM",
        }
    }
}

impl std::convert::TryFrom< &str> for KeyDataFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_FORMAT_UNSPECIFIED" => Ok(KeyDataFormatEnum::KEYFORMATUNSPECIFIED),
           "RSA_X509_PEM" => Ok(KeyDataFormatEnum::RSAX509PEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a KeyDataFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region KeyDataKeySpecEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The specifications for the key.
pub enum KeyDataKeySpecEnum {
    

    /// No key specification specified.
    ///
    /// "KEY_SPEC_UNSPECIFIED"
    #[serde(rename="KEY_SPEC_UNSPECIFIED")]
    KEYSPECUNSPECIFIED,
    

    /// A 2048 bit RSA key.
    ///
    /// "RSA_2048"
    #[serde(rename="RSA_2048")]
    RSA2048,
    

    /// A 3072 bit RSA key.
    ///
    /// "RSA_3072"
    #[serde(rename="RSA_3072")]
    RSA3072,
    

    /// A 4096 bit RSA key.
    ///
    /// "RSA_4096"
    #[serde(rename="RSA_4096")]
    RSA4096,
}

impl AsRef<str> for KeyDataKeySpecEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KeyDataKeySpecEnum::KEYSPECUNSPECIFIED => "KEY_SPEC_UNSPECIFIED",
            KeyDataKeySpecEnum::RSA2048 => "RSA_2048",
            KeyDataKeySpecEnum::RSA3072 => "RSA_3072",
            KeyDataKeySpecEnum::RSA4096 => "RSA_4096",
        }
    }
}

impl std::convert::TryFrom< &str> for KeyDataKeySpecEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_SPEC_UNSPECIFIED" => Ok(KeyDataKeySpecEnum::KEYSPECUNSPECIFIED),
           "RSA_2048" => Ok(KeyDataKeySpecEnum::RSA2048),
           "RSA_3072" => Ok(KeyDataKeySpecEnum::RSA3072),
           "RSA_4096" => Ok(KeyDataKeySpecEnum::RSA4096),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a KeyDataKeySpecEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LintResultLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The validation unit level.
pub enum LintResultLevelEnum {
    

    /// Level is unspecified.
    ///
    /// "LEVEL_UNSPECIFIED"
    #[serde(rename="LEVEL_UNSPECIFIED")]
    LEVELUNSPECIFIED,
    

    /// A validation unit which operates on an individual condition within a binding.
    ///
    /// "CONDITION"
    #[serde(rename="CONDITION")]
    CONDITION,
}

impl AsRef<str> for LintResultLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LintResultLevelEnum::LEVELUNSPECIFIED => "LEVEL_UNSPECIFIED",
            LintResultLevelEnum::CONDITION => "CONDITION",
        }
    }
}

impl std::convert::TryFrom< &str> for LintResultLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LEVEL_UNSPECIFIED" => Ok(LintResultLevelEnum::LEVELUNSPECIFIED),
           "CONDITION" => Ok(LintResultLevelEnum::CONDITION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LintResultLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LintResultSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The validation unit severity.
pub enum LintResultSeverityEnum {
    

    /// Severity is unspecified.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// A validation unit returns an error only for critical issues. If an attempt is made to set the problematic policy without rectifying the critical issue, it causes the `setPolicy` operation to fail.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Any issue which is severe enough but does not cause an error. For example, suspicious constructs in the input object will not necessarily fail `setPolicy`, but there is a high likelihood that they won't behave as expected during policy evaluation in `checkPolicy`. This includes the following common scenarios: - Unsatisfiable condition: Expired timestamp in date/time condition. - Ineffective condition: Condition on a pair which is granted unconditionally in another binding of the same policy.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Reserved for the issues that are not severe as `ERROR`/`WARNING`, but need special handling. For instance, messages about skipped validation units are issued as `NOTICE`.
    ///
    /// "NOTICE"
    #[serde(rename="NOTICE")]
    NOTICE,
    

    /// Any informative statement which is not severe enough to raise `ERROR`/`WARNING`/`NOTICE`, like auto-correction recommendations on the input content. Note that current version of the linter does not utilize `INFO`.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// Deprecated severity level.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for LintResultSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LintResultSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            LintResultSeverityEnum::ERROR => "ERROR",
            LintResultSeverityEnum::WARNING => "WARNING",
            LintResultSeverityEnum::NOTICE => "NOTICE",
            LintResultSeverityEnum::INFO => "INFO",
            LintResultSeverityEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for LintResultSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(LintResultSeverityEnum::SEVERITYUNSPECIFIED),
           "ERROR" => Ok(LintResultSeverityEnum::ERROR),
           "WARNING" => Ok(LintResultSeverityEnum::WARNING),
           "NOTICE" => Ok(LintResultSeverityEnum::NOTICE),
           "INFO" => Ok(LintResultSeverityEnum::INFO),
           "DEPRECATED" => Ok(LintResultSeverityEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LintResultSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OauthClientAllowedGrantTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The list of OAuth grant type is allowed for the oauth client.
pub enum OauthClientAllowedGrantTypesEnum {
    

    /// should not be used
    ///
    /// "GRANT_TYPE_UNSPECIFIED"
    #[serde(rename="GRANT_TYPE_UNSPECIFIED")]
    GRANTTYPEUNSPECIFIED,
    

    /// authorization code grant
    ///
    /// "AUTHORIZATION_CODE_GRANT"
    #[serde(rename="AUTHORIZATION_CODE_GRANT")]
    AUTHORIZATIONCODEGRANT,
    

    /// refresh token grant
    ///
    /// "REFRESH_TOKEN_GRANT"
    #[serde(rename="REFRESH_TOKEN_GRANT")]
    REFRESHTOKENGRANT,
}

impl AsRef<str> for OauthClientAllowedGrantTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OauthClientAllowedGrantTypesEnum::GRANTTYPEUNSPECIFIED => "GRANT_TYPE_UNSPECIFIED",
            OauthClientAllowedGrantTypesEnum::AUTHORIZATIONCODEGRANT => "AUTHORIZATION_CODE_GRANT",
            OauthClientAllowedGrantTypesEnum::REFRESHTOKENGRANT => "REFRESH_TOKEN_GRANT",
        }
    }
}

impl std::convert::TryFrom< &str> for OauthClientAllowedGrantTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GRANT_TYPE_UNSPECIFIED" => Ok(OauthClientAllowedGrantTypesEnum::GRANTTYPEUNSPECIFIED),
           "AUTHORIZATION_CODE_GRANT" => Ok(OauthClientAllowedGrantTypesEnum::AUTHORIZATIONCODEGRANT),
           "REFRESH_TOKEN_GRANT" => Ok(OauthClientAllowedGrantTypesEnum::REFRESHTOKENGRANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OauthClientAllowedGrantTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OauthClientClientTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The type of oauth client. either public or private.
pub enum OauthClientClientTypeEnum {
    

    /// should not be used
    ///
    /// "CLIENT_TYPE_UNSPECIFIED"
    #[serde(rename="CLIENT_TYPE_UNSPECIFIED")]
    CLIENTTYPEUNSPECIFIED,
    

    /// public client has no secret
    ///
    /// "PUBLIC_CLIENT"
    #[serde(rename="PUBLIC_CLIENT")]
    PUBLICCLIENT,
    

    /// private client
    ///
    /// "CONFIDENTIAL_CLIENT"
    #[serde(rename="CONFIDENTIAL_CLIENT")]
    CONFIDENTIALCLIENT,
}

impl AsRef<str> for OauthClientClientTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OauthClientClientTypeEnum::CLIENTTYPEUNSPECIFIED => "CLIENT_TYPE_UNSPECIFIED",
            OauthClientClientTypeEnum::PUBLICCLIENT => "PUBLIC_CLIENT",
            OauthClientClientTypeEnum::CONFIDENTIALCLIENT => "CONFIDENTIAL_CLIENT",
        }
    }
}

impl std::convert::TryFrom< &str> for OauthClientClientTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLIENT_TYPE_UNSPECIFIED" => Ok(OauthClientClientTypeEnum::CLIENTTYPEUNSPECIFIED),
           "PUBLIC_CLIENT" => Ok(OauthClientClientTypeEnum::PUBLICCLIENT),
           "CONFIDENTIAL_CLIENT" => Ok(OauthClientClientTypeEnum::CONFIDENTIALCLIENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OauthClientClientTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OauthClientStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the oauth client.
pub enum OauthClientStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The oauth client is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The oauth client is soft-deleted. Soft-deleted oauth client is permanently deleted after approximately 30 days unless restored via UndeleteOauthClient.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for OauthClientStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OauthClientStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            OauthClientStateEnum::ACTIVE => "ACTIVE",
            OauthClientStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for OauthClientStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(OauthClientStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(OauthClientStateEnum::ACTIVE),
           "DELETED" => Ok(OauthClientStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OauthClientStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PermissionCustomRolesSupportLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current custom role support level.
pub enum PermissionCustomRolesSupportLevelEnum {
    

    /// Default state. Permission is fully supported for custom role use.
    ///
    /// "SUPPORTED"
    #[serde(rename="SUPPORTED")]
    SUPPORTED,
    

    /// Permission is being tested to check custom role compatibility.
    ///
    /// "TESTING"
    #[serde(rename="TESTING")]
    TESTING,
    

    /// Permission is not supported for custom role use.
    ///
    /// "NOT_SUPPORTED"
    #[serde(rename="NOT_SUPPORTED")]
    NOTSUPPORTED,
}

impl AsRef<str> for PermissionCustomRolesSupportLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PermissionCustomRolesSupportLevelEnum::SUPPORTED => "SUPPORTED",
            PermissionCustomRolesSupportLevelEnum::TESTING => "TESTING",
            PermissionCustomRolesSupportLevelEnum::NOTSUPPORTED => "NOT_SUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for PermissionCustomRolesSupportLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUPPORTED" => Ok(PermissionCustomRolesSupportLevelEnum::SUPPORTED),
           "TESTING" => Ok(PermissionCustomRolesSupportLevelEnum::TESTING),
           "NOT_SUPPORTED" => Ok(PermissionCustomRolesSupportLevelEnum::NOTSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PermissionCustomRolesSupportLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PermissionStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current launch stage of the permission.
pub enum PermissionStageEnum {
    

    /// The permission is currently in an alpha phase.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// The permission is currently in a beta phase.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// The permission is generally available.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// The permission is being deprecated.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for PermissionStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PermissionStageEnum::ALPHA => "ALPHA",
            PermissionStageEnum::BETA => "BETA",
            PermissionStageEnum::GA => "GA",
            PermissionStageEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for PermissionStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALPHA" => Ok(PermissionStageEnum::ALPHA),
           "BETA" => Ok(PermissionStageEnum::BETA),
           "GA" => Ok(PermissionStageEnum::GA),
           "DEPRECATED" => Ok(PermissionStageEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PermissionStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueryGrantableRolesRequestViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryGrantableRolesRequestViewEnum {
    

    /// Omits the `included_permissions` field. This is the default value.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Returns all fields.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for QueryGrantableRolesRequestViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueryGrantableRolesRequestViewEnum::BASIC => "BASIC",
            QueryGrantableRolesRequestViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for QueryGrantableRolesRequestViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC" => Ok(QueryGrantableRolesRequestViewEnum::BASIC),
           "FULL" => Ok(QueryGrantableRolesRequestViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueryGrantableRolesRequestViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RoleStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current launch stage of the role. If the `ALPHA` launch stage has been selected for a role, the `stage` field will not be included in the returned definition for the role.
pub enum RoleStageEnum {
    

    /// The user has indicated this role is currently in an Alpha phase. If this launch stage is selected, the `stage` field will not be included when requesting the definition for a given role.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// The user has indicated this role is currently in a Beta phase.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// The user has indicated this role is generally available.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// The user has indicated this role is being deprecated.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
    

    /// This role is disabled and will not contribute permissions to any principals it is granted to in policies.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// The user has indicated this role is currently in an EAP phase.
    ///
    /// "EAP"
    #[serde(rename="EAP")]
    EAP,
}

impl AsRef<str> for RoleStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RoleStageEnum::ALPHA => "ALPHA",
            RoleStageEnum::BETA => "BETA",
            RoleStageEnum::GA => "GA",
            RoleStageEnum::DEPRECATED => "DEPRECATED",
            RoleStageEnum::DISABLED => "DISABLED",
            RoleStageEnum::EAP => "EAP",
        }
    }
}

impl std::convert::TryFrom< &str> for RoleStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALPHA" => Ok(RoleStageEnum::ALPHA),
           "BETA" => Ok(RoleStageEnum::BETA),
           "GA" => Ok(RoleStageEnum::GA),
           "DEPRECATED" => Ok(RoleStageEnum::DEPRECATED),
           "DISABLED" => Ok(RoleStageEnum::DISABLED),
           "EAP" => Ok(RoleStageEnum::EAP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RoleStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceAccountKeyDisableReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// optional. If the key is disabled, it may have a DisableReason describing why it was disabled.
pub enum ServiceAccountKeyDisableReasonEnum {
    

    /// Unspecified disable reason
    ///
    /// "SERVICE_ACCOUNT_KEY_DISABLE_REASON_UNSPECIFIED"
    #[serde(rename="SERVICE_ACCOUNT_KEY_DISABLE_REASON_UNSPECIFIED")]
    SERVICEACCOUNTKEYDISABLEREASONUNSPECIFIED,
    

    /// Disabled by the user
    ///
    /// "SERVICE_ACCOUNT_KEY_DISABLE_REASON_USER_INITIATED"
    #[serde(rename="SERVICE_ACCOUNT_KEY_DISABLE_REASON_USER_INITIATED")]
    SERVICEACCOUNTKEYDISABLEREASONUSERINITIATED,
    

    /// Google detected this Service Account external key's private key data as exposed, typically in a public repository on GitHub or similar.
    ///
    /// "SERVICE_ACCOUNT_KEY_DISABLE_REASON_EXPOSED"
    #[serde(rename="SERVICE_ACCOUNT_KEY_DISABLE_REASON_EXPOSED")]
    SERVICEACCOUNTKEYDISABLEREASONEXPOSED,
    

    /// This service account external key was detected as compromised and used by an attacker.
    ///
    /// "SERVICE_ACCOUNT_KEY_DISABLE_REASON_COMPROMISE_DETECTED"
    #[serde(rename="SERVICE_ACCOUNT_KEY_DISABLE_REASON_COMPROMISE_DETECTED")]
    SERVICEACCOUNTKEYDISABLEREASONCOMPROMISEDETECTED,
}

impl AsRef<str> for ServiceAccountKeyDisableReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONUNSPECIFIED => "SERVICE_ACCOUNT_KEY_DISABLE_REASON_UNSPECIFIED",
            ServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONUSERINITIATED => "SERVICE_ACCOUNT_KEY_DISABLE_REASON_USER_INITIATED",
            ServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONEXPOSED => "SERVICE_ACCOUNT_KEY_DISABLE_REASON_EXPOSED",
            ServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONCOMPROMISEDETECTED => "SERVICE_ACCOUNT_KEY_DISABLE_REASON_COMPROMISE_DETECTED",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceAccountKeyDisableReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVICE_ACCOUNT_KEY_DISABLE_REASON_UNSPECIFIED" => Ok(ServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONUNSPECIFIED),
           "SERVICE_ACCOUNT_KEY_DISABLE_REASON_USER_INITIATED" => Ok(ServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONUSERINITIATED),
           "SERVICE_ACCOUNT_KEY_DISABLE_REASON_EXPOSED" => Ok(ServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONEXPOSED),
           "SERVICE_ACCOUNT_KEY_DISABLE_REASON_COMPROMISE_DETECTED" => Ok(ServiceAccountKeyDisableReasonEnum::SERVICEACCOUNTKEYDISABLEREASONCOMPROMISEDETECTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceAccountKeyDisableReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceAccountKeyKeyAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the algorithm (and possibly key size) for the key.
pub enum ServiceAccountKeyKeyAlgorithmEnum {
    

    /// An unspecified key algorithm.
    ///
    /// "KEY_ALG_UNSPECIFIED"
    #[serde(rename="KEY_ALG_UNSPECIFIED")]
    KEYALGUNSPECIFIED,
    

    /// 1k RSA Key.
    ///
    /// "KEY_ALG_RSA_1024"
    #[serde(rename="KEY_ALG_RSA_1024")]
    KEYALGRSA1024,
    

    /// 2k RSA Key.
    ///
    /// "KEY_ALG_RSA_2048"
    #[serde(rename="KEY_ALG_RSA_2048")]
    KEYALGRSA2048,
}

impl AsRef<str> for ServiceAccountKeyKeyAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceAccountKeyKeyAlgorithmEnum::KEYALGUNSPECIFIED => "KEY_ALG_UNSPECIFIED",
            ServiceAccountKeyKeyAlgorithmEnum::KEYALGRSA1024 => "KEY_ALG_RSA_1024",
            ServiceAccountKeyKeyAlgorithmEnum::KEYALGRSA2048 => "KEY_ALG_RSA_2048",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceAccountKeyKeyAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_ALG_UNSPECIFIED" => Ok(ServiceAccountKeyKeyAlgorithmEnum::KEYALGUNSPECIFIED),
           "KEY_ALG_RSA_1024" => Ok(ServiceAccountKeyKeyAlgorithmEnum::KEYALGRSA1024),
           "KEY_ALG_RSA_2048" => Ok(ServiceAccountKeyKeyAlgorithmEnum::KEYALGRSA2048),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceAccountKeyKeyAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceAccountKeyKeyOriginEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The key origin.
pub enum ServiceAccountKeyKeyOriginEnum {
    

    /// Unspecified key origin.
    ///
    /// "ORIGIN_UNSPECIFIED"
    #[serde(rename="ORIGIN_UNSPECIFIED")]
    ORIGINUNSPECIFIED,
    

    /// Key is provided by user.
    ///
    /// "USER_PROVIDED"
    #[serde(rename="USER_PROVIDED")]
    USERPROVIDED,
    

    /// Key is provided by Google.
    ///
    /// "GOOGLE_PROVIDED"
    #[serde(rename="GOOGLE_PROVIDED")]
    GOOGLEPROVIDED,
}

impl AsRef<str> for ServiceAccountKeyKeyOriginEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceAccountKeyKeyOriginEnum::ORIGINUNSPECIFIED => "ORIGIN_UNSPECIFIED",
            ServiceAccountKeyKeyOriginEnum::USERPROVIDED => "USER_PROVIDED",
            ServiceAccountKeyKeyOriginEnum::GOOGLEPROVIDED => "GOOGLE_PROVIDED",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceAccountKeyKeyOriginEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORIGIN_UNSPECIFIED" => Ok(ServiceAccountKeyKeyOriginEnum::ORIGINUNSPECIFIED),
           "USER_PROVIDED" => Ok(ServiceAccountKeyKeyOriginEnum::USERPROVIDED),
           "GOOGLE_PROVIDED" => Ok(ServiceAccountKeyKeyOriginEnum::GOOGLEPROVIDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceAccountKeyKeyOriginEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceAccountKeyKeyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The key type.
pub enum ServiceAccountKeyKeyTypeEnum {
    

    /// Unspecified key type. The presence of this in the message will immediately result in an error.
    ///
    /// "KEY_TYPE_UNSPECIFIED"
    #[serde(rename="KEY_TYPE_UNSPECIFIED")]
    KEYTYPEUNSPECIFIED,
    

    /// User-managed keys (managed and rotated by the user).
    ///
    /// "USER_MANAGED"
    #[serde(rename="USER_MANAGED")]
    USERMANAGED,
    

    /// System-managed keys (managed and rotated by Google).
    ///
    /// "SYSTEM_MANAGED"
    #[serde(rename="SYSTEM_MANAGED")]
    SYSTEMMANAGED,
}

impl AsRef<str> for ServiceAccountKeyKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceAccountKeyKeyTypeEnum::KEYTYPEUNSPECIFIED => "KEY_TYPE_UNSPECIFIED",
            ServiceAccountKeyKeyTypeEnum::USERMANAGED => "USER_MANAGED",
            ServiceAccountKeyKeyTypeEnum::SYSTEMMANAGED => "SYSTEM_MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceAccountKeyKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_TYPE_UNSPECIFIED" => Ok(ServiceAccountKeyKeyTypeEnum::KEYTYPEUNSPECIFIED),
           "USER_MANAGED" => Ok(ServiceAccountKeyKeyTypeEnum::USERMANAGED),
           "SYSTEM_MANAGED" => Ok(ServiceAccountKeyKeyTypeEnum::SYSTEMMANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceAccountKeyKeyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceAccountKeyPrivateKeyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The output format for the private key. Only provided in `CreateServiceAccountKey` responses, not in `GetServiceAccountKey` or `ListServiceAccountKey` responses. Google never exposes system-managed private keys, and never retains user-managed private keys.
pub enum ServiceAccountKeyPrivateKeyTypeEnum {
    

    /// Unspecified. Equivalent to `TYPE_GOOGLE_CREDENTIALS_FILE`.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// PKCS12 format. The password for the PKCS12 file is `notasecret`. For more information, see https://tools.ietf.org/html/rfc7292.
    ///
    /// "TYPE_PKCS12_FILE"
    #[serde(rename="TYPE_PKCS12_FILE")]
    TYPEPKCS12FILE,
    

    /// Google Credentials File format.
    ///
    /// "TYPE_GOOGLE_CREDENTIALS_FILE"
    #[serde(rename="TYPE_GOOGLE_CREDENTIALS_FILE")]
    TYPEGOOGLECREDENTIALSFILE,
}

impl AsRef<str> for ServiceAccountKeyPrivateKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceAccountKeyPrivateKeyTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            ServiceAccountKeyPrivateKeyTypeEnum::TYPEPKCS12FILE => "TYPE_PKCS12_FILE",
            ServiceAccountKeyPrivateKeyTypeEnum::TYPEGOOGLECREDENTIALSFILE => "TYPE_GOOGLE_CREDENTIALS_FILE",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceAccountKeyPrivateKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(ServiceAccountKeyPrivateKeyTypeEnum::TYPEUNSPECIFIED),
           "TYPE_PKCS12_FILE" => Ok(ServiceAccountKeyPrivateKeyTypeEnum::TYPEPKCS12FILE),
           "TYPE_GOOGLE_CREDENTIALS_FILE" => Ok(ServiceAccountKeyPrivateKeyTypeEnum::TYPEGOOGLECREDENTIALSFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceAccountKeyPrivateKeyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WorkforcePoolStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the pool.
pub enum WorkforcePoolStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The pool is active and may be used in Google Cloud policies.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The pool is soft-deleted. Soft-deleted pools are permanently deleted after approximately 30 days. You can restore a soft-deleted pool using UndeleteWorkforcePool. You cannot reuse the ID of a soft-deleted pool until it is permanently deleted. While a pool is deleted, you cannot use it to exchange tokens, or use existing tokens to access resources. If the pool is undeleted, existing tokens grant access again.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for WorkforcePoolStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkforcePoolStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            WorkforcePoolStateEnum::ACTIVE => "ACTIVE",
            WorkforcePoolStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkforcePoolStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(WorkforcePoolStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(WorkforcePoolStateEnum::ACTIVE),
           "DELETED" => Ok(WorkforcePoolStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkforcePoolStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WorkforcePoolProviderStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the provider.
pub enum WorkforcePoolProviderStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The provider is active and may be used to validate authentication credentials.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The provider is soft-deleted. Soft-deleted providers are permanently deleted after approximately 30 days. You can restore a soft-deleted provider using UndeleteWorkforcePoolProvider.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for WorkforcePoolProviderStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkforcePoolProviderStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            WorkforcePoolProviderStateEnum::ACTIVE => "ACTIVE",
            WorkforcePoolProviderStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkforcePoolProviderStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(WorkforcePoolProviderStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(WorkforcePoolProviderStateEnum::ACTIVE),
           "DELETED" => Ok(WorkforcePoolProviderStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkforcePoolProviderStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WorkforcePoolProviderKeyStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the key.
pub enum WorkforcePoolProviderKeyStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The key is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The key is soft-deleted. Soft-deleted keys are permanently deleted after approximately 30 days. You can restore a soft-deleted key using UndeleteWorkforcePoolProviderKey.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for WorkforcePoolProviderKeyStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkforcePoolProviderKeyStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            WorkforcePoolProviderKeyStateEnum::ACTIVE => "ACTIVE",
            WorkforcePoolProviderKeyStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkforcePoolProviderKeyStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(WorkforcePoolProviderKeyStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(WorkforcePoolProviderKeyStateEnum::ACTIVE),
           "DELETED" => Ok(WorkforcePoolProviderKeyStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkforcePoolProviderKeyStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WorkforcePoolProviderKeyUseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The purpose of the key.
pub enum WorkforcePoolProviderKeyUseEnum {
    

    /// KeyUse unspecified.
    ///
    /// "KEY_USE_UNSPECIFIED"
    #[serde(rename="KEY_USE_UNSPECIFIED")]
    KEYUSEUNSPECIFIED,
    

    /// The key is used for encryption.
    ///
    /// "ENCRYPTION"
    #[serde(rename="ENCRYPTION")]
    ENCRYPTION,
}

impl AsRef<str> for WorkforcePoolProviderKeyUseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkforcePoolProviderKeyUseEnum::KEYUSEUNSPECIFIED => "KEY_USE_UNSPECIFIED",
            WorkforcePoolProviderKeyUseEnum::ENCRYPTION => "ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkforcePoolProviderKeyUseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_USE_UNSPECIFIED" => Ok(WorkforcePoolProviderKeyUseEnum::KEYUSEUNSPECIFIED),
           "ENCRYPTION" => Ok(WorkforcePoolProviderKeyUseEnum::ENCRYPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkforcePoolProviderKeyUseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WorkloadIdentityPoolStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the pool.
pub enum WorkloadIdentityPoolStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The pool is active, and may be used in Google Cloud policies.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The pool is soft-deleted. Soft-deleted pools are permanently deleted after approximately 30 days. You can restore a soft-deleted pool using UndeleteWorkloadIdentityPool. You cannot reuse the ID of a soft-deleted pool until it is permanently deleted. While a pool is deleted, you cannot use it to exchange tokens, or use existing tokens to access resources. If the pool is undeleted, existing tokens grant access again.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for WorkloadIdentityPoolStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkloadIdentityPoolStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            WorkloadIdentityPoolStateEnum::ACTIVE => "ACTIVE",
            WorkloadIdentityPoolStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkloadIdentityPoolStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(WorkloadIdentityPoolStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(WorkloadIdentityPoolStateEnum::ACTIVE),
           "DELETED" => Ok(WorkloadIdentityPoolStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkloadIdentityPoolStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WorkloadIdentityPoolProviderStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the provider.
pub enum WorkloadIdentityPoolProviderStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The provider is active, and may be used to validate authentication credentials.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The provider is soft-deleted. Soft-deleted providers are permanently deleted after approximately 30 days. You can restore a soft-deleted provider using UndeleteWorkloadIdentityPoolProvider. You cannot reuse the ID of a soft-deleted provider until it is permanently deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for WorkloadIdentityPoolProviderStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkloadIdentityPoolProviderStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            WorkloadIdentityPoolProviderStateEnum::ACTIVE => "ACTIVE",
            WorkloadIdentityPoolProviderStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkloadIdentityPoolProviderStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(WorkloadIdentityPoolProviderStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(WorkloadIdentityPoolProviderStateEnum::ACTIVE),
           "DELETED" => Ok(WorkloadIdentityPoolProviderStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkloadIdentityPoolProviderStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WorkloadIdentityPoolProviderKeyStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the key.
pub enum WorkloadIdentityPoolProviderKeyStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The key is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The key is soft-deleted. Soft-deleted keys are permanently deleted after approximately 30 days. You can restore a soft-deleted key using UndeleteWorkloadIdentityPoolProviderKey. While a key is deleted, you cannot use it during the federation.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for WorkloadIdentityPoolProviderKeyStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkloadIdentityPoolProviderKeyStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            WorkloadIdentityPoolProviderKeyStateEnum::ACTIVE => "ACTIVE",
            WorkloadIdentityPoolProviderKeyStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkloadIdentityPoolProviderKeyStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(WorkloadIdentityPoolProviderKeyStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(WorkloadIdentityPoolProviderKeyStateEnum::ACTIVE),
           "DELETED" => Ok(WorkloadIdentityPoolProviderKeyStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkloadIdentityPoolProviderKeyStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WorkloadIdentityPoolProviderKeyUseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The purpose of the key.
pub enum WorkloadIdentityPoolProviderKeyUseEnum {
    

    /// The key use is not known.
    ///
    /// "KEY_USE_UNSPECIFIED"
    #[serde(rename="KEY_USE_UNSPECIFIED")]
    KEYUSEUNSPECIFIED,
    

    /// The public key is used for encryption purposes.
    ///
    /// "ENCRYPTION"
    #[serde(rename="ENCRYPTION")]
    ENCRYPTION,
}

impl AsRef<str> for WorkloadIdentityPoolProviderKeyUseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkloadIdentityPoolProviderKeyUseEnum::KEYUSEUNSPECIFIED => "KEY_USE_UNSPECIFIED",
            WorkloadIdentityPoolProviderKeyUseEnum::ENCRYPTION => "ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkloadIdentityPoolProviderKeyUseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_USE_UNSPECIFIED" => Ok(WorkloadIdentityPoolProviderKeyUseEnum::KEYUSEUNSPECIFIED),
           "ENCRYPTION" => Ok(WorkloadIdentityPoolProviderKeyUseEnum::ENCRYPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkloadIdentityPoolProviderKeyUseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional view for the returned Role objects. When `FULL` is specified, the `includedPermissions` field is returned, which includes a list of all permissions in the role. The default value is `BASIC`, which does not return the `includedPermissions` field.
pub enum OrganizationViewEnum {
    

    /// Omits the `included_permissions` field. This is the default value.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Returns all fields.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for OrganizationViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationViewEnum::BASIC => "BASIC",
            OrganizationViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC" => Ok(OrganizationViewEnum::BASIC),
           "FULL" => Ok(OrganizationViewEnum::FULL),
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
/// Optional view for the returned Role objects. When `FULL` is specified, the `includedPermissions` field is returned, which includes a list of all permissions in the role. The default value is `BASIC`, which does not return the `includedPermissions` field.
pub enum ProjectViewEnum {
    

    /// Omits the `included_permissions` field. This is the default value.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Returns all fields.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::BASIC => "BASIC",
            ProjectViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
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


// region ProjectPublicKeyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The output format of the public key. The default is `TYPE_NONE`, which means that the public key is not returned.
pub enum ProjectPublicKeyTypeEnum {
    

    /// Do not return the public key.
    ///
    /// "TYPE_NONE"
    #[serde(rename="TYPE_NONE")]
    TYPENONE,
    

    /// X509 PEM format.
    ///
    /// "TYPE_X509_PEM_FILE"
    #[serde(rename="TYPE_X509_PEM_FILE")]
    TYPEX509PEMFILE,
    

    /// Raw public key.
    ///
    /// "TYPE_RAW_PUBLIC_KEY"
    #[serde(rename="TYPE_RAW_PUBLIC_KEY")]
    TYPERAWPUBLICKEY,
}

impl AsRef<str> for ProjectPublicKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectPublicKeyTypeEnum::TYPENONE => "TYPE_NONE",
            ProjectPublicKeyTypeEnum::TYPEX509PEMFILE => "TYPE_X509_PEM_FILE",
            ProjectPublicKeyTypeEnum::TYPERAWPUBLICKEY => "TYPE_RAW_PUBLIC_KEY",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectPublicKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_NONE" => Ok(ProjectPublicKeyTypeEnum::TYPENONE),
           "TYPE_X509_PEM_FILE" => Ok(ProjectPublicKeyTypeEnum::TYPEX509PEMFILE),
           "TYPE_RAW_PUBLIC_KEY" => Ok(ProjectPublicKeyTypeEnum::TYPERAWPUBLICKEY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectPublicKeyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectKeyTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filters the types of keys the user wants to include in the list response. Duplicate key types are not allowed. If no key type is provided, all keys are returned.
pub enum ProjectKeyTypesEnum {
    

    /// Unspecified key type. The presence of this in the message will immediately result in an error.
    ///
    /// "KEY_TYPE_UNSPECIFIED"
    #[serde(rename="KEY_TYPE_UNSPECIFIED")]
    KEYTYPEUNSPECIFIED,
    

    /// User-managed keys (managed and rotated by the user).
    ///
    /// "USER_MANAGED"
    #[serde(rename="USER_MANAGED")]
    USERMANAGED,
    

    /// System-managed keys (managed and rotated by Google).
    ///
    /// "SYSTEM_MANAGED"
    #[serde(rename="SYSTEM_MANAGED")]
    SYSTEMMANAGED,
}

impl AsRef<str> for ProjectKeyTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectKeyTypesEnum::KEYTYPEUNSPECIFIED => "KEY_TYPE_UNSPECIFIED",
            ProjectKeyTypesEnum::USERMANAGED => "USER_MANAGED",
            ProjectKeyTypesEnum::SYSTEMMANAGED => "SYSTEM_MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectKeyTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_TYPE_UNSPECIFIED" => Ok(ProjectKeyTypesEnum::KEYTYPEUNSPECIFIED),
           "USER_MANAGED" => Ok(ProjectKeyTypesEnum::USERMANAGED),
           "SYSTEM_MANAGED" => Ok(ProjectKeyTypesEnum::SYSTEMMANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectKeyTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RoleViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional view for the returned Role objects. When `FULL` is specified, the `includedPermissions` field is returned, which includes a list of all permissions in the role. The default value is `BASIC`, which does not return the `includedPermissions` field.
pub enum RoleViewEnum {
    

    /// Omits the `included_permissions` field. This is the default value.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Returns all fields.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for RoleViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RoleViewEnum::BASIC => "BASIC",
            RoleViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for RoleViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC" => Ok(RoleViewEnum::BASIC),
           "FULL" => Ok(RoleViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RoleViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


