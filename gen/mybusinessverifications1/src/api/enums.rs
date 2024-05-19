use super::*;



// region ComplyWithGuidelineRecommendationReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason why the location is being recommended to comply with guidelines.
pub enum ComplyWithGuidelineRecommendationReasonEnum {
    

    /// Not specified.
    ///
    /// "RECOMMENDATION_REASON_UNSPECIFIED"
    #[serde(rename="RECOMMENDATION_REASON_UNSPECIFIED")]
    RECOMMENDATIONREASONUNSPECIFIED,
    

    /// The business location is suspended. To fix this issue, consult the [Help Center article](https://support.google.com/business/answer/4569145).
    ///
    /// "BUSINESS_LOCATION_SUSPENDED"
    #[serde(rename="BUSINESS_LOCATION_SUSPENDED")]
    BUSINESSLOCATIONSUSPENDED,
    

    /// The business location is disabled. To fix this issue, consult the [Help Center article](https://support.google.com/business/answer/9334246).
    ///
    /// "BUSINESS_LOCATION_DISABLED"
    #[serde(rename="BUSINESS_LOCATION_DISABLED")]
    BUSINESSLOCATIONDISABLED,
}

impl AsRef<str> for ComplyWithGuidelineRecommendationReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ComplyWithGuidelineRecommendationReasonEnum::RECOMMENDATIONREASONUNSPECIFIED => "RECOMMENDATION_REASON_UNSPECIFIED",
            ComplyWithGuidelineRecommendationReasonEnum::BUSINESSLOCATIONSUSPENDED => "BUSINESS_LOCATION_SUSPENDED",
            ComplyWithGuidelineRecommendationReasonEnum::BUSINESSLOCATIONDISABLED => "BUSINESS_LOCATION_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for ComplyWithGuidelineRecommendationReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECOMMENDATION_REASON_UNSPECIFIED" => Ok(ComplyWithGuidelineRecommendationReasonEnum::RECOMMENDATIONREASONUNSPECIFIED),
           "BUSINESS_LOCATION_SUSPENDED" => Ok(ComplyWithGuidelineRecommendationReasonEnum::BUSINESSLOCATIONSUSPENDED),
           "BUSINESS_LOCATION_DISABLED" => Ok(ComplyWithGuidelineRecommendationReasonEnum::BUSINESSLOCATIONDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ComplyWithGuidelineRecommendationReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VerificationMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The method of the verification.
pub enum VerificationMethodEnum {
    

    /// Default value, will result in errors.
    ///
    /// "VERIFICATION_METHOD_UNSPECIFIED"
    #[serde(rename="VERIFICATION_METHOD_UNSPECIFIED")]
    VERIFICATIONMETHODUNSPECIFIED,
    

    /// Send a postcard with a verification PIN to a specific mailing address. The PIN is used to complete verification with Google.
    ///
    /// "ADDRESS"
    #[serde(rename="ADDRESS")]
    ADDRESS,
    

    /// Send an email with a verification PIN to a specific email address. The PIN is used to complete verification with Google.
    ///
    /// "EMAIL"
    #[serde(rename="EMAIL")]
    EMAIL,
    

    /// Make a phone call with a verification PIN to a specific phone number. The PIN is used to complete verification with Google.
    ///
    /// "PHONE_CALL"
    #[serde(rename="PHONE_CALL")]
    PHONECALL,
    

    /// Send an SMS with a verification PIN to a specific phone number. The PIN is used to complete verification with Google.
    ///
    /// "SMS"
    #[serde(rename="SMS")]
    SMS,
    

    /// Verify the location without additional user action. This option may not be available for all locations.
    ///
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
    

    /// This option may not be available for all locations.
    ///
    /// "VETTED_PARTNER"
    #[serde(rename="VETTED_PARTNER")]
    VETTEDPARTNER,
}

impl AsRef<str> for VerificationMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VerificationMethodEnum::VERIFICATIONMETHODUNSPECIFIED => "VERIFICATION_METHOD_UNSPECIFIED",
            VerificationMethodEnum::ADDRESS => "ADDRESS",
            VerificationMethodEnum::EMAIL => "EMAIL",
            VerificationMethodEnum::PHONECALL => "PHONE_CALL",
            VerificationMethodEnum::SMS => "SMS",
            VerificationMethodEnum::AUTO => "AUTO",
            VerificationMethodEnum::VETTEDPARTNER => "VETTED_PARTNER",
        }
    }
}

impl std::convert::TryFrom< &str> for VerificationMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERIFICATION_METHOD_UNSPECIFIED" => Ok(VerificationMethodEnum::VERIFICATIONMETHODUNSPECIFIED),
           "ADDRESS" => Ok(VerificationMethodEnum::ADDRESS),
           "EMAIL" => Ok(VerificationMethodEnum::EMAIL),
           "PHONE_CALL" => Ok(VerificationMethodEnum::PHONECALL),
           "SMS" => Ok(VerificationMethodEnum::SMS),
           "AUTO" => Ok(VerificationMethodEnum::AUTO),
           "VETTED_PARTNER" => Ok(VerificationMethodEnum::VETTEDPARTNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VerificationMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VerificationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the verification.
pub enum VerificationStateEnum {
    

    /// Default value, will result in errors.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The verification is pending.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The verification is completed.
    ///
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// The verification is failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for VerificationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VerificationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            VerificationStateEnum::PENDING => "PENDING",
            VerificationStateEnum::COMPLETED => "COMPLETED",
            VerificationStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for VerificationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(VerificationStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(VerificationStateEnum::PENDING),
           "COMPLETED" => Ok(VerificationStateEnum::COMPLETED),
           "FAILED" => Ok(VerificationStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VerificationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VerificationOptionVerificationMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Method to verify the location.
pub enum VerificationOptionVerificationMethodEnum {
    

    /// Default value, will result in errors.
    ///
    /// "VERIFICATION_METHOD_UNSPECIFIED"
    #[serde(rename="VERIFICATION_METHOD_UNSPECIFIED")]
    VERIFICATIONMETHODUNSPECIFIED,
    

    /// Send a postcard with a verification PIN to a specific mailing address. The PIN is used to complete verification with Google.
    ///
    /// "ADDRESS"
    #[serde(rename="ADDRESS")]
    ADDRESS,
    

    /// Send an email with a verification PIN to a specific email address. The PIN is used to complete verification with Google.
    ///
    /// "EMAIL"
    #[serde(rename="EMAIL")]
    EMAIL,
    

    /// Make a phone call with a verification PIN to a specific phone number. The PIN is used to complete verification with Google.
    ///
    /// "PHONE_CALL"
    #[serde(rename="PHONE_CALL")]
    PHONECALL,
    

    /// Send an SMS with a verification PIN to a specific phone number. The PIN is used to complete verification with Google.
    ///
    /// "SMS"
    #[serde(rename="SMS")]
    SMS,
    

    /// Verify the location without additional user action. This option may not be available for all locations.
    ///
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
    

    /// This option may not be available for all locations.
    ///
    /// "VETTED_PARTNER"
    #[serde(rename="VETTED_PARTNER")]
    VETTEDPARTNER,
}

impl AsRef<str> for VerificationOptionVerificationMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VerificationOptionVerificationMethodEnum::VERIFICATIONMETHODUNSPECIFIED => "VERIFICATION_METHOD_UNSPECIFIED",
            VerificationOptionVerificationMethodEnum::ADDRESS => "ADDRESS",
            VerificationOptionVerificationMethodEnum::EMAIL => "EMAIL",
            VerificationOptionVerificationMethodEnum::PHONECALL => "PHONE_CALL",
            VerificationOptionVerificationMethodEnum::SMS => "SMS",
            VerificationOptionVerificationMethodEnum::AUTO => "AUTO",
            VerificationOptionVerificationMethodEnum::VETTEDPARTNER => "VETTED_PARTNER",
        }
    }
}

impl std::convert::TryFrom< &str> for VerificationOptionVerificationMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERIFICATION_METHOD_UNSPECIFIED" => Ok(VerificationOptionVerificationMethodEnum::VERIFICATIONMETHODUNSPECIFIED),
           "ADDRESS" => Ok(VerificationOptionVerificationMethodEnum::ADDRESS),
           "EMAIL" => Ok(VerificationOptionVerificationMethodEnum::EMAIL),
           "PHONE_CALL" => Ok(VerificationOptionVerificationMethodEnum::PHONECALL),
           "SMS" => Ok(VerificationOptionVerificationMethodEnum::SMS),
           "AUTO" => Ok(VerificationOptionVerificationMethodEnum::AUTO),
           "VETTED_PARTNER" => Ok(VerificationOptionVerificationMethodEnum::VETTEDPARTNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VerificationOptionVerificationMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VerifyLocationRequestMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Verification method.
pub enum VerifyLocationRequestMethodEnum {
    

    /// Default value, will result in errors.
    ///
    /// "VERIFICATION_METHOD_UNSPECIFIED"
    #[serde(rename="VERIFICATION_METHOD_UNSPECIFIED")]
    VERIFICATIONMETHODUNSPECIFIED,
    

    /// Send a postcard with a verification PIN to a specific mailing address. The PIN is used to complete verification with Google.
    ///
    /// "ADDRESS"
    #[serde(rename="ADDRESS")]
    ADDRESS,
    

    /// Send an email with a verification PIN to a specific email address. The PIN is used to complete verification with Google.
    ///
    /// "EMAIL"
    #[serde(rename="EMAIL")]
    EMAIL,
    

    /// Make a phone call with a verification PIN to a specific phone number. The PIN is used to complete verification with Google.
    ///
    /// "PHONE_CALL"
    #[serde(rename="PHONE_CALL")]
    PHONECALL,
    

    /// Send an SMS with a verification PIN to a specific phone number. The PIN is used to complete verification with Google.
    ///
    /// "SMS"
    #[serde(rename="SMS")]
    SMS,
    

    /// Verify the location without additional user action. This option may not be available for all locations.
    ///
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
    

    /// This option may not be available for all locations.
    ///
    /// "VETTED_PARTNER"
    #[serde(rename="VETTED_PARTNER")]
    VETTEDPARTNER,
}

impl AsRef<str> for VerifyLocationRequestMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VerifyLocationRequestMethodEnum::VERIFICATIONMETHODUNSPECIFIED => "VERIFICATION_METHOD_UNSPECIFIED",
            VerifyLocationRequestMethodEnum::ADDRESS => "ADDRESS",
            VerifyLocationRequestMethodEnum::EMAIL => "EMAIL",
            VerifyLocationRequestMethodEnum::PHONECALL => "PHONE_CALL",
            VerifyLocationRequestMethodEnum::SMS => "SMS",
            VerifyLocationRequestMethodEnum::AUTO => "AUTO",
            VerifyLocationRequestMethodEnum::VETTEDPARTNER => "VETTED_PARTNER",
        }
    }
}

impl std::convert::TryFrom< &str> for VerifyLocationRequestMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERIFICATION_METHOD_UNSPECIFIED" => Ok(VerifyLocationRequestMethodEnum::VERIFICATIONMETHODUNSPECIFIED),
           "ADDRESS" => Ok(VerifyLocationRequestMethodEnum::ADDRESS),
           "EMAIL" => Ok(VerifyLocationRequestMethodEnum::EMAIL),
           "PHONE_CALL" => Ok(VerifyLocationRequestMethodEnum::PHONECALL),
           "SMS" => Ok(VerifyLocationRequestMethodEnum::SMS),
           "AUTO" => Ok(VerifyLocationRequestMethodEnum::AUTO),
           "VETTED_PARTNER" => Ok(VerifyLocationRequestMethodEnum::VETTEDPARTNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VerifyLocationRequestMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


