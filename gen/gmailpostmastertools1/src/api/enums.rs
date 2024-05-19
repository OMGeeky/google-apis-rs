use super::*;



// region DeliveryErrorErrorClassEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The class of delivery error.
pub enum DeliveryErrorErrorClassEnum {
    

    /// The default value which should never be used explicitly.
    ///
    /// "DELIVERY_ERROR_CLASS_UNSPECIFIED"
    #[serde(rename="DELIVERY_ERROR_CLASS_UNSPECIFIED")]
    DELIVERYERRORCLASSUNSPECIFIED,
    

    /// Delivery of message has been rejected.
    ///
    /// "PERMANENT_ERROR"
    #[serde(rename="PERMANENT_ERROR")]
    PERMANENTERROR,
    

    /// Temporary failure of message delivery to the recipient.
    ///
    /// "TEMPORARY_ERROR"
    #[serde(rename="TEMPORARY_ERROR")]
    TEMPORARYERROR,
}

impl AsRef<str> for DeliveryErrorErrorClassEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeliveryErrorErrorClassEnum::DELIVERYERRORCLASSUNSPECIFIED => "DELIVERY_ERROR_CLASS_UNSPECIFIED",
            DeliveryErrorErrorClassEnum::PERMANENTERROR => "PERMANENT_ERROR",
            DeliveryErrorErrorClassEnum::TEMPORARYERROR => "TEMPORARY_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for DeliveryErrorErrorClassEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DELIVERY_ERROR_CLASS_UNSPECIFIED" => Ok(DeliveryErrorErrorClassEnum::DELIVERYERRORCLASSUNSPECIFIED),
           "PERMANENT_ERROR" => Ok(DeliveryErrorErrorClassEnum::PERMANENTERROR),
           "TEMPORARY_ERROR" => Ok(DeliveryErrorErrorClassEnum::TEMPORARYERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeliveryErrorErrorClassEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeliveryErrorErrorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of delivery error.
pub enum DeliveryErrorErrorTypeEnum {
    

    /// The default value which should never be used explicitly.
    ///
    /// "DELIVERY_ERROR_TYPE_UNSPECIFIED"
    #[serde(rename="DELIVERY_ERROR_TYPE_UNSPECIFIED")]
    DELIVERYERRORTYPEUNSPECIFIED,
    

    /// The Domain or IP is sending traffic at a suspiciously high rate, due to which temporary rate limits have been imposed. The limit will be lifted when Gmail is confident enough of the nature of the traffic.
    ///
    /// "RATE_LIMIT_EXCEEDED"
    #[serde(rename="RATE_LIMIT_EXCEEDED")]
    RATELIMITEXCEEDED,
    

    /// The traffic is suspected to be spam, by Gmail, for various reasons.
    ///
    /// "SUSPECTED_SPAM"
    #[serde(rename="SUSPECTED_SPAM")]
    SUSPECTEDSPAM,
    

    /// The traffic is suspected to be spammy, specific to the content.
    ///
    /// "CONTENT_SPAMMY"
    #[serde(rename="CONTENT_SPAMMY")]
    CONTENTSPAMMY,
    

    /// Traffic contains attachments not supported by Gmail.
    ///
    /// "BAD_ATTACHMENT"
    #[serde(rename="BAD_ATTACHMENT")]
    BADATTACHMENT,
    

    /// The sender domain has set up a DMARC rejection policy.
    ///
    /// "BAD_DMARC_POLICY"
    #[serde(rename="BAD_DMARC_POLICY")]
    BADDMARCPOLICY,
    

    /// The IP reputation of the sending IP is very low.
    ///
    /// "LOW_IP_REPUTATION"
    #[serde(rename="LOW_IP_REPUTATION")]
    LOWIPREPUTATION,
    

    /// The Domain reputation of the sending domain is very low.
    ///
    /// "LOW_DOMAIN_REPUTATION"
    #[serde(rename="LOW_DOMAIN_REPUTATION")]
    LOWDOMAINREPUTATION,
    

    /// The IP is listed in one or more public [Real-time Blackhole Lists](http://en.wikipedia.org/wiki/DNSBL). Work with the RBL to get your IP delisted.
    ///
    /// "IP_IN_RBL"
    #[serde(rename="IP_IN_RBL")]
    IPINRBL,
    

    /// The Domain is listed in one or more public [Real-time Blackhole Lists](http://en.wikipedia.org/wiki/DNSBL). Work with the RBL to get your domain delisted.
    ///
    /// "DOMAIN_IN_RBL"
    #[serde(rename="DOMAIN_IN_RBL")]
    DOMAININRBL,
    

    /// The sending IP is missing a [PTR record](https://support.google.com/domains/answer/3251147#ptr).
    ///
    /// "BAD_PTR_RECORD"
    #[serde(rename="BAD_PTR_RECORD")]
    BADPTRRECORD,
}

impl AsRef<str> for DeliveryErrorErrorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeliveryErrorErrorTypeEnum::DELIVERYERRORTYPEUNSPECIFIED => "DELIVERY_ERROR_TYPE_UNSPECIFIED",
            DeliveryErrorErrorTypeEnum::RATELIMITEXCEEDED => "RATE_LIMIT_EXCEEDED",
            DeliveryErrorErrorTypeEnum::SUSPECTEDSPAM => "SUSPECTED_SPAM",
            DeliveryErrorErrorTypeEnum::CONTENTSPAMMY => "CONTENT_SPAMMY",
            DeliveryErrorErrorTypeEnum::BADATTACHMENT => "BAD_ATTACHMENT",
            DeliveryErrorErrorTypeEnum::BADDMARCPOLICY => "BAD_DMARC_POLICY",
            DeliveryErrorErrorTypeEnum::LOWIPREPUTATION => "LOW_IP_REPUTATION",
            DeliveryErrorErrorTypeEnum::LOWDOMAINREPUTATION => "LOW_DOMAIN_REPUTATION",
            DeliveryErrorErrorTypeEnum::IPINRBL => "IP_IN_RBL",
            DeliveryErrorErrorTypeEnum::DOMAININRBL => "DOMAIN_IN_RBL",
            DeliveryErrorErrorTypeEnum::BADPTRRECORD => "BAD_PTR_RECORD",
        }
    }
}

impl std::convert::TryFrom< &str> for DeliveryErrorErrorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DELIVERY_ERROR_TYPE_UNSPECIFIED" => Ok(DeliveryErrorErrorTypeEnum::DELIVERYERRORTYPEUNSPECIFIED),
           "RATE_LIMIT_EXCEEDED" => Ok(DeliveryErrorErrorTypeEnum::RATELIMITEXCEEDED),
           "SUSPECTED_SPAM" => Ok(DeliveryErrorErrorTypeEnum::SUSPECTEDSPAM),
           "CONTENT_SPAMMY" => Ok(DeliveryErrorErrorTypeEnum::CONTENTSPAMMY),
           "BAD_ATTACHMENT" => Ok(DeliveryErrorErrorTypeEnum::BADATTACHMENT),
           "BAD_DMARC_POLICY" => Ok(DeliveryErrorErrorTypeEnum::BADDMARCPOLICY),
           "LOW_IP_REPUTATION" => Ok(DeliveryErrorErrorTypeEnum::LOWIPREPUTATION),
           "LOW_DOMAIN_REPUTATION" => Ok(DeliveryErrorErrorTypeEnum::LOWDOMAINREPUTATION),
           "IP_IN_RBL" => Ok(DeliveryErrorErrorTypeEnum::IPINRBL),
           "DOMAIN_IN_RBL" => Ok(DeliveryErrorErrorTypeEnum::DOMAININRBL),
           "BAD_PTR_RECORD" => Ok(DeliveryErrorErrorTypeEnum::BADPTRRECORD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeliveryErrorErrorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DomainPermissionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// User’s permission for this domain. Assigned by the server.
pub enum DomainPermissionEnum {
    

    /// The default value and should never be used explicitly.
    ///
    /// "PERMISSION_UNSPECIFIED"
    #[serde(rename="PERMISSION_UNSPECIFIED")]
    PERMISSIONUNSPECIFIED,
    

    /// User has read access to the domain and can share access with others.
    ///
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
    

    /// User has read access to the domain.
    ///
    /// "READER"
    #[serde(rename="READER")]
    READER,
    

    /// User doesn't have permission to access information about the domain. User did not verify ownership of domain nor was access granted by other domain owners.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
}

impl AsRef<str> for DomainPermissionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DomainPermissionEnum::PERMISSIONUNSPECIFIED => "PERMISSION_UNSPECIFIED",
            DomainPermissionEnum::OWNER => "OWNER",
            DomainPermissionEnum::READER => "READER",
            DomainPermissionEnum::NONE => "NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for DomainPermissionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERMISSION_UNSPECIFIED" => Ok(DomainPermissionEnum::PERMISSIONUNSPECIFIED),
           "OWNER" => Ok(DomainPermissionEnum::OWNER),
           "READER" => Ok(DomainPermissionEnum::READER),
           "NONE" => Ok(DomainPermissionEnum::NONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DomainPermissionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IpReputationReputationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reputation category this IP reputation represents.
pub enum IpReputationReputationEnum {
    

    /// The default value which should never be used explicitly. This represents the state where no reputation information is available.
    ///
    /// "REPUTATION_CATEGORY_UNSPECIFIED"
    #[serde(rename="REPUTATION_CATEGORY_UNSPECIFIED")]
    REPUTATIONCATEGORYUNSPECIFIED,
    

    /// Has a good track record of a very low spam rate, and complies with Gmail's sender guidelines. Mail will rarely be marked by the spam filter.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Known to send good mail, but is prone to sending a low volume of spam intermittently. Most of the email from this entity will have a fair deliverability rate, except when there is a notable increase in spam levels.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Known to send a considerable volume of spam regularly, and mail from this sender will likely be marked as spam.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// History of sending an enormously high volume of spam. Mail coming from this entity will almost always be rejected at SMTP level or marked as spam.
    ///
    /// "BAD"
    #[serde(rename="BAD")]
    BAD,
}

impl AsRef<str> for IpReputationReputationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IpReputationReputationEnum::REPUTATIONCATEGORYUNSPECIFIED => "REPUTATION_CATEGORY_UNSPECIFIED",
            IpReputationReputationEnum::HIGH => "HIGH",
            IpReputationReputationEnum::MEDIUM => "MEDIUM",
            IpReputationReputationEnum::LOW => "LOW",
            IpReputationReputationEnum::BAD => "BAD",
        }
    }
}

impl std::convert::TryFrom< &str> for IpReputationReputationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPUTATION_CATEGORY_UNSPECIFIED" => Ok(IpReputationReputationEnum::REPUTATIONCATEGORYUNSPECIFIED),
           "HIGH" => Ok(IpReputationReputationEnum::HIGH),
           "MEDIUM" => Ok(IpReputationReputationEnum::MEDIUM),
           "LOW" => Ok(IpReputationReputationEnum::LOW),
           "BAD" => Ok(IpReputationReputationEnum::BAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IpReputationReputationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TrafficStatDomainReputationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Reputation of the domain.
pub enum TrafficStatDomainReputationEnum {
    

    /// The default value which should never be used explicitly. This represents the state where no reputation information is available.
    ///
    /// "REPUTATION_CATEGORY_UNSPECIFIED"
    #[serde(rename="REPUTATION_CATEGORY_UNSPECIFIED")]
    REPUTATIONCATEGORYUNSPECIFIED,
    

    /// Has a good track record of a very low spam rate, and complies with Gmail's sender guidelines. Mail will rarely be marked by the spam filter.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Known to send good mail, but is prone to sending a low volume of spam intermittently. Most of the email from this entity will have a fair deliverability rate, except when there is a notable increase in spam levels.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Known to send a considerable volume of spam regularly, and mail from this sender will likely be marked as spam.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// History of sending an enormously high volume of spam. Mail coming from this entity will almost always be rejected at SMTP level or marked as spam.
    ///
    /// "BAD"
    #[serde(rename="BAD")]
    BAD,
}

impl AsRef<str> for TrafficStatDomainReputationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TrafficStatDomainReputationEnum::REPUTATIONCATEGORYUNSPECIFIED => "REPUTATION_CATEGORY_UNSPECIFIED",
            TrafficStatDomainReputationEnum::HIGH => "HIGH",
            TrafficStatDomainReputationEnum::MEDIUM => "MEDIUM",
            TrafficStatDomainReputationEnum::LOW => "LOW",
            TrafficStatDomainReputationEnum::BAD => "BAD",
        }
    }
}

impl std::convert::TryFrom< &str> for TrafficStatDomainReputationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPUTATION_CATEGORY_UNSPECIFIED" => Ok(TrafficStatDomainReputationEnum::REPUTATIONCATEGORYUNSPECIFIED),
           "HIGH" => Ok(TrafficStatDomainReputationEnum::HIGH),
           "MEDIUM" => Ok(TrafficStatDomainReputationEnum::MEDIUM),
           "LOW" => Ok(TrafficStatDomainReputationEnum::LOW),
           "BAD" => Ok(TrafficStatDomainReputationEnum::BAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TrafficStatDomainReputationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


