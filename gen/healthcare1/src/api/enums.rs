use super::*;



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


// region DicomConfigFilterProfileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tag filtering profile that determines which tags to keep/remove.
pub enum DicomConfigFilterProfileEnum {
    

    /// No tag filtration profile provided. Same as KEEP_ALL_PROFILE.
    ///
    /// "TAG_FILTER_PROFILE_UNSPECIFIED"
    #[serde(rename="TAG_FILTER_PROFILE_UNSPECIFIED")]
    TAGFILTERPROFILEUNSPECIFIED,
    

    /// Keep only tags required to produce valid DICOM.
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
    

    /// Inspects within tag contents and replaces sensitive text. The process can be configured using the TextConfig. Applies to all tags with the following Value Representation names: AE, LO, LT, PN, SH, ST, UC, UT, DA, DT, AS
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
    

    /// Users must specify a version on store creation or an error is returned.
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
    

    /// Transform the entire field.
    ///
    /// "TRANSFORM"
    #[serde(rename="TRANSFORM")]
    TRANSFORM,
    

    /// Inspect and transform any found PHI.
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
    

    /// Not specified, equivalent to FULL.
    ///
    /// "MESSAGE_VIEW_UNSPECIFIED"
    #[serde(rename="MESSAGE_VIEW_UNSPECIFIED")]
    MESSAGEVIEWUNSPECIFIED,
    

    /// Server responses include all the message fields except parsed_data field, and schematized_data fields.
    ///
    /// "RAW_ONLY"
    #[serde(rename="RAW_ONLY")]
    RAWONLY,
    

    /// Server responses include all the message fields except data field, and schematized_data fields.
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


// region GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines whether the existing table in the destination is to be overwritten or appended to. If a write_disposition is specified, the `force` parameter is ignored.
pub enum GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum {
    

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

impl AsRef<str> for GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED => "WRITE_DISPOSITION_UNSPECIFIED",
            GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum::WRITEEMPTY => "WRITE_EMPTY",
            GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum::WRITETRUNCATE => "WRITE_TRUNCATE",
            GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum::WRITEAPPEND => "WRITE_APPEND",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WRITE_DISPOSITION_UNSPECIFIED" => Ok(GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED),
           "WRITE_EMPTY" => Ok(GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum::WRITEEMPTY),
           "WRITE_TRUNCATE" => Ok(GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum::WRITETRUNCATE),
           "WRITE_APPEND" => Ok(GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum::WRITEAPPEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines if existing data in the destination dataset is overwritten, appended to, or not written if the tables contain data. If a write_disposition is specified, the `force` parameter is ignored.
pub enum GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum {
    

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

impl AsRef<str> for GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED => "WRITE_DISPOSITION_UNSPECIFIED",
            GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum::WRITEEMPTY => "WRITE_EMPTY",
            GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum::WRITETRUNCATE => "WRITE_TRUNCATE",
            GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum::WRITEAPPEND => "WRITE_APPEND",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WRITE_DISPOSITION_UNSPECIFIED" => Ok(GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED),
           "WRITE_EMPTY" => Ok(GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum::WRITEEMPTY),
           "WRITE_TRUNCATE" => Ok(GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum::WRITETRUNCATE),
           "WRITE_APPEND" => Ok(GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum::WRITEAPPEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum {
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
}

impl AsRef<str> for ImageConfigTextRedactionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImageConfigTextRedactionModeEnum::TEXTREDACTIONMODEUNSPECIFIED => "TEXT_REDACTION_MODE_UNSPECIFIED",
            ImageConfigTextRedactionModeEnum::REDACTALLTEXT => "REDACT_ALL_TEXT",
            ImageConfigTextRedactionModeEnum::REDACTSENSITIVETEXT => "REDACT_SENSITIVE_TEXT",
            ImageConfigTextRedactionModeEnum::REDACTNOTEXT => "REDACT_NO_TEXT",
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
    

    /// Not specified, equivalent to FULL.
    ///
    /// "MESSAGE_VIEW_UNSPECIFIED"
    #[serde(rename="MESSAGE_VIEW_UNSPECIFIED")]
    MESSAGEVIEWUNSPECIFIED,
    

    /// Server responses include all the message fields except parsed_data field, and schematized_data fields.
    ///
    /// "RAW_ONLY"
    #[serde(rename="RAW_ONLY")]
    RAWONLY,
    

    /// Server responses include all the message fields except data field, and schematized_data fields.
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


