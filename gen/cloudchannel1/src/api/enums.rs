use super::*;



// region GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. State of the channel partner link.
pub enum GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum {
    

    /// Not used.
    ///
    /// "CHANNEL_PARTNER_LINK_STATE_UNSPECIFIED"
    #[serde(rename="CHANNEL_PARTNER_LINK_STATE_UNSPECIFIED")]
    CHANNELPARTNERLINKSTATEUNSPECIFIED,
    

    /// An invitation has been sent to the reseller to create a channel partner link.
    ///
    /// "INVITED"
    #[serde(rename="INVITED")]
    INVITED,
    

    /// Status when the reseller is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Status when the reseller has been revoked by the distributor.
    ///
    /// "REVOKED"
    #[serde(rename="REVOKED")]
    REVOKED,
    

    /// Status when the reseller is suspended by Google or distributor.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
}

impl AsRef<str> for GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum::CHANNELPARTNERLINKSTATEUNSPECIFIED => "CHANNEL_PARTNER_LINK_STATE_UNSPECIFIED",
            GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum::INVITED => "INVITED",
            GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum::REVOKED => "REVOKED",
            GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum::SUSPENDED => "SUSPENDED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHANNEL_PARTNER_LINK_STATE_UNSPECIFIED" => Ok(GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum::CHANNELPARTNERLINKSTATEUNSPECIFIED),
           "INVITED" => Ok(GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum::INVITED),
           "ACTIVE" => Ok(GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum::ACTIVE),
           "REVOKED" => Ok(GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum::REVOKED),
           "SUSPENDED" => Ok(GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum::SUSPENDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// CustomerType indicates verification type needed for using services.
pub enum GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum {
    

    /// Not used.
    ///
    /// "CUSTOMER_TYPE_UNSPECIFIED"
    #[serde(rename="CUSTOMER_TYPE_UNSPECIFIED")]
    CUSTOMERTYPEUNSPECIFIED,
    

    /// Domain-owning customer which needs domain verification to use services.
    ///
    /// "DOMAIN"
    #[serde(rename="DOMAIN")]
    DOMAIN,
    

    /// Team customer which needs email verification to use services.
    ///
    /// "TEAM"
    #[serde(rename="TEAM")]
    TEAM,
}

impl AsRef<str> for GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum::CUSTOMERTYPEUNSPECIFIED => "CUSTOMER_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum::DOMAIN => "DOMAIN",
            GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum::TEAM => "TEAM",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CUSTOMER_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum::CUSTOMERTYPEUNSPECIFIED),
           "DOMAIN" => Ok(GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum::DOMAIN),
           "TEAM" => Ok(GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum::TEAM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1ColumnDataTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the values for this column.
pub enum GoogleCloudChannelV1ColumnDataTypeEnum {
    

    /// Not used.
    ///
    /// "DATA_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_TYPE_UNSPECIFIED")]
    DATATYPEUNSPECIFIED,
    

    /// ReportValues for this column will use string_value.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// ReportValues for this column will use int_value.
    ///
    /// "INT"
    #[serde(rename="INT")]
    INT,
    

    /// ReportValues for this column will use decimal_value.
    ///
    /// "DECIMAL"
    #[serde(rename="DECIMAL")]
    DECIMAL,
    

    /// ReportValues for this column will use money_value.
    ///
    /// "MONEY"
    #[serde(rename="MONEY")]
    MONEY,
    

    /// ReportValues for this column will use date_value.
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// ReportValues for this column will use date_time_value.
    ///
    /// "DATE_TIME"
    #[serde(rename="DATE_TIME")]
    DATETIME,
}

impl AsRef<str> for GoogleCloudChannelV1ColumnDataTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1ColumnDataTypeEnum::DATATYPEUNSPECIFIED => "DATA_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1ColumnDataTypeEnum::STRING => "STRING",
            GoogleCloudChannelV1ColumnDataTypeEnum::INT => "INT",
            GoogleCloudChannelV1ColumnDataTypeEnum::DECIMAL => "DECIMAL",
            GoogleCloudChannelV1ColumnDataTypeEnum::MONEY => "MONEY",
            GoogleCloudChannelV1ColumnDataTypeEnum::DATE => "DATE",
            GoogleCloudChannelV1ColumnDataTypeEnum::DATETIME => "DATE_TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1ColumnDataTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1ColumnDataTypeEnum::DATATYPEUNSPECIFIED),
           "STRING" => Ok(GoogleCloudChannelV1ColumnDataTypeEnum::STRING),
           "INT" => Ok(GoogleCloudChannelV1ColumnDataTypeEnum::INT),
           "DECIMAL" => Ok(GoogleCloudChannelV1ColumnDataTypeEnum::DECIMAL),
           "MONEY" => Ok(GoogleCloudChannelV1ColumnDataTypeEnum::MONEY),
           "DATE" => Ok(GoogleCloudChannelV1ColumnDataTypeEnum::DATE),
           "DATE_TIME" => Ok(GoogleCloudChannelV1ColumnDataTypeEnum::DATETIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1ColumnDataTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The RebillingBasis to use for the applied override. Shows the relative cost based on your repricing costs.
pub enum GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum {
    

    /// Not used.
    ///
    /// "REBILLING_BASIS_UNSPECIFIED"
    #[serde(rename="REBILLING_BASIS_UNSPECIFIED")]
    REBILLINGBASISUNSPECIFIED,
    

    /// Use the list cost, also known as the MSRP.
    ///
    /// "COST_AT_LIST"
    #[serde(rename="COST_AT_LIST")]
    COSTATLIST,
    

    /// Pass through all discounts except the Reseller Program Discount. If this is the default cost base and no adjustments are specified, the output cost will be exactly what the customer would see if they viewed the bill in the Google Cloud Console.
    ///
    /// "DIRECT_CUSTOMER_COST"
    #[serde(rename="DIRECT_CUSTOMER_COST")]
    DIRECTCUSTOMERCOST,
}

impl AsRef<str> for GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum::REBILLINGBASISUNSPECIFIED => "REBILLING_BASIS_UNSPECIFIED",
            GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum::COSTATLIST => "COST_AT_LIST",
            GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum::DIRECTCUSTOMERCOST => "DIRECT_CUSTOMER_COST",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REBILLING_BASIS_UNSPECIFIED" => Ok(GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum::REBILLINGBASISUNSPECIFIED),
           "COST_AT_LIST" => Ok(GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum::COSTATLIST),
           "DIRECT_CUSTOMER_COST" => Ok(GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum::DIRECTCUSTOMERCOST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allowed Customer Type.
pub enum GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum {
    

    /// Not used.
    ///
    /// "CUSTOMER_TYPE_UNSPECIFIED"
    #[serde(rename="CUSTOMER_TYPE_UNSPECIFIED")]
    CUSTOMERTYPEUNSPECIFIED,
    

    /// Domain-owning customer which needs domain verification to use services.
    ///
    /// "DOMAIN"
    #[serde(rename="DOMAIN")]
    DOMAIN,
    

    /// Team customer which needs email verification to use services.
    ///
    /// "TEAM"
    #[serde(rename="TEAM")]
    TEAM,
}

impl AsRef<str> for GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum::CUSTOMERTYPEUNSPECIFIED => "CUSTOMER_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum::DOMAIN => "DOMAIN",
            GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum::TEAM => "TEAM",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CUSTOMER_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum::CUSTOMERTYPEUNSPECIFIED),
           "DOMAIN" => Ok(GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum::DOMAIN),
           "TEAM" => Ok(GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum::TEAM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allowed Promotional Order Type. Present for Promotional offers.
pub enum GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum {
    

    /// Not used.
    ///
    /// "PROMOTIONAL_TYPE_UNSPECIFIED"
    #[serde(rename="PROMOTIONAL_TYPE_UNSPECIFIED")]
    PROMOTIONALTYPEUNSPECIFIED,
    

    /// Order used for new customers, trial conversions and upgrades.
    ///
    /// "NEW_UPGRADE"
    #[serde(rename="NEW_UPGRADE")]
    NEWUPGRADE,
    

    /// All orders for transferring an existing customer.
    ///
    /// "TRANSFER"
    #[serde(rename="TRANSFER")]
    TRANSFER,
    

    /// Orders for modifying an existing customer's promotion on the same SKU.
    ///
    /// "PROMOTION_SWITCH"
    #[serde(rename="PROMOTION_SWITCH")]
    PROMOTIONSWITCH,
}

impl AsRef<str> for GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum::PROMOTIONALTYPEUNSPECIFIED => "PROMOTIONAL_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum::NEWUPGRADE => "NEW_UPGRADE",
            GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum::TRANSFER => "TRANSFER",
            GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum::PROMOTIONSWITCH => "PROMOTION_SWITCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROMOTIONAL_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum::PROMOTIONALTYPEUNSPECIFIED),
           "NEW_UPGRADE" => Ok(GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum::NEWUPGRADE),
           "TRANSFER" => Ok(GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum::TRANSFER),
           "PROMOTION_SWITCH" => Ok(GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum::PROMOTIONSWITCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1EduDataInstituteSizeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Size of the institute.
pub enum GoogleCloudChannelV1EduDataInstituteSizeEnum {
    

    /// Not used.
    ///
    /// "INSTITUTE_SIZE_UNSPECIFIED"
    #[serde(rename="INSTITUTE_SIZE_UNSPECIFIED")]
    INSTITUTESIZEUNSPECIFIED,
    

    /// 1 - 100
    ///
    /// "SIZE_1_100"
    #[serde(rename="SIZE_1_100")]
    SIZE1100,
    

    /// 101 - 500
    ///
    /// "SIZE_101_500"
    #[serde(rename="SIZE_101_500")]
    SIZE101500,
    

    /// 501 - 1,000
    ///
    /// "SIZE_501_1000"
    #[serde(rename="SIZE_501_1000")]
    SIZE5011000,
    

    /// 1,001 - 2,000
    ///
    /// "SIZE_1001_2000"
    #[serde(rename="SIZE_1001_2000")]
    SIZE10012000,
    

    /// 2,001 - 5,000
    ///
    /// "SIZE_2001_5000"
    #[serde(rename="SIZE_2001_5000")]
    SIZE20015000,
    

    /// 5,001 - 10,000
    ///
    /// "SIZE_5001_10000"
    #[serde(rename="SIZE_5001_10000")]
    SIZE500110000,
    

    /// 10,001 +
    ///
    /// "SIZE_10001_OR_MORE"
    #[serde(rename="SIZE_10001_OR_MORE")]
    SIZE10001ORMORE,
}

impl AsRef<str> for GoogleCloudChannelV1EduDataInstituteSizeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1EduDataInstituteSizeEnum::INSTITUTESIZEUNSPECIFIED => "INSTITUTE_SIZE_UNSPECIFIED",
            GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE1100 => "SIZE_1_100",
            GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE101500 => "SIZE_101_500",
            GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE5011000 => "SIZE_501_1000",
            GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE10012000 => "SIZE_1001_2000",
            GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE20015000 => "SIZE_2001_5000",
            GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE500110000 => "SIZE_5001_10000",
            GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE10001ORMORE => "SIZE_10001_OR_MORE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1EduDataInstituteSizeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSTITUTE_SIZE_UNSPECIFIED" => Ok(GoogleCloudChannelV1EduDataInstituteSizeEnum::INSTITUTESIZEUNSPECIFIED),
           "SIZE_1_100" => Ok(GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE1100),
           "SIZE_101_500" => Ok(GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE101500),
           "SIZE_501_1000" => Ok(GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE5011000),
           "SIZE_1001_2000" => Ok(GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE10012000),
           "SIZE_2001_5000" => Ok(GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE20015000),
           "SIZE_5001_10000" => Ok(GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE500110000),
           "SIZE_10001_OR_MORE" => Ok(GoogleCloudChannelV1EduDataInstituteSizeEnum::SIZE10001ORMORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1EduDataInstituteSizeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1EduDataInstituteTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Designated institute type of customer.
pub enum GoogleCloudChannelV1EduDataInstituteTypeEnum {
    

    /// Not used.
    ///
    /// "INSTITUTE_TYPE_UNSPECIFIED"
    #[serde(rename="INSTITUTE_TYPE_UNSPECIFIED")]
    INSTITUTETYPEUNSPECIFIED,
    

    /// Elementary/Secondary Schools & Districts
    ///
    /// "K12"
    #[serde(rename="K12")]
    K12,
    

    /// Higher Education Universities & Colleges
    ///
    /// "UNIVERSITY"
    #[serde(rename="UNIVERSITY")]
    UNIVERSITY,
}

impl AsRef<str> for GoogleCloudChannelV1EduDataInstituteTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1EduDataInstituteTypeEnum::INSTITUTETYPEUNSPECIFIED => "INSTITUTE_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1EduDataInstituteTypeEnum::K12 => "K12",
            GoogleCloudChannelV1EduDataInstituteTypeEnum::UNIVERSITY => "UNIVERSITY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1EduDataInstituteTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSTITUTE_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1EduDataInstituteTypeEnum::INSTITUTETYPEUNSPECIFIED),
           "K12" => Ok(GoogleCloudChannelV1EduDataInstituteTypeEnum::K12),
           "UNIVERSITY" => Ok(GoogleCloudChannelV1EduDataInstituteTypeEnum::UNIVERSITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1EduDataInstituteTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1EntitlementProvisioningStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current provisioning state of the entitlement.
pub enum GoogleCloudChannelV1EntitlementProvisioningStateEnum {
    

    /// Not used.
    ///
    /// "PROVISIONING_STATE_UNSPECIFIED"
    #[serde(rename="PROVISIONING_STATE_UNSPECIFIED")]
    PROVISIONINGSTATEUNSPECIFIED,
    

    /// The entitlement is currently active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The entitlement is currently suspended.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
}

impl AsRef<str> for GoogleCloudChannelV1EntitlementProvisioningStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1EntitlementProvisioningStateEnum::PROVISIONINGSTATEUNSPECIFIED => "PROVISIONING_STATE_UNSPECIFIED",
            GoogleCloudChannelV1EntitlementProvisioningStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudChannelV1EntitlementProvisioningStateEnum::SUSPENDED => "SUSPENDED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1EntitlementProvisioningStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROVISIONING_STATE_UNSPECIFIED" => Ok(GoogleCloudChannelV1EntitlementProvisioningStateEnum::PROVISIONINGSTATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudChannelV1EntitlementProvisioningStateEnum::ACTIVE),
           "SUSPENDED" => Ok(GoogleCloudChannelV1EntitlementProvisioningStateEnum::SUSPENDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1EntitlementProvisioningStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1EntitlementSuspensionReasonsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Enumerable of all current suspension reasons for an entitlement.
pub enum GoogleCloudChannelV1EntitlementSuspensionReasonsEnum {
    

    /// Not used.
    ///
    /// "SUSPENSION_REASON_UNSPECIFIED"
    #[serde(rename="SUSPENSION_REASON_UNSPECIFIED")]
    SUSPENSIONREASONUNSPECIFIED,
    

    /// Entitlement was manually suspended by the Reseller.
    ///
    /// "RESELLER_INITIATED"
    #[serde(rename="RESELLER_INITIATED")]
    RESELLERINITIATED,
    

    /// Trial ended.
    ///
    /// "TRIAL_ENDED"
    #[serde(rename="TRIAL_ENDED")]
    TRIALENDED,
    

    /// Entitlement renewal was canceled.
    ///
    /// "RENEWAL_WITH_TYPE_CANCEL"
    #[serde(rename="RENEWAL_WITH_TYPE_CANCEL")]
    RENEWALWITHTYPECANCEL,
    

    /// Entitlement was automatically suspended on creation for pending ToS acceptance on customer.
    ///
    /// "PENDING_TOS_ACCEPTANCE"
    #[serde(rename="PENDING_TOS_ACCEPTANCE")]
    PENDINGTOSACCEPTANCE,
    

    /// Other reasons (internal reasons, abuse, etc.).
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for GoogleCloudChannelV1EntitlementSuspensionReasonsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::SUSPENSIONREASONUNSPECIFIED => "SUSPENSION_REASON_UNSPECIFIED",
            GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::RESELLERINITIATED => "RESELLER_INITIATED",
            GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::TRIALENDED => "TRIAL_ENDED",
            GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::RENEWALWITHTYPECANCEL => "RENEWAL_WITH_TYPE_CANCEL",
            GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::PENDINGTOSACCEPTANCE => "PENDING_TOS_ACCEPTANCE",
            GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1EntitlementSuspensionReasonsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUSPENSION_REASON_UNSPECIFIED" => Ok(GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::SUSPENSIONREASONUNSPECIFIED),
           "RESELLER_INITIATED" => Ok(GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::RESELLERINITIATED),
           "TRIAL_ENDED" => Ok(GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::TRIALENDED),
           "RENEWAL_WITH_TYPE_CANCEL" => Ok(GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::RENEWALWITHTYPECANCEL),
           "PENDING_TOS_ACCEPTANCE" => Ok(GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::PENDINGTOSACCEPTANCE),
           "OTHER" => Ok(GoogleCloudChannelV1EntitlementSuspensionReasonsEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1EntitlementSuspensionReasonsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1EntitlementChangeActivationReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Entitlement's activation reason
pub enum GoogleCloudChannelV1EntitlementChangeActivationReasonEnum {
    

    /// Not used.
    ///
    /// "ACTIVATION_REASON_UNSPECIFIED"
    #[serde(rename="ACTIVATION_REASON_UNSPECIFIED")]
    ACTIVATIONREASONUNSPECIFIED,
    

    /// Reseller reactivated a suspended Entitlement.
    ///
    /// "RESELLER_REVOKED_SUSPENSION"
    #[serde(rename="RESELLER_REVOKED_SUSPENSION")]
    RESELLERREVOKEDSUSPENSION,
    

    /// Customer accepted pending terms of service.
    ///
    /// "CUSTOMER_ACCEPTED_PENDING_TOS"
    #[serde(rename="CUSTOMER_ACCEPTED_PENDING_TOS")]
    CUSTOMERACCEPTEDPENDINGTOS,
    

    /// Reseller updated the renewal settings on an entitlement that was suspended due to cancellation, and this update reactivated the entitlement.
    ///
    /// "RENEWAL_SETTINGS_CHANGED"
    #[serde(rename="RENEWAL_SETTINGS_CHANGED")]
    RENEWALSETTINGSCHANGED,
    

    /// Other reasons (Activated temporarily for cancellation, added a payment plan to a trial entitlement, etc.)
    ///
    /// "OTHER_ACTIVATION_REASON"
    #[serde(rename="OTHER_ACTIVATION_REASON")]
    OTHERACTIVATIONREASON,
}

impl AsRef<str> for GoogleCloudChannelV1EntitlementChangeActivationReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1EntitlementChangeActivationReasonEnum::ACTIVATIONREASONUNSPECIFIED => "ACTIVATION_REASON_UNSPECIFIED",
            GoogleCloudChannelV1EntitlementChangeActivationReasonEnum::RESELLERREVOKEDSUSPENSION => "RESELLER_REVOKED_SUSPENSION",
            GoogleCloudChannelV1EntitlementChangeActivationReasonEnum::CUSTOMERACCEPTEDPENDINGTOS => "CUSTOMER_ACCEPTED_PENDING_TOS",
            GoogleCloudChannelV1EntitlementChangeActivationReasonEnum::RENEWALSETTINGSCHANGED => "RENEWAL_SETTINGS_CHANGED",
            GoogleCloudChannelV1EntitlementChangeActivationReasonEnum::OTHERACTIVATIONREASON => "OTHER_ACTIVATION_REASON",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1EntitlementChangeActivationReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTIVATION_REASON_UNSPECIFIED" => Ok(GoogleCloudChannelV1EntitlementChangeActivationReasonEnum::ACTIVATIONREASONUNSPECIFIED),
           "RESELLER_REVOKED_SUSPENSION" => Ok(GoogleCloudChannelV1EntitlementChangeActivationReasonEnum::RESELLERREVOKEDSUSPENSION),
           "CUSTOMER_ACCEPTED_PENDING_TOS" => Ok(GoogleCloudChannelV1EntitlementChangeActivationReasonEnum::CUSTOMERACCEPTEDPENDINGTOS),
           "RENEWAL_SETTINGS_CHANGED" => Ok(GoogleCloudChannelV1EntitlementChangeActivationReasonEnum::RENEWALSETTINGSCHANGED),
           "OTHER_ACTIVATION_REASON" => Ok(GoogleCloudChannelV1EntitlementChangeActivationReasonEnum::OTHERACTIVATIONREASON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1EntitlementChangeActivationReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Cancellation reason for the Entitlement.
pub enum GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum {
    

    /// Not used.
    ///
    /// "CANCELLATION_REASON_UNSPECIFIED"
    #[serde(rename="CANCELLATION_REASON_UNSPECIFIED")]
    CANCELLATIONREASONUNSPECIFIED,
    

    /// Reseller triggered a cancellation of the service.
    ///
    /// "SERVICE_TERMINATED"
    #[serde(rename="SERVICE_TERMINATED")]
    SERVICETERMINATED,
    

    /// Relationship between the reseller and customer has ended due to a transfer.
    ///
    /// "RELATIONSHIP_ENDED"
    #[serde(rename="RELATIONSHIP_ENDED")]
    RELATIONSHIPENDED,
    

    /// Entitlement transferred away from reseller while still keeping other entitlement(s) with the reseller.
    ///
    /// "PARTIAL_TRANSFER"
    #[serde(rename="PARTIAL_TRANSFER")]
    PARTIALTRANSFER,
}

impl AsRef<str> for GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum::CANCELLATIONREASONUNSPECIFIED => "CANCELLATION_REASON_UNSPECIFIED",
            GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum::SERVICETERMINATED => "SERVICE_TERMINATED",
            GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum::RELATIONSHIPENDED => "RELATIONSHIP_ENDED",
            GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum::PARTIALTRANSFER => "PARTIAL_TRANSFER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CANCELLATION_REASON_UNSPECIFIED" => Ok(GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum::CANCELLATIONREASONUNSPECIFIED),
           "SERVICE_TERMINATED" => Ok(GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum::SERVICETERMINATED),
           "RELATIONSHIP_ENDED" => Ok(GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum::RELATIONSHIPENDED),
           "PARTIAL_TRANSFER" => Ok(GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum::PARTIALTRANSFER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1EntitlementChangeCancellationReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1EntitlementChangeChangeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The change action type.
pub enum GoogleCloudChannelV1EntitlementChangeChangeTypeEnum {
    

    /// Not used.
    ///
    /// "CHANGE_TYPE_UNSPECIFIED"
    #[serde(rename="CHANGE_TYPE_UNSPECIFIED")]
    CHANGETYPEUNSPECIFIED,
    

    /// New Entitlement was created.
    ///
    /// "CREATED"
    #[serde(rename="CREATED")]
    CREATED,
    

    /// Price plan associated with an Entitlement was changed.
    ///
    /// "PRICE_PLAN_SWITCHED"
    #[serde(rename="PRICE_PLAN_SWITCHED")]
    PRICEPLANSWITCHED,
    

    /// Number of seats committed for a commitment Entitlement was changed.
    ///
    /// "COMMITMENT_CHANGED"
    #[serde(rename="COMMITMENT_CHANGED")]
    COMMITMENTCHANGED,
    

    /// An annual Entitlement was renewed.
    ///
    /// "RENEWED"
    #[serde(rename="RENEWED")]
    RENEWED,
    

    /// Entitlement was suspended.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
    

    /// Entitlement was activated.
    ///
    /// "ACTIVATED"
    #[serde(rename="ACTIVATED")]
    ACTIVATED,
    

    /// Entitlement was cancelled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// Entitlement was upgraded or downgraded for ex. from Google Workspace Business Standard to Google Workspace Business Plus.
    ///
    /// "SKU_CHANGED"
    #[serde(rename="SKU_CHANGED")]
    SKUCHANGED,
    

    /// The settings for renewal of an Entitlement have changed.
    ///
    /// "RENEWAL_SETTING_CHANGED"
    #[serde(rename="RENEWAL_SETTING_CHANGED")]
    RENEWALSETTINGCHANGED,
    

    /// Use for Google Workspace subscription. Either a trial was converted to a paid subscription or a new subscription with no trial is created.
    ///
    /// "PAID_SUBSCRIPTION_STARTED"
    #[serde(rename="PAID_SUBSCRIPTION_STARTED")]
    PAIDSUBSCRIPTIONSTARTED,
    

    /// License cap was changed for the entitlement.
    ///
    /// "LICENSE_CAP_CHANGED"
    #[serde(rename="LICENSE_CAP_CHANGED")]
    LICENSECAPCHANGED,
    

    /// The suspension details have changed (but it is still suspended).
    ///
    /// "SUSPENSION_DETAILS_CHANGED"
    #[serde(rename="SUSPENSION_DETAILS_CHANGED")]
    SUSPENSIONDETAILSCHANGED,
    

    /// The trial end date was extended.
    ///
    /// "TRIAL_END_DATE_EXTENDED"
    #[serde(rename="TRIAL_END_DATE_EXTENDED")]
    TRIALENDDATEEXTENDED,
    

    /// Entitlement started trial.
    ///
    /// "TRIAL_STARTED"
    #[serde(rename="TRIAL_STARTED")]
    TRIALSTARTED,
}

impl AsRef<str> for GoogleCloudChannelV1EntitlementChangeChangeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::CHANGETYPEUNSPECIFIED => "CHANGE_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::CREATED => "CREATED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::PRICEPLANSWITCHED => "PRICE_PLAN_SWITCHED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::COMMITMENTCHANGED => "COMMITMENT_CHANGED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::RENEWED => "RENEWED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::SUSPENDED => "SUSPENDED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::ACTIVATED => "ACTIVATED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::CANCELLED => "CANCELLED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::SKUCHANGED => "SKU_CHANGED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::RENEWALSETTINGCHANGED => "RENEWAL_SETTING_CHANGED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::PAIDSUBSCRIPTIONSTARTED => "PAID_SUBSCRIPTION_STARTED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::LICENSECAPCHANGED => "LICENSE_CAP_CHANGED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::SUSPENSIONDETAILSCHANGED => "SUSPENSION_DETAILS_CHANGED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::TRIALENDDATEEXTENDED => "TRIAL_END_DATE_EXTENDED",
            GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::TRIALSTARTED => "TRIAL_STARTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1EntitlementChangeChangeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHANGE_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::CHANGETYPEUNSPECIFIED),
           "CREATED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::CREATED),
           "PRICE_PLAN_SWITCHED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::PRICEPLANSWITCHED),
           "COMMITMENT_CHANGED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::COMMITMENTCHANGED),
           "RENEWED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::RENEWED),
           "SUSPENDED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::SUSPENDED),
           "ACTIVATED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::ACTIVATED),
           "CANCELLED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::CANCELLED),
           "SKU_CHANGED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::SKUCHANGED),
           "RENEWAL_SETTING_CHANGED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::RENEWALSETTINGCHANGED),
           "PAID_SUBSCRIPTION_STARTED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::PAIDSUBSCRIPTIONSTARTED),
           "LICENSE_CAP_CHANGED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::LICENSECAPCHANGED),
           "SUSPENSION_DETAILS_CHANGED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::SUSPENSIONDETAILSCHANGED),
           "TRIAL_END_DATE_EXTENDED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::TRIALENDDATEEXTENDED),
           "TRIAL_STARTED" => Ok(GoogleCloudChannelV1EntitlementChangeChangeTypeEnum::TRIALSTARTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1EntitlementChangeChangeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Operator type responsible for the change.
pub enum GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum {
    

    /// Not used.
    ///
    /// "OPERATOR_TYPE_UNSPECIFIED"
    #[serde(rename="OPERATOR_TYPE_UNSPECIFIED")]
    OPERATORTYPEUNSPECIFIED,
    

    /// Customer service representative.
    ///
    /// "CUSTOMER_SERVICE_REPRESENTATIVE"
    #[serde(rename="CUSTOMER_SERVICE_REPRESENTATIVE")]
    CUSTOMERSERVICEREPRESENTATIVE,
    

    /// System auto job.
    ///
    /// "SYSTEM"
    #[serde(rename="SYSTEM")]
    SYSTEM,
    

    /// Customer user.
    ///
    /// "CUSTOMER"
    #[serde(rename="CUSTOMER")]
    CUSTOMER,
    

    /// Reseller user.
    ///
    /// "RESELLER"
    #[serde(rename="RESELLER")]
    RESELLER,
}

impl AsRef<str> for GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum::OPERATORTYPEUNSPECIFIED => "OPERATOR_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum::CUSTOMERSERVICEREPRESENTATIVE => "CUSTOMER_SERVICE_REPRESENTATIVE",
            GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum::SYSTEM => "SYSTEM",
            GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum::CUSTOMER => "CUSTOMER",
            GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum::RESELLER => "RESELLER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum::OPERATORTYPEUNSPECIFIED),
           "CUSTOMER_SERVICE_REPRESENTATIVE" => Ok(GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum::CUSTOMERSERVICEREPRESENTATIVE),
           "SYSTEM" => Ok(GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum::SYSTEM),
           "CUSTOMER" => Ok(GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum::CUSTOMER),
           "RESELLER" => Ok(GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum::RESELLER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1EntitlementChangeOperatorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Suspension reason for the Entitlement.
pub enum GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum {
    

    /// Not used.
    ///
    /// "SUSPENSION_REASON_UNSPECIFIED"
    #[serde(rename="SUSPENSION_REASON_UNSPECIFIED")]
    SUSPENSIONREASONUNSPECIFIED,
    

    /// Entitlement was manually suspended by the Reseller.
    ///
    /// "RESELLER_INITIATED"
    #[serde(rename="RESELLER_INITIATED")]
    RESELLERINITIATED,
    

    /// Trial ended.
    ///
    /// "TRIAL_ENDED"
    #[serde(rename="TRIAL_ENDED")]
    TRIALENDED,
    

    /// Entitlement renewal was canceled.
    ///
    /// "RENEWAL_WITH_TYPE_CANCEL"
    #[serde(rename="RENEWAL_WITH_TYPE_CANCEL")]
    RENEWALWITHTYPECANCEL,
    

    /// Entitlement was automatically suspended on creation for pending ToS acceptance on customer.
    ///
    /// "PENDING_TOS_ACCEPTANCE"
    #[serde(rename="PENDING_TOS_ACCEPTANCE")]
    PENDINGTOSACCEPTANCE,
    

    /// Other reasons (internal reasons, abuse, etc.).
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::SUSPENSIONREASONUNSPECIFIED => "SUSPENSION_REASON_UNSPECIFIED",
            GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::RESELLERINITIATED => "RESELLER_INITIATED",
            GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::TRIALENDED => "TRIAL_ENDED",
            GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::RENEWALWITHTYPECANCEL => "RENEWAL_WITH_TYPE_CANCEL",
            GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::PENDINGTOSACCEPTANCE => "PENDING_TOS_ACCEPTANCE",
            GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUSPENSION_REASON_UNSPECIFIED" => Ok(GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::SUSPENSIONREASONUNSPECIFIED),
           "RESELLER_INITIATED" => Ok(GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::RESELLERINITIATED),
           "TRIAL_ENDED" => Ok(GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::TRIALENDED),
           "RENEWAL_WITH_TYPE_CANCEL" => Ok(GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::RENEWALWITHTYPECANCEL),
           "PENDING_TOS_ACCEPTANCE" => Ok(GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::PENDINGTOSACCEPTANCE),
           "OTHER" => Ok(GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1EntitlementChangeSuspensionReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1MediaTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the media.
pub enum GoogleCloudChannelV1MediaTypeEnum {
    

    /// Not used.
    ///
    /// "MEDIA_TYPE_UNSPECIFIED"
    #[serde(rename="MEDIA_TYPE_UNSPECIFIED")]
    MEDIATYPEUNSPECIFIED,
    

    /// Type of image.
    ///
    /// "MEDIA_TYPE_IMAGE"
    #[serde(rename="MEDIA_TYPE_IMAGE")]
    MEDIATYPEIMAGE,
}

impl AsRef<str> for GoogleCloudChannelV1MediaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1MediaTypeEnum::MEDIATYPEUNSPECIFIED => "MEDIA_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1MediaTypeEnum::MEDIATYPEIMAGE => "MEDIA_TYPE_IMAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1MediaTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEDIA_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1MediaTypeEnum::MEDIATYPEUNSPECIFIED),
           "MEDIA_TYPE_IMAGE" => Ok(GoogleCloudChannelV1MediaTypeEnum::MEDIATYPEIMAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1MediaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Data type of the parameter. Minimal value, Maximum value and allowed values will use specified data type here.
pub enum GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum {
    

    /// Not used.
    ///
    /// "PARAMETER_TYPE_UNSPECIFIED"
    #[serde(rename="PARAMETER_TYPE_UNSPECIFIED")]
    PARAMETERTYPEUNSPECIFIED,
    

    /// Int64 type.
    ///
    /// "INT64"
    #[serde(rename="INT64")]
    INT64,
    

    /// String type.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// Double type.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
    

    /// Boolean type.
    ///
    /// "BOOLEAN"
    #[serde(rename="BOOLEAN")]
    BOOLEAN,
}

impl AsRef<str> for GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum::PARAMETERTYPEUNSPECIFIED => "PARAMETER_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum::INT64 => "INT64",
            GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum::STRING => "STRING",
            GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum::DOUBLE => "DOUBLE",
            GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum::BOOLEAN => "BOOLEAN",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARAMETER_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum::PARAMETERTYPEUNSPECIFIED),
           "INT64" => Ok(GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum::INT64),
           "STRING" => Ok(GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum::STRING),
           "DOUBLE" => Ok(GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum::DOUBLE),
           "BOOLEAN" => Ok(GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum::BOOLEAN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1PeriodPeriodTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Period Type.
pub enum GoogleCloudChannelV1PeriodPeriodTypeEnum {
    

    /// Not used.
    ///
    /// "PERIOD_TYPE_UNSPECIFIED"
    #[serde(rename="PERIOD_TYPE_UNSPECIFIED")]
    PERIODTYPEUNSPECIFIED,
    

    /// Day.
    ///
    /// "DAY"
    #[serde(rename="DAY")]
    DAY,
    

    /// Month.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// Year.
    ///
    /// "YEAR"
    #[serde(rename="YEAR")]
    YEAR,
}

impl AsRef<str> for GoogleCloudChannelV1PeriodPeriodTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1PeriodPeriodTypeEnum::PERIODTYPEUNSPECIFIED => "PERIOD_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1PeriodPeriodTypeEnum::DAY => "DAY",
            GoogleCloudChannelV1PeriodPeriodTypeEnum::MONTH => "MONTH",
            GoogleCloudChannelV1PeriodPeriodTypeEnum::YEAR => "YEAR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1PeriodPeriodTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERIOD_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1PeriodPeriodTypeEnum::PERIODTYPEUNSPECIFIED),
           "DAY" => Ok(GoogleCloudChannelV1PeriodPeriodTypeEnum::DAY),
           "MONTH" => Ok(GoogleCloudChannelV1PeriodPeriodTypeEnum::MONTH),
           "YEAR" => Ok(GoogleCloudChannelV1PeriodPeriodTypeEnum::YEAR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1PeriodPeriodTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1PlanPaymentPlanEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Describes how a reseller will be billed.
pub enum GoogleCloudChannelV1PlanPaymentPlanEnum {
    

    /// Not used.
    ///
    /// "PAYMENT_PLAN_UNSPECIFIED"
    #[serde(rename="PAYMENT_PLAN_UNSPECIFIED")]
    PAYMENTPLANUNSPECIFIED,
    

    /// Commitment.
    ///
    /// "COMMITMENT"
    #[serde(rename="COMMITMENT")]
    COMMITMENT,
    

    /// No commitment.
    ///
    /// "FLEXIBLE"
    #[serde(rename="FLEXIBLE")]
    FLEXIBLE,
    

    /// Free.
    ///
    /// "FREE"
    #[serde(rename="FREE")]
    FREE,
    

    /// Trial.
    ///
    /// "TRIAL"
    #[serde(rename="TRIAL")]
    TRIAL,
    

    /// Price and ordering not available through API.
    ///
    /// "OFFLINE"
    #[serde(rename="OFFLINE")]
    OFFLINE,
}

impl AsRef<str> for GoogleCloudChannelV1PlanPaymentPlanEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1PlanPaymentPlanEnum::PAYMENTPLANUNSPECIFIED => "PAYMENT_PLAN_UNSPECIFIED",
            GoogleCloudChannelV1PlanPaymentPlanEnum::COMMITMENT => "COMMITMENT",
            GoogleCloudChannelV1PlanPaymentPlanEnum::FLEXIBLE => "FLEXIBLE",
            GoogleCloudChannelV1PlanPaymentPlanEnum::FREE => "FREE",
            GoogleCloudChannelV1PlanPaymentPlanEnum::TRIAL => "TRIAL",
            GoogleCloudChannelV1PlanPaymentPlanEnum::OFFLINE => "OFFLINE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1PlanPaymentPlanEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PAYMENT_PLAN_UNSPECIFIED" => Ok(GoogleCloudChannelV1PlanPaymentPlanEnum::PAYMENTPLANUNSPECIFIED),
           "COMMITMENT" => Ok(GoogleCloudChannelV1PlanPaymentPlanEnum::COMMITMENT),
           "FLEXIBLE" => Ok(GoogleCloudChannelV1PlanPaymentPlanEnum::FLEXIBLE),
           "FREE" => Ok(GoogleCloudChannelV1PlanPaymentPlanEnum::FREE),
           "TRIAL" => Ok(GoogleCloudChannelV1PlanPaymentPlanEnum::TRIAL),
           "OFFLINE" => Ok(GoogleCloudChannelV1PlanPaymentPlanEnum::OFFLINE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1PlanPaymentPlanEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1PlanPaymentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies when the payment needs to happen.
pub enum GoogleCloudChannelV1PlanPaymentTypeEnum {
    

    /// Not used.
    ///
    /// "PAYMENT_TYPE_UNSPECIFIED"
    #[serde(rename="PAYMENT_TYPE_UNSPECIFIED")]
    PAYMENTTYPEUNSPECIFIED,
    

    /// Prepay. Amount has to be paid before service is rendered.
    ///
    /// "PREPAY"
    #[serde(rename="PREPAY")]
    PREPAY,
    

    /// Postpay. Reseller is charged at the end of the Payment cycle.
    ///
    /// "POSTPAY"
    #[serde(rename="POSTPAY")]
    POSTPAY,
}

impl AsRef<str> for GoogleCloudChannelV1PlanPaymentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1PlanPaymentTypeEnum::PAYMENTTYPEUNSPECIFIED => "PAYMENT_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1PlanPaymentTypeEnum::PREPAY => "PREPAY",
            GoogleCloudChannelV1PlanPaymentTypeEnum::POSTPAY => "POSTPAY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1PlanPaymentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PAYMENT_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1PlanPaymentTypeEnum::PAYMENTTYPEUNSPECIFIED),
           "PREPAY" => Ok(GoogleCloudChannelV1PlanPaymentTypeEnum::PREPAY),
           "POSTPAY" => Ok(GoogleCloudChannelV1PlanPaymentTypeEnum::POSTPAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1PlanPaymentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1PriceByResourceResourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Resource Type. Example: SEAT
pub enum GoogleCloudChannelV1PriceByResourceResourceTypeEnum {
    

    /// Not used.
    ///
    /// "RESOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="RESOURCE_TYPE_UNSPECIFIED")]
    RESOURCETYPEUNSPECIFIED,
    

    /// Seat.
    ///
    /// "SEAT"
    #[serde(rename="SEAT")]
    SEAT,
    

    /// Monthly active user.
    ///
    /// "MAU"
    #[serde(rename="MAU")]
    MAU,
    

    /// GB (used for storage SKUs).
    ///
    /// "GB"
    #[serde(rename="GB")]
    GB,
    

    /// Active licensed users(for Voice SKUs).
    ///
    /// "LICENSED_USER"
    #[serde(rename="LICENSED_USER")]
    LICENSEDUSER,
    

    /// Voice usage.
    ///
    /// "MINUTES"
    #[serde(rename="MINUTES")]
    MINUTES,
    

    /// For IaaS SKUs like Google Cloud, monetization is based on usage accrued on your billing account irrespective of the type of monetizable resource. This enum represents an aggregated resource/container for all usage SKUs on a billing account. Currently, only applicable to Google Cloud.
    ///
    /// "IAAS_USAGE"
    #[serde(rename="IAAS_USAGE")]
    IAASUSAGE,
    

    /// For Google Cloud subscriptions like Anthos or SAP.
    ///
    /// "SUBSCRIPTION"
    #[serde(rename="SUBSCRIPTION")]
    SUBSCRIPTION,
}

impl AsRef<str> for GoogleCloudChannelV1PriceByResourceResourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1PriceByResourceResourceTypeEnum::RESOURCETYPEUNSPECIFIED => "RESOURCE_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1PriceByResourceResourceTypeEnum::SEAT => "SEAT",
            GoogleCloudChannelV1PriceByResourceResourceTypeEnum::MAU => "MAU",
            GoogleCloudChannelV1PriceByResourceResourceTypeEnum::GB => "GB",
            GoogleCloudChannelV1PriceByResourceResourceTypeEnum::LICENSEDUSER => "LICENSED_USER",
            GoogleCloudChannelV1PriceByResourceResourceTypeEnum::MINUTES => "MINUTES",
            GoogleCloudChannelV1PriceByResourceResourceTypeEnum::IAASUSAGE => "IAAS_USAGE",
            GoogleCloudChannelV1PriceByResourceResourceTypeEnum::SUBSCRIPTION => "SUBSCRIPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1PriceByResourceResourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1PriceByResourceResourceTypeEnum::RESOURCETYPEUNSPECIFIED),
           "SEAT" => Ok(GoogleCloudChannelV1PriceByResourceResourceTypeEnum::SEAT),
           "MAU" => Ok(GoogleCloudChannelV1PriceByResourceResourceTypeEnum::MAU),
           "GB" => Ok(GoogleCloudChannelV1PriceByResourceResourceTypeEnum::GB),
           "LICENSED_USER" => Ok(GoogleCloudChannelV1PriceByResourceResourceTypeEnum::LICENSEDUSER),
           "MINUTES" => Ok(GoogleCloudChannelV1PriceByResourceResourceTypeEnum::MINUTES),
           "IAAS_USAGE" => Ok(GoogleCloudChannelV1PriceByResourceResourceTypeEnum::IAASUSAGE),
           "SUBSCRIPTION" => Ok(GoogleCloudChannelV1PriceByResourceResourceTypeEnum::SUBSCRIPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1PriceByResourceResourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1PricePhasePeriodTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defines the phase period type.
pub enum GoogleCloudChannelV1PricePhasePeriodTypeEnum {
    

    /// Not used.
    ///
    /// "PERIOD_TYPE_UNSPECIFIED"
    #[serde(rename="PERIOD_TYPE_UNSPECIFIED")]
    PERIODTYPEUNSPECIFIED,
    

    /// Day.
    ///
    /// "DAY"
    #[serde(rename="DAY")]
    DAY,
    

    /// Month.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// Year.
    ///
    /// "YEAR"
    #[serde(rename="YEAR")]
    YEAR,
}

impl AsRef<str> for GoogleCloudChannelV1PricePhasePeriodTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1PricePhasePeriodTypeEnum::PERIODTYPEUNSPECIFIED => "PERIOD_TYPE_UNSPECIFIED",
            GoogleCloudChannelV1PricePhasePeriodTypeEnum::DAY => "DAY",
            GoogleCloudChannelV1PricePhasePeriodTypeEnum::MONTH => "MONTH",
            GoogleCloudChannelV1PricePhasePeriodTypeEnum::YEAR => "YEAR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1PricePhasePeriodTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERIOD_TYPE_UNSPECIFIED" => Ok(GoogleCloudChannelV1PricePhasePeriodTypeEnum::PERIODTYPEUNSPECIFIED),
           "DAY" => Ok(GoogleCloudChannelV1PricePhasePeriodTypeEnum::DAY),
           "MONTH" => Ok(GoogleCloudChannelV1PricePhasePeriodTypeEnum::MONTH),
           "YEAR" => Ok(GoogleCloudChannelV1PricePhasePeriodTypeEnum::YEAR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1PricePhasePeriodTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1RenewalSettingPaymentPlanEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Describes how a reseller will be billed.
pub enum GoogleCloudChannelV1RenewalSettingPaymentPlanEnum {
    

    /// Not used.
    ///
    /// "PAYMENT_PLAN_UNSPECIFIED"
    #[serde(rename="PAYMENT_PLAN_UNSPECIFIED")]
    PAYMENTPLANUNSPECIFIED,
    

    /// Commitment.
    ///
    /// "COMMITMENT"
    #[serde(rename="COMMITMENT")]
    COMMITMENT,
    

    /// No commitment.
    ///
    /// "FLEXIBLE"
    #[serde(rename="FLEXIBLE")]
    FLEXIBLE,
    

    /// Free.
    ///
    /// "FREE"
    #[serde(rename="FREE")]
    FREE,
    

    /// Trial.
    ///
    /// "TRIAL"
    #[serde(rename="TRIAL")]
    TRIAL,
    

    /// Price and ordering not available through API.
    ///
    /// "OFFLINE"
    #[serde(rename="OFFLINE")]
    OFFLINE,
}

impl AsRef<str> for GoogleCloudChannelV1RenewalSettingPaymentPlanEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::PAYMENTPLANUNSPECIFIED => "PAYMENT_PLAN_UNSPECIFIED",
            GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::COMMITMENT => "COMMITMENT",
            GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::FLEXIBLE => "FLEXIBLE",
            GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::FREE => "FREE",
            GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::TRIAL => "TRIAL",
            GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::OFFLINE => "OFFLINE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1RenewalSettingPaymentPlanEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PAYMENT_PLAN_UNSPECIFIED" => Ok(GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::PAYMENTPLANUNSPECIFIED),
           "COMMITMENT" => Ok(GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::COMMITMENT),
           "FLEXIBLE" => Ok(GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::FLEXIBLE),
           "FREE" => Ok(GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::FREE),
           "TRIAL" => Ok(GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::TRIAL),
           "OFFLINE" => Ok(GoogleCloudChannelV1RenewalSettingPaymentPlanEnum::OFFLINE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1RenewalSettingPaymentPlanEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1RepricingConfigRebillingBasisEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The RebillingBasis to use for this bill. Specifies the relative cost based on repricing costs you will apply.
pub enum GoogleCloudChannelV1RepricingConfigRebillingBasisEnum {
    

    /// Not used.
    ///
    /// "REBILLING_BASIS_UNSPECIFIED"
    #[serde(rename="REBILLING_BASIS_UNSPECIFIED")]
    REBILLINGBASISUNSPECIFIED,
    

    /// Use the list cost, also known as the MSRP.
    ///
    /// "COST_AT_LIST"
    #[serde(rename="COST_AT_LIST")]
    COSTATLIST,
    

    /// Pass through all discounts except the Reseller Program Discount. If this is the default cost base and no adjustments are specified, the output cost will be exactly what the customer would see if they viewed the bill in the Google Cloud Console.
    ///
    /// "DIRECT_CUSTOMER_COST"
    #[serde(rename="DIRECT_CUSTOMER_COST")]
    DIRECTCUSTOMERCOST,
}

impl AsRef<str> for GoogleCloudChannelV1RepricingConfigRebillingBasisEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1RepricingConfigRebillingBasisEnum::REBILLINGBASISUNSPECIFIED => "REBILLING_BASIS_UNSPECIFIED",
            GoogleCloudChannelV1RepricingConfigRebillingBasisEnum::COSTATLIST => "COST_AT_LIST",
            GoogleCloudChannelV1RepricingConfigRebillingBasisEnum::DIRECTCUSTOMERCOST => "DIRECT_CUSTOMER_COST",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1RepricingConfigRebillingBasisEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REBILLING_BASIS_UNSPECIFIED" => Ok(GoogleCloudChannelV1RepricingConfigRebillingBasisEnum::REBILLINGBASISUNSPECIFIED),
           "COST_AT_LIST" => Ok(GoogleCloudChannelV1RepricingConfigRebillingBasisEnum::COSTATLIST),
           "DIRECT_CUSTOMER_COST" => Ok(GoogleCloudChannelV1RepricingConfigRebillingBasisEnum::DIRECTCUSTOMERCOST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1RepricingConfigRebillingBasisEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specified the reason for ineligibility.
pub enum GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum {
    

    /// Not used.
    ///
    /// "REASON_UNSPECIFIED"
    #[serde(rename="REASON_UNSPECIFIED")]
    REASONUNSPECIFIED,
    

    /// Reseller needs to accept TOS before transferring the SKU.
    ///
    /// "PENDING_TOS_ACCEPTANCE"
    #[serde(rename="PENDING_TOS_ACCEPTANCE")]
    PENDINGTOSACCEPTANCE,
    

    /// Reseller not eligible to sell the SKU.
    ///
    /// "SKU_NOT_ELIGIBLE"
    #[serde(rename="SKU_NOT_ELIGIBLE")]
    SKUNOTELIGIBLE,
    

    /// SKU subscription is suspended
    ///
    /// "SKU_SUSPENDED"
    #[serde(rename="SKU_SUSPENDED")]
    SKUSUSPENDED,
    

    /// The reseller is not authorized to transact on this Product. See https://support.google.com/channelservices/answer/9759265
    ///
    /// "CHANNEL_PARTNER_NOT_AUTHORIZED_FOR_SKU"
    #[serde(rename="CHANNEL_PARTNER_NOT_AUTHORIZED_FOR_SKU")]
    CHANNELPARTNERNOTAUTHORIZEDFORSKU,
}

impl AsRef<str> for GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum::REASONUNSPECIFIED => "REASON_UNSPECIFIED",
            GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum::PENDINGTOSACCEPTANCE => "PENDING_TOS_ACCEPTANCE",
            GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum::SKUNOTELIGIBLE => "SKU_NOT_ELIGIBLE",
            GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum::SKUSUSPENDED => "SKU_SUSPENDED",
            GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum::CHANNELPARTNERNOTAUTHORIZEDFORSKU => "CHANNEL_PARTNER_NOT_AUTHORIZED_FOR_SKU",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REASON_UNSPECIFIED" => Ok(GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum::REASONUNSPECIFIED),
           "PENDING_TOS_ACCEPTANCE" => Ok(GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum::PENDINGTOSACCEPTANCE),
           "SKU_NOT_ELIGIBLE" => Ok(GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum::SKUNOTELIGIBLE),
           "SKU_SUSPENDED" => Ok(GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum::SKUSUSPENDED),
           "CHANNEL_PARTNER_NOT_AUTHORIZED_FOR_SKU" => Ok(GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum::CHANNELPARTNERNOTAUTHORIZEDFORSKU),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The level of granularity the ChannelPartnerLink will display.
pub enum AccountViewEnum {
    

    /// The default / unset value. The API will default to the BASIC view.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Includes all fields except the ChannelPartnerLink.channel_partner_cloud_identity_info.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Includes all fields.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for AccountViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountViewEnum::UNSPECIFIED => "UNSPECIFIED",
            AccountViewEnum::BASIC => "BASIC",
            AccountViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(AccountViewEnum::UNSPECIFIED),
           "BASIC" => Ok(AccountViewEnum::BASIC),
           "FULL" => Ok(AccountViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountChangeOfferPurchaseChangeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Change Type for the entitlement.
pub enum AccountChangeOfferPurchaseChangeTypeEnum {
    

    /// Not used.
    ///
    /// "CHANGE_TYPE_UNSPECIFIED"
    #[serde(rename="CHANGE_TYPE_UNSPECIFIED")]
    CHANGETYPEUNSPECIFIED,
    

    /// SKU is an upgrade on the current entitlement.
    ///
    /// "UPGRADE"
    #[serde(rename="UPGRADE")]
    UPGRADE,
    

    /// SKU is a downgrade on the current entitlement.
    ///
    /// "DOWNGRADE"
    #[serde(rename="DOWNGRADE")]
    DOWNGRADE,
}

impl AsRef<str> for AccountChangeOfferPurchaseChangeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountChangeOfferPurchaseChangeTypeEnum::CHANGETYPEUNSPECIFIED => "CHANGE_TYPE_UNSPECIFIED",
            AccountChangeOfferPurchaseChangeTypeEnum::UPGRADE => "UPGRADE",
            AccountChangeOfferPurchaseChangeTypeEnum::DOWNGRADE => "DOWNGRADE",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountChangeOfferPurchaseChangeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHANGE_TYPE_UNSPECIFIED" => Ok(AccountChangeOfferPurchaseChangeTypeEnum::CHANGETYPEUNSPECIFIED),
           "UPGRADE" => Ok(AccountChangeOfferPurchaseChangeTypeEnum::UPGRADE),
           "DOWNGRADE" => Ok(AccountChangeOfferPurchaseChangeTypeEnum::DOWNGRADE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountChangeOfferPurchaseChangeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


