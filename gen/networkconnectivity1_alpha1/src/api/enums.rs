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


// region HubStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current lifecycle state of this Hub.
pub enum HubStateEnum {
    

    /// No state information available
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The resource's create operation is in progress
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The resource is active
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource's Delete operation is in progress
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource's Update operation is in progress
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for HubStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HubStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            HubStateEnum::CREATING => "CREATING",
            HubStateEnum::ACTIVE => "ACTIVE",
            HubStateEnum::DELETING => "DELETING",
            HubStateEnum::UPDATING => "UPDATING",
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
           "UPDATING" => Ok(HubStateEnum::UPDATING),
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
    

    /// Allow creation of static routes more specific than the current internal range.
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
    

    /// This behavior can be set when the internal range is being reserved for usage by the peers. This means that no resource within the VPC in which it is being created can use this to associate with a VPC resource, but one of the peers can. This represents donating a range for peers to use.
    ///
    /// "FOR_PEER"
    #[serde(rename="FOR_PEER")]
    FORPEER,
    

    /// This behavior can be set when the internal range is being reserved for usage by the VPC in which it is created but not shared with the peers. In a sense it is local to the VPC. This can be used to create internal ranges for various purposes like HTTP_INTERNAL_LOAD_BALANCER or for Interconnect routes that are not shared with peers. This also implies that peers cannot use this range in a way that is visible to this VPC, but can re-use this range as long as it is NOT_SHARED from the peer VPC, too.
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
/// The type of usage set for this internal range.
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
    

    /// Ranges created with EXTERNAL_TO_VPC cannot be associated with VPC resources and are meant to block out address ranges for various use cases such as usage on-premises, with dynamic route announcements via Interconnect.
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


// region SpokeStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current lifecycle state of this Hub.
pub enum SpokeStateEnum {
    

    /// No state information available
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The resource's create operation is in progress
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The resource is active
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource's Delete operation is in progress
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The resource's Update operation is in progress
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for SpokeStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpokeStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SpokeStateEnum::CREATING => "CREATING",
            SpokeStateEnum::ACTIVE => "ACTIVE",
            SpokeStateEnum::DELETING => "DELETING",
            SpokeStateEnum::UPDATING => "UPDATING",
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
           "UPDATING" => Ok(SpokeStateEnum::UPDATING),
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


