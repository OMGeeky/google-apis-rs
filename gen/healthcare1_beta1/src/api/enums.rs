use super::*;



// region AccessDeterminationLogConfigLogLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Controls the amount of detail to include as part of the audit logs.
pub enum AccessDeterminationLogConfigLogLevelEnum {
    

    /// No log level specified. This value is unused.
    ///
    /// "LOG_LEVEL_UNSPECIFIED"
    #[serde(rename="LOG_LEVEL_UNSPECIFIED")]
    LOGLEVELUNSPECIFIED,
    

    /// No additional consent-related logging is added to audit logs.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// The following information is included: * One of the following [`consentMode`](https://cloud.google.com/healthcare-api/docs/fhir-consent#audit_logs) fields: (`off`|`emptyScope`|`enforced`|`btg`|`bypass`). * The accessor's request headers * The `log_level` of the AccessDeterminationLogConfig * The final consent evaluation (`PERMIT`, `DENY`, or `NO_CONSENT`) * A human-readable summary of the evaluation
    ///
    /// "MINIMUM"
    #[serde(rename="MINIMUM")]
    MINIMUM,
    

    /// Includes `MINIMUM` and, for each resource owner, returns: * The resource owner's name * Most specific part of the `X-Consent-Scope` resulting in consensual determination * Timestamp of the applied enforcement leading to the decision * Enforcement version at the time the applicable consents were applied * The Consent resource name * The timestamp of the Consent resource used for enforcement * Policy type (`PATIENT` or `ADMIN`) Note that this mode adds some overhead to CRUD operations.
    ///
    /// "VERBOSE"
    #[serde(rename="VERBOSE")]
    VERBOSE,
}

impl AsRef<str> for AccessDeterminationLogConfigLogLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccessDeterminationLogConfigLogLevelEnum::LOGLEVELUNSPECIFIED => "LOG_LEVEL_UNSPECIFIED",
            AccessDeterminationLogConfigLogLevelEnum::DISABLED => "DISABLED",
            AccessDeterminationLogConfigLogLevelEnum::MINIMUM => "MINIMUM",
            AccessDeterminationLogConfigLogLevelEnum::VERBOSE => "VERBOSE",
        }
    }
}

impl std::convert::TryFrom< &str> for AccessDeterminationLogConfigLogLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_LEVEL_UNSPECIFIED" => Ok(AccessDeterminationLogConfigLogLevelEnum::LOGLEVELUNSPECIFIED),
           "DISABLED" => Ok(AccessDeterminationLogConfigLogLevelEnum::DISABLED),
           "MINIMUM" => Ok(AccessDeterminationLogConfigLogLevelEnum::MINIMUM),
           "VERBOSE" => Ok(AccessDeterminationLogConfigLogLevelEnum::VERBOSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccessDeterminationLogConfigLogLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AnalyzeEntitiesRequestAlternativeOutputFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Alternative output format to be generated based on the results of analysis.
pub enum AnalyzeEntitiesRequestAlternativeOutputFormatEnum {
    

    /// No alternative output format is specified.
    ///
    /// "ALTERNATIVE_OUTPUT_FORMAT_UNSPECIFIED"
    #[serde(rename="ALTERNATIVE_OUTPUT_FORMAT_UNSPECIFIED")]
    ALTERNATIVEOUTPUTFORMATUNSPECIFIED,
    

    /// FHIR bundle output.
    ///
    /// "FHIR_BUNDLE"
    #[serde(rename="FHIR_BUNDLE")]
    FHIRBUNDLE,
}

impl AsRef<str> for AnalyzeEntitiesRequestAlternativeOutputFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AnalyzeEntitiesRequestAlternativeOutputFormatEnum::ALTERNATIVEOUTPUTFORMATUNSPECIFIED => "ALTERNATIVE_OUTPUT_FORMAT_UNSPECIFIED",
            AnalyzeEntitiesRequestAlternativeOutputFormatEnum::FHIRBUNDLE => "FHIR_BUNDLE",
        }
    }
}

impl std::convert::TryFrom< &str> for AnalyzeEntitiesRequestAlternativeOutputFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALTERNATIVE_OUTPUT_FORMAT_UNSPECIFIED" => Ok(AnalyzeEntitiesRequestAlternativeOutputFormatEnum::ALTERNATIVEOUTPUTFORMATUNSPECIFIED),
           "FHIR_BUNDLE" => Ok(AnalyzeEntitiesRequestAlternativeOutputFormatEnum::FHIRBUNDLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AnalyzeEntitiesRequestAlternativeOutputFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AnalyzeEntitiesRequestLicensedVocabulariesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of licensed vocabularies to use in the request, in addition to the default unlicensed vocabularies.
pub enum AnalyzeEntitiesRequestLicensedVocabulariesEnum {
    

    /// No licensed vocabulary specified.
    ///
    /// "LICENSED_VOCABULARY_UNSPECIFIED"
    #[serde(rename="LICENSED_VOCABULARY_UNSPECIFIED")]
    LICENSEDVOCABULARYUNSPECIFIED,
    

    /// ICD-10-CM vocabulary
    ///
    /// "ICD10CM"
    #[serde(rename="ICD10CM")]
    ICD10CM,
    

    /// SNOMED CT (US version) vocabulary
    ///
    /// "SNOMEDCT_US"
    #[serde(rename="SNOMEDCT_US")]
    SNOMEDCTUS,
}

impl AsRef<str> for AnalyzeEntitiesRequestLicensedVocabulariesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AnalyzeEntitiesRequestLicensedVocabulariesEnum::LICENSEDVOCABULARYUNSPECIFIED => "LICENSED_VOCABULARY_UNSPECIFIED",
            AnalyzeEntitiesRequestLicensedVocabulariesEnum::ICD10CM => "ICD10CM",
            AnalyzeEntitiesRequestLicensedVocabulariesEnum::SNOMEDCTUS => "SNOMEDCT_US",
        }
    }
}

impl std::convert::TryFrom< &str> for AnalyzeEntitiesRequestLicensedVocabulariesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LICENSED_VOCABULARY_UNSPECIFIED" => Ok(AnalyzeEntitiesRequestLicensedVocabulariesEnum::LICENSEDVOCABULARYUNSPECIFIED),
           "ICD10CM" => Ok(AnalyzeEntitiesRequestLicensedVocabulariesEnum::ICD10CM),
           "SNOMEDCT_US" => Ok(AnalyzeEntitiesRequestLicensedVocabulariesEnum::SNOMEDCTUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AnalyzeEntitiesRequestLicensedVocabulariesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AttributeDefinitionCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The category of the attribute. The value of this field cannot be changed after creation.
pub enum AttributeDefinitionCategoryEnum {
    

    /// No category specified. This option is invalid.
    ///
    /// "CATEGORY_UNSPECIFIED"
    #[serde(rename="CATEGORY_UNSPECIFIED")]
    CATEGORYUNSPECIFIED,
    

    /// Specify this category when this attribute describes the properties of resources. For example, data anonymity or data type.
    ///
    /// "RESOURCE"
    #[serde(rename="RESOURCE")]
    RESOURCE,
    

    /// Specify this category when this attribute describes the properties of requests. For example, requester's role or requester's organization.
    ///
    /// "REQUEST"
    #[serde(rename="REQUEST")]
    REQUEST,
}

impl AsRef<str> for AttributeDefinitionCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AttributeDefinitionCategoryEnum::CATEGORYUNSPECIFIED => "CATEGORY_UNSPECIFIED",
            AttributeDefinitionCategoryEnum::RESOURCE => "RESOURCE",
            AttributeDefinitionCategoryEnum::REQUEST => "REQUEST",
        }
    }
}

impl std::convert::TryFrom< &str> for AttributeDefinitionCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CATEGORY_UNSPECIFIED" => Ok(AttributeDefinitionCategoryEnum::CATEGORYUNSPECIFIED),
           "RESOURCE" => Ok(AttributeDefinitionCategoryEnum::RESOURCE),
           "REQUEST" => Ok(AttributeDefinitionCategoryEnum::REQUEST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AttributeDefinitionCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


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


// region BlobStorageInfoStorageClassEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The storage class in which the Blob data is stored.
pub enum BlobStorageInfoStorageClassEnum {
    

    /// If unspecified in CreateDataset, the StorageClass defaults to STANDARD. If unspecified in UpdateDataset and the StorageClass is set in the field mask, an InvalidRequest error is thrown.
    ///
    /// "BLOB_STORAGE_CLASS_UNSPECIFIED"
    #[serde(rename="BLOB_STORAGE_CLASS_UNSPECIFIED")]
    BLOBSTORAGECLASSUNSPECIFIED,
    

    /// This stores the Object in Blob Standard Storage: https://cloud.google.com/storage/docs/storage-classes#standard
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// This stores the Object in Blob Nearline Storage: https://cloud.google.com/storage/docs/storage-classes#nearline
    ///
    /// "NEARLINE"
    #[serde(rename="NEARLINE")]
    NEARLINE,
    

    /// This stores the Object in Blob Coldline Storage: https://cloud.google.com/storage/docs/storage-classes#coldline
    ///
    /// "COLDLINE"
    #[serde(rename="COLDLINE")]
    COLDLINE,
    

    /// This stores the Object in Blob Archive Storage: https://cloud.google.com/storage/docs/storage-classes#archive
    ///
    /// "ARCHIVE"
    #[serde(rename="ARCHIVE")]
    ARCHIVE,
}

impl AsRef<str> for BlobStorageInfoStorageClassEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BlobStorageInfoStorageClassEnum::BLOBSTORAGECLASSUNSPECIFIED => "BLOB_STORAGE_CLASS_UNSPECIFIED",
            BlobStorageInfoStorageClassEnum::STANDARD => "STANDARD",
            BlobStorageInfoStorageClassEnum::NEARLINE => "NEARLINE",
            BlobStorageInfoStorageClassEnum::COLDLINE => "COLDLINE",
            BlobStorageInfoStorageClassEnum::ARCHIVE => "ARCHIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for BlobStorageInfoStorageClassEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BLOB_STORAGE_CLASS_UNSPECIFIED" => Ok(BlobStorageInfoStorageClassEnum::BLOBSTORAGECLASSUNSPECIFIED),
           "STANDARD" => Ok(BlobStorageInfoStorageClassEnum::STANDARD),
           "NEARLINE" => Ok(BlobStorageInfoStorageClassEnum::NEARLINE),
           "COLDLINE" => Ok(BlobStorageInfoStorageClassEnum::COLDLINE),
           "ARCHIVE" => Ok(BlobStorageInfoStorageClassEnum::ARCHIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BlobStorageInfoStorageClassEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BlobStorageSettingBlobStorageClassEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Storage class in which the Blob data is stored.
pub enum BlobStorageSettingBlobStorageClassEnum {
    

    /// If unspecified in CreateDataset, the StorageClass defaults to STANDARD. If unspecified in UpdateDataset and the StorageClass is set in the field mask, an InvalidRequest error is thrown.
    ///
    /// "BLOB_STORAGE_CLASS_UNSPECIFIED"
    #[serde(rename="BLOB_STORAGE_CLASS_UNSPECIFIED")]
    BLOBSTORAGECLASSUNSPECIFIED,
    

    /// This stores the Object in Blob Standard Storage: https://cloud.google.com/storage/docs/storage-classes#standard
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// This stores the Object in Blob Nearline Storage: https://cloud.google.com/storage/docs/storage-classes#nearline
    ///
    /// "NEARLINE"
    #[serde(rename="NEARLINE")]
    NEARLINE,
    

    /// This stores the Object in Blob Coldline Storage: https://cloud.google.com/storage/docs/storage-classes#coldline
    ///
    /// "COLDLINE"
    #[serde(rename="COLDLINE")]
    COLDLINE,
    

    /// This stores the Object in Blob Archive Storage: https://cloud.google.com/storage/docs/storage-classes#archive
    ///
    /// "ARCHIVE"
    #[serde(rename="ARCHIVE")]
    ARCHIVE,
}

impl AsRef<str> for BlobStorageSettingBlobStorageClassEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BlobStorageSettingBlobStorageClassEnum::BLOBSTORAGECLASSUNSPECIFIED => "BLOB_STORAGE_CLASS_UNSPECIFIED",
            BlobStorageSettingBlobStorageClassEnum::STANDARD => "STANDARD",
            BlobStorageSettingBlobStorageClassEnum::NEARLINE => "NEARLINE",
            BlobStorageSettingBlobStorageClassEnum::COLDLINE => "COLDLINE",
            BlobStorageSettingBlobStorageClassEnum::ARCHIVE => "ARCHIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for BlobStorageSettingBlobStorageClassEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BLOB_STORAGE_CLASS_UNSPECIFIED" => Ok(BlobStorageSettingBlobStorageClassEnum::BLOBSTORAGECLASSUNSPECIFIED),
           "STANDARD" => Ok(BlobStorageSettingBlobStorageClassEnum::STANDARD),
           "NEARLINE" => Ok(BlobStorageSettingBlobStorageClassEnum::NEARLINE),
           "COLDLINE" => Ok(BlobStorageSettingBlobStorageClassEnum::COLDLINE),
           "ARCHIVE" => Ok(BlobStorageSettingBlobStorageClassEnum::ARCHIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BlobStorageSettingBlobStorageClassEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CheckDataAccessRequestResponseViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The view for CheckDataAccessResponse. If unspecified, defaults to `BASIC` and returns `consented` as `TRUE` or `FALSE`.
pub enum CheckDataAccessRequestResponseViewEnum {
    

    /// No response view specified. The API will default to the BASIC view.
    ///
    /// "RESPONSE_VIEW_UNSPECIFIED"
    #[serde(rename="RESPONSE_VIEW_UNSPECIFIED")]
    RESPONSEVIEWUNSPECIFIED,
    

    /// Only the `consented` field is populated in CheckDataAccessResponse.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// All fields within CheckDataAccessResponse are populated. When set to `FULL`, all `ACTIVE` Consents are evaluated even if a matching policy is found during evaluation.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for CheckDataAccessRequestResponseViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CheckDataAccessRequestResponseViewEnum::RESPONSEVIEWUNSPECIFIED => "RESPONSE_VIEW_UNSPECIFIED",
            CheckDataAccessRequestResponseViewEnum::BASIC => "BASIC",
            CheckDataAccessRequestResponseViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for CheckDataAccessRequestResponseViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESPONSE_VIEW_UNSPECIFIED" => Ok(CheckDataAccessRequestResponseViewEnum::RESPONSEVIEWUNSPECIFIED),
           "BASIC" => Ok(CheckDataAccessRequestResponseViewEnum::BASIC),
           "FULL" => Ok(CheckDataAccessRequestResponseViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CheckDataAccessRequestResponseViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConsentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates the current state of this Consent.
pub enum ConsentStateEnum {
    

    /// No state specified. Treated as ACTIVE only at the time of resource creation.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The Consent is active and is considered when evaluating a user's consent on resources.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The archived state is currently not being used.
    ///
    /// "ARCHIVED"
    #[serde(rename="ARCHIVED")]
    ARCHIVED,
    

    /// A revoked Consent is not considered when evaluating a user's consent on resources.
    ///
    /// "REVOKED"
    #[serde(rename="REVOKED")]
    REVOKED,
    

    /// A draft Consent is not considered when evaluating a user's consent on resources unless explicitly specified.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// When a draft Consent is rejected by a user, it is set to a rejected state. A rejected Consent is not considered when evaluating a user's consent on resources.
    ///
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
}

impl AsRef<str> for ConsentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConsentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ConsentStateEnum::ACTIVE => "ACTIVE",
            ConsentStateEnum::ARCHIVED => "ARCHIVED",
            ConsentStateEnum::REVOKED => "REVOKED",
            ConsentStateEnum::DRAFT => "DRAFT",
            ConsentStateEnum::REJECTED => "REJECTED",
        }
    }
}

impl std::convert::TryFrom< &str> for ConsentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ConsentStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(ConsentStateEnum::ACTIVE),
           "ARCHIVED" => Ok(ConsentStateEnum::ARCHIVED),
           "REVOKED" => Ok(ConsentStateEnum::REVOKED),
           "DRAFT" => Ok(ConsentStateEnum::DRAFT),
           "REJECTED" => Ok(ConsentStateEnum::REJECTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConsentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConsentConfigVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Specifies which consent enforcement version is being used for this FHIR store. This field can only be set once by either CreateFhirStore or UpdateFhirStore. After that, you must call ApplyConsents to change the version.
pub enum ConsentConfigVersionEnum {
    

    /// Users must specify an enforcement version or an error is returned.
    ///
    /// "CONSENT_ENFORCEMENT_VERSION_UNSPECIFIED"
    #[serde(rename="CONSENT_ENFORCEMENT_VERSION_UNSPECIFIED")]
    CONSENTENFORCEMENTVERSIONUNSPECIFIED,
    

    /// Enforcement version 1. See the [FHIR Consent resources in the Cloud Healthcare API](https://cloud.google.com/healthcare-api/docs/fhir-consent) guide for more details.
    ///
    /// "V1"
    #[serde(rename="V1")]
    V1,
}

impl AsRef<str> for ConsentConfigVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConsentConfigVersionEnum::CONSENTENFORCEMENTVERSIONUNSPECIFIED => "CONSENT_ENFORCEMENT_VERSION_UNSPECIFIED",
            ConsentConfigVersionEnum::V1 => "V1",
        }
    }
}

impl std::convert::TryFrom< &str> for ConsentConfigVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONSENT_ENFORCEMENT_VERSION_UNSPECIFIED" => Ok(ConsentConfigVersionEnum::CONSENTENFORCEMENTVERSIONUNSPECIFIED),
           "V1" => Ok(ConsentConfigVersionEnum::V1),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConsentConfigVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConsentEvaluationEvaluationResultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The evaluation result.
pub enum ConsentEvaluationEvaluationResultEnum {
    

    /// No evaluation result specified. This option is invalid.
    ///
    /// "EVALUATION_RESULT_UNSPECIFIED"
    #[serde(rename="EVALUATION_RESULT_UNSPECIFIED")]
    EVALUATIONRESULTUNSPECIFIED,
    

    /// The Consent is not applicable to the requested access determination. For example, the Consent does not apply to the user for which the access determination is requested, or it has a `state` of `REVOKED`, or it has expired.
    ///
    /// "NOT_APPLICABLE"
    #[serde(rename="NOT_APPLICABLE")]
    NOTAPPLICABLE,
    

    /// The Consent does not have a policy that matches the `resource_attributes` of the evaluated resource.
    ///
    /// "NO_MATCHING_POLICY"
    #[serde(rename="NO_MATCHING_POLICY")]
    NOMATCHINGPOLICY,
    

    /// The Consent has at least one policy that matches the `resource_attributes` of the evaluated resource, but no `authorization_rule` was satisfied.
    ///
    /// "NO_SATISFIED_POLICY"
    #[serde(rename="NO_SATISFIED_POLICY")]
    NOSATISFIEDPOLICY,
    

    /// The Consent has at least one policy that matches the `resource_attributes` of the evaluated resource, and at least one `authorization_rule` was satisfied.
    ///
    /// "HAS_SATISFIED_POLICY"
    #[serde(rename="HAS_SATISFIED_POLICY")]
    HASSATISFIEDPOLICY,
}

impl AsRef<str> for ConsentEvaluationEvaluationResultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConsentEvaluationEvaluationResultEnum::EVALUATIONRESULTUNSPECIFIED => "EVALUATION_RESULT_UNSPECIFIED",
            ConsentEvaluationEvaluationResultEnum::NOTAPPLICABLE => "NOT_APPLICABLE",
            ConsentEvaluationEvaluationResultEnum::NOMATCHINGPOLICY => "NO_MATCHING_POLICY",
            ConsentEvaluationEvaluationResultEnum::NOSATISFIEDPOLICY => "NO_SATISFIED_POLICY",
            ConsentEvaluationEvaluationResultEnum::HASSATISFIEDPOLICY => "HAS_SATISFIED_POLICY",
        }
    }
}

impl std::convert::TryFrom< &str> for ConsentEvaluationEvaluationResultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVALUATION_RESULT_UNSPECIFIED" => Ok(ConsentEvaluationEvaluationResultEnum::EVALUATIONRESULTUNSPECIFIED),
           "NOT_APPLICABLE" => Ok(ConsentEvaluationEvaluationResultEnum::NOTAPPLICABLE),
           "NO_MATCHING_POLICY" => Ok(ConsentEvaluationEvaluationResultEnum::NOMATCHINGPOLICY),
           "NO_SATISFIED_POLICY" => Ok(ConsentEvaluationEvaluationResultEnum::NOSATISFIEDPOLICY),
           "HAS_SATISFIED_POLICY" => Ok(ConsentEvaluationEvaluationResultEnum::HASSATISFIEDPOLICY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConsentEvaluationEvaluationResultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConsentHeaderHandlingProfileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies the default server behavior when the header is empty. If not specified, the `ScopeProfile.PERMIT_EMPTY_SCOPE` option is used.
pub enum ConsentHeaderHandlingProfileEnum {
    

    /// If not specified, the default value `PERMIT_EMPTY_SCOPE` is used.
    ///
    /// "SCOPE_PROFILE_UNSPECIFIED"
    #[serde(rename="SCOPE_PROFILE_UNSPECIFIED")]
    SCOPEPROFILEUNSPECIFIED,
    

    /// When no consent scopes are provided (for example, if there's an empty or missing header), then consent check is disabled, similar to when `access_enforced` is `false`. You can use audit logs to differentiate these two cases by looking at the value of `protopayload.metadata.consentMode`. If consents scopes are present, they must be valid and within the allowed limits, otherwise the request will be rejected with a `4xx` code.
    ///
    /// "PERMIT_EMPTY_SCOPE"
    #[serde(rename="PERMIT_EMPTY_SCOPE")]
    PERMITEMPTYSCOPE,
    

    /// The consent header must be non-empty when performing read and search operations, otherwise the request is rejected with a `4xx` code. Additionally, invalid consent scopes or scopes exceeding the allowed limits are rejected.
    ///
    /// "REQUIRED_ON_READ"
    #[serde(rename="REQUIRED_ON_READ")]
    REQUIREDONREAD,
}

impl AsRef<str> for ConsentHeaderHandlingProfileEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConsentHeaderHandlingProfileEnum::SCOPEPROFILEUNSPECIFIED => "SCOPE_PROFILE_UNSPECIFIED",
            ConsentHeaderHandlingProfileEnum::PERMITEMPTYSCOPE => "PERMIT_EMPTY_SCOPE",
            ConsentHeaderHandlingProfileEnum::REQUIREDONREAD => "REQUIRED_ON_READ",
        }
    }
}

impl std::convert::TryFrom< &str> for ConsentHeaderHandlingProfileEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCOPE_PROFILE_UNSPECIFIED" => Ok(ConsentHeaderHandlingProfileEnum::SCOPEPROFILEUNSPECIFIED),
           "PERMIT_EMPTY_SCOPE" => Ok(ConsentHeaderHandlingProfileEnum::PERMITEMPTYSCOPE),
           "REQUIRED_ON_READ" => Ok(ConsentHeaderHandlingProfileEnum::REQUIREDONREAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConsentHeaderHandlingProfileEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DicomConfigFilterProfileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tag filtering profile that determines which tags to keep/remove.
pub enum DicomConfigFilterProfileEnum {
    

    /// No tag filtration profile provided. Same as KEEP_ALL_PROFILE.
    ///
    /// "TAG_FILTER_PROFILE_UNSPECIFIED"
    #[serde(rename="TAG_FILTER_PROFILE_UNSPECIFIED")]
    TAGFILTERPROFILEUNSPECIFIED,
    

    /// Keep only the tags required to produce valid DICOM objects.
    ///
    /// "MINIMAL_KEEP_LIST_PROFILE"
    #[serde(rename="MINIMAL_KEEP_LIST_PROFILE")]
    MINIMALKEEPLISTPROFILE,
    

    /// Remove tags based on DICOM Standard's Attribute Confidentiality Basic Profile (DICOM Standard Edition 2018e) http://dicom.nema.org/medical/dicom/2018e/output/chtml/part15/chapter_E.html.
    ///
    /// "ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE"
    #[serde(rename="ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE")]
    ATTRIBUTECONFIDENTIALITYBASICPROFILE,
    

    /// Keep all tags.
    ///
    /// "KEEP_ALL_PROFILE"
    #[serde(rename="KEEP_ALL_PROFILE")]
    KEEPALLPROFILE,
    

    /// Inspect within tag contents and replace sensitive text. The process can be configured using the TextConfig. Applies to all tags with the following Value Representation names: AE, LO, LT, PN, SH, ST, UC, UT, DA, DT, AS
    ///
    /// "DEIDENTIFY_TAG_CONTENTS"
    #[serde(rename="DEIDENTIFY_TAG_CONTENTS")]
    DEIDENTIFYTAGCONTENTS,
}

impl AsRef<str> for DicomConfigFilterProfileEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DicomConfigFilterProfileEnum::TAGFILTERPROFILEUNSPECIFIED => "TAG_FILTER_PROFILE_UNSPECIFIED",
            DicomConfigFilterProfileEnum::MINIMALKEEPLISTPROFILE => "MINIMAL_KEEP_LIST_PROFILE",
            DicomConfigFilterProfileEnum::ATTRIBUTECONFIDENTIALITYBASICPROFILE => "ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE",
            DicomConfigFilterProfileEnum::KEEPALLPROFILE => "KEEP_ALL_PROFILE",
            DicomConfigFilterProfileEnum::DEIDENTIFYTAGCONTENTS => "DEIDENTIFY_TAG_CONTENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for DicomConfigFilterProfileEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TAG_FILTER_PROFILE_UNSPECIFIED" => Ok(DicomConfigFilterProfileEnum::TAGFILTERPROFILEUNSPECIFIED),
           "MINIMAL_KEEP_LIST_PROFILE" => Ok(DicomConfigFilterProfileEnum::MINIMALKEEPLISTPROFILE),
           "ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE" => Ok(DicomConfigFilterProfileEnum::ATTRIBUTECONFIDENTIALITYBASICPROFILE),
           "KEEP_ALL_PROFILE" => Ok(DicomConfigFilterProfileEnum::KEEPALLPROFILE),
           "DEIDENTIFY_TAG_CONTENTS" => Ok(DicomConfigFilterProfileEnum::DEIDENTIFYTAGCONTENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DicomConfigFilterProfileEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DicomTagConfigProfileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Base profile type for handling DICOM tags.
pub enum DicomTagConfigProfileTypeEnum {
    

    /// No profile provided. Same as `ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE`.
    ///
    /// "PROFILE_TYPE_UNSPECIFIED"
    #[serde(rename="PROFILE_TYPE_UNSPECIFIED")]
    PROFILETYPEUNSPECIFIED,
    

    /// Keep only the tags required to produce valid DICOM objects.
    ///
    /// "MINIMAL_KEEP_LIST_PROFILE"
    #[serde(rename="MINIMAL_KEEP_LIST_PROFILE")]
    MINIMALKEEPLISTPROFILE,
    

    /// Remove tags based on DICOM Standard's [Attribute Confidentiality Basic Profile (DICOM Standard Edition 2018e)](http://dicom.nema.org/medical/dicom/2018e/output/chtml/part15/chapter_E.html).
    ///
    /// "ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE"
    #[serde(rename="ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE")]
    ATTRIBUTECONFIDENTIALITYBASICPROFILE,
    

    /// Keep all tags.
    ///
    /// "KEEP_ALL_PROFILE"
    #[serde(rename="KEEP_ALL_PROFILE")]
    KEEPALLPROFILE,
    

    /// Inspect tag contents and replace sensitive text. The process can be configured using the TextConfig. Applies to all tags with the following [Value Representations] (http://dicom.nema.org/medical/dicom/2018e/output/chtml/part05/sect_6.2.html#table_6.2-1): AE, LO, LT, PN, SH, ST, UC, UT, DA, DT, AS
    ///
    /// "DEIDENTIFY_TAG_CONTENTS"
    #[serde(rename="DEIDENTIFY_TAG_CONTENTS")]
    DEIDENTIFYTAGCONTENTS,
}

impl AsRef<str> for DicomTagConfigProfileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DicomTagConfigProfileTypeEnum::PROFILETYPEUNSPECIFIED => "PROFILE_TYPE_UNSPECIFIED",
            DicomTagConfigProfileTypeEnum::MINIMALKEEPLISTPROFILE => "MINIMAL_KEEP_LIST_PROFILE",
            DicomTagConfigProfileTypeEnum::ATTRIBUTECONFIDENTIALITYBASICPROFILE => "ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE",
            DicomTagConfigProfileTypeEnum::KEEPALLPROFILE => "KEEP_ALL_PROFILE",
            DicomTagConfigProfileTypeEnum::DEIDENTIFYTAGCONTENTS => "DEIDENTIFY_TAG_CONTENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for DicomTagConfigProfileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROFILE_TYPE_UNSPECIFIED" => Ok(DicomTagConfigProfileTypeEnum::PROFILETYPEUNSPECIFIED),
           "MINIMAL_KEEP_LIST_PROFILE" => Ok(DicomTagConfigProfileTypeEnum::MINIMALKEEPLISTPROFILE),
           "ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE" => Ok(DicomTagConfigProfileTypeEnum::ATTRIBUTECONFIDENTIALITYBASICPROFILE),
           "KEEP_ALL_PROFILE" => Ok(DicomTagConfigProfileTypeEnum::KEEPALLPROFILE),
           "DEIDENTIFY_TAG_CONTENTS" => Ok(DicomTagConfigProfileTypeEnum::DEIDENTIFYTAGCONTENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DicomTagConfigProfileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EvaluateUserConsentsRequestResponseViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The view for EvaluateUserConsentsResponse. If unspecified, defaults to `BASIC` and returns `consented` as `TRUE` or `FALSE`.
pub enum EvaluateUserConsentsRequestResponseViewEnum {
    

    /// No response view specified. The API will default to the BASIC view.
    ///
    /// "RESPONSE_VIEW_UNSPECIFIED"
    #[serde(rename="RESPONSE_VIEW_UNSPECIFIED")]
    RESPONSEVIEWUNSPECIFIED,
    

    /// Only the `data_id` and `consented` fields are populated in the response.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// All fields within the response are populated. When set to `FULL`, all `ACTIVE` Consents are evaluated even if a matching policy is found during evaluation.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for EvaluateUserConsentsRequestResponseViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EvaluateUserConsentsRequestResponseViewEnum::RESPONSEVIEWUNSPECIFIED => "RESPONSE_VIEW_UNSPECIFIED",
            EvaluateUserConsentsRequestResponseViewEnum::BASIC => "BASIC",
            EvaluateUserConsentsRequestResponseViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for EvaluateUserConsentsRequestResponseViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESPONSE_VIEW_UNSPECIFIED" => Ok(EvaluateUserConsentsRequestResponseViewEnum::RESPONSEVIEWUNSPECIFIED),
           "BASIC" => Ok(EvaluateUserConsentsRequestResponseViewEnum::BASIC),
           "FULL" => Ok(EvaluateUserConsentsRequestResponseViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EvaluateUserConsentsRequestResponseViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExplainDataAccessConsentInfoTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The policy type of consent resource (e.g. PATIENT, ADMIN).
pub enum ExplainDataAccessConsentInfoTypeEnum {
    

    /// Unspecified policy type.
    ///
    /// "CONSENT_POLICY_TYPE_UNSPECIFIED"
    #[serde(rename="CONSENT_POLICY_TYPE_UNSPECIFIED")]
    CONSENTPOLICYTYPEUNSPECIFIED,
    

    /// Consent represent a patient consent.
    ///
    /// "CONSENT_POLICY_TYPE_PATIENT"
    #[serde(rename="CONSENT_POLICY_TYPE_PATIENT")]
    CONSENTPOLICYTYPEPATIENT,
    

    /// Consent represent an admin consent.
    ///
    /// "CONSENT_POLICY_TYPE_ADMIN"
    #[serde(rename="CONSENT_POLICY_TYPE_ADMIN")]
    CONSENTPOLICYTYPEADMIN,
}

impl AsRef<str> for ExplainDataAccessConsentInfoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExplainDataAccessConsentInfoTypeEnum::CONSENTPOLICYTYPEUNSPECIFIED => "CONSENT_POLICY_TYPE_UNSPECIFIED",
            ExplainDataAccessConsentInfoTypeEnum::CONSENTPOLICYTYPEPATIENT => "CONSENT_POLICY_TYPE_PATIENT",
            ExplainDataAccessConsentInfoTypeEnum::CONSENTPOLICYTYPEADMIN => "CONSENT_POLICY_TYPE_ADMIN",
        }
    }
}

impl std::convert::TryFrom< &str> for ExplainDataAccessConsentInfoTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONSENT_POLICY_TYPE_UNSPECIFIED" => Ok(ExplainDataAccessConsentInfoTypeEnum::CONSENTPOLICYTYPEUNSPECIFIED),
           "CONSENT_POLICY_TYPE_PATIENT" => Ok(ExplainDataAccessConsentInfoTypeEnum::CONSENTPOLICYTYPEPATIENT),
           "CONSENT_POLICY_TYPE_ADMIN" => Ok(ExplainDataAccessConsentInfoTypeEnum::CONSENTPOLICYTYPEADMIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExplainDataAccessConsentInfoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExplainDataAccessConsentInfoVariantsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The consent's variant combinations. A single consent may have multiple variants.
pub enum ExplainDataAccessConsentInfoVariantsEnum {
    

    /// Consent variant unspecified.
    ///
    /// "CONSENT_VARIANT_UNSPECIFIED"
    #[serde(rename="CONSENT_VARIANT_UNSPECIFIED")]
    CONSENTVARIANTUNSPECIFIED,
    

    /// Consent is a standard patient or admin consent.
    ///
    /// "CONSENT_VARIANT_STANDARD"
    #[serde(rename="CONSENT_VARIANT_STANDARD")]
    CONSENTVARIANTSTANDARD,
    

    /// Consent is a cascading consent.
    ///
    /// "CONSENT_VARIANT_CASCADE"
    #[serde(rename="CONSENT_VARIANT_CASCADE")]
    CONSENTVARIANTCASCADE,
}

impl AsRef<str> for ExplainDataAccessConsentInfoVariantsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExplainDataAccessConsentInfoVariantsEnum::CONSENTVARIANTUNSPECIFIED => "CONSENT_VARIANT_UNSPECIFIED",
            ExplainDataAccessConsentInfoVariantsEnum::CONSENTVARIANTSTANDARD => "CONSENT_VARIANT_STANDARD",
            ExplainDataAccessConsentInfoVariantsEnum::CONSENTVARIANTCASCADE => "CONSENT_VARIANT_CASCADE",
        }
    }
}

impl std::convert::TryFrom< &str> for ExplainDataAccessConsentInfoVariantsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONSENT_VARIANT_UNSPECIFIED" => Ok(ExplainDataAccessConsentInfoVariantsEnum::CONSENTVARIANTUNSPECIFIED),
           "CONSENT_VARIANT_STANDARD" => Ok(ExplainDataAccessConsentInfoVariantsEnum::CONSENTVARIANTSTANDARD),
           "CONSENT_VARIANT_CASCADE" => Ok(ExplainDataAccessConsentInfoVariantsEnum::CONSENTVARIANTCASCADE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExplainDataAccessConsentInfoVariantsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExplainDataAccessConsentScopeDecisionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the current consent scope is permitted or denied access on the requested resource.
pub enum ExplainDataAccessConsentScopeDecisionEnum {
    

    /// Unspecified consent decision type.
    ///
    /// "CONSENT_DECISION_TYPE_UNSPECIFIED"
    #[serde(rename="CONSENT_DECISION_TYPE_UNSPECIFIED")]
    CONSENTDECISIONTYPEUNSPECIFIED,
    

    /// Consent permitted access.
    ///
    /// "CONSENT_DECISION_TYPE_PERMIT"
    #[serde(rename="CONSENT_DECISION_TYPE_PERMIT")]
    CONSENTDECISIONTYPEPERMIT,
    

    /// Consent denied access.
    ///
    /// "CONSENT_DECISION_TYPE_DENY"
    #[serde(rename="CONSENT_DECISION_TYPE_DENY")]
    CONSENTDECISIONTYPEDENY,
}

impl AsRef<str> for ExplainDataAccessConsentScopeDecisionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExplainDataAccessConsentScopeDecisionEnum::CONSENTDECISIONTYPEUNSPECIFIED => "CONSENT_DECISION_TYPE_UNSPECIFIED",
            ExplainDataAccessConsentScopeDecisionEnum::CONSENTDECISIONTYPEPERMIT => "CONSENT_DECISION_TYPE_PERMIT",
            ExplainDataAccessConsentScopeDecisionEnum::CONSENTDECISIONTYPEDENY => "CONSENT_DECISION_TYPE_DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for ExplainDataAccessConsentScopeDecisionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONSENT_DECISION_TYPE_UNSPECIFIED" => Ok(ExplainDataAccessConsentScopeDecisionEnum::CONSENTDECISIONTYPEUNSPECIFIED),
           "CONSENT_DECISION_TYPE_PERMIT" => Ok(ExplainDataAccessConsentScopeDecisionEnum::CONSENTDECISIONTYPEPERMIT),
           "CONSENT_DECISION_TYPE_DENY" => Ok(ExplainDataAccessConsentScopeDecisionEnum::CONSENTDECISIONTYPEDENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExplainDataAccessConsentScopeDecisionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FhirFieldConfigProfileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Base profile type for handling FHIR fields.
pub enum FhirFieldConfigProfileTypeEnum {
    

    /// No profile provided. Same as `BASIC`.
    ///
    /// "PROFILE_TYPE_UNSPECIFIED"
    #[serde(rename="PROFILE_TYPE_UNSPECIFIED")]
    PROFILETYPEUNSPECIFIED,
    

    /// Keep all fields.
    ///
    /// "KEEP_ALL"
    #[serde(rename="KEEP_ALL")]
    KEEPALL,
    

    /// Transforms known [HIPAA 18](https://www.hhs.gov/hipaa/for-professionals/privacy/special-topics/de-identification/index.html#standard) fields and cleans known unstructured text fields.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Cleans all supported tags. Applies to types: Code, Date, DateTime, Decimal, HumanName, Id, LanguageCode, Markdown, Oid, String, Uri, Uuid, Xhtml.
    ///
    /// "CLEAN_ALL"
    #[serde(rename="CLEAN_ALL")]
    CLEANALL,
}

impl AsRef<str> for FhirFieldConfigProfileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FhirFieldConfigProfileTypeEnum::PROFILETYPEUNSPECIFIED => "PROFILE_TYPE_UNSPECIFIED",
            FhirFieldConfigProfileTypeEnum::KEEPALL => "KEEP_ALL",
            FhirFieldConfigProfileTypeEnum::BASIC => "BASIC",
            FhirFieldConfigProfileTypeEnum::CLEANALL => "CLEAN_ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for FhirFieldConfigProfileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROFILE_TYPE_UNSPECIFIED" => Ok(FhirFieldConfigProfileTypeEnum::PROFILETYPEUNSPECIFIED),
           "KEEP_ALL" => Ok(FhirFieldConfigProfileTypeEnum::KEEPALL),
           "BASIC" => Ok(FhirFieldConfigProfileTypeEnum::BASIC),
           "CLEAN_ALL" => Ok(FhirFieldConfigProfileTypeEnum::CLEANALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FhirFieldConfigProfileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FhirStoreComplexDataTypeReferenceParsingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Enable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources.
pub enum FhirStoreComplexDataTypeReferenceParsingEnum {
    

    /// No parsing behavior specified. This is the same as DISABLED for backwards compatibility.
    ///
    /// "COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED"
    #[serde(rename="COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED")]
    COMPLEXDATATYPEREFERENCEPARSINGUNSPECIFIED,
    

    /// References in complex data types are ignored.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// References in complex data types are parsed.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
}

impl AsRef<str> for FhirStoreComplexDataTypeReferenceParsingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FhirStoreComplexDataTypeReferenceParsingEnum::COMPLEXDATATYPEREFERENCEPARSINGUNSPECIFIED => "COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED",
            FhirStoreComplexDataTypeReferenceParsingEnum::DISABLED => "DISABLED",
            FhirStoreComplexDataTypeReferenceParsingEnum::ENABLED => "ENABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for FhirStoreComplexDataTypeReferenceParsingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED" => Ok(FhirStoreComplexDataTypeReferenceParsingEnum::COMPLEXDATATYPEREFERENCEPARSINGUNSPECIFIED),
           "DISABLED" => Ok(FhirStoreComplexDataTypeReferenceParsingEnum::DISABLED),
           "ENABLED" => Ok(FhirStoreComplexDataTypeReferenceParsingEnum::ENABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FhirStoreComplexDataTypeReferenceParsingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FhirStoreVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The FHIR specification version that this FHIR store supports natively. This field is immutable after store creation. Requests are rejected if they contain FHIR resources of a different version. Version is required for every FHIR store.
pub enum FhirStoreVersionEnum {
    

    /// VERSION_UNSPECIFIED is treated as STU3 to accommodate the existing FHIR stores.
    ///
    /// "VERSION_UNSPECIFIED"
    #[serde(rename="VERSION_UNSPECIFIED")]
    VERSIONUNSPECIFIED,
    

    /// Draft Standard for Trial Use, [Release 2](https://www.hl7.org/fhir/DSTU2)
    ///
    /// "DSTU2"
    #[serde(rename="DSTU2")]
    DSTU2,
    

    /// Standard for Trial Use, [Release 3](https://www.hl7.org/fhir/STU3)
    ///
    /// "STU3"
    #[serde(rename="STU3")]
    STU3,
    

    /// [Release 4](https://www.hl7.org/fhir/R4)
    ///
    /// "R4"
    #[serde(rename="R4")]
    R4,
}

impl AsRef<str> for FhirStoreVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FhirStoreVersionEnum::VERSIONUNSPECIFIED => "VERSION_UNSPECIFIED",
            FhirStoreVersionEnum::DSTU2 => "DSTU2",
            FhirStoreVersionEnum::STU3 => "STU3",
            FhirStoreVersionEnum::R4 => "R4",
        }
    }
}

impl std::convert::TryFrom< &str> for FhirStoreVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERSION_UNSPECIFIED" => Ok(FhirStoreVersionEnum::VERSIONUNSPECIFIED),
           "DSTU2" => Ok(FhirStoreVersionEnum::DSTU2),
           "STU3" => Ok(FhirStoreVersionEnum::STU3),
           "R4" => Ok(FhirStoreVersionEnum::R4),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FhirStoreVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FieldMetadataActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deidentify action for one field.
pub enum FieldMetadataActionEnum {
    

    /// No action specified.
    ///
    /// "ACTION_UNSPECIFIED"
    #[serde(rename="ACTION_UNSPECIFIED")]
    ACTIONUNSPECIFIED,
    

    /// Transform the entire field based on transformations specified in TextConfig. When the specified transformation cannot be applied to a field, RedactConfig is used. For example, a Crypto Hash transformation can't be applied to a FHIR Date field.
    ///
    /// "TRANSFORM"
    #[serde(rename="TRANSFORM")]
    TRANSFORM,
    

    /// Inspect and transform any found PHI. When `AnnotationConfig` is provided, annotations of PHI will be generated, except for Date and Datetime.
    ///
    /// "INSPECT_AND_TRANSFORM"
    #[serde(rename="INSPECT_AND_TRANSFORM")]
    INSPECTANDTRANSFORM,
    

    /// Do not transform.
    ///
    /// "DO_NOT_TRANSFORM"
    #[serde(rename="DO_NOT_TRANSFORM")]
    DONOTTRANSFORM,
}

impl AsRef<str> for FieldMetadataActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FieldMetadataActionEnum::ACTIONUNSPECIFIED => "ACTION_UNSPECIFIED",
            FieldMetadataActionEnum::TRANSFORM => "TRANSFORM",
            FieldMetadataActionEnum::INSPECTANDTRANSFORM => "INSPECT_AND_TRANSFORM",
            FieldMetadataActionEnum::DONOTTRANSFORM => "DO_NOT_TRANSFORM",
        }
    }
}

impl std::convert::TryFrom< &str> for FieldMetadataActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_UNSPECIFIED" => Ok(FieldMetadataActionEnum::ACTIONUNSPECIFIED),
           "TRANSFORM" => Ok(FieldMetadataActionEnum::TRANSFORM),
           "INSPECT_AND_TRANSFORM" => Ok(FieldMetadataActionEnum::INSPECTANDTRANSFORM),
           "DO_NOT_TRANSFORM" => Ok(FieldMetadataActionEnum::DONOTTRANSFORM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FieldMetadataActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GcsDestinationContentStructureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The format of the exported HL7v2 message files.
pub enum GcsDestinationContentStructureEnum {
    

    /// If the content structure is not specified, the default value `MESSAGE_JSON` will be used.
    ///
    /// "CONTENT_STRUCTURE_UNSPECIFIED"
    #[serde(rename="CONTENT_STRUCTURE_UNSPECIFIED")]
    CONTENTSTRUCTUREUNSPECIFIED,
    

    /// Messages are printed using the JSON format returned from the `GetMessage` API. Messages are delimited with newlines.
    ///
    /// "MESSAGE_JSON"
    #[serde(rename="MESSAGE_JSON")]
    MESSAGEJSON,
}

impl AsRef<str> for GcsDestinationContentStructureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GcsDestinationContentStructureEnum::CONTENTSTRUCTUREUNSPECIFIED => "CONTENT_STRUCTURE_UNSPECIFIED",
            GcsDestinationContentStructureEnum::MESSAGEJSON => "MESSAGE_JSON",
        }
    }
}

impl std::convert::TryFrom< &str> for GcsDestinationContentStructureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_STRUCTURE_UNSPECIFIED" => Ok(GcsDestinationContentStructureEnum::CONTENTSTRUCTUREUNSPECIFIED),
           "MESSAGE_JSON" => Ok(GcsDestinationContentStructureEnum::MESSAGEJSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GcsDestinationContentStructureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GcsDestinationMessageViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the parts of the Message resource to include in the export. If not specified, FULL is used.
pub enum GcsDestinationMessageViewEnum {
    

    /// Not specified, equivalent to FULL for getMessage, equivalent to BASIC for listMessages.
    ///
    /// "MESSAGE_VIEW_UNSPECIFIED"
    #[serde(rename="MESSAGE_VIEW_UNSPECIFIED")]
    MESSAGEVIEWUNSPECIFIED,
    

    /// Server responses include all the message fields except parsed_data, and schematized_data fields.
    ///
    /// "RAW_ONLY"
    #[serde(rename="RAW_ONLY")]
    RAWONLY,
    

    /// Server responses include all the message fields except data and schematized_data fields.
    ///
    /// "PARSED_ONLY"
    #[serde(rename="PARSED_ONLY")]
    PARSEDONLY,
    

    /// Server responses include all the message fields.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Server responses include all the message fields except data and parsed_data fields.
    ///
    /// "SCHEMATIZED_ONLY"
    #[serde(rename="SCHEMATIZED_ONLY")]
    SCHEMATIZEDONLY,
    

    /// Server responses include only the name field.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
}

impl AsRef<str> for GcsDestinationMessageViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GcsDestinationMessageViewEnum::MESSAGEVIEWUNSPECIFIED => "MESSAGE_VIEW_UNSPECIFIED",
            GcsDestinationMessageViewEnum::RAWONLY => "RAW_ONLY",
            GcsDestinationMessageViewEnum::PARSEDONLY => "PARSED_ONLY",
            GcsDestinationMessageViewEnum::FULL => "FULL",
            GcsDestinationMessageViewEnum::SCHEMATIZEDONLY => "SCHEMATIZED_ONLY",
            GcsDestinationMessageViewEnum::BASIC => "BASIC",
        }
    }
}

impl std::convert::TryFrom< &str> for GcsDestinationMessageViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MESSAGE_VIEW_UNSPECIFIED" => Ok(GcsDestinationMessageViewEnum::MESSAGEVIEWUNSPECIFIED),
           "RAW_ONLY" => Ok(GcsDestinationMessageViewEnum::RAWONLY),
           "PARSED_ONLY" => Ok(GcsDestinationMessageViewEnum::PARSEDONLY),
           "FULL" => Ok(GcsDestinationMessageViewEnum::FULL),
           "SCHEMATIZED_ONLY" => Ok(GcsDestinationMessageViewEnum::SCHEMATIZEDONLY),
           "BASIC" => Ok(GcsDestinationMessageViewEnum::BASIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GcsDestinationMessageViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationSchemaTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the schema format to export.
pub enum GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationSchemaTypeEnum {
    

    /// Same as SIMPLE.
    ///
    /// "SCHEMA_TYPE_UNSPECIFIED"
    #[serde(rename="SCHEMA_TYPE_UNSPECIFIED")]
    SCHEMATYPEUNSPECIFIED,
    

    /// A flatterned version of Annotation.
    ///
    /// "SIMPLE"
    #[serde(rename="SIMPLE")]
    SIMPLE,
}

impl AsRef<str> for GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationSchemaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationSchemaTypeEnum::SCHEMATYPEUNSPECIFIED => "SCHEMA_TYPE_UNSPECIFIED",
            GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationSchemaTypeEnum::SIMPLE => "SIMPLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationSchemaTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCHEMA_TYPE_UNSPECIFIED" => Ok(GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationSchemaTypeEnum::SCHEMATYPEUNSPECIFIED),
           "SIMPLE" => Ok(GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationSchemaTypeEnum::SIMPLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationSchemaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines if existing data in the destination dataset is overwritten, appended to, or not written if the tables contain data. If a write_disposition is specified, the `force` parameter is ignored.
pub enum GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum {
    

    /// Default behavior is the same as WRITE_EMPTY.
    ///
    /// "WRITE_DISPOSITION_UNSPECIFIED"
    #[serde(rename="WRITE_DISPOSITION_UNSPECIFIED")]
    WRITEDISPOSITIONUNSPECIFIED,
    

    /// Only export data if the destination table is empty.
    ///
    /// "WRITE_EMPTY"
    #[serde(rename="WRITE_EMPTY")]
    WRITEEMPTY,
    

    /// Erase all existing data in a table before writing the instances.
    ///
    /// "WRITE_TRUNCATE"
    #[serde(rename="WRITE_TRUNCATE")]
    WRITETRUNCATE,
    

    /// Append data to the existing table.
    ///
    /// "WRITE_APPEND"
    #[serde(rename="WRITE_APPEND")]
    WRITEAPPEND,
}

impl AsRef<str> for GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED => "WRITE_DISPOSITION_UNSPECIFIED",
            GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum::WRITEEMPTY => "WRITE_EMPTY",
            GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum::WRITETRUNCATE => "WRITE_TRUNCATE",
            GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum::WRITEAPPEND => "WRITE_APPEND",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WRITE_DISPOSITION_UNSPECIFIED" => Ok(GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED),
           "WRITE_EMPTY" => Ok(GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum::WRITEEMPTY),
           "WRITE_TRUNCATE" => Ok(GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum::WRITETRUNCATE),
           "WRITE_APPEND" => Ok(GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum::WRITEAPPEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudHealthcareV1beta1AnnotationBigQueryDestinationWriteDispositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines whether the existing table in the destination is to be overwritten or appended to. If a write_disposition is specified, the `force` parameter is ignored.
pub enum GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum {
    

    /// Default behavior is the same as WRITE_EMPTY.
    ///
    /// "WRITE_DISPOSITION_UNSPECIFIED"
    #[serde(rename="WRITE_DISPOSITION_UNSPECIFIED")]
    WRITEDISPOSITIONUNSPECIFIED,
    

    /// Only export data if the destination table is empty.
    ///
    /// "WRITE_EMPTY"
    #[serde(rename="WRITE_EMPTY")]
    WRITEEMPTY,
    

    /// Erase all existing data in the destination table before writing the instances.
    ///
    /// "WRITE_TRUNCATE"
    #[serde(rename="WRITE_TRUNCATE")]
    WRITETRUNCATE,
    

    /// Append data to the destination table.
    ///
    /// "WRITE_APPEND"
    #[serde(rename="WRITE_APPEND")]
    WRITEAPPEND,
}

impl AsRef<str> for GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED => "WRITE_DISPOSITION_UNSPECIFIED",
            GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum::WRITEEMPTY => "WRITE_EMPTY",
            GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum::WRITETRUNCATE => "WRITE_TRUNCATE",
            GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum::WRITEAPPEND => "WRITE_APPEND",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WRITE_DISPOSITION_UNSPECIFIED" => Ok(GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED),
           "WRITE_EMPTY" => Ok(GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum::WRITEEMPTY),
           "WRITE_TRUNCATE" => Ok(GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum::WRITETRUNCATE),
           "WRITE_APPEND" => Ok(GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum::WRITEAPPEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudHealthcareV1beta1DicomBigQueryDestinationWriteDispositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines if existing data in the destination dataset is overwritten, appended to, or not written if the tables contain data. If a write_disposition is specified, the `force` parameter is ignored.
pub enum GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum {
    

    /// Default behavior is the same as WRITE_EMPTY.
    ///
    /// "WRITE_DISPOSITION_UNSPECIFIED"
    #[serde(rename="WRITE_DISPOSITION_UNSPECIFIED")]
    WRITEDISPOSITIONUNSPECIFIED,
    

    /// Only export data if the destination tables are empty.
    ///
    /// "WRITE_EMPTY"
    #[serde(rename="WRITE_EMPTY")]
    WRITEEMPTY,
    

    /// Erase all existing data in the destination tables before writing the FHIR resources.
    ///
    /// "WRITE_TRUNCATE"
    #[serde(rename="WRITE_TRUNCATE")]
    WRITETRUNCATE,
    

    /// Append data to the destination tables.
    ///
    /// "WRITE_APPEND"
    #[serde(rename="WRITE_APPEND")]
    WRITEAPPEND,
}

impl AsRef<str> for GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED => "WRITE_DISPOSITION_UNSPECIFIED",
            GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum::WRITEEMPTY => "WRITE_EMPTY",
            GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum::WRITETRUNCATE => "WRITE_TRUNCATE",
            GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum::WRITEAPPEND => "WRITE_APPEND",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WRITE_DISPOSITION_UNSPECIFIED" => Ok(GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED),
           "WRITE_EMPTY" => Ok(GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum::WRITEEMPTY),
           "WRITE_TRUNCATE" => Ok(GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum::WRITETRUNCATE),
           "WRITE_APPEND" => Ok(GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum::WRITEAPPEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudHealthcareV1beta1FhirBigQueryDestinationWriteDispositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImageConfigTextRedactionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how to redact text from image.
pub enum ImageConfigTextRedactionModeEnum {
    

    /// No text redaction specified. Same as REDACT_NO_TEXT.
    ///
    /// "TEXT_REDACTION_MODE_UNSPECIFIED"
    #[serde(rename="TEXT_REDACTION_MODE_UNSPECIFIED")]
    TEXTREDACTIONMODEUNSPECIFIED,
    

    /// Redact all text.
    ///
    /// "REDACT_ALL_TEXT"
    #[serde(rename="REDACT_ALL_TEXT")]
    REDACTALLTEXT,
    

    /// Redact sensitive text. Uses the set of [Default DICOM InfoTypes](https://cloud.google.com/healthcare-api/docs/how-tos/dicom-deidentify#default_dicom_infotypes).
    ///
    /// "REDACT_SENSITIVE_TEXT"
    #[serde(rename="REDACT_SENSITIVE_TEXT")]
    REDACTSENSITIVETEXT,
    

    /// Do not redact text.
    ///
    /// "REDACT_NO_TEXT"
    #[serde(rename="REDACT_NO_TEXT")]
    REDACTNOTEXT,
    

    /// This mode is like `REDACT_SENSITIVE_TEXT` with the addition of the [Clean Descriptors Option] (https://dicom.nema.org/medical/dicom/2018e/output/chtml/part15/sect_E.3.5.html) enabled: When cleaning text, the process attempts to transform phrases matching any of the tags marked for removal (action codes D, Z, X, and U) in the [Basic Profile] (https://dicom.nema.org/medical/dicom/2018e/output/chtml/part15/chapter_E.html). These contextual phrases are replaced with the token "[CTX]". This mode uses an additional InfoType during inspection.
    ///
    /// "REDACT_SENSITIVE_TEXT_CLEAN_DESCRIPTORS"
    #[serde(rename="REDACT_SENSITIVE_TEXT_CLEAN_DESCRIPTORS")]
    REDACTSENSITIVETEXTCLEANDESCRIPTORS,
}

impl AsRef<str> for ImageConfigTextRedactionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImageConfigTextRedactionModeEnum::TEXTREDACTIONMODEUNSPECIFIED => "TEXT_REDACTION_MODE_UNSPECIFIED",
            ImageConfigTextRedactionModeEnum::REDACTALLTEXT => "REDACT_ALL_TEXT",
            ImageConfigTextRedactionModeEnum::REDACTSENSITIVETEXT => "REDACT_SENSITIVE_TEXT",
            ImageConfigTextRedactionModeEnum::REDACTNOTEXT => "REDACT_NO_TEXT",
            ImageConfigTextRedactionModeEnum::REDACTSENSITIVETEXTCLEANDESCRIPTORS => "REDACT_SENSITIVE_TEXT_CLEAN_DESCRIPTORS",
        }
    }
}

impl std::convert::TryFrom< &str> for ImageConfigTextRedactionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TEXT_REDACTION_MODE_UNSPECIFIED" => Ok(ImageConfigTextRedactionModeEnum::TEXTREDACTIONMODEUNSPECIFIED),
           "REDACT_ALL_TEXT" => Ok(ImageConfigTextRedactionModeEnum::REDACTALLTEXT),
           "REDACT_SENSITIVE_TEXT" => Ok(ImageConfigTextRedactionModeEnum::REDACTSENSITIVETEXT),
           "REDACT_NO_TEXT" => Ok(ImageConfigTextRedactionModeEnum::REDACTNOTEXT),
           "REDACT_SENSITIVE_TEXT_CLEAN_DESCRIPTORS" => Ok(ImageConfigTextRedactionModeEnum::REDACTSENSITIVETEXTCLEANDESCRIPTORS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImageConfigTextRedactionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImportResourcesRequestContentStructureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The content structure in the source location. If not specified, the server treats the input source files as BUNDLE.
pub enum ImportResourcesRequestContentStructureEnum {
    

    /// If the content structure is not specified, the default value `BUNDLE` is used.
    ///
    /// "CONTENT_STRUCTURE_UNSPECIFIED"
    #[serde(rename="CONTENT_STRUCTURE_UNSPECIFIED")]
    CONTENTSTRUCTUREUNSPECIFIED,
    

    /// The source file contains one or more lines of newline-delimited JSON (ndjson). Each line is a bundle that contains one or more resources.
    ///
    /// "BUNDLE"
    #[serde(rename="BUNDLE")]
    BUNDLE,
    

    /// The source file contains one or more lines of newline-delimited JSON (ndjson). Each line is a single resource.
    ///
    /// "RESOURCE"
    #[serde(rename="RESOURCE")]
    RESOURCE,
    

    /// The entire file is one JSON bundle. The JSON can span multiple lines.
    ///
    /// "BUNDLE_PRETTY"
    #[serde(rename="BUNDLE_PRETTY")]
    BUNDLEPRETTY,
    

    /// The entire file is one JSON resource. The JSON can span multiple lines.
    ///
    /// "RESOURCE_PRETTY"
    #[serde(rename="RESOURCE_PRETTY")]
    RESOURCEPRETTY,
}

impl AsRef<str> for ImportResourcesRequestContentStructureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImportResourcesRequestContentStructureEnum::CONTENTSTRUCTUREUNSPECIFIED => "CONTENT_STRUCTURE_UNSPECIFIED",
            ImportResourcesRequestContentStructureEnum::BUNDLE => "BUNDLE",
            ImportResourcesRequestContentStructureEnum::RESOURCE => "RESOURCE",
            ImportResourcesRequestContentStructureEnum::BUNDLEPRETTY => "BUNDLE_PRETTY",
            ImportResourcesRequestContentStructureEnum::RESOURCEPRETTY => "RESOURCE_PRETTY",
        }
    }
}

impl std::convert::TryFrom< &str> for ImportResourcesRequestContentStructureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_STRUCTURE_UNSPECIFIED" => Ok(ImportResourcesRequestContentStructureEnum::CONTENTSTRUCTUREUNSPECIFIED),
           "BUNDLE" => Ok(ImportResourcesRequestContentStructureEnum::BUNDLE),
           "RESOURCE" => Ok(ImportResourcesRequestContentStructureEnum::RESOURCE),
           "BUNDLE_PRETTY" => Ok(ImportResourcesRequestContentStructureEnum::BUNDLEPRETTY),
           "RESOURCE_PRETTY" => Ok(ImportResourcesRequestContentStructureEnum::RESOURCEPRETTY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImportResourcesRequestContentStructureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OptionPrimaryIdsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Set `Action` for [`StudyInstanceUID`, `SeriesInstanceUID`, `SOPInstanceUID`, and `MediaStorageSOPInstanceUID`](http://dicom.nema.org/medical/dicom/2018e/output/chtml/part06/chapter_6.html).
pub enum OptionPrimaryIdsEnum {
    

    /// No value provided. Default to the behavior specified by the base profile.
    ///
    /// "PRIMARY_IDS_OPTION_UNSPECIFIED"
    #[serde(rename="PRIMARY_IDS_OPTION_UNSPECIFIED")]
    PRIMARYIDSOPTIONUNSPECIFIED,
    

    /// Keep primary IDs.
    ///
    /// "KEEP"
    #[serde(rename="KEEP")]
    KEEP,
    

    /// Regenerate primary IDs.
    ///
    /// "REGEN"
    #[serde(rename="REGEN")]
    REGEN,
}

impl AsRef<str> for OptionPrimaryIdsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OptionPrimaryIdsEnum::PRIMARYIDSOPTIONUNSPECIFIED => "PRIMARY_IDS_OPTION_UNSPECIFIED",
            OptionPrimaryIdsEnum::KEEP => "KEEP",
            OptionPrimaryIdsEnum::REGEN => "REGEN",
        }
    }
}

impl std::convert::TryFrom< &str> for OptionPrimaryIdsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIMARY_IDS_OPTION_UNSPECIFIED" => Ok(OptionPrimaryIdsEnum::PRIMARYIDSOPTIONUNSPECIFIED),
           "KEEP" => Ok(OptionPrimaryIdsEnum::KEEP),
           "REGEN" => Ok(OptionPrimaryIdsEnum::REGEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OptionPrimaryIdsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParserConfigVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. Determines the version of both the default parser to be used when `schema` is not given, as well as the schematized parser used when `schema` is specified. This field is immutable after HL7v2 store creation.
pub enum ParserConfigVersionEnum {
    

    /// Unspecified parser version, equivalent to V1.
    ///
    /// "PARSER_VERSION_UNSPECIFIED"
    #[serde(rename="PARSER_VERSION_UNSPECIFIED")]
    PARSERVERSIONUNSPECIFIED,
    

    /// The `parsed_data` includes every given non-empty message field except the Field Separator (MSH-1) field. As a result, the parsed MSH segment starts with the MSH-2 field and the field numbers are off-by-one with respect to the HL7 standard.
    ///
    /// "V1"
    #[serde(rename="V1")]
    V1,
    

    /// The `parsed_data` includes every given non-empty message field.
    ///
    /// "V2"
    #[serde(rename="V2")]
    V2,
    

    /// This version is the same as V2, with the following change. The `parsed_data` contains unescaped escaped field separators, component separators, sub-component separators, repetition separators, escape characters, and truncation characters. If `schema` is specified, the schematized parser uses improved parsing heuristics compared to previous versions.
    ///
    /// "V3"
    #[serde(rename="V3")]
    V3,
}

impl AsRef<str> for ParserConfigVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParserConfigVersionEnum::PARSERVERSIONUNSPECIFIED => "PARSER_VERSION_UNSPECIFIED",
            ParserConfigVersionEnum::V1 => "V1",
            ParserConfigVersionEnum::V2 => "V2",
            ParserConfigVersionEnum::V3 => "V3",
        }
    }
}

impl std::convert::TryFrom< &str> for ParserConfigVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARSER_VERSION_UNSPECIFIED" => Ok(ParserConfigVersionEnum::PARSERVERSIONUNSPECIFIED),
           "V1" => Ok(ParserConfigVersionEnum::V1),
           "V2" => Ok(ParserConfigVersionEnum::V2),
           "V3" => Ok(ParserConfigVersionEnum::V3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParserConfigVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RollbackFhirResourcesRequestChangeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. CREATE/UPDATE/DELETE/ALL for reverting all txns of a certain type.
pub enum RollbackFhirResourcesRequestChangeTypeEnum {
    

    /// When unspecified, revert all transactions
    ///
    /// "CHANGE_TYPE_UNSPECIFIED"
    #[serde(rename="CHANGE_TYPE_UNSPECIFIED")]
    CHANGETYPEUNSPECIFIED,
    

    /// All transactions
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// Revert only CREATE transactions
    ///
    /// "CREATE"
    #[serde(rename="CREATE")]
    CREATE,
    

    /// Revert only Update transactions
    ///
    /// "UPDATE"
    #[serde(rename="UPDATE")]
    UPDATE,
    

    /// Revert only Delete transactions
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
}

impl AsRef<str> for RollbackFhirResourcesRequestChangeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RollbackFhirResourcesRequestChangeTypeEnum::CHANGETYPEUNSPECIFIED => "CHANGE_TYPE_UNSPECIFIED",
            RollbackFhirResourcesRequestChangeTypeEnum::ALL => "ALL",
            RollbackFhirResourcesRequestChangeTypeEnum::CREATE => "CREATE",
            RollbackFhirResourcesRequestChangeTypeEnum::UPDATE => "UPDATE",
            RollbackFhirResourcesRequestChangeTypeEnum::DELETE => "DELETE",
        }
    }
}

impl std::convert::TryFrom< &str> for RollbackFhirResourcesRequestChangeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHANGE_TYPE_UNSPECIFIED" => Ok(RollbackFhirResourcesRequestChangeTypeEnum::CHANGETYPEUNSPECIFIED),
           "ALL" => Ok(RollbackFhirResourcesRequestChangeTypeEnum::ALL),
           "CREATE" => Ok(RollbackFhirResourcesRequestChangeTypeEnum::CREATE),
           "UPDATE" => Ok(RollbackFhirResourcesRequestChangeTypeEnum::UPDATE),
           "DELETE" => Ok(RollbackFhirResourcesRequestChangeTypeEnum::DELETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RollbackFhirResourcesRequestChangeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SchemaConfigSchemaTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the output schema type. Schema type is required.
pub enum SchemaConfigSchemaTypeEnum {
    

    /// No schema type specified. This type is unsupported.
    ///
    /// "SCHEMA_TYPE_UNSPECIFIED"
    #[serde(rename="SCHEMA_TYPE_UNSPECIFIED")]
    SCHEMATYPEUNSPECIFIED,
    

    /// A data-driven schema generated from the fields present in the FHIR data being exported, with no additional simplification. This type cannot be used for streaming to BigQuery.
    ///
    /// "LOSSLESS"
    #[serde(rename="LOSSLESS")]
    LOSSLESS,
    

    /// Analytics schema defined by the FHIR community. See https://github.com/FHIR/sql-on-fhir/blob/master/sql-on-fhir.md. BigQuery only allows a maximum of 10,000 columns per table. Due to this limitation, the server will not generate schemas for fields of type `Resource`, which can hold any resource type. The affected fields are `Parameters.parameter.resource`, `Bundle.entry.resource`, and `Bundle.entry.response.outcome`. Analytics schema does not gracefully handle extensions with one or more occurrences, anaytics schema also does not handle contained resource.
    ///
    /// "ANALYTICS"
    #[serde(rename="ANALYTICS")]
    ANALYTICS,
    

    /// Analytics V2, similar to schema defined by the FHIR community, with added support for extensions with one or more occurrences and contained resources in stringified JSON. Analytics V2 uses more space in the destination table than Analytics V1. It is generally recommended to use Analytics V2 over Analytics.
    ///
    /// "ANALYTICS_V2"
    #[serde(rename="ANALYTICS_V2")]
    ANALYTICSV2,
}

impl AsRef<str> for SchemaConfigSchemaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SchemaConfigSchemaTypeEnum::SCHEMATYPEUNSPECIFIED => "SCHEMA_TYPE_UNSPECIFIED",
            SchemaConfigSchemaTypeEnum::LOSSLESS => "LOSSLESS",
            SchemaConfigSchemaTypeEnum::ANALYTICS => "ANALYTICS",
            SchemaConfigSchemaTypeEnum::ANALYTICSV2 => "ANALYTICS_V2",
        }
    }
}

impl std::convert::TryFrom< &str> for SchemaConfigSchemaTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCHEMA_TYPE_UNSPECIFIED" => Ok(SchemaConfigSchemaTypeEnum::SCHEMATYPEUNSPECIFIED),
           "LOSSLESS" => Ok(SchemaConfigSchemaTypeEnum::LOSSLESS),
           "ANALYTICS" => Ok(SchemaConfigSchemaTypeEnum::ANALYTICS),
           "ANALYTICS_V2" => Ok(SchemaConfigSchemaTypeEnum::ANALYTICSV2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SchemaConfigSchemaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SchemaPackageSchematizedParsingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how messages that fail to parse are handled.
pub enum SchemaPackageSchematizedParsingTypeEnum {
    

    /// Unspecified schematized parsing type, equivalent to `SOFT_FAIL`.
    ///
    /// "SCHEMATIZED_PARSING_TYPE_UNSPECIFIED"
    #[serde(rename="SCHEMATIZED_PARSING_TYPE_UNSPECIFIED")]
    SCHEMATIZEDPARSINGTYPEUNSPECIFIED,
    

    /// Messages that fail to parse are still stored and ACKed but a parser error is stored in place of the schematized data.
    ///
    /// "SOFT_FAIL"
    #[serde(rename="SOFT_FAIL")]
    SOFTFAIL,
    

    /// Messages that fail to parse are rejected from ingestion/insertion and return an error code.
    ///
    /// "HARD_FAIL"
    #[serde(rename="HARD_FAIL")]
    HARDFAIL,
}

impl AsRef<str> for SchemaPackageSchematizedParsingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SchemaPackageSchematizedParsingTypeEnum::SCHEMATIZEDPARSINGTYPEUNSPECIFIED => "SCHEMATIZED_PARSING_TYPE_UNSPECIFIED",
            SchemaPackageSchematizedParsingTypeEnum::SOFTFAIL => "SOFT_FAIL",
            SchemaPackageSchematizedParsingTypeEnum::HARDFAIL => "HARD_FAIL",
        }
    }
}

impl std::convert::TryFrom< &str> for SchemaPackageSchematizedParsingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCHEMATIZED_PARSING_TYPE_UNSPECIFIED" => Ok(SchemaPackageSchematizedParsingTypeEnum::SCHEMATIZEDPARSINGTYPEUNSPECIFIED),
           "SOFT_FAIL" => Ok(SchemaPackageSchematizedParsingTypeEnum::SOFTFAIL),
           "HARD_FAIL" => Ok(SchemaPackageSchematizedParsingTypeEnum::HARDFAIL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SchemaPackageSchematizedParsingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SchemaPackageUnexpectedSegmentHandlingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how unexpected segments (segments not matched to the schema) are handled.
pub enum SchemaPackageUnexpectedSegmentHandlingEnum {
    

    /// Unspecified handling mode, equivalent to FAIL.
    ///
    /// "UNEXPECTED_SEGMENT_HANDLING_MODE_UNSPECIFIED"
    #[serde(rename="UNEXPECTED_SEGMENT_HANDLING_MODE_UNSPECIFIED")]
    UNEXPECTEDSEGMENTHANDLINGMODEUNSPECIFIED,
    

    /// Unexpected segments fail to parse and return an error.
    ///
    /// "FAIL"
    #[serde(rename="FAIL")]
    FAIL,
    

    /// Unexpected segments do not fail, but are omitted from the output.
    ///
    /// "SKIP"
    #[serde(rename="SKIP")]
    SKIP,
    

    /// Unexpected segments do not fail, but are parsed in place and added to the current group. If a segment has a type definition, it is used, otherwise it is parsed as VARIES.
    ///
    /// "PARSE"
    #[serde(rename="PARSE")]
    PARSE,
}

impl AsRef<str> for SchemaPackageUnexpectedSegmentHandlingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SchemaPackageUnexpectedSegmentHandlingEnum::UNEXPECTEDSEGMENTHANDLINGMODEUNSPECIFIED => "UNEXPECTED_SEGMENT_HANDLING_MODE_UNSPECIFIED",
            SchemaPackageUnexpectedSegmentHandlingEnum::FAIL => "FAIL",
            SchemaPackageUnexpectedSegmentHandlingEnum::SKIP => "SKIP",
            SchemaPackageUnexpectedSegmentHandlingEnum::PARSE => "PARSE",
        }
    }
}

impl std::convert::TryFrom< &str> for SchemaPackageUnexpectedSegmentHandlingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNEXPECTED_SEGMENT_HANDLING_MODE_UNSPECIFIED" => Ok(SchemaPackageUnexpectedSegmentHandlingEnum::UNEXPECTEDSEGMENTHANDLINGMODEUNSPECIFIED),
           "FAIL" => Ok(SchemaPackageUnexpectedSegmentHandlingEnum::FAIL),
           "SKIP" => Ok(SchemaPackageUnexpectedSegmentHandlingEnum::SKIP),
           "PARSE" => Ok(SchemaPackageUnexpectedSegmentHandlingEnum::PARSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SchemaPackageUnexpectedSegmentHandlingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TextConfigProfileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Base profile type for text transformation.
pub enum TextConfigProfileTypeEnum {
    

    /// No profile provided. Same as BASIC.
    ///
    /// "PROFILE_TYPE_UNSPECIFIED"
    #[serde(rename="PROFILE_TYPE_UNSPECIFIED")]
    PROFILETYPEUNSPECIFIED,
    

    /// Empty profile which does not perform any transformations.
    ///
    /// "EMPTY"
    #[serde(rename="EMPTY")]
    EMPTY,
    

    /// Automatically converts "DATE" infoTypes using a DateShiftConfig, and all other infoTypes using a ReplaceWithInfoTypeConfig.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
}

impl AsRef<str> for TextConfigProfileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TextConfigProfileTypeEnum::PROFILETYPEUNSPECIFIED => "PROFILE_TYPE_UNSPECIFIED",
            TextConfigProfileTypeEnum::EMPTY => "EMPTY",
            TextConfigProfileTypeEnum::BASIC => "BASIC",
        }
    }
}

impl std::convert::TryFrom< &str> for TextConfigProfileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROFILE_TYPE_UNSPECIFIED" => Ok(TextConfigProfileTypeEnum::PROFILETYPEUNSPECIFIED),
           "EMPTY" => Ok(TextConfigProfileTypeEnum::EMPTY),
           "BASIC" => Ok(TextConfigProfileTypeEnum::BASIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TextConfigProfileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TimePartitioningTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of partitioning.
pub enum TimePartitioningTypeEnum {
    

    /// Default unknown time.
    ///
    /// "PARTITION_TYPE_UNSPECIFIED"
    #[serde(rename="PARTITION_TYPE_UNSPECIFIED")]
    PARTITIONTYPEUNSPECIFIED,
    

    /// Data partitioned by hour.
    ///
    /// "HOUR"
    #[serde(rename="HOUR")]
    HOUR,
    

    /// Data partitioned by day.
    ///
    /// "DAY"
    #[serde(rename="DAY")]
    DAY,
    

    /// Data partitioned by month.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// Data partitioned by year.
    ///
    /// "YEAR"
    #[serde(rename="YEAR")]
    YEAR,
}

impl AsRef<str> for TimePartitioningTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TimePartitioningTypeEnum::PARTITIONTYPEUNSPECIFIED => "PARTITION_TYPE_UNSPECIFIED",
            TimePartitioningTypeEnum::HOUR => "HOUR",
            TimePartitioningTypeEnum::DAY => "DAY",
            TimePartitioningTypeEnum::MONTH => "MONTH",
            TimePartitioningTypeEnum::YEAR => "YEAR",
        }
    }
}

impl std::convert::TryFrom< &str> for TimePartitioningTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARTITION_TYPE_UNSPECIFIED" => Ok(TimePartitioningTypeEnum::PARTITIONTYPEUNSPECIFIED),
           "HOUR" => Ok(TimePartitioningTypeEnum::HOUR),
           "DAY" => Ok(TimePartitioningTypeEnum::DAY),
           "MONTH" => Ok(TimePartitioningTypeEnum::MONTH),
           "YEAR" => Ok(TimePartitioningTypeEnum::YEAR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TimePartitioningTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TypePrimitiveEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If this is a primitive type then this field is the type of the primitive For example, STRING. Leave unspecified for composite types.
pub enum TypePrimitiveEnum {
    

    /// Not a primitive.
    ///
    /// "PRIMITIVE_UNSPECIFIED"
    #[serde(rename="PRIMITIVE_UNSPECIFIED")]
    PRIMITIVEUNSPECIFIED,
    

    /// String primitive.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// Element that can have unschematized children.
    ///
    /// "VARIES"
    #[serde(rename="VARIES")]
    VARIES,
    

    /// Like STRING, but all delimiters below this element are ignored.
    ///
    /// "UNESCAPED_STRING"
    #[serde(rename="UNESCAPED_STRING")]
    UNESCAPEDSTRING,
}

impl AsRef<str> for TypePrimitiveEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TypePrimitiveEnum::PRIMITIVEUNSPECIFIED => "PRIMITIVE_UNSPECIFIED",
            TypePrimitiveEnum::STRING => "STRING",
            TypePrimitiveEnum::VARIES => "VARIES",
            TypePrimitiveEnum::UNESCAPEDSTRING => "UNESCAPED_STRING",
        }
    }
}

impl std::convert::TryFrom< &str> for TypePrimitiveEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIMITIVE_UNSPECIFIED" => Ok(TypePrimitiveEnum::PRIMITIVEUNSPECIFIED),
           "STRING" => Ok(TypePrimitiveEnum::STRING),
           "VARIES" => Ok(TypePrimitiveEnum::VARIES),
           "UNESCAPED_STRING" => Ok(TypePrimitiveEnum::UNESCAPEDSTRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TypePrimitiveEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the parts of the Message to return in the response. When unspecified, equivalent to BASIC. Setting this to anything other than BASIC with a `page_size` larger than the default can generate a large response, which impacts the performance of this method.
pub enum ProjectViewEnum {
    

    /// Not specified, equivalent to FULL for getMessage, equivalent to BASIC for listMessages.
    ///
    /// "MESSAGE_VIEW_UNSPECIFIED"
    #[serde(rename="MESSAGE_VIEW_UNSPECIFIED")]
    MESSAGEVIEWUNSPECIFIED,
    

    /// Server responses include all the message fields except parsed_data, and schematized_data fields.
    ///
    /// "RAW_ONLY"
    #[serde(rename="RAW_ONLY")]
    RAWONLY,
    

    /// Server responses include all the message fields except data and schematized_data fields.
    ///
    /// "PARSED_ONLY"
    #[serde(rename="PARSED_ONLY")]
    PARSEDONLY,
    

    /// Server responses include all the message fields.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Server responses include all the message fields except data and parsed_data fields.
    ///
    /// "SCHEMATIZED_ONLY"
    #[serde(rename="SCHEMATIZED_ONLY")]
    SCHEMATIZEDONLY,
    

    /// Server responses include only the name field.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::MESSAGEVIEWUNSPECIFIED => "MESSAGE_VIEW_UNSPECIFIED",
            ProjectViewEnum::RAWONLY => "RAW_ONLY",
            ProjectViewEnum::PARSEDONLY => "PARSED_ONLY",
            ProjectViewEnum::FULL => "FULL",
            ProjectViewEnum::SCHEMATIZEDONLY => "SCHEMATIZED_ONLY",
            ProjectViewEnum::BASIC => "BASIC",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MESSAGE_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::MESSAGEVIEWUNSPECIFIED),
           "RAW_ONLY" => Ok(ProjectViewEnum::RAWONLY),
           "PARSED_ONLY" => Ok(ProjectViewEnum::PARSEDONLY),
           "FULL" => Ok(ProjectViewEnum::FULL),
           "SCHEMATIZED_ONLY" => Ok(ProjectViewEnum::SCHEMATIZEDONLY),
           "BASIC" => Ok(ProjectViewEnum::BASIC),
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


