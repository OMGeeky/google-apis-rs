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


// region EndpointMatcherMetadataLabelMatcherMetadataLabelMatchCriteriaEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how matching should be done. Supported values are: MATCH_ANY: At least one of the Labels specified in the matcher should match the metadata presented by xDS client. MATCH_ALL: The metadata presented by the xDS client should contain all of the labels specified here. The selection is determined based on the best match. For example, suppose there are three EndpointPolicy resources P1, P2 and P3 and if P1 has a the matcher as MATCH_ANY , P2 has MATCH_ALL , and P3 has MATCH_ALL . If a client with label connects, the config from P1 will be selected. If a client with label connects, the config from P2 will be selected. If a client with label connects, the config from P3 will be selected. If there is more than one best match, (for example, if a config P4 with selector exists and if a client with label connects), pick up the one with older creation time.
pub enum EndpointMatcherMetadataLabelMatcherMetadataLabelMatchCriteriaEnum {
    

    /// Default value. Should not be used.
    ///
    /// "METADATA_LABEL_MATCH_CRITERIA_UNSPECIFIED"
    #[serde(rename="METADATA_LABEL_MATCH_CRITERIA_UNSPECIFIED")]
    METADATALABELMATCHCRITERIAUNSPECIFIED,
    

    /// At least one of the Labels specified in the matcher should match the metadata presented by xDS client.
    ///
    /// "MATCH_ANY"
    #[serde(rename="MATCH_ANY")]
    MATCHANY,
    

    /// The metadata presented by the xDS client should contain all of the labels specified here.
    ///
    /// "MATCH_ALL"
    #[serde(rename="MATCH_ALL")]
    MATCHALL,
}

impl AsRef<str> for EndpointMatcherMetadataLabelMatcherMetadataLabelMatchCriteriaEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EndpointMatcherMetadataLabelMatcherMetadataLabelMatchCriteriaEnum::METADATALABELMATCHCRITERIAUNSPECIFIED => "METADATA_LABEL_MATCH_CRITERIA_UNSPECIFIED",
            EndpointMatcherMetadataLabelMatcherMetadataLabelMatchCriteriaEnum::MATCHANY => "MATCH_ANY",
            EndpointMatcherMetadataLabelMatcherMetadataLabelMatchCriteriaEnum::MATCHALL => "MATCH_ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for EndpointMatcherMetadataLabelMatcherMetadataLabelMatchCriteriaEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METADATA_LABEL_MATCH_CRITERIA_UNSPECIFIED" => Ok(EndpointMatcherMetadataLabelMatcherMetadataLabelMatchCriteriaEnum::METADATALABELMATCHCRITERIAUNSPECIFIED),
           "MATCH_ANY" => Ok(EndpointMatcherMetadataLabelMatcherMetadataLabelMatchCriteriaEnum::MATCHANY),
           "MATCH_ALL" => Ok(EndpointMatcherMetadataLabelMatcherMetadataLabelMatchCriteriaEnum::MATCHALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EndpointMatcherMetadataLabelMatcherMetadataLabelMatchCriteriaEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EndpointPolicyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of endpoint policy. This is primarily used to validate the configuration.
pub enum EndpointPolicyTypeEnum {
    

    /// Default value. Must not be used.
    ///
    /// "ENDPOINT_POLICY_TYPE_UNSPECIFIED"
    #[serde(rename="ENDPOINT_POLICY_TYPE_UNSPECIFIED")]
    ENDPOINTPOLICYTYPEUNSPECIFIED,
    

    /// Represents a proxy deployed as a sidecar.
    ///
    /// "SIDECAR_PROXY"
    #[serde(rename="SIDECAR_PROXY")]
    SIDECARPROXY,
    

    /// Represents a proxyless gRPC backend.
    ///
    /// "GRPC_SERVER"
    #[serde(rename="GRPC_SERVER")]
    GRPCSERVER,
}

impl AsRef<str> for EndpointPolicyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EndpointPolicyTypeEnum::ENDPOINTPOLICYTYPEUNSPECIFIED => "ENDPOINT_POLICY_TYPE_UNSPECIFIED",
            EndpointPolicyTypeEnum::SIDECARPROXY => "SIDECAR_PROXY",
            EndpointPolicyTypeEnum::GRPCSERVER => "GRPC_SERVER",
        }
    }
}

impl std::convert::TryFrom< &str> for EndpointPolicyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENDPOINT_POLICY_TYPE_UNSPECIFIED" => Ok(EndpointPolicyTypeEnum::ENDPOINTPOLICYTYPEUNSPECIFIED),
           "SIDECAR_PROXY" => Ok(EndpointPolicyTypeEnum::SIDECARPROXY),
           "GRPC_SERVER" => Ok(EndpointPolicyTypeEnum::GRPCSERVER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EndpointPolicyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExtensionChainExtensionSupportedEventsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A set of events during request or response processing for which this extension is called. This field is required for the `LbTrafficExtension` resource. It's not relevant for the `LbRouteExtension` resource.
pub enum ExtensionChainExtensionSupportedEventsEnum {
    

    /// Unspecified value. Do not use.
    ///
    /// "EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="EVENT_TYPE_UNSPECIFIED")]
    EVENTTYPEUNSPECIFIED,
    

    /// If included in `supported_events`, the extension is called when the HTTP request headers arrive.
    ///
    /// "REQUEST_HEADERS"
    #[serde(rename="REQUEST_HEADERS")]
    REQUESTHEADERS,
    

    /// If included in `supported_events`, the extension is called when the HTTP request body arrives.
    ///
    /// "REQUEST_BODY"
    #[serde(rename="REQUEST_BODY")]
    REQUESTBODY,
    

    /// If included in `supported_events`, the extension is called when the HTTP response headers arrive.
    ///
    /// "RESPONSE_HEADERS"
    #[serde(rename="RESPONSE_HEADERS")]
    RESPONSEHEADERS,
    

    /// If included in `supported_events`, the extension is called when the HTTP response body arrives.
    ///
    /// "RESPONSE_BODY"
    #[serde(rename="RESPONSE_BODY")]
    RESPONSEBODY,
    

    /// If included in `supported_events`, the extension is called when the HTTP request trailers arrives.
    ///
    /// "REQUEST_TRAILERS"
    #[serde(rename="REQUEST_TRAILERS")]
    REQUESTTRAILERS,
    

    /// If included in `supported_events`, the extension is called when the HTTP response trailers arrives.
    ///
    /// "RESPONSE_TRAILERS"
    #[serde(rename="RESPONSE_TRAILERS")]
    RESPONSETRAILERS,
}

impl AsRef<str> for ExtensionChainExtensionSupportedEventsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExtensionChainExtensionSupportedEventsEnum::EVENTTYPEUNSPECIFIED => "EVENT_TYPE_UNSPECIFIED",
            ExtensionChainExtensionSupportedEventsEnum::REQUESTHEADERS => "REQUEST_HEADERS",
            ExtensionChainExtensionSupportedEventsEnum::REQUESTBODY => "REQUEST_BODY",
            ExtensionChainExtensionSupportedEventsEnum::RESPONSEHEADERS => "RESPONSE_HEADERS",
            ExtensionChainExtensionSupportedEventsEnum::RESPONSEBODY => "RESPONSE_BODY",
            ExtensionChainExtensionSupportedEventsEnum::REQUESTTRAILERS => "REQUEST_TRAILERS",
            ExtensionChainExtensionSupportedEventsEnum::RESPONSETRAILERS => "RESPONSE_TRAILERS",
        }
    }
}

impl std::convert::TryFrom< &str> for ExtensionChainExtensionSupportedEventsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_TYPE_UNSPECIFIED" => Ok(ExtensionChainExtensionSupportedEventsEnum::EVENTTYPEUNSPECIFIED),
           "REQUEST_HEADERS" => Ok(ExtensionChainExtensionSupportedEventsEnum::REQUESTHEADERS),
           "REQUEST_BODY" => Ok(ExtensionChainExtensionSupportedEventsEnum::REQUESTBODY),
           "RESPONSE_HEADERS" => Ok(ExtensionChainExtensionSupportedEventsEnum::RESPONSEHEADERS),
           "RESPONSE_BODY" => Ok(ExtensionChainExtensionSupportedEventsEnum::RESPONSEBODY),
           "REQUEST_TRAILERS" => Ok(ExtensionChainExtensionSupportedEventsEnum::REQUESTTRAILERS),
           "RESPONSE_TRAILERS" => Ok(ExtensionChainExtensionSupportedEventsEnum::RESPONSETRAILERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExtensionChainExtensionSupportedEventsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GatewayEnvoyHeadersEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Determines if envoy will insert internal debug headers into upstream requests. Other Envoy headers may still be injected. By default, envoy will not insert any debug headers.
pub enum GatewayEnvoyHeadersEnum {
    

    /// Defaults to NONE.
    ///
    /// "ENVOY_HEADERS_UNSPECIFIED"
    #[serde(rename="ENVOY_HEADERS_UNSPECIFIED")]
    ENVOYHEADERSUNSPECIFIED,
    

    /// Suppress envoy debug headers.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Envoy will insert default internal debug headers into upstream requests: x-envoy-attempt-count x-envoy-is-timeout-retry x-envoy-expected-rq-timeout-ms x-envoy-original-path x-envoy-upstream-stream-duration-ms
    ///
    /// "DEBUG_HEADERS"
    #[serde(rename="DEBUG_HEADERS")]
    DEBUGHEADERS,
}

impl AsRef<str> for GatewayEnvoyHeadersEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GatewayEnvoyHeadersEnum::ENVOYHEADERSUNSPECIFIED => "ENVOY_HEADERS_UNSPECIFIED",
            GatewayEnvoyHeadersEnum::NONE => "NONE",
            GatewayEnvoyHeadersEnum::DEBUGHEADERS => "DEBUG_HEADERS",
        }
    }
}

impl std::convert::TryFrom< &str> for GatewayEnvoyHeadersEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENVOY_HEADERS_UNSPECIFIED" => Ok(GatewayEnvoyHeadersEnum::ENVOYHEADERSUNSPECIFIED),
           "NONE" => Ok(GatewayEnvoyHeadersEnum::NONE),
           "DEBUG_HEADERS" => Ok(GatewayEnvoyHeadersEnum::DEBUGHEADERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GatewayEnvoyHeadersEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GatewayIpVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The IP Version that will be used by this gateway. Valid options are IPV4 or IPV6. Default is IPV4.
pub enum GatewayIpVersionEnum {
    

    /// The type when IP version is not specified. Defaults to IPV4.
    ///
    /// "IP_VERSION_UNSPECIFIED"
    #[serde(rename="IP_VERSION_UNSPECIFIED")]
    IPVERSIONUNSPECIFIED,
    

    /// The type for IP version 4.
    ///
    /// "IPV4"
    #[serde(rename="IPV4")]
    IPV4,
    

    /// The type for IP version 6.
    ///
    /// "IPV6"
    #[serde(rename="IPV6")]
    IPV6,
}

impl AsRef<str> for GatewayIpVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GatewayIpVersionEnum::IPVERSIONUNSPECIFIED => "IP_VERSION_UNSPECIFIED",
            GatewayIpVersionEnum::IPV4 => "IPV4",
            GatewayIpVersionEnum::IPV6 => "IPV6",
        }
    }
}

impl std::convert::TryFrom< &str> for GatewayIpVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IP_VERSION_UNSPECIFIED" => Ok(GatewayIpVersionEnum::IPVERSIONUNSPECIFIED),
           "IPV4" => Ok(GatewayIpVersionEnum::IPV4),
           "IPV6" => Ok(GatewayIpVersionEnum::IPV6),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GatewayIpVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GatewayTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The type of the customer managed gateway. This field is required. If unspecified, an error is returned.
pub enum GatewayTypeEnum {
    

    /// The type of the customer managed gateway is unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The type of the customer managed gateway is TrafficDirector Open Mesh.
    ///
    /// "OPEN_MESH"
    #[serde(rename="OPEN_MESH")]
    OPENMESH,
    

    /// The type of the customer managed gateway is SecureWebGateway (SWG).
    ///
    /// "SECURE_WEB_GATEWAY"
    #[serde(rename="SECURE_WEB_GATEWAY")]
    SECUREWEBGATEWAY,
}

impl AsRef<str> for GatewayTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GatewayTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GatewayTypeEnum::OPENMESH => "OPEN_MESH",
            GatewayTypeEnum::SECUREWEBGATEWAY => "SECURE_WEB_GATEWAY",
        }
    }
}

impl std::convert::TryFrom< &str> for GatewayTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GatewayTypeEnum::TYPEUNSPECIFIED),
           "OPEN_MESH" => Ok(GatewayTypeEnum::OPENMESH),
           "SECURE_WEB_GATEWAY" => Ok(GatewayTypeEnum::SECUREWEBGATEWAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GatewayTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GrpcRouteHeaderMatchTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies how to match against the value of the header. If not specified, a default value of EXACT is used.
pub enum GrpcRouteHeaderMatchTypeEnum {
    

    /// Unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Will only match the exact value provided.
    ///
    /// "EXACT"
    #[serde(rename="EXACT")]
    EXACT,
    

    /// Will match paths conforming to the prefix specified by value. RE2 syntax is supported.
    ///
    /// "REGULAR_EXPRESSION"
    #[serde(rename="REGULAR_EXPRESSION")]
    REGULAREXPRESSION,
}

impl AsRef<str> for GrpcRouteHeaderMatchTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GrpcRouteHeaderMatchTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GrpcRouteHeaderMatchTypeEnum::EXACT => "EXACT",
            GrpcRouteHeaderMatchTypeEnum::REGULAREXPRESSION => "REGULAR_EXPRESSION",
        }
    }
}

impl std::convert::TryFrom< &str> for GrpcRouteHeaderMatchTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GrpcRouteHeaderMatchTypeEnum::TYPEUNSPECIFIED),
           "EXACT" => Ok(GrpcRouteHeaderMatchTypeEnum::EXACT),
           "REGULAR_EXPRESSION" => Ok(GrpcRouteHeaderMatchTypeEnum::REGULAREXPRESSION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GrpcRouteHeaderMatchTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GrpcRouteMethodMatchTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies how to match against the name. If not specified, a default value of "EXACT" is used.
pub enum GrpcRouteMethodMatchTypeEnum {
    

    /// Unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Will only match the exact name provided.
    ///
    /// "EXACT"
    #[serde(rename="EXACT")]
    EXACT,
    

    /// Will interpret grpc_method and grpc_service as regexes. RE2 syntax is supported.
    ///
    /// "REGULAR_EXPRESSION"
    #[serde(rename="REGULAR_EXPRESSION")]
    REGULAREXPRESSION,
}

impl AsRef<str> for GrpcRouteMethodMatchTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GrpcRouteMethodMatchTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GrpcRouteMethodMatchTypeEnum::EXACT => "EXACT",
            GrpcRouteMethodMatchTypeEnum::REGULAREXPRESSION => "REGULAR_EXPRESSION",
        }
    }
}

impl std::convert::TryFrom< &str> for GrpcRouteMethodMatchTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GrpcRouteMethodMatchTypeEnum::TYPEUNSPECIFIED),
           "EXACT" => Ok(GrpcRouteMethodMatchTypeEnum::EXACT),
           "REGULAR_EXPRESSION" => Ok(GrpcRouteMethodMatchTypeEnum::REGULAREXPRESSION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GrpcRouteMethodMatchTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HttpRouteRedirectResponseCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The HTTP Status code to use for the redirect.
pub enum HttpRouteRedirectResponseCodeEnum {
    

    /// Default value
    ///
    /// "RESPONSE_CODE_UNSPECIFIED"
    #[serde(rename="RESPONSE_CODE_UNSPECIFIED")]
    RESPONSECODEUNSPECIFIED,
    

    /// Corresponds to 301.
    ///
    /// "MOVED_PERMANENTLY_DEFAULT"
    #[serde(rename="MOVED_PERMANENTLY_DEFAULT")]
    MOVEDPERMANENTLYDEFAULT,
    

    /// Corresponds to 302.
    ///
    /// "FOUND"
    #[serde(rename="FOUND")]
    FOUND,
    

    /// Corresponds to 303.
    ///
    /// "SEE_OTHER"
    #[serde(rename="SEE_OTHER")]
    SEEOTHER,
    

    /// Corresponds to 307. In this case, the request method will be retained.
    ///
    /// "TEMPORARY_REDIRECT"
    #[serde(rename="TEMPORARY_REDIRECT")]
    TEMPORARYREDIRECT,
    

    /// Corresponds to 308. In this case, the request method will be retained.
    ///
    /// "PERMANENT_REDIRECT"
    #[serde(rename="PERMANENT_REDIRECT")]
    PERMANENTREDIRECT,
}

impl AsRef<str> for HttpRouteRedirectResponseCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HttpRouteRedirectResponseCodeEnum::RESPONSECODEUNSPECIFIED => "RESPONSE_CODE_UNSPECIFIED",
            HttpRouteRedirectResponseCodeEnum::MOVEDPERMANENTLYDEFAULT => "MOVED_PERMANENTLY_DEFAULT",
            HttpRouteRedirectResponseCodeEnum::FOUND => "FOUND",
            HttpRouteRedirectResponseCodeEnum::SEEOTHER => "SEE_OTHER",
            HttpRouteRedirectResponseCodeEnum::TEMPORARYREDIRECT => "TEMPORARY_REDIRECT",
            HttpRouteRedirectResponseCodeEnum::PERMANENTREDIRECT => "PERMANENT_REDIRECT",
        }
    }
}

impl std::convert::TryFrom< &str> for HttpRouteRedirectResponseCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESPONSE_CODE_UNSPECIFIED" => Ok(HttpRouteRedirectResponseCodeEnum::RESPONSECODEUNSPECIFIED),
           "MOVED_PERMANENTLY_DEFAULT" => Ok(HttpRouteRedirectResponseCodeEnum::MOVEDPERMANENTLYDEFAULT),
           "FOUND" => Ok(HttpRouteRedirectResponseCodeEnum::FOUND),
           "SEE_OTHER" => Ok(HttpRouteRedirectResponseCodeEnum::SEEOTHER),
           "TEMPORARY_REDIRECT" => Ok(HttpRouteRedirectResponseCodeEnum::TEMPORARYREDIRECT),
           "PERMANENT_REDIRECT" => Ok(HttpRouteRedirectResponseCodeEnum::PERMANENTREDIRECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HttpRouteRedirectResponseCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LbRouteExtensionLoadBalancingSchemeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service).
pub enum LbRouteExtensionLoadBalancingSchemeEnum {
    

    /// Default value. Do not use.
    ///
    /// "LOAD_BALANCING_SCHEME_UNSPECIFIED"
    #[serde(rename="LOAD_BALANCING_SCHEME_UNSPECIFIED")]
    LOADBALANCINGSCHEMEUNSPECIFIED,
    

    /// Signifies that this is used for Internal HTTP(S) Load Balancing.
    ///
    /// "INTERNAL_MANAGED"
    #[serde(rename="INTERNAL_MANAGED")]
    INTERNALMANAGED,
    

    /// Signifies that this is used for External Managed HTTP(S) Load Balancing.
    ///
    /// "EXTERNAL_MANAGED"
    #[serde(rename="EXTERNAL_MANAGED")]
    EXTERNALMANAGED,
}

impl AsRef<str> for LbRouteExtensionLoadBalancingSchemeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LbRouteExtensionLoadBalancingSchemeEnum::LOADBALANCINGSCHEMEUNSPECIFIED => "LOAD_BALANCING_SCHEME_UNSPECIFIED",
            LbRouteExtensionLoadBalancingSchemeEnum::INTERNALMANAGED => "INTERNAL_MANAGED",
            LbRouteExtensionLoadBalancingSchemeEnum::EXTERNALMANAGED => "EXTERNAL_MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for LbRouteExtensionLoadBalancingSchemeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOAD_BALANCING_SCHEME_UNSPECIFIED" => Ok(LbRouteExtensionLoadBalancingSchemeEnum::LOADBALANCINGSCHEMEUNSPECIFIED),
           "INTERNAL_MANAGED" => Ok(LbRouteExtensionLoadBalancingSchemeEnum::INTERNALMANAGED),
           "EXTERNAL_MANAGED" => Ok(LbRouteExtensionLoadBalancingSchemeEnum::EXTERNALMANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LbRouteExtensionLoadBalancingSchemeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LbTrafficExtensionLoadBalancingSchemeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service).
pub enum LbTrafficExtensionLoadBalancingSchemeEnum {
    

    /// Default value. Do not use.
    ///
    /// "LOAD_BALANCING_SCHEME_UNSPECIFIED"
    #[serde(rename="LOAD_BALANCING_SCHEME_UNSPECIFIED")]
    LOADBALANCINGSCHEMEUNSPECIFIED,
    

    /// Signifies that this is used for Internal HTTP(S) Load Balancing.
    ///
    /// "INTERNAL_MANAGED"
    #[serde(rename="INTERNAL_MANAGED")]
    INTERNALMANAGED,
    

    /// Signifies that this is used for External Managed HTTP(S) Load Balancing.
    ///
    /// "EXTERNAL_MANAGED"
    #[serde(rename="EXTERNAL_MANAGED")]
    EXTERNALMANAGED,
}

impl AsRef<str> for LbTrafficExtensionLoadBalancingSchemeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LbTrafficExtensionLoadBalancingSchemeEnum::LOADBALANCINGSCHEMEUNSPECIFIED => "LOAD_BALANCING_SCHEME_UNSPECIFIED",
            LbTrafficExtensionLoadBalancingSchemeEnum::INTERNALMANAGED => "INTERNAL_MANAGED",
            LbTrafficExtensionLoadBalancingSchemeEnum::EXTERNALMANAGED => "EXTERNAL_MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for LbTrafficExtensionLoadBalancingSchemeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOAD_BALANCING_SCHEME_UNSPECIFIED" => Ok(LbTrafficExtensionLoadBalancingSchemeEnum::LOADBALANCINGSCHEMEUNSPECIFIED),
           "INTERNAL_MANAGED" => Ok(LbTrafficExtensionLoadBalancingSchemeEnum::INTERNALMANAGED),
           "EXTERNAL_MANAGED" => Ok(LbTrafficExtensionLoadBalancingSchemeEnum::EXTERNALMANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LbTrafficExtensionLoadBalancingSchemeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MeshEnvoyHeadersEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Determines if envoy will insert internal debug headers into upstream requests. Other Envoy headers may still be injected. By default, envoy will not insert any debug headers.
pub enum MeshEnvoyHeadersEnum {
    

    /// Defaults to NONE.
    ///
    /// "ENVOY_HEADERS_UNSPECIFIED"
    #[serde(rename="ENVOY_HEADERS_UNSPECIFIED")]
    ENVOYHEADERSUNSPECIFIED,
    

    /// Suppress envoy debug headers.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Envoy will insert default internal debug headers into upstream requests: x-envoy-attempt-count x-envoy-is-timeout-retry x-envoy-expected-rq-timeout-ms x-envoy-original-path x-envoy-upstream-stream-duration-ms
    ///
    /// "DEBUG_HEADERS"
    #[serde(rename="DEBUG_HEADERS")]
    DEBUGHEADERS,
}

impl AsRef<str> for MeshEnvoyHeadersEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MeshEnvoyHeadersEnum::ENVOYHEADERSUNSPECIFIED => "ENVOY_HEADERS_UNSPECIFIED",
            MeshEnvoyHeadersEnum::NONE => "NONE",
            MeshEnvoyHeadersEnum::DEBUGHEADERS => "DEBUG_HEADERS",
        }
    }
}

impl std::convert::TryFrom< &str> for MeshEnvoyHeadersEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENVOY_HEADERS_UNSPECIFIED" => Ok(MeshEnvoyHeadersEnum::ENVOYHEADERSUNSPECIFIED),
           "NONE" => Ok(MeshEnvoyHeadersEnum::NONE),
           "DEBUG_HEADERS" => Ok(MeshEnvoyHeadersEnum::DEBUGHEADERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MeshEnvoyHeadersEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceLbPolicyLoadBalancingAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of load balancing algorithm to be used. The default behavior is WATERFALL_BY_REGION.
pub enum ServiceLbPolicyLoadBalancingAlgorithmEnum {
    

    /// The type of the loadbalancing algorithm is unspecified.
    ///
    /// "LOAD_BALANCING_ALGORITHM_UNSPECIFIED"
    #[serde(rename="LOAD_BALANCING_ALGORITHM_UNSPECIFIED")]
    LOADBALANCINGALGORITHMUNSPECIFIED,
    

    /// Balance traffic across all backends across the world proportionally based on capacity.
    ///
    /// "SPRAY_TO_WORLD"
    #[serde(rename="SPRAY_TO_WORLD")]
    SPRAYTOWORLD,
    

    /// Direct traffic to the nearest region with endpoints and capacity before spilling over to other regions and spread the traffic from each client to all the MIGs/NEGs in a region.
    ///
    /// "SPRAY_TO_REGION"
    #[serde(rename="SPRAY_TO_REGION")]
    SPRAYTOREGION,
    

    /// Direct traffic to the nearest region with endpoints and capacity before spilling over to other regions. All MIGs/NEGs within a region are evenly loaded but each client might not spread the traffic to all the MIGs/NEGs in the region.
    ///
    /// "WATERFALL_BY_REGION"
    #[serde(rename="WATERFALL_BY_REGION")]
    WATERFALLBYREGION,
    

    /// Attempt to keep traffic in a single zone closest to the client, before spilling over to other zones.
    ///
    /// "WATERFALL_BY_ZONE"
    #[serde(rename="WATERFALL_BY_ZONE")]
    WATERFALLBYZONE,
}

impl AsRef<str> for ServiceLbPolicyLoadBalancingAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceLbPolicyLoadBalancingAlgorithmEnum::LOADBALANCINGALGORITHMUNSPECIFIED => "LOAD_BALANCING_ALGORITHM_UNSPECIFIED",
            ServiceLbPolicyLoadBalancingAlgorithmEnum::SPRAYTOWORLD => "SPRAY_TO_WORLD",
            ServiceLbPolicyLoadBalancingAlgorithmEnum::SPRAYTOREGION => "SPRAY_TO_REGION",
            ServiceLbPolicyLoadBalancingAlgorithmEnum::WATERFALLBYREGION => "WATERFALL_BY_REGION",
            ServiceLbPolicyLoadBalancingAlgorithmEnum::WATERFALLBYZONE => "WATERFALL_BY_ZONE",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceLbPolicyLoadBalancingAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOAD_BALANCING_ALGORITHM_UNSPECIFIED" => Ok(ServiceLbPolicyLoadBalancingAlgorithmEnum::LOADBALANCINGALGORITHMUNSPECIFIED),
           "SPRAY_TO_WORLD" => Ok(ServiceLbPolicyLoadBalancingAlgorithmEnum::SPRAYTOWORLD),
           "SPRAY_TO_REGION" => Ok(ServiceLbPolicyLoadBalancingAlgorithmEnum::SPRAYTOREGION),
           "WATERFALL_BY_REGION" => Ok(ServiceLbPolicyLoadBalancingAlgorithmEnum::WATERFALLBYREGION),
           "WATERFALL_BY_ZONE" => Ok(ServiceLbPolicyLoadBalancingAlgorithmEnum::WATERFALLBYZONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceLbPolicyLoadBalancingAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


