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


// region ConsumerPscConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Overall state of PSC Connections management for this consumer psc config.
pub enum ConsumerPscConfigStateEnum {
    

    /// Default state, when Connection Map is created initially.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Set when policy and map configuration is valid, and their matching can lead to allowing creation of PSC Connections subject to other constraints like connections limit.
    ///
    /// "VALID"
    #[serde(rename="VALID")]
    VALID,
    

    /// No Service Connection Policy found for this network and Service Class
    ///
    /// "CONNECTION_POLICY_MISSING"
    #[serde(rename="CONNECTION_POLICY_MISSING")]
    CONNECTIONPOLICYMISSING,
    

    /// Service Connection Policy limit reached for this network and Service Class
    ///
    /// "POLICY_LIMIT_REACHED"
    #[serde(rename="POLICY_LIMIT_REACHED")]
    POLICYLIMITREACHED,
}

impl AsRef<str> for ConsumerPscConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConsumerPscConfigStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ConsumerPscConfigStateEnum::VALID => "VALID",
            ConsumerPscConfigStateEnum::CONNECTIONPOLICYMISSING => "CONNECTION_POLICY_MISSING",
            ConsumerPscConfigStateEnum::POLICYLIMITREACHED => "POLICY_LIMIT_REACHED",
        }
    }
}

impl std::convert::TryFrom< &str> for ConsumerPscConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ConsumerPscConfigStateEnum::STATEUNSPECIFIED),
           "VALID" => Ok(ConsumerPscConfigStateEnum::VALID),
           "CONNECTION_POLICY_MISSING" => Ok(ConsumerPscConfigStateEnum::CONNECTIONPOLICYMISSING),
           "POLICY_LIMIT_REACHED" => Ok(ConsumerPscConfigStateEnum::POLICYLIMITREACHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConsumerPscConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConsumerPscConnectionErrorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The error type indicates whether the error is consumer facing, producer facing or system internal.
pub enum ConsumerPscConnectionErrorTypeEnum {
    

    /// An invalid error type as the default case.
    ///
    /// "CONNECTION_ERROR_TYPE_UNSPECIFIED"
    #[serde(rename="CONNECTION_ERROR_TYPE_UNSPECIFIED")]
    CONNECTIONERRORTYPEUNSPECIFIED,
    

    /// The error is due to Service Automation system internal.
    ///
    /// "ERROR_INTERNAL"
    #[serde(rename="ERROR_INTERNAL")]
    ERRORINTERNAL,
    

    /// The error is due to the setup on consumer side.
    ///
    /// "ERROR_CONSUMER_SIDE"
    #[serde(rename="ERROR_CONSUMER_SIDE")]
    ERRORCONSUMERSIDE,
    

    /// The error is due to the setup on producer side.
    ///
    /// "ERROR_PRODUCER_SIDE"
    #[serde(rename="ERROR_PRODUCER_SIDE")]
    ERRORPRODUCERSIDE,
}

impl AsRef<str> for ConsumerPscConnectionErrorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConsumerPscConnectionErrorTypeEnum::CONNECTIONERRORTYPEUNSPECIFIED => "CONNECTION_ERROR_TYPE_UNSPECIFIED",
            ConsumerPscConnectionErrorTypeEnum::ERRORINTERNAL => "ERROR_INTERNAL",
            ConsumerPscConnectionErrorTypeEnum::ERRORCONSUMERSIDE => "ERROR_CONSUMER_SIDE",
            ConsumerPscConnectionErrorTypeEnum::ERRORPRODUCERSIDE => "ERROR_PRODUCER_SIDE",
        }
    }
}

impl std::convert::TryFrom< &str> for ConsumerPscConnectionErrorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTION_ERROR_TYPE_UNSPECIFIED" => Ok(ConsumerPscConnectionErrorTypeEnum::CONNECTIONERRORTYPEUNSPECIFIED),
           "ERROR_INTERNAL" => Ok(ConsumerPscConnectionErrorTypeEnum::ERRORINTERNAL),
           "ERROR_CONSUMER_SIDE" => Ok(ConsumerPscConnectionErrorTypeEnum::ERRORCONSUMERSIDE),
           "ERROR_PRODUCER_SIDE" => Ok(ConsumerPscConnectionErrorTypeEnum::ERRORPRODUCERSIDE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConsumerPscConnectionErrorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConsumerPscConnectionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the PSC connection.
pub enum ConsumerPscConnectionStateEnum {
    

    /// An invalid state as the default case.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The connection is fully established and ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The connection is not functional since some resources on the connection fail to be created.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The connection is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The connection is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for ConsumerPscConnectionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConsumerPscConnectionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ConsumerPscConnectionStateEnum::ACTIVE => "ACTIVE",
            ConsumerPscConnectionStateEnum::FAILED => "FAILED",
            ConsumerPscConnectionStateEnum::CREATING => "CREATING",
            ConsumerPscConnectionStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for ConsumerPscConnectionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ConsumerPscConnectionStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(ConsumerPscConnectionStateEnum::ACTIVE),
           "FAILED" => Ok(ConsumerPscConnectionStateEnum::FAILED),
           "CREATING" => Ok(ConsumerPscConnectionStateEnum::CREATING),
           "DELETING" => Ok(ConsumerPscConnectionStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConsumerPscConnectionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FilterProtocolVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Internet protocol versions this policy-based route applies to. For this version, only IPV4 is supported.
pub enum FilterProtocolVersionEnum {
    

    /// Default value.
    ///
    /// "PROTOCOL_VERSION_UNSPECIFIED"
    #[serde(rename="PROTOCOL_VERSION_UNSPECIFIED")]
    PROTOCOLVERSIONUNSPECIFIED,
    

    /// The PBR is for IPv4 internet protocol traffic.
    ///
    /// "IPV4"
    #[serde(rename="IPV4")]
    IPV4,
}

impl AsRef<str> for FilterProtocolVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FilterProtocolVersionEnum::PROTOCOLVERSIONUNSPECIFIED => "PROTOCOL_VERSION_UNSPECIFIED",
            FilterProtocolVersionEnum::IPV4 => "IPV4",
        }
    }
}

impl std::convert::TryFrom< &str> for FilterProtocolVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTOCOL_VERSION_UNSPECIFIED" => Ok(FilterProtocolVersionEnum::PROTOCOLVERSIONUNSPECIFIED),
           "IPV4" => Ok(FilterProtocolVersionEnum::IPV4),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FilterProtocolVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GroupStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current lifecycle state of this group.
pub enum GroupStateEnum {
    

    /// No state information available
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The resource's create operation is in progress.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The resource is active
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource's delete operation is in progress.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource's accept operation is in progress.
    ///
    /// "ACCEPTING"
    #[serde(rename="ACCEPTING")]
    ACCEPTING,
    

    /// The resource's reject operation is in progress.
    ///
    /// "REJECTING"
    #[serde(rename="REJECTING")]
    REJECTING,
    

    /// The resource's update operation is in progress.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The resource is inactive.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// The hub associated with this spoke resource has been deleted. This state applies to spoke resources only.
    ///
    /// "OBSOLETE"
    #[serde(rename="OBSOLETE")]
    OBSOLETE,
}

impl AsRef<str> for GroupStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GroupStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GroupStateEnum::CREATING => "CREATING",
            GroupStateEnum::ACTIVE => "ACTIVE",
            GroupStateEnum::DELETING => "DELETING",
            GroupStateEnum::ACCEPTING => "ACCEPTING",
            GroupStateEnum::REJECTING => "REJECTING",
            GroupStateEnum::UPDATING => "UPDATING",
            GroupStateEnum::INACTIVE => "INACTIVE",
            GroupStateEnum::OBSOLETE => "OBSOLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for GroupStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GroupStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GroupStateEnum::CREATING),
           "ACTIVE" => Ok(GroupStateEnum::ACTIVE),
           "DELETING" => Ok(GroupStateEnum::DELETING),
           "ACCEPTING" => Ok(GroupStateEnum::ACCEPTING),
           "REJECTING" => Ok(GroupStateEnum::REJECTING),
           "UPDATING" => Ok(GroupStateEnum::UPDATING),
           "INACTIVE" => Ok(GroupStateEnum::INACTIVE),
           "OBSOLETE" => Ok(GroupStateEnum::OBSOLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GroupStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HubStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current lifecycle state of this hub.
pub enum HubStateEnum {
    

    /// No state information available
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The resource's create operation is in progress.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The resource is active
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource's delete operation is in progress.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource's accept operation is in progress.
    ///
    /// "ACCEPTING"
    #[serde(rename="ACCEPTING")]
    ACCEPTING,
    

    /// The resource's reject operation is in progress.
    ///
    /// "REJECTING"
    #[serde(rename="REJECTING")]
    REJECTING,
    

    /// The resource's update operation is in progress.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The resource is inactive.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// The hub associated with this spoke resource has been deleted. This state applies to spoke resources only.
    ///
    /// "OBSOLETE"
    #[serde(rename="OBSOLETE")]
    OBSOLETE,
}

impl AsRef<str> for HubStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HubStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            HubStateEnum::CREATING => "CREATING",
            HubStateEnum::ACTIVE => "ACTIVE",
            HubStateEnum::DELETING => "DELETING",
            HubStateEnum::ACCEPTING => "ACCEPTING",
            HubStateEnum::REJECTING => "REJECTING",
            HubStateEnum::UPDATING => "UPDATING",
            HubStateEnum::INACTIVE => "INACTIVE",
            HubStateEnum::OBSOLETE => "OBSOLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for HubStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(HubStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(HubStateEnum::CREATING),
           "ACTIVE" => Ok(HubStateEnum::ACTIVE),
           "DELETING" => Ok(HubStateEnum::DELETING),
           "ACCEPTING" => Ok(HubStateEnum::ACCEPTING),
           "REJECTING" => Ok(HubStateEnum::REJECTING),
           "UPDATING" => Ok(HubStateEnum::UPDATING),
           "INACTIVE" => Ok(HubStateEnum::INACTIVE),
           "OBSOLETE" => Ok(HubStateEnum::OBSOLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HubStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InternalRangeOverlapsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Types of resources that are allowed to overlap with the current internal range.
pub enum InternalRangeOverlapsEnum {
    

    /// No overlap overrides.
    ///
    /// "OVERLAP_UNSPECIFIED"
    #[serde(rename="OVERLAP_UNSPECIFIED")]
    OVERLAPUNSPECIFIED,
    

    /// Allow creation of static routes more specific that the current internal range.
    ///
    /// "OVERLAP_ROUTE_RANGE"
    #[serde(rename="OVERLAP_ROUTE_RANGE")]
    OVERLAPROUTERANGE,
    

    /// Allow creation of internal ranges that overlap with existing subnets.
    ///
    /// "OVERLAP_EXISTING_SUBNET_RANGE"
    #[serde(rename="OVERLAP_EXISTING_SUBNET_RANGE")]
    OVERLAPEXISTINGSUBNETRANGE,
}

impl AsRef<str> for InternalRangeOverlapsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InternalRangeOverlapsEnum::OVERLAPUNSPECIFIED => "OVERLAP_UNSPECIFIED",
            InternalRangeOverlapsEnum::OVERLAPROUTERANGE => "OVERLAP_ROUTE_RANGE",
            InternalRangeOverlapsEnum::OVERLAPEXISTINGSUBNETRANGE => "OVERLAP_EXISTING_SUBNET_RANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for InternalRangeOverlapsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OVERLAP_UNSPECIFIED" => Ok(InternalRangeOverlapsEnum::OVERLAPUNSPECIFIED),
           "OVERLAP_ROUTE_RANGE" => Ok(InternalRangeOverlapsEnum::OVERLAPROUTERANGE),
           "OVERLAP_EXISTING_SUBNET_RANGE" => Ok(InternalRangeOverlapsEnum::OVERLAPEXISTINGSUBNETRANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InternalRangeOverlapsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InternalRangePeeringEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of peering set for this internal range.
pub enum InternalRangePeeringEnum {
    

    /// If Peering is left unspecified in CreateInternalRange or UpdateInternalRange, it will be defaulted to FOR_SELF.
    ///
    /// "PEERING_UNSPECIFIED"
    #[serde(rename="PEERING_UNSPECIFIED")]
    PEERINGUNSPECIFIED,
    

    /// This is the default behavior and represents the case that this internal range is intended to be used in the VPC in which it is created and is accessible from its peers. This implies that peers or peers-of-peers cannot use this range.
    ///
    /// "FOR_SELF"
    #[serde(rename="FOR_SELF")]
    FORSELF,
    

    /// This behavior can be set when the internal range is being reserved for usage by peers. This means that no resource within the VPC in which it is being created can use this to associate with a VPC resource, but one of the peers can. This represents donating a range for peers to use.
    ///
    /// "FOR_PEER"
    #[serde(rename="FOR_PEER")]
    FORPEER,
    

    /// This behavior can be set when the internal range is being reserved for usage by the VPC in which it is created, but not shared with peers. In a sense, it is local to the VPC. This can be used to create internal ranges for various purposes like HTTP_INTERNAL_LOAD_BALANCER or for Interconnect routes that are not shared with peers. This also implies that peers cannot use this range in a way that is visible to this VPC, but can re-use this range as long as it is NOT_SHARED from the peer VPC, too.
    ///
    /// "NOT_SHARED"
    #[serde(rename="NOT_SHARED")]
    NOTSHARED,
}

impl AsRef<str> for InternalRangePeeringEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InternalRangePeeringEnum::PEERINGUNSPECIFIED => "PEERING_UNSPECIFIED",
            InternalRangePeeringEnum::FORSELF => "FOR_SELF",
            InternalRangePeeringEnum::FORPEER => "FOR_PEER",
            InternalRangePeeringEnum::NOTSHARED => "NOT_SHARED",
        }
    }
}

impl std::convert::TryFrom< &str> for InternalRangePeeringEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PEERING_UNSPECIFIED" => Ok(InternalRangePeeringEnum::PEERINGUNSPECIFIED),
           "FOR_SELF" => Ok(InternalRangePeeringEnum::FORSELF),
           "FOR_PEER" => Ok(InternalRangePeeringEnum::FORPEER),
           "NOT_SHARED" => Ok(InternalRangePeeringEnum::NOTSHARED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InternalRangePeeringEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InternalRangeUsageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of usage set for this InternalRange.
pub enum InternalRangeUsageEnum {
    

    /// Unspecified usage is allowed in calls which identify the resource by other fields and do not need Usage set to complete. These are, i.e.: GetInternalRange and DeleteInternalRange. Usage needs to be specified explicitly in CreateInternalRange or UpdateInternalRange calls.
    ///
    /// "USAGE_UNSPECIFIED"
    #[serde(rename="USAGE_UNSPECIFIED")]
    USAGEUNSPECIFIED,
    

    /// A VPC resource can use the reserved CIDR block by associating it with the internal range resource if usage is set to FOR_VPC.
    ///
    /// "FOR_VPC"
    #[serde(rename="FOR_VPC")]
    FORVPC,
    

    /// Ranges created with EXTERNAL_TO_VPC cannot be associated with VPC resources and are meant to block out address ranges for various use cases, like for example, usage on-prem, with dynamic route announcements via interconnect.
    ///
    /// "EXTERNAL_TO_VPC"
    #[serde(rename="EXTERNAL_TO_VPC")]
    EXTERNALTOVPC,
}

impl AsRef<str> for InternalRangeUsageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InternalRangeUsageEnum::USAGEUNSPECIFIED => "USAGE_UNSPECIFIED",
            InternalRangeUsageEnum::FORVPC => "FOR_VPC",
            InternalRangeUsageEnum::EXTERNALTOVPC => "EXTERNAL_TO_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for InternalRangeUsageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USAGE_UNSPECIFIED" => Ok(InternalRangeUsageEnum::USAGEUNSPECIFIED),
           "FOR_VPC" => Ok(InternalRangeUsageEnum::FORVPC),
           "EXTERNAL_TO_VPC" => Ok(InternalRangeUsageEnum::EXTERNALTOVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InternalRangeUsageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyBasedRouteNextHopOtherRoutesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Other routes that will be referenced to determine the next hop of the packet.
pub enum PolicyBasedRouteNextHopOtherRoutesEnum {
    

    /// Default value.
    ///
    /// "OTHER_ROUTES_UNSPECIFIED"
    #[serde(rename="OTHER_ROUTES_UNSPECIFIED")]
    OTHERROUTESUNSPECIFIED,
    

    /// Use the routes from the default routing tables (system-generated routes, custom routes, peering route) to determine the next hop. This will effectively exclude matching packets being applied on other PBRs with a lower priority.
    ///
    /// "DEFAULT_ROUTING"
    #[serde(rename="DEFAULT_ROUTING")]
    DEFAULTROUTING,
}

impl AsRef<str> for PolicyBasedRouteNextHopOtherRoutesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyBasedRouteNextHopOtherRoutesEnum::OTHERROUTESUNSPECIFIED => "OTHER_ROUTES_UNSPECIFIED",
            PolicyBasedRouteNextHopOtherRoutesEnum::DEFAULTROUTING => "DEFAULT_ROUTING",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyBasedRouteNextHopOtherRoutesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OTHER_ROUTES_UNSPECIFIED" => Ok(PolicyBasedRouteNextHopOtherRoutesEnum::OTHERROUTESUNSPECIFIED),
           "DEFAULT_ROUTING" => Ok(PolicyBasedRouteNextHopOtherRoutesEnum::DEFAULTROUTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyBasedRouteNextHopOtherRoutesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PscConnectionErrorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The error type indicates whether the error is consumer facing, producer facing or system internal.
pub enum PscConnectionErrorTypeEnum {
    

    /// An invalid error type as the default case.
    ///
    /// "CONNECTION_ERROR_TYPE_UNSPECIFIED"
    #[serde(rename="CONNECTION_ERROR_TYPE_UNSPECIFIED")]
    CONNECTIONERRORTYPEUNSPECIFIED,
    

    /// The error is due to Service Automation system internal.
    ///
    /// "ERROR_INTERNAL"
    #[serde(rename="ERROR_INTERNAL")]
    ERRORINTERNAL,
    

    /// The error is due to the setup on consumer side.
    ///
    /// "ERROR_CONSUMER_SIDE"
    #[serde(rename="ERROR_CONSUMER_SIDE")]
    ERRORCONSUMERSIDE,
    

    /// The error is due to the setup on producer side.
    ///
    /// "ERROR_PRODUCER_SIDE"
    #[serde(rename="ERROR_PRODUCER_SIDE")]
    ERRORPRODUCERSIDE,
}

impl AsRef<str> for PscConnectionErrorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PscConnectionErrorTypeEnum::CONNECTIONERRORTYPEUNSPECIFIED => "CONNECTION_ERROR_TYPE_UNSPECIFIED",
            PscConnectionErrorTypeEnum::ERRORINTERNAL => "ERROR_INTERNAL",
            PscConnectionErrorTypeEnum::ERRORCONSUMERSIDE => "ERROR_CONSUMER_SIDE",
            PscConnectionErrorTypeEnum::ERRORPRODUCERSIDE => "ERROR_PRODUCER_SIDE",
        }
    }
}

impl std::convert::TryFrom< &str> for PscConnectionErrorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTION_ERROR_TYPE_UNSPECIFIED" => Ok(PscConnectionErrorTypeEnum::CONNECTIONERRORTYPEUNSPECIFIED),
           "ERROR_INTERNAL" => Ok(PscConnectionErrorTypeEnum::ERRORINTERNAL),
           "ERROR_CONSUMER_SIDE" => Ok(PscConnectionErrorTypeEnum::ERRORCONSUMERSIDE),
           "ERROR_PRODUCER_SIDE" => Ok(PscConnectionErrorTypeEnum::ERRORPRODUCERSIDE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PscConnectionErrorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PscConnectionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the PSC Connection
pub enum PscConnectionStateEnum {
    

    /// An invalid state as the default case.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The connection is fully established and ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The connection is not functional since some resources on the connection fail to be created.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The connection is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The connection is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for PscConnectionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PscConnectionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            PscConnectionStateEnum::ACTIVE => "ACTIVE",
            PscConnectionStateEnum::FAILED => "FAILED",
            PscConnectionStateEnum::CREATING => "CREATING",
            PscConnectionStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for PscConnectionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(PscConnectionStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(PscConnectionStateEnum::ACTIVE),
           "FAILED" => Ok(PscConnectionStateEnum::FAILED),
           "CREATING" => Ok(PscConnectionStateEnum::CREATING),
           "DELETING" => Ok(PscConnectionStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PscConnectionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RouteStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current lifecycle state of the route.
pub enum RouteStateEnum {
    

    /// No state information available
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The resource's create operation is in progress.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The resource is active
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource's delete operation is in progress.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource's accept operation is in progress.
    ///
    /// "ACCEPTING"
    #[serde(rename="ACCEPTING")]
    ACCEPTING,
    

    /// The resource's reject operation is in progress.
    ///
    /// "REJECTING"
    #[serde(rename="REJECTING")]
    REJECTING,
    

    /// The resource's update operation is in progress.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The resource is inactive.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// The hub associated with this spoke resource has been deleted. This state applies to spoke resources only.
    ///
    /// "OBSOLETE"
    #[serde(rename="OBSOLETE")]
    OBSOLETE,
}

impl AsRef<str> for RouteStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RouteStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            RouteStateEnum::CREATING => "CREATING",
            RouteStateEnum::ACTIVE => "ACTIVE",
            RouteStateEnum::DELETING => "DELETING",
            RouteStateEnum::ACCEPTING => "ACCEPTING",
            RouteStateEnum::REJECTING => "REJECTING",
            RouteStateEnum::UPDATING => "UPDATING",
            RouteStateEnum::INACTIVE => "INACTIVE",
            RouteStateEnum::OBSOLETE => "OBSOLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for RouteStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(RouteStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(RouteStateEnum::CREATING),
           "ACTIVE" => Ok(RouteStateEnum::ACTIVE),
           "DELETING" => Ok(RouteStateEnum::DELETING),
           "ACCEPTING" => Ok(RouteStateEnum::ACCEPTING),
           "REJECTING" => Ok(RouteStateEnum::REJECTING),
           "UPDATING" => Ok(RouteStateEnum::UPDATING),
           "INACTIVE" => Ok(RouteStateEnum::INACTIVE),
           "OBSOLETE" => Ok(RouteStateEnum::OBSOLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RouteStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RouteTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The route's type. Its type is determined by the properties of its IP address range.
pub enum RouteTypeEnum {
    

    /// No route type information specified
    ///
    /// "ROUTE_TYPE_UNSPECIFIED"
    #[serde(rename="ROUTE_TYPE_UNSPECIFIED")]
    ROUTETYPEUNSPECIFIED,
    

    /// The route leads to a destination within the primary address range of the VPC network's subnet.
    ///
    /// "VPC_PRIMARY_SUBNET"
    #[serde(rename="VPC_PRIMARY_SUBNET")]
    VPCPRIMARYSUBNET,
    

    /// The route leads to a destination within the secondary address range of the VPC network's subnet.
    ///
    /// "VPC_SECONDARY_SUBNET"
    #[serde(rename="VPC_SECONDARY_SUBNET")]
    VPCSECONDARYSUBNET,
}

impl AsRef<str> for RouteTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RouteTypeEnum::ROUTETYPEUNSPECIFIED => "ROUTE_TYPE_UNSPECIFIED",
            RouteTypeEnum::VPCPRIMARYSUBNET => "VPC_PRIMARY_SUBNET",
            RouteTypeEnum::VPCSECONDARYSUBNET => "VPC_SECONDARY_SUBNET",
        }
    }
}

impl std::convert::TryFrom< &str> for RouteTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROUTE_TYPE_UNSPECIFIED" => Ok(RouteTypeEnum::ROUTETYPEUNSPECIFIED),
           "VPC_PRIMARY_SUBNET" => Ok(RouteTypeEnum::VPCPRIMARYSUBNET),
           "VPC_SECONDARY_SUBNET" => Ok(RouteTypeEnum::VPCSECONDARYSUBNET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RouteTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RouteTableStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current lifecycle state of this route table.
pub enum RouteTableStateEnum {
    

    /// No state information available
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The resource's create operation is in progress.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The resource is active
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource's delete operation is in progress.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource's accept operation is in progress.
    ///
    /// "ACCEPTING"
    #[serde(rename="ACCEPTING")]
    ACCEPTING,
    

    /// The resource's reject operation is in progress.
    ///
    /// "REJECTING"
    #[serde(rename="REJECTING")]
    REJECTING,
    

    /// The resource's update operation is in progress.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The resource is inactive.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// The hub associated with this spoke resource has been deleted. This state applies to spoke resources only.
    ///
    /// "OBSOLETE"
    #[serde(rename="OBSOLETE")]
    OBSOLETE,
}

impl AsRef<str> for RouteTableStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RouteTableStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            RouteTableStateEnum::CREATING => "CREATING",
            RouteTableStateEnum::ACTIVE => "ACTIVE",
            RouteTableStateEnum::DELETING => "DELETING",
            RouteTableStateEnum::ACCEPTING => "ACCEPTING",
            RouteTableStateEnum::REJECTING => "REJECTING",
            RouteTableStateEnum::UPDATING => "UPDATING",
            RouteTableStateEnum::INACTIVE => "INACTIVE",
            RouteTableStateEnum::OBSOLETE => "OBSOLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for RouteTableStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(RouteTableStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(RouteTableStateEnum::CREATING),
           "ACTIVE" => Ok(RouteTableStateEnum::ACTIVE),
           "DELETING" => Ok(RouteTableStateEnum::DELETING),
           "ACCEPTING" => Ok(RouteTableStateEnum::ACCEPTING),
           "REJECTING" => Ok(RouteTableStateEnum::REJECTING),
           "UPDATING" => Ok(RouteTableStateEnum::UPDATING),
           "INACTIVE" => Ok(RouteTableStateEnum::INACTIVE),
           "OBSOLETE" => Ok(RouteTableStateEnum::OBSOLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RouteTableStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceConnectionMapInfrastructureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The infrastructure used for connections between consumers/producers.
pub enum ServiceConnectionMapInfrastructureEnum {
    

    /// An invalid infrastructure as the default case.
    ///
    /// "INFRASTRUCTURE_UNSPECIFIED"
    #[serde(rename="INFRASTRUCTURE_UNSPECIFIED")]
    INFRASTRUCTUREUNSPECIFIED,
    

    /// Private Service Connect is used for connections.
    ///
    /// "PSC"
    #[serde(rename="PSC")]
    PSC,
}

impl AsRef<str> for ServiceConnectionMapInfrastructureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceConnectionMapInfrastructureEnum::INFRASTRUCTUREUNSPECIFIED => "INFRASTRUCTURE_UNSPECIFIED",
            ServiceConnectionMapInfrastructureEnum::PSC => "PSC",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceConnectionMapInfrastructureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INFRASTRUCTURE_UNSPECIFIED" => Ok(ServiceConnectionMapInfrastructureEnum::INFRASTRUCTUREUNSPECIFIED),
           "PSC" => Ok(ServiceConnectionMapInfrastructureEnum::PSC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceConnectionMapInfrastructureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceConnectionPolicyInfrastructureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of underlying resources used to create the connection.
pub enum ServiceConnectionPolicyInfrastructureEnum {
    

    /// An invalid infrastructure as the default case.
    ///
    /// "INFRASTRUCTURE_UNSPECIFIED"
    #[serde(rename="INFRASTRUCTURE_UNSPECIFIED")]
    INFRASTRUCTUREUNSPECIFIED,
    

    /// Private Service Connect is used for connections.
    ///
    /// "PSC"
    #[serde(rename="PSC")]
    PSC,
}

impl AsRef<str> for ServiceConnectionPolicyInfrastructureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceConnectionPolicyInfrastructureEnum::INFRASTRUCTUREUNSPECIFIED => "INFRASTRUCTURE_UNSPECIFIED",
            ServiceConnectionPolicyInfrastructureEnum::PSC => "PSC",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceConnectionPolicyInfrastructureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INFRASTRUCTURE_UNSPECIFIED" => Ok(ServiceConnectionPolicyInfrastructureEnum::INFRASTRUCTUREUNSPECIFIED),
           "PSC" => Ok(ServiceConnectionPolicyInfrastructureEnum::PSC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceConnectionPolicyInfrastructureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpokeSpokeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of resource associated with the spoke.
pub enum SpokeSpokeTypeEnum {
    

    /// Unspecified spoke type.
    ///
    /// "SPOKE_TYPE_UNSPECIFIED"
    #[serde(rename="SPOKE_TYPE_UNSPECIFIED")]
    SPOKETYPEUNSPECIFIED,
    

    /// Spokes associated with VPN tunnels.
    ///
    /// "VPN_TUNNEL"
    #[serde(rename="VPN_TUNNEL")]
    VPNTUNNEL,
    

    /// Spokes associated with VLAN attachments.
    ///
    /// "INTERCONNECT_ATTACHMENT"
    #[serde(rename="INTERCONNECT_ATTACHMENT")]
    INTERCONNECTATTACHMENT,
    

    /// Spokes associated with router appliance instances.
    ///
    /// "ROUTER_APPLIANCE"
    #[serde(rename="ROUTER_APPLIANCE")]
    ROUTERAPPLIANCE,
    

    /// Spokes associated with VPC networks.
    ///
    /// "VPC_NETWORK"
    #[serde(rename="VPC_NETWORK")]
    VPCNETWORK,
}

impl AsRef<str> for SpokeSpokeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpokeSpokeTypeEnum::SPOKETYPEUNSPECIFIED => "SPOKE_TYPE_UNSPECIFIED",
            SpokeSpokeTypeEnum::VPNTUNNEL => "VPN_TUNNEL",
            SpokeSpokeTypeEnum::INTERCONNECTATTACHMENT => "INTERCONNECT_ATTACHMENT",
            SpokeSpokeTypeEnum::ROUTERAPPLIANCE => "ROUTER_APPLIANCE",
            SpokeSpokeTypeEnum::VPCNETWORK => "VPC_NETWORK",
        }
    }
}

impl std::convert::TryFrom< &str> for SpokeSpokeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPOKE_TYPE_UNSPECIFIED" => Ok(SpokeSpokeTypeEnum::SPOKETYPEUNSPECIFIED),
           "VPN_TUNNEL" => Ok(SpokeSpokeTypeEnum::VPNTUNNEL),
           "INTERCONNECT_ATTACHMENT" => Ok(SpokeSpokeTypeEnum::INTERCONNECTATTACHMENT),
           "ROUTER_APPLIANCE" => Ok(SpokeSpokeTypeEnum::ROUTERAPPLIANCE),
           "VPC_NETWORK" => Ok(SpokeSpokeTypeEnum::VPCNETWORK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpokeSpokeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpokeStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current lifecycle state of this spoke.
pub enum SpokeStateEnum {
    

    /// No state information available
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The resource's create operation is in progress.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The resource is active
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource's delete operation is in progress.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource's accept operation is in progress.
    ///
    /// "ACCEPTING"
    #[serde(rename="ACCEPTING")]
    ACCEPTING,
    

    /// The resource's reject operation is in progress.
    ///
    /// "REJECTING"
    #[serde(rename="REJECTING")]
    REJECTING,
    

    /// The resource's update operation is in progress.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The resource is inactive.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// The hub associated with this spoke resource has been deleted. This state applies to spoke resources only.
    ///
    /// "OBSOLETE"
    #[serde(rename="OBSOLETE")]
    OBSOLETE,
}

impl AsRef<str> for SpokeStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpokeStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SpokeStateEnum::CREATING => "CREATING",
            SpokeStateEnum::ACTIVE => "ACTIVE",
            SpokeStateEnum::DELETING => "DELETING",
            SpokeStateEnum::ACCEPTING => "ACCEPTING",
            SpokeStateEnum::REJECTING => "REJECTING",
            SpokeStateEnum::UPDATING => "UPDATING",
            SpokeStateEnum::INACTIVE => "INACTIVE",
            SpokeStateEnum::OBSOLETE => "OBSOLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for SpokeStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(SpokeStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(SpokeStateEnum::CREATING),
           "ACTIVE" => Ok(SpokeStateEnum::ACTIVE),
           "DELETING" => Ok(SpokeStateEnum::DELETING),
           "ACCEPTING" => Ok(SpokeStateEnum::ACCEPTING),
           "REJECTING" => Ok(SpokeStateEnum::REJECTING),
           "UPDATING" => Ok(SpokeStateEnum::UPDATING),
           "INACTIVE" => Ok(SpokeStateEnum::INACTIVE),
           "OBSOLETE" => Ok(SpokeStateEnum::OBSOLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpokeStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpokeStateCountStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the spokes.
pub enum SpokeStateCountStateEnum {
    

    /// No state information available
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The resource's create operation is in progress.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The resource is active
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource's delete operation is in progress.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource's accept operation is in progress.
    ///
    /// "ACCEPTING"
    #[serde(rename="ACCEPTING")]
    ACCEPTING,
    

    /// The resource's reject operation is in progress.
    ///
    /// "REJECTING"
    #[serde(rename="REJECTING")]
    REJECTING,
    

    /// The resource's update operation is in progress.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The resource is inactive.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// The hub associated with this spoke resource has been deleted. This state applies to spoke resources only.
    ///
    /// "OBSOLETE"
    #[serde(rename="OBSOLETE")]
    OBSOLETE,
}

impl AsRef<str> for SpokeStateCountStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpokeStateCountStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SpokeStateCountStateEnum::CREATING => "CREATING",
            SpokeStateCountStateEnum::ACTIVE => "ACTIVE",
            SpokeStateCountStateEnum::DELETING => "DELETING",
            SpokeStateCountStateEnum::ACCEPTING => "ACCEPTING",
            SpokeStateCountStateEnum::REJECTING => "REJECTING",
            SpokeStateCountStateEnum::UPDATING => "UPDATING",
            SpokeStateCountStateEnum::INACTIVE => "INACTIVE",
            SpokeStateCountStateEnum::OBSOLETE => "OBSOLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for SpokeStateCountStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(SpokeStateCountStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(SpokeStateCountStateEnum::CREATING),
           "ACTIVE" => Ok(SpokeStateCountStateEnum::ACTIVE),
           "DELETING" => Ok(SpokeStateCountStateEnum::DELETING),
           "ACCEPTING" => Ok(SpokeStateCountStateEnum::ACCEPTING),
           "REJECTING" => Ok(SpokeStateCountStateEnum::REJECTING),
           "UPDATING" => Ok(SpokeStateCountStateEnum::UPDATING),
           "INACTIVE" => Ok(SpokeStateCountStateEnum::INACTIVE),
           "OBSOLETE" => Ok(SpokeStateCountStateEnum::OBSOLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpokeStateCountStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpokeStateReasonCountStateReasonCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The reason that a spoke is inactive.
pub enum SpokeStateReasonCountStateReasonCodeEnum {
    

    /// No information available.
    ///
    /// "CODE_UNSPECIFIED"
    #[serde(rename="CODE_UNSPECIFIED")]
    CODEUNSPECIFIED,
    

    /// The proposed spoke is pending review.
    ///
    /// "PENDING_REVIEW"
    #[serde(rename="PENDING_REVIEW")]
    PENDINGREVIEW,
    

    /// The proposed spoke has been rejected by the hub administrator.
    ///
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// The spoke has been deactivated internally.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// Network Connectivity Center encountered errors while accepting the spoke.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for SpokeStateReasonCountStateReasonCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpokeStateReasonCountStateReasonCodeEnum::CODEUNSPECIFIED => "CODE_UNSPECIFIED",
            SpokeStateReasonCountStateReasonCodeEnum::PENDINGREVIEW => "PENDING_REVIEW",
            SpokeStateReasonCountStateReasonCodeEnum::REJECTED => "REJECTED",
            SpokeStateReasonCountStateReasonCodeEnum::PAUSED => "PAUSED",
            SpokeStateReasonCountStateReasonCodeEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for SpokeStateReasonCountStateReasonCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CODE_UNSPECIFIED" => Ok(SpokeStateReasonCountStateReasonCodeEnum::CODEUNSPECIFIED),
           "PENDING_REVIEW" => Ok(SpokeStateReasonCountStateReasonCodeEnum::PENDINGREVIEW),
           "REJECTED" => Ok(SpokeStateReasonCountStateReasonCodeEnum::REJECTED),
           "PAUSED" => Ok(SpokeStateReasonCountStateReasonCodeEnum::PAUSED),
           "FAILED" => Ok(SpokeStateReasonCountStateReasonCodeEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpokeStateReasonCountStateReasonCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpokeTypeCountSpokeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the spokes.
pub enum SpokeTypeCountSpokeTypeEnum {
    

    /// Unspecified spoke type.
    ///
    /// "SPOKE_TYPE_UNSPECIFIED"
    #[serde(rename="SPOKE_TYPE_UNSPECIFIED")]
    SPOKETYPEUNSPECIFIED,
    

    /// Spokes associated with VPN tunnels.
    ///
    /// "VPN_TUNNEL"
    #[serde(rename="VPN_TUNNEL")]
    VPNTUNNEL,
    

    /// Spokes associated with VLAN attachments.
    ///
    /// "INTERCONNECT_ATTACHMENT"
    #[serde(rename="INTERCONNECT_ATTACHMENT")]
    INTERCONNECTATTACHMENT,
    

    /// Spokes associated with router appliance instances.
    ///
    /// "ROUTER_APPLIANCE"
    #[serde(rename="ROUTER_APPLIANCE")]
    ROUTERAPPLIANCE,
    

    /// Spokes associated with VPC networks.
    ///
    /// "VPC_NETWORK"
    #[serde(rename="VPC_NETWORK")]
    VPCNETWORK,
}

impl AsRef<str> for SpokeTypeCountSpokeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpokeTypeCountSpokeTypeEnum::SPOKETYPEUNSPECIFIED => "SPOKE_TYPE_UNSPECIFIED",
            SpokeTypeCountSpokeTypeEnum::VPNTUNNEL => "VPN_TUNNEL",
            SpokeTypeCountSpokeTypeEnum::INTERCONNECTATTACHMENT => "INTERCONNECT_ATTACHMENT",
            SpokeTypeCountSpokeTypeEnum::ROUTERAPPLIANCE => "ROUTER_APPLIANCE",
            SpokeTypeCountSpokeTypeEnum::VPCNETWORK => "VPC_NETWORK",
        }
    }
}

impl std::convert::TryFrom< &str> for SpokeTypeCountSpokeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPOKE_TYPE_UNSPECIFIED" => Ok(SpokeTypeCountSpokeTypeEnum::SPOKETYPEUNSPECIFIED),
           "VPN_TUNNEL" => Ok(SpokeTypeCountSpokeTypeEnum::VPNTUNNEL),
           "INTERCONNECT_ATTACHMENT" => Ok(SpokeTypeCountSpokeTypeEnum::INTERCONNECTATTACHMENT),
           "ROUTER_APPLIANCE" => Ok(SpokeTypeCountSpokeTypeEnum::ROUTERAPPLIANCE),
           "VPC_NETWORK" => Ok(SpokeTypeCountSpokeTypeEnum::VPCNETWORK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpokeTypeCountSpokeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StateReasonCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The code associated with this reason.
pub enum StateReasonCodeEnum {
    

    /// No information available.
    ///
    /// "CODE_UNSPECIFIED"
    #[serde(rename="CODE_UNSPECIFIED")]
    CODEUNSPECIFIED,
    

    /// The proposed spoke is pending review.
    ///
    /// "PENDING_REVIEW"
    #[serde(rename="PENDING_REVIEW")]
    PENDINGREVIEW,
    

    /// The proposed spoke has been rejected by the hub administrator.
    ///
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// The spoke has been deactivated internally.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// Network Connectivity Center encountered errors while accepting the spoke.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for StateReasonCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StateReasonCodeEnum::CODEUNSPECIFIED => "CODE_UNSPECIFIED",
            StateReasonCodeEnum::PENDINGREVIEW => "PENDING_REVIEW",
            StateReasonCodeEnum::REJECTED => "REJECTED",
            StateReasonCodeEnum::PAUSED => "PAUSED",
            StateReasonCodeEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for StateReasonCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CODE_UNSPECIFIED" => Ok(StateReasonCodeEnum::CODEUNSPECIFIED),
           "PENDING_REVIEW" => Ok(StateReasonCodeEnum::PENDINGREVIEW),
           "REJECTED" => Ok(StateReasonCodeEnum::REJECTED),
           "PAUSED" => Ok(StateReasonCodeEnum::PAUSED),
           "FAILED" => Ok(StateReasonCodeEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StateReasonCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WarningCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. A warning code, if applicable.
pub enum WarningCodeEnum {
    

    /// Default value.
    ///
    /// "WARNING_UNSPECIFIED"
    #[serde(rename="WARNING_UNSPECIFIED")]
    WARNINGUNSPECIFIED,
    

    /// The policy-based route is not active and functioning. Common causes are the dependent network was deleted or the resource project was turned off.
    ///
    /// "RESOURCE_NOT_ACTIVE"
    #[serde(rename="RESOURCE_NOT_ACTIVE")]
    RESOURCENOTACTIVE,
    

    /// The policy-based route is being modified (e.g. created/deleted) at this time.
    ///
    /// "RESOURCE_BEING_MODIFIED"
    #[serde(rename="RESOURCE_BEING_MODIFIED")]
    RESOURCEBEINGMODIFIED,
}

impl AsRef<str> for WarningCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WarningCodeEnum::WARNINGUNSPECIFIED => "WARNING_UNSPECIFIED",
            WarningCodeEnum::RESOURCENOTACTIVE => "RESOURCE_NOT_ACTIVE",
            WarningCodeEnum::RESOURCEBEINGMODIFIED => "RESOURCE_BEING_MODIFIED",
        }
    }
}

impl std::convert::TryFrom< &str> for WarningCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WARNING_UNSPECIFIED" => Ok(WarningCodeEnum::WARNINGUNSPECIFIED),
           "RESOURCE_NOT_ACTIVE" => Ok(WarningCodeEnum::RESOURCENOTACTIVE),
           "RESOURCE_BEING_MODIFIED" => Ok(WarningCodeEnum::RESOURCEBEINGMODIFIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WarningCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The view of the spoke to return. The view that you use determines which spoke fields are included in the response.
pub enum ProjectViewEnum {
    

    /// The spoke view is unspecified. When the spoke view is unspecified, the API returns the same fields as the `BASIC` view.
    ///
    /// "SPOKE_VIEW_UNSPECIFIED"
    #[serde(rename="SPOKE_VIEW_UNSPECIFIED")]
    SPOKEVIEWUNSPECIFIED,
    

    /// Includes `name`, `create_time`, `hub`, `unique_id`, `state`, `reasons`, and `spoke_type`. This is the default value.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Includes all spoke fields except `labels`. You can use the `DETAILED` view only when you set the `spoke_locations` field to `[global]`.
    ///
    /// "DETAILED"
    #[serde(rename="DETAILED")]
    DETAILED,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::SPOKEVIEWUNSPECIFIED => "SPOKE_VIEW_UNSPECIFIED",
            ProjectViewEnum::BASIC => "BASIC",
            ProjectViewEnum::DETAILED => "DETAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPOKE_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::SPOKEVIEWUNSPECIFIED),
           "BASIC" => Ok(ProjectViewEnum::BASIC),
           "DETAILED" => Ok(ProjectViewEnum::DETAILED),
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


