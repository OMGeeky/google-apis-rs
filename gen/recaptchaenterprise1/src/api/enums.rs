use super::*;



// region GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Labels for this request.
pub enum GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum {
    

    /// Default unspecified type.
    ///
    /// "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED"
    #[serde(rename="ACCOUNT_DEFENDER_LABEL_UNSPECIFIED")]
    ACCOUNTDEFENDERLABELUNSPECIFIED,
    

    /// The request matches a known good profile for the user.
    ///
    /// "PROFILE_MATCH"
    #[serde(rename="PROFILE_MATCH")]
    PROFILEMATCH,
    

    /// The request is potentially a suspicious login event and must be further verified either through multi-factor authentication or another system.
    ///
    /// "SUSPICIOUS_LOGIN_ACTIVITY"
    #[serde(rename="SUSPICIOUS_LOGIN_ACTIVITY")]
    SUSPICIOUSLOGINACTIVITY,
    

    /// The request matched a profile that previously had suspicious account creation behavior. This can mean that this is a fake account.
    ///
    /// "SUSPICIOUS_ACCOUNT_CREATION"
    #[serde(rename="SUSPICIOUS_ACCOUNT_CREATION")]
    SUSPICIOUSACCOUNTCREATION,
    

    /// The account in the request has a high number of related accounts. It does not necessarily imply that the account is bad but can require further investigation.
    ///
    /// "RELATED_ACCOUNTS_NUMBER_HIGH"
    #[serde(rename="RELATED_ACCOUNTS_NUMBER_HIGH")]
    RELATEDACCOUNTSNUMBERHIGH,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum::ACCOUNTDEFENDERLABELUNSPECIFIED => "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum::PROFILEMATCH => "PROFILE_MATCH",
            GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum::SUSPICIOUSLOGINACTIVITY => "SUSPICIOUS_LOGIN_ACTIVITY",
            GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum::SUSPICIOUSACCOUNTCREATION => "SUSPICIOUS_ACCOUNT_CREATION",
            GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum::RELATEDACCOUNTSNUMBERHIGH => "RELATED_ACCOUNTS_NUMBER_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum::ACCOUNTDEFENDERLABELUNSPECIFIED),
           "PROFILE_MATCH" => Ok(GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum::PROFILEMATCH),
           "SUSPICIOUS_LOGIN_ACTIVITY" => Ok(GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum::SUSPICIOUSLOGINACTIVITY),
           "SUSPICIOUS_ACCOUNT_CREATION" => Ok(GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum::SUSPICIOUSACCOUNTCREATION),
           "RELATED_ACCOUNTS_NUMBER_HIGH" => Ok(GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum::RELATEDACCOUNTSNUMBERHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Result of the latest account verification challenge.
pub enum GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum {
    

    /// No information about the latest account verification.
    ///
    /// "RESULT_UNSPECIFIED"
    #[serde(rename="RESULT_UNSPECIFIED")]
    RESULTUNSPECIFIED,
    

    /// The user was successfully verified. This means the account verification challenge was successfully completed.
    ///
    /// "SUCCESS_USER_VERIFIED"
    #[serde(rename="SUCCESS_USER_VERIFIED")]
    SUCCESSUSERVERIFIED,
    

    /// The user failed the verification challenge.
    ///
    /// "ERROR_USER_NOT_VERIFIED"
    #[serde(rename="ERROR_USER_NOT_VERIFIED")]
    ERRORUSERNOTVERIFIED,
    

    /// The site is not properly onboarded to use the account verification feature.
    ///
    /// "ERROR_SITE_ONBOARDING_INCOMPLETE"
    #[serde(rename="ERROR_SITE_ONBOARDING_INCOMPLETE")]
    ERRORSITEONBOARDINGINCOMPLETE,
    

    /// The recipient is not allowed for account verification. This can occur during integration but should not occur in production.
    ///
    /// "ERROR_RECIPIENT_NOT_ALLOWED"
    #[serde(rename="ERROR_RECIPIENT_NOT_ALLOWED")]
    ERRORRECIPIENTNOTALLOWED,
    

    /// The recipient has already been sent too many verification codes in a short amount of time.
    ///
    /// "ERROR_RECIPIENT_ABUSE_LIMIT_EXHAUSTED"
    #[serde(rename="ERROR_RECIPIENT_ABUSE_LIMIT_EXHAUSTED")]
    ERRORRECIPIENTABUSELIMITEXHAUSTED,
    

    /// The verification flow could not be completed due to a critical internal error.
    ///
    /// "ERROR_CRITICAL_INTERNAL"
    #[serde(rename="ERROR_CRITICAL_INTERNAL")]
    ERRORCRITICALINTERNAL,
    

    /// The client has exceeded their two factor request quota for this period of time.
    ///
    /// "ERROR_CUSTOMER_QUOTA_EXHAUSTED"
    #[serde(rename="ERROR_CUSTOMER_QUOTA_EXHAUSTED")]
    ERRORCUSTOMERQUOTAEXHAUSTED,
    

    /// The request cannot be processed at the time because of an incident. This bypass can be restricted to a problematic destination email domain, a customer, or could affect the entire service.
    ///
    /// "ERROR_VERIFICATION_BYPASSED"
    #[serde(rename="ERROR_VERIFICATION_BYPASSED")]
    ERRORVERIFICATIONBYPASSED,
    

    /// The request parameters do not match with the token provided and cannot be processed.
    ///
    /// "ERROR_VERDICT_MISMATCH"
    #[serde(rename="ERROR_VERDICT_MISMATCH")]
    ERRORVERDICTMISMATCH,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::RESULTUNSPECIFIED => "RESULT_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::SUCCESSUSERVERIFIED => "SUCCESS_USER_VERIFIED",
            GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORUSERNOTVERIFIED => "ERROR_USER_NOT_VERIFIED",
            GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORSITEONBOARDINGINCOMPLETE => "ERROR_SITE_ONBOARDING_INCOMPLETE",
            GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORRECIPIENTNOTALLOWED => "ERROR_RECIPIENT_NOT_ALLOWED",
            GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORRECIPIENTABUSELIMITEXHAUSTED => "ERROR_RECIPIENT_ABUSE_LIMIT_EXHAUSTED",
            GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORCRITICALINTERNAL => "ERROR_CRITICAL_INTERNAL",
            GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORCUSTOMERQUOTAEXHAUSTED => "ERROR_CUSTOMER_QUOTA_EXHAUSTED",
            GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORVERIFICATIONBYPASSED => "ERROR_VERIFICATION_BYPASSED",
            GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORVERDICTMISMATCH => "ERROR_VERDICT_MISMATCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESULT_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::RESULTUNSPECIFIED),
           "SUCCESS_USER_VERIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::SUCCESSUSERVERIFIED),
           "ERROR_USER_NOT_VERIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORUSERNOTVERIFIED),
           "ERROR_SITE_ONBOARDING_INCOMPLETE" => Ok(GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORSITEONBOARDINGINCOMPLETE),
           "ERROR_RECIPIENT_NOT_ALLOWED" => Ok(GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORRECIPIENTNOTALLOWED),
           "ERROR_RECIPIENT_ABUSE_LIMIT_EXHAUSTED" => Ok(GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORRECIPIENTABUSELIMITEXHAUSTED),
           "ERROR_CRITICAL_INTERNAL" => Ok(GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORCRITICALINTERNAL),
           "ERROR_CUSTOMER_QUOTA_EXHAUSTED" => Ok(GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORCUSTOMERQUOTAEXHAUSTED),
           "ERROR_VERIFICATION_BYPASSED" => Ok(GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORVERIFICATIONBYPASSED),
           "ERROR_VERDICT_MISMATCH" => Ok(GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum::ERRORVERDICTMISMATCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The annotation that will be assigned to the Event. This field can be left empty to provide reasons that apply to an event without concluding whether the event is legitimate or fraudulent.
pub enum GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum {
    

    /// Default unspecified type.
    ///
    /// "ANNOTATION_UNSPECIFIED"
    #[serde(rename="ANNOTATION_UNSPECIFIED")]
    ANNOTATIONUNSPECIFIED,
    

    /// Provides information that the event turned out to be legitimate.
    ///
    /// "LEGITIMATE"
    #[serde(rename="LEGITIMATE")]
    LEGITIMATE,
    

    /// Provides information that the event turned out to be fraudulent.
    ///
    /// "FRAUDULENT"
    #[serde(rename="FRAUDULENT")]
    FRAUDULENT,
    

    /// Provides information that the event was related to a login event in which the user typed the correct password. Deprecated, prefer indicating CORRECT_PASSWORD through the reasons field instead.
    ///
    /// "PASSWORD_CORRECT"
    #[serde(rename="PASSWORD_CORRECT")]
    PASSWORDCORRECT,
    

    /// Provides information that the event was related to a login event in which the user typed the incorrect password. Deprecated, prefer indicating INCORRECT_PASSWORD through the reasons field instead.
    ///
    /// "PASSWORD_INCORRECT"
    #[serde(rename="PASSWORD_INCORRECT")]
    PASSWORDINCORRECT,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum::ANNOTATIONUNSPECIFIED => "ANNOTATION_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum::LEGITIMATE => "LEGITIMATE",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum::FRAUDULENT => "FRAUDULENT",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum::PASSWORDCORRECT => "PASSWORD_CORRECT",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum::PASSWORDINCORRECT => "PASSWORD_INCORRECT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANNOTATION_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum::ANNOTATIONUNSPECIFIED),
           "LEGITIMATE" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum::LEGITIMATE),
           "FRAUDULENT" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum::FRAUDULENT),
           "PASSWORD_CORRECT" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum::PASSWORDCORRECT),
           "PASSWORD_INCORRECT" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum::PASSWORDINCORRECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Reasons for the annotation that are assigned to the event.
pub enum GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum {
    

    /// Default unspecified reason.
    ///
    /// "REASON_UNSPECIFIED"
    #[serde(rename="REASON_UNSPECIFIED")]
    REASONUNSPECIFIED,
    

    /// Indicates that the transaction had a chargeback issued with no other details. When possible, specify the type by using CHARGEBACK_FRAUD or CHARGEBACK_DISPUTE instead.
    ///
    /// "CHARGEBACK"
    #[serde(rename="CHARGEBACK")]
    CHARGEBACK,
    

    /// Indicates that the transaction had a chargeback issued related to an alleged unauthorized transaction from the cardholder's perspective (for example, the card number was stolen).
    ///
    /// "CHARGEBACK_FRAUD"
    #[serde(rename="CHARGEBACK_FRAUD")]
    CHARGEBACKFRAUD,
    

    /// Indicates that the transaction had a chargeback issued related to the cardholder having provided their card details but allegedly not being satisfied with the purchase (for example, misrepresentation, attempted cancellation).
    ///
    /// "CHARGEBACK_DISPUTE"
    #[serde(rename="CHARGEBACK_DISPUTE")]
    CHARGEBACKDISPUTE,
    

    /// Indicates that the completed payment transaction was refunded by the seller.
    ///
    /// "REFUND"
    #[serde(rename="REFUND")]
    REFUND,
    

    /// Indicates that the completed payment transaction was determined to be fraudulent by the seller, and was cancelled and refunded as a result.
    ///
    /// "REFUND_FRAUD"
    #[serde(rename="REFUND_FRAUD")]
    REFUNDFRAUD,
    

    /// Indicates that the payment transaction was accepted, and the user was charged.
    ///
    /// "TRANSACTION_ACCEPTED"
    #[serde(rename="TRANSACTION_ACCEPTED")]
    TRANSACTIONACCEPTED,
    

    /// Indicates that the payment transaction was declined, for example due to invalid card details.
    ///
    /// "TRANSACTION_DECLINED"
    #[serde(rename="TRANSACTION_DECLINED")]
    TRANSACTIONDECLINED,
    

    /// Indicates the transaction associated with the assessment is suspected of being fraudulent based on the payment method, billing details, shipping address or other transaction information.
    ///
    /// "PAYMENT_HEURISTICS"
    #[serde(rename="PAYMENT_HEURISTICS")]
    PAYMENTHEURISTICS,
    

    /// Indicates that the user was served a 2FA challenge. An old assessment with `ENUM_VALUES.INITIATED_TWO_FACTOR` reason that has not been overwritten with `PASSED_TWO_FACTOR` is treated as an abandoned 2FA flow. This is equivalent to `FAILED_TWO_FACTOR`.
    ///
    /// "INITIATED_TWO_FACTOR"
    #[serde(rename="INITIATED_TWO_FACTOR")]
    INITIATEDTWOFACTOR,
    

    /// Indicates that the user passed a 2FA challenge.
    ///
    /// "PASSED_TWO_FACTOR"
    #[serde(rename="PASSED_TWO_FACTOR")]
    PASSEDTWOFACTOR,
    

    /// Indicates that the user failed a 2FA challenge.
    ///
    /// "FAILED_TWO_FACTOR"
    #[serde(rename="FAILED_TWO_FACTOR")]
    FAILEDTWOFACTOR,
    

    /// Indicates the user provided the correct password.
    ///
    /// "CORRECT_PASSWORD"
    #[serde(rename="CORRECT_PASSWORD")]
    CORRECTPASSWORD,
    

    /// Indicates the user provided an incorrect password.
    ///
    /// "INCORRECT_PASSWORD"
    #[serde(rename="INCORRECT_PASSWORD")]
    INCORRECTPASSWORD,
    

    /// Indicates that the user sent unwanted and abusive messages to other users of the platform, such as spam, scams, phishing, or social engineering.
    ///
    /// "SOCIAL_SPAM"
    #[serde(rename="SOCIAL_SPAM")]
    SOCIALSPAM,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::REASONUNSPECIFIED => "REASON_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::CHARGEBACK => "CHARGEBACK",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::CHARGEBACKFRAUD => "CHARGEBACK_FRAUD",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::CHARGEBACKDISPUTE => "CHARGEBACK_DISPUTE",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::REFUND => "REFUND",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::REFUNDFRAUD => "REFUND_FRAUD",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::TRANSACTIONACCEPTED => "TRANSACTION_ACCEPTED",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::TRANSACTIONDECLINED => "TRANSACTION_DECLINED",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::PAYMENTHEURISTICS => "PAYMENT_HEURISTICS",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::INITIATEDTWOFACTOR => "INITIATED_TWO_FACTOR",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::PASSEDTWOFACTOR => "PASSED_TWO_FACTOR",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::FAILEDTWOFACTOR => "FAILED_TWO_FACTOR",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::CORRECTPASSWORD => "CORRECT_PASSWORD",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::INCORRECTPASSWORD => "INCORRECT_PASSWORD",
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::SOCIALSPAM => "SOCIAL_SPAM",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REASON_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::REASONUNSPECIFIED),
           "CHARGEBACK" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::CHARGEBACK),
           "CHARGEBACK_FRAUD" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::CHARGEBACKFRAUD),
           "CHARGEBACK_DISPUTE" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::CHARGEBACKDISPUTE),
           "REFUND" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::REFUND),
           "REFUND_FRAUD" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::REFUNDFRAUD),
           "TRANSACTION_ACCEPTED" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::TRANSACTIONACCEPTED),
           "TRANSACTION_DECLINED" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::TRANSACTIONDECLINED),
           "PAYMENT_HEURISTICS" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::PAYMENTHEURISTICS),
           "INITIATED_TWO_FACTOR" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::INITIATEDTWOFACTOR),
           "PASSED_TWO_FACTOR" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::PASSEDTWOFACTOR),
           "FAILED_TWO_FACTOR" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::FAILEDTWOFACTOR),
           "CORRECT_PASSWORD" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::CORRECTPASSWORD),
           "INCORRECT_PASSWORD" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::INCORRECTPASSWORD),
           "SOCIAL_SPAM" => Ok(GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum::SOCIALSPAM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The Fraud Prevention setting for this assessment.
pub enum GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum {
    

    /// Default, unspecified setting. If opted in for automatic detection, `fraud_prevention_assessment` is returned based on the request. Otherwise, `fraud_prevention_assessment` is returned if `transaction_data` is present in the `Event` and Fraud Prevention is enabled in the Google Cloud console.
    ///
    /// "FRAUD_PREVENTION_UNSPECIFIED"
    #[serde(rename="FRAUD_PREVENTION_UNSPECIFIED")]
    FRAUDPREVENTIONUNSPECIFIED,
    

    /// Enable Fraud Prevention for this assessment, if Fraud Prevention is enabled in the Google Cloud console.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// Disable Fraud Prevention for this assessment, regardless of opt-in status or Google Cloud console settings.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum::FRAUDPREVENTIONUNSPECIFIED => "FRAUD_PREVENTION_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum::ENABLED => "ENABLED",
            GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FRAUD_PREVENTION_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum::FRAUDPREVENTIONUNSPECIFIED),
           "ENABLED" => Ok(GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum::ENABLED),
           "DISABLED" => Ok(GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The labels for the payment card in this transaction.
pub enum GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum {
    

    /// No label specified.
    ///
    /// "CARD_LABEL_UNSPECIFIED"
    #[serde(rename="CARD_LABEL_UNSPECIFIED")]
    CARDLABELUNSPECIFIED,
    

    /// This card has been detected as prepaid.
    ///
    /// "PREPAID"
    #[serde(rename="PREPAID")]
    PREPAID,
    

    /// This card has been detected as virtual, such as a card number generated for a single transaction or merchant.
    ///
    /// "VIRTUAL"
    #[serde(rename="VIRTUAL")]
    VIRTUAL,
    

    /// This card has been detected as being used in an unexpected geographic location.
    ///
    /// "UNEXPECTED_LOCATION"
    #[serde(rename="UNEXPECTED_LOCATION")]
    UNEXPECTEDLOCATION,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum::CARDLABELUNSPECIFIED => "CARD_LABEL_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum::PREPAID => "PREPAID",
            GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum::VIRTUAL => "VIRTUAL",
            GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum::UNEXPECTEDLOCATION => "UNEXPECTED_LOCATION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CARD_LABEL_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum::CARDLABELUNSPECIFIED),
           "PREPAID" => Ok(GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum::PREPAID),
           "VIRTUAL" => Ok(GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum::VIRTUAL),
           "UNEXPECTED_LOCATION" => Ok(GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum::UNEXPECTEDLOCATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Reasons contributing to the risk analysis verdict.
pub enum GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum {
    

    /// Default unspecified type.
    ///
    /// "CLASSIFICATION_REASON_UNSPECIFIED"
    #[serde(rename="CLASSIFICATION_REASON_UNSPECIFIED")]
    CLASSIFICATIONREASONUNSPECIFIED,
    

    /// Interactions matched the behavior of an automated agent.
    ///
    /// "AUTOMATION"
    #[serde(rename="AUTOMATION")]
    AUTOMATION,
    

    /// The event originated from an illegitimate environment.
    ///
    /// "UNEXPECTED_ENVIRONMENT"
    #[serde(rename="UNEXPECTED_ENVIRONMENT")]
    UNEXPECTEDENVIRONMENT,
    

    /// Traffic volume from the event source is higher than normal.
    ///
    /// "TOO_MUCH_TRAFFIC"
    #[serde(rename="TOO_MUCH_TRAFFIC")]
    TOOMUCHTRAFFIC,
    

    /// Interactions with the site were significantly different than expected patterns.
    ///
    /// "UNEXPECTED_USAGE_PATTERNS"
    #[serde(rename="UNEXPECTED_USAGE_PATTERNS")]
    UNEXPECTEDUSAGEPATTERNS,
    

    /// Too little traffic has been received from this site thus far to generate quality risk analysis.
    ///
    /// "LOW_CONFIDENCE_SCORE"
    #[serde(rename="LOW_CONFIDENCE_SCORE")]
    LOWCONFIDENCESCORE,
    

    /// The request matches behavioral characteristics of a carding attack.
    ///
    /// "SUSPECTED_CARDING"
    #[serde(rename="SUSPECTED_CARDING")]
    SUSPECTEDCARDING,
    

    /// The request matches behavioral characteristics of chargebacks for fraud.
    ///
    /// "SUSPECTED_CHARGEBACK"
    #[serde(rename="SUSPECTED_CHARGEBACK")]
    SUSPECTEDCHARGEBACK,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::CLASSIFICATIONREASONUNSPECIFIED => "CLASSIFICATION_REASON_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::AUTOMATION => "AUTOMATION",
            GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::UNEXPECTEDENVIRONMENT => "UNEXPECTED_ENVIRONMENT",
            GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::TOOMUCHTRAFFIC => "TOO_MUCH_TRAFFIC",
            GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::UNEXPECTEDUSAGEPATTERNS => "UNEXPECTED_USAGE_PATTERNS",
            GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::LOWCONFIDENCESCORE => "LOW_CONFIDENCE_SCORE",
            GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::SUSPECTEDCARDING => "SUSPECTED_CARDING",
            GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::SUSPECTEDCHARGEBACK => "SUSPECTED_CHARGEBACK",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLASSIFICATION_REASON_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::CLASSIFICATIONREASONUNSPECIFIED),
           "AUTOMATION" => Ok(GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::AUTOMATION),
           "UNEXPECTED_ENVIRONMENT" => Ok(GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::UNEXPECTEDENVIRONMENT),
           "TOO_MUCH_TRAFFIC" => Ok(GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::TOOMUCHTRAFFIC),
           "UNEXPECTED_USAGE_PATTERNS" => Ok(GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::UNEXPECTEDUSAGEPATTERNS),
           "LOW_CONFIDENCE_SCORE" => Ok(GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::LOWCONFIDENCESCORE),
           "SUSPECTED_CARDING" => Ok(GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::SUSPECTEDCARDING),
           "SUSPECTED_CHARGEBACK" => Ok(GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum::SUSPECTEDCHARGEBACK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. For challenge-based keys only (CHECKBOX, INVISIBLE), all challenge requests for this site will return nocaptcha if NOCAPTCHA, or an unsolvable challenge if CHALLENGE.
pub enum GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum {
    

    /// Perform the normal risk analysis and return either nocaptcha or a challenge depending on risk and trust factors.
    ///
    /// "TESTING_CHALLENGE_UNSPECIFIED"
    #[serde(rename="TESTING_CHALLENGE_UNSPECIFIED")]
    TESTINGCHALLENGEUNSPECIFIED,
    

    /// Challenge requests for this key always return a nocaptcha, which does not require a solution.
    ///
    /// "NOCAPTCHA"
    #[serde(rename="NOCAPTCHA")]
    NOCAPTCHA,
    

    /// Challenge requests for this key always return an unsolvable challenge.
    ///
    /// "UNSOLVABLE_CHALLENGE"
    #[serde(rename="UNSOLVABLE_CHALLENGE")]
    UNSOLVABLECHALLENGE,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum::TESTINGCHALLENGEUNSPECIFIED => "TESTING_CHALLENGE_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum::NOCAPTCHA => "NOCAPTCHA",
            GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum::UNSOLVABLECHALLENGE => "UNSOLVABLE_CHALLENGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TESTING_CHALLENGE_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum::TESTINGCHALLENGEUNSPECIFIED),
           "NOCAPTCHA" => Ok(GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum::NOCAPTCHA),
           "UNSOLVABLE_CHALLENGE" => Ok(GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum::UNSOLVABLECHALLENGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Reason associated with the response when valid = false.
pub enum GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum {
    

    /// Default unspecified type.
    ///
    /// "INVALID_REASON_UNSPECIFIED"
    #[serde(rename="INVALID_REASON_UNSPECIFIED")]
    INVALIDREASONUNSPECIFIED,
    

    /// If the failure reason was not accounted for.
    ///
    /// "UNKNOWN_INVALID_REASON"
    #[serde(rename="UNKNOWN_INVALID_REASON")]
    UNKNOWNINVALIDREASON,
    

    /// The provided user verification token was malformed.
    ///
    /// "MALFORMED"
    #[serde(rename="MALFORMED")]
    MALFORMED,
    

    /// The user verification token had expired.
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// The user verification had already been seen.
    ///
    /// "DUPE"
    #[serde(rename="DUPE")]
    DUPE,
    

    /// The user verification token was not present.
    ///
    /// "MISSING"
    #[serde(rename="MISSING")]
    MISSING,
    

    /// A retriable error (such as network failure) occurred on the browser. Could easily be simulated by an attacker.
    ///
    /// "BROWSER_ERROR"
    #[serde(rename="BROWSER_ERROR")]
    BROWSERERROR,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::INVALIDREASONUNSPECIFIED => "INVALID_REASON_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::UNKNOWNINVALIDREASON => "UNKNOWN_INVALID_REASON",
            GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::MALFORMED => "MALFORMED",
            GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::EXPIRED => "EXPIRED",
            GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::DUPE => "DUPE",
            GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::MISSING => "MISSING",
            GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::BROWSERERROR => "BROWSER_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVALID_REASON_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::INVALIDREASONUNSPECIFIED),
           "UNKNOWN_INVALID_REASON" => Ok(GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::UNKNOWNINVALIDREASON),
           "MALFORMED" => Ok(GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::MALFORMED),
           "EXPIRED" => Ok(GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::EXPIRED),
           "DUPE" => Ok(GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::DUPE),
           "MISSING" => Ok(GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::MISSING),
           "BROWSER_ERROR" => Ok(GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum::BROWSERERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of this transaction event.
pub enum GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum {
    

    /// Default, unspecified event type.
    ///
    /// "TRANSACTION_EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="TRANSACTION_EVENT_TYPE_UNSPECIFIED")]
    TRANSACTIONEVENTTYPEUNSPECIFIED,
    

    /// Indicates that the transaction is approved by the merchant. The accompanying reasons can include terms such as 'INHOUSE', 'ACCERTIFY', 'CYBERSOURCE', or 'MANUAL_REVIEW'.
    ///
    /// "MERCHANT_APPROVE"
    #[serde(rename="MERCHANT_APPROVE")]
    MERCHANTAPPROVE,
    

    /// Indicates that the transaction is denied and concluded due to risks detected by the merchant. The accompanying reasons can include terms such as 'INHOUSE', 'ACCERTIFY', 'CYBERSOURCE', or 'MANUAL_REVIEW'.
    ///
    /// "MERCHANT_DENY"
    #[serde(rename="MERCHANT_DENY")]
    MERCHANTDENY,
    

    /// Indicates that the transaction is being evaluated by a human, due to suspicion or risk.
    ///
    /// "MANUAL_REVIEW"
    #[serde(rename="MANUAL_REVIEW")]
    MANUALREVIEW,
    

    /// Indicates that the authorization attempt with the card issuer succeeded.
    ///
    /// "AUTHORIZATION"
    #[serde(rename="AUTHORIZATION")]
    AUTHORIZATION,
    

    /// Indicates that the authorization attempt with the card issuer failed. The accompanying reasons can include Visa's '54' indicating that the card is expired, or '82' indicating that the CVV is incorrect.
    ///
    /// "AUTHORIZATION_DECLINE"
    #[serde(rename="AUTHORIZATION_DECLINE")]
    AUTHORIZATIONDECLINE,
    

    /// Indicates that the transaction is completed because the funds were settled.
    ///
    /// "PAYMENT_CAPTURE"
    #[serde(rename="PAYMENT_CAPTURE")]
    PAYMENTCAPTURE,
    

    /// Indicates that the transaction could not be completed because the funds were not settled.
    ///
    /// "PAYMENT_CAPTURE_DECLINE"
    #[serde(rename="PAYMENT_CAPTURE_DECLINE")]
    PAYMENTCAPTUREDECLINE,
    

    /// Indicates that the transaction has been canceled. Specify the reason for the cancellation. For example, 'INSUFFICIENT_INVENTORY'.
    ///
    /// "CANCEL"
    #[serde(rename="CANCEL")]
    CANCEL,
    

    /// Indicates that the merchant has received a chargeback inquiry due to fraud for the transaction, requesting additional information before a fraud chargeback is officially issued and a formal chargeback notification is sent.
    ///
    /// "CHARGEBACK_INQUIRY"
    #[serde(rename="CHARGEBACK_INQUIRY")]
    CHARGEBACKINQUIRY,
    

    /// Indicates that the merchant has received a chargeback alert due to fraud for the transaction. The process of resolving the dispute without involving the payment network is started.
    ///
    /// "CHARGEBACK_ALERT"
    #[serde(rename="CHARGEBACK_ALERT")]
    CHARGEBACKALERT,
    

    /// Indicates that a fraud notification is issued for the transaction, sent by the payment instrument's issuing bank because the transaction appears to be fraudulent. We recommend including TC40 or SAFE data in the `reason` field for this event type. For partial chargebacks, we recommend that you include an amount in the `value` field.
    ///
    /// "FRAUD_NOTIFICATION"
    #[serde(rename="FRAUD_NOTIFICATION")]
    FRAUDNOTIFICATION,
    

    /// Indicates that the merchant is informed by the payment network that the transaction has entered the chargeback process due to fraud. Reason code examples include Discover's '6005' and '6041'. For partial chargebacks, we recommend that you include an amount in the `value` field.
    ///
    /// "CHARGEBACK"
    #[serde(rename="CHARGEBACK")]
    CHARGEBACK,
    

    /// Indicates that the transaction has entered the chargeback process due to fraud, and that the merchant has chosen to enter representment. Reason examples include Discover's '6005' and '6041'. For partial chargebacks, we recommend that you include an amount in the `value` field.
    ///
    /// "CHARGEBACK_REPRESENTMENT"
    #[serde(rename="CHARGEBACK_REPRESENTMENT")]
    CHARGEBACKREPRESENTMENT,
    

    /// Indicates that the transaction has had a fraud chargeback which was illegitimate and was reversed as a result. For partial chargebacks, we recommend that you include an amount in the `value` field.
    ///
    /// "CHARGEBACK_REVERSE"
    #[serde(rename="CHARGEBACK_REVERSE")]
    CHARGEBACKREVERSE,
    

    /// Indicates that the merchant has received a refund for a completed transaction. For partial refunds, we recommend that you include an amount in the `value` field. Reason example: 'TAX_EXEMPT' (partial refund of exempt tax)
    ///
    /// "REFUND_REQUEST"
    #[serde(rename="REFUND_REQUEST")]
    REFUNDREQUEST,
    

    /// Indicates that the merchant has received a refund request for this transaction, but that they have declined it. For partial refunds, we recommend that you include an amount in the `value` field. Reason example: 'TAX_EXEMPT' (partial refund of exempt tax)
    ///
    /// "REFUND_DECLINE"
    #[serde(rename="REFUND_DECLINE")]
    REFUNDDECLINE,
    

    /// Indicates that the completed transaction was refunded by the merchant. For partial refunds, we recommend that you include an amount in the `value` field. Reason example: 'TAX_EXEMPT' (partial refund of exempt tax)
    ///
    /// "REFUND"
    #[serde(rename="REFUND")]
    REFUND,
    

    /// Indicates that the completed transaction was refunded by the merchant, and that this refund was reversed. For partial refunds, we recommend that you include an amount in the `value` field.
    ///
    /// "REFUND_REVERSE"
    #[serde(rename="REFUND_REVERSE")]
    REFUNDREVERSE,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::TRANSACTIONEVENTTYPEUNSPECIFIED => "TRANSACTION_EVENT_TYPE_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::MERCHANTAPPROVE => "MERCHANT_APPROVE",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::MERCHANTDENY => "MERCHANT_DENY",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::MANUALREVIEW => "MANUAL_REVIEW",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::AUTHORIZATION => "AUTHORIZATION",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::AUTHORIZATIONDECLINE => "AUTHORIZATION_DECLINE",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::PAYMENTCAPTURE => "PAYMENT_CAPTURE",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::PAYMENTCAPTUREDECLINE => "PAYMENT_CAPTURE_DECLINE",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CANCEL => "CANCEL",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CHARGEBACKINQUIRY => "CHARGEBACK_INQUIRY",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CHARGEBACKALERT => "CHARGEBACK_ALERT",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::FRAUDNOTIFICATION => "FRAUD_NOTIFICATION",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CHARGEBACK => "CHARGEBACK",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CHARGEBACKREPRESENTMENT => "CHARGEBACK_REPRESENTMENT",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CHARGEBACKREVERSE => "CHARGEBACK_REVERSE",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::REFUNDREQUEST => "REFUND_REQUEST",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::REFUNDDECLINE => "REFUND_DECLINE",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::REFUND => "REFUND",
            GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::REFUNDREVERSE => "REFUND_REVERSE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSACTION_EVENT_TYPE_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::TRANSACTIONEVENTTYPEUNSPECIFIED),
           "MERCHANT_APPROVE" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::MERCHANTAPPROVE),
           "MERCHANT_DENY" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::MERCHANTDENY),
           "MANUAL_REVIEW" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::MANUALREVIEW),
           "AUTHORIZATION" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::AUTHORIZATION),
           "AUTHORIZATION_DECLINE" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::AUTHORIZATIONDECLINE),
           "PAYMENT_CAPTURE" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::PAYMENTCAPTURE),
           "PAYMENT_CAPTURE_DECLINE" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::PAYMENTCAPTUREDECLINE),
           "CANCEL" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CANCEL),
           "CHARGEBACK_INQUIRY" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CHARGEBACKINQUIRY),
           "CHARGEBACK_ALERT" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CHARGEBACKALERT),
           "FRAUD_NOTIFICATION" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::FRAUDNOTIFICATION),
           "CHARGEBACK" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CHARGEBACK),
           "CHARGEBACK_REPRESENTMENT" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CHARGEBACKREPRESENTMENT),
           "CHARGEBACK_REVERSE" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::CHARGEBACKREVERSE),
           "REFUND_REQUEST" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::REFUNDREQUEST),
           "REFUND_DECLINE" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::REFUNDDECLINE),
           "REFUND" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::REFUND),
           "REFUND_REVERSE" => Ok(GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum::REFUNDREVERSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The WAF feature for which this key is enabled.
pub enum GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum {
    

    /// Undefined feature.
    ///
    /// "WAF_FEATURE_UNSPECIFIED"
    #[serde(rename="WAF_FEATURE_UNSPECIFIED")]
    WAFFEATUREUNSPECIFIED,
    

    /// Redirects suspicious traffic to reCAPTCHA.
    ///
    /// "CHALLENGE_PAGE"
    #[serde(rename="CHALLENGE_PAGE")]
    CHALLENGEPAGE,
    

    /// Use reCAPTCHA session-tokens to protect the whole user session on the site's domain.
    ///
    /// "SESSION_TOKEN"
    #[serde(rename="SESSION_TOKEN")]
    SESSIONTOKEN,
    

    /// Use reCAPTCHA action-tokens to protect user actions.
    ///
    /// "ACTION_TOKEN"
    #[serde(rename="ACTION_TOKEN")]
    ACTIONTOKEN,
    

    /// Use reCAPTCHA WAF express protection to protect any content other than web pages, like APIs and IoT devices.
    ///
    /// "EXPRESS"
    #[serde(rename="EXPRESS")]
    EXPRESS,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum::WAFFEATUREUNSPECIFIED => "WAF_FEATURE_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum::CHALLENGEPAGE => "CHALLENGE_PAGE",
            GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum::SESSIONTOKEN => "SESSION_TOKEN",
            GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum::ACTIONTOKEN => "ACTION_TOKEN",
            GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum::EXPRESS => "EXPRESS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WAF_FEATURE_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum::WAFFEATUREUNSPECIFIED),
           "CHALLENGE_PAGE" => Ok(GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum::CHALLENGEPAGE),
           "SESSION_TOKEN" => Ok(GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum::SESSIONTOKEN),
           "ACTION_TOKEN" => Ok(GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum::ACTIONTOKEN),
           "EXPRESS" => Ok(GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum::EXPRESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The WAF service that uses this key.
pub enum GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum {
    

    /// Undefined WAF
    ///
    /// "WAF_SERVICE_UNSPECIFIED"
    #[serde(rename="WAF_SERVICE_UNSPECIFIED")]
    WAFSERVICEUNSPECIFIED,
    

    /// Cloud Armor
    ///
    /// "CA"
    #[serde(rename="CA")]
    CA,
    

    /// Fastly
    ///
    /// "FASTLY"
    #[serde(rename="FASTLY")]
    FASTLY,
    

    /// Cloudflare
    ///
    /// "CLOUDFLARE"
    #[serde(rename="CLOUDFLARE")]
    CLOUDFLARE,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum::WAFSERVICEUNSPECIFIED => "WAF_SERVICE_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum::CA => "CA",
            GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum::FASTLY => "FASTLY",
            GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum::CLOUDFLARE => "CLOUDFLARE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WAF_SERVICE_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum::WAFSERVICEUNSPECIFIED),
           "CA" => Ok(GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum::CA),
           "FASTLY" => Ok(GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum::FASTLY),
           "CLOUDFLARE" => Ok(GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum::CLOUDFLARE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Settings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE.
pub enum GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum {
    

    /// Default type that indicates this enum hasn't been specified.
    ///
    /// "CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED"
    #[serde(rename="CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED")]
    CHALLENGESECURITYPREFERENCEUNSPECIFIED,
    

    /// Key tends to show fewer and easier challenges.
    ///
    /// "USABILITY"
    #[serde(rename="USABILITY")]
    USABILITY,
    

    /// Key tends to show balanced (in amount and difficulty) challenges.
    ///
    /// "BALANCE"
    #[serde(rename="BALANCE")]
    BALANCE,
    

    /// Key tends to show more and harder challenges.
    ///
    /// "SECURITY"
    #[serde(rename="SECURITY")]
    SECURITY,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum::CHALLENGESECURITYPREFERENCEUNSPECIFIED => "CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum::USABILITY => "USABILITY",
            GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum::BALANCE => "BALANCE",
            GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum::SECURITY => "SECURITY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum::CHALLENGESECURITYPREFERENCEUNSPECIFIED),
           "USABILITY" => Ok(GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum::USABILITY),
           "BALANCE" => Ok(GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum::BALANCE),
           "SECURITY" => Ok(GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum::SECURITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Describes how this key is integrated with the website.
pub enum GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum {
    

    /// Default type that indicates this enum hasn't been specified. This is not a valid IntegrationType, one of the other types must be specified instead.
    ///
    /// "INTEGRATION_TYPE_UNSPECIFIED"
    #[serde(rename="INTEGRATION_TYPE_UNSPECIFIED")]
    INTEGRATIONTYPEUNSPECIFIED,
    

    /// Only used to produce scores. It doesn't display the "I'm not a robot" checkbox and never shows captcha challenges.
    ///
    /// "SCORE"
    #[serde(rename="SCORE")]
    SCORE,
    

    /// Displays the "I'm not a robot" checkbox and may show captcha challenges after it is checked.
    ///
    /// "CHECKBOX"
    #[serde(rename="CHECKBOX")]
    CHECKBOX,
    

    /// Doesn't display the "I'm not a robot" checkbox, but may show captcha challenges after risk analysis.
    ///
    /// "INVISIBLE"
    #[serde(rename="INVISIBLE")]
    INVISIBLE,
}

impl AsRef<str> for GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum::INTEGRATIONTYPEUNSPECIFIED => "INTEGRATION_TYPE_UNSPECIFIED",
            GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum::SCORE => "SCORE",
            GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum::CHECKBOX => "CHECKBOX",
            GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum::INVISIBLE => "INVISIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTEGRATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum::INTEGRATIONTYPEUNSPECIFIED),
           "SCORE" => Ok(GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum::SCORE),
           "CHECKBOX" => Ok(GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum::CHECKBOX),
           "INVISIBLE" => Ok(GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum::INVISIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


