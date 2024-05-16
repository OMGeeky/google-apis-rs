use super::*;



// region GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequestAcknowledgeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Acknowledge type of specified violation.
pub enum GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequestAcknowledgeTypeEnum {
    

    /// Acknowledge type unspecified.
    ///
    /// "ACKNOWLEDGE_TYPE_UNSPECIFIED"
    #[serde(rename="ACKNOWLEDGE_TYPE_UNSPECIFIED")]
    ACKNOWLEDGETYPEUNSPECIFIED,
    

    /// Acknowledge only the specific violation.
    ///
    /// "SINGLE_VIOLATION"
    #[serde(rename="SINGLE_VIOLATION")]
    SINGLEVIOLATION,
    

    /// Acknowledge specified orgPolicy violation and also associated resource violations.
    ///
    /// "EXISTING_CHILD_RESOURCE_VIOLATIONS"
    #[serde(rename="EXISTING_CHILD_RESOURCE_VIOLATIONS")]
    EXISTINGCHILDRESOURCEVIOLATIONS,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequestAcknowledgeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequestAcknowledgeTypeEnum::ACKNOWLEDGETYPEUNSPECIFIED => "ACKNOWLEDGE_TYPE_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequestAcknowledgeTypeEnum::SINGLEVIOLATION => "SINGLE_VIOLATION",
            GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequestAcknowledgeTypeEnum::EXISTINGCHILDRESOURCEVIOLATIONS => "EXISTING_CHILD_RESOURCE_VIOLATIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequestAcknowledgeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACKNOWLEDGE_TYPE_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequestAcknowledgeTypeEnum::ACKNOWLEDGETYPEUNSPECIFIED),
           "SINGLE_VIOLATION" => Ok(GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequestAcknowledgeTypeEnum::SINGLEVIOLATION),
           "EXISTING_CHILD_RESOURCE_VIOLATIONS" => Ok(GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequestAcknowledgeTypeEnum::EXISTINGCHILDRESOURCEVIOLATIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequestAcknowledgeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of restriction for using gcp products in the Workload environment.
pub enum GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum {
    

    /// Unknown restriction type.
    ///
    /// "RESTRICTION_TYPE_UNSPECIFIED"
    #[serde(rename="RESTRICTION_TYPE_UNSPECIFIED")]
    RESTRICTIONTYPEUNSPECIFIED,
    

    /// Allow the use all of all gcp products, irrespective of the compliance posture. This effectively removes gcp.restrictServiceUsage OrgPolicy on the AssuredWorkloads Folder.
    ///
    /// "ALLOW_ALL_GCP_RESOURCES"
    #[serde(rename="ALLOW_ALL_GCP_RESOURCES")]
    ALLOWALLGCPRESOURCES,
    

    /// Based on Workload's compliance regime, allowed list changes. See - https://cloud.google.com/assured-workloads/docs/supported-products for the list of supported resources.
    ///
    /// "ALLOW_COMPLIANT_RESOURCES"
    #[serde(rename="ALLOW_COMPLIANT_RESOURCES")]
    ALLOWCOMPLIANTRESOURCES,
    

    /// Similar to ALLOW_COMPLIANT_RESOURCES but adds the list of compliant resources to the existing list of compliant resources. Effective org-policy of the Folder is considered to ensure there is no disruption to the existing customer workflows.
    ///
    /// "APPEND_COMPLIANT_RESOURCES"
    #[serde(rename="APPEND_COMPLIANT_RESOURCES")]
    APPENDCOMPLIANTRESOURCES,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum::RESTRICTIONTYPEUNSPECIFIED => "RESTRICTION_TYPE_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum::ALLOWALLGCPRESOURCES => "ALLOW_ALL_GCP_RESOURCES",
            GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum::ALLOWCOMPLIANTRESOURCES => "ALLOW_COMPLIANT_RESOURCES",
            GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum::APPENDCOMPLIANTRESOURCES => "APPEND_COMPLIANT_RESOURCES",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESTRICTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum::RESTRICTIONTYPEUNSPECIFIED),
           "ALLOW_ALL_GCP_RESOURCES" => Ok(GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum::ALLOWALLGCPRESOURCES),
           "ALLOW_COMPLIANT_RESOURCES" => Ok(GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum::ALLOWCOMPLIANTRESOURCES),
           "APPEND_COMPLIANT_RESOURCES" => Ok(GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum::APPENDCOMPLIANTRESOURCES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1ViolationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the violation
pub enum GoogleCloudAssuredworkloadsV1ViolationStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Violation is resolved.
    ///
    /// "RESOLVED"
    #[serde(rename="RESOLVED")]
    RESOLVED,
    

    /// Violation is Unresolved
    ///
    /// "UNRESOLVED"
    #[serde(rename="UNRESOLVED")]
    UNRESOLVED,
    

    /// Violation is Exception
    ///
    /// "EXCEPTION"
    #[serde(rename="EXCEPTION")]
    EXCEPTION,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1ViolationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1ViolationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1ViolationStateEnum::RESOLVED => "RESOLVED",
            GoogleCloudAssuredworkloadsV1ViolationStateEnum::UNRESOLVED => "UNRESOLVED",
            GoogleCloudAssuredworkloadsV1ViolationStateEnum::EXCEPTION => "EXCEPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1ViolationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1ViolationStateEnum::STATEUNSPECIFIED),
           "RESOLVED" => Ok(GoogleCloudAssuredworkloadsV1ViolationStateEnum::RESOLVED),
           "UNRESOLVED" => Ok(GoogleCloudAssuredworkloadsV1ViolationStateEnum::UNRESOLVED),
           "EXCEPTION" => Ok(GoogleCloudAssuredworkloadsV1ViolationStateEnum::EXCEPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1ViolationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1ViolationViolationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Type of the violation
pub enum GoogleCloudAssuredworkloadsV1ViolationViolationTypeEnum {
    

    /// Unspecified type.
    ///
    /// "VIOLATION_TYPE_UNSPECIFIED"
    #[serde(rename="VIOLATION_TYPE_UNSPECIFIED")]
    VIOLATIONTYPEUNSPECIFIED,
    

    /// Org Policy Violation.
    ///
    /// "ORG_POLICY"
    #[serde(rename="ORG_POLICY")]
    ORGPOLICY,
    

    /// Resource Violation.
    ///
    /// "RESOURCE"
    #[serde(rename="RESOURCE")]
    RESOURCE,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1ViolationViolationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1ViolationViolationTypeEnum::VIOLATIONTYPEUNSPECIFIED => "VIOLATION_TYPE_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1ViolationViolationTypeEnum::ORGPOLICY => "ORG_POLICY",
            GoogleCloudAssuredworkloadsV1ViolationViolationTypeEnum::RESOURCE => "RESOURCE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1ViolationViolationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIOLATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1ViolationViolationTypeEnum::VIOLATIONTYPEUNSPECIFIED),
           "ORG_POLICY" => Ok(GoogleCloudAssuredworkloadsV1ViolationViolationTypeEnum::ORGPOLICY),
           "RESOURCE" => Ok(GoogleCloudAssuredworkloadsV1ViolationViolationTypeEnum::RESOURCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1ViolationViolationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Reemediation type based on the type of org policy values violated
pub enum GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum {
    

    /// Unspecified remediation type
    ///
    /// "REMEDIATION_TYPE_UNSPECIFIED"
    #[serde(rename="REMEDIATION_TYPE_UNSPECIFIED")]
    REMEDIATIONTYPEUNSPECIFIED,
    

    /// Remediation type for boolean org policy
    ///
    /// "REMEDIATION_BOOLEAN_ORG_POLICY_VIOLATION"
    #[serde(rename="REMEDIATION_BOOLEAN_ORG_POLICY_VIOLATION")]
    REMEDIATIONBOOLEANORGPOLICYVIOLATION,
    

    /// Remediation type for list org policy which have allowed values in the monitoring rule
    ///
    /// "REMEDIATION_LIST_ALLOWED_VALUES_ORG_POLICY_VIOLATION"
    #[serde(rename="REMEDIATION_LIST_ALLOWED_VALUES_ORG_POLICY_VIOLATION")]
    REMEDIATIONLISTALLOWEDVALUESORGPOLICYVIOLATION,
    

    /// Remediation type for list org policy which have denied values in the monitoring rule
    ///
    /// "REMEDIATION_LIST_DENIED_VALUES_ORG_POLICY_VIOLATION"
    #[serde(rename="REMEDIATION_LIST_DENIED_VALUES_ORG_POLICY_VIOLATION")]
    REMEDIATIONLISTDENIEDVALUESORGPOLICYVIOLATION,
    

    /// Remediation type for gcp.restrictCmekCryptoKeyProjects
    ///
    /// "REMEDIATION_RESTRICT_CMEK_CRYPTO_KEY_PROJECTS_ORG_POLICY_VIOLATION"
    #[serde(rename="REMEDIATION_RESTRICT_CMEK_CRYPTO_KEY_PROJECTS_ORG_POLICY_VIOLATION")]
    REMEDIATIONRESTRICTCMEKCRYPTOKEYPROJECTSORGPOLICYVIOLATION,
    

    /// Remediation type for resource violation.
    ///
    /// "REMEDIATION_RESOURCE_VIOLATION"
    #[serde(rename="REMEDIATION_RESOURCE_VIOLATION")]
    REMEDIATIONRESOURCEVIOLATION,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONTYPEUNSPECIFIED => "REMEDIATION_TYPE_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONBOOLEANORGPOLICYVIOLATION => "REMEDIATION_BOOLEAN_ORG_POLICY_VIOLATION",
            GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONLISTALLOWEDVALUESORGPOLICYVIOLATION => "REMEDIATION_LIST_ALLOWED_VALUES_ORG_POLICY_VIOLATION",
            GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONLISTDENIEDVALUESORGPOLICYVIOLATION => "REMEDIATION_LIST_DENIED_VALUES_ORG_POLICY_VIOLATION",
            GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONRESTRICTCMEKCRYPTOKEYPROJECTSORGPOLICYVIOLATION => "REMEDIATION_RESTRICT_CMEK_CRYPTO_KEY_PROJECTS_ORG_POLICY_VIOLATION",
            GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONRESOURCEVIOLATION => "REMEDIATION_RESOURCE_VIOLATION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REMEDIATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONTYPEUNSPECIFIED),
           "REMEDIATION_BOOLEAN_ORG_POLICY_VIOLATION" => Ok(GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONBOOLEANORGPOLICYVIOLATION),
           "REMEDIATION_LIST_ALLOWED_VALUES_ORG_POLICY_VIOLATION" => Ok(GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONLISTALLOWEDVALUESORGPOLICYVIOLATION),
           "REMEDIATION_LIST_DENIED_VALUES_ORG_POLICY_VIOLATION" => Ok(GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONLISTDENIEDVALUESORGPOLICYVIOLATION),
           "REMEDIATION_RESTRICT_CMEK_CRYPTO_KEY_PROJECTS_ORG_POLICY_VIOLATION" => Ok(GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONRESTRICTCMEKCRYPTOKEYPROJECTSORGPOLICYVIOLATION),
           "REMEDIATION_RESOURCE_VIOLATION" => Ok(GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum::REMEDIATIONRESOURCEVIOLATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. Compliance Regime associated with this workload.
pub enum GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum {
    

    /// Unknown compliance regime.
    ///
    /// "COMPLIANCE_REGIME_UNSPECIFIED"
    #[serde(rename="COMPLIANCE_REGIME_UNSPECIFIED")]
    COMPLIANCEREGIMEUNSPECIFIED,
    

    /// Information protection as per DoD IL4 requirements.
    ///
    /// "IL4"
    #[serde(rename="IL4")]
    IL4,
    

    /// Criminal Justice Information Services (CJIS) Security policies.
    ///
    /// "CJIS"
    #[serde(rename="CJIS")]
    CJIS,
    

    /// FedRAMP High data protection controls
    ///
    /// "FEDRAMP_HIGH"
    #[serde(rename="FEDRAMP_HIGH")]
    FEDRAMPHIGH,
    

    /// FedRAMP Moderate data protection controls
    ///
    /// "FEDRAMP_MODERATE"
    #[serde(rename="FEDRAMP_MODERATE")]
    FEDRAMPMODERATE,
    

    /// Assured Workloads For US Regions data protection controls
    ///
    /// "US_REGIONAL_ACCESS"
    #[serde(rename="US_REGIONAL_ACCESS")]
    USREGIONALACCESS,
    

    /// Health Insurance Portability and Accountability Act controls
    ///
    /// "HIPAA"
    #[serde(rename="HIPAA")]
    HIPAA,
    

    /// Health Information Trust Alliance controls
    ///
    /// "HITRUST"
    #[serde(rename="HITRUST")]
    HITRUST,
    

    /// Assured Workloads For EU Regions and Support controls
    ///
    /// "EU_REGIONS_AND_SUPPORT"
    #[serde(rename="EU_REGIONS_AND_SUPPORT")]
    EUREGIONSANDSUPPORT,
    

    /// Assured Workloads For Canada Regions and Support controls
    ///
    /// "CA_REGIONS_AND_SUPPORT"
    #[serde(rename="CA_REGIONS_AND_SUPPORT")]
    CAREGIONSANDSUPPORT,
    

    /// International Traffic in Arms Regulations
    ///
    /// "ITAR"
    #[serde(rename="ITAR")]
    ITAR,
    

    /// Assured Workloads for Australia Regions and Support controls
    ///
    /// "AU_REGIONS_AND_US_SUPPORT"
    #[serde(rename="AU_REGIONS_AND_US_SUPPORT")]
    AUREGIONSANDUSSUPPORT,
    

    /// Assured Workloads for Partners;
    ///
    /// "ASSURED_WORKLOADS_FOR_PARTNERS"
    #[serde(rename="ASSURED_WORKLOADS_FOR_PARTNERS")]
    ASSUREDWORKLOADSFORPARTNERS,
    

    /// Assured Workloads for Israel
    ///
    /// "ISR_REGIONS"
    #[serde(rename="ISR_REGIONS")]
    ISRREGIONS,
    

    /// Assured Workloads for Israel Regions
    ///
    /// "ISR_REGIONS_AND_SUPPORT"
    #[serde(rename="ISR_REGIONS_AND_SUPPORT")]
    ISRREGIONSANDSUPPORT,
    

    /// Assured Workloads for Canada Protected B regime
    ///
    /// "CA_PROTECTED_B"
    #[serde(rename="CA_PROTECTED_B")]
    CAPROTECTEDB,
    

    /// Information protection as per DoD IL5 requirements.
    ///
    /// "IL5"
    #[serde(rename="IL5")]
    IL5,
    

    /// Information protection as per DoD IL2 requirements.
    ///
    /// "IL2"
    #[serde(rename="IL2")]
    IL2,
    

    /// Assured Workloads for Japan Regions
    ///
    /// "JP_REGIONS_AND_SUPPORT"
    #[serde(rename="JP_REGIONS_AND_SUPPORT")]
    JPREGIONSANDSUPPORT,
    

    /// KSA R5 Controls.
    ///
    /// "KSA_REGIONS_AND_SUPPORT_WITH_SOVEREIGNTY_CONTROLS"
    #[serde(rename="KSA_REGIONS_AND_SUPPORT_WITH_SOVEREIGNTY_CONTROLS")]
    KSAREGIONSANDSUPPORTWITHSOVEREIGNTYCONTROLS,
    

    /// Assured Workloads Free Regions
    ///
    /// "FREE_REGIONS"
    #[serde(rename="FREE_REGIONS")]
    FREEREGIONS,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::COMPLIANCEREGIMEUNSPECIFIED => "COMPLIANCE_REGIME_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::IL4 => "IL4",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::CJIS => "CJIS",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::FEDRAMPHIGH => "FEDRAMP_HIGH",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::FEDRAMPMODERATE => "FEDRAMP_MODERATE",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::USREGIONALACCESS => "US_REGIONAL_ACCESS",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::HIPAA => "HIPAA",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::HITRUST => "HITRUST",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::EUREGIONSANDSUPPORT => "EU_REGIONS_AND_SUPPORT",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::CAREGIONSANDSUPPORT => "CA_REGIONS_AND_SUPPORT",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::ITAR => "ITAR",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::AUREGIONSANDUSSUPPORT => "AU_REGIONS_AND_US_SUPPORT",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::ASSUREDWORKLOADSFORPARTNERS => "ASSURED_WORKLOADS_FOR_PARTNERS",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::ISRREGIONS => "ISR_REGIONS",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::ISRREGIONSANDSUPPORT => "ISR_REGIONS_AND_SUPPORT",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::CAPROTECTEDB => "CA_PROTECTED_B",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::IL5 => "IL5",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::IL2 => "IL2",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::JPREGIONSANDSUPPORT => "JP_REGIONS_AND_SUPPORT",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::KSAREGIONSANDSUPPORTWITHSOVEREIGNTYCONTROLS => "KSA_REGIONS_AND_SUPPORT_WITH_SOVEREIGNTY_CONTROLS",
            GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::FREEREGIONS => "FREE_REGIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPLIANCE_REGIME_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::COMPLIANCEREGIMEUNSPECIFIED),
           "IL4" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::IL4),
           "CJIS" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::CJIS),
           "FEDRAMP_HIGH" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::FEDRAMPHIGH),
           "FEDRAMP_MODERATE" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::FEDRAMPMODERATE),
           "US_REGIONAL_ACCESS" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::USREGIONALACCESS),
           "HIPAA" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::HIPAA),
           "HITRUST" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::HITRUST),
           "EU_REGIONS_AND_SUPPORT" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::EUREGIONSANDSUPPORT),
           "CA_REGIONS_AND_SUPPORT" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::CAREGIONSANDSUPPORT),
           "ITAR" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::ITAR),
           "AU_REGIONS_AND_US_SUPPORT" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::AUREGIONSANDUSSUPPORT),
           "ASSURED_WORKLOADS_FOR_PARTNERS" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::ASSUREDWORKLOADSFORPARTNERS),
           "ISR_REGIONS" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::ISRREGIONS),
           "ISR_REGIONS_AND_SUPPORT" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::ISRREGIONSANDSUPPORT),
           "CA_PROTECTED_B" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::CAPROTECTEDB),
           "IL5" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::IL5),
           "IL2" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::IL2),
           "JP_REGIONS_AND_SUPPORT" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::JPREGIONSANDSUPPORT),
           "KSA_REGIONS_AND_SUPPORT_WITH_SOVEREIGNTY_CONTROLS" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::KSAREGIONSANDSUPPORTWITHSOVEREIGNTYCONTROLS),
           "FREE_REGIONS" => Ok(GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum::FREEREGIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Represents the KAJ enrollment state of the given workload.
pub enum GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum {
    

    /// Default State for KAJ Enrollment.
    ///
    /// "KAJ_ENROLLMENT_STATE_UNSPECIFIED"
    #[serde(rename="KAJ_ENROLLMENT_STATE_UNSPECIFIED")]
    KAJENROLLMENTSTATEUNSPECIFIED,
    

    /// Pending State for KAJ Enrollment.
    ///
    /// "KAJ_ENROLLMENT_STATE_PENDING"
    #[serde(rename="KAJ_ENROLLMENT_STATE_PENDING")]
    KAJENROLLMENTSTATEPENDING,
    

    /// Complete State for KAJ Enrollment.
    ///
    /// "KAJ_ENROLLMENT_STATE_COMPLETE"
    #[serde(rename="KAJ_ENROLLMENT_STATE_COMPLETE")]
    KAJENROLLMENTSTATECOMPLETE,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum::KAJENROLLMENTSTATEUNSPECIFIED => "KAJ_ENROLLMENT_STATE_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum::KAJENROLLMENTSTATEPENDING => "KAJ_ENROLLMENT_STATE_PENDING",
            GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum::KAJENROLLMENTSTATECOMPLETE => "KAJ_ENROLLMENT_STATE_COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KAJ_ENROLLMENT_STATE_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum::KAJENROLLMENTSTATEUNSPECIFIED),
           "KAJ_ENROLLMENT_STATE_PENDING" => Ok(GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum::KAJENROLLMENTSTATEPENDING),
           "KAJ_ENROLLMENT_STATE_COMPLETE" => Ok(GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum::KAJENROLLMENTSTATECOMPLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Partner regime associated with this workload.
pub enum GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum {
    
    /// "PARTNER_UNSPECIFIED"
    #[serde(rename="PARTNER_UNSPECIFIED")]
    PARTNERUNSPECIFIED,
    

    /// Enum representing S3NS (Thales) partner.
    ///
    /// "LOCAL_CONTROLS_BY_S3NS"
    #[serde(rename="LOCAL_CONTROLS_BY_S3NS")]
    LOCALCONTROLSBYS3NS,
    

    /// Enum representing T_SYSTEM (TSI) partner.
    ///
    /// "SOVEREIGN_CONTROLS_BY_T_SYSTEMS"
    #[serde(rename="SOVEREIGN_CONTROLS_BY_T_SYSTEMS")]
    SOVEREIGNCONTROLSBYTSYSTEMS,
    

    /// Enum representing SIA_MINSAIT (Indra) partner.
    ///
    /// "SOVEREIGN_CONTROLS_BY_SIA_MINSAIT"
    #[serde(rename="SOVEREIGN_CONTROLS_BY_SIA_MINSAIT")]
    SOVEREIGNCONTROLSBYSIAMINSAIT,
    

    /// Enum representing PSN (TIM) partner.
    ///
    /// "SOVEREIGN_CONTROLS_BY_PSN"
    #[serde(rename="SOVEREIGN_CONTROLS_BY_PSN")]
    SOVEREIGNCONTROLSBYPSN,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum::PARTNERUNSPECIFIED => "PARTNER_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum::LOCALCONTROLSBYS3NS => "LOCAL_CONTROLS_BY_S3NS",
            GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum::SOVEREIGNCONTROLSBYTSYSTEMS => "SOVEREIGN_CONTROLS_BY_T_SYSTEMS",
            GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum::SOVEREIGNCONTROLSBYSIAMINSAIT => "SOVEREIGN_CONTROLS_BY_SIA_MINSAIT",
            GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum::SOVEREIGNCONTROLSBYPSN => "SOVEREIGN_CONTROLS_BY_PSN",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARTNER_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum::PARTNERUNSPECIFIED),
           "LOCAL_CONTROLS_BY_S3NS" => Ok(GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum::LOCALCONTROLSBYS3NS),
           "SOVEREIGN_CONTROLS_BY_T_SYSTEMS" => Ok(GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum::SOVEREIGNCONTROLSBYTSYSTEMS),
           "SOVEREIGN_CONTROLS_BY_SIA_MINSAIT" => Ok(GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum::SOVEREIGNCONTROLSBYSIAMINSAIT),
           "SOVEREIGN_CONTROLS_BY_PSN" => Ok(GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum::SOVEREIGNCONTROLSBYPSN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates Ekm provisioning error if any.
pub enum GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum {
    

    /// No error domain
    ///
    /// "EKM_PROVISIONING_ERROR_DOMAIN_UNSPECIFIED"
    #[serde(rename="EKM_PROVISIONING_ERROR_DOMAIN_UNSPECIFIED")]
    EKMPROVISIONINGERRORDOMAINUNSPECIFIED,
    

    /// Error but domain is unspecified.
    ///
    /// "UNSPECIFIED_ERROR"
    #[serde(rename="UNSPECIFIED_ERROR")]
    UNSPECIFIEDERROR,
    

    /// Internal logic breaks within provisioning code.
    ///
    /// "GOOGLE_SERVER_ERROR"
    #[serde(rename="GOOGLE_SERVER_ERROR")]
    GOOGLESERVERERROR,
    

    /// Error occurred with the customer not granting permission/creating resource.
    ///
    /// "EXTERNAL_USER_ERROR"
    #[serde(rename="EXTERNAL_USER_ERROR")]
    EXTERNALUSERERROR,
    

    /// Error occurred within the partner's provisioning cluster.
    ///
    /// "EXTERNAL_PARTNER_ERROR"
    #[serde(rename="EXTERNAL_PARTNER_ERROR")]
    EXTERNALPARTNERERROR,
    

    /// Resource wasn't provisioned in the required 7 day time period
    ///
    /// "TIMEOUT_ERROR"
    #[serde(rename="TIMEOUT_ERROR")]
    TIMEOUTERROR,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::EKMPROVISIONINGERRORDOMAINUNSPECIFIED => "EKM_PROVISIONING_ERROR_DOMAIN_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::UNSPECIFIEDERROR => "UNSPECIFIED_ERROR",
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::GOOGLESERVERERROR => "GOOGLE_SERVER_ERROR",
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::EXTERNALUSERERROR => "EXTERNAL_USER_ERROR",
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::EXTERNALPARTNERERROR => "EXTERNAL_PARTNER_ERROR",
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::TIMEOUTERROR => "TIMEOUT_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EKM_PROVISIONING_ERROR_DOMAIN_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::EKMPROVISIONINGERRORDOMAINUNSPECIFIED),
           "UNSPECIFIED_ERROR" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::UNSPECIFIEDERROR),
           "GOOGLE_SERVER_ERROR" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::GOOGLESERVERERROR),
           "EXTERNAL_USER_ERROR" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::EXTERNALUSERERROR),
           "EXTERNAL_PARTNER_ERROR" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::EXTERNALPARTNERERROR),
           "TIMEOUT_ERROR" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum::TIMEOUTERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorDomainEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Detailed error message if Ekm provisioning fails
pub enum GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum {
    

    /// Error is unspecified.
    ///
    /// "EKM_PROVISIONING_ERROR_MAPPING_UNSPECIFIED"
    #[serde(rename="EKM_PROVISIONING_ERROR_MAPPING_UNSPECIFIED")]
    EKMPROVISIONINGERRORMAPPINGUNSPECIFIED,
    

    /// Service account is used is invalid.
    ///
    /// "INVALID_SERVICE_ACCOUNT"
    #[serde(rename="INVALID_SERVICE_ACCOUNT")]
    INVALIDSERVICEACCOUNT,
    

    /// Iam permission monitoring.MetricsScopeAdmin wasn't applied.
    ///
    /// "MISSING_METRICS_SCOPE_ADMIN_PERMISSION"
    #[serde(rename="MISSING_METRICS_SCOPE_ADMIN_PERMISSION")]
    MISSINGMETRICSSCOPEADMINPERMISSION,
    

    /// Iam permission cloudkms.ekmConnectionsAdmin wasn't applied.
    ///
    /// "MISSING_EKM_CONNECTION_ADMIN_PERMISSION"
    #[serde(rename="MISSING_EKM_CONNECTION_ADMIN_PERMISSION")]
    MISSINGEKMCONNECTIONADMINPERMISSION,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum::EKMPROVISIONINGERRORMAPPINGUNSPECIFIED => "EKM_PROVISIONING_ERROR_MAPPING_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum::INVALIDSERVICEACCOUNT => "INVALID_SERVICE_ACCOUNT",
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum::MISSINGMETRICSSCOPEADMINPERMISSION => "MISSING_METRICS_SCOPE_ADMIN_PERMISSION",
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum::MISSINGEKMCONNECTIONADMINPERMISSION => "MISSING_EKM_CONNECTION_ADMIN_PERMISSION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EKM_PROVISIONING_ERROR_MAPPING_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum::EKMPROVISIONINGERRORMAPPINGUNSPECIFIED),
           "INVALID_SERVICE_ACCOUNT" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum::INVALIDSERVICEACCOUNT),
           "MISSING_METRICS_SCOPE_ADMIN_PERMISSION" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum::MISSINGMETRICSSCOPEADMINPERMISSION),
           "MISSING_EKM_CONNECTION_ADMIN_PERMISSION" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum::MISSINGEKMCONNECTIONADMINPERMISSION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningErrorMappingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates Ekm enrollment Provisioning of a given workload.
pub enum GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum {
    

    /// Default State for Ekm Provisioning
    ///
    /// "EKM_PROVISIONING_STATE_UNSPECIFIED"
    #[serde(rename="EKM_PROVISIONING_STATE_UNSPECIFIED")]
    EKMPROVISIONINGSTATEUNSPECIFIED,
    

    /// Pending State for Ekm Provisioning
    ///
    /// "EKM_PROVISIONING_STATE_PENDING"
    #[serde(rename="EKM_PROVISIONING_STATE_PENDING")]
    EKMPROVISIONINGSTATEPENDING,
    

    /// Failed State for Ekm Provisioning
    ///
    /// "EKM_PROVISIONING_STATE_FAILED"
    #[serde(rename="EKM_PROVISIONING_STATE_FAILED")]
    EKMPROVISIONINGSTATEFAILED,
    

    /// Completed State for Ekm Provisioning
    ///
    /// "EKM_PROVISIONING_STATE_COMPLETED"
    #[serde(rename="EKM_PROVISIONING_STATE_COMPLETED")]
    EKMPROVISIONINGSTATECOMPLETED,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum::EKMPROVISIONINGSTATEUNSPECIFIED => "EKM_PROVISIONING_STATE_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum::EKMPROVISIONINGSTATEPENDING => "EKM_PROVISIONING_STATE_PENDING",
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum::EKMPROVISIONINGSTATEFAILED => "EKM_PROVISIONING_STATE_FAILED",
            GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum::EKMPROVISIONINGSTATECOMPLETED => "EKM_PROVISIONING_STATE_COMPLETED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EKM_PROVISIONING_STATE_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum::EKMPROVISIONINGSTATEUNSPECIFIED),
           "EKM_PROVISIONING_STATE_PENDING" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum::EKMPROVISIONINGSTATEPENDING),
           "EKM_PROVISIONING_STATE_FAILED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum::EKMPROVISIONINGSTATEFAILED),
           "EKM_PROVISIONING_STATE_COMPLETED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum::EKMPROVISIONINGSTATECOMPLETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1WorkloadEkmProvisioningResponseEkmProvisioningStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the type of resource.
pub enum GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum {
    

    /// Unknown resource type.
    ///
    /// "RESOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="RESOURCE_TYPE_UNSPECIFIED")]
    RESOURCETYPEUNSPECIFIED,
    

    /// Deprecated. Existing workloads will continue to support this, but new CreateWorkloadRequests should not specify this as an input value.
    ///
    /// "CONSUMER_PROJECT"
    #[serde(rename="CONSUMER_PROJECT")]
    CONSUMERPROJECT,
    

    /// Consumer Folder.
    ///
    /// "CONSUMER_FOLDER"
    #[serde(rename="CONSUMER_FOLDER")]
    CONSUMERFOLDER,
    

    /// Consumer project containing encryption keys.
    ///
    /// "ENCRYPTION_KEYS_PROJECT"
    #[serde(rename="ENCRYPTION_KEYS_PROJECT")]
    ENCRYPTIONKEYSPROJECT,
    

    /// Keyring resource that hosts encryption keys.
    ///
    /// "KEYRING"
    #[serde(rename="KEYRING")]
    KEYRING,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum::RESOURCETYPEUNSPECIFIED => "RESOURCE_TYPE_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum::CONSUMERPROJECT => "CONSUMER_PROJECT",
            GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum::CONSUMERFOLDER => "CONSUMER_FOLDER",
            GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum::ENCRYPTIONKEYSPROJECT => "ENCRYPTION_KEYS_PROJECT",
            GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum::KEYRING => "KEYRING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_TYPE_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum::RESOURCETYPEUNSPECIFIED),
           "CONSUMER_PROJECT" => Ok(GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum::CONSUMERPROJECT),
           "CONSUMER_FOLDER" => Ok(GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum::CONSUMERFOLDER),
           "ENCRYPTION_KEYS_PROJECT" => Ok(GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum::ENCRYPTIONKEYSPROJECT),
           "KEYRING" => Ok(GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum::KEYRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the type of resource. This field should be specified to correspond the id to the right project type (CONSUMER_PROJECT or ENCRYPTION_KEYS_PROJECT)
pub enum GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum {
    

    /// Unknown resource type.
    ///
    /// "RESOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="RESOURCE_TYPE_UNSPECIFIED")]
    RESOURCETYPEUNSPECIFIED,
    

    /// Deprecated. Existing workloads will continue to support this, but new CreateWorkloadRequests should not specify this as an input value.
    ///
    /// "CONSUMER_PROJECT"
    #[serde(rename="CONSUMER_PROJECT")]
    CONSUMERPROJECT,
    

    /// Consumer Folder.
    ///
    /// "CONSUMER_FOLDER"
    #[serde(rename="CONSUMER_FOLDER")]
    CONSUMERFOLDER,
    

    /// Consumer project containing encryption keys.
    ///
    /// "ENCRYPTION_KEYS_PROJECT"
    #[serde(rename="ENCRYPTION_KEYS_PROJECT")]
    ENCRYPTIONKEYSPROJECT,
    

    /// Keyring resource that hosts encryption keys.
    ///
    /// "KEYRING"
    #[serde(rename="KEYRING")]
    KEYRING,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum::RESOURCETYPEUNSPECIFIED => "RESOURCE_TYPE_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum::CONSUMERPROJECT => "CONSUMER_PROJECT",
            GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum::CONSUMERFOLDER => "CONSUMER_FOLDER",
            GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum::ENCRYPTIONKEYSPROJECT => "ENCRYPTION_KEYS_PROJECT",
            GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum::KEYRING => "KEYRING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_TYPE_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum::RESOURCETYPEUNSPECIFIED),
           "CONSUMER_PROJECT" => Ok(GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum::CONSUMERPROJECT),
           "CONSUMER_FOLDER" => Ok(GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum::CONSUMERFOLDER),
           "ENCRYPTION_KEYS_PROJECT" => Ok(GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum::ENCRYPTIONKEYSPROJECT),
           "KEYRING" => Ok(GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum::KEYRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates SAA enrollment setup error if any.
pub enum GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum {
    

    /// Unspecified.
    ///
    /// "SETUP_ERROR_UNSPECIFIED"
    #[serde(rename="SETUP_ERROR_UNSPECIFIED")]
    SETUPERRORUNSPECIFIED,
    

    /// Invalid states for all customers, to be redirected to AA UI for additional details.
    ///
    /// "ERROR_INVALID_BASE_SETUP"
    #[serde(rename="ERROR_INVALID_BASE_SETUP")]
    ERRORINVALIDBASESETUP,
    

    /// Returned when there is not an EKM key configured.
    ///
    /// "ERROR_MISSING_EXTERNAL_SIGNING_KEY"
    #[serde(rename="ERROR_MISSING_EXTERNAL_SIGNING_KEY")]
    ERRORMISSINGEXTERNALSIGNINGKEY,
    

    /// Returned when there are no enrolled services or the customer is enrolled in CAA only for a subset of services.
    ///
    /// "ERROR_NOT_ALL_SERVICES_ENROLLED"
    #[serde(rename="ERROR_NOT_ALL_SERVICES_ENROLLED")]
    ERRORNOTALLSERVICESENROLLED,
    

    /// Returned when exception was encountered during evaluation of other criteria.
    ///
    /// "ERROR_SETUP_CHECK_FAILED"
    #[serde(rename="ERROR_SETUP_CHECK_FAILED")]
    ERRORSETUPCHECKFAILED,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum::SETUPERRORUNSPECIFIED => "SETUP_ERROR_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum::ERRORINVALIDBASESETUP => "ERROR_INVALID_BASE_SETUP",
            GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum::ERRORMISSINGEXTERNALSIGNINGKEY => "ERROR_MISSING_EXTERNAL_SIGNING_KEY",
            GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum::ERRORNOTALLSERVICESENROLLED => "ERROR_NOT_ALL_SERVICES_ENROLLED",
            GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum::ERRORSETUPCHECKFAILED => "ERROR_SETUP_CHECK_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SETUP_ERROR_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum::SETUPERRORUNSPECIFIED),
           "ERROR_INVALID_BASE_SETUP" => Ok(GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum::ERRORINVALIDBASESETUP),
           "ERROR_MISSING_EXTERNAL_SIGNING_KEY" => Ok(GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum::ERRORMISSINGEXTERNALSIGNINGKEY),
           "ERROR_NOT_ALL_SERVICES_ENROLLED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum::ERRORNOTALLSERVICESENROLLED),
           "ERROR_SETUP_CHECK_FAILED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum::ERRORSETUPCHECKFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates SAA enrollment status of a given workload.
pub enum GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum {
    

    /// Unspecified.
    ///
    /// "SETUP_STATE_UNSPECIFIED"
    #[serde(rename="SETUP_STATE_UNSPECIFIED")]
    SETUPSTATEUNSPECIFIED,
    

    /// SAA enrollment pending.
    ///
    /// "STATUS_PENDING"
    #[serde(rename="STATUS_PENDING")]
    STATUSPENDING,
    

    /// SAA enrollment comopleted.
    ///
    /// "STATUS_COMPLETE"
    #[serde(rename="STATUS_COMPLETE")]
    STATUSCOMPLETE,
}

impl AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum::SETUPSTATEUNSPECIFIED => "SETUP_STATE_UNSPECIFIED",
            GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum::STATUSPENDING => "STATUS_PENDING",
            GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum::STATUSCOMPLETE => "STATUS_COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SETUP_STATE_UNSPECIFIED" => Ok(GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum::SETUPSTATEUNSPECIFIED),
           "STATUS_PENDING" => Ok(GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum::STATUSPENDING),
           "STATUS_COMPLETE" => Ok(GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum::STATUSCOMPLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


