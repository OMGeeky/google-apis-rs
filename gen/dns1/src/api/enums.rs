use super::*;



// region ChangeStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of the operation (output only). A status of "done" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet.
pub enum ChangeStatusEnum {
    
    /// "pending"
    #[serde(rename="pending")]
    Pending,
    
    /// "done"
    #[serde(rename="done")]
    Done,
}

impl AsRef<str> for ChangeStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChangeStatusEnum::Pending => "pending",
            ChangeStatusEnum::Done => "done",
        }
    }
}

impl std::convert::TryFrom< &str> for ChangeStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "pending" => Ok(ChangeStatusEnum::Pending),
           "done" => Ok(ChangeStatusEnum::Done),
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
    
    /// "rsasha1"
    #[serde(rename="rsasha1")]
    Rsasha1,
    
    /// "rsasha256"
    #[serde(rename="rsasha256")]
    Rsasha256,
    
    /// "rsasha512"
    #[serde(rename="rsasha512")]
    Rsasha512,
    
    /// "ecdsap256sha256"
    #[serde(rename="ecdsap256sha256")]
    Ecdsap256sha256,
    
    /// "ecdsap384sha384"
    #[serde(rename="ecdsap384sha384")]
    Ecdsap384sha384,
}

impl AsRef<str> for DnsKeyAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsKeyAlgorithmEnum::Rsasha1 => "rsasha1",
            DnsKeyAlgorithmEnum::Rsasha256 => "rsasha256",
            DnsKeyAlgorithmEnum::Rsasha512 => "rsasha512",
            DnsKeyAlgorithmEnum::Ecdsap256sha256 => "ecdsap256sha256",
            DnsKeyAlgorithmEnum::Ecdsap384sha384 => "ecdsap384sha384",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsKeyAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "rsasha1" => Ok(DnsKeyAlgorithmEnum::Rsasha1),
           "rsasha256" => Ok(DnsKeyAlgorithmEnum::Rsasha256),
           "rsasha512" => Ok(DnsKeyAlgorithmEnum::Rsasha512),
           "ecdsap256sha256" => Ok(DnsKeyAlgorithmEnum::Ecdsap256sha256),
           "ecdsap384sha384" => Ok(DnsKeyAlgorithmEnum::Ecdsap384sha384),
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
    
    /// "keySigning"
    #[serde(rename="keySigning")]
    KeySigning,
    
    /// "zoneSigning"
    #[serde(rename="zoneSigning")]
    ZoneSigning,
}

impl AsRef<str> for DnsKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsKeyTypeEnum::KeySigning => "keySigning",
            DnsKeyTypeEnum::ZoneSigning => "zoneSigning",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "keySigning" => Ok(DnsKeyTypeEnum::KeySigning),
           "zoneSigning" => Ok(DnsKeyTypeEnum::ZoneSigning),
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
    
    /// "sha1"
    #[serde(rename="sha1")]
    Sha1,
    
    /// "sha256"
    #[serde(rename="sha256")]
    Sha256,
    
    /// "sha384"
    #[serde(rename="sha384")]
    Sha384,
}

impl AsRef<str> for DnsKeyDigestTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsKeyDigestTypeEnum::Sha1 => "sha1",
            DnsKeyDigestTypeEnum::Sha256 => "sha256",
            DnsKeyDigestTypeEnum::Sha384 => "sha384",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsKeyDigestTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "sha1" => Ok(DnsKeyDigestTypeEnum::Sha1),
           "sha256" => Ok(DnsKeyDigestTypeEnum::Sha256),
           "sha384" => Ok(DnsKeyDigestTypeEnum::Sha384),
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
    
    /// "rsasha1"
    #[serde(rename="rsasha1")]
    Rsasha1,
    
    /// "rsasha256"
    #[serde(rename="rsasha256")]
    Rsasha256,
    
    /// "rsasha512"
    #[serde(rename="rsasha512")]
    Rsasha512,
    
    /// "ecdsap256sha256"
    #[serde(rename="ecdsap256sha256")]
    Ecdsap256sha256,
    
    /// "ecdsap384sha384"
    #[serde(rename="ecdsap384sha384")]
    Ecdsap384sha384,
}

impl AsRef<str> for DnsKeySpecAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsKeySpecAlgorithmEnum::Rsasha1 => "rsasha1",
            DnsKeySpecAlgorithmEnum::Rsasha256 => "rsasha256",
            DnsKeySpecAlgorithmEnum::Rsasha512 => "rsasha512",
            DnsKeySpecAlgorithmEnum::Ecdsap256sha256 => "ecdsap256sha256",
            DnsKeySpecAlgorithmEnum::Ecdsap384sha384 => "ecdsap384sha384",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsKeySpecAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "rsasha1" => Ok(DnsKeySpecAlgorithmEnum::Rsasha1),
           "rsasha256" => Ok(DnsKeySpecAlgorithmEnum::Rsasha256),
           "rsasha512" => Ok(DnsKeySpecAlgorithmEnum::Rsasha512),
           "ecdsap256sha256" => Ok(DnsKeySpecAlgorithmEnum::Ecdsap256sha256),
           "ecdsap384sha384" => Ok(DnsKeySpecAlgorithmEnum::Ecdsap384sha384),
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
    
    /// "keySigning"
    #[serde(rename="keySigning")]
    KeySigning,
    
    /// "zoneSigning"
    #[serde(rename="zoneSigning")]
    ZoneSigning,
}

impl AsRef<str> for DnsKeySpecKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsKeySpecKeyTypeEnum::KeySigning => "keySigning",
            DnsKeySpecKeyTypeEnum::ZoneSigning => "zoneSigning",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsKeySpecKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "keySigning" => Ok(DnsKeySpecKeyTypeEnum::KeySigning),
           "zoneSigning" => Ok(DnsKeySpecKeyTypeEnum::ZoneSigning),
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
    
    /// "public"
    #[serde(rename="public")]
    Public,
    
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for ManagedZoneVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedZoneVisibilityEnum::Public => "public",
            ManagedZoneVisibilityEnum::Private => "private",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedZoneVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "public" => Ok(ManagedZoneVisibilityEnum::Public),
           "private" => Ok(ManagedZoneVisibilityEnum::Private),
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
    
    /// "nsec"
    #[serde(rename="nsec")]
    Nsec,
    
    /// "nsec3"
    #[serde(rename="nsec3")]
    Nsec3,
}

impl AsRef<str> for ManagedZoneDnsSecConfigNonExistenceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedZoneDnsSecConfigNonExistenceEnum::Nsec => "nsec",
            ManagedZoneDnsSecConfigNonExistenceEnum::Nsec3 => "nsec3",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedZoneDnsSecConfigNonExistenceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "nsec" => Ok(ManagedZoneDnsSecConfigNonExistenceEnum::Nsec),
           "nsec3" => Ok(ManagedZoneDnsSecConfigNonExistenceEnum::Nsec3),
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
    /// "off"
    #[serde(rename="off")]
    Off,
    

    /// DNSSEC is enabled; the zone is signed and fully managed.
    ///
    /// "on"
    #[serde(rename="on")]
    On,
    

    /// DNSSEC is enabled, but in a "transfer" mode.
    ///
    /// "transfer"
    #[serde(rename="transfer")]
    Transfer,
}

impl AsRef<str> for ManagedZoneDnsSecConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedZoneDnsSecConfigStateEnum::Off => "off",
            ManagedZoneDnsSecConfigStateEnum::On => "on",
            ManagedZoneDnsSecConfigStateEnum::Transfer => "transfer",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedZoneDnsSecConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "off" => Ok(ManagedZoneDnsSecConfigStateEnum::Off),
           "on" => Ok(ManagedZoneDnsSecConfigStateEnum::On),
           "transfer" => Ok(ManagedZoneDnsSecConfigStateEnum::Transfer),
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
    /// "default"
    #[serde(rename="default")]
    Default,
    

    /// Cloud DNS always forwards to this target through the VPC.
    ///
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum::Default => "default",
            ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum::Private => "private",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "default" => Ok(ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum::Default),
           "private" => Ok(ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum::Private),
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
    
    /// "pending"
    #[serde(rename="pending")]
    Pending,
    
    /// "done"
    #[serde(rename="done")]
    Done,
}

impl AsRef<str> for OperationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationStatusEnum::Pending => "pending",
            OperationStatusEnum::Done => "done",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "pending" => Ok(OperationStatusEnum::Pending),
           "done" => Ok(OperationStatusEnum::Done),
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
    /// "default"
    #[serde(rename="default")]
    Default,
    

    /// Cloud DNS always forwards to this target through the VPC.
    ///
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum::Default => "default",
            PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum::Private => "private",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "default" => Ok(PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum::Default),
           "private" => Ok(PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum::Private),
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
    
    /// "undefined"
    #[serde(rename="undefined")]
    Undefined,
    
    /// "tcp"
    #[serde(rename="tcp")]
    Tcp,
    
    /// "udp"
    #[serde(rename="udp")]
    Udp,
}

impl AsRef<str> for RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::Undefined => "undefined",
            RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::Tcp => "tcp",
            RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::Udp => "udp",
        }
    }
}

impl std::convert::TryFrom< &str> for RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "undefined" => Ok(RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::Undefined),
           "tcp" => Ok(RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::Tcp),
           "udp" => Ok(RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum::Udp),
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
    
    /// "none"
    #[serde(rename="none")]
    None,
    
    /// "globalL7ilb"
    #[serde(rename="globalL7ilb")]
    GlobalL7ilb,
    
    /// "regionalL4ilb"
    #[serde(rename="regionalL4ilb")]
    RegionalL4ilb,
    
    /// "regionalL7ilb"
    #[serde(rename="regionalL7ilb")]
    RegionalL7ilb,
}

impl AsRef<str> for RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::None => "none",
            RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::GlobalL7ilb => "globalL7ilb",
            RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::RegionalL4ilb => "regionalL4ilb",
            RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::RegionalL7ilb => "regionalL7ilb",
        }
    }
}

impl std::convert::TryFrom< &str> for RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "none" => Ok(RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::None),
           "globalL7ilb" => Ok(RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::GlobalL7ilb),
           "regionalL4ilb" => Ok(RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::RegionalL4ilb),
           "regionalL7ilb" => Ok(RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum::RegionalL7ilb),
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
    
    /// "behaviorUnspecified"
    #[serde(rename="behaviorUnspecified")]
    BehaviorUnspecified,
    

    /// Skip a less-specific ResponsePolicyRule and continue normal query logic. This can be used with a less-specific wildcard selector to exempt a subset of the wildcard ResponsePolicyRule from the ResponsePolicy behavior and query the public Internet instead. For instance, if these rules exist: *.example.com -> LocalData 1.2.3.4 foo.example.com -> Behavior 'bypassResponsePolicy' Then a query for 'foo.example.com' skips the wildcard. This additionally functions to facilitate the allowlist feature. RPZs can be applied to multiple levels in the (eventually org, folder, project, network) hierarchy. If a rule is applied at a higher level of the hierarchy, adding a passthru rule at a lower level will supersede that, and a query from an affected vm to that domain will be exempt from the RPZ and proceed to normal resolution behavior.
    ///
    /// "bypassResponsePolicy"
    #[serde(rename="bypassResponsePolicy")]
    BypassResponsePolicy,
}

impl AsRef<str> for ResponsePolicyRuleBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResponsePolicyRuleBehaviorEnum::BehaviorUnspecified => "behaviorUnspecified",
            ResponsePolicyRuleBehaviorEnum::BypassResponsePolicy => "bypassResponsePolicy",
        }
    }
}

impl std::convert::TryFrom< &str> for ResponsePolicyRuleBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "behaviorUnspecified" => Ok(ResponsePolicyRuleBehaviorEnum::BehaviorUnspecified),
           "bypassResponsePolicy" => Ok(ResponsePolicyRuleBehaviorEnum::BypassResponsePolicy),
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
    
    /// "changeSequence"
    #[serde(rename="changeSequence")]
    ChangeSequence,
}

impl AsRef<str> for ChangeSortByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChangeSortByEnum::ChangeSequence => "changeSequence",
        }
    }
}

impl std::convert::TryFrom< &str> for ChangeSortByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "changeSequence" => Ok(ChangeSortByEnum::ChangeSequence),
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
        ChangeSortByEnum::ChangeSequence
    }
}

// endregion


// region ManagedZoneOperationSortByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sorting criterion. The only supported values are START_TIME and ID.
pub enum ManagedZoneOperationSortByEnum {
    
    /// "startTime"
    #[serde(rename="startTime")]
    StartTime,
    
    /// "id"
    #[serde(rename="id")]
    Id,
}

impl AsRef<str> for ManagedZoneOperationSortByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedZoneOperationSortByEnum::StartTime => "startTime",
            ManagedZoneOperationSortByEnum::Id => "id",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedZoneOperationSortByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "startTime" => Ok(ManagedZoneOperationSortByEnum::StartTime),
           "id" => Ok(ManagedZoneOperationSortByEnum::Id),
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
        ManagedZoneOperationSortByEnum::StartTime
    }
}

// endregion


