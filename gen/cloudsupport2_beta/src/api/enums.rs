use super::*;



// region CasePriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The priority of this case.
pub enum CasePriorityEnum {
    

    /// Priority is undefined or has not been set yet.
    ///
    /// "PRIORITY_UNSPECIFIED"
    #[serde(rename="PRIORITY_UNSPECIFIED")]
    PRIORITYUNSPECIFIED,
    

    /// Extreme impact on a production service. Service is hard down.
    ///
    /// "P0"
    #[serde(rename="P0")]
    P0,
    

    /// Critical impact on a production service. Service is currently unusable.
    ///
    /// "P1"
    #[serde(rename="P1")]
    P1,
    

    /// Severe impact on a production service. Service is usable but greatly impaired.
    ///
    /// "P2"
    #[serde(rename="P2")]
    P2,
    

    /// Medium impact on a production service. Service is available, but moderately impaired.
    ///
    /// "P3"
    #[serde(rename="P3")]
    P3,
    

    /// General questions or minor issues. Production service is fully available.
    ///
    /// "P4"
    #[serde(rename="P4")]
    P4,
}

impl AsRef<str> for CasePriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CasePriorityEnum::PRIORITYUNSPECIFIED => "PRIORITY_UNSPECIFIED",
            CasePriorityEnum::P0 => "P0",
            CasePriorityEnum::P1 => "P1",
            CasePriorityEnum::P2 => "P2",
            CasePriorityEnum::P3 => "P3",
            CasePriorityEnum::P4 => "P4",
        }
    }
}

impl std::convert::TryFrom< &str> for CasePriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIORITY_UNSPECIFIED" => Ok(CasePriorityEnum::PRIORITYUNSPECIFIED),
           "P0" => Ok(CasePriorityEnum::P0),
           "P1" => Ok(CasePriorityEnum::P1),
           "P2" => Ok(CasePriorityEnum::P2),
           "P3" => Ok(CasePriorityEnum::P3),
           "P4" => Ok(CasePriorityEnum::P4),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CasePriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CaseSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// REMOVED. The severity of this case. Use priority instead.
pub enum CaseSeverityEnum {
    

    /// Severity is undefined or has not been set yet.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Extreme impact on a production service. Service is hard down.
    ///
    /// "S0"
    #[serde(rename="S0")]
    S0,
    

    /// Critical impact on a production service. Service is currently unusable.
    ///
    /// "S1"
    #[serde(rename="S1")]
    S1,
    

    /// Severe impact on a production service. Service is usable but greatly impaired.
    ///
    /// "S2"
    #[serde(rename="S2")]
    S2,
    

    /// Medium impact on a production service. Service is available, but moderately impaired.
    ///
    /// "S3"
    #[serde(rename="S3")]
    S3,
    

    /// General questions or minor issues. Production service is fully available.
    ///
    /// "S4"
    #[serde(rename="S4")]
    S4,
}

impl AsRef<str> for CaseSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CaseSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            CaseSeverityEnum::S0 => "S0",
            CaseSeverityEnum::S1 => "S1",
            CaseSeverityEnum::S2 => "S2",
            CaseSeverityEnum::S3 => "S3",
            CaseSeverityEnum::S4 => "S4",
        }
    }
}

impl std::convert::TryFrom< &str> for CaseSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(CaseSeverityEnum::SEVERITYUNSPECIFIED),
           "S0" => Ok(CaseSeverityEnum::S0),
           "S1" => Ok(CaseSeverityEnum::S1),
           "S2" => Ok(CaseSeverityEnum::S2),
           "S3" => Ok(CaseSeverityEnum::S3),
           "S4" => Ok(CaseSeverityEnum::S4),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CaseSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CaseStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current status of the support case.
pub enum CaseStateEnum {
    

    /// Case is in an unknown state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The case has been created but no one is assigned to work on it yet.
    ///
    /// "NEW"
    #[serde(rename="NEW")]
    NEW,
    

    /// The case is currently being handled by Google support.
    ///
    /// "IN_PROGRESS_GOOGLE_SUPPORT"
    #[serde(rename="IN_PROGRESS_GOOGLE_SUPPORT")]
    INPROGRESSGOOGLESUPPORT,
    

    /// Google is waiting for a response.
    ///
    /// "ACTION_REQUIRED"
    #[serde(rename="ACTION_REQUIRED")]
    ACTIONREQUIRED,
    

    /// A solution has been offered for the case, but it isn't yet closed.
    ///
    /// "SOLUTION_PROVIDED"
    #[serde(rename="SOLUTION_PROVIDED")]
    SOLUTIONPROVIDED,
    

    /// The case has been resolved.
    ///
    /// "CLOSED"
    #[serde(rename="CLOSED")]
    CLOSED,
}

impl AsRef<str> for CaseStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CaseStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CaseStateEnum::NEW => "NEW",
            CaseStateEnum::INPROGRESSGOOGLESUPPORT => "IN_PROGRESS_GOOGLE_SUPPORT",
            CaseStateEnum::ACTIONREQUIRED => "ACTION_REQUIRED",
            CaseStateEnum::SOLUTIONPROVIDED => "SOLUTION_PROVIDED",
            CaseStateEnum::CLOSED => "CLOSED",
        }
    }
}

impl std::convert::TryFrom< &str> for CaseStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CaseStateEnum::STATEUNSPECIFIED),
           "NEW" => Ok(CaseStateEnum::NEW),
           "IN_PROGRESS_GOOGLE_SUPPORT" => Ok(CaseStateEnum::INPROGRESSGOOGLESUPPORT),
           "ACTION_REQUIRED" => Ok(CaseStateEnum::ACTIONREQUIRED),
           "SOLUTION_PROVIDED" => Ok(CaseStateEnum::SOLUTIONPROVIDED),
           "CLOSED" => Ok(CaseStateEnum::CLOSED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CaseStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompositeMediaReferenceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// # gdata.* are outside protos with mising documentation
pub enum CompositeMediaReferenceTypeEnum {
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "PATH"
    #[serde(rename="PATH")]
    PATH,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "BLOB_REF"
    #[serde(rename="BLOB_REF")]
    BLOBREF,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "INLINE"
    #[serde(rename="INLINE")]
    INLINE,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "BIGSTORE_REF"
    #[serde(rename="BIGSTORE_REF")]
    BIGSTOREREF,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "COSMO_BINARY_REFERENCE"
    #[serde(rename="COSMO_BINARY_REFERENCE")]
    COSMOBINARYREFERENCE,
}

impl AsRef<str> for CompositeMediaReferenceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompositeMediaReferenceTypeEnum::PATH => "PATH",
            CompositeMediaReferenceTypeEnum::BLOBREF => "BLOB_REF",
            CompositeMediaReferenceTypeEnum::INLINE => "INLINE",
            CompositeMediaReferenceTypeEnum::BIGSTOREREF => "BIGSTORE_REF",
            CompositeMediaReferenceTypeEnum::COSMOBINARYREFERENCE => "COSMO_BINARY_REFERENCE",
        }
    }
}

impl std::convert::TryFrom< &str> for CompositeMediaReferenceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PATH" => Ok(CompositeMediaReferenceTypeEnum::PATH),
           "BLOB_REF" => Ok(CompositeMediaReferenceTypeEnum::BLOBREF),
           "INLINE" => Ok(CompositeMediaReferenceTypeEnum::INLINE),
           "BIGSTORE_REF" => Ok(CompositeMediaReferenceTypeEnum::BIGSTOREREF),
           "COSMO_BINARY_REFERENCE" => Ok(CompositeMediaReferenceTypeEnum::COSMOBINARYREFERENCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompositeMediaReferenceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EscalationReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The reason why the Case is being escalated.
pub enum EscalationReasonEnum {
    

    /// The escalation reason is in an unknown state or has not been specified.
    ///
    /// "REASON_UNSPECIFIED"
    #[serde(rename="REASON_UNSPECIFIED")]
    REASONUNSPECIFIED,
    

    /// The case is taking too long to resolve.
    ///
    /// "RESOLUTION_TIME"
    #[serde(rename="RESOLUTION_TIME")]
    RESOLUTIONTIME,
    

    /// The support agent does not have the expertise required to successfully resolve the issue.
    ///
    /// "TECHNICAL_EXPERTISE"
    #[serde(rename="TECHNICAL_EXPERTISE")]
    TECHNICALEXPERTISE,
    

    /// The issue is having a significant business impact.
    ///
    /// "BUSINESS_IMPACT"
    #[serde(rename="BUSINESS_IMPACT")]
    BUSINESSIMPACT,
}

impl AsRef<str> for EscalationReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EscalationReasonEnum::REASONUNSPECIFIED => "REASON_UNSPECIFIED",
            EscalationReasonEnum::RESOLUTIONTIME => "RESOLUTION_TIME",
            EscalationReasonEnum::TECHNICALEXPERTISE => "TECHNICAL_EXPERTISE",
            EscalationReasonEnum::BUSINESSIMPACT => "BUSINESS_IMPACT",
        }
    }
}

impl std::convert::TryFrom< &str> for EscalationReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REASON_UNSPECIFIED" => Ok(EscalationReasonEnum::REASONUNSPECIFIED),
           "RESOLUTION_TIME" => Ok(EscalationReasonEnum::RESOLUTIONTIME),
           "TECHNICAL_EXPERTISE" => Ok(EscalationReasonEnum::TECHNICALEXPERTISE),
           "BUSINESS_IMPACT" => Ok(EscalationReasonEnum::BUSINESSIMPACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EscalationReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediaReferenceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// # gdata.* are outside protos with mising documentation
pub enum MediaReferenceTypeEnum {
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "PATH"
    #[serde(rename="PATH")]
    PATH,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "BLOB_REF"
    #[serde(rename="BLOB_REF")]
    BLOBREF,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "INLINE"
    #[serde(rename="INLINE")]
    INLINE,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "GET_MEDIA"
    #[serde(rename="GET_MEDIA")]
    GETMEDIA,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "COMPOSITE_MEDIA"
    #[serde(rename="COMPOSITE_MEDIA")]
    COMPOSITEMEDIA,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "BIGSTORE_REF"
    #[serde(rename="BIGSTORE_REF")]
    BIGSTOREREF,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "DIFF_VERSION_RESPONSE"
    #[serde(rename="DIFF_VERSION_RESPONSE")]
    DIFFVERSIONRESPONSE,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "DIFF_CHECKSUMS_RESPONSE"
    #[serde(rename="DIFF_CHECKSUMS_RESPONSE")]
    DIFFCHECKSUMSRESPONSE,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "DIFF_DOWNLOAD_RESPONSE"
    #[serde(rename="DIFF_DOWNLOAD_RESPONSE")]
    DIFFDOWNLOADRESPONSE,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "DIFF_UPLOAD_REQUEST"
    #[serde(rename="DIFF_UPLOAD_REQUEST")]
    DIFFUPLOADREQUEST,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "DIFF_UPLOAD_RESPONSE"
    #[serde(rename="DIFF_UPLOAD_RESPONSE")]
    DIFFUPLOADRESPONSE,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "COSMO_BINARY_REFERENCE"
    #[serde(rename="COSMO_BINARY_REFERENCE")]
    COSMOBINARYREFERENCE,
    

    /// # gdata.* are outside protos with mising documentation
    ///
    /// "ARBITRARY_BYTES"
    #[serde(rename="ARBITRARY_BYTES")]
    ARBITRARYBYTES,
}

impl AsRef<str> for MediaReferenceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediaReferenceTypeEnum::PATH => "PATH",
            MediaReferenceTypeEnum::BLOBREF => "BLOB_REF",
            MediaReferenceTypeEnum::INLINE => "INLINE",
            MediaReferenceTypeEnum::GETMEDIA => "GET_MEDIA",
            MediaReferenceTypeEnum::COMPOSITEMEDIA => "COMPOSITE_MEDIA",
            MediaReferenceTypeEnum::BIGSTOREREF => "BIGSTORE_REF",
            MediaReferenceTypeEnum::DIFFVERSIONRESPONSE => "DIFF_VERSION_RESPONSE",
            MediaReferenceTypeEnum::DIFFCHECKSUMSRESPONSE => "DIFF_CHECKSUMS_RESPONSE",
            MediaReferenceTypeEnum::DIFFDOWNLOADRESPONSE => "DIFF_DOWNLOAD_RESPONSE",
            MediaReferenceTypeEnum::DIFFUPLOADREQUEST => "DIFF_UPLOAD_REQUEST",
            MediaReferenceTypeEnum::DIFFUPLOADRESPONSE => "DIFF_UPLOAD_RESPONSE",
            MediaReferenceTypeEnum::COSMOBINARYREFERENCE => "COSMO_BINARY_REFERENCE",
            MediaReferenceTypeEnum::ARBITRARYBYTES => "ARBITRARY_BYTES",
        }
    }
}

impl std::convert::TryFrom< &str> for MediaReferenceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PATH" => Ok(MediaReferenceTypeEnum::PATH),
           "BLOB_REF" => Ok(MediaReferenceTypeEnum::BLOBREF),
           "INLINE" => Ok(MediaReferenceTypeEnum::INLINE),
           "GET_MEDIA" => Ok(MediaReferenceTypeEnum::GETMEDIA),
           "COMPOSITE_MEDIA" => Ok(MediaReferenceTypeEnum::COMPOSITEMEDIA),
           "BIGSTORE_REF" => Ok(MediaReferenceTypeEnum::BIGSTOREREF),
           "DIFF_VERSION_RESPONSE" => Ok(MediaReferenceTypeEnum::DIFFVERSIONRESPONSE),
           "DIFF_CHECKSUMS_RESPONSE" => Ok(MediaReferenceTypeEnum::DIFFCHECKSUMSRESPONSE),
           "DIFF_DOWNLOAD_RESPONSE" => Ok(MediaReferenceTypeEnum::DIFFDOWNLOADRESPONSE),
           "DIFF_UPLOAD_REQUEST" => Ok(MediaReferenceTypeEnum::DIFFUPLOADREQUEST),
           "DIFF_UPLOAD_RESPONSE" => Ok(MediaReferenceTypeEnum::DIFFUPLOADRESPONSE),
           "COSMO_BINARY_REFERENCE" => Ok(MediaReferenceTypeEnum::COSMOBINARYREFERENCE),
           "ARBITRARY_BYTES" => Ok(MediaReferenceTypeEnum::ARBITRARYBYTES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediaReferenceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


