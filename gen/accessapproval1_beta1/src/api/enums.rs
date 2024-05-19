use super::*;



// region AccessReasonTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of access justification.
pub enum AccessReasonTypeEnum {
    

    /// Default value for proto, shouldn't be used.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Customer made a request or raised an issue that required the principal to
access customer data. `detail` is of the form ("#####" is the issue ID):
<ol>
  <li>"Feedback Report: #####"</li>
  <li>"Case Number: #####"</li>
  <li>"Case ID: #####"</li>
  <li>"E-PIN Reference: #####"</li>
  <li>"Google-#####"</li>
  <li>"T-#####"</li>
</ol>
    ///
    /// "CUSTOMER_INITIATED_SUPPORT"
    #[serde(rename="CUSTOMER_INITIATED_SUPPORT")]
    CUSTOMERINITIATEDSUPPORT,
    

    /// The principal accessed customer data in order to diagnose or resolve a
suspected issue in services or a known outage. Often this access is used
to confirm that customers are not affected by a suspected service issue
or to remediate a reversible system issue.
    ///
    /// "GOOGLE_INITIATED_SERVICE"
    #[serde(rename="GOOGLE_INITIATED_SERVICE")]
    GOOGLEINITIATEDSERVICE,
    

    /// Google initiated service for security, fraud, abuse, or compliance
purposes.
    ///
    /// "GOOGLE_INITIATED_REVIEW"
    #[serde(rename="GOOGLE_INITIATED_REVIEW")]
    GOOGLEINITIATEDREVIEW,
}

impl AsRef<str> for AccessReasonTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccessReasonTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            AccessReasonTypeEnum::CUSTOMERINITIATEDSUPPORT => "CUSTOMER_INITIATED_SUPPORT",
            AccessReasonTypeEnum::GOOGLEINITIATEDSERVICE => "GOOGLE_INITIATED_SERVICE",
            AccessReasonTypeEnum::GOOGLEINITIATEDREVIEW => "GOOGLE_INITIATED_REVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for AccessReasonTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(AccessReasonTypeEnum::TYPEUNSPECIFIED),
           "CUSTOMER_INITIATED_SUPPORT" => Ok(AccessReasonTypeEnum::CUSTOMERINITIATEDSUPPORT),
           "GOOGLE_INITIATED_SERVICE" => Ok(AccessReasonTypeEnum::GOOGLEINITIATEDSERVICE),
           "GOOGLE_INITIATED_REVIEW" => Ok(AccessReasonTypeEnum::GOOGLEINITIATEDREVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccessReasonTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnrolledServiceEnrollmentLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The enrollment level of the service.
pub enum EnrolledServiceEnrollmentLevelEnum {
    

    /// Default value for proto, shouldn't be used.
    ///
    /// "ENROLLMENT_LEVEL_UNSPECIFIED"
    #[serde(rename="ENROLLMENT_LEVEL_UNSPECIFIED")]
    ENROLLMENTLEVELUNSPECIFIED,
    

    /// Service is enrolled in Access Approval for all requests
    ///
    /// "BLOCK_ALL"
    #[serde(rename="BLOCK_ALL")]
    BLOCKALL,
}

impl AsRef<str> for EnrolledServiceEnrollmentLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnrolledServiceEnrollmentLevelEnum::ENROLLMENTLEVELUNSPECIFIED => "ENROLLMENT_LEVEL_UNSPECIFIED",
            EnrolledServiceEnrollmentLevelEnum::BLOCKALL => "BLOCK_ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for EnrolledServiceEnrollmentLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENROLLMENT_LEVEL_UNSPECIFIED" => Ok(EnrolledServiceEnrollmentLevelEnum::ENROLLMENTLEVELUNSPECIFIED),
           "BLOCK_ALL" => Ok(EnrolledServiceEnrollmentLevelEnum::BLOCKALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnrolledServiceEnrollmentLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


