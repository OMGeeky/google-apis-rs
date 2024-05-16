use super::*;



// region GoogleCloudApigeeV1AliaTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of alias.
pub enum GoogleCloudApigeeV1AliaTypeEnum {
    

    /// Alias type is not specified.
    ///
    /// "ALIAS_TYPE_UNSPECIFIED"
    #[serde(rename="ALIAS_TYPE_UNSPECIFIED")]
    ALIASTYPEUNSPECIFIED,
    

    /// Certificate.
    ///
    /// "CERT"
    #[serde(rename="CERT")]
    CERT,
    

    /// Key/certificate pair.
    ///
    /// "KEY_CERT"
    #[serde(rename="KEY_CERT")]
    KEYCERT,
}

impl AsRef<str> for GoogleCloudApigeeV1AliaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1AliaTypeEnum::ALIASTYPEUNSPECIFIED => "ALIAS_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1AliaTypeEnum::CERT => "CERT",
            GoogleCloudApigeeV1AliaTypeEnum::KEYCERT => "KEY_CERT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1AliaTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALIAS_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1AliaTypeEnum::ALIASTYPEUNSPECIFIED),
           "CERT" => Ok(GoogleCloudApigeeV1AliaTypeEnum::CERT),
           "KEY_CERT" => Ok(GoogleCloudApigeeV1AliaTypeEnum::KEYCERT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1AliaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1AliasRevisionConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum GoogleCloudApigeeV1AliasRevisionConfigTypeEnum {
    

    /// Alias type is not specified.
    ///
    /// "ALIAS_TYPE_UNSPECIFIED"
    #[serde(rename="ALIAS_TYPE_UNSPECIFIED")]
    ALIASTYPEUNSPECIFIED,
    

    /// Certificate.
    ///
    /// "CERT"
    #[serde(rename="CERT")]
    CERT,
    

    /// Key/certificate pair.
    ///
    /// "KEY_CERT"
    #[serde(rename="KEY_CERT")]
    KEYCERT,
}

impl AsRef<str> for GoogleCloudApigeeV1AliasRevisionConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1AliasRevisionConfigTypeEnum::ALIASTYPEUNSPECIFIED => "ALIAS_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1AliasRevisionConfigTypeEnum::CERT => "CERT",
            GoogleCloudApigeeV1AliasRevisionConfigTypeEnum::KEYCERT => "KEY_CERT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1AliasRevisionConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALIAS_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1AliasRevisionConfigTypeEnum::ALIASTYPEUNSPECIFIED),
           "CERT" => Ok(GoogleCloudApigeeV1AliasRevisionConfigTypeEnum::CERT),
           "KEY_CERT" => Ok(GoogleCloudApigeeV1AliasRevisionConfigTypeEnum::KEYCERT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1AliasRevisionConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1AnalyticsConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the Analytics add-on.
pub enum GoogleCloudApigeeV1AnalyticsConfigStateEnum {
    

    /// Default value.
    ///
    /// "ADDON_STATE_UNSPECIFIED"
    #[serde(rename="ADDON_STATE_UNSPECIFIED")]
    ADDONSTATEUNSPECIFIED,
    

    /// Add-on is in progress of enabling.
    ///
    /// "ENABLING"
    #[serde(rename="ENABLING")]
    ENABLING,
    

    /// Add-on is fully enabled and ready to use.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// Add-on is in progress of disabling.
    ///
    /// "DISABLING"
    #[serde(rename="DISABLING")]
    DISABLING,
    

    /// Add-on is fully disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for GoogleCloudApigeeV1AnalyticsConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1AnalyticsConfigStateEnum::ADDONSTATEUNSPECIFIED => "ADDON_STATE_UNSPECIFIED",
            GoogleCloudApigeeV1AnalyticsConfigStateEnum::ENABLING => "ENABLING",
            GoogleCloudApigeeV1AnalyticsConfigStateEnum::ENABLED => "ENABLED",
            GoogleCloudApigeeV1AnalyticsConfigStateEnum::DISABLING => "DISABLING",
            GoogleCloudApigeeV1AnalyticsConfigStateEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1AnalyticsConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADDON_STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1AnalyticsConfigStateEnum::ADDONSTATEUNSPECIFIED),
           "ENABLING" => Ok(GoogleCloudApigeeV1AnalyticsConfigStateEnum::ENABLING),
           "ENABLED" => Ok(GoogleCloudApigeeV1AnalyticsConfigStateEnum::ENABLED),
           "DISABLING" => Ok(GoogleCloudApigeeV1AnalyticsConfigStateEnum::DISABLING),
           "DISABLED" => Ok(GoogleCloudApigeeV1AnalyticsConfigStateEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1AnalyticsConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1ApiProductQuotaCounterScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Scope of the quota decides how the quota counter gets applied and evaluate for quota violation. If the Scope is set as PROXY, then all the operations defined for the APIproduct that are associated with the same proxy will share the same quota counter set at the APIproduct level, making it a global counter at a proxy level. If the Scope is set as OPERATION, then each operations get the counter set at the API product dedicated, making it a local counter. Note that, the QuotaCounterScope applies only when an operation does not have dedicated quota set for itself.
pub enum GoogleCloudApigeeV1ApiProductQuotaCounterScopeEnum {
    

    /// When quota is not explicitly defined for each operation(REST/GraphQL), the limits set at product level will be used as a local counter for quota evaluation by all the operations, independent of proxy association.
    ///
    /// "QUOTA_COUNTER_SCOPE_UNSPECIFIED"
    #[serde(rename="QUOTA_COUNTER_SCOPE_UNSPECIFIED")]
    QUOTACOUNTERSCOPEUNSPECIFIED,
    

    /// When quota is not explicitly defined for each operation(REST/GraphQL), set at product level will be used as a global counter for quota evaluation by all the operations associated with a particular proxy.
    ///
    /// "PROXY"
    #[serde(rename="PROXY")]
    PROXY,
    

    /// When quota is not explicitly defined for each operation(REST/GraphQL), the limits set at product level will be used as a local counter for quota evaluation by all the operations, independent of proxy association. This behavior mimics the same as QUOTA_COUNTER_SCOPE_UNSPECIFIED.
    ///
    /// "OPERATION"
    #[serde(rename="OPERATION")]
    OPERATION,
}

impl AsRef<str> for GoogleCloudApigeeV1ApiProductQuotaCounterScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1ApiProductQuotaCounterScopeEnum::QUOTACOUNTERSCOPEUNSPECIFIED => "QUOTA_COUNTER_SCOPE_UNSPECIFIED",
            GoogleCloudApigeeV1ApiProductQuotaCounterScopeEnum::PROXY => "PROXY",
            GoogleCloudApigeeV1ApiProductQuotaCounterScopeEnum::OPERATION => "OPERATION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1ApiProductQuotaCounterScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "QUOTA_COUNTER_SCOPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1ApiProductQuotaCounterScopeEnum::QUOTACOUNTERSCOPEUNSPECIFIED),
           "PROXY" => Ok(GoogleCloudApigeeV1ApiProductQuotaCounterScopeEnum::PROXY),
           "OPERATION" => Ok(GoogleCloudApigeeV1ApiProductQuotaCounterScopeEnum::OPERATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1ApiProductQuotaCounterScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1ApiProxyApiProxyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the API proxy.
pub enum GoogleCloudApigeeV1ApiProxyApiProxyTypeEnum {
    

    /// API proxy type not specified.
    ///
    /// "API_PROXY_TYPE_UNSPECIFIED"
    #[serde(rename="API_PROXY_TYPE_UNSPECIFIED")]
    APIPROXYTYPEUNSPECIFIED,
    

    /// Programmable API Proxies enable you to develop APIs with highly flexible behavior using bundled policy configuration and one or more programming languages to describe complex sequential and/or conditional flows of logic.
    ///
    /// "PROGRAMMABLE"
    #[serde(rename="PROGRAMMABLE")]
    PROGRAMMABLE,
    

    /// Configurable API Proxies enable you to develop efficient APIs using simple configuration while complex execution control flow logic is handled by Apigee. This type only works with the ARCHIVE deployment type and cannot be combined with the PROXY deployment type.
    ///
    /// "CONFIGURABLE"
    #[serde(rename="CONFIGURABLE")]
    CONFIGURABLE,
}

impl AsRef<str> for GoogleCloudApigeeV1ApiProxyApiProxyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1ApiProxyApiProxyTypeEnum::APIPROXYTYPEUNSPECIFIED => "API_PROXY_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1ApiProxyApiProxyTypeEnum::PROGRAMMABLE => "PROGRAMMABLE",
            GoogleCloudApigeeV1ApiProxyApiProxyTypeEnum::CONFIGURABLE => "CONFIGURABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1ApiProxyApiProxyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "API_PROXY_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1ApiProxyApiProxyTypeEnum::APIPROXYTYPEUNSPECIFIED),
           "PROGRAMMABLE" => Ok(GoogleCloudApigeeV1ApiProxyApiProxyTypeEnum::PROGRAMMABLE),
           "CONFIGURABLE" => Ok(GoogleCloudApigeeV1ApiProxyApiProxyTypeEnum::CONFIGURABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1ApiProxyApiProxyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1BatchComputeSecurityAssessmentResultsRequestResourceArrayResourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Type of this resource.
pub enum GoogleCloudApigeeV1BatchComputeSecurityAssessmentResultsRequestResourceArrayResourceTypeEnum {
    

    /// ResourceType not specified.
    ///
    /// "RESOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="RESOURCE_TYPE_UNSPECIFIED")]
    RESOURCETYPEUNSPECIFIED,
    

    /// Resource is an Apigee Proxy.
    ///
    /// "API_PROXY"
    #[serde(rename="API_PROXY")]
    APIPROXY,
}

impl AsRef<str> for GoogleCloudApigeeV1BatchComputeSecurityAssessmentResultsRequestResourceArrayResourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1BatchComputeSecurityAssessmentResultsRequestResourceArrayResourceTypeEnum::RESOURCETYPEUNSPECIFIED => "RESOURCE_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1BatchComputeSecurityAssessmentResultsRequestResourceArrayResourceTypeEnum::APIPROXY => "API_PROXY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1BatchComputeSecurityAssessmentResultsRequestResourceArrayResourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1BatchComputeSecurityAssessmentResultsRequestResourceArrayResourceTypeEnum::RESOURCETYPEUNSPECIFIED),
           "API_PROXY" => Ok(GoogleCloudApigeeV1BatchComputeSecurityAssessmentResultsRequestResourceArrayResourceTypeEnum::APIPROXY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1BatchComputeSecurityAssessmentResultsRequestResourceArrayResourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1CanaryEvaluationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the canary evaluation.
pub enum GoogleCloudApigeeV1CanaryEvaluationStateEnum {
    

    /// No state has been specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The canary evaluation is still in progress.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The canary evaluation has finished.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
}

impl AsRef<str> for GoogleCloudApigeeV1CanaryEvaluationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1CanaryEvaluationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudApigeeV1CanaryEvaluationStateEnum::RUNNING => "RUNNING",
            GoogleCloudApigeeV1CanaryEvaluationStateEnum::SUCCEEDED => "SUCCEEDED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1CanaryEvaluationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1CanaryEvaluationStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(GoogleCloudApigeeV1CanaryEvaluationStateEnum::RUNNING),
           "SUCCEEDED" => Ok(GoogleCloudApigeeV1CanaryEvaluationStateEnum::SUCCEEDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1CanaryEvaluationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1CanaryEvaluationVerdictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The resulting verdict of the canary evaluations: NONE, PASS, or FAIL.
pub enum GoogleCloudApigeeV1CanaryEvaluationVerdictEnum {
    

    /// Verdict is not available yet.
    ///
    /// "VERDICT_UNSPECIFIED"
    #[serde(rename="VERDICT_UNSPECIFIED")]
    VERDICTUNSPECIFIED,
    

    /// No verdict reached.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Evaluation is not good.
    ///
    /// "FAIL"
    #[serde(rename="FAIL")]
    FAIL,
    

    /// Evaluation is good.
    ///
    /// "PASS"
    #[serde(rename="PASS")]
    PASS,
}

impl AsRef<str> for GoogleCloudApigeeV1CanaryEvaluationVerdictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1CanaryEvaluationVerdictEnum::VERDICTUNSPECIFIED => "VERDICT_UNSPECIFIED",
            GoogleCloudApigeeV1CanaryEvaluationVerdictEnum::NONE => "NONE",
            GoogleCloudApigeeV1CanaryEvaluationVerdictEnum::FAIL => "FAIL",
            GoogleCloudApigeeV1CanaryEvaluationVerdictEnum::PASS => "PASS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1CanaryEvaluationVerdictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERDICT_UNSPECIFIED" => Ok(GoogleCloudApigeeV1CanaryEvaluationVerdictEnum::VERDICTUNSPECIFIED),
           "NONE" => Ok(GoogleCloudApigeeV1CanaryEvaluationVerdictEnum::NONE),
           "FAIL" => Ok(GoogleCloudApigeeV1CanaryEvaluationVerdictEnum::FAIL),
           "PASS" => Ok(GoogleCloudApigeeV1CanaryEvaluationVerdictEnum::PASS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1CanaryEvaluationVerdictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1DataCollectorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The type of data this data collector will collect.
pub enum GoogleCloudApigeeV1DataCollectorTypeEnum {
    

    /// For future compatibility.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// For integer values.
    ///
    /// "INTEGER"
    #[serde(rename="INTEGER")]
    INTEGER,
    

    /// For float values.
    ///
    /// "FLOAT"
    #[serde(rename="FLOAT")]
    FLOAT,
    

    /// For string values.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// For boolean values.
    ///
    /// "BOOLEAN"
    #[serde(rename="BOOLEAN")]
    BOOLEAN,
    

    /// For datetime values.
    ///
    /// "DATETIME"
    #[serde(rename="DATETIME")]
    DATETIME,
}

impl AsRef<str> for GoogleCloudApigeeV1DataCollectorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1DataCollectorTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1DataCollectorTypeEnum::INTEGER => "INTEGER",
            GoogleCloudApigeeV1DataCollectorTypeEnum::FLOAT => "FLOAT",
            GoogleCloudApigeeV1DataCollectorTypeEnum::STRING => "STRING",
            GoogleCloudApigeeV1DataCollectorTypeEnum::BOOLEAN => "BOOLEAN",
            GoogleCloudApigeeV1DataCollectorTypeEnum::DATETIME => "DATETIME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1DataCollectorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1DataCollectorTypeEnum::TYPEUNSPECIFIED),
           "INTEGER" => Ok(GoogleCloudApigeeV1DataCollectorTypeEnum::INTEGER),
           "FLOAT" => Ok(GoogleCloudApigeeV1DataCollectorTypeEnum::FLOAT),
           "STRING" => Ok(GoogleCloudApigeeV1DataCollectorTypeEnum::STRING),
           "BOOLEAN" => Ok(GoogleCloudApigeeV1DataCollectorTypeEnum::BOOLEAN),
           "DATETIME" => Ok(GoogleCloudApigeeV1DataCollectorTypeEnum::DATETIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1DataCollectorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1DataCollectorConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Data type accepted by the data collector.
pub enum GoogleCloudApigeeV1DataCollectorConfigTypeEnum {
    

    /// For future compatibility.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// For integer values.
    ///
    /// "INTEGER"
    #[serde(rename="INTEGER")]
    INTEGER,
    

    /// For float values.
    ///
    /// "FLOAT"
    #[serde(rename="FLOAT")]
    FLOAT,
    

    /// For string values.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// For boolean values.
    ///
    /// "BOOLEAN"
    #[serde(rename="BOOLEAN")]
    BOOLEAN,
    

    /// For datetime values.
    ///
    /// "DATETIME"
    #[serde(rename="DATETIME")]
    DATETIME,
}

impl AsRef<str> for GoogleCloudApigeeV1DataCollectorConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1DataCollectorConfigTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1DataCollectorConfigTypeEnum::INTEGER => "INTEGER",
            GoogleCloudApigeeV1DataCollectorConfigTypeEnum::FLOAT => "FLOAT",
            GoogleCloudApigeeV1DataCollectorConfigTypeEnum::STRING => "STRING",
            GoogleCloudApigeeV1DataCollectorConfigTypeEnum::BOOLEAN => "BOOLEAN",
            GoogleCloudApigeeV1DataCollectorConfigTypeEnum::DATETIME => "DATETIME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1DataCollectorConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1DataCollectorConfigTypeEnum::TYPEUNSPECIFIED),
           "INTEGER" => Ok(GoogleCloudApigeeV1DataCollectorConfigTypeEnum::INTEGER),
           "FLOAT" => Ok(GoogleCloudApigeeV1DataCollectorConfigTypeEnum::FLOAT),
           "STRING" => Ok(GoogleCloudApigeeV1DataCollectorConfigTypeEnum::STRING),
           "BOOLEAN" => Ok(GoogleCloudApigeeV1DataCollectorConfigTypeEnum::BOOLEAN),
           "DATETIME" => Ok(GoogleCloudApigeeV1DataCollectorConfigTypeEnum::DATETIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1DataCollectorConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1DeploymentProxyDeploymentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the deployment (standard or extensible) Deployed proxy revision will be marked as extensible in following 2 cases. 1. The deployed proxy revision uses extensible policies. 2. If a environment supports flowhooks and flow hook is configured.
pub enum GoogleCloudApigeeV1DeploymentProxyDeploymentTypeEnum {
    

    /// Default value till public preview. After public preview this value should not be returned.
    ///
    /// "PROXY_DEPLOYMENT_TYPE_UNSPECIFIED"
    #[serde(rename="PROXY_DEPLOYMENT_TYPE_UNSPECIFIED")]
    PROXYDEPLOYMENTTYPEUNSPECIFIED,
    

    /// Deployment will be of type Standard if only Standard proxies are used
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// Proxy will be of type Extensible if deployments uses one or more Extensible proxies
    ///
    /// "EXTENSIBLE"
    #[serde(rename="EXTENSIBLE")]
    EXTENSIBLE,
}

impl AsRef<str> for GoogleCloudApigeeV1DeploymentProxyDeploymentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1DeploymentProxyDeploymentTypeEnum::PROXYDEPLOYMENTTYPEUNSPECIFIED => "PROXY_DEPLOYMENT_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1DeploymentProxyDeploymentTypeEnum::STANDARD => "STANDARD",
            GoogleCloudApigeeV1DeploymentProxyDeploymentTypeEnum::EXTENSIBLE => "EXTENSIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1DeploymentProxyDeploymentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROXY_DEPLOYMENT_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1DeploymentProxyDeploymentTypeEnum::PROXYDEPLOYMENTTYPEUNSPECIFIED),
           "STANDARD" => Ok(GoogleCloudApigeeV1DeploymentProxyDeploymentTypeEnum::STANDARD),
           "EXTENSIBLE" => Ok(GoogleCloudApigeeV1DeploymentProxyDeploymentTypeEnum::EXTENSIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1DeploymentProxyDeploymentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1DeploymentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Current state of the deployment. **Note**: This field is displayed only when viewing deployment status.
pub enum GoogleCloudApigeeV1DeploymentStateEnum {
    

    /// This value should never be returned.
    ///
    /// "RUNTIME_STATE_UNSPECIFIED"
    #[serde(rename="RUNTIME_STATE_UNSPECIFIED")]
    RUNTIMESTATEUNSPECIFIED,
    

    /// Runtime has loaded the deployment.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Deployment is not fully ready in the runtime.
    ///
    /// "PROGRESSING"
    #[serde(rename="PROGRESSING")]
    PROGRESSING,
    

    /// Encountered an error with the deployment that requires intervention.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for GoogleCloudApigeeV1DeploymentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1DeploymentStateEnum::RUNTIMESTATEUNSPECIFIED => "RUNTIME_STATE_UNSPECIFIED",
            GoogleCloudApigeeV1DeploymentStateEnum::READY => "READY",
            GoogleCloudApigeeV1DeploymentStateEnum::PROGRESSING => "PROGRESSING",
            GoogleCloudApigeeV1DeploymentStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1DeploymentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RUNTIME_STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1DeploymentStateEnum::RUNTIMESTATEUNSPECIFIED),
           "READY" => Ok(GoogleCloudApigeeV1DeploymentStateEnum::READY),
           "PROGRESSING" => Ok(GoogleCloudApigeeV1DeploymentStateEnum::PROGRESSING),
           "ERROR" => Ok(GoogleCloudApigeeV1DeploymentStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1DeploymentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1DeploymentGroupConfigDeploymentGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the deployment group, which will be either Standard or Extensible.
pub enum GoogleCloudApigeeV1DeploymentGroupConfigDeploymentGroupTypeEnum {
    

    /// Unspecified type
    ///
    /// "DEPLOYMENT_GROUP_TYPE_UNSPECIFIED"
    #[serde(rename="DEPLOYMENT_GROUP_TYPE_UNSPECIFIED")]
    DEPLOYMENTGROUPTYPEUNSPECIFIED,
    

    /// Standard type
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// Extensible Type
    ///
    /// "EXTENSIBLE"
    #[serde(rename="EXTENSIBLE")]
    EXTENSIBLE,
}

impl AsRef<str> for GoogleCloudApigeeV1DeploymentGroupConfigDeploymentGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1DeploymentGroupConfigDeploymentGroupTypeEnum::DEPLOYMENTGROUPTYPEUNSPECIFIED => "DEPLOYMENT_GROUP_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1DeploymentGroupConfigDeploymentGroupTypeEnum::STANDARD => "STANDARD",
            GoogleCloudApigeeV1DeploymentGroupConfigDeploymentGroupTypeEnum::EXTENSIBLE => "EXTENSIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1DeploymentGroupConfigDeploymentGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEPLOYMENT_GROUP_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1DeploymentGroupConfigDeploymentGroupTypeEnum::DEPLOYMENTGROUPTYPEUNSPECIFIED),
           "STANDARD" => Ok(GoogleCloudApigeeV1DeploymentGroupConfigDeploymentGroupTypeEnum::STANDARD),
           "EXTENSIBLE" => Ok(GoogleCloudApigeeV1DeploymentGroupConfigDeploymentGroupTypeEnum::EXTENSIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1DeploymentGroupConfigDeploymentGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1DeveloperMonetizationConfigBillingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Billing type.
pub enum GoogleCloudApigeeV1DeveloperMonetizationConfigBillingTypeEnum {
    

    /// The default/unset value.
    ///
    /// "BILLING_TYPE_UNSPECIFIED"
    #[serde(rename="BILLING_TYPE_UNSPECIFIED")]
    BILLINGTYPEUNSPECIFIED,
    

    /// Developer pays in advance for the use of APIs and the charged amount is deducted from their account balance.
    ///
    /// "PREPAID"
    #[serde(rename="PREPAID")]
    PREPAID,
    

    /// Developer does not maintain an account balance. The API provider bills the developer for API usage.
    ///
    /// "POSTPAID"
    #[serde(rename="POSTPAID")]
    POSTPAID,
}

impl AsRef<str> for GoogleCloudApigeeV1DeveloperMonetizationConfigBillingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1DeveloperMonetizationConfigBillingTypeEnum::BILLINGTYPEUNSPECIFIED => "BILLING_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1DeveloperMonetizationConfigBillingTypeEnum::PREPAID => "PREPAID",
            GoogleCloudApigeeV1DeveloperMonetizationConfigBillingTypeEnum::POSTPAID => "POSTPAID",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1DeveloperMonetizationConfigBillingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BILLING_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1DeveloperMonetizationConfigBillingTypeEnum::BILLINGTYPEUNSPECIFIED),
           "PREPAID" => Ok(GoogleCloudApigeeV1DeveloperMonetizationConfigBillingTypeEnum::PREPAID),
           "POSTPAID" => Ok(GoogleCloudApigeeV1DeveloperMonetizationConfigBillingTypeEnum::POSTPAID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1DeveloperMonetizationConfigBillingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the endpoint attachment connection to the service attachment.
pub enum GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum {
    

    /// The connection state has not been set.
    ///
    /// "CONNECTION_STATE_UNSPECIFIED"
    #[serde(rename="CONNECTION_STATE_UNSPECIFIED")]
    CONNECTIONSTATEUNSPECIFIED,
    

    /// The connection state is unavailable at this time, possibly because the endpoint attachment is currently being provisioned.
    ///
    /// "UNAVAILABLE"
    #[serde(rename="UNAVAILABLE")]
    UNAVAILABLE,
    

    /// The connection is pending acceptance by the PSC producer.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The connection has been accepted by the PSC producer.
    ///
    /// "ACCEPTED"
    #[serde(rename="ACCEPTED")]
    ACCEPTED,
    

    /// The connection has been rejected by the PSC producer.
    ///
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// The connection has been closed by the PSC producer and will not serve traffic going forward.
    ///
    /// "CLOSED"
    #[serde(rename="CLOSED")]
    CLOSED,
    

    /// The connection has been frozen by the PSC producer and will not serve traffic.
    ///
    /// "FROZEN"
    #[serde(rename="FROZEN")]
    FROZEN,
    

    /// The connection has been accepted by the PSC producer, but it is not ready to serve the traffic due to producer side issues.
    ///
    /// "NEEDS_ATTENTION"
    #[serde(rename="NEEDS_ATTENTION")]
    NEEDSATTENTION,
}

impl AsRef<str> for GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::CONNECTIONSTATEUNSPECIFIED => "CONNECTION_STATE_UNSPECIFIED",
            GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::UNAVAILABLE => "UNAVAILABLE",
            GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::PENDING => "PENDING",
            GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::ACCEPTED => "ACCEPTED",
            GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::REJECTED => "REJECTED",
            GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::CLOSED => "CLOSED",
            GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::FROZEN => "FROZEN",
            GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::NEEDSATTENTION => "NEEDS_ATTENTION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTION_STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::CONNECTIONSTATEUNSPECIFIED),
           "UNAVAILABLE" => Ok(GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::UNAVAILABLE),
           "PENDING" => Ok(GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::PENDING),
           "ACCEPTED" => Ok(GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::ACCEPTED),
           "REJECTED" => Ok(GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::REJECTED),
           "CLOSED" => Ok(GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::CLOSED),
           "FROZEN" => Ok(GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::FROZEN),
           "NEEDS_ATTENTION" => Ok(GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum::NEEDSATTENTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1EndpointAttachmentConnectionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1EndpointAttachmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the endpoint attachment. Values other than `ACTIVE` mean the resource is not ready to use.
pub enum GoogleCloudApigeeV1EndpointAttachmentStateEnum {
    

    /// Resource is in an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is provisioned and ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for GoogleCloudApigeeV1EndpointAttachmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1EndpointAttachmentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudApigeeV1EndpointAttachmentStateEnum::CREATING => "CREATING",
            GoogleCloudApigeeV1EndpointAttachmentStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudApigeeV1EndpointAttachmentStateEnum::DELETING => "DELETING",
            GoogleCloudApigeeV1EndpointAttachmentStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1EndpointAttachmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1EndpointAttachmentStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleCloudApigeeV1EndpointAttachmentStateEnum::CREATING),
           "ACTIVE" => Ok(GoogleCloudApigeeV1EndpointAttachmentStateEnum::ACTIVE),
           "DELETING" => Ok(GoogleCloudApigeeV1EndpointAttachmentStateEnum::DELETING),
           "UPDATING" => Ok(GoogleCloudApigeeV1EndpointAttachmentStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1EndpointAttachmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1EnvironmentApiProxyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. API Proxy type supported by the environment. The type can be set when creating the Environment and cannot be changed.
pub enum GoogleCloudApigeeV1EnvironmentApiProxyTypeEnum {
    

    /// API proxy type not specified.
    ///
    /// "API_PROXY_TYPE_UNSPECIFIED"
    #[serde(rename="API_PROXY_TYPE_UNSPECIFIED")]
    APIPROXYTYPEUNSPECIFIED,
    

    /// Programmable API Proxies enable you to develop APIs with highly flexible behavior using bundled policy configuration and one or more programming languages to describe complex sequential and/or conditional flows of logic.
    ///
    /// "PROGRAMMABLE"
    #[serde(rename="PROGRAMMABLE")]
    PROGRAMMABLE,
    

    /// Configurable API Proxies enable you to develop efficient APIs using simple configuration while complex execution control flow logic is handled by Apigee. This type only works with the ARCHIVE deployment type and cannot be combined with the PROXY deployment type.
    ///
    /// "CONFIGURABLE"
    #[serde(rename="CONFIGURABLE")]
    CONFIGURABLE,
}

impl AsRef<str> for GoogleCloudApigeeV1EnvironmentApiProxyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1EnvironmentApiProxyTypeEnum::APIPROXYTYPEUNSPECIFIED => "API_PROXY_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1EnvironmentApiProxyTypeEnum::PROGRAMMABLE => "PROGRAMMABLE",
            GoogleCloudApigeeV1EnvironmentApiProxyTypeEnum::CONFIGURABLE => "CONFIGURABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1EnvironmentApiProxyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "API_PROXY_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1EnvironmentApiProxyTypeEnum::APIPROXYTYPEUNSPECIFIED),
           "PROGRAMMABLE" => Ok(GoogleCloudApigeeV1EnvironmentApiProxyTypeEnum::PROGRAMMABLE),
           "CONFIGURABLE" => Ok(GoogleCloudApigeeV1EnvironmentApiProxyTypeEnum::CONFIGURABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1EnvironmentApiProxyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1EnvironmentDeploymentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Deployment type supported by the environment. The deployment type can be set when creating the environment and cannot be changed. When you enable archive deployment, you will be **prevented from performing** a [subset of actions](/apigee/docs/api-platform/local-development/overview#prevented-actions) within the environment, including: * Managing the deployment of API proxy or shared flow revisions * Creating, updating, or deleting resource files * Creating, updating, or deleting target servers
pub enum GoogleCloudApigeeV1EnvironmentDeploymentTypeEnum {
    

    /// Deployment type not specified.
    ///
    /// "DEPLOYMENT_TYPE_UNSPECIFIED"
    #[serde(rename="DEPLOYMENT_TYPE_UNSPECIFIED")]
    DEPLOYMENTTYPEUNSPECIFIED,
    

    /// Proxy deployment enables you to develop and deploy API proxies using Apigee on Google Cloud. This cannot currently be combined with the CONFIGURABLE API proxy type.
    ///
    /// "PROXY"
    #[serde(rename="PROXY")]
    PROXY,
    

    /// Archive deployment enables you to develop API proxies locally then deploy an archive of your API proxy configuration to an environment in Apigee on Google Cloud. You will be prevented from performing a [subset of actions](/apigee/docs/api-platform/local-development/overview#prevented-actions) within the environment.
    ///
    /// "ARCHIVE"
    #[serde(rename="ARCHIVE")]
    ARCHIVE,
}

impl AsRef<str> for GoogleCloudApigeeV1EnvironmentDeploymentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1EnvironmentDeploymentTypeEnum::DEPLOYMENTTYPEUNSPECIFIED => "DEPLOYMENT_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1EnvironmentDeploymentTypeEnum::PROXY => "PROXY",
            GoogleCloudApigeeV1EnvironmentDeploymentTypeEnum::ARCHIVE => "ARCHIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1EnvironmentDeploymentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEPLOYMENT_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1EnvironmentDeploymentTypeEnum::DEPLOYMENTTYPEUNSPECIFIED),
           "PROXY" => Ok(GoogleCloudApigeeV1EnvironmentDeploymentTypeEnum::PROXY),
           "ARCHIVE" => Ok(GoogleCloudApigeeV1EnvironmentDeploymentTypeEnum::ARCHIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1EnvironmentDeploymentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1EnvironmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the environment. Values other than ACTIVE means the resource is not ready to use.
pub enum GoogleCloudApigeeV1EnvironmentStateEnum {
    

    /// Resource is in an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is provisioned and ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for GoogleCloudApigeeV1EnvironmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1EnvironmentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudApigeeV1EnvironmentStateEnum::CREATING => "CREATING",
            GoogleCloudApigeeV1EnvironmentStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudApigeeV1EnvironmentStateEnum::DELETING => "DELETING",
            GoogleCloudApigeeV1EnvironmentStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1EnvironmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1EnvironmentStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleCloudApigeeV1EnvironmentStateEnum::CREATING),
           "ACTIVE" => Ok(GoogleCloudApigeeV1EnvironmentStateEnum::ACTIVE),
           "DELETING" => Ok(GoogleCloudApigeeV1EnvironmentStateEnum::DELETING),
           "UPDATING" => Ok(GoogleCloudApigeeV1EnvironmentStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1EnvironmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1EnvironmentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. EnvironmentType selected for the environment.
pub enum GoogleCloudApigeeV1EnvironmentTypeEnum {
    

    /// Environment type not specified.
    ///
    /// "ENVIRONMENT_TYPE_UNSPECIFIED"
    #[serde(rename="ENVIRONMENT_TYPE_UNSPECIFIED")]
    ENVIRONMENTTYPEUNSPECIFIED,
    

    /// This is the default type. Base environment has limited capacity and capabilities and are usually used when you are getting started with Apigee or while experimenting. Refer to Apigee's public documentation for more details.
    ///
    /// "BASE"
    #[serde(rename="BASE")]
    BASE,
    

    /// Intermediate environment supports API management features and higher capacity than Base environment. Refer to Apigee's public documentation for more details.
    ///
    /// "INTERMEDIATE"
    #[serde(rename="INTERMEDIATE")]
    INTERMEDIATE,
    

    /// Comprehensive environment supports advanced capabilites and even higher capacity than Intermediate environment. Refer to Apigee's public documentation for more details.
    ///
    /// "COMPREHENSIVE"
    #[serde(rename="COMPREHENSIVE")]
    COMPREHENSIVE,
}

impl AsRef<str> for GoogleCloudApigeeV1EnvironmentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1EnvironmentTypeEnum::ENVIRONMENTTYPEUNSPECIFIED => "ENVIRONMENT_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1EnvironmentTypeEnum::BASE => "BASE",
            GoogleCloudApigeeV1EnvironmentTypeEnum::INTERMEDIATE => "INTERMEDIATE",
            GoogleCloudApigeeV1EnvironmentTypeEnum::COMPREHENSIVE => "COMPREHENSIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1EnvironmentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENVIRONMENT_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1EnvironmentTypeEnum::ENVIRONMENTTYPEUNSPECIFIED),
           "BASE" => Ok(GoogleCloudApigeeV1EnvironmentTypeEnum::BASE),
           "INTERMEDIATE" => Ok(GoogleCloudApigeeV1EnvironmentTypeEnum::INTERMEDIATE),
           "COMPREHENSIVE" => Ok(GoogleCloudApigeeV1EnvironmentTypeEnum::COMPREHENSIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1EnvironmentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1EnvironmentGroupStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the environment group. Values other than ACTIVE means the resource is not ready to use.
pub enum GoogleCloudApigeeV1EnvironmentGroupStateEnum {
    

    /// Resource is in an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is provisioned and ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for GoogleCloudApigeeV1EnvironmentGroupStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1EnvironmentGroupStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudApigeeV1EnvironmentGroupStateEnum::CREATING => "CREATING",
            GoogleCloudApigeeV1EnvironmentGroupStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudApigeeV1EnvironmentGroupStateEnum::DELETING => "DELETING",
            GoogleCloudApigeeV1EnvironmentGroupStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1EnvironmentGroupStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1EnvironmentGroupStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleCloudApigeeV1EnvironmentGroupStateEnum::CREATING),
           "ACTIVE" => Ok(GoogleCloudApigeeV1EnvironmentGroupStateEnum::ACTIVE),
           "DELETING" => Ok(GoogleCloudApigeeV1EnvironmentGroupStateEnum::DELETING),
           "UPDATING" => Ok(GoogleCloudApigeeV1EnvironmentGroupStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1EnvironmentGroupStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1InstancePeeringCidrRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Size of the CIDR block range that will be reserved by the instance. PAID organizations support `SLASH_16` to `SLASH_20` and defaults to `SLASH_16`. Evaluation organizations support only `SLASH_23`.
pub enum GoogleCloudApigeeV1InstancePeeringCidrRangeEnum {
    

    /// Range not specified.
    ///
    /// "CIDR_RANGE_UNSPECIFIED"
    #[serde(rename="CIDR_RANGE_UNSPECIFIED")]
    CIDRRANGEUNSPECIFIED,
    

    /// `/16` CIDR range.
    ///
    /// "SLASH_16"
    #[serde(rename="SLASH_16")]
    SLASH16,
    

    /// `/17` CIDR range.
    ///
    /// "SLASH_17"
    #[serde(rename="SLASH_17")]
    SLASH17,
    

    /// `/18` CIDR range.
    ///
    /// "SLASH_18"
    #[serde(rename="SLASH_18")]
    SLASH18,
    

    /// `/19` CIDR range.
    ///
    /// "SLASH_19"
    #[serde(rename="SLASH_19")]
    SLASH19,
    

    /// `/20` CIDR range.
    ///
    /// "SLASH_20"
    #[serde(rename="SLASH_20")]
    SLASH20,
    

    /// `/22` CIDR range. Supported for evaluation only.
    ///
    /// "SLASH_22"
    #[serde(rename="SLASH_22")]
    SLASH22,
    

    /// `/23` CIDR range. Supported for evaluation only.
    ///
    /// "SLASH_23"
    #[serde(rename="SLASH_23")]
    SLASH23,
}

impl AsRef<str> for GoogleCloudApigeeV1InstancePeeringCidrRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::CIDRRANGEUNSPECIFIED => "CIDR_RANGE_UNSPECIFIED",
            GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH16 => "SLASH_16",
            GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH17 => "SLASH_17",
            GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH18 => "SLASH_18",
            GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH19 => "SLASH_19",
            GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH20 => "SLASH_20",
            GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH22 => "SLASH_22",
            GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH23 => "SLASH_23",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1InstancePeeringCidrRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CIDR_RANGE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::CIDRRANGEUNSPECIFIED),
           "SLASH_16" => Ok(GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH16),
           "SLASH_17" => Ok(GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH17),
           "SLASH_18" => Ok(GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH18),
           "SLASH_19" => Ok(GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH19),
           "SLASH_20" => Ok(GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH20),
           "SLASH_22" => Ok(GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH22),
           "SLASH_23" => Ok(GoogleCloudApigeeV1InstancePeeringCidrRangeEnum::SLASH23),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1InstancePeeringCidrRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1InstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the instance. Values other than `ACTIVE` means the resource is not ready to use.
pub enum GoogleCloudApigeeV1InstanceStateEnum {
    

    /// Resource is in an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is provisioned and ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for GoogleCloudApigeeV1InstanceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1InstanceStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudApigeeV1InstanceStateEnum::CREATING => "CREATING",
            GoogleCloudApigeeV1InstanceStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudApigeeV1InstanceStateEnum::DELETING => "DELETING",
            GoogleCloudApigeeV1InstanceStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1InstanceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1InstanceStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleCloudApigeeV1InstanceStateEnum::CREATING),
           "ACTIVE" => Ok(GoogleCloudApigeeV1InstanceStateEnum::ACTIVE),
           "DELETING" => Ok(GoogleCloudApigeeV1InstanceStateEnum::DELETING),
           "UPDATING" => Ok(GoogleCloudApigeeV1InstanceStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1InstanceStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1MetricAggregationAggregationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Aggregation function associated with the metric.
pub enum GoogleCloudApigeeV1MetricAggregationAggregationEnum {
    

    /// Unspecified Aggregation function.
    ///
    /// "AGGREGATION_FUNCTION_UNSPECIFIED"
    #[serde(rename="AGGREGATION_FUNCTION_UNSPECIFIED")]
    AGGREGATIONFUNCTIONUNSPECIFIED,
    

    /// Average.
    ///
    /// "AVG"
    #[serde(rename="AVG")]
    AVG,
    

    /// Summation.
    ///
    /// "SUM"
    #[serde(rename="SUM")]
    SUM,
    

    /// Min.
    ///
    /// "MIN"
    #[serde(rename="MIN")]
    MIN,
    

    /// Max.
    ///
    /// "MAX"
    #[serde(rename="MAX")]
    MAX,
    

    /// Count distinct
    ///
    /// "COUNT_DISTINCT"
    #[serde(rename="COUNT_DISTINCT")]
    COUNTDISTINCT,
}

impl AsRef<str> for GoogleCloudApigeeV1MetricAggregationAggregationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1MetricAggregationAggregationEnum::AGGREGATIONFUNCTIONUNSPECIFIED => "AGGREGATION_FUNCTION_UNSPECIFIED",
            GoogleCloudApigeeV1MetricAggregationAggregationEnum::AVG => "AVG",
            GoogleCloudApigeeV1MetricAggregationAggregationEnum::SUM => "SUM",
            GoogleCloudApigeeV1MetricAggregationAggregationEnum::MIN => "MIN",
            GoogleCloudApigeeV1MetricAggregationAggregationEnum::MAX => "MAX",
            GoogleCloudApigeeV1MetricAggregationAggregationEnum::COUNTDISTINCT => "COUNT_DISTINCT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1MetricAggregationAggregationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AGGREGATION_FUNCTION_UNSPECIFIED" => Ok(GoogleCloudApigeeV1MetricAggregationAggregationEnum::AGGREGATIONFUNCTIONUNSPECIFIED),
           "AVG" => Ok(GoogleCloudApigeeV1MetricAggregationAggregationEnum::AVG),
           "SUM" => Ok(GoogleCloudApigeeV1MetricAggregationAggregationEnum::SUM),
           "MIN" => Ok(GoogleCloudApigeeV1MetricAggregationAggregationEnum::MIN),
           "MAX" => Ok(GoogleCloudApigeeV1MetricAggregationAggregationEnum::MAX),
           "COUNT_DISTINCT" => Ok(GoogleCloudApigeeV1MetricAggregationAggregationEnum::COUNTDISTINCT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1MetricAggregationAggregationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1MetricAggregationOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Ordering for this aggregation in the result. For time series this is ignored since the ordering of points depends only on the timestamp, not the values.
pub enum GoogleCloudApigeeV1MetricAggregationOrderEnum {
    

    /// Unspecified order. Default is Descending.
    ///
    /// "ORDER_UNSPECIFIED"
    #[serde(rename="ORDER_UNSPECIFIED")]
    ORDERUNSPECIFIED,
    

    /// Ascending sort order.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// Descending sort order.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for GoogleCloudApigeeV1MetricAggregationOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1MetricAggregationOrderEnum::ORDERUNSPECIFIED => "ORDER_UNSPECIFIED",
            GoogleCloudApigeeV1MetricAggregationOrderEnum::ASCENDING => "ASCENDING",
            GoogleCloudApigeeV1MetricAggregationOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1MetricAggregationOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORDER_UNSPECIFIED" => Ok(GoogleCloudApigeeV1MetricAggregationOrderEnum::ORDERUNSPECIFIED),
           "ASCENDING" => Ok(GoogleCloudApigeeV1MetricAggregationOrderEnum::ASCENDING),
           "DESCENDING" => Ok(GoogleCloudApigeeV1MetricAggregationOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1MetricAggregationOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1NatAddresStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the nat address.
pub enum GoogleCloudApigeeV1NatAddresStateEnum {
    

    /// The resource is in an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The NAT address is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The NAT address is reserved but not yet used for Internet egress.
    ///
    /// "RESERVED"
    #[serde(rename="RESERVED")]
    RESERVED,
    

    /// The NAT address is active and used for Internet egress.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The NAT address is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for GoogleCloudApigeeV1NatAddresStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1NatAddresStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudApigeeV1NatAddresStateEnum::CREATING => "CREATING",
            GoogleCloudApigeeV1NatAddresStateEnum::RESERVED => "RESERVED",
            GoogleCloudApigeeV1NatAddresStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudApigeeV1NatAddresStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1NatAddresStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1NatAddresStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleCloudApigeeV1NatAddresStateEnum::CREATING),
           "RESERVED" => Ok(GoogleCloudApigeeV1NatAddresStateEnum::RESERVED),
           "ACTIVE" => Ok(GoogleCloudApigeeV1NatAddresStateEnum::ACTIVE),
           "DELETING" => Ok(GoogleCloudApigeeV1NatAddresStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1NatAddresStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1OASDocumentationFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The format of the input specification file contents.
pub enum GoogleCloudApigeeV1OASDocumentationFormatEnum {
    

    /// The format is not available.
    ///
    /// "FORMAT_UNSPECIFIED"
    #[serde(rename="FORMAT_UNSPECIFIED")]
    FORMATUNSPECIFIED,
    

    /// YAML format.
    ///
    /// "YAML"
    #[serde(rename="YAML")]
    YAML,
    

    /// JSON format.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
}

impl AsRef<str> for GoogleCloudApigeeV1OASDocumentationFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1OASDocumentationFormatEnum::FORMATUNSPECIFIED => "FORMAT_UNSPECIFIED",
            GoogleCloudApigeeV1OASDocumentationFormatEnum::YAML => "YAML",
            GoogleCloudApigeeV1OASDocumentationFormatEnum::JSON => "JSON",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1OASDocumentationFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORMAT_UNSPECIFIED" => Ok(GoogleCloudApigeeV1OASDocumentationFormatEnum::FORMATUNSPECIFIED),
           "YAML" => Ok(GoogleCloudApigeeV1OASDocumentationFormatEnum::YAML),
           "JSON" => Ok(GoogleCloudApigeeV1OASDocumentationFormatEnum::JSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1OASDocumentationFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1OrganizationBillingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing).
pub enum GoogleCloudApigeeV1OrganizationBillingTypeEnum {
    

    /// Billing type not specified.
    ///
    /// "BILLING_TYPE_UNSPECIFIED"
    #[serde(rename="BILLING_TYPE_UNSPECIFIED")]
    BILLINGTYPEUNSPECIFIED,
    

    /// A pre-paid subscription to Apigee.
    ///
    /// "SUBSCRIPTION"
    #[serde(rename="SUBSCRIPTION")]
    SUBSCRIPTION,
    

    /// Free and limited access to Apigee for evaluation purposes only.
    ///
    /// "EVALUATION"
    #[serde(rename="EVALUATION")]
    EVALUATION,
    

    /// Access to Apigee using a Pay-As-You-Go plan.
    ///
    /// "PAYG"
    #[serde(rename="PAYG")]
    PAYG,
}

impl AsRef<str> for GoogleCloudApigeeV1OrganizationBillingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1OrganizationBillingTypeEnum::BILLINGTYPEUNSPECIFIED => "BILLING_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1OrganizationBillingTypeEnum::SUBSCRIPTION => "SUBSCRIPTION",
            GoogleCloudApigeeV1OrganizationBillingTypeEnum::EVALUATION => "EVALUATION",
            GoogleCloudApigeeV1OrganizationBillingTypeEnum::PAYG => "PAYG",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1OrganizationBillingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BILLING_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1OrganizationBillingTypeEnum::BILLINGTYPEUNSPECIFIED),
           "SUBSCRIPTION" => Ok(GoogleCloudApigeeV1OrganizationBillingTypeEnum::SUBSCRIPTION),
           "EVALUATION" => Ok(GoogleCloudApigeeV1OrganizationBillingTypeEnum::EVALUATION),
           "PAYG" => Ok(GoogleCloudApigeeV1OrganizationBillingTypeEnum::PAYG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1OrganizationBillingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1OrganizationRuntimeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Runtime type of the Apigee organization based on the Apigee subscription purchased.
pub enum GoogleCloudApigeeV1OrganizationRuntimeTypeEnum {
    

    /// Runtime type not specified.
    ///
    /// "RUNTIME_TYPE_UNSPECIFIED"
    #[serde(rename="RUNTIME_TYPE_UNSPECIFIED")]
    RUNTIMETYPEUNSPECIFIED,
    

    /// Google-managed Apigee runtime.
    ///
    /// "CLOUD"
    #[serde(rename="CLOUD")]
    CLOUD,
    

    /// User-managed Apigee hybrid runtime.
    ///
    /// "HYBRID"
    #[serde(rename="HYBRID")]
    HYBRID,
}

impl AsRef<str> for GoogleCloudApigeeV1OrganizationRuntimeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1OrganizationRuntimeTypeEnum::RUNTIMETYPEUNSPECIFIED => "RUNTIME_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1OrganizationRuntimeTypeEnum::CLOUD => "CLOUD",
            GoogleCloudApigeeV1OrganizationRuntimeTypeEnum::HYBRID => "HYBRID",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1OrganizationRuntimeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RUNTIME_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1OrganizationRuntimeTypeEnum::RUNTIMETYPEUNSPECIFIED),
           "CLOUD" => Ok(GoogleCloudApigeeV1OrganizationRuntimeTypeEnum::CLOUD),
           "HYBRID" => Ok(GoogleCloudApigeeV1OrganizationRuntimeTypeEnum::HYBRID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1OrganizationRuntimeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1OrganizationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the organization. Values other than ACTIVE means the resource is not ready to use.
pub enum GoogleCloudApigeeV1OrganizationStateEnum {
    

    /// Resource is in an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is provisioned and ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for GoogleCloudApigeeV1OrganizationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1OrganizationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudApigeeV1OrganizationStateEnum::CREATING => "CREATING",
            GoogleCloudApigeeV1OrganizationStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudApigeeV1OrganizationStateEnum::DELETING => "DELETING",
            GoogleCloudApigeeV1OrganizationStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1OrganizationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1OrganizationStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleCloudApigeeV1OrganizationStateEnum::CREATING),
           "ACTIVE" => Ok(GoogleCloudApigeeV1OrganizationStateEnum::ACTIVE),
           "DELETING" => Ok(GoogleCloudApigeeV1OrganizationStateEnum::DELETING),
           "UPDATING" => Ok(GoogleCloudApigeeV1OrganizationStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1OrganizationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1OrganizationSubscriptionPlanEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Subscription plan that the customer has purchased. Output only.
pub enum GoogleCloudApigeeV1OrganizationSubscriptionPlanEnum {
    

    /// Subscription plan not specified.
    ///
    /// "SUBSCRIPTION_PLAN_UNSPECIFIED"
    #[serde(rename="SUBSCRIPTION_PLAN_UNSPECIFIED")]
    SUBSCRIPTIONPLANUNSPECIFIED,
    

    /// Traditional subscription plan.
    ///
    /// "SUBSCRIPTION_2021"
    #[serde(rename="SUBSCRIPTION_2021")]
    SUBSCRIPTION2021,
    

    /// New subscription plan that provides standard proxy and scaled proxy implementation.
    ///
    /// "SUBSCRIPTION_2024"
    #[serde(rename="SUBSCRIPTION_2024")]
    SUBSCRIPTION2024,
}

impl AsRef<str> for GoogleCloudApigeeV1OrganizationSubscriptionPlanEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1OrganizationSubscriptionPlanEnum::SUBSCRIPTIONPLANUNSPECIFIED => "SUBSCRIPTION_PLAN_UNSPECIFIED",
            GoogleCloudApigeeV1OrganizationSubscriptionPlanEnum::SUBSCRIPTION2021 => "SUBSCRIPTION_2021",
            GoogleCloudApigeeV1OrganizationSubscriptionPlanEnum::SUBSCRIPTION2024 => "SUBSCRIPTION_2024",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1OrganizationSubscriptionPlanEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBSCRIPTION_PLAN_UNSPECIFIED" => Ok(GoogleCloudApigeeV1OrganizationSubscriptionPlanEnum::SUBSCRIPTIONPLANUNSPECIFIED),
           "SUBSCRIPTION_2021" => Ok(GoogleCloudApigeeV1OrganizationSubscriptionPlanEnum::SUBSCRIPTION2021),
           "SUBSCRIPTION_2024" => Ok(GoogleCloudApigeeV1OrganizationSubscriptionPlanEnum::SUBSCRIPTION2024),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1OrganizationSubscriptionPlanEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. DEPRECATED: This will eventually be replaced by BillingType. Subscription type of the Apigee organization. Valid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased). See [Apigee pricing](https://cloud.google.com/apigee/pricing/).
pub enum GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum {
    

    /// Subscription type not specified.
    ///
    /// "SUBSCRIPTION_TYPE_UNSPECIFIED"
    #[serde(rename="SUBSCRIPTION_TYPE_UNSPECIFIED")]
    SUBSCRIPTIONTYPEUNSPECIFIED,
    

    /// Full subscription to Apigee has been purchased.
    ///
    /// "PAID"
    #[serde(rename="PAID")]
    PAID,
    

    /// Subscription to Apigee is free, limited, and used for evaluation purposes only.
    ///
    /// "TRIAL"
    #[serde(rename="TRIAL")]
    TRIAL,
}

impl AsRef<str> for GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum::SUBSCRIPTIONTYPEUNSPECIFIED => "SUBSCRIPTION_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum::PAID => "PAID",
            GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum::TRIAL => "TRIAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBSCRIPTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum::SUBSCRIPTIONTYPEUNSPECIFIED),
           "PAID" => Ok(GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum::PAID),
           "TRIAL" => Ok(GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum::TRIAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1OrganizationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Not used by Apigee.
pub enum GoogleCloudApigeeV1OrganizationTypeEnum {
    

    /// Subscription type not specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Subscription to Apigee is free, limited, and used for evaluation purposes only.
    ///
    /// "TYPE_TRIAL"
    #[serde(rename="TYPE_TRIAL")]
    TYPETRIAL,
    

    /// Full subscription to Apigee has been purchased. See [Apigee pricing](https://cloud.google.com/apigee/pricing/).
    ///
    /// "TYPE_PAID"
    #[serde(rename="TYPE_PAID")]
    TYPEPAID,
    

    /// For internal users only.
    ///
    /// "TYPE_INTERNAL"
    #[serde(rename="TYPE_INTERNAL")]
    TYPEINTERNAL,
}

impl AsRef<str> for GoogleCloudApigeeV1OrganizationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1OrganizationTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1OrganizationTypeEnum::TYPETRIAL => "TYPE_TRIAL",
            GoogleCloudApigeeV1OrganizationTypeEnum::TYPEPAID => "TYPE_PAID",
            GoogleCloudApigeeV1OrganizationTypeEnum::TYPEINTERNAL => "TYPE_INTERNAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1OrganizationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1OrganizationTypeEnum::TYPEUNSPECIFIED),
           "TYPE_TRIAL" => Ok(GoogleCloudApigeeV1OrganizationTypeEnum::TYPETRIAL),
           "TYPE_PAID" => Ok(GoogleCloudApigeeV1OrganizationTypeEnum::TYPEPAID),
           "TYPE_INTERNAL" => Ok(GoogleCloudApigeeV1OrganizationTypeEnum::TYPEINTERNAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1OrganizationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1QueryTimeSeriesStatsRequestTimestampOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order the sequences in increasing or decreasing order of timestamps. Default is descending order of timestamps (latest first).
pub enum GoogleCloudApigeeV1QueryTimeSeriesStatsRequestTimestampOrderEnum {
    

    /// Unspecified order. Default is Descending.
    ///
    /// "ORDER_UNSPECIFIED"
    #[serde(rename="ORDER_UNSPECIFIED")]
    ORDERUNSPECIFIED,
    

    /// Ascending sort order.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// Descending sort order.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for GoogleCloudApigeeV1QueryTimeSeriesStatsRequestTimestampOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1QueryTimeSeriesStatsRequestTimestampOrderEnum::ORDERUNSPECIFIED => "ORDER_UNSPECIFIED",
            GoogleCloudApigeeV1QueryTimeSeriesStatsRequestTimestampOrderEnum::ASCENDING => "ASCENDING",
            GoogleCloudApigeeV1QueryTimeSeriesStatsRequestTimestampOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1QueryTimeSeriesStatsRequestTimestampOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORDER_UNSPECIFIED" => Ok(GoogleCloudApigeeV1QueryTimeSeriesStatsRequestTimestampOrderEnum::ORDERUNSPECIFIED),
           "ASCENDING" => Ok(GoogleCloudApigeeV1QueryTimeSeriesStatsRequestTimestampOrderEnum::ASCENDING),
           "DESCENDING" => Ok(GoogleCloudApigeeV1QueryTimeSeriesStatsRequestTimestampOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1QueryTimeSeriesStatsRequestTimestampOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Time buckets to group the stats by.
pub enum GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum {
    

    /// Unspecified window size. Default is 1 hour.
    ///
    /// "WINDOW_SIZE_UNSPECIFIED"
    #[serde(rename="WINDOW_SIZE_UNSPECIFIED")]
    WINDOWSIZEUNSPECIFIED,
    

    /// 1 Minute window
    ///
    /// "MINUTE"
    #[serde(rename="MINUTE")]
    MINUTE,
    

    /// 1 Hour window
    ///
    /// "HOUR"
    #[serde(rename="HOUR")]
    HOUR,
    

    /// 1 Day window
    ///
    /// "DAY"
    #[serde(rename="DAY")]
    DAY,
    

    /// 1 Month window
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
}

impl AsRef<str> for GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum::WINDOWSIZEUNSPECIFIED => "WINDOW_SIZE_UNSPECIFIED",
            GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum::MINUTE => "MINUTE",
            GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum::HOUR => "HOUR",
            GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum::DAY => "DAY",
            GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum::MONTH => "MONTH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WINDOW_SIZE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum::WINDOWSIZEUNSPECIFIED),
           "MINUTE" => Ok(GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum::MINUTE),
           "HOUR" => Ok(GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum::HOUR),
           "DAY" => Ok(GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum::DAY),
           "MONTH" => Ok(GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum::MONTH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1QueryTimeSeriesStatsRequestWindowSizeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1RatePlanBillingPeriodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Frequency at which the customer will be billed.
pub enum GoogleCloudApigeeV1RatePlanBillingPeriodEnum {
    

    /// Billing period not specified.
    ///
    /// "BILLING_PERIOD_UNSPECIFIED"
    #[serde(rename="BILLING_PERIOD_UNSPECIFIED")]
    BILLINGPERIODUNSPECIFIED,
    

    /// Weekly billing period. **Note**: Not supported by Apigee at this time.
    ///
    /// "WEEKLY"
    #[serde(rename="WEEKLY")]
    WEEKLY,
    

    /// Monthly billing period.
    ///
    /// "MONTHLY"
    #[serde(rename="MONTHLY")]
    MONTHLY,
}

impl AsRef<str> for GoogleCloudApigeeV1RatePlanBillingPeriodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1RatePlanBillingPeriodEnum::BILLINGPERIODUNSPECIFIED => "BILLING_PERIOD_UNSPECIFIED",
            GoogleCloudApigeeV1RatePlanBillingPeriodEnum::WEEKLY => "WEEKLY",
            GoogleCloudApigeeV1RatePlanBillingPeriodEnum::MONTHLY => "MONTHLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1RatePlanBillingPeriodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BILLING_PERIOD_UNSPECIFIED" => Ok(GoogleCloudApigeeV1RatePlanBillingPeriodEnum::BILLINGPERIODUNSPECIFIED),
           "WEEKLY" => Ok(GoogleCloudApigeeV1RatePlanBillingPeriodEnum::WEEKLY),
           "MONTHLY" => Ok(GoogleCloudApigeeV1RatePlanBillingPeriodEnum::MONTHLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1RatePlanBillingPeriodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Pricing model used for consumption-based charges.
pub enum GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum {
    

    /// Pricing model not specified. This is the default.
    ///
    /// "CONSUMPTION_PRICING_TYPE_UNSPECIFIED"
    #[serde(rename="CONSUMPTION_PRICING_TYPE_UNSPECIFIED")]
    CONSUMPTIONPRICINGTYPEUNSPECIFIED,
    

    /// Fixed rate charged for each API call.
    ///
    /// "FIXED_PER_UNIT"
    #[serde(rename="FIXED_PER_UNIT")]
    FIXEDPERUNIT,
    

    /// Variable rate charged for each API call based on price tiers. Example: * 1-100 calls cost $2 per call * 101-200 calls cost $1.50 per call * 201-300 calls cost $1 per call * Total price for 50 calls: 50 x $2 = $100 * Total price for 150 calls: 100 x $2 + 50 x $1.5 = $275 * Total price for 250 calls: 100 x $2 + 100 x $1.5 + 50 x $1 = $400. **Note**: Not supported by Apigee at this time.
    ///
    /// "BANDED"
    #[serde(rename="BANDED")]
    BANDED,
    

    /// **Note**: Not supported by Apigee at this time.
    ///
    /// "TIERED"
    #[serde(rename="TIERED")]
    TIERED,
    

    /// **Note**: Not supported by Apigee at this time.
    ///
    /// "STAIRSTEP"
    #[serde(rename="STAIRSTEP")]
    STAIRSTEP,
}

impl AsRef<str> for GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum::CONSUMPTIONPRICINGTYPEUNSPECIFIED => "CONSUMPTION_PRICING_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum::FIXEDPERUNIT => "FIXED_PER_UNIT",
            GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum::BANDED => "BANDED",
            GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum::TIERED => "TIERED",
            GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum::STAIRSTEP => "STAIRSTEP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONSUMPTION_PRICING_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum::CONSUMPTIONPRICINGTYPEUNSPECIFIED),
           "FIXED_PER_UNIT" => Ok(GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum::FIXEDPERUNIT),
           "BANDED" => Ok(GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum::BANDED),
           "TIERED" => Ok(GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum::TIERED),
           "STAIRSTEP" => Ok(GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum::STAIRSTEP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1RatePlanConsumptionPricingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1RatePlanPaymentFundingModelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// DEPRECATED: This field is no longer supported and will eventually be removed when Apigee Hybrid 1.5/1.6 is no longer supported. Instead, use the `billingType` field inside `DeveloperMonetizationConfig` resource. Flag that specifies the billing account type, prepaid or postpaid.
pub enum GoogleCloudApigeeV1RatePlanPaymentFundingModelEnum {
    

    /// Billing account type not specified.
    ///
    /// "PAYMENT_FUNDING_MODEL_UNSPECIFIED"
    #[serde(rename="PAYMENT_FUNDING_MODEL_UNSPECIFIED")]
    PAYMENTFUNDINGMODELUNSPECIFIED,
    

    /// Prepaid billing account type. Developer pays in advance for the use of your API products. Funds are deducted from their prepaid account balance. **Note**: Not supported by Apigee at this time.
    ///
    /// "PREPAID"
    #[serde(rename="PREPAID")]
    PREPAID,
    

    /// Postpaid billing account type. Developer is billed through an invoice after using your API products.
    ///
    /// "POSTPAID"
    #[serde(rename="POSTPAID")]
    POSTPAID,
}

impl AsRef<str> for GoogleCloudApigeeV1RatePlanPaymentFundingModelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1RatePlanPaymentFundingModelEnum::PAYMENTFUNDINGMODELUNSPECIFIED => "PAYMENT_FUNDING_MODEL_UNSPECIFIED",
            GoogleCloudApigeeV1RatePlanPaymentFundingModelEnum::PREPAID => "PREPAID",
            GoogleCloudApigeeV1RatePlanPaymentFundingModelEnum::POSTPAID => "POSTPAID",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1RatePlanPaymentFundingModelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PAYMENT_FUNDING_MODEL_UNSPECIFIED" => Ok(GoogleCloudApigeeV1RatePlanPaymentFundingModelEnum::PAYMENTFUNDINGMODELUNSPECIFIED),
           "PREPAID" => Ok(GoogleCloudApigeeV1RatePlanPaymentFundingModelEnum::PREPAID),
           "POSTPAID" => Ok(GoogleCloudApigeeV1RatePlanPaymentFundingModelEnum::POSTPAID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1RatePlanPaymentFundingModelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1RatePlanRevenueShareTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Method used to calculate the revenue that is shared with developers.
pub enum GoogleCloudApigeeV1RatePlanRevenueShareTypeEnum {
    

    /// Revenue share type is not specified.
    ///
    /// "REVENUE_SHARE_TYPE_UNSPECIFIED"
    #[serde(rename="REVENUE_SHARE_TYPE_UNSPECIFIED")]
    REVENUESHARETYPEUNSPECIFIED,
    

    /// Fixed percentage of the total revenue will be shared. The percentage to be shared can be configured by the API provider.
    ///
    /// "FIXED"
    #[serde(rename="FIXED")]
    FIXED,
    

    /// Amount of revenue shared depends on the number of API calls. The API call volume ranges and the revenue share percentage for each volume can be configured by the API provider. **Note**: Not supported by Apigee at this time.
    ///
    /// "VOLUME_BANDED"
    #[serde(rename="VOLUME_BANDED")]
    VOLUMEBANDED,
}

impl AsRef<str> for GoogleCloudApigeeV1RatePlanRevenueShareTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1RatePlanRevenueShareTypeEnum::REVENUESHARETYPEUNSPECIFIED => "REVENUE_SHARE_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1RatePlanRevenueShareTypeEnum::FIXED => "FIXED",
            GoogleCloudApigeeV1RatePlanRevenueShareTypeEnum::VOLUMEBANDED => "VOLUME_BANDED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1RatePlanRevenueShareTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVENUE_SHARE_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1RatePlanRevenueShareTypeEnum::REVENUESHARETYPEUNSPECIFIED),
           "FIXED" => Ok(GoogleCloudApigeeV1RatePlanRevenueShareTypeEnum::FIXED),
           "VOLUME_BANDED" => Ok(GoogleCloudApigeeV1RatePlanRevenueShareTypeEnum::VOLUMEBANDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1RatePlanRevenueShareTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1RatePlanStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Current state of the rate plan (draft or published).
pub enum GoogleCloudApigeeV1RatePlanStateEnum {
    

    /// State of the rate plan is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Rate plan is in draft mode and only visible to API providers.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Rate plan is published and will become visible to developers for the configured duration (between `startTime` and `endTime`).
    ///
    /// "PUBLISHED"
    #[serde(rename="PUBLISHED")]
    PUBLISHED,
}

impl AsRef<str> for GoogleCloudApigeeV1RatePlanStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1RatePlanStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudApigeeV1RatePlanStateEnum::DRAFT => "DRAFT",
            GoogleCloudApigeeV1RatePlanStateEnum::PUBLISHED => "PUBLISHED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1RatePlanStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1RatePlanStateEnum::STATEUNSPECIFIED),
           "DRAFT" => Ok(GoogleCloudApigeeV1RatePlanStateEnum::DRAFT),
           "PUBLISHED" => Ok(GoogleCloudApigeeV1RatePlanStateEnum::PUBLISHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1RatePlanStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Exporter that is used to view the distributed trace captured using OpenCensus. An exporter sends traces to any backend that is capable of consuming them. Recorded spans can be exported by registered exporters.
pub enum GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum {
    

    /// Exporter unspecified
    ///
    /// "EXPORTER_UNSPECIFIED"
    #[serde(rename="EXPORTER_UNSPECIFIED")]
    EXPORTERUNSPECIFIED,
    

    /// Jaeger exporter
    ///
    /// "JAEGER"
    #[serde(rename="JAEGER")]
    JAEGER,
    

    /// Cloudtrace exporter
    ///
    /// "CLOUD_TRACE"
    #[serde(rename="CLOUD_TRACE")]
    CLOUDTRACE,
}

impl AsRef<str> for GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum::EXPORTERUNSPECIFIED => "EXPORTER_UNSPECIFIED",
            GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum::JAEGER => "JAEGER",
            GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum::CLOUDTRACE => "CLOUD_TRACE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPORTER_UNSPECIFIED" => Ok(GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum::EXPORTERUNSPECIFIED),
           "JAEGER" => Ok(GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum::JAEGER),
           "CLOUD_TRACE" => Ok(GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum::CLOUDTRACE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sampler of distributed tracing. OFF is the default value.
pub enum GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum {
    

    /// Sampler unspecified.
    ///
    /// "SAMPLER_UNSPECIFIED"
    #[serde(rename="SAMPLER_UNSPECIFIED")]
    SAMPLERUNSPECIFIED,
    

    /// OFF means distributed trace is disabled, or the sampling probability is 0.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
    

    /// PROBABILITY means traces are captured on a probability that defined by sampling_rate. The sampling rate is limited to 0 to 0.5 when this is set.
    ///
    /// "PROBABILITY"
    #[serde(rename="PROBABILITY")]
    PROBABILITY,
}

impl AsRef<str> for GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum::SAMPLERUNSPECIFIED => "SAMPLER_UNSPECIFIED",
            GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum::OFF => "OFF",
            GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum::PROBABILITY => "PROBABILITY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SAMPLER_UNSPECIFIED" => Ok(GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum::SAMPLERUNSPECIFIED),
           "OFF" => Ok(GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum::OFF),
           "PROBABILITY" => Ok(GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum::PROBABILITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1SecurityActionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Only an ENABLED SecurityAction is enforced. An ENABLED SecurityAction past its expiration time will not be enforced.
pub enum GoogleCloudApigeeV1SecurityActionStateEnum {
    

    /// The default value. This only exists for forward compatibility. A create request with this value will be rejected.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// An ENABLED SecurityAction is actively enforced if the `expiration_time` is in the future.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// A disabled SecurityAction is never enforced.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for GoogleCloudApigeeV1SecurityActionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1SecurityActionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudApigeeV1SecurityActionStateEnum::ENABLED => "ENABLED",
            GoogleCloudApigeeV1SecurityActionStateEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1SecurityActionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1SecurityActionStateEnum::STATEUNSPECIFIED),
           "ENABLED" => Ok(GoogleCloudApigeeV1SecurityActionStateEnum::ENABLED),
           "DISABLED" => Ok(GoogleCloudApigeeV1SecurityActionStateEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1SecurityActionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1SecurityAssessmentResultResourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Type of this resource.
pub enum GoogleCloudApigeeV1SecurityAssessmentResultResourceTypeEnum {
    

    /// ResourceType not specified.
    ///
    /// "RESOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="RESOURCE_TYPE_UNSPECIFIED")]
    RESOURCETYPEUNSPECIFIED,
    

    /// Resource is an Apigee Proxy.
    ///
    /// "API_PROXY"
    #[serde(rename="API_PROXY")]
    APIPROXY,
}

impl AsRef<str> for GoogleCloudApigeeV1SecurityAssessmentResultResourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1SecurityAssessmentResultResourceTypeEnum::RESOURCETYPEUNSPECIFIED => "RESOURCE_TYPE_UNSPECIFIED",
            GoogleCloudApigeeV1SecurityAssessmentResultResourceTypeEnum::APIPROXY => "API_PROXY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1SecurityAssessmentResultResourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_TYPE_UNSPECIFIED" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultResourceTypeEnum::RESOURCETYPEUNSPECIFIED),
           "API_PROXY" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultResourceTypeEnum::APIPROXY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1SecurityAssessmentResultResourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The severity of the assessment.
pub enum GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum {
    

    /// Severity is not defined.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Severity is low.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Severity is medium.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Severity is high.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Severity is none.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Severity represents no risk
    ///
    /// "NO_RISK"
    #[serde(rename="NO_RISK")]
    NORISK,
    

    /// Severity is minimal
    ///
    /// "MINIMAL"
    #[serde(rename="MINIMAL")]
    MINIMAL,
}

impl AsRef<str> for GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::LOW => "LOW",
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::MEDIUM => "MEDIUM",
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::HIGH => "HIGH",
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::NONE => "NONE",
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::NORISK => "NO_RISK",
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::MINIMAL => "MINIMAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::SEVERITYUNSPECIFIED),
           "LOW" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::LOW),
           "MEDIUM" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::MEDIUM),
           "HIGH" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::HIGH),
           "NONE" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::NONE),
           "NO_RISK" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::NORISK),
           "MINIMAL" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum::MINIMAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1SecurityAssessmentResultScoringResultSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationVerdictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Verdict indicates the assessment result.
pub enum GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationVerdictEnum {
    

    /// The verdict is unspecified.
    ///
    /// "VERDICT_UNSPECIFIED"
    #[serde(rename="VERDICT_UNSPECIFIED")]
    VERDICTUNSPECIFIED,
    

    /// The assessment has passed.
    ///
    /// "PASS"
    #[serde(rename="PASS")]
    PASS,
    

    /// The assessment has failed.
    ///
    /// "FAIL"
    #[serde(rename="FAIL")]
    FAIL,
}

impl AsRef<str> for GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationVerdictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationVerdictEnum::VERDICTUNSPECIFIED => "VERDICT_UNSPECIFIED",
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationVerdictEnum::PASS => "PASS",
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationVerdictEnum::FAIL => "FAIL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationVerdictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERDICT_UNSPECIFIED" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationVerdictEnum::VERDICTUNSPECIFIED),
           "PASS" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationVerdictEnum::PASS),
           "FAIL" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationVerdictEnum::FAIL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationVerdictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The weight of the assessment which was set in the profile.
pub enum GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum {
    

    /// The weight is unspecified.
    ///
    /// "WEIGHT_UNSPECIFIED"
    #[serde(rename="WEIGHT_UNSPECIFIED")]
    WEIGHTUNSPECIFIED,
    

    /// The weight is minor.
    ///
    /// "MINOR"
    #[serde(rename="MINOR")]
    MINOR,
    

    /// The weight is moderate.
    ///
    /// "MODERATE"
    #[serde(rename="MODERATE")]
    MODERATE,
    

    /// The weight is major.
    ///
    /// "MAJOR"
    #[serde(rename="MAJOR")]
    MAJOR,
}

impl AsRef<str> for GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum::WEIGHTUNSPECIFIED => "WEIGHT_UNSPECIFIED",
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum::MINOR => "MINOR",
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum::MODERATE => "MODERATE",
            GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum::MAJOR => "MAJOR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WEIGHT_UNSPECIFIED" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum::WEIGHTUNSPECIFIED),
           "MINOR" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum::MINOR),
           "MODERATE" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum::MODERATE),
           "MAJOR" => Ok(GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum::MAJOR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1SecurityAssessmentResultScoringResultAssessmentRecommendationWeightEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1SecurityIncidentObservabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Indicates if the user archived this incident.
pub enum GoogleCloudApigeeV1SecurityIncidentObservabilityEnum {
    

    /// The incident observability is unspecified.
    ///
    /// "OBSERVABILITY_UNSPECIFIED"
    #[serde(rename="OBSERVABILITY_UNSPECIFIED")]
    OBSERVABILITYUNSPECIFIED,
    

    /// The incident is currently active. Can change to this status from archived.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The incident is currently archived and was archived by the customer.
    ///
    /// "ARCHIVED"
    #[serde(rename="ARCHIVED")]
    ARCHIVED,
}

impl AsRef<str> for GoogleCloudApigeeV1SecurityIncidentObservabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1SecurityIncidentObservabilityEnum::OBSERVABILITYUNSPECIFIED => "OBSERVABILITY_UNSPECIFIED",
            GoogleCloudApigeeV1SecurityIncidentObservabilityEnum::ACTIVE => "ACTIVE",
            GoogleCloudApigeeV1SecurityIncidentObservabilityEnum::ARCHIVED => "ARCHIVED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1SecurityIncidentObservabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OBSERVABILITY_UNSPECIFIED" => Ok(GoogleCloudApigeeV1SecurityIncidentObservabilityEnum::OBSERVABILITYUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudApigeeV1SecurityIncidentObservabilityEnum::ACTIVE),
           "ARCHIVED" => Ok(GoogleCloudApigeeV1SecurityIncidentObservabilityEnum::ARCHIVED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1SecurityIncidentObservabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Risk level of the incident.
pub enum GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum {
    

    /// Risk Level Unspecified.
    ///
    /// "RISK_LEVEL_UNSPECIFIED"
    #[serde(rename="RISK_LEVEL_UNSPECIFIED")]
    RISKLEVELUNSPECIFIED,
    

    /// Risk level of the incident is low.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Risk level of the incident is moderate.
    ///
    /// "MODERATE"
    #[serde(rename="MODERATE")]
    MODERATE,
    

    /// Risk level of the incident is severe.
    ///
    /// "SEVERE"
    #[serde(rename="SEVERE")]
    SEVERE,
}

impl AsRef<str> for GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum::RISKLEVELUNSPECIFIED => "RISK_LEVEL_UNSPECIFIED",
            GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum::LOW => "LOW",
            GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum::MODERATE => "MODERATE",
            GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum::SEVERE => "SEVERE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RISK_LEVEL_UNSPECIFIED" => Ok(GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum::RISKLEVELUNSPECIFIED),
           "LOW" => Ok(GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum::LOW),
           "MODERATE" => Ok(GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum::MODERATE),
           "SEVERE" => Ok(GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum::SEVERE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1SecurityIncidentRiskLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1TargetServerProtocolEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The protocol used by this TargetServer.
pub enum GoogleCloudApigeeV1TargetServerProtocolEnum {
    

    /// UNSPECIFIED defaults to HTTP for backwards compatibility.
    ///
    /// "PROTOCOL_UNSPECIFIED"
    #[serde(rename="PROTOCOL_UNSPECIFIED")]
    PROTOCOLUNSPECIFIED,
    

    /// The TargetServer uses HTTP.
    ///
    /// "HTTP"
    #[serde(rename="HTTP")]
    HTTP,
    

    /// The TargetSever uses HTTP2.
    ///
    /// "HTTP2"
    #[serde(rename="HTTP2")]
    HTTP2,
    

    /// The TargetServer uses GRPC.
    ///
    /// "GRPC_TARGET"
    #[serde(rename="GRPC_TARGET")]
    GRPCTARGET,
    

    /// GRPC TargetServer to be used in ExternalCallout Policy. Prefer to use EXTERNAL_CALLOUT instead. TODO(b/266125112) deprecate once EXTERNAL _CALLOUT generally available.
    ///
    /// "GRPC"
    #[serde(rename="GRPC")]
    GRPC,
    

    /// The TargetServer is to be used in the ExternalCallout Policy
    ///
    /// "EXTERNAL_CALLOUT"
    #[serde(rename="EXTERNAL_CALLOUT")]
    EXTERNALCALLOUT,
}

impl AsRef<str> for GoogleCloudApigeeV1TargetServerProtocolEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1TargetServerProtocolEnum::PROTOCOLUNSPECIFIED => "PROTOCOL_UNSPECIFIED",
            GoogleCloudApigeeV1TargetServerProtocolEnum::HTTP => "HTTP",
            GoogleCloudApigeeV1TargetServerProtocolEnum::HTTP2 => "HTTP2",
            GoogleCloudApigeeV1TargetServerProtocolEnum::GRPCTARGET => "GRPC_TARGET",
            GoogleCloudApigeeV1TargetServerProtocolEnum::GRPC => "GRPC",
            GoogleCloudApigeeV1TargetServerProtocolEnum::EXTERNALCALLOUT => "EXTERNAL_CALLOUT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1TargetServerProtocolEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTOCOL_UNSPECIFIED" => Ok(GoogleCloudApigeeV1TargetServerProtocolEnum::PROTOCOLUNSPECIFIED),
           "HTTP" => Ok(GoogleCloudApigeeV1TargetServerProtocolEnum::HTTP),
           "HTTP2" => Ok(GoogleCloudApigeeV1TargetServerProtocolEnum::HTTP2),
           "GRPC_TARGET" => Ok(GoogleCloudApigeeV1TargetServerProtocolEnum::GRPCTARGET),
           "GRPC" => Ok(GoogleCloudApigeeV1TargetServerProtocolEnum::GRPC),
           "EXTERNAL_CALLOUT" => Ok(GoogleCloudApigeeV1TargetServerProtocolEnum::EXTERNALCALLOUT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1TargetServerProtocolEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1TargetServerConfigProtocolEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The protocol used by this target server.
pub enum GoogleCloudApigeeV1TargetServerConfigProtocolEnum {
    

    /// UNSPECIFIED defaults to HTTP for backwards compatibility.
    ///
    /// "PROTOCOL_UNSPECIFIED"
    #[serde(rename="PROTOCOL_UNSPECIFIED")]
    PROTOCOLUNSPECIFIED,
    

    /// The TargetServer uses HTTP.
    ///
    /// "HTTP"
    #[serde(rename="HTTP")]
    HTTP,
    

    /// The TargetSever uses HTTP2.
    ///
    /// "HTTP2"
    #[serde(rename="HTTP2")]
    HTTP2,
    

    /// The TargetServer uses GRPC.
    ///
    /// "GRPC_TARGET"
    #[serde(rename="GRPC_TARGET")]
    GRPCTARGET,
    

    /// GRPC TargetServer to be used in ExternalCallout Policy. Prefer to use EXTERNAL_CALLOUT instead. TODO(b/266125112) deprecate once EXTERNAL _CALLOUT generally available.
    ///
    /// "GRPC"
    #[serde(rename="GRPC")]
    GRPC,
    

    /// The TargetServer is to be used in the ExternalCallout Policy
    ///
    /// "EXTERNAL_CALLOUT"
    #[serde(rename="EXTERNAL_CALLOUT")]
    EXTERNALCALLOUT,
}

impl AsRef<str> for GoogleCloudApigeeV1TargetServerConfigProtocolEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1TargetServerConfigProtocolEnum::PROTOCOLUNSPECIFIED => "PROTOCOL_UNSPECIFIED",
            GoogleCloudApigeeV1TargetServerConfigProtocolEnum::HTTP => "HTTP",
            GoogleCloudApigeeV1TargetServerConfigProtocolEnum::HTTP2 => "HTTP2",
            GoogleCloudApigeeV1TargetServerConfigProtocolEnum::GRPCTARGET => "GRPC_TARGET",
            GoogleCloudApigeeV1TargetServerConfigProtocolEnum::GRPC => "GRPC",
            GoogleCloudApigeeV1TargetServerConfigProtocolEnum::EXTERNALCALLOUT => "EXTERNAL_CALLOUT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1TargetServerConfigProtocolEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTOCOL_UNSPECIFIED" => Ok(GoogleCloudApigeeV1TargetServerConfigProtocolEnum::PROTOCOLUNSPECIFIED),
           "HTTP" => Ok(GoogleCloudApigeeV1TargetServerConfigProtocolEnum::HTTP),
           "HTTP2" => Ok(GoogleCloudApigeeV1TargetServerConfigProtocolEnum::HTTP2),
           "GRPC_TARGET" => Ok(GoogleCloudApigeeV1TargetServerConfigProtocolEnum::GRPCTARGET),
           "GRPC" => Ok(GoogleCloudApigeeV1TargetServerConfigProtocolEnum::GRPC),
           "EXTERNAL_CALLOUT" => Ok(GoogleCloudApigeeV1TargetServerConfigProtocolEnum::EXTERNALCALLOUT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1TargetServerConfigProtocolEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1TraceConfigExporterEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Exporter that is used to view the distributed trace captured using OpenCensus. An exporter sends traces to any backend that is capable of consuming them. Recorded spans can be exported by registered exporters.
pub enum GoogleCloudApigeeV1TraceConfigExporterEnum {
    

    /// Exporter unspecified
    ///
    /// "EXPORTER_UNSPECIFIED"
    #[serde(rename="EXPORTER_UNSPECIFIED")]
    EXPORTERUNSPECIFIED,
    

    /// Jaeger exporter
    ///
    /// "JAEGER"
    #[serde(rename="JAEGER")]
    JAEGER,
    

    /// Cloudtrace exporter
    ///
    /// "CLOUD_TRACE"
    #[serde(rename="CLOUD_TRACE")]
    CLOUDTRACE,
}

impl AsRef<str> for GoogleCloudApigeeV1TraceConfigExporterEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1TraceConfigExporterEnum::EXPORTERUNSPECIFIED => "EXPORTER_UNSPECIFIED",
            GoogleCloudApigeeV1TraceConfigExporterEnum::JAEGER => "JAEGER",
            GoogleCloudApigeeV1TraceConfigExporterEnum::CLOUDTRACE => "CLOUD_TRACE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1TraceConfigExporterEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPORTER_UNSPECIFIED" => Ok(GoogleCloudApigeeV1TraceConfigExporterEnum::EXPORTERUNSPECIFIED),
           "JAEGER" => Ok(GoogleCloudApigeeV1TraceConfigExporterEnum::JAEGER),
           "CLOUD_TRACE" => Ok(GoogleCloudApigeeV1TraceConfigExporterEnum::CLOUDTRACE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1TraceConfigExporterEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1TraceSamplingConfigSamplerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sampler of distributed tracing. OFF is the default value.
pub enum GoogleCloudApigeeV1TraceSamplingConfigSamplerEnum {
    

    /// Sampler unspecified.
    ///
    /// "SAMPLER_UNSPECIFIED"
    #[serde(rename="SAMPLER_UNSPECIFIED")]
    SAMPLERUNSPECIFIED,
    

    /// OFF means distributed trace is disabled, or the sampling probability is 0.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
    

    /// PROBABILITY means traces are captured on a probability that defined by sampling_rate. The sampling rate is limited to 0 to 0.5 when this is set.
    ///
    /// "PROBABILITY"
    #[serde(rename="PROBABILITY")]
    PROBABILITY,
}

impl AsRef<str> for GoogleCloudApigeeV1TraceSamplingConfigSamplerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1TraceSamplingConfigSamplerEnum::SAMPLERUNSPECIFIED => "SAMPLER_UNSPECIFIED",
            GoogleCloudApigeeV1TraceSamplingConfigSamplerEnum::OFF => "OFF",
            GoogleCloudApigeeV1TraceSamplingConfigSamplerEnum::PROBABILITY => "PROBABILITY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1TraceSamplingConfigSamplerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SAMPLER_UNSPECIFIED" => Ok(GoogleCloudApigeeV1TraceSamplingConfigSamplerEnum::SAMPLERUNSPECIFIED),
           "OFF" => Ok(GoogleCloudApigeeV1TraceSamplingConfigSamplerEnum::OFF),
           "PROBABILITY" => Ok(GoogleCloudApigeeV1TraceSamplingConfigSamplerEnum::PROBABILITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1TraceSamplingConfigSamplerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudApigeeV1UpdateErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status code.
pub enum GoogleCloudApigeeV1UpdateErrorCodeEnum {
    

    /// Not an error; returned on success. HTTP Mapping: 200 OK
    ///
    /// "OK"
    #[serde(rename="OK")]
    OK,
    

    /// The operation was cancelled, typically by the caller. HTTP Mapping: 499 Client Closed Request
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// Unknown error. For example, this error may be returned when a `Status` value received from another address space belongs to an error space that is not known in this address space. Also errors raised by APIs that do not return enough error information may be converted to this error. HTTP Mapping: 500 Internal Server Error
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The client specified an invalid argument. Note that this differs from `FAILED_PRECONDITION`. `INVALID_ARGUMENT` indicates arguments that are problematic regardless of the state of the system (e.g., a malformed file name). HTTP Mapping: 400 Bad Request
    ///
    /// "INVALID_ARGUMENT"
    #[serde(rename="INVALID_ARGUMENT")]
    INVALIDARGUMENT,
    

    /// The deadline expired before the operation could complete. For operations that change the state of the system, this error may be returned even if the operation has completed successfully. For example, a successful response from a server could have been delayed long enough for the deadline to expire. HTTP Mapping: 504 Gateway Timeout
    ///
    /// "DEADLINE_EXCEEDED"
    #[serde(rename="DEADLINE_EXCEEDED")]
    DEADLINEEXCEEDED,
    

    /// Some requested entity (e.g., file or directory) was not found. Note to server developers: if a request is denied for an entire class of users, such as gradual feature rollout or undocumented allowlist, `NOT_FOUND` may be used. If a request is denied for some users within a class of users, such as user-based access control, `PERMISSION_DENIED` must be used. HTTP Mapping: 404 Not Found
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
    

    /// The entity that a client attempted to create (e.g., file or directory) already exists. HTTP Mapping: 409 Conflict
    ///
    /// "ALREADY_EXISTS"
    #[serde(rename="ALREADY_EXISTS")]
    ALREADYEXISTS,
    

    /// The caller does not have permission to execute the specified operation. `PERMISSION_DENIED` must not be used for rejections caused by exhausting some resource (use `RESOURCE_EXHAUSTED` instead for those errors). `PERMISSION_DENIED` must not be used if the caller can not be identified (use `UNAUTHENTICATED` instead for those errors). This error code does not imply the request is valid or the requested entity exists or satisfies other pre-conditions. HTTP Mapping: 403 Forbidden
    ///
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    

    /// The request does not have valid authentication credentials for the operation. HTTP Mapping: 401 Unauthorized
    ///
    /// "UNAUTHENTICATED"
    #[serde(rename="UNAUTHENTICATED")]
    UNAUTHENTICATED,
    

    /// Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file system is out of space. HTTP Mapping: 429 Too Many Requests
    ///
    /// "RESOURCE_EXHAUSTED"
    #[serde(rename="RESOURCE_EXHAUSTED")]
    RESOURCEEXHAUSTED,
    

    /// The operation was rejected because the system is not in a state required for the operation's execution. For example, the directory to be deleted is non-empty, an rmdir operation is applied to a non-directory, etc. Service implementors can use the following guidelines to decide between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`: (a) Use `UNAVAILABLE` if the client can retry just the failing call. (b) Use `ABORTED` if the client should retry at a higher level. For example, when a client-specified test-and-set fails, indicating the client should restart a read-modify-write sequence. (c) Use `FAILED_PRECONDITION` if the client should not retry until the system state has been explicitly fixed. For example, if an "rmdir" fails because the directory is non-empty, `FAILED_PRECONDITION` should be returned since the client should not retry unless the files are deleted from the directory. HTTP Mapping: 400 Bad Request
    ///
    /// "FAILED_PRECONDITION"
    #[serde(rename="FAILED_PRECONDITION")]
    FAILEDPRECONDITION,
    

    /// The operation was aborted, typically due to a concurrency issue such as a sequencer check failure or transaction abort. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 409 Conflict
    ///
    /// "ABORTED"
    #[serde(rename="ABORTED")]
    ABORTED,
    

    /// The operation was attempted past the valid range. E.g., seeking or reading past end-of-file. Unlike `INVALID_ARGUMENT`, this error indicates a problem that may be fixed if the system state changes. For example, a 32-bit file system will generate `INVALID_ARGUMENT` if asked to read at an offset that is not in the range [0,2^32-1], but it will generate `OUT_OF_RANGE` if asked to read from an offset past the current file size. There is a fair bit of overlap between `FAILED_PRECONDITION` and `OUT_OF_RANGE`. We recommend using `OUT_OF_RANGE` (the more specific error) when it applies so that callers who are iterating through a space can easily look for an `OUT_OF_RANGE` error to detect when they are done. HTTP Mapping: 400 Bad Request
    ///
    /// "OUT_OF_RANGE"
    #[serde(rename="OUT_OF_RANGE")]
    OUTOFRANGE,
    

    /// The operation is not implemented or is not supported/enabled in this service. HTTP Mapping: 501 Not Implemented
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Internal errors. This means that some invariants expected by the underlying system have been broken. This error code is reserved for serious errors. HTTP Mapping: 500 Internal Server Error
    ///
    /// "INTERNAL"
    #[serde(rename="INTERNAL")]
    INTERNAL,
    

    /// The service is currently unavailable. This is most likely a transient condition, which can be corrected by retrying with a backoff. Note that it is not always safe to retry non-idempotent operations. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 503 Service Unavailable
    ///
    /// "UNAVAILABLE"
    #[serde(rename="UNAVAILABLE")]
    UNAVAILABLE,
    

    /// Unrecoverable data loss or corruption. HTTP Mapping: 500 Internal Server Error
    ///
    /// "DATA_LOSS"
    #[serde(rename="DATA_LOSS")]
    DATALOSS,
}

impl AsRef<str> for GoogleCloudApigeeV1UpdateErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudApigeeV1UpdateErrorCodeEnum::OK => "OK",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::CANCELLED => "CANCELLED",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::UNKNOWN => "UNKNOWN",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::INVALIDARGUMENT => "INVALID_ARGUMENT",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::DEADLINEEXCEEDED => "DEADLINE_EXCEEDED",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::NOTFOUND => "NOT_FOUND",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::ALREADYEXISTS => "ALREADY_EXISTS",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::UNAUTHENTICATED => "UNAUTHENTICATED",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::RESOURCEEXHAUSTED => "RESOURCE_EXHAUSTED",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::FAILEDPRECONDITION => "FAILED_PRECONDITION",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::ABORTED => "ABORTED",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::OUTOFRANGE => "OUT_OF_RANGE",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::INTERNAL => "INTERNAL",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::UNAVAILABLE => "UNAVAILABLE",
            GoogleCloudApigeeV1UpdateErrorCodeEnum::DATALOSS => "DATA_LOSS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudApigeeV1UpdateErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OK" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::OK),
           "CANCELLED" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::CANCELLED),
           "UNKNOWN" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::UNKNOWN),
           "INVALID_ARGUMENT" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::INVALIDARGUMENT),
           "DEADLINE_EXCEEDED" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::DEADLINEEXCEEDED),
           "NOT_FOUND" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::NOTFOUND),
           "ALREADY_EXISTS" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::ALREADYEXISTS),
           "PERMISSION_DENIED" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::PERMISSIONDENIED),
           "UNAUTHENTICATED" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::UNAUTHENTICATED),
           "RESOURCE_EXHAUSTED" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::RESOURCEEXHAUSTED),
           "FAILED_PRECONDITION" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::FAILEDPRECONDITION),
           "ABORTED" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::ABORTED),
           "OUT_OF_RANGE" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::OUTOFRANGE),
           "UNIMPLEMENTED" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::UNIMPLEMENTED),
           "INTERNAL" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::INTERNAL),
           "UNAVAILABLE" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::UNAVAILABLE),
           "DATA_LOSS" => Ok(GoogleCloudApigeeV1UpdateErrorCodeEnum::DATALOSS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudApigeeV1UpdateErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIamV1AuditLogConfigLogTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The log type that this config enables.
pub enum GoogleIamV1AuditLogConfigLogTypeEnum {
    

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

impl AsRef<str> for GoogleIamV1AuditLogConfigLogTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIamV1AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            GoogleIamV1AuditLogConfigLogTypeEnum::ADMINREAD => "ADMIN_READ",
            GoogleIamV1AuditLogConfigLogTypeEnum::DATAWRITE => "DATA_WRITE",
            GoogleIamV1AuditLogConfigLogTypeEnum::DATAREAD => "DATA_READ",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIamV1AuditLogConfigLogTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED),
           "ADMIN_READ" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::ADMINREAD),
           "DATA_WRITE" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::DATAWRITE),
           "DATA_READ" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::DATAREAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIamV1AuditLogConfigLogTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the rate plans (`DRAFT`, `PUBLISHED`) that you want to display.
pub enum OrganizationStateEnum {
    

    /// State of the rate plan is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Rate plan is in draft mode and only visible to API providers.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Rate plan is published and will become visible to developers for the configured duration (between `startTime` and `endTime`).
    ///
    /// "PUBLISHED"
    #[serde(rename="PUBLISHED")]
    PUBLISHED,
}

impl AsRef<str> for OrganizationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            OrganizationStateEnum::DRAFT => "DRAFT",
            OrganizationStateEnum::PUBLISHED => "PUBLISHED",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(OrganizationStateEnum::STATEUNSPECIFIED),
           "DRAFT" => Ok(OrganizationStateEnum::DRAFT),
           "PUBLISHED" => Ok(OrganizationStateEnum::PUBLISHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// When set to FULL, additional details about the specific deployments receiving traffic will be included in the IngressConfig response's RoutingRules.
pub enum OrganizationViewEnum {
    

    /// The default/unset value. The API will default to the BASIC view.
    ///
    /// "INGRESS_CONFIG_VIEW_UNSPECIFIED"
    #[serde(rename="INGRESS_CONFIG_VIEW_UNSPECIFIED")]
    INGRESSCONFIGVIEWUNSPECIFIED,
    

    /// Include all ingress config data necessary for the runtime to configure ingress, but no more. Routing rules will include only basepath and destination environment. This the default value.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Include all ingress config data, including internal debug info for each routing rule such as the proxy claiming a particular basepath and when the routing rule first appeared in the env group.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for OrganizationViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationViewEnum::INGRESSCONFIGVIEWUNSPECIFIED => "INGRESS_CONFIG_VIEW_UNSPECIFIED",
            OrganizationViewEnum::BASIC => "BASIC",
            OrganizationViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INGRESS_CONFIG_VIEW_UNSPECIFIED" => Ok(OrganizationViewEnum::INGRESSCONFIGVIEWUNSPECIFIED),
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


// region OrganizationRetentionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. This setting is applicable only for organizations that are soft-deleted (i.e., BillingType is not EVALUATION). It controls how long Organization data will be retained after the initial delete operation completes. During this period, the Organization may be restored to its last known state. After this period, the Organization will no longer be able to be restored. **Note: During the data retention period specified using this field, the Apigee organization cannot be recreated in the same GCP project.**
pub enum OrganizationRetentionEnum {
    

    /// Default data retention setting of seven days will be applied.
    ///
    /// "DELETION_RETENTION_UNSPECIFIED"
    #[serde(rename="DELETION_RETENTION_UNSPECIFIED")]
    DELETIONRETENTIONUNSPECIFIED,
    

    /// Organization data will be retained for the minimum period of 24 hours.
    ///
    /// "MINIMUM"
    #[serde(rename="MINIMUM")]
    MINIMUM,
}

impl AsRef<str> for OrganizationRetentionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationRetentionEnum::DELETIONRETENTIONUNSPECIFIED => "DELETION_RETENTION_UNSPECIFIED",
            OrganizationRetentionEnum::MINIMUM => "MINIMUM",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationRetentionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DELETION_RETENTION_UNSPECIFIED" => Ok(OrganizationRetentionEnum::DELETIONRETENTIONUNSPECIFIED),
           "MINIMUM" => Ok(OrganizationRetentionEnum::MINIMUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationRetentionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


