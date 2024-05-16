use super::*;



// region AbortInfoCauseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Causes that the analysis is aborted.
pub enum AbortInfoCauseEnum {
    

    /// Cause is unspecified.
    ///
    /// "CAUSE_UNSPECIFIED"
    #[serde(rename="CAUSE_UNSPECIFIED")]
    CAUSEUNSPECIFIED,
    

    /// Aborted due to unknown network. Deprecated, not used in the new tests.
    ///
    /// "UNKNOWN_NETWORK"
    #[serde(rename="UNKNOWN_NETWORK")]
    UNKNOWNNETWORK,
    

    /// Aborted because no project information can be derived from the test input. Deprecated, not used in the new tests.
    ///
    /// "UNKNOWN_PROJECT"
    #[serde(rename="UNKNOWN_PROJECT")]
    UNKNOWNPROJECT,
    

    /// Aborted because traffic is sent from a public IP to an instance without an external IP. Deprecated, not used in the new tests.
    ///
    /// "NO_EXTERNAL_IP"
    #[serde(rename="NO_EXTERNAL_IP")]
    NOEXTERNALIP,
    

    /// Aborted because none of the traces matches destination information specified in the input test request. Deprecated, not used in the new tests.
    ///
    /// "UNINTENDED_DESTINATION"
    #[serde(rename="UNINTENDED_DESTINATION")]
    UNINTENDEDDESTINATION,
    

    /// Aborted because the source endpoint could not be found. Deprecated, not used in the new tests.
    ///
    /// "SOURCE_ENDPOINT_NOT_FOUND"
    #[serde(rename="SOURCE_ENDPOINT_NOT_FOUND")]
    SOURCEENDPOINTNOTFOUND,
    

    /// Aborted because the source network does not match the source endpoint. Deprecated, not used in the new tests.
    ///
    /// "MISMATCHED_SOURCE_NETWORK"
    #[serde(rename="MISMATCHED_SOURCE_NETWORK")]
    MISMATCHEDSOURCENETWORK,
    

    /// Aborted because the destination endpoint could not be found. Deprecated, not used in the new tests.
    ///
    /// "DESTINATION_ENDPOINT_NOT_FOUND"
    #[serde(rename="DESTINATION_ENDPOINT_NOT_FOUND")]
    DESTINATIONENDPOINTNOTFOUND,
    

    /// Aborted because the destination network does not match the destination endpoint. Deprecated, not used in the new tests.
    ///
    /// "MISMATCHED_DESTINATION_NETWORK"
    #[serde(rename="MISMATCHED_DESTINATION_NETWORK")]
    MISMATCHEDDESTINATIONNETWORK,
    

    /// Aborted because no endpoint with the packet's destination IP address is found.
    ///
    /// "UNKNOWN_IP"
    #[serde(rename="UNKNOWN_IP")]
    UNKNOWNIP,
    

    /// Aborted because the source IP address doesn't belong to any of the subnets of the source VPC network.
    ///
    /// "SOURCE_IP_ADDRESS_NOT_IN_SOURCE_NETWORK"
    #[serde(rename="SOURCE_IP_ADDRESS_NOT_IN_SOURCE_NETWORK")]
    SOURCEIPADDRESSNOTINSOURCENETWORK,
    

    /// Aborted because user lacks permission to access all or part of the network configurations required to run the test.
    ///
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    

    /// Aborted because user lacks permission to access Cloud NAT configs required to run the test.
    ///
    /// "PERMISSION_DENIED_NO_CLOUD_NAT_CONFIGS"
    #[serde(rename="PERMISSION_DENIED_NO_CLOUD_NAT_CONFIGS")]
    PERMISSIONDENIEDNOCLOUDNATCONFIGS,
    

    /// Aborted because user lacks permission to access Network endpoint group endpoint configs required to run the test.
    ///
    /// "PERMISSION_DENIED_NO_NEG_ENDPOINT_CONFIGS"
    #[serde(rename="PERMISSION_DENIED_NO_NEG_ENDPOINT_CONFIGS")]
    PERMISSIONDENIEDNONEGENDPOINTCONFIGS,
    

    /// Aborted because no valid source or destination endpoint is derived from the input test request.
    ///
    /// "NO_SOURCE_LOCATION"
    #[serde(rename="NO_SOURCE_LOCATION")]
    NOSOURCELOCATION,
    

    /// Aborted because the source or destination endpoint specified in the request is invalid. Some examples: - The request might contain malformed resource URI, project ID, or IP address. - The request might contain inconsistent information (for example, the request might include both the instance and the network, but the instance might not have a NIC in that network).
    ///
    /// "INVALID_ARGUMENT"
    #[serde(rename="INVALID_ARGUMENT")]
    INVALIDARGUMENT,
    

    /// Aborted because the number of steps in the trace exceeds a certain limit. It might be caused by a routing loop.
    ///
    /// "TRACE_TOO_LONG"
    #[serde(rename="TRACE_TOO_LONG")]
    TRACETOOLONG,
    

    /// Aborted due to internal server error.
    ///
    /// "INTERNAL_ERROR"
    #[serde(rename="INTERNAL_ERROR")]
    INTERNALERROR,
    

    /// Aborted because the test scenario is not supported.
    ///
    /// "UNSUPPORTED"
    #[serde(rename="UNSUPPORTED")]
    UNSUPPORTED,
    

    /// Aborted because the source and destination resources have no common IP version.
    ///
    /// "MISMATCHED_IP_VERSION"
    #[serde(rename="MISMATCHED_IP_VERSION")]
    MISMATCHEDIPVERSION,
    

    /// Aborted because the connection between the control plane and the node of the source cluster is initiated by the node and managed by the Konnectivity proxy.
    ///
    /// "GKE_KONNECTIVITY_PROXY_UNSUPPORTED"
    #[serde(rename="GKE_KONNECTIVITY_PROXY_UNSUPPORTED")]
    GKEKONNECTIVITYPROXYUNSUPPORTED,
    

    /// Aborted because expected resource configuration was missing.
    ///
    /// "RESOURCE_CONFIG_NOT_FOUND"
    #[serde(rename="RESOURCE_CONFIG_NOT_FOUND")]
    RESOURCECONFIGNOTFOUND,
    

    /// Aborted because expected VM instance configuration was missing.
    ///
    /// "VM_INSTANCE_CONFIG_NOT_FOUND"
    #[serde(rename="VM_INSTANCE_CONFIG_NOT_FOUND")]
    VMINSTANCECONFIGNOTFOUND,
    

    /// Aborted because expected network configuration was missing.
    ///
    /// "NETWORK_CONFIG_NOT_FOUND"
    #[serde(rename="NETWORK_CONFIG_NOT_FOUND")]
    NETWORKCONFIGNOTFOUND,
    

    /// Aborted because expected firewall configuration was missing.
    ///
    /// "FIREWALL_CONFIG_NOT_FOUND"
    #[serde(rename="FIREWALL_CONFIG_NOT_FOUND")]
    FIREWALLCONFIGNOTFOUND,
    

    /// Aborted because expected route configuration was missing.
    ///
    /// "ROUTE_CONFIG_NOT_FOUND"
    #[serde(rename="ROUTE_CONFIG_NOT_FOUND")]
    ROUTECONFIGNOTFOUND,
    

    /// Aborted because a PSC endpoint selection for the Google-managed service is ambiguous (several PSC endpoints satisfy test input).
    ///
    /// "GOOGLE_MANAGED_SERVICE_AMBIGUOUS_PSC_ENDPOINT"
    #[serde(rename="GOOGLE_MANAGED_SERVICE_AMBIGUOUS_PSC_ENDPOINT")]
    GOOGLEMANAGEDSERVICEAMBIGUOUSPSCENDPOINT,
    

    /// Aborted because tests with a PSC-based Cloud SQL instance as a source are not supported.
    ///
    /// "SOURCE_PSC_CLOUD_SQL_UNSUPPORTED"
    #[serde(rename="SOURCE_PSC_CLOUD_SQL_UNSUPPORTED")]
    SOURCEPSCCLOUDSQLUNSUPPORTED,
    

    /// Aborted because tests with a forwarding rule as a source are not supported.
    ///
    /// "SOURCE_FORWARDING_RULE_UNSUPPORTED"
    #[serde(rename="SOURCE_FORWARDING_RULE_UNSUPPORTED")]
    SOURCEFORWARDINGRULEUNSUPPORTED,
    

    /// Aborted because one of the endpoints is a non-routable IP address (loopback, link-local, etc).
    ///
    /// "NON_ROUTABLE_IP_ADDRESS"
    #[serde(rename="NON_ROUTABLE_IP_ADDRESS")]
    NONROUTABLEIPADDRESS,
    

    /// Aborted due to an unknown issue in the Google-managed project.
    ///
    /// "UNKNOWN_ISSUE_IN_GOOGLE_MANAGED_PROJECT"
    #[serde(rename="UNKNOWN_ISSUE_IN_GOOGLE_MANAGED_PROJECT")]
    UNKNOWNISSUEINGOOGLEMANAGEDPROJECT,
    

    /// Aborted due to an unsupported configuration of the Google-managed project.
    ///
    /// "UNSUPPORTED_GOOGLE_MANAGED_PROJECT_CONFIG"
    #[serde(rename="UNSUPPORTED_GOOGLE_MANAGED_PROJECT_CONFIG")]
    UNSUPPORTEDGOOGLEMANAGEDPROJECTCONFIG,
}

impl AsRef<str> for AbortInfoCauseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AbortInfoCauseEnum::CAUSEUNSPECIFIED => "CAUSE_UNSPECIFIED",
            AbortInfoCauseEnum::UNKNOWNNETWORK => "UNKNOWN_NETWORK",
            AbortInfoCauseEnum::UNKNOWNPROJECT => "UNKNOWN_PROJECT",
            AbortInfoCauseEnum::NOEXTERNALIP => "NO_EXTERNAL_IP",
            AbortInfoCauseEnum::UNINTENDEDDESTINATION => "UNINTENDED_DESTINATION",
            AbortInfoCauseEnum::SOURCEENDPOINTNOTFOUND => "SOURCE_ENDPOINT_NOT_FOUND",
            AbortInfoCauseEnum::MISMATCHEDSOURCENETWORK => "MISMATCHED_SOURCE_NETWORK",
            AbortInfoCauseEnum::DESTINATIONENDPOINTNOTFOUND => "DESTINATION_ENDPOINT_NOT_FOUND",
            AbortInfoCauseEnum::MISMATCHEDDESTINATIONNETWORK => "MISMATCHED_DESTINATION_NETWORK",
            AbortInfoCauseEnum::UNKNOWNIP => "UNKNOWN_IP",
            AbortInfoCauseEnum::SOURCEIPADDRESSNOTINSOURCENETWORK => "SOURCE_IP_ADDRESS_NOT_IN_SOURCE_NETWORK",
            AbortInfoCauseEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            AbortInfoCauseEnum::PERMISSIONDENIEDNOCLOUDNATCONFIGS => "PERMISSION_DENIED_NO_CLOUD_NAT_CONFIGS",
            AbortInfoCauseEnum::PERMISSIONDENIEDNONEGENDPOINTCONFIGS => "PERMISSION_DENIED_NO_NEG_ENDPOINT_CONFIGS",
            AbortInfoCauseEnum::NOSOURCELOCATION => "NO_SOURCE_LOCATION",
            AbortInfoCauseEnum::INVALIDARGUMENT => "INVALID_ARGUMENT",
            AbortInfoCauseEnum::TRACETOOLONG => "TRACE_TOO_LONG",
            AbortInfoCauseEnum::INTERNALERROR => "INTERNAL_ERROR",
            AbortInfoCauseEnum::UNSUPPORTED => "UNSUPPORTED",
            AbortInfoCauseEnum::MISMATCHEDIPVERSION => "MISMATCHED_IP_VERSION",
            AbortInfoCauseEnum::GKEKONNECTIVITYPROXYUNSUPPORTED => "GKE_KONNECTIVITY_PROXY_UNSUPPORTED",
            AbortInfoCauseEnum::RESOURCECONFIGNOTFOUND => "RESOURCE_CONFIG_NOT_FOUND",
            AbortInfoCauseEnum::VMINSTANCECONFIGNOTFOUND => "VM_INSTANCE_CONFIG_NOT_FOUND",
            AbortInfoCauseEnum::NETWORKCONFIGNOTFOUND => "NETWORK_CONFIG_NOT_FOUND",
            AbortInfoCauseEnum::FIREWALLCONFIGNOTFOUND => "FIREWALL_CONFIG_NOT_FOUND",
            AbortInfoCauseEnum::ROUTECONFIGNOTFOUND => "ROUTE_CONFIG_NOT_FOUND",
            AbortInfoCauseEnum::GOOGLEMANAGEDSERVICEAMBIGUOUSPSCENDPOINT => "GOOGLE_MANAGED_SERVICE_AMBIGUOUS_PSC_ENDPOINT",
            AbortInfoCauseEnum::SOURCEPSCCLOUDSQLUNSUPPORTED => "SOURCE_PSC_CLOUD_SQL_UNSUPPORTED",
            AbortInfoCauseEnum::SOURCEFORWARDINGRULEUNSUPPORTED => "SOURCE_FORWARDING_RULE_UNSUPPORTED",
            AbortInfoCauseEnum::NONROUTABLEIPADDRESS => "NON_ROUTABLE_IP_ADDRESS",
            AbortInfoCauseEnum::UNKNOWNISSUEINGOOGLEMANAGEDPROJECT => "UNKNOWN_ISSUE_IN_GOOGLE_MANAGED_PROJECT",
            AbortInfoCauseEnum::UNSUPPORTEDGOOGLEMANAGEDPROJECTCONFIG => "UNSUPPORTED_GOOGLE_MANAGED_PROJECT_CONFIG",
        }
    }
}

impl std::convert::TryFrom< &str> for AbortInfoCauseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CAUSE_UNSPECIFIED" => Ok(AbortInfoCauseEnum::CAUSEUNSPECIFIED),
           "UNKNOWN_NETWORK" => Ok(AbortInfoCauseEnum::UNKNOWNNETWORK),
           "UNKNOWN_PROJECT" => Ok(AbortInfoCauseEnum::UNKNOWNPROJECT),
           "NO_EXTERNAL_IP" => Ok(AbortInfoCauseEnum::NOEXTERNALIP),
           "UNINTENDED_DESTINATION" => Ok(AbortInfoCauseEnum::UNINTENDEDDESTINATION),
           "SOURCE_ENDPOINT_NOT_FOUND" => Ok(AbortInfoCauseEnum::SOURCEENDPOINTNOTFOUND),
           "MISMATCHED_SOURCE_NETWORK" => Ok(AbortInfoCauseEnum::MISMATCHEDSOURCENETWORK),
           "DESTINATION_ENDPOINT_NOT_FOUND" => Ok(AbortInfoCauseEnum::DESTINATIONENDPOINTNOTFOUND),
           "MISMATCHED_DESTINATION_NETWORK" => Ok(AbortInfoCauseEnum::MISMATCHEDDESTINATIONNETWORK),
           "UNKNOWN_IP" => Ok(AbortInfoCauseEnum::UNKNOWNIP),
           "SOURCE_IP_ADDRESS_NOT_IN_SOURCE_NETWORK" => Ok(AbortInfoCauseEnum::SOURCEIPADDRESSNOTINSOURCENETWORK),
           "PERMISSION_DENIED" => Ok(AbortInfoCauseEnum::PERMISSIONDENIED),
           "PERMISSION_DENIED_NO_CLOUD_NAT_CONFIGS" => Ok(AbortInfoCauseEnum::PERMISSIONDENIEDNOCLOUDNATCONFIGS),
           "PERMISSION_DENIED_NO_NEG_ENDPOINT_CONFIGS" => Ok(AbortInfoCauseEnum::PERMISSIONDENIEDNONEGENDPOINTCONFIGS),
           "NO_SOURCE_LOCATION" => Ok(AbortInfoCauseEnum::NOSOURCELOCATION),
           "INVALID_ARGUMENT" => Ok(AbortInfoCauseEnum::INVALIDARGUMENT),
           "TRACE_TOO_LONG" => Ok(AbortInfoCauseEnum::TRACETOOLONG),
           "INTERNAL_ERROR" => Ok(AbortInfoCauseEnum::INTERNALERROR),
           "UNSUPPORTED" => Ok(AbortInfoCauseEnum::UNSUPPORTED),
           "MISMATCHED_IP_VERSION" => Ok(AbortInfoCauseEnum::MISMATCHEDIPVERSION),
           "GKE_KONNECTIVITY_PROXY_UNSUPPORTED" => Ok(AbortInfoCauseEnum::GKEKONNECTIVITYPROXYUNSUPPORTED),
           "RESOURCE_CONFIG_NOT_FOUND" => Ok(AbortInfoCauseEnum::RESOURCECONFIGNOTFOUND),
           "VM_INSTANCE_CONFIG_NOT_FOUND" => Ok(AbortInfoCauseEnum::VMINSTANCECONFIGNOTFOUND),
           "NETWORK_CONFIG_NOT_FOUND" => Ok(AbortInfoCauseEnum::NETWORKCONFIGNOTFOUND),
           "FIREWALL_CONFIG_NOT_FOUND" => Ok(AbortInfoCauseEnum::FIREWALLCONFIGNOTFOUND),
           "ROUTE_CONFIG_NOT_FOUND" => Ok(AbortInfoCauseEnum::ROUTECONFIGNOTFOUND),
           "GOOGLE_MANAGED_SERVICE_AMBIGUOUS_PSC_ENDPOINT" => Ok(AbortInfoCauseEnum::GOOGLEMANAGEDSERVICEAMBIGUOUSPSCENDPOINT),
           "SOURCE_PSC_CLOUD_SQL_UNSUPPORTED" => Ok(AbortInfoCauseEnum::SOURCEPSCCLOUDSQLUNSUPPORTED),
           "SOURCE_FORWARDING_RULE_UNSUPPORTED" => Ok(AbortInfoCauseEnum::SOURCEFORWARDINGRULEUNSUPPORTED),
           "NON_ROUTABLE_IP_ADDRESS" => Ok(AbortInfoCauseEnum::NONROUTABLEIPADDRESS),
           "UNKNOWN_ISSUE_IN_GOOGLE_MANAGED_PROJECT" => Ok(AbortInfoCauseEnum::UNKNOWNISSUEINGOOGLEMANAGEDPROJECT),
           "UNSUPPORTED_GOOGLE_MANAGED_PROJECT_CONFIG" => Ok(AbortInfoCauseEnum::UNSUPPORTEDGOOGLEMANAGEDPROJECTCONFIG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AbortInfoCauseEnum {
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


// region DeliverInfoTargetEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Target type where the packet is delivered to.
pub enum DeliverInfoTargetEnum {
    

    /// Target not specified.
    ///
    /// "TARGET_UNSPECIFIED"
    #[serde(rename="TARGET_UNSPECIFIED")]
    TARGETUNSPECIFIED,
    

    /// Target is a Compute Engine instance.
    ///
    /// "INSTANCE"
    #[serde(rename="INSTANCE")]
    INSTANCE,
    

    /// Target is the internet.
    ///
    /// "INTERNET"
    #[serde(rename="INTERNET")]
    INTERNET,
    

    /// Target is a Google API.
    ///
    /// "GOOGLE_API"
    #[serde(rename="GOOGLE_API")]
    GOOGLEAPI,
    

    /// Target is a Google Kubernetes Engine cluster master.
    ///
    /// "GKE_MASTER"
    #[serde(rename="GKE_MASTER")]
    GKEMASTER,
    

    /// Target is a Cloud SQL instance.
    ///
    /// "CLOUD_SQL_INSTANCE"
    #[serde(rename="CLOUD_SQL_INSTANCE")]
    CLOUDSQLINSTANCE,
    

    /// Target is a published service that uses [Private Service Connect](https://cloud.google.com/vpc/docs/configure-private-service-connect-services).
    ///
    /// "PSC_PUBLISHED_SERVICE"
    #[serde(rename="PSC_PUBLISHED_SERVICE")]
    PSCPUBLISHEDSERVICE,
    

    /// Target is all Google APIs that use [Private Service Connect](https://cloud.google.com/vpc/docs/configure-private-service-connect-apis).
    ///
    /// "PSC_GOOGLE_API"
    #[serde(rename="PSC_GOOGLE_API")]
    PSCGOOGLEAPI,
    

    /// Target is a VPC-SC that uses [Private Service Connect](https://cloud.google.com/vpc/docs/configure-private-service-connect-apis).
    ///
    /// "PSC_VPC_SC"
    #[serde(rename="PSC_VPC_SC")]
    PSCVPCSC,
    

    /// Target is a serverless network endpoint group.
    ///
    /// "SERVERLESS_NEG"
    #[serde(rename="SERVERLESS_NEG")]
    SERVERLESSNEG,
    

    /// Target is a Cloud Storage bucket.
    ///
    /// "STORAGE_BUCKET"
    #[serde(rename="STORAGE_BUCKET")]
    STORAGEBUCKET,
    

    /// Target is a private network. Used only for return traces.
    ///
    /// "PRIVATE_NETWORK"
    #[serde(rename="PRIVATE_NETWORK")]
    PRIVATENETWORK,
    

    /// Target is a Cloud Function. Used only for return traces.
    ///
    /// "CLOUD_FUNCTION"
    #[serde(rename="CLOUD_FUNCTION")]
    CLOUDFUNCTION,
    

    /// Target is a App Engine service version. Used only for return traces.
    ///
    /// "APP_ENGINE_VERSION"
    #[serde(rename="APP_ENGINE_VERSION")]
    APPENGINEVERSION,
    

    /// Target is a Cloud Run revision. Used only for return traces.
    ///
    /// "CLOUD_RUN_REVISION"
    #[serde(rename="CLOUD_RUN_REVISION")]
    CLOUDRUNREVISION,
}

impl AsRef<str> for DeliverInfoTargetEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeliverInfoTargetEnum::TARGETUNSPECIFIED => "TARGET_UNSPECIFIED",
            DeliverInfoTargetEnum::INSTANCE => "INSTANCE",
            DeliverInfoTargetEnum::INTERNET => "INTERNET",
            DeliverInfoTargetEnum::GOOGLEAPI => "GOOGLE_API",
            DeliverInfoTargetEnum::GKEMASTER => "GKE_MASTER",
            DeliverInfoTargetEnum::CLOUDSQLINSTANCE => "CLOUD_SQL_INSTANCE",
            DeliverInfoTargetEnum::PSCPUBLISHEDSERVICE => "PSC_PUBLISHED_SERVICE",
            DeliverInfoTargetEnum::PSCGOOGLEAPI => "PSC_GOOGLE_API",
            DeliverInfoTargetEnum::PSCVPCSC => "PSC_VPC_SC",
            DeliverInfoTargetEnum::SERVERLESSNEG => "SERVERLESS_NEG",
            DeliverInfoTargetEnum::STORAGEBUCKET => "STORAGE_BUCKET",
            DeliverInfoTargetEnum::PRIVATENETWORK => "PRIVATE_NETWORK",
            DeliverInfoTargetEnum::CLOUDFUNCTION => "CLOUD_FUNCTION",
            DeliverInfoTargetEnum::APPENGINEVERSION => "APP_ENGINE_VERSION",
            DeliverInfoTargetEnum::CLOUDRUNREVISION => "CLOUD_RUN_REVISION",
        }
    }
}

impl std::convert::TryFrom< &str> for DeliverInfoTargetEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGET_UNSPECIFIED" => Ok(DeliverInfoTargetEnum::TARGETUNSPECIFIED),
           "INSTANCE" => Ok(DeliverInfoTargetEnum::INSTANCE),
           "INTERNET" => Ok(DeliverInfoTargetEnum::INTERNET),
           "GOOGLE_API" => Ok(DeliverInfoTargetEnum::GOOGLEAPI),
           "GKE_MASTER" => Ok(DeliverInfoTargetEnum::GKEMASTER),
           "CLOUD_SQL_INSTANCE" => Ok(DeliverInfoTargetEnum::CLOUDSQLINSTANCE),
           "PSC_PUBLISHED_SERVICE" => Ok(DeliverInfoTargetEnum::PSCPUBLISHEDSERVICE),
           "PSC_GOOGLE_API" => Ok(DeliverInfoTargetEnum::PSCGOOGLEAPI),
           "PSC_VPC_SC" => Ok(DeliverInfoTargetEnum::PSCVPCSC),
           "SERVERLESS_NEG" => Ok(DeliverInfoTargetEnum::SERVERLESSNEG),
           "STORAGE_BUCKET" => Ok(DeliverInfoTargetEnum::STORAGEBUCKET),
           "PRIVATE_NETWORK" => Ok(DeliverInfoTargetEnum::PRIVATENETWORK),
           "CLOUD_FUNCTION" => Ok(DeliverInfoTargetEnum::CLOUDFUNCTION),
           "APP_ENGINE_VERSION" => Ok(DeliverInfoTargetEnum::APPENGINEVERSION),
           "CLOUD_RUN_REVISION" => Ok(DeliverInfoTargetEnum::CLOUDRUNREVISION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeliverInfoTargetEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DropInfoCauseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Cause that the packet is dropped.
pub enum DropInfoCauseEnum {
    

    /// Cause is unspecified.
    ///
    /// "CAUSE_UNSPECIFIED"
    #[serde(rename="CAUSE_UNSPECIFIED")]
    CAUSEUNSPECIFIED,
    

    /// Destination external address cannot be resolved to a known target. If the address is used in a Google Cloud project, provide the project ID as test input.
    ///
    /// "UNKNOWN_EXTERNAL_ADDRESS"
    #[serde(rename="UNKNOWN_EXTERNAL_ADDRESS")]
    UNKNOWNEXTERNALADDRESS,
    

    /// A Compute Engine instance can only send or receive a packet with a foreign IP address if ip_forward is enabled.
    ///
    /// "FOREIGN_IP_DISALLOWED"
    #[serde(rename="FOREIGN_IP_DISALLOWED")]
    FOREIGNIPDISALLOWED,
    

    /// Dropped due to a firewall rule, unless allowed due to connection tracking.
    ///
    /// "FIREWALL_RULE"
    #[serde(rename="FIREWALL_RULE")]
    FIREWALLRULE,
    

    /// Dropped due to no matching routes.
    ///
    /// "NO_ROUTE"
    #[serde(rename="NO_ROUTE")]
    NOROUTE,
    

    /// Dropped due to invalid route. Route's next hop is a blackhole.
    ///
    /// "ROUTE_BLACKHOLE"
    #[serde(rename="ROUTE_BLACKHOLE")]
    ROUTEBLACKHOLE,
    

    /// Packet is sent to a wrong (unintended) network. Example: you trace a packet from VM1:Network1 to VM2:Network2, however, the route configured in Network1 sends the packet destined for VM2's IP address to Network3.
    ///
    /// "ROUTE_WRONG_NETWORK"
    #[serde(rename="ROUTE_WRONG_NETWORK")]
    ROUTEWRONGNETWORK,
    

    /// Route's next hop IP address cannot be resolved to a GCP resource.
    ///
    /// "ROUTE_NEXT_HOP_IP_ADDRESS_NOT_RESOLVED"
    #[serde(rename="ROUTE_NEXT_HOP_IP_ADDRESS_NOT_RESOLVED")]
    ROUTENEXTHOPIPADDRESSNOTRESOLVED,
    

    /// Route's next hop resource is not found.
    ///
    /// "ROUTE_NEXT_HOP_RESOURCE_NOT_FOUND"
    #[serde(rename="ROUTE_NEXT_HOP_RESOURCE_NOT_FOUND")]
    ROUTENEXTHOPRESOURCENOTFOUND,
    

    /// Route's next hop instance doesn't have a NIC in the route's network.
    ///
    /// "ROUTE_NEXT_HOP_INSTANCE_WRONG_NETWORK"
    #[serde(rename="ROUTE_NEXT_HOP_INSTANCE_WRONG_NETWORK")]
    ROUTENEXTHOPINSTANCEWRONGNETWORK,
    

    /// Route's next hop IP address is not a primary IP address of the next hop instance.
    ///
    /// "ROUTE_NEXT_HOP_INSTANCE_NON_PRIMARY_IP"
    #[serde(rename="ROUTE_NEXT_HOP_INSTANCE_NON_PRIMARY_IP")]
    ROUTENEXTHOPINSTANCENONPRIMARYIP,
    

    /// Route's next hop forwarding rule doesn't match next hop IP address.
    ///
    /// "ROUTE_NEXT_HOP_FORWARDING_RULE_IP_MISMATCH"
    #[serde(rename="ROUTE_NEXT_HOP_FORWARDING_RULE_IP_MISMATCH")]
    ROUTENEXTHOPFORWARDINGRULEIPMISMATCH,
    

    /// Route's next hop VPN tunnel is down (does not have valid IKE SAs).
    ///
    /// "ROUTE_NEXT_HOP_VPN_TUNNEL_NOT_ESTABLISHED"
    #[serde(rename="ROUTE_NEXT_HOP_VPN_TUNNEL_NOT_ESTABLISHED")]
    ROUTENEXTHOPVPNTUNNELNOTESTABLISHED,
    

    /// Route's next hop forwarding rule type is invalid (it's not a forwarding rule of the internal passthrough load balancer).
    ///
    /// "ROUTE_NEXT_HOP_FORWARDING_RULE_TYPE_INVALID"
    #[serde(rename="ROUTE_NEXT_HOP_FORWARDING_RULE_TYPE_INVALID")]
    ROUTENEXTHOPFORWARDINGRULETYPEINVALID,
    

    /// Packet is sent from the Internet to the private IPv6 address.
    ///
    /// "NO_ROUTE_FROM_INTERNET_TO_PRIVATE_IPV6_ADDRESS"
    #[serde(rename="NO_ROUTE_FROM_INTERNET_TO_PRIVATE_IPV6_ADDRESS")]
    NOROUTEFROMINTERNETTOPRIVATEIPV6ADDRESS,
    

    /// The packet does not match a policy-based VPN tunnel local selector.
    ///
    /// "VPN_TUNNEL_LOCAL_SELECTOR_MISMATCH"
    #[serde(rename="VPN_TUNNEL_LOCAL_SELECTOR_MISMATCH")]
    VPNTUNNELLOCALSELECTORMISMATCH,
    

    /// The packet does not match a policy-based VPN tunnel remote selector.
    ///
    /// "VPN_TUNNEL_REMOTE_SELECTOR_MISMATCH"
    #[serde(rename="VPN_TUNNEL_REMOTE_SELECTOR_MISMATCH")]
    VPNTUNNELREMOTESELECTORMISMATCH,
    

    /// Packet with internal destination address sent to the internet gateway.
    ///
    /// "PRIVATE_TRAFFIC_TO_INTERNET"
    #[serde(rename="PRIVATE_TRAFFIC_TO_INTERNET")]
    PRIVATETRAFFICTOINTERNET,
    

    /// Instance with only an internal IP address tries to access Google API and services, but private Google access is not enabled in the subnet.
    ///
    /// "PRIVATE_GOOGLE_ACCESS_DISALLOWED"
    #[serde(rename="PRIVATE_GOOGLE_ACCESS_DISALLOWED")]
    PRIVATEGOOGLEACCESSDISALLOWED,
    

    /// Source endpoint tries to access Google API and services through the VPN tunnel to another network, but Private Google Access needs to be enabled in the source endpoint network.
    ///
    /// "PRIVATE_GOOGLE_ACCESS_VIA_VPN_TUNNEL_UNSUPPORTED"
    #[serde(rename="PRIVATE_GOOGLE_ACCESS_VIA_VPN_TUNNEL_UNSUPPORTED")]
    PRIVATEGOOGLEACCESSVIAVPNTUNNELUNSUPPORTED,
    

    /// Instance with only an internal IP address tries to access external hosts, but Cloud NAT is not enabled in the subnet, unless special configurations on a VM allow this connection.
    ///
    /// "NO_EXTERNAL_ADDRESS"
    #[serde(rename="NO_EXTERNAL_ADDRESS")]
    NOEXTERNALADDRESS,
    

    /// Destination internal address cannot be resolved to a known target. If this is a shared VPC scenario, verify if the service project ID is provided as test input. Otherwise, verify if the IP address is being used in the project.
    ///
    /// "UNKNOWN_INTERNAL_ADDRESS"
    #[serde(rename="UNKNOWN_INTERNAL_ADDRESS")]
    UNKNOWNINTERNALADDRESS,
    

    /// Forwarding rule's protocol and ports do not match the packet header.
    ///
    /// "FORWARDING_RULE_MISMATCH"
    #[serde(rename="FORWARDING_RULE_MISMATCH")]
    FORWARDINGRULEMISMATCH,
    

    /// Forwarding rule does not have backends configured.
    ///
    /// "FORWARDING_RULE_NO_INSTANCES"
    #[serde(rename="FORWARDING_RULE_NO_INSTANCES")]
    FORWARDINGRULENOINSTANCES,
    

    /// Firewalls block the health check probes to the backends and cause the backends to be unavailable for traffic from the load balancer. For more details, see [Health check firewall rules](https://cloud.google.com/load-balancing/docs/health-checks#firewall_rules).
    ///
    /// "FIREWALL_BLOCKING_LOAD_BALANCER_BACKEND_HEALTH_CHECK"
    #[serde(rename="FIREWALL_BLOCKING_LOAD_BALANCER_BACKEND_HEALTH_CHECK")]
    FIREWALLBLOCKINGLOADBALANCERBACKENDHEALTHCHECK,
    

    /// Packet is sent from or to a Compute Engine instance that is not in a running state.
    ///
    /// "INSTANCE_NOT_RUNNING"
    #[serde(rename="INSTANCE_NOT_RUNNING")]
    INSTANCENOTRUNNING,
    

    /// Packet sent from or to a GKE cluster that is not in running state.
    ///
    /// "GKE_CLUSTER_NOT_RUNNING"
    #[serde(rename="GKE_CLUSTER_NOT_RUNNING")]
    GKECLUSTERNOTRUNNING,
    

    /// Packet sent from or to a Cloud SQL instance that is not in running state.
    ///
    /// "CLOUD_SQL_INSTANCE_NOT_RUNNING"
    #[serde(rename="CLOUD_SQL_INSTANCE_NOT_RUNNING")]
    CLOUDSQLINSTANCENOTRUNNING,
    

    /// The type of traffic is blocked and the user cannot configure a firewall rule to enable it. See [Always blocked traffic](https://cloud.google.com/vpc/docs/firewalls#blockedtraffic) for more details.
    ///
    /// "TRAFFIC_TYPE_BLOCKED"
    #[serde(rename="TRAFFIC_TYPE_BLOCKED")]
    TRAFFICTYPEBLOCKED,
    

    /// Access to Google Kubernetes Engine cluster master's endpoint is not authorized. See [Access to the cluster endpoints](https://cloud.google.com/kubernetes-engine/docs/how-to/private-clusters#access_to_the_cluster_endpoints) for more details.
    ///
    /// "GKE_MASTER_UNAUTHORIZED_ACCESS"
    #[serde(rename="GKE_MASTER_UNAUTHORIZED_ACCESS")]
    GKEMASTERUNAUTHORIZEDACCESS,
    

    /// Access to the Cloud SQL instance endpoint is not authorized. See [Authorizing with authorized networks](https://cloud.google.com/sql/docs/mysql/authorize-networks) for more details.
    ///
    /// "CLOUD_SQL_INSTANCE_UNAUTHORIZED_ACCESS"
    #[serde(rename="CLOUD_SQL_INSTANCE_UNAUTHORIZED_ACCESS")]
    CLOUDSQLINSTANCEUNAUTHORIZEDACCESS,
    

    /// Packet was dropped inside Google Kubernetes Engine Service.
    ///
    /// "DROPPED_INSIDE_GKE_SERVICE"
    #[serde(rename="DROPPED_INSIDE_GKE_SERVICE")]
    DROPPEDINSIDEGKESERVICE,
    

    /// Packet was dropped inside Cloud SQL Service.
    ///
    /// "DROPPED_INSIDE_CLOUD_SQL_SERVICE"
    #[serde(rename="DROPPED_INSIDE_CLOUD_SQL_SERVICE")]
    DROPPEDINSIDECLOUDSQLSERVICE,
    

    /// Packet was dropped because there is no peering between the originating network and the Google Managed Services Network.
    ///
    /// "GOOGLE_MANAGED_SERVICE_NO_PEERING"
    #[serde(rename="GOOGLE_MANAGED_SERVICE_NO_PEERING")]
    GOOGLEMANAGEDSERVICENOPEERING,
    

    /// Packet was dropped because the Google-managed service uses Private Service Connect (PSC), but the PSC endpoint is not found in the project.
    ///
    /// "GOOGLE_MANAGED_SERVICE_NO_PSC_ENDPOINT"
    #[serde(rename="GOOGLE_MANAGED_SERVICE_NO_PSC_ENDPOINT")]
    GOOGLEMANAGEDSERVICENOPSCENDPOINT,
    

    /// Packet was dropped because the GKE cluster uses Private Service Connect (PSC), but the PSC endpoint is not found in the project.
    ///
    /// "GKE_PSC_ENDPOINT_MISSING"
    #[serde(rename="GKE_PSC_ENDPOINT_MISSING")]
    GKEPSCENDPOINTMISSING,
    

    /// Packet was dropped because the Cloud SQL instance has neither a private nor a public IP address.
    ///
    /// "CLOUD_SQL_INSTANCE_NO_IP_ADDRESS"
    #[serde(rename="CLOUD_SQL_INSTANCE_NO_IP_ADDRESS")]
    CLOUDSQLINSTANCENOIPADDRESS,
    

    /// Packet was dropped because a GKE cluster private endpoint is unreachable from a region different from the cluster's region.
    ///
    /// "GKE_CONTROL_PLANE_REGION_MISMATCH"
    #[serde(rename="GKE_CONTROL_PLANE_REGION_MISMATCH")]
    GKECONTROLPLANEREGIONMISMATCH,
    

    /// Packet sent from a public GKE cluster control plane to a private IP address.
    ///
    /// "PUBLIC_GKE_CONTROL_PLANE_TO_PRIVATE_DESTINATION"
    #[serde(rename="PUBLIC_GKE_CONTROL_PLANE_TO_PRIVATE_DESTINATION")]
    PUBLICGKECONTROLPLANETOPRIVATEDESTINATION,
    

    /// Packet was dropped because there is no route from a GKE cluster control plane to a destination network.
    ///
    /// "GKE_CONTROL_PLANE_NO_ROUTE"
    #[serde(rename="GKE_CONTROL_PLANE_NO_ROUTE")]
    GKECONTROLPLANENOROUTE,
    

    /// Packet sent from a Cloud SQL instance to an external IP address is not allowed. The Cloud SQL instance is not configured to send packets to external IP addresses.
    ///
    /// "CLOUD_SQL_INSTANCE_NOT_CONFIGURED_FOR_EXTERNAL_TRAFFIC"
    #[serde(rename="CLOUD_SQL_INSTANCE_NOT_CONFIGURED_FOR_EXTERNAL_TRAFFIC")]
    CLOUDSQLINSTANCENOTCONFIGUREDFOREXTERNALTRAFFIC,
    

    /// Packet sent from a Cloud SQL instance with only a public IP address to a private IP address.
    ///
    /// "PUBLIC_CLOUD_SQL_INSTANCE_TO_PRIVATE_DESTINATION"
    #[serde(rename="PUBLIC_CLOUD_SQL_INSTANCE_TO_PRIVATE_DESTINATION")]
    PUBLICCLOUDSQLINSTANCETOPRIVATEDESTINATION,
    

    /// Packet was dropped because there is no route from a Cloud SQL instance to a destination network.
    ///
    /// "CLOUD_SQL_INSTANCE_NO_ROUTE"
    #[serde(rename="CLOUD_SQL_INSTANCE_NO_ROUTE")]
    CLOUDSQLINSTANCENOROUTE,
    

    /// Packet could be dropped because the Cloud Function is not in an active status.
    ///
    /// "CLOUD_FUNCTION_NOT_ACTIVE"
    #[serde(rename="CLOUD_FUNCTION_NOT_ACTIVE")]
    CLOUDFUNCTIONNOTACTIVE,
    

    /// Packet could be dropped because no VPC connector is set.
    ///
    /// "VPC_CONNECTOR_NOT_SET"
    #[serde(rename="VPC_CONNECTOR_NOT_SET")]
    VPCCONNECTORNOTSET,
    

    /// Packet could be dropped because the VPC connector is not in a running state.
    ///
    /// "VPC_CONNECTOR_NOT_RUNNING"
    #[serde(rename="VPC_CONNECTOR_NOT_RUNNING")]
    VPCCONNECTORNOTRUNNING,
    

    /// Packet could be dropped because it was sent from a different region to a regional forwarding without global access.
    ///
    /// "FORWARDING_RULE_REGION_MISMATCH"
    #[serde(rename="FORWARDING_RULE_REGION_MISMATCH")]
    FORWARDINGRULEREGIONMISMATCH,
    

    /// The Private Service Connect endpoint is in a project that is not approved to connect to the service.
    ///
    /// "PSC_CONNECTION_NOT_ACCEPTED"
    #[serde(rename="PSC_CONNECTION_NOT_ACCEPTED")]
    PSCCONNECTIONNOTACCEPTED,
    

    /// The packet is sent to the Private Service Connect endpoint over the peering, but [it's not supported](https://cloud.google.com/vpc/docs/configure-private-service-connect-services#on-premises).
    ///
    /// "PSC_ENDPOINT_ACCESSED_FROM_PEERED_NETWORK"
    #[serde(rename="PSC_ENDPOINT_ACCESSED_FROM_PEERED_NETWORK")]
    PSCENDPOINTACCESSEDFROMPEEREDNETWORK,
    

    /// The packet is sent to the Private Service Connect backend (network endpoint group), but the producer PSC forwarding rule does not have global access enabled.
    ///
    /// "PSC_NEG_PRODUCER_ENDPOINT_NO_GLOBAL_ACCESS"
    #[serde(rename="PSC_NEG_PRODUCER_ENDPOINT_NO_GLOBAL_ACCESS")]
    PSCNEGPRODUCERENDPOINTNOGLOBALACCESS,
    

    /// The packet is sent to the Private Service Connect backend (network endpoint group), but the producer PSC forwarding rule has multiple ports specified.
    ///
    /// "PSC_NEG_PRODUCER_FORWARDING_RULE_MULTIPLE_PORTS"
    #[serde(rename="PSC_NEG_PRODUCER_FORWARDING_RULE_MULTIPLE_PORTS")]
    PSCNEGPRODUCERFORWARDINGRULEMULTIPLEPORTS,
    

    /// The packet is sent to the Private Service Connect backend (network endpoint group) targeting a Cloud SQL service attachment, but this configuration is not supported.
    ///
    /// "CLOUD_SQL_PSC_NEG_UNSUPPORTED"
    #[serde(rename="CLOUD_SQL_PSC_NEG_UNSUPPORTED")]
    CLOUDSQLPSCNEGUNSUPPORTED,
    

    /// No NAT subnets are defined for the PSC service attachment.
    ///
    /// "NO_NAT_SUBNETS_FOR_PSC_SERVICE_ATTACHMENT"
    #[serde(rename="NO_NAT_SUBNETS_FOR_PSC_SERVICE_ATTACHMENT")]
    NONATSUBNETSFORPSCSERVICEATTACHMENT,
    

    /// The packet sent from the hybrid NEG proxy matches a non-dynamic route, but such a configuration is not supported.
    ///
    /// "HYBRID_NEG_NON_DYNAMIC_ROUTE_MATCHED"
    #[serde(rename="HYBRID_NEG_NON_DYNAMIC_ROUTE_MATCHED")]
    HYBRIDNEGNONDYNAMICROUTEMATCHED,
    

    /// The packet sent from the hybrid NEG proxy matches a dynamic route with a next hop in a different region, but such a configuration is not supported.
    ///
    /// "HYBRID_NEG_NON_LOCAL_DYNAMIC_ROUTE_MATCHED"
    #[serde(rename="HYBRID_NEG_NON_LOCAL_DYNAMIC_ROUTE_MATCHED")]
    HYBRIDNEGNONLOCALDYNAMICROUTEMATCHED,
    

    /// Packet sent from a Cloud Run revision that is not ready.
    ///
    /// "CLOUD_RUN_REVISION_NOT_READY"
    #[serde(rename="CLOUD_RUN_REVISION_NOT_READY")]
    CLOUDRUNREVISIONNOTREADY,
    

    /// Packet was dropped inside Private Service Connect service producer.
    ///
    /// "DROPPED_INSIDE_PSC_SERVICE_PRODUCER"
    #[serde(rename="DROPPED_INSIDE_PSC_SERVICE_PRODUCER")]
    DROPPEDINSIDEPSCSERVICEPRODUCER,
    

    /// Packet sent to a load balancer, which requires a proxy-only subnet and the subnet is not found.
    ///
    /// "LOAD_BALANCER_HAS_NO_PROXY_SUBNET"
    #[serde(rename="LOAD_BALANCER_HAS_NO_PROXY_SUBNET")]
    LOADBALANCERHASNOPROXYSUBNET,
    

    /// Packet sent to Cloud Nat without active NAT IPs.
    ///
    /// "CLOUD_NAT_NO_ADDRESSES"
    #[serde(rename="CLOUD_NAT_NO_ADDRESSES")]
    CLOUDNATNOADDRESSES,
    

    /// Packet is stuck in a routing loop.
    ///
    /// "ROUTING_LOOP"
    #[serde(rename="ROUTING_LOOP")]
    ROUTINGLOOP,
}

impl AsRef<str> for DropInfoCauseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DropInfoCauseEnum::CAUSEUNSPECIFIED => "CAUSE_UNSPECIFIED",
            DropInfoCauseEnum::UNKNOWNEXTERNALADDRESS => "UNKNOWN_EXTERNAL_ADDRESS",
            DropInfoCauseEnum::FOREIGNIPDISALLOWED => "FOREIGN_IP_DISALLOWED",
            DropInfoCauseEnum::FIREWALLRULE => "FIREWALL_RULE",
            DropInfoCauseEnum::NOROUTE => "NO_ROUTE",
            DropInfoCauseEnum::ROUTEBLACKHOLE => "ROUTE_BLACKHOLE",
            DropInfoCauseEnum::ROUTEWRONGNETWORK => "ROUTE_WRONG_NETWORK",
            DropInfoCauseEnum::ROUTENEXTHOPIPADDRESSNOTRESOLVED => "ROUTE_NEXT_HOP_IP_ADDRESS_NOT_RESOLVED",
            DropInfoCauseEnum::ROUTENEXTHOPRESOURCENOTFOUND => "ROUTE_NEXT_HOP_RESOURCE_NOT_FOUND",
            DropInfoCauseEnum::ROUTENEXTHOPINSTANCEWRONGNETWORK => "ROUTE_NEXT_HOP_INSTANCE_WRONG_NETWORK",
            DropInfoCauseEnum::ROUTENEXTHOPINSTANCENONPRIMARYIP => "ROUTE_NEXT_HOP_INSTANCE_NON_PRIMARY_IP",
            DropInfoCauseEnum::ROUTENEXTHOPFORWARDINGRULEIPMISMATCH => "ROUTE_NEXT_HOP_FORWARDING_RULE_IP_MISMATCH",
            DropInfoCauseEnum::ROUTENEXTHOPVPNTUNNELNOTESTABLISHED => "ROUTE_NEXT_HOP_VPN_TUNNEL_NOT_ESTABLISHED",
            DropInfoCauseEnum::ROUTENEXTHOPFORWARDINGRULETYPEINVALID => "ROUTE_NEXT_HOP_FORWARDING_RULE_TYPE_INVALID",
            DropInfoCauseEnum::NOROUTEFROMINTERNETTOPRIVATEIPV6ADDRESS => "NO_ROUTE_FROM_INTERNET_TO_PRIVATE_IPV6_ADDRESS",
            DropInfoCauseEnum::VPNTUNNELLOCALSELECTORMISMATCH => "VPN_TUNNEL_LOCAL_SELECTOR_MISMATCH",
            DropInfoCauseEnum::VPNTUNNELREMOTESELECTORMISMATCH => "VPN_TUNNEL_REMOTE_SELECTOR_MISMATCH",
            DropInfoCauseEnum::PRIVATETRAFFICTOINTERNET => "PRIVATE_TRAFFIC_TO_INTERNET",
            DropInfoCauseEnum::PRIVATEGOOGLEACCESSDISALLOWED => "PRIVATE_GOOGLE_ACCESS_DISALLOWED",
            DropInfoCauseEnum::PRIVATEGOOGLEACCESSVIAVPNTUNNELUNSUPPORTED => "PRIVATE_GOOGLE_ACCESS_VIA_VPN_TUNNEL_UNSUPPORTED",
            DropInfoCauseEnum::NOEXTERNALADDRESS => "NO_EXTERNAL_ADDRESS",
            DropInfoCauseEnum::UNKNOWNINTERNALADDRESS => "UNKNOWN_INTERNAL_ADDRESS",
            DropInfoCauseEnum::FORWARDINGRULEMISMATCH => "FORWARDING_RULE_MISMATCH",
            DropInfoCauseEnum::FORWARDINGRULENOINSTANCES => "FORWARDING_RULE_NO_INSTANCES",
            DropInfoCauseEnum::FIREWALLBLOCKINGLOADBALANCERBACKENDHEALTHCHECK => "FIREWALL_BLOCKING_LOAD_BALANCER_BACKEND_HEALTH_CHECK",
            DropInfoCauseEnum::INSTANCENOTRUNNING => "INSTANCE_NOT_RUNNING",
            DropInfoCauseEnum::GKECLUSTERNOTRUNNING => "GKE_CLUSTER_NOT_RUNNING",
            DropInfoCauseEnum::CLOUDSQLINSTANCENOTRUNNING => "CLOUD_SQL_INSTANCE_NOT_RUNNING",
            DropInfoCauseEnum::TRAFFICTYPEBLOCKED => "TRAFFIC_TYPE_BLOCKED",
            DropInfoCauseEnum::GKEMASTERUNAUTHORIZEDACCESS => "GKE_MASTER_UNAUTHORIZED_ACCESS",
            DropInfoCauseEnum::CLOUDSQLINSTANCEUNAUTHORIZEDACCESS => "CLOUD_SQL_INSTANCE_UNAUTHORIZED_ACCESS",
            DropInfoCauseEnum::DROPPEDINSIDEGKESERVICE => "DROPPED_INSIDE_GKE_SERVICE",
            DropInfoCauseEnum::DROPPEDINSIDECLOUDSQLSERVICE => "DROPPED_INSIDE_CLOUD_SQL_SERVICE",
            DropInfoCauseEnum::GOOGLEMANAGEDSERVICENOPEERING => "GOOGLE_MANAGED_SERVICE_NO_PEERING",
            DropInfoCauseEnum::GOOGLEMANAGEDSERVICENOPSCENDPOINT => "GOOGLE_MANAGED_SERVICE_NO_PSC_ENDPOINT",
            DropInfoCauseEnum::GKEPSCENDPOINTMISSING => "GKE_PSC_ENDPOINT_MISSING",
            DropInfoCauseEnum::CLOUDSQLINSTANCENOIPADDRESS => "CLOUD_SQL_INSTANCE_NO_IP_ADDRESS",
            DropInfoCauseEnum::GKECONTROLPLANEREGIONMISMATCH => "GKE_CONTROL_PLANE_REGION_MISMATCH",
            DropInfoCauseEnum::PUBLICGKECONTROLPLANETOPRIVATEDESTINATION => "PUBLIC_GKE_CONTROL_PLANE_TO_PRIVATE_DESTINATION",
            DropInfoCauseEnum::GKECONTROLPLANENOROUTE => "GKE_CONTROL_PLANE_NO_ROUTE",
            DropInfoCauseEnum::CLOUDSQLINSTANCENOTCONFIGUREDFOREXTERNALTRAFFIC => "CLOUD_SQL_INSTANCE_NOT_CONFIGURED_FOR_EXTERNAL_TRAFFIC",
            DropInfoCauseEnum::PUBLICCLOUDSQLINSTANCETOPRIVATEDESTINATION => "PUBLIC_CLOUD_SQL_INSTANCE_TO_PRIVATE_DESTINATION",
            DropInfoCauseEnum::CLOUDSQLINSTANCENOROUTE => "CLOUD_SQL_INSTANCE_NO_ROUTE",
            DropInfoCauseEnum::CLOUDFUNCTIONNOTACTIVE => "CLOUD_FUNCTION_NOT_ACTIVE",
            DropInfoCauseEnum::VPCCONNECTORNOTSET => "VPC_CONNECTOR_NOT_SET",
            DropInfoCauseEnum::VPCCONNECTORNOTRUNNING => "VPC_CONNECTOR_NOT_RUNNING",
            DropInfoCauseEnum::FORWARDINGRULEREGIONMISMATCH => "FORWARDING_RULE_REGION_MISMATCH",
            DropInfoCauseEnum::PSCCONNECTIONNOTACCEPTED => "PSC_CONNECTION_NOT_ACCEPTED",
            DropInfoCauseEnum::PSCENDPOINTACCESSEDFROMPEEREDNETWORK => "PSC_ENDPOINT_ACCESSED_FROM_PEERED_NETWORK",
            DropInfoCauseEnum::PSCNEGPRODUCERENDPOINTNOGLOBALACCESS => "PSC_NEG_PRODUCER_ENDPOINT_NO_GLOBAL_ACCESS",
            DropInfoCauseEnum::PSCNEGPRODUCERFORWARDINGRULEMULTIPLEPORTS => "PSC_NEG_PRODUCER_FORWARDING_RULE_MULTIPLE_PORTS",
            DropInfoCauseEnum::CLOUDSQLPSCNEGUNSUPPORTED => "CLOUD_SQL_PSC_NEG_UNSUPPORTED",
            DropInfoCauseEnum::NONATSUBNETSFORPSCSERVICEATTACHMENT => "NO_NAT_SUBNETS_FOR_PSC_SERVICE_ATTACHMENT",
            DropInfoCauseEnum::HYBRIDNEGNONDYNAMICROUTEMATCHED => "HYBRID_NEG_NON_DYNAMIC_ROUTE_MATCHED",
            DropInfoCauseEnum::HYBRIDNEGNONLOCALDYNAMICROUTEMATCHED => "HYBRID_NEG_NON_LOCAL_DYNAMIC_ROUTE_MATCHED",
            DropInfoCauseEnum::CLOUDRUNREVISIONNOTREADY => "CLOUD_RUN_REVISION_NOT_READY",
            DropInfoCauseEnum::DROPPEDINSIDEPSCSERVICEPRODUCER => "DROPPED_INSIDE_PSC_SERVICE_PRODUCER",
            DropInfoCauseEnum::LOADBALANCERHASNOPROXYSUBNET => "LOAD_BALANCER_HAS_NO_PROXY_SUBNET",
            DropInfoCauseEnum::CLOUDNATNOADDRESSES => "CLOUD_NAT_NO_ADDRESSES",
            DropInfoCauseEnum::ROUTINGLOOP => "ROUTING_LOOP",
        }
    }
}

impl std::convert::TryFrom< &str> for DropInfoCauseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CAUSE_UNSPECIFIED" => Ok(DropInfoCauseEnum::CAUSEUNSPECIFIED),
           "UNKNOWN_EXTERNAL_ADDRESS" => Ok(DropInfoCauseEnum::UNKNOWNEXTERNALADDRESS),
           "FOREIGN_IP_DISALLOWED" => Ok(DropInfoCauseEnum::FOREIGNIPDISALLOWED),
           "FIREWALL_RULE" => Ok(DropInfoCauseEnum::FIREWALLRULE),
           "NO_ROUTE" => Ok(DropInfoCauseEnum::NOROUTE),
           "ROUTE_BLACKHOLE" => Ok(DropInfoCauseEnum::ROUTEBLACKHOLE),
           "ROUTE_WRONG_NETWORK" => Ok(DropInfoCauseEnum::ROUTEWRONGNETWORK),
           "ROUTE_NEXT_HOP_IP_ADDRESS_NOT_RESOLVED" => Ok(DropInfoCauseEnum::ROUTENEXTHOPIPADDRESSNOTRESOLVED),
           "ROUTE_NEXT_HOP_RESOURCE_NOT_FOUND" => Ok(DropInfoCauseEnum::ROUTENEXTHOPRESOURCENOTFOUND),
           "ROUTE_NEXT_HOP_INSTANCE_WRONG_NETWORK" => Ok(DropInfoCauseEnum::ROUTENEXTHOPINSTANCEWRONGNETWORK),
           "ROUTE_NEXT_HOP_INSTANCE_NON_PRIMARY_IP" => Ok(DropInfoCauseEnum::ROUTENEXTHOPINSTANCENONPRIMARYIP),
           "ROUTE_NEXT_HOP_FORWARDING_RULE_IP_MISMATCH" => Ok(DropInfoCauseEnum::ROUTENEXTHOPFORWARDINGRULEIPMISMATCH),
           "ROUTE_NEXT_HOP_VPN_TUNNEL_NOT_ESTABLISHED" => Ok(DropInfoCauseEnum::ROUTENEXTHOPVPNTUNNELNOTESTABLISHED),
           "ROUTE_NEXT_HOP_FORWARDING_RULE_TYPE_INVALID" => Ok(DropInfoCauseEnum::ROUTENEXTHOPFORWARDINGRULETYPEINVALID),
           "NO_ROUTE_FROM_INTERNET_TO_PRIVATE_IPV6_ADDRESS" => Ok(DropInfoCauseEnum::NOROUTEFROMINTERNETTOPRIVATEIPV6ADDRESS),
           "VPN_TUNNEL_LOCAL_SELECTOR_MISMATCH" => Ok(DropInfoCauseEnum::VPNTUNNELLOCALSELECTORMISMATCH),
           "VPN_TUNNEL_REMOTE_SELECTOR_MISMATCH" => Ok(DropInfoCauseEnum::VPNTUNNELREMOTESELECTORMISMATCH),
           "PRIVATE_TRAFFIC_TO_INTERNET" => Ok(DropInfoCauseEnum::PRIVATETRAFFICTOINTERNET),
           "PRIVATE_GOOGLE_ACCESS_DISALLOWED" => Ok(DropInfoCauseEnum::PRIVATEGOOGLEACCESSDISALLOWED),
           "PRIVATE_GOOGLE_ACCESS_VIA_VPN_TUNNEL_UNSUPPORTED" => Ok(DropInfoCauseEnum::PRIVATEGOOGLEACCESSVIAVPNTUNNELUNSUPPORTED),
           "NO_EXTERNAL_ADDRESS" => Ok(DropInfoCauseEnum::NOEXTERNALADDRESS),
           "UNKNOWN_INTERNAL_ADDRESS" => Ok(DropInfoCauseEnum::UNKNOWNINTERNALADDRESS),
           "FORWARDING_RULE_MISMATCH" => Ok(DropInfoCauseEnum::FORWARDINGRULEMISMATCH),
           "FORWARDING_RULE_NO_INSTANCES" => Ok(DropInfoCauseEnum::FORWARDINGRULENOINSTANCES),
           "FIREWALL_BLOCKING_LOAD_BALANCER_BACKEND_HEALTH_CHECK" => Ok(DropInfoCauseEnum::FIREWALLBLOCKINGLOADBALANCERBACKENDHEALTHCHECK),
           "INSTANCE_NOT_RUNNING" => Ok(DropInfoCauseEnum::INSTANCENOTRUNNING),
           "GKE_CLUSTER_NOT_RUNNING" => Ok(DropInfoCauseEnum::GKECLUSTERNOTRUNNING),
           "CLOUD_SQL_INSTANCE_NOT_RUNNING" => Ok(DropInfoCauseEnum::CLOUDSQLINSTANCENOTRUNNING),
           "TRAFFIC_TYPE_BLOCKED" => Ok(DropInfoCauseEnum::TRAFFICTYPEBLOCKED),
           "GKE_MASTER_UNAUTHORIZED_ACCESS" => Ok(DropInfoCauseEnum::GKEMASTERUNAUTHORIZEDACCESS),
           "CLOUD_SQL_INSTANCE_UNAUTHORIZED_ACCESS" => Ok(DropInfoCauseEnum::CLOUDSQLINSTANCEUNAUTHORIZEDACCESS),
           "DROPPED_INSIDE_GKE_SERVICE" => Ok(DropInfoCauseEnum::DROPPEDINSIDEGKESERVICE),
           "DROPPED_INSIDE_CLOUD_SQL_SERVICE" => Ok(DropInfoCauseEnum::DROPPEDINSIDECLOUDSQLSERVICE),
           "GOOGLE_MANAGED_SERVICE_NO_PEERING" => Ok(DropInfoCauseEnum::GOOGLEMANAGEDSERVICENOPEERING),
           "GOOGLE_MANAGED_SERVICE_NO_PSC_ENDPOINT" => Ok(DropInfoCauseEnum::GOOGLEMANAGEDSERVICENOPSCENDPOINT),
           "GKE_PSC_ENDPOINT_MISSING" => Ok(DropInfoCauseEnum::GKEPSCENDPOINTMISSING),
           "CLOUD_SQL_INSTANCE_NO_IP_ADDRESS" => Ok(DropInfoCauseEnum::CLOUDSQLINSTANCENOIPADDRESS),
           "GKE_CONTROL_PLANE_REGION_MISMATCH" => Ok(DropInfoCauseEnum::GKECONTROLPLANEREGIONMISMATCH),
           "PUBLIC_GKE_CONTROL_PLANE_TO_PRIVATE_DESTINATION" => Ok(DropInfoCauseEnum::PUBLICGKECONTROLPLANETOPRIVATEDESTINATION),
           "GKE_CONTROL_PLANE_NO_ROUTE" => Ok(DropInfoCauseEnum::GKECONTROLPLANENOROUTE),
           "CLOUD_SQL_INSTANCE_NOT_CONFIGURED_FOR_EXTERNAL_TRAFFIC" => Ok(DropInfoCauseEnum::CLOUDSQLINSTANCENOTCONFIGUREDFOREXTERNALTRAFFIC),
           "PUBLIC_CLOUD_SQL_INSTANCE_TO_PRIVATE_DESTINATION" => Ok(DropInfoCauseEnum::PUBLICCLOUDSQLINSTANCETOPRIVATEDESTINATION),
           "CLOUD_SQL_INSTANCE_NO_ROUTE" => Ok(DropInfoCauseEnum::CLOUDSQLINSTANCENOROUTE),
           "CLOUD_FUNCTION_NOT_ACTIVE" => Ok(DropInfoCauseEnum::CLOUDFUNCTIONNOTACTIVE),
           "VPC_CONNECTOR_NOT_SET" => Ok(DropInfoCauseEnum::VPCCONNECTORNOTSET),
           "VPC_CONNECTOR_NOT_RUNNING" => Ok(DropInfoCauseEnum::VPCCONNECTORNOTRUNNING),
           "FORWARDING_RULE_REGION_MISMATCH" => Ok(DropInfoCauseEnum::FORWARDINGRULEREGIONMISMATCH),
           "PSC_CONNECTION_NOT_ACCEPTED" => Ok(DropInfoCauseEnum::PSCCONNECTIONNOTACCEPTED),
           "PSC_ENDPOINT_ACCESSED_FROM_PEERED_NETWORK" => Ok(DropInfoCauseEnum::PSCENDPOINTACCESSEDFROMPEEREDNETWORK),
           "PSC_NEG_PRODUCER_ENDPOINT_NO_GLOBAL_ACCESS" => Ok(DropInfoCauseEnum::PSCNEGPRODUCERENDPOINTNOGLOBALACCESS),
           "PSC_NEG_PRODUCER_FORWARDING_RULE_MULTIPLE_PORTS" => Ok(DropInfoCauseEnum::PSCNEGPRODUCERFORWARDINGRULEMULTIPLEPORTS),
           "CLOUD_SQL_PSC_NEG_UNSUPPORTED" => Ok(DropInfoCauseEnum::CLOUDSQLPSCNEGUNSUPPORTED),
           "NO_NAT_SUBNETS_FOR_PSC_SERVICE_ATTACHMENT" => Ok(DropInfoCauseEnum::NONATSUBNETSFORPSCSERVICEATTACHMENT),
           "HYBRID_NEG_NON_DYNAMIC_ROUTE_MATCHED" => Ok(DropInfoCauseEnum::HYBRIDNEGNONDYNAMICROUTEMATCHED),
           "HYBRID_NEG_NON_LOCAL_DYNAMIC_ROUTE_MATCHED" => Ok(DropInfoCauseEnum::HYBRIDNEGNONLOCALDYNAMICROUTEMATCHED),
           "CLOUD_RUN_REVISION_NOT_READY" => Ok(DropInfoCauseEnum::CLOUDRUNREVISIONNOTREADY),
           "DROPPED_INSIDE_PSC_SERVICE_PRODUCER" => Ok(DropInfoCauseEnum::DROPPEDINSIDEPSCSERVICEPRODUCER),
           "LOAD_BALANCER_HAS_NO_PROXY_SUBNET" => Ok(DropInfoCauseEnum::LOADBALANCERHASNOPROXYSUBNET),
           "CLOUD_NAT_NO_ADDRESSES" => Ok(DropInfoCauseEnum::CLOUDNATNOADDRESSES),
           "ROUTING_LOOP" => Ok(DropInfoCauseEnum::ROUTINGLOOP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DropInfoCauseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EndpointForwardingRuleTargetEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the type of the target of the forwarding rule.
pub enum EndpointForwardingRuleTargetEnum {
    

    /// Forwarding rule target is unknown.
    ///
    /// "FORWARDING_RULE_TARGET_UNSPECIFIED"
    #[serde(rename="FORWARDING_RULE_TARGET_UNSPECIFIED")]
    FORWARDINGRULETARGETUNSPECIFIED,
    

    /// Compute Engine instance for protocol forwarding.
    ///
    /// "INSTANCE"
    #[serde(rename="INSTANCE")]
    INSTANCE,
    

    /// Load Balancer. The specific type can be found from load_balancer_type.
    ///
    /// "LOAD_BALANCER"
    #[serde(rename="LOAD_BALANCER")]
    LOADBALANCER,
    

    /// Classic Cloud VPN Gateway.
    ///
    /// "VPN_GATEWAY"
    #[serde(rename="VPN_GATEWAY")]
    VPNGATEWAY,
    

    /// Forwarding Rule is a Private Service Connect endpoint.
    ///
    /// "PSC"
    #[serde(rename="PSC")]
    PSC,
}

impl AsRef<str> for EndpointForwardingRuleTargetEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EndpointForwardingRuleTargetEnum::FORWARDINGRULETARGETUNSPECIFIED => "FORWARDING_RULE_TARGET_UNSPECIFIED",
            EndpointForwardingRuleTargetEnum::INSTANCE => "INSTANCE",
            EndpointForwardingRuleTargetEnum::LOADBALANCER => "LOAD_BALANCER",
            EndpointForwardingRuleTargetEnum::VPNGATEWAY => "VPN_GATEWAY",
            EndpointForwardingRuleTargetEnum::PSC => "PSC",
        }
    }
}

impl std::convert::TryFrom< &str> for EndpointForwardingRuleTargetEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORWARDING_RULE_TARGET_UNSPECIFIED" => Ok(EndpointForwardingRuleTargetEnum::FORWARDINGRULETARGETUNSPECIFIED),
           "INSTANCE" => Ok(EndpointForwardingRuleTargetEnum::INSTANCE),
           "LOAD_BALANCER" => Ok(EndpointForwardingRuleTargetEnum::LOADBALANCER),
           "VPN_GATEWAY" => Ok(EndpointForwardingRuleTargetEnum::VPNGATEWAY),
           "PSC" => Ok(EndpointForwardingRuleTargetEnum::PSC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EndpointForwardingRuleTargetEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EndpointLoadBalancerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Type of the load balancer the forwarding rule points to.
pub enum EndpointLoadBalancerTypeEnum {
    

    /// Forwarding rule points to a different target than a load balancer or a load balancer type is unknown.
    ///
    /// "LOAD_BALANCER_TYPE_UNSPECIFIED"
    #[serde(rename="LOAD_BALANCER_TYPE_UNSPECIFIED")]
    LOADBALANCERTYPEUNSPECIFIED,
    

    /// Global external HTTP(S) load balancer.
    ///
    /// "HTTPS_ADVANCED_LOAD_BALANCER"
    #[serde(rename="HTTPS_ADVANCED_LOAD_BALANCER")]
    HTTPSADVANCEDLOADBALANCER,
    

    /// Global external HTTP(S) load balancer (classic)
    ///
    /// "HTTPS_LOAD_BALANCER"
    #[serde(rename="HTTPS_LOAD_BALANCER")]
    HTTPSLOADBALANCER,
    

    /// Regional external HTTP(S) load balancer.
    ///
    /// "REGIONAL_HTTPS_LOAD_BALANCER"
    #[serde(rename="REGIONAL_HTTPS_LOAD_BALANCER")]
    REGIONALHTTPSLOADBALANCER,
    

    /// Internal HTTP(S) load balancer.
    ///
    /// "INTERNAL_HTTPS_LOAD_BALANCER"
    #[serde(rename="INTERNAL_HTTPS_LOAD_BALANCER")]
    INTERNALHTTPSLOADBALANCER,
    

    /// External SSL proxy load balancer.
    ///
    /// "SSL_PROXY_LOAD_BALANCER"
    #[serde(rename="SSL_PROXY_LOAD_BALANCER")]
    SSLPROXYLOADBALANCER,
    

    /// External TCP proxy load balancer.
    ///
    /// "TCP_PROXY_LOAD_BALANCER"
    #[serde(rename="TCP_PROXY_LOAD_BALANCER")]
    TCPPROXYLOADBALANCER,
    

    /// Internal regional TCP proxy load balancer.
    ///
    /// "INTERNAL_TCP_PROXY_LOAD_BALANCER"
    #[serde(rename="INTERNAL_TCP_PROXY_LOAD_BALANCER")]
    INTERNALTCPPROXYLOADBALANCER,
    

    /// External TCP/UDP Network load balancer.
    ///
    /// "NETWORK_LOAD_BALANCER"
    #[serde(rename="NETWORK_LOAD_BALANCER")]
    NETWORKLOADBALANCER,
    

    /// Target-pool based external TCP/UDP Network load balancer.
    ///
    /// "LEGACY_NETWORK_LOAD_BALANCER"
    #[serde(rename="LEGACY_NETWORK_LOAD_BALANCER")]
    LEGACYNETWORKLOADBALANCER,
    

    /// Internal TCP/UDP load balancer.
    ///
    /// "TCP_UDP_INTERNAL_LOAD_BALANCER"
    #[serde(rename="TCP_UDP_INTERNAL_LOAD_BALANCER")]
    TCPUDPINTERNALLOADBALANCER,
}

impl AsRef<str> for EndpointLoadBalancerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EndpointLoadBalancerTypeEnum::LOADBALANCERTYPEUNSPECIFIED => "LOAD_BALANCER_TYPE_UNSPECIFIED",
            EndpointLoadBalancerTypeEnum::HTTPSADVANCEDLOADBALANCER => "HTTPS_ADVANCED_LOAD_BALANCER",
            EndpointLoadBalancerTypeEnum::HTTPSLOADBALANCER => "HTTPS_LOAD_BALANCER",
            EndpointLoadBalancerTypeEnum::REGIONALHTTPSLOADBALANCER => "REGIONAL_HTTPS_LOAD_BALANCER",
            EndpointLoadBalancerTypeEnum::INTERNALHTTPSLOADBALANCER => "INTERNAL_HTTPS_LOAD_BALANCER",
            EndpointLoadBalancerTypeEnum::SSLPROXYLOADBALANCER => "SSL_PROXY_LOAD_BALANCER",
            EndpointLoadBalancerTypeEnum::TCPPROXYLOADBALANCER => "TCP_PROXY_LOAD_BALANCER",
            EndpointLoadBalancerTypeEnum::INTERNALTCPPROXYLOADBALANCER => "INTERNAL_TCP_PROXY_LOAD_BALANCER",
            EndpointLoadBalancerTypeEnum::NETWORKLOADBALANCER => "NETWORK_LOAD_BALANCER",
            EndpointLoadBalancerTypeEnum::LEGACYNETWORKLOADBALANCER => "LEGACY_NETWORK_LOAD_BALANCER",
            EndpointLoadBalancerTypeEnum::TCPUDPINTERNALLOADBALANCER => "TCP_UDP_INTERNAL_LOAD_BALANCER",
        }
    }
}

impl std::convert::TryFrom< &str> for EndpointLoadBalancerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOAD_BALANCER_TYPE_UNSPECIFIED" => Ok(EndpointLoadBalancerTypeEnum::LOADBALANCERTYPEUNSPECIFIED),
           "HTTPS_ADVANCED_LOAD_BALANCER" => Ok(EndpointLoadBalancerTypeEnum::HTTPSADVANCEDLOADBALANCER),
           "HTTPS_LOAD_BALANCER" => Ok(EndpointLoadBalancerTypeEnum::HTTPSLOADBALANCER),
           "REGIONAL_HTTPS_LOAD_BALANCER" => Ok(EndpointLoadBalancerTypeEnum::REGIONALHTTPSLOADBALANCER),
           "INTERNAL_HTTPS_LOAD_BALANCER" => Ok(EndpointLoadBalancerTypeEnum::INTERNALHTTPSLOADBALANCER),
           "SSL_PROXY_LOAD_BALANCER" => Ok(EndpointLoadBalancerTypeEnum::SSLPROXYLOADBALANCER),
           "TCP_PROXY_LOAD_BALANCER" => Ok(EndpointLoadBalancerTypeEnum::TCPPROXYLOADBALANCER),
           "INTERNAL_TCP_PROXY_LOAD_BALANCER" => Ok(EndpointLoadBalancerTypeEnum::INTERNALTCPPROXYLOADBALANCER),
           "NETWORK_LOAD_BALANCER" => Ok(EndpointLoadBalancerTypeEnum::NETWORKLOADBALANCER),
           "LEGACY_NETWORK_LOAD_BALANCER" => Ok(EndpointLoadBalancerTypeEnum::LEGACYNETWORKLOADBALANCER),
           "TCP_UDP_INTERNAL_LOAD_BALANCER" => Ok(EndpointLoadBalancerTypeEnum::TCPUDPINTERNALLOADBALANCER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EndpointLoadBalancerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EndpointNetworkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the network where the endpoint is located. Applicable only to source endpoint, as destination network type can be inferred from the source.
pub enum EndpointNetworkTypeEnum {
    

    /// Default type if unspecified.
    ///
    /// "NETWORK_TYPE_UNSPECIFIED"
    #[serde(rename="NETWORK_TYPE_UNSPECIFIED")]
    NETWORKTYPEUNSPECIFIED,
    

    /// A network hosted within Google Cloud. To receive more detailed output, specify the URI for the source or destination network.
    ///
    /// "GCP_NETWORK"
    #[serde(rename="GCP_NETWORK")]
    GCPNETWORK,
    

    /// A network hosted outside of Google Cloud. This can be an on-premises network, or a network hosted by another cloud provider.
    ///
    /// "NON_GCP_NETWORK"
    #[serde(rename="NON_GCP_NETWORK")]
    NONGCPNETWORK,
}

impl AsRef<str> for EndpointNetworkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EndpointNetworkTypeEnum::NETWORKTYPEUNSPECIFIED => "NETWORK_TYPE_UNSPECIFIED",
            EndpointNetworkTypeEnum::GCPNETWORK => "GCP_NETWORK",
            EndpointNetworkTypeEnum::NONGCPNETWORK => "NON_GCP_NETWORK",
        }
    }
}

impl std::convert::TryFrom< &str> for EndpointNetworkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NETWORK_TYPE_UNSPECIFIED" => Ok(EndpointNetworkTypeEnum::NETWORKTYPEUNSPECIFIED),
           "GCP_NETWORK" => Ok(EndpointNetworkTypeEnum::GCPNETWORK),
           "NON_GCP_NETWORK" => Ok(EndpointNetworkTypeEnum::NONGCPNETWORK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EndpointNetworkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirewallInfoFirewallRuleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The firewall rule's type.
pub enum FirewallInfoFirewallRuleTypeEnum {
    

    /// Unspecified type.
    ///
    /// "FIREWALL_RULE_TYPE_UNSPECIFIED"
    #[serde(rename="FIREWALL_RULE_TYPE_UNSPECIFIED")]
    FIREWALLRULETYPEUNSPECIFIED,
    

    /// Hierarchical firewall policy rule. For details, see [Hierarchical firewall policies overview](https://cloud.google.com/vpc/docs/firewall-policies).
    ///
    /// "HIERARCHICAL_FIREWALL_POLICY_RULE"
    #[serde(rename="HIERARCHICAL_FIREWALL_POLICY_RULE")]
    HIERARCHICALFIREWALLPOLICYRULE,
    

    /// VPC firewall rule. For details, see [VPC firewall rules overview](https://cloud.google.com/vpc/docs/firewalls).
    ///
    /// "VPC_FIREWALL_RULE"
    #[serde(rename="VPC_FIREWALL_RULE")]
    VPCFIREWALLRULE,
    

    /// Implied VPC firewall rule. For details, see [Implied rules](https://cloud.google.com/vpc/docs/firewalls#default_firewall_rules).
    ///
    /// "IMPLIED_VPC_FIREWALL_RULE"
    #[serde(rename="IMPLIED_VPC_FIREWALL_RULE")]
    IMPLIEDVPCFIREWALLRULE,
    

    /// Implicit firewall rules that are managed by serverless VPC access to allow ingress access. They are not visible in the Google Cloud console. For details, see [VPC connector's implicit rules](https://cloud.google.com/functions/docs/networking/connecting-vpc#restrict-access).
    ///
    /// "SERVERLESS_VPC_ACCESS_MANAGED_FIREWALL_RULE"
    #[serde(rename="SERVERLESS_VPC_ACCESS_MANAGED_FIREWALL_RULE")]
    SERVERLESSVPCACCESSMANAGEDFIREWALLRULE,
    

    /// Global network firewall policy rule. For details, see [Network firewall policies](https://cloud.google.com/vpc/docs/network-firewall-policies).
    ///
    /// "NETWORK_FIREWALL_POLICY_RULE"
    #[serde(rename="NETWORK_FIREWALL_POLICY_RULE")]
    NETWORKFIREWALLPOLICYRULE,
    

    /// Regional network firewall policy rule. For details, see [Regional network firewall policies](https://cloud.google.com/firewall/docs/regional-firewall-policies).
    ///
    /// "NETWORK_REGIONAL_FIREWALL_POLICY_RULE"
    #[serde(rename="NETWORK_REGIONAL_FIREWALL_POLICY_RULE")]
    NETWORKREGIONALFIREWALLPOLICYRULE,
    

    /// Firewall policy rule containing attributes not yet supported in Connectivity tests. Firewall analysis is skipped if such a rule can potentially be matched. Please see the [list of unsupported configurations](https://cloud.google.com/network-intelligence-center/docs/connectivity-tests/concepts/overview#unsupported-configs).
    ///
    /// "UNSUPPORTED_FIREWALL_POLICY_RULE"
    #[serde(rename="UNSUPPORTED_FIREWALL_POLICY_RULE")]
    UNSUPPORTEDFIREWALLPOLICYRULE,
    

    /// Tracking state for response traffic created when request traffic goes through allow firewall rule. For details, see [firewall rules specifications](https://cloud.google.com/firewall/docs/firewalls#specifications)
    ///
    /// "TRACKING_STATE"
    #[serde(rename="TRACKING_STATE")]
    TRACKINGSTATE,
}

impl AsRef<str> for FirewallInfoFirewallRuleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirewallInfoFirewallRuleTypeEnum::FIREWALLRULETYPEUNSPECIFIED => "FIREWALL_RULE_TYPE_UNSPECIFIED",
            FirewallInfoFirewallRuleTypeEnum::HIERARCHICALFIREWALLPOLICYRULE => "HIERARCHICAL_FIREWALL_POLICY_RULE",
            FirewallInfoFirewallRuleTypeEnum::VPCFIREWALLRULE => "VPC_FIREWALL_RULE",
            FirewallInfoFirewallRuleTypeEnum::IMPLIEDVPCFIREWALLRULE => "IMPLIED_VPC_FIREWALL_RULE",
            FirewallInfoFirewallRuleTypeEnum::SERVERLESSVPCACCESSMANAGEDFIREWALLRULE => "SERVERLESS_VPC_ACCESS_MANAGED_FIREWALL_RULE",
            FirewallInfoFirewallRuleTypeEnum::NETWORKFIREWALLPOLICYRULE => "NETWORK_FIREWALL_POLICY_RULE",
            FirewallInfoFirewallRuleTypeEnum::NETWORKREGIONALFIREWALLPOLICYRULE => "NETWORK_REGIONAL_FIREWALL_POLICY_RULE",
            FirewallInfoFirewallRuleTypeEnum::UNSUPPORTEDFIREWALLPOLICYRULE => "UNSUPPORTED_FIREWALL_POLICY_RULE",
            FirewallInfoFirewallRuleTypeEnum::TRACKINGSTATE => "TRACKING_STATE",
        }
    }
}

impl std::convert::TryFrom< &str> for FirewallInfoFirewallRuleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FIREWALL_RULE_TYPE_UNSPECIFIED" => Ok(FirewallInfoFirewallRuleTypeEnum::FIREWALLRULETYPEUNSPECIFIED),
           "HIERARCHICAL_FIREWALL_POLICY_RULE" => Ok(FirewallInfoFirewallRuleTypeEnum::HIERARCHICALFIREWALLPOLICYRULE),
           "VPC_FIREWALL_RULE" => Ok(FirewallInfoFirewallRuleTypeEnum::VPCFIREWALLRULE),
           "IMPLIED_VPC_FIREWALL_RULE" => Ok(FirewallInfoFirewallRuleTypeEnum::IMPLIEDVPCFIREWALLRULE),
           "SERVERLESS_VPC_ACCESS_MANAGED_FIREWALL_RULE" => Ok(FirewallInfoFirewallRuleTypeEnum::SERVERLESSVPCACCESSMANAGEDFIREWALLRULE),
           "NETWORK_FIREWALL_POLICY_RULE" => Ok(FirewallInfoFirewallRuleTypeEnum::NETWORKFIREWALLPOLICYRULE),
           "NETWORK_REGIONAL_FIREWALL_POLICY_RULE" => Ok(FirewallInfoFirewallRuleTypeEnum::NETWORKREGIONALFIREWALLPOLICYRULE),
           "UNSUPPORTED_FIREWALL_POLICY_RULE" => Ok(FirewallInfoFirewallRuleTypeEnum::UNSUPPORTEDFIREWALLPOLICYRULE),
           "TRACKING_STATE" => Ok(FirewallInfoFirewallRuleTypeEnum::TRACKINGSTATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirewallInfoFirewallRuleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ForwardInfoTargetEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Target type where this packet is forwarded to.
pub enum ForwardInfoTargetEnum {
    

    /// Target not specified.
    ///
    /// "TARGET_UNSPECIFIED"
    #[serde(rename="TARGET_UNSPECIFIED")]
    TARGETUNSPECIFIED,
    

    /// Forwarded to a VPC peering network.
    ///
    /// "PEERING_VPC"
    #[serde(rename="PEERING_VPC")]
    PEERINGVPC,
    

    /// Forwarded to a Cloud VPN gateway.
    ///
    /// "VPN_GATEWAY"
    #[serde(rename="VPN_GATEWAY")]
    VPNGATEWAY,
    

    /// Forwarded to a Cloud Interconnect connection.
    ///
    /// "INTERCONNECT"
    #[serde(rename="INTERCONNECT")]
    INTERCONNECT,
    

    /// Forwarded to a Google Kubernetes Engine Container cluster master.
    ///
    /// "GKE_MASTER"
    #[serde(rename="GKE_MASTER")]
    GKEMASTER,
    

    /// Forwarded to the next hop of a custom route imported from a peering VPC.
    ///
    /// "IMPORTED_CUSTOM_ROUTE_NEXT_HOP"
    #[serde(rename="IMPORTED_CUSTOM_ROUTE_NEXT_HOP")]
    IMPORTEDCUSTOMROUTENEXTHOP,
    

    /// Forwarded to a Cloud SQL instance.
    ///
    /// "CLOUD_SQL_INSTANCE"
    #[serde(rename="CLOUD_SQL_INSTANCE")]
    CLOUDSQLINSTANCE,
    

    /// Forwarded to a VPC network in another project.
    ///
    /// "ANOTHER_PROJECT"
    #[serde(rename="ANOTHER_PROJECT")]
    ANOTHERPROJECT,
    

    /// Forwarded to an NCC Hub.
    ///
    /// "NCC_HUB"
    #[serde(rename="NCC_HUB")]
    NCCHUB,
    

    /// Forwarded to a router appliance.
    ///
    /// "ROUTER_APPLIANCE"
    #[serde(rename="ROUTER_APPLIANCE")]
    ROUTERAPPLIANCE,
}

impl AsRef<str> for ForwardInfoTargetEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ForwardInfoTargetEnum::TARGETUNSPECIFIED => "TARGET_UNSPECIFIED",
            ForwardInfoTargetEnum::PEERINGVPC => "PEERING_VPC",
            ForwardInfoTargetEnum::VPNGATEWAY => "VPN_GATEWAY",
            ForwardInfoTargetEnum::INTERCONNECT => "INTERCONNECT",
            ForwardInfoTargetEnum::GKEMASTER => "GKE_MASTER",
            ForwardInfoTargetEnum::IMPORTEDCUSTOMROUTENEXTHOP => "IMPORTED_CUSTOM_ROUTE_NEXT_HOP",
            ForwardInfoTargetEnum::CLOUDSQLINSTANCE => "CLOUD_SQL_INSTANCE",
            ForwardInfoTargetEnum::ANOTHERPROJECT => "ANOTHER_PROJECT",
            ForwardInfoTargetEnum::NCCHUB => "NCC_HUB",
            ForwardInfoTargetEnum::ROUTERAPPLIANCE => "ROUTER_APPLIANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for ForwardInfoTargetEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGET_UNSPECIFIED" => Ok(ForwardInfoTargetEnum::TARGETUNSPECIFIED),
           "PEERING_VPC" => Ok(ForwardInfoTargetEnum::PEERINGVPC),
           "VPN_GATEWAY" => Ok(ForwardInfoTargetEnum::VPNGATEWAY),
           "INTERCONNECT" => Ok(ForwardInfoTargetEnum::INTERCONNECT),
           "GKE_MASTER" => Ok(ForwardInfoTargetEnum::GKEMASTER),
           "IMPORTED_CUSTOM_ROUTE_NEXT_HOP" => Ok(ForwardInfoTargetEnum::IMPORTEDCUSTOMROUTENEXTHOP),
           "CLOUD_SQL_INSTANCE" => Ok(ForwardInfoTargetEnum::CLOUDSQLINSTANCE),
           "ANOTHER_PROJECT" => Ok(ForwardInfoTargetEnum::ANOTHERPROJECT),
           "NCC_HUB" => Ok(ForwardInfoTargetEnum::NCCHUB),
           "ROUTER_APPLIANCE" => Ok(ForwardInfoTargetEnum::ROUTERAPPLIANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ForwardInfoTargetEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleServiceInfoGoogleServiceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Recognized type of a Google Service.
pub enum GoogleServiceInfoGoogleServiceTypeEnum {
    

    /// Unspecified Google Service.
    ///
    /// "GOOGLE_SERVICE_TYPE_UNSPECIFIED"
    #[serde(rename="GOOGLE_SERVICE_TYPE_UNSPECIFIED")]
    GOOGLESERVICETYPEUNSPECIFIED,
    

    /// Identity aware proxy. https://cloud.google.com/iap/docs/using-tcp-forwarding
    ///
    /// "IAP"
    #[serde(rename="IAP")]
    IAP,
    

    /// One of two services sharing IP ranges: * Load Balancer proxy * Centralized Health Check prober https://cloud.google.com/load-balancing/docs/firewall-rules
    ///
    /// "GFE_PROXY_OR_HEALTH_CHECK_PROBER"
    #[serde(rename="GFE_PROXY_OR_HEALTH_CHECK_PROBER")]
    GFEPROXYORHEALTHCHECKPROBER,
    

    /// Connectivity from Cloud DNS to forwarding targets or alternate name servers that use private routing. https://cloud.google.com/dns/docs/zones/forwarding-zones#firewall-rules https://cloud.google.com/dns/docs/policies#firewall-rules
    ///
    /// "CLOUD_DNS"
    #[serde(rename="CLOUD_DNS")]
    CLOUDDNS,
    

    /// private.googleapis.com and restricted.googleapis.com
    ///
    /// "GOOGLE_API"
    #[serde(rename="GOOGLE_API")]
    GOOGLEAPI,
    

    /// Google API via Private Service Connect. https://cloud.google.com/vpc/docs/configure-private-service-connect-apis
    ///
    /// "GOOGLE_API_PSC"
    #[serde(rename="GOOGLE_API_PSC")]
    GOOGLEAPIPSC,
    

    /// Google API via VPC Service Controls. https://cloud.google.com/vpc/docs/configure-private-service-connect-apis
    ///
    /// "GOOGLE_API_VPC_SC"
    #[serde(rename="GOOGLE_API_VPC_SC")]
    GOOGLEAPIVPCSC,
}

impl AsRef<str> for GoogleServiceInfoGoogleServiceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleServiceInfoGoogleServiceTypeEnum::GOOGLESERVICETYPEUNSPECIFIED => "GOOGLE_SERVICE_TYPE_UNSPECIFIED",
            GoogleServiceInfoGoogleServiceTypeEnum::IAP => "IAP",
            GoogleServiceInfoGoogleServiceTypeEnum::GFEPROXYORHEALTHCHECKPROBER => "GFE_PROXY_OR_HEALTH_CHECK_PROBER",
            GoogleServiceInfoGoogleServiceTypeEnum::CLOUDDNS => "CLOUD_DNS",
            GoogleServiceInfoGoogleServiceTypeEnum::GOOGLEAPI => "GOOGLE_API",
            GoogleServiceInfoGoogleServiceTypeEnum::GOOGLEAPIPSC => "GOOGLE_API_PSC",
            GoogleServiceInfoGoogleServiceTypeEnum::GOOGLEAPIVPCSC => "GOOGLE_API_VPC_SC",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleServiceInfoGoogleServiceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GOOGLE_SERVICE_TYPE_UNSPECIFIED" => Ok(GoogleServiceInfoGoogleServiceTypeEnum::GOOGLESERVICETYPEUNSPECIFIED),
           "IAP" => Ok(GoogleServiceInfoGoogleServiceTypeEnum::IAP),
           "GFE_PROXY_OR_HEALTH_CHECK_PROBER" => Ok(GoogleServiceInfoGoogleServiceTypeEnum::GFEPROXYORHEALTHCHECKPROBER),
           "CLOUD_DNS" => Ok(GoogleServiceInfoGoogleServiceTypeEnum::CLOUDDNS),
           "GOOGLE_API" => Ok(GoogleServiceInfoGoogleServiceTypeEnum::GOOGLEAPI),
           "GOOGLE_API_PSC" => Ok(GoogleServiceInfoGoogleServiceTypeEnum::GOOGLEAPIPSC),
           "GOOGLE_API_VPC_SC" => Ok(GoogleServiceInfoGoogleServiceTypeEnum::GOOGLEAPIVPCSC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleServiceInfoGoogleServiceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoadBalancerBackendHealthCheckFirewallStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the health check firewall configuration.
pub enum LoadBalancerBackendHealthCheckFirewallStateEnum {
    

    /// State is unspecified. Default state if not populated.
    ///
    /// "HEALTH_CHECK_FIREWALL_STATE_UNSPECIFIED"
    #[serde(rename="HEALTH_CHECK_FIREWALL_STATE_UNSPECIFIED")]
    HEALTHCHECKFIREWALLSTATEUNSPECIFIED,
    

    /// There are configured firewall rules to allow health check probes to the backend.
    ///
    /// "CONFIGURED"
    #[serde(rename="CONFIGURED")]
    CONFIGURED,
    

    /// There are firewall rules configured to allow partial health check ranges or block all health check ranges. If a health check probe is sent from denied IP ranges, the health check to the backend will fail. Then, the backend will be marked unhealthy and will not receive traffic sent to the load balancer.
    ///
    /// "MISCONFIGURED"
    #[serde(rename="MISCONFIGURED")]
    MISCONFIGURED,
}

impl AsRef<str> for LoadBalancerBackendHealthCheckFirewallStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoadBalancerBackendHealthCheckFirewallStateEnum::HEALTHCHECKFIREWALLSTATEUNSPECIFIED => "HEALTH_CHECK_FIREWALL_STATE_UNSPECIFIED",
            LoadBalancerBackendHealthCheckFirewallStateEnum::CONFIGURED => "CONFIGURED",
            LoadBalancerBackendHealthCheckFirewallStateEnum::MISCONFIGURED => "MISCONFIGURED",
        }
    }
}

impl std::convert::TryFrom< &str> for LoadBalancerBackendHealthCheckFirewallStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEALTH_CHECK_FIREWALL_STATE_UNSPECIFIED" => Ok(LoadBalancerBackendHealthCheckFirewallStateEnum::HEALTHCHECKFIREWALLSTATEUNSPECIFIED),
           "CONFIGURED" => Ok(LoadBalancerBackendHealthCheckFirewallStateEnum::CONFIGURED),
           "MISCONFIGURED" => Ok(LoadBalancerBackendHealthCheckFirewallStateEnum::MISCONFIGURED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoadBalancerBackendHealthCheckFirewallStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Health check firewalls configuration state for the backend. This is a result of the static firewall analysis (verifying that health check traffic from required IP ranges to the backend is allowed or not). The backend might still be unhealthy even if these firewalls are configured. Please refer to the documentation for more information: https://cloud.google.com/load-balancing/docs/firewall-rules
pub enum LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum {
    

    /// Configuration state unspecified. It usually means that the backend has no health check attached, or there was an unexpected configuration error preventing Connectivity tests from verifying health check configuration.
    ///
    /// "HEALTH_CHECK_FIREWALLS_CONFIG_STATE_UNSPECIFIED"
    #[serde(rename="HEALTH_CHECK_FIREWALLS_CONFIG_STATE_UNSPECIFIED")]
    HEALTHCHECKFIREWALLSCONFIGSTATEUNSPECIFIED,
    

    /// Firewall rules (policies) allowing health check traffic from all required IP ranges to the backend are configured.
    ///
    /// "FIREWALLS_CONFIGURED"
    #[serde(rename="FIREWALLS_CONFIGURED")]
    FIREWALLSCONFIGURED,
    

    /// Firewall rules (policies) allow health check traffic only from a part of required IP ranges.
    ///
    /// "FIREWALLS_PARTIALLY_CONFIGURED"
    #[serde(rename="FIREWALLS_PARTIALLY_CONFIGURED")]
    FIREWALLSPARTIALLYCONFIGURED,
    

    /// Firewall rules (policies) deny health check traffic from all required IP ranges to the backend.
    ///
    /// "FIREWALLS_NOT_CONFIGURED"
    #[serde(rename="FIREWALLS_NOT_CONFIGURED")]
    FIREWALLSNOTCONFIGURED,
    

    /// The network contains firewall rules of unsupported types, so Connectivity tests were not able to verify health check configuration status. Please refer to the documentation for the list of unsupported configurations: https://cloud.google.com/network-intelligence-center/docs/connectivity-tests/concepts/overview#unsupported-configs
    ///
    /// "FIREWALLS_UNSUPPORTED"
    #[serde(rename="FIREWALLS_UNSUPPORTED")]
    FIREWALLSUNSUPPORTED,
}

impl AsRef<str> for LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum::HEALTHCHECKFIREWALLSCONFIGSTATEUNSPECIFIED => "HEALTH_CHECK_FIREWALLS_CONFIG_STATE_UNSPECIFIED",
            LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum::FIREWALLSCONFIGURED => "FIREWALLS_CONFIGURED",
            LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum::FIREWALLSPARTIALLYCONFIGURED => "FIREWALLS_PARTIALLY_CONFIGURED",
            LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum::FIREWALLSNOTCONFIGURED => "FIREWALLS_NOT_CONFIGURED",
            LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum::FIREWALLSUNSUPPORTED => "FIREWALLS_UNSUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEALTH_CHECK_FIREWALLS_CONFIG_STATE_UNSPECIFIED" => Ok(LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum::HEALTHCHECKFIREWALLSCONFIGSTATEUNSPECIFIED),
           "FIREWALLS_CONFIGURED" => Ok(LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum::FIREWALLSCONFIGURED),
           "FIREWALLS_PARTIALLY_CONFIGURED" => Ok(LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum::FIREWALLSPARTIALLYCONFIGURED),
           "FIREWALLS_NOT_CONFIGURED" => Ok(LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum::FIREWALLSNOTCONFIGURED),
           "FIREWALLS_UNSUPPORTED" => Ok(LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum::FIREWALLSUNSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoadBalancerBackendInfoHealthCheckFirewallsConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoadBalancerInfoBackendTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of load balancer's backend configuration.
pub enum LoadBalancerInfoBackendTypeEnum {
    

    /// Type is unspecified.
    ///
    /// "BACKEND_TYPE_UNSPECIFIED"
    #[serde(rename="BACKEND_TYPE_UNSPECIFIED")]
    BACKENDTYPEUNSPECIFIED,
    

    /// Backend Service as the load balancer's backend.
    ///
    /// "BACKEND_SERVICE"
    #[serde(rename="BACKEND_SERVICE")]
    BACKENDSERVICE,
    

    /// Target Pool as the load balancer's backend.
    ///
    /// "TARGET_POOL"
    #[serde(rename="TARGET_POOL")]
    TARGETPOOL,
    

    /// Target Instance as the load balancer's backend.
    ///
    /// "TARGET_INSTANCE"
    #[serde(rename="TARGET_INSTANCE")]
    TARGETINSTANCE,
}

impl AsRef<str> for LoadBalancerInfoBackendTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoadBalancerInfoBackendTypeEnum::BACKENDTYPEUNSPECIFIED => "BACKEND_TYPE_UNSPECIFIED",
            LoadBalancerInfoBackendTypeEnum::BACKENDSERVICE => "BACKEND_SERVICE",
            LoadBalancerInfoBackendTypeEnum::TARGETPOOL => "TARGET_POOL",
            LoadBalancerInfoBackendTypeEnum::TARGETINSTANCE => "TARGET_INSTANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for LoadBalancerInfoBackendTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BACKEND_TYPE_UNSPECIFIED" => Ok(LoadBalancerInfoBackendTypeEnum::BACKENDTYPEUNSPECIFIED),
           "BACKEND_SERVICE" => Ok(LoadBalancerInfoBackendTypeEnum::BACKENDSERVICE),
           "TARGET_POOL" => Ok(LoadBalancerInfoBackendTypeEnum::TARGETPOOL),
           "TARGET_INSTANCE" => Ok(LoadBalancerInfoBackendTypeEnum::TARGETINSTANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoadBalancerInfoBackendTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoadBalancerInfoLoadBalancerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the load balancer.
pub enum LoadBalancerInfoLoadBalancerTypeEnum {
    

    /// Type is unspecified.
    ///
    /// "LOAD_BALANCER_TYPE_UNSPECIFIED"
    #[serde(rename="LOAD_BALANCER_TYPE_UNSPECIFIED")]
    LOADBALANCERTYPEUNSPECIFIED,
    

    /// Internal TCP/UDP load balancer.
    ///
    /// "INTERNAL_TCP_UDP"
    #[serde(rename="INTERNAL_TCP_UDP")]
    INTERNALTCPUDP,
    

    /// Network TCP/UDP load balancer.
    ///
    /// "NETWORK_TCP_UDP"
    #[serde(rename="NETWORK_TCP_UDP")]
    NETWORKTCPUDP,
    

    /// HTTP(S) proxy load balancer.
    ///
    /// "HTTP_PROXY"
    #[serde(rename="HTTP_PROXY")]
    HTTPPROXY,
    

    /// TCP proxy load balancer.
    ///
    /// "TCP_PROXY"
    #[serde(rename="TCP_PROXY")]
    TCPPROXY,
    

    /// SSL proxy load balancer.
    ///
    /// "SSL_PROXY"
    #[serde(rename="SSL_PROXY")]
    SSLPROXY,
}

impl AsRef<str> for LoadBalancerInfoLoadBalancerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoadBalancerInfoLoadBalancerTypeEnum::LOADBALANCERTYPEUNSPECIFIED => "LOAD_BALANCER_TYPE_UNSPECIFIED",
            LoadBalancerInfoLoadBalancerTypeEnum::INTERNALTCPUDP => "INTERNAL_TCP_UDP",
            LoadBalancerInfoLoadBalancerTypeEnum::NETWORKTCPUDP => "NETWORK_TCP_UDP",
            LoadBalancerInfoLoadBalancerTypeEnum::HTTPPROXY => "HTTP_PROXY",
            LoadBalancerInfoLoadBalancerTypeEnum::TCPPROXY => "TCP_PROXY",
            LoadBalancerInfoLoadBalancerTypeEnum::SSLPROXY => "SSL_PROXY",
        }
    }
}

impl std::convert::TryFrom< &str> for LoadBalancerInfoLoadBalancerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOAD_BALANCER_TYPE_UNSPECIFIED" => Ok(LoadBalancerInfoLoadBalancerTypeEnum::LOADBALANCERTYPEUNSPECIFIED),
           "INTERNAL_TCP_UDP" => Ok(LoadBalancerInfoLoadBalancerTypeEnum::INTERNALTCPUDP),
           "NETWORK_TCP_UDP" => Ok(LoadBalancerInfoLoadBalancerTypeEnum::NETWORKTCPUDP),
           "HTTP_PROXY" => Ok(LoadBalancerInfoLoadBalancerTypeEnum::HTTPPROXY),
           "TCP_PROXY" => Ok(LoadBalancerInfoLoadBalancerTypeEnum::TCPPROXY),
           "SSL_PROXY" => Ok(LoadBalancerInfoLoadBalancerTypeEnum::SSLPROXY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoadBalancerInfoLoadBalancerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NatInfoTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of NAT.
pub enum NatInfoTypeEnum {
    

    /// Type is unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// From Compute Engine instance's internal address to external address.
    ///
    /// "INTERNAL_TO_EXTERNAL"
    #[serde(rename="INTERNAL_TO_EXTERNAL")]
    INTERNALTOEXTERNAL,
    

    /// From Compute Engine instance's external address to internal address.
    ///
    /// "EXTERNAL_TO_INTERNAL"
    #[serde(rename="EXTERNAL_TO_INTERNAL")]
    EXTERNALTOINTERNAL,
    

    /// Cloud NAT Gateway.
    ///
    /// "CLOUD_NAT"
    #[serde(rename="CLOUD_NAT")]
    CLOUDNAT,
    

    /// Private service connect NAT.
    ///
    /// "PRIVATE_SERVICE_CONNECT"
    #[serde(rename="PRIVATE_SERVICE_CONNECT")]
    PRIVATESERVICECONNECT,
}

impl AsRef<str> for NatInfoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NatInfoTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            NatInfoTypeEnum::INTERNALTOEXTERNAL => "INTERNAL_TO_EXTERNAL",
            NatInfoTypeEnum::EXTERNALTOINTERNAL => "EXTERNAL_TO_INTERNAL",
            NatInfoTypeEnum::CLOUDNAT => "CLOUD_NAT",
            NatInfoTypeEnum::PRIVATESERVICECONNECT => "PRIVATE_SERVICE_CONNECT",
        }
    }
}

impl std::convert::TryFrom< &str> for NatInfoTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(NatInfoTypeEnum::TYPEUNSPECIFIED),
           "INTERNAL_TO_EXTERNAL" => Ok(NatInfoTypeEnum::INTERNALTOEXTERNAL),
           "EXTERNAL_TO_INTERNAL" => Ok(NatInfoTypeEnum::EXTERNALTOINTERNAL),
           "CLOUD_NAT" => Ok(NatInfoTypeEnum::CLOUDNAT),
           "PRIVATE_SERVICE_CONNECT" => Ok(NatInfoTypeEnum::PRIVATESERVICECONNECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NatInfoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProbingDetailAbortCauseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason probing was aborted.
pub enum ProbingDetailAbortCauseEnum {
    

    /// No reason was specified.
    ///
    /// "PROBING_ABORT_CAUSE_UNSPECIFIED"
    #[serde(rename="PROBING_ABORT_CAUSE_UNSPECIFIED")]
    PROBINGABORTCAUSEUNSPECIFIED,
    

    /// The user lacks permission to access some of the network resources required to run the test.
    ///
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    

    /// No valid source endpoint could be derived from the request.
    ///
    /// "NO_SOURCE_LOCATION"
    #[serde(rename="NO_SOURCE_LOCATION")]
    NOSOURCELOCATION,
}

impl AsRef<str> for ProbingDetailAbortCauseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProbingDetailAbortCauseEnum::PROBINGABORTCAUSEUNSPECIFIED => "PROBING_ABORT_CAUSE_UNSPECIFIED",
            ProbingDetailAbortCauseEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            ProbingDetailAbortCauseEnum::NOSOURCELOCATION => "NO_SOURCE_LOCATION",
        }
    }
}

impl std::convert::TryFrom< &str> for ProbingDetailAbortCauseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROBING_ABORT_CAUSE_UNSPECIFIED" => Ok(ProbingDetailAbortCauseEnum::PROBINGABORTCAUSEUNSPECIFIED),
           "PERMISSION_DENIED" => Ok(ProbingDetailAbortCauseEnum::PERMISSIONDENIED),
           "NO_SOURCE_LOCATION" => Ok(ProbingDetailAbortCauseEnum::NOSOURCELOCATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProbingDetailAbortCauseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProbingDetailResultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The overall result of active probing.
pub enum ProbingDetailResultEnum {
    

    /// No result was specified.
    ///
    /// "PROBING_RESULT_UNSPECIFIED"
    #[serde(rename="PROBING_RESULT_UNSPECIFIED")]
    PROBINGRESULTUNSPECIFIED,
    

    /// At least 95% of packets reached the destination.
    ///
    /// "REACHABLE"
    #[serde(rename="REACHABLE")]
    REACHABLE,
    

    /// No packets reached the destination.
    ///
    /// "UNREACHABLE"
    #[serde(rename="UNREACHABLE")]
    UNREACHABLE,
    

    /// Less than 95% of packets reached the destination.
    ///
    /// "REACHABILITY_INCONSISTENT"
    #[serde(rename="REACHABILITY_INCONSISTENT")]
    REACHABILITYINCONSISTENT,
    

    /// Reachability could not be determined. Possible reasons are: * The user lacks permission to access some of the network resources required to run the test. * No valid source endpoint could be derived from the request. * An internal error occurred.
    ///
    /// "UNDETERMINED"
    #[serde(rename="UNDETERMINED")]
    UNDETERMINED,
}

impl AsRef<str> for ProbingDetailResultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProbingDetailResultEnum::PROBINGRESULTUNSPECIFIED => "PROBING_RESULT_UNSPECIFIED",
            ProbingDetailResultEnum::REACHABLE => "REACHABLE",
            ProbingDetailResultEnum::UNREACHABLE => "UNREACHABLE",
            ProbingDetailResultEnum::REACHABILITYINCONSISTENT => "REACHABILITY_INCONSISTENT",
            ProbingDetailResultEnum::UNDETERMINED => "UNDETERMINED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProbingDetailResultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROBING_RESULT_UNSPECIFIED" => Ok(ProbingDetailResultEnum::PROBINGRESULTUNSPECIFIED),
           "REACHABLE" => Ok(ProbingDetailResultEnum::REACHABLE),
           "UNREACHABLE" => Ok(ProbingDetailResultEnum::UNREACHABLE),
           "REACHABILITY_INCONSISTENT" => Ok(ProbingDetailResultEnum::REACHABILITYINCONSISTENT),
           "UNDETERMINED" => Ok(ProbingDetailResultEnum::UNDETERMINED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProbingDetailResultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReachabilityDetailResultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The overall result of the test's configuration analysis.
pub enum ReachabilityDetailResultEnum {
    

    /// No result was specified.
    ///
    /// "RESULT_UNSPECIFIED"
    #[serde(rename="RESULT_UNSPECIFIED")]
    RESULTUNSPECIFIED,
    

    /// Possible scenarios are: * The configuration analysis determined that a packet originating from the source is expected to reach the destination. * The analysis didn't complete because the user lacks permission for some of the resources in the trace. However, at the time the user's permission became insufficient, the trace had been successful so far.
    ///
    /// "REACHABLE"
    #[serde(rename="REACHABLE")]
    REACHABLE,
    

    /// A packet originating from the source is expected to be dropped before reaching the destination.
    ///
    /// "UNREACHABLE"
    #[serde(rename="UNREACHABLE")]
    UNREACHABLE,
    

    /// The source and destination endpoints do not uniquely identify the test location in the network, and the reachability result contains multiple traces. For some traces, a packet could be delivered, and for others, it would not be. This result is also assigned to configuration analysis of return path if on its own it should be REACHABLE, but configuration analysis of forward path is AMBIGUOUS.
    ///
    /// "AMBIGUOUS"
    #[serde(rename="AMBIGUOUS")]
    AMBIGUOUS,
    

    /// The configuration analysis did not complete. Possible reasons are: * A permissions error occurred--for example, the user might not have read permission for all of the resources named in the test. * An internal error occurred. * The analyzer received an invalid or unsupported argument or was unable to identify a known endpoint.
    ///
    /// "UNDETERMINED"
    #[serde(rename="UNDETERMINED")]
    UNDETERMINED,
}

impl AsRef<str> for ReachabilityDetailResultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReachabilityDetailResultEnum::RESULTUNSPECIFIED => "RESULT_UNSPECIFIED",
            ReachabilityDetailResultEnum::REACHABLE => "REACHABLE",
            ReachabilityDetailResultEnum::UNREACHABLE => "UNREACHABLE",
            ReachabilityDetailResultEnum::AMBIGUOUS => "AMBIGUOUS",
            ReachabilityDetailResultEnum::UNDETERMINED => "UNDETERMINED",
        }
    }
}

impl std::convert::TryFrom< &str> for ReachabilityDetailResultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESULT_UNSPECIFIED" => Ok(ReachabilityDetailResultEnum::RESULTUNSPECIFIED),
           "REACHABLE" => Ok(ReachabilityDetailResultEnum::REACHABLE),
           "UNREACHABLE" => Ok(ReachabilityDetailResultEnum::UNREACHABLE),
           "AMBIGUOUS" => Ok(ReachabilityDetailResultEnum::AMBIGUOUS),
           "UNDETERMINED" => Ok(ReachabilityDetailResultEnum::UNDETERMINED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReachabilityDetailResultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RouteInfoNextHopTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of next hop.
pub enum RouteInfoNextHopTypeEnum {
    

    /// Unspecified type. Default value.
    ///
    /// "NEXT_HOP_TYPE_UNSPECIFIED"
    #[serde(rename="NEXT_HOP_TYPE_UNSPECIFIED")]
    NEXTHOPTYPEUNSPECIFIED,
    

    /// Next hop is an IP address.
    ///
    /// "NEXT_HOP_IP"
    #[serde(rename="NEXT_HOP_IP")]
    NEXTHOPIP,
    

    /// Next hop is a Compute Engine instance.
    ///
    /// "NEXT_HOP_INSTANCE"
    #[serde(rename="NEXT_HOP_INSTANCE")]
    NEXTHOPINSTANCE,
    

    /// Next hop is a VPC network gateway.
    ///
    /// "NEXT_HOP_NETWORK"
    #[serde(rename="NEXT_HOP_NETWORK")]
    NEXTHOPNETWORK,
    

    /// Next hop is a peering VPC.
    ///
    /// "NEXT_HOP_PEERING"
    #[serde(rename="NEXT_HOP_PEERING")]
    NEXTHOPPEERING,
    

    /// Next hop is an interconnect.
    ///
    /// "NEXT_HOP_INTERCONNECT"
    #[serde(rename="NEXT_HOP_INTERCONNECT")]
    NEXTHOPINTERCONNECT,
    

    /// Next hop is a VPN tunnel.
    ///
    /// "NEXT_HOP_VPN_TUNNEL"
    #[serde(rename="NEXT_HOP_VPN_TUNNEL")]
    NEXTHOPVPNTUNNEL,
    

    /// Next hop is a VPN gateway. This scenario only happens when tracing connectivity from an on-premises network to Google Cloud through a VPN. The analysis simulates a packet departing from the on-premises network through a VPN tunnel and arriving at a Cloud VPN gateway.
    ///
    /// "NEXT_HOP_VPN_GATEWAY"
    #[serde(rename="NEXT_HOP_VPN_GATEWAY")]
    NEXTHOPVPNGATEWAY,
    

    /// Next hop is an internet gateway.
    ///
    /// "NEXT_HOP_INTERNET_GATEWAY"
    #[serde(rename="NEXT_HOP_INTERNET_GATEWAY")]
    NEXTHOPINTERNETGATEWAY,
    

    /// Next hop is blackhole; that is, the next hop either does not exist or is not running.
    ///
    /// "NEXT_HOP_BLACKHOLE"
    #[serde(rename="NEXT_HOP_BLACKHOLE")]
    NEXTHOPBLACKHOLE,
    

    /// Next hop is the forwarding rule of an Internal Load Balancer.
    ///
    /// "NEXT_HOP_ILB"
    #[serde(rename="NEXT_HOP_ILB")]
    NEXTHOPILB,
    

    /// Next hop is a [router appliance instance](https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/ra-overview).
    ///
    /// "NEXT_HOP_ROUTER_APPLIANCE"
    #[serde(rename="NEXT_HOP_ROUTER_APPLIANCE")]
    NEXTHOPROUTERAPPLIANCE,
    

    /// Next hop is an NCC hub.
    ///
    /// "NEXT_HOP_NCC_HUB"
    #[serde(rename="NEXT_HOP_NCC_HUB")]
    NEXTHOPNCCHUB,
}

impl AsRef<str> for RouteInfoNextHopTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RouteInfoNextHopTypeEnum::NEXTHOPTYPEUNSPECIFIED => "NEXT_HOP_TYPE_UNSPECIFIED",
            RouteInfoNextHopTypeEnum::NEXTHOPIP => "NEXT_HOP_IP",
            RouteInfoNextHopTypeEnum::NEXTHOPINSTANCE => "NEXT_HOP_INSTANCE",
            RouteInfoNextHopTypeEnum::NEXTHOPNETWORK => "NEXT_HOP_NETWORK",
            RouteInfoNextHopTypeEnum::NEXTHOPPEERING => "NEXT_HOP_PEERING",
            RouteInfoNextHopTypeEnum::NEXTHOPINTERCONNECT => "NEXT_HOP_INTERCONNECT",
            RouteInfoNextHopTypeEnum::NEXTHOPVPNTUNNEL => "NEXT_HOP_VPN_TUNNEL",
            RouteInfoNextHopTypeEnum::NEXTHOPVPNGATEWAY => "NEXT_HOP_VPN_GATEWAY",
            RouteInfoNextHopTypeEnum::NEXTHOPINTERNETGATEWAY => "NEXT_HOP_INTERNET_GATEWAY",
            RouteInfoNextHopTypeEnum::NEXTHOPBLACKHOLE => "NEXT_HOP_BLACKHOLE",
            RouteInfoNextHopTypeEnum::NEXTHOPILB => "NEXT_HOP_ILB",
            RouteInfoNextHopTypeEnum::NEXTHOPROUTERAPPLIANCE => "NEXT_HOP_ROUTER_APPLIANCE",
            RouteInfoNextHopTypeEnum::NEXTHOPNCCHUB => "NEXT_HOP_NCC_HUB",
        }
    }
}

impl std::convert::TryFrom< &str> for RouteInfoNextHopTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NEXT_HOP_TYPE_UNSPECIFIED" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPTYPEUNSPECIFIED),
           "NEXT_HOP_IP" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPIP),
           "NEXT_HOP_INSTANCE" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPINSTANCE),
           "NEXT_HOP_NETWORK" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPNETWORK),
           "NEXT_HOP_PEERING" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPPEERING),
           "NEXT_HOP_INTERCONNECT" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPINTERCONNECT),
           "NEXT_HOP_VPN_TUNNEL" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPVPNTUNNEL),
           "NEXT_HOP_VPN_GATEWAY" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPVPNGATEWAY),
           "NEXT_HOP_INTERNET_GATEWAY" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPINTERNETGATEWAY),
           "NEXT_HOP_BLACKHOLE" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPBLACKHOLE),
           "NEXT_HOP_ILB" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPILB),
           "NEXT_HOP_ROUTER_APPLIANCE" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPROUTERAPPLIANCE),
           "NEXT_HOP_NCC_HUB" => Ok(RouteInfoNextHopTypeEnum::NEXTHOPNCCHUB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RouteInfoNextHopTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RouteInfoRouteScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates where route is applicable.
pub enum RouteInfoRouteScopeEnum {
    

    /// Unspecified scope. Default value.
    ///
    /// "ROUTE_SCOPE_UNSPECIFIED"
    #[serde(rename="ROUTE_SCOPE_UNSPECIFIED")]
    ROUTESCOPEUNSPECIFIED,
    

    /// Route is applicable to packets in Network.
    ///
    /// "NETWORK"
    #[serde(rename="NETWORK")]
    NETWORK,
    

    /// Route is applicable to packets using NCC Hub's routing table.
    ///
    /// "NCC_HUB"
    #[serde(rename="NCC_HUB")]
    NCCHUB,
}

impl AsRef<str> for RouteInfoRouteScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RouteInfoRouteScopeEnum::ROUTESCOPEUNSPECIFIED => "ROUTE_SCOPE_UNSPECIFIED",
            RouteInfoRouteScopeEnum::NETWORK => "NETWORK",
            RouteInfoRouteScopeEnum::NCCHUB => "NCC_HUB",
        }
    }
}

impl std::convert::TryFrom< &str> for RouteInfoRouteScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROUTE_SCOPE_UNSPECIFIED" => Ok(RouteInfoRouteScopeEnum::ROUTESCOPEUNSPECIFIED),
           "NETWORK" => Ok(RouteInfoRouteScopeEnum::NETWORK),
           "NCC_HUB" => Ok(RouteInfoRouteScopeEnum::NCCHUB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RouteInfoRouteScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RouteInfoRouteTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of route.
pub enum RouteInfoRouteTypeEnum {
    

    /// Unspecified type. Default value.
    ///
    /// "ROUTE_TYPE_UNSPECIFIED"
    #[serde(rename="ROUTE_TYPE_UNSPECIFIED")]
    ROUTETYPEUNSPECIFIED,
    

    /// Route is a subnet route automatically created by the system.
    ///
    /// "SUBNET"
    #[serde(rename="SUBNET")]
    SUBNET,
    

    /// Static route created by the user, including the default route to the internet.
    ///
    /// "STATIC"
    #[serde(rename="STATIC")]
    STATIC,
    

    /// Dynamic route exchanged between BGP peers.
    ///
    /// "DYNAMIC"
    #[serde(rename="DYNAMIC")]
    DYNAMIC,
    

    /// A subnet route received from peering network.
    ///
    /// "PEERING_SUBNET"
    #[serde(rename="PEERING_SUBNET")]
    PEERINGSUBNET,
    

    /// A static route received from peering network.
    ///
    /// "PEERING_STATIC"
    #[serde(rename="PEERING_STATIC")]
    PEERINGSTATIC,
    

    /// A dynamic route received from peering network.
    ///
    /// "PEERING_DYNAMIC"
    #[serde(rename="PEERING_DYNAMIC")]
    PEERINGDYNAMIC,
    

    /// Policy based route.
    ///
    /// "POLICY_BASED"
    #[serde(rename="POLICY_BASED")]
    POLICYBASED,
}

impl AsRef<str> for RouteInfoRouteTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RouteInfoRouteTypeEnum::ROUTETYPEUNSPECIFIED => "ROUTE_TYPE_UNSPECIFIED",
            RouteInfoRouteTypeEnum::SUBNET => "SUBNET",
            RouteInfoRouteTypeEnum::STATIC => "STATIC",
            RouteInfoRouteTypeEnum::DYNAMIC => "DYNAMIC",
            RouteInfoRouteTypeEnum::PEERINGSUBNET => "PEERING_SUBNET",
            RouteInfoRouteTypeEnum::PEERINGSTATIC => "PEERING_STATIC",
            RouteInfoRouteTypeEnum::PEERINGDYNAMIC => "PEERING_DYNAMIC",
            RouteInfoRouteTypeEnum::POLICYBASED => "POLICY_BASED",
        }
    }
}

impl std::convert::TryFrom< &str> for RouteInfoRouteTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROUTE_TYPE_UNSPECIFIED" => Ok(RouteInfoRouteTypeEnum::ROUTETYPEUNSPECIFIED),
           "SUBNET" => Ok(RouteInfoRouteTypeEnum::SUBNET),
           "STATIC" => Ok(RouteInfoRouteTypeEnum::STATIC),
           "DYNAMIC" => Ok(RouteInfoRouteTypeEnum::DYNAMIC),
           "PEERING_SUBNET" => Ok(RouteInfoRouteTypeEnum::PEERINGSUBNET),
           "PEERING_STATIC" => Ok(RouteInfoRouteTypeEnum::PEERINGSTATIC),
           "PEERING_DYNAMIC" => Ok(RouteInfoRouteTypeEnum::PEERINGDYNAMIC),
           "POLICY_BASED" => Ok(RouteInfoRouteTypeEnum::POLICYBASED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RouteInfoRouteTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StepStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Each step is in one of the pre-defined states.
pub enum StepStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Initial state: packet originating from a Compute Engine instance. An InstanceInfo is populated with starting instance information.
    ///
    /// "START_FROM_INSTANCE"
    #[serde(rename="START_FROM_INSTANCE")]
    STARTFROMINSTANCE,
    

    /// Initial state: packet originating from the internet. The endpoint information is populated.
    ///
    /// "START_FROM_INTERNET"
    #[serde(rename="START_FROM_INTERNET")]
    STARTFROMINTERNET,
    

    /// Initial state: packet originating from a Google service. The google_service information is populated.
    ///
    /// "START_FROM_GOOGLE_SERVICE"
    #[serde(rename="START_FROM_GOOGLE_SERVICE")]
    STARTFROMGOOGLESERVICE,
    

    /// Initial state: packet originating from a VPC or on-premises network with internal source IP. If the source is a VPC network visible to the user, a NetworkInfo is populated with details of the network.
    ///
    /// "START_FROM_PRIVATE_NETWORK"
    #[serde(rename="START_FROM_PRIVATE_NETWORK")]
    STARTFROMPRIVATENETWORK,
    

    /// Initial state: packet originating from a Google Kubernetes Engine cluster master. A GKEMasterInfo is populated with starting instance information.
    ///
    /// "START_FROM_GKE_MASTER"
    #[serde(rename="START_FROM_GKE_MASTER")]
    STARTFROMGKEMASTER,
    

    /// Initial state: packet originating from a Cloud SQL instance. A CloudSQLInstanceInfo is populated with starting instance information.
    ///
    /// "START_FROM_CLOUD_SQL_INSTANCE"
    #[serde(rename="START_FROM_CLOUD_SQL_INSTANCE")]
    STARTFROMCLOUDSQLINSTANCE,
    

    /// Initial state: packet originating from a Cloud Function. A CloudFunctionInfo is populated with starting function information.
    ///
    /// "START_FROM_CLOUD_FUNCTION"
    #[serde(rename="START_FROM_CLOUD_FUNCTION")]
    STARTFROMCLOUDFUNCTION,
    

    /// Initial state: packet originating from an App Engine service version. An AppEngineVersionInfo is populated with starting version information.
    ///
    /// "START_FROM_APP_ENGINE_VERSION"
    #[serde(rename="START_FROM_APP_ENGINE_VERSION")]
    STARTFROMAPPENGINEVERSION,
    

    /// Initial state: packet originating from a Cloud Run revision. A CloudRunRevisionInfo is populated with starting revision information.
    ///
    /// "START_FROM_CLOUD_RUN_REVISION"
    #[serde(rename="START_FROM_CLOUD_RUN_REVISION")]
    STARTFROMCLOUDRUNREVISION,
    

    /// Initial state: packet originating from a Storage Bucket. Used only for return traces. The storage_bucket information is populated.
    ///
    /// "START_FROM_STORAGE_BUCKET"
    #[serde(rename="START_FROM_STORAGE_BUCKET")]
    STARTFROMSTORAGEBUCKET,
    

    /// Initial state: packet originating from a published service that uses Private Service Connect. Used only for return traces.
    ///
    /// "START_FROM_PSC_PUBLISHED_SERVICE"
    #[serde(rename="START_FROM_PSC_PUBLISHED_SERVICE")]
    STARTFROMPSCPUBLISHEDSERVICE,
    

    /// Config checking state: verify ingress firewall rule.
    ///
    /// "APPLY_INGRESS_FIREWALL_RULE"
    #[serde(rename="APPLY_INGRESS_FIREWALL_RULE")]
    APPLYINGRESSFIREWALLRULE,
    

    /// Config checking state: verify egress firewall rule.
    ///
    /// "APPLY_EGRESS_FIREWALL_RULE"
    #[serde(rename="APPLY_EGRESS_FIREWALL_RULE")]
    APPLYEGRESSFIREWALLRULE,
    

    /// Config checking state: verify route.
    ///
    /// "APPLY_ROUTE"
    #[serde(rename="APPLY_ROUTE")]
    APPLYROUTE,
    

    /// Config checking state: match forwarding rule.
    ///
    /// "APPLY_FORWARDING_RULE"
    #[serde(rename="APPLY_FORWARDING_RULE")]
    APPLYFORWARDINGRULE,
    

    /// Config checking state: verify load balancer backend configuration.
    ///
    /// "ANALYZE_LOAD_BALANCER_BACKEND"
    #[serde(rename="ANALYZE_LOAD_BALANCER_BACKEND")]
    ANALYZELOADBALANCERBACKEND,
    

    /// Config checking state: packet sent or received under foreign IP address and allowed.
    ///
    /// "SPOOFING_APPROVED"
    #[serde(rename="SPOOFING_APPROVED")]
    SPOOFINGAPPROVED,
    

    /// Forwarding state: arriving at a Compute Engine instance.
    ///
    /// "ARRIVE_AT_INSTANCE"
    #[serde(rename="ARRIVE_AT_INSTANCE")]
    ARRIVEATINSTANCE,
    

    /// Forwarding state: arriving at a Compute Engine internal load balancer.
    ///
    /// "ARRIVE_AT_INTERNAL_LOAD_BALANCER"
    #[serde(rename="ARRIVE_AT_INTERNAL_LOAD_BALANCER")]
    ARRIVEATINTERNALLOADBALANCER,
    

    /// Forwarding state: arriving at a Compute Engine external load balancer.
    ///
    /// "ARRIVE_AT_EXTERNAL_LOAD_BALANCER"
    #[serde(rename="ARRIVE_AT_EXTERNAL_LOAD_BALANCER")]
    ARRIVEATEXTERNALLOADBALANCER,
    

    /// Forwarding state: arriving at a Cloud VPN gateway.
    ///
    /// "ARRIVE_AT_VPN_GATEWAY"
    #[serde(rename="ARRIVE_AT_VPN_GATEWAY")]
    ARRIVEATVPNGATEWAY,
    

    /// Forwarding state: arriving at a Cloud VPN tunnel.
    ///
    /// "ARRIVE_AT_VPN_TUNNEL"
    #[serde(rename="ARRIVE_AT_VPN_TUNNEL")]
    ARRIVEATVPNTUNNEL,
    

    /// Forwarding state: arriving at a VPC connector.
    ///
    /// "ARRIVE_AT_VPC_CONNECTOR"
    #[serde(rename="ARRIVE_AT_VPC_CONNECTOR")]
    ARRIVEATVPCCONNECTOR,
    

    /// Transition state: packet header translated.
    ///
    /// "NAT"
    #[serde(rename="NAT")]
    NAT,
    

    /// Transition state: original connection is terminated and a new proxied connection is initiated.
    ///
    /// "PROXY_CONNECTION"
    #[serde(rename="PROXY_CONNECTION")]
    PROXYCONNECTION,
    

    /// Final state: packet could be delivered.
    ///
    /// "DELIVER"
    #[serde(rename="DELIVER")]
    DELIVER,
    

    /// Final state: packet could be dropped.
    ///
    /// "DROP"
    #[serde(rename="DROP")]
    DROP,
    

    /// Final state: packet could be forwarded to a network with an unknown configuration.
    ///
    /// "FORWARD"
    #[serde(rename="FORWARD")]
    FORWARD,
    

    /// Final state: analysis is aborted.
    ///
    /// "ABORT"
    #[serde(rename="ABORT")]
    ABORT,
    

    /// Special state: viewer of the test result does not have permission to see the configuration in this step.
    ///
    /// "VIEWER_PERMISSION_MISSING"
    #[serde(rename="VIEWER_PERMISSION_MISSING")]
    VIEWERPERMISSIONMISSING,
}

impl AsRef<str> for StepStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StepStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            StepStateEnum::STARTFROMINSTANCE => "START_FROM_INSTANCE",
            StepStateEnum::STARTFROMINTERNET => "START_FROM_INTERNET",
            StepStateEnum::STARTFROMGOOGLESERVICE => "START_FROM_GOOGLE_SERVICE",
            StepStateEnum::STARTFROMPRIVATENETWORK => "START_FROM_PRIVATE_NETWORK",
            StepStateEnum::STARTFROMGKEMASTER => "START_FROM_GKE_MASTER",
            StepStateEnum::STARTFROMCLOUDSQLINSTANCE => "START_FROM_CLOUD_SQL_INSTANCE",
            StepStateEnum::STARTFROMCLOUDFUNCTION => "START_FROM_CLOUD_FUNCTION",
            StepStateEnum::STARTFROMAPPENGINEVERSION => "START_FROM_APP_ENGINE_VERSION",
            StepStateEnum::STARTFROMCLOUDRUNREVISION => "START_FROM_CLOUD_RUN_REVISION",
            StepStateEnum::STARTFROMSTORAGEBUCKET => "START_FROM_STORAGE_BUCKET",
            StepStateEnum::STARTFROMPSCPUBLISHEDSERVICE => "START_FROM_PSC_PUBLISHED_SERVICE",
            StepStateEnum::APPLYINGRESSFIREWALLRULE => "APPLY_INGRESS_FIREWALL_RULE",
            StepStateEnum::APPLYEGRESSFIREWALLRULE => "APPLY_EGRESS_FIREWALL_RULE",
            StepStateEnum::APPLYROUTE => "APPLY_ROUTE",
            StepStateEnum::APPLYFORWARDINGRULE => "APPLY_FORWARDING_RULE",
            StepStateEnum::ANALYZELOADBALANCERBACKEND => "ANALYZE_LOAD_BALANCER_BACKEND",
            StepStateEnum::SPOOFINGAPPROVED => "SPOOFING_APPROVED",
            StepStateEnum::ARRIVEATINSTANCE => "ARRIVE_AT_INSTANCE",
            StepStateEnum::ARRIVEATINTERNALLOADBALANCER => "ARRIVE_AT_INTERNAL_LOAD_BALANCER",
            StepStateEnum::ARRIVEATEXTERNALLOADBALANCER => "ARRIVE_AT_EXTERNAL_LOAD_BALANCER",
            StepStateEnum::ARRIVEATVPNGATEWAY => "ARRIVE_AT_VPN_GATEWAY",
            StepStateEnum::ARRIVEATVPNTUNNEL => "ARRIVE_AT_VPN_TUNNEL",
            StepStateEnum::ARRIVEATVPCCONNECTOR => "ARRIVE_AT_VPC_CONNECTOR",
            StepStateEnum::NAT => "NAT",
            StepStateEnum::PROXYCONNECTION => "PROXY_CONNECTION",
            StepStateEnum::DELIVER => "DELIVER",
            StepStateEnum::DROP => "DROP",
            StepStateEnum::FORWARD => "FORWARD",
            StepStateEnum::ABORT => "ABORT",
            StepStateEnum::VIEWERPERMISSIONMISSING => "VIEWER_PERMISSION_MISSING",
        }
    }
}

impl std::convert::TryFrom< &str> for StepStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(StepStateEnum::STATEUNSPECIFIED),
           "START_FROM_INSTANCE" => Ok(StepStateEnum::STARTFROMINSTANCE),
           "START_FROM_INTERNET" => Ok(StepStateEnum::STARTFROMINTERNET),
           "START_FROM_GOOGLE_SERVICE" => Ok(StepStateEnum::STARTFROMGOOGLESERVICE),
           "START_FROM_PRIVATE_NETWORK" => Ok(StepStateEnum::STARTFROMPRIVATENETWORK),
           "START_FROM_GKE_MASTER" => Ok(StepStateEnum::STARTFROMGKEMASTER),
           "START_FROM_CLOUD_SQL_INSTANCE" => Ok(StepStateEnum::STARTFROMCLOUDSQLINSTANCE),
           "START_FROM_CLOUD_FUNCTION" => Ok(StepStateEnum::STARTFROMCLOUDFUNCTION),
           "START_FROM_APP_ENGINE_VERSION" => Ok(StepStateEnum::STARTFROMAPPENGINEVERSION),
           "START_FROM_CLOUD_RUN_REVISION" => Ok(StepStateEnum::STARTFROMCLOUDRUNREVISION),
           "START_FROM_STORAGE_BUCKET" => Ok(StepStateEnum::STARTFROMSTORAGEBUCKET),
           "START_FROM_PSC_PUBLISHED_SERVICE" => Ok(StepStateEnum::STARTFROMPSCPUBLISHEDSERVICE),
           "APPLY_INGRESS_FIREWALL_RULE" => Ok(StepStateEnum::APPLYINGRESSFIREWALLRULE),
           "APPLY_EGRESS_FIREWALL_RULE" => Ok(StepStateEnum::APPLYEGRESSFIREWALLRULE),
           "APPLY_ROUTE" => Ok(StepStateEnum::APPLYROUTE),
           "APPLY_FORWARDING_RULE" => Ok(StepStateEnum::APPLYFORWARDINGRULE),
           "ANALYZE_LOAD_BALANCER_BACKEND" => Ok(StepStateEnum::ANALYZELOADBALANCERBACKEND),
           "SPOOFING_APPROVED" => Ok(StepStateEnum::SPOOFINGAPPROVED),
           "ARRIVE_AT_INSTANCE" => Ok(StepStateEnum::ARRIVEATINSTANCE),
           "ARRIVE_AT_INTERNAL_LOAD_BALANCER" => Ok(StepStateEnum::ARRIVEATINTERNALLOADBALANCER),
           "ARRIVE_AT_EXTERNAL_LOAD_BALANCER" => Ok(StepStateEnum::ARRIVEATEXTERNALLOADBALANCER),
           "ARRIVE_AT_VPN_GATEWAY" => Ok(StepStateEnum::ARRIVEATVPNGATEWAY),
           "ARRIVE_AT_VPN_TUNNEL" => Ok(StepStateEnum::ARRIVEATVPNTUNNEL),
           "ARRIVE_AT_VPC_CONNECTOR" => Ok(StepStateEnum::ARRIVEATVPCCONNECTOR),
           "NAT" => Ok(StepStateEnum::NAT),
           "PROXY_CONNECTION" => Ok(StepStateEnum::PROXYCONNECTION),
           "DELIVER" => Ok(StepStateEnum::DELIVER),
           "DROP" => Ok(StepStateEnum::DROP),
           "FORWARD" => Ok(StepStateEnum::FORWARD),
           "ABORT" => Ok(StepStateEnum::ABORT),
           "VIEWER_PERMISSION_MISSING" => Ok(StepStateEnum::VIEWERPERMISSIONMISSING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StepStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VpnTunnelInfoRoutingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the routing policy.
pub enum VpnTunnelInfoRoutingTypeEnum {
    

    /// Unspecified type. Default value.
    ///
    /// "ROUTING_TYPE_UNSPECIFIED"
    #[serde(rename="ROUTING_TYPE_UNSPECIFIED")]
    ROUTINGTYPEUNSPECIFIED,
    

    /// Route based VPN.
    ///
    /// "ROUTE_BASED"
    #[serde(rename="ROUTE_BASED")]
    ROUTEBASED,
    

    /// Policy based routing.
    ///
    /// "POLICY_BASED"
    #[serde(rename="POLICY_BASED")]
    POLICYBASED,
    

    /// Dynamic (BGP) routing.
    ///
    /// "DYNAMIC"
    #[serde(rename="DYNAMIC")]
    DYNAMIC,
}

impl AsRef<str> for VpnTunnelInfoRoutingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VpnTunnelInfoRoutingTypeEnum::ROUTINGTYPEUNSPECIFIED => "ROUTING_TYPE_UNSPECIFIED",
            VpnTunnelInfoRoutingTypeEnum::ROUTEBASED => "ROUTE_BASED",
            VpnTunnelInfoRoutingTypeEnum::POLICYBASED => "POLICY_BASED",
            VpnTunnelInfoRoutingTypeEnum::DYNAMIC => "DYNAMIC",
        }
    }
}

impl std::convert::TryFrom< &str> for VpnTunnelInfoRoutingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROUTING_TYPE_UNSPECIFIED" => Ok(VpnTunnelInfoRoutingTypeEnum::ROUTINGTYPEUNSPECIFIED),
           "ROUTE_BASED" => Ok(VpnTunnelInfoRoutingTypeEnum::ROUTEBASED),
           "POLICY_BASED" => Ok(VpnTunnelInfoRoutingTypeEnum::POLICYBASED),
           "DYNAMIC" => Ok(VpnTunnelInfoRoutingTypeEnum::DYNAMIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VpnTunnelInfoRoutingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


