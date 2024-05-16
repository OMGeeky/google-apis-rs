use super::*;



// region ChangeStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of the operation (output only). A status of "done" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet.
pub enum ChangeStatusEnum {
    
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
}

impl AsRef<str> for ChangeStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChangeStatusEnum::PENDING => "PENDING",
            ChangeStatusEnum::DONE => "DONE",
        }
    }
}

impl std::convert::TryFrom< &str> for ChangeStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PENDING" => Ok(ChangeStatusEnum::PENDING),
           "DONE" => Ok(ChangeStatusEnum::DONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChangeStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DnsKeyAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time.
pub enum DnsKeyAlgorithmEnum {
    
    /// "RSASHA1"
    #[serde(rename="RSASHA1")]
    RSASHA1,
    
    /// "RSASHA256"
    #[serde(rename="RSASHA256")]
    RSASHA256,
    
    /// "RSASHA512"
    #[serde(rename="RSASHA512")]
    RSASHA512,
    
    /// "ECDSAP256SHA256"
    #[serde(rename="ECDSAP256SHA256")]
    ECDSAP256SHA256,
    
    /// "ECDSAP384SHA384"
    #[serde(rename="ECDSAP384SHA384")]
    ECDSAP384SHA384,
}

impl AsRef<str> for DnsKeyAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsKeyAlgorithmEnum::RSASHA1 => "RSASHA1",
            DnsKeyAlgorithmEnum::RSASHA256 => "RSASHA256",
            DnsKeyAlgorithmEnum::RSASHA512 => "RSASHA512",
            DnsKeyAlgorithmEnum::ECDSAP256SHA256 => "ECDSAP256SHA256",
            DnsKeyAlgorithmEnum::ECDSAP384SHA384 => "ECDSAP384SHA384",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsKeyAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RSASHA1" => Ok(DnsKeyAlgorithmEnum::RSASHA1),
           "RSASHA256" => Ok(DnsKeyAlgorithmEnum::RSASHA256),
           "RSASHA512" => Ok(DnsKeyAlgorithmEnum::RSASHA512),
           "ECDSAP256SHA256" => Ok(DnsKeyAlgorithmEnum::ECDSAP256SHA256),
           "ECDSAP384SHA384" => Ok(DnsKeyAlgorithmEnum::ECDSAP384SHA384),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DnsKeyAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DnsKeyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// One of "KEY_SIGNING" or "ZONE_SIGNING". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, are used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag is cleared, and this key is used to sign only resource record sets of other types. Immutable after creation time.
pub enum DnsKeyTypeEnum {
    
    /// "KEY_SIGNING"
    #[serde(rename="KEY_SIGNING")]
    KEYSIGNING,
    
    /// "ZONE_SIGNING"
    #[serde(rename="ZONE_SIGNING")]
    ZONESIGNING,
}

impl AsRef<str> for DnsKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsKeyTypeEnum::KEYSIGNING => "KEY_SIGNING",
            DnsKeyTypeEnum::ZONESIGNING => "ZONE_SIGNING",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_SIGNING" => Ok(DnsKeyTypeEnum::KEYSIGNING),
           "ZONE_SIGNING" => Ok(DnsKeyTypeEnum::ZONESIGNING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DnsKeyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DnsKeyDigestTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the algorithm used to calculate this digest.
pub enum DnsKeyDigestTypeEnum {
    
    /// "SHA1"
    #[serde(rename="SHA1")]
    SHA1,
    
    /// "SHA256"
    #[serde(rename="SHA256")]
    SHA256,
    
    /// "SHA384"
    #[serde(rename="SHA384")]
    SHA384,
}

impl AsRef<str> for DnsKeyDigestTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsKeyDigestTypeEnum::SHA1 => "SHA1",
            DnsKeyDigestTypeEnum::SHA256 => "SHA256",
            DnsKeyDigestTypeEnum::SHA384 => "SHA384",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsKeyDigestTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SHA1" => Ok(DnsKeyDigestTypeEnum::SHA1),
           "SHA256" => Ok(DnsKeyDigestTypeEnum::SHA256),
           "SHA384" => Ok(DnsKeyDigestTypeEnum::SHA384),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DnsKeyDigestTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DnsKeySpecAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// String mnemonic specifying the DNSSEC algorithm of this key.
pub enum DnsKeySpecAlgorithmEnum {
    
    /// "RSASHA1"
    #[serde(rename="RSASHA1")]
    RSASHA1,
    
    /// "RSASHA256"
    #[serde(rename="RSASHA256")]
    RSASHA256,
    
    /// "RSASHA512"
    #[serde(rename="RSASHA512")]
    RSASHA512,
    
    /// "ECDSAP256SHA256"
    #[serde(rename="ECDSAP256SHA256")]
    ECDSAP256SHA256,
    
    /// "ECDSAP384SHA384"
    #[serde(rename="ECDSAP384SHA384")]
    ECDSAP384SHA384,
}

impl AsRef<str> for DnsKeySpecAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsKeySpecAlgorithmEnum::RSASHA1 => "RSASHA1",
            DnsKeySpecAlgorithmEnum::RSASHA256 => "RSASHA256",
            DnsKeySpecAlgorithmEnum::RSASHA512 => "RSASHA512",
            DnsKeySpecAlgorithmEnum::ECDSAP256SHA256 => "ECDSAP256SHA256",
            DnsKeySpecAlgorithmEnum::ECDSAP384SHA384 => "ECDSAP384SHA384",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsKeySpecAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RSASHA1" => Ok(DnsKeySpecAlgorithmEnum::RSASHA1),
           "RSASHA256" => Ok(DnsKeySpecAlgorithmEnum::RSASHA256),
           "RSASHA512" => Ok(DnsKeySpecAlgorithmEnum::RSASHA512),
           "ECDSAP256SHA256" => Ok(DnsKeySpecAlgorithmEnum::ECDSAP256SHA256),
           "ECDSAP384SHA384" => Ok(DnsKeySpecAlgorithmEnum::ECDSAP384SHA384),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DnsKeySpecAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DnsKeySpecKeyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether this is a key signing key (KSK) or a zone signing key (ZSK). Key signing keys have the Secure Entry Point flag set and, when active, are only used to sign resource record sets of type DNSKEY. Zone signing keys do not have the Secure Entry Point flag set and are used to sign all other types of resource record sets.
pub enum DnsKeySpecKeyTypeEnum {
    
    /// "KEY_SIGNING"
    #[serde(rename="KEY_SIGNING")]
    KEYSIGNING,
    
    /// "ZONE_SIGNING"
    #[serde(rename="ZONE_SIGNING")]
    ZONESIGNING,
}

impl AsRef<str> for DnsKeySpecKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsKeySpecKeyTypeEnum::KEYSIGNING => "KEY_SIGNING",
            DnsKeySpecKeyTypeEnum::ZONESIGNING => "ZONE_SIGNING",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsKeySpecKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_SIGNING" => Ok(DnsKeySpecKeyTypeEnum::KEYSIGNING),
           "ZONE_SIGNING" => Ok(DnsKeySpecKeyTypeEnum::ZONESIGNING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DnsKeySpecKeyTypeEnum {
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


// region ManagedZoneVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources.
pub enum ManagedZoneVisibilityEnum {
    
    /// "PUBLIC"
    #[serde(rename="PUBLIC")]
    PUBLIC,
    
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
}

impl AsRef<str> for ManagedZoneVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedZoneVisibilityEnum::PUBLIC => "PUBLIC",
            ManagedZoneVisibilityEnum::PRIVATE => "PRIVATE",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedZoneVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PUBLIC" => Ok(ManagedZoneVisibilityEnum::PUBLIC),
           "PRIVATE" => Ok(ManagedZoneVisibilityEnum::PRIVATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagedZoneVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagedZoneDnsSecConfigNonExistenceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the mechanism for authenticated denial-of-existence responses. Can only be changed while the state is OFF.
pub enum ManagedZoneDnsSecConfigNonExistenceEnum {
    
    /// "NSEC"
    #[serde(rename="NSEC")]
    NSEC,
    
    /// "NSEC3"
    #[serde(rename="NSEC3")]
    NSEC3,
}

impl AsRef<str> for ManagedZoneDnsSecConfigNonExistenceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedZoneDnsSecConfigNonExistenceEnum::NSEC => "NSEC",
            ManagedZoneDnsSecConfigNonExistenceEnum::NSEC3 => "NSEC3",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedZoneDnsSecConfigNonExistenceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NSEC" => Ok(ManagedZoneDnsSecConfigNonExistenceEnum::NSEC),
           "NSEC3" => Ok(ManagedZoneDnsSecConfigNonExistenceEnum::NSEC3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagedZoneDnsSecConfigNonExistenceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagedZoneDnsSecConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether DNSSEC is enabled, and what mode it is in.
pub enum ManagedZoneDnsSecConfigStateEnum {
    

    /// DNSSEC is disabled; the zone is not signed.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
    

    /// DNSSEC is enabled; the zone is signed and fully managed.
    ///
    /// "ON"
    #[serde(rename="ON")]
    ON,
    

    /// DNSSEC is enabled, but in a "transfer" mode.
    ///
    /// "TRANSFER"
    #[serde(rename="TRANSFER")]
    TRANSFER,
}

impl AsRef<str> for ManagedZoneDnsSecConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedZoneDnsSecConfigStateEnum::OFF => "OFF",
            ManagedZoneDnsSecConfigStateEnum::ON => "ON",
            ManagedZoneDnsSecConfigStateEnum::TRANSFER => "TRANSFER",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedZoneDnsSecConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFF" => Ok(ManagedZoneDnsSecConfigStateEnum::OFF),
           "ON" => Ok(ManagedZoneDnsSecConfigStateEnum::ON),
           "TRANSFER" => Ok(ManagedZoneDnsSecConfigStateEnum::TRANSFER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagedZoneDnsSecConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Forwarding path for this NameServerTarget. If unset or set to DEFAULT, Cloud DNS makes forwarding decisions based on IP address ranges; that is, RFC1918 addresses go to the VPC network, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through the VPC network for this target.
pub enum ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum {
    

    /// Cloud DNS makes forwarding decisions based on address ranges; that is, RFC1918 addresses forward to the target through the VPC and non-RFC1918 addresses forward to the target through the internet
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// Cloud DNS always forwards to this target through the VPC.
    ///
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
}

impl AsRef<str> for ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum::DEFAULT => "DEFAULT",
            ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum::PRIVATE => "PRIVATE",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum::DEFAULT),
           "PRIVATE" => Ok(ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum::PRIVATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OperationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of the operation. Can be one of the following: "PENDING" or "DONE" (output only). A status of "DONE" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet.
pub enum OperationStatusEnum {
    
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
}

impl AsRef<str> for OperationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationStatusEnum::PENDING => "PENDING",
            OperationStatusEnum::DONE => "DONE",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PENDING" => Ok(OperationStatusEnum::PENDING),
           "DONE" => Ok(OperationStatusEnum::DONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OperationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Forwarding path for this TargetNameServer. If unset or set to DEFAULT, Cloud DNS makes forwarding decisions based on address ranges; that is, RFC1918 addresses go to the VPC network, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through the VPC network for this target.
pub enum PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum {
    

    /// Cloud DNS makes forwarding decision based on IP address ranges; that is, RFC1918 addresses forward to the target through the VPC and non-RFC1918 addresses forward to the target through the internet
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// Cloud DNS always forwards to this target through the VPC.
    ///
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
}

impl AsRef<str> for PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum::DEFAULT => "DEFAULT",
            PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum::PRIVATE => "PRIVATE",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum::DEFAULT),
           "PRIVATE" => Ok(PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum::PRIVATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The protocol of the load balancer to health check.
pub enum RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum {
    
    /// "UNDEFINED"
    #[serde(rename="UNDEFINED")]
    UNDEFINED,
    
    /// "TCP"
    #[serde(rename="TCP")]
    TCP,
    
    /// "UDP"
    #[serde(rename="UDP")]
    UDP,
}

impl AsRef<str> for RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::UNDEFINED => "UNDEFINED",
            RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::TCP => "TCP",
            RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::UDP => "UDP",
        }
    }
}

impl std::convert::TryFrom< &str> for RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNDEFINED" => Ok(RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::UNDEFINED),
           "TCP" => Ok(RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::TCP),
           "UDP" => Ok(RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::UDP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of load balancer specified by this target. This value must match the configuration of the load balancer located at the LoadBalancerTarget's IP address, port, and region. Use the following: - *regionalL4ilb*: for a regional internal passthrough Network Load Balancer. - *regionalL7ilb*: for a regional internal Application Load Balancer. - *globalL7ilb*: for a global internal Application Load Balancer. 
pub enum RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum {
    
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    
    /// "GLOBAL_L7ILB"
    #[serde(rename="GLOBAL_L7ILB")]
    GLOBALL7ILB,
    
    /// "REGIONAL_L4ILB"
    #[serde(rename="REGIONAL_L4ILB")]
    REGIONALL4ILB,
    
    /// "REGIONAL_L7ILB"
    #[serde(rename="REGIONAL_L7ILB")]
    REGIONALL7ILB,
}

impl AsRef<str> for RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::NONE => "NONE",
            RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::GLOBALL7ILB => "GLOBAL_L7ILB",
            RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::REGIONALL4ILB => "REGIONAL_L4ILB",
            RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::REGIONALL7ILB => "REGIONAL_L7ILB",
        }
    }
}

impl std::convert::TryFrom< &str> for RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::NONE),
           "GLOBAL_L7ILB" => Ok(RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::GLOBALL7ILB),
           "REGIONAL_L4ILB" => Ok(RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::REGIONALL4ILB),
           "REGIONAL_L7ILB" => Ok(RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::REGIONALL7ILB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResponsePolicyRuleBehaviorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Answer this query with a behavior rather than DNS data.
pub enum ResponsePolicyRuleBehaviorEnum {
    
    /// "BEHAVIOR_UNSPECIFIED"
    #[serde(rename="BEHAVIOR_UNSPECIFIED")]
    BEHAVIORUNSPECIFIED,
    

    /// Skip a less-specific ResponsePolicyRule and continue normal query logic. This can be used with a less-specific wildcard selector to exempt a subset of the wildcard ResponsePolicyRule from the ResponsePolicy behavior and query the public Internet instead. For instance, if these rules exist: *.example.com -> LocalData 1.2.3.4 foo.example.com -> Behavior 'bypassResponsePolicy' Then a query for 'foo.example.com' skips the wildcard. This additionally functions to facilitate the allowlist feature. RPZs can be applied to multiple levels in the (eventually org, folder, project, network) hierarchy. If a rule is applied at a higher level of the hierarchy, adding a passthru rule at a lower level will supersede that, and a query from an affected vm to that domain will be exempt from the RPZ and proceed to normal resolution behavior.
    ///
    /// "BYPASS_RESPONSE_POLICY"
    #[serde(rename="BYPASS_RESPONSE_POLICY")]
    BYPASSRESPONSEPOLICY,
}

impl AsRef<str> for ResponsePolicyRuleBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResponsePolicyRuleBehaviorEnum::BEHAVIORUNSPECIFIED => "BEHAVIOR_UNSPECIFIED",
            ResponsePolicyRuleBehaviorEnum::BYPASSRESPONSEPOLICY => "BYPASS_RESPONSE_POLICY",
        }
    }
}

impl std::convert::TryFrom< &str> for ResponsePolicyRuleBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BEHAVIOR_UNSPECIFIED" => Ok(ResponsePolicyRuleBehaviorEnum::BEHAVIORUNSPECIFIED),
           "BYPASS_RESPONSE_POLICY" => Ok(ResponsePolicyRuleBehaviorEnum::BYPASSRESPONSEPOLICY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResponsePolicyRuleBehaviorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChangeSortByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sorting criterion. The only supported value is change sequence.
pub enum ChangeSortByEnum {
    
    /// "CHANGE_SEQUENCE"
    #[serde(rename="CHANGE_SEQUENCE")]
    CHANGESEQUENCE,
}

impl AsRef<str> for ChangeSortByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChangeSortByEnum::CHANGESEQUENCE => "CHANGE_SEQUENCE",
        }
    }
}

impl std::convert::TryFrom< &str> for ChangeSortByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHANGE_SEQUENCE" => Ok(ChangeSortByEnum::CHANGESEQUENCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChangeSortByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ChangeSortByEnum {
    fn default() -> ChangeSortByEnum {
        ChangeSortByEnum::CHANGESEQUENCE
    }
}

// endregion


// region ManagedZoneOperationSortByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sorting criterion. The only supported values are START_TIME and ID.
pub enum ManagedZoneOperationSortByEnum {
    
    /// "START_TIME"
    #[serde(rename="START_TIME")]
    STARTTIME,
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
}

impl AsRef<str> for ManagedZoneOperationSortByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedZoneOperationSortByEnum::STARTTIME => "START_TIME",
            ManagedZoneOperationSortByEnum::ID => "ID",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedZoneOperationSortByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "START_TIME" => Ok(ManagedZoneOperationSortByEnum::STARTTIME),
           "ID" => Ok(ManagedZoneOperationSortByEnum::ID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagedZoneOperationSortByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ManagedZoneOperationSortByEnum {
    fn default() -> ManagedZoneOperationSortByEnum {
        ManagedZoneOperationSortByEnum::STARTTIME
    }
}

// endregion


