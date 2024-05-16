use super::*;



// region AbiAliasEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Alias for an abi.
pub enum AbiAliasEnum {
    

    /// Unspecified abi.
    ///
    /// "UNSPECIFIED_CPU_ARCHITECTURE"
    #[serde(rename="UNSPECIFIED_CPU_ARCHITECTURE")]
    UNSPECIFIEDCPUARCHITECTURE,
    

    /// ARMEABI abi.
    ///
    /// "ARMEABI"
    #[serde(rename="ARMEABI")]
    ARMEABI,
    

    /// ARMEABI_V7A abi.
    ///
    /// "ARMEABI_V7A"
    #[serde(rename="ARMEABI_V7A")]
    ARMEABIV7A,
    

    /// ARM64_V8A abi.
    ///
    /// "ARM64_V8A"
    #[serde(rename="ARM64_V8A")]
    ARM64V8A,
    

    /// X86 abi.
    ///
    /// "X86"
    #[serde(rename="X86")]
    X86,
    

    /// X86_64 abi.
    ///
    /// "X86_64"
    #[serde(rename="X86_64")]
    X8664,
    

    /// RISCV64 abi.
    ///
    /// "RISCV64"
    #[serde(rename="RISCV64")]
    RISCV64,
}

impl AsRef<str> for AbiAliasEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AbiAliasEnum::UNSPECIFIEDCPUARCHITECTURE => "UNSPECIFIED_CPU_ARCHITECTURE",
            AbiAliasEnum::ARMEABI => "ARMEABI",
            AbiAliasEnum::ARMEABIV7A => "ARMEABI_V7A",
            AbiAliasEnum::ARM64V8A => "ARM64_V8A",
            AbiAliasEnum::X86 => "X86",
            AbiAliasEnum::X8664 => "X86_64",
            AbiAliasEnum::RISCV64 => "RISCV64",
        }
    }
}

impl std::convert::TryFrom< &str> for AbiAliasEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_CPU_ARCHITECTURE" => Ok(AbiAliasEnum::UNSPECIFIEDCPUARCHITECTURE),
           "ARMEABI" => Ok(AbiAliasEnum::ARMEABI),
           "ARMEABI_V7A" => Ok(AbiAliasEnum::ARMEABIV7A),
           "ARM64_V8A" => Ok(AbiAliasEnum::ARM64V8A),
           "X86" => Ok(AbiAliasEnum::X86),
           "X86_64" => Ok(AbiAliasEnum::X8664),
           "RISCV64" => Ok(AbiAliasEnum::RISCV64),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AbiAliasEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivateBasePlanRequestLatencyToleranceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive.
pub enum ActivateBasePlanRequestLatencyToleranceEnum {
    

    /// Defaults to PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED")]
    PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED,
    

    /// The update will propagate to clients within several minutes on average and up to a few hours in rare cases. Throughput is limited to 7,200 updates per app per hour.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE,
    

    /// The update will propagate to clients within 24 hours. Supports high throughput of up to 720,000 updates per app per hour using batch modification methods.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT,
}

impl AsRef<str> for ActivateBasePlanRequestLatencyToleranceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED => "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED",
            ActivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE",
            ActivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivateBasePlanRequestLatencyToleranceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED" => Ok(ActivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE" => Ok(ActivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT" => Ok(ActivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivateBasePlanRequestLatencyToleranceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivateSubscriptionOfferRequestLatencyToleranceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive.
pub enum ActivateSubscriptionOfferRequestLatencyToleranceEnum {
    

    /// Defaults to PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED")]
    PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED,
    

    /// The update will propagate to clients within several minutes on average and up to a few hours in rare cases. Throughput is limited to 7,200 updates per app per hour.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE,
    

    /// The update will propagate to clients within 24 hours. Supports high throughput of up to 720,000 updates per app per hour using batch modification methods.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT,
}

impl AsRef<str> for ActivateSubscriptionOfferRequestLatencyToleranceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED => "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED",
            ActivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE",
            ActivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivateSubscriptionOfferRequestLatencyToleranceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED" => Ok(ActivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE" => Ok(ActivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT" => Ok(ActivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivateSubscriptionOfferRequestLatencyToleranceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppRecoveryActionStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the recovery action.
pub enum AppRecoveryActionStatusEnum {
    

    /// RecoveryStatus is unspecified.
    ///
    /// "RECOVERY_STATUS_UNSPECIFIED"
    #[serde(rename="RECOVERY_STATUS_UNSPECIFIED")]
    RECOVERYSTATUSUNSPECIFIED,
    

    /// The app recovery action has not been canceled since it has been created.
    ///
    /// "RECOVERY_STATUS_ACTIVE"
    #[serde(rename="RECOVERY_STATUS_ACTIVE")]
    RECOVERYSTATUSACTIVE,
    

    /// The recovery action has been canceled. The action cannot be resumed.
    ///
    /// "RECOVERY_STATUS_CANCELED"
    #[serde(rename="RECOVERY_STATUS_CANCELED")]
    RECOVERYSTATUSCANCELED,
    

    /// The recovery action is in the draft state and has not yet been deployed to users.
    ///
    /// "RECOVERY_STATUS_DRAFT"
    #[serde(rename="RECOVERY_STATUS_DRAFT")]
    RECOVERYSTATUSDRAFT,
    

    /// The recovery action is generating recovery apks.
    ///
    /// "RECOVERY_STATUS_GENERATION_IN_PROGRESS"
    #[serde(rename="RECOVERY_STATUS_GENERATION_IN_PROGRESS")]
    RECOVERYSTATUSGENERATIONINPROGRESS,
    

    /// The app recovery action generation has failed.
    ///
    /// "RECOVERY_STATUS_GENERATION_FAILED"
    #[serde(rename="RECOVERY_STATUS_GENERATION_FAILED")]
    RECOVERYSTATUSGENERATIONFAILED,
}

impl AsRef<str> for AppRecoveryActionStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppRecoveryActionStatusEnum::RECOVERYSTATUSUNSPECIFIED => "RECOVERY_STATUS_UNSPECIFIED",
            AppRecoveryActionStatusEnum::RECOVERYSTATUSACTIVE => "RECOVERY_STATUS_ACTIVE",
            AppRecoveryActionStatusEnum::RECOVERYSTATUSCANCELED => "RECOVERY_STATUS_CANCELED",
            AppRecoveryActionStatusEnum::RECOVERYSTATUSDRAFT => "RECOVERY_STATUS_DRAFT",
            AppRecoveryActionStatusEnum::RECOVERYSTATUSGENERATIONINPROGRESS => "RECOVERY_STATUS_GENERATION_IN_PROGRESS",
            AppRecoveryActionStatusEnum::RECOVERYSTATUSGENERATIONFAILED => "RECOVERY_STATUS_GENERATION_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for AppRecoveryActionStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECOVERY_STATUS_UNSPECIFIED" => Ok(AppRecoveryActionStatusEnum::RECOVERYSTATUSUNSPECIFIED),
           "RECOVERY_STATUS_ACTIVE" => Ok(AppRecoveryActionStatusEnum::RECOVERYSTATUSACTIVE),
           "RECOVERY_STATUS_CANCELED" => Ok(AppRecoveryActionStatusEnum::RECOVERYSTATUSCANCELED),
           "RECOVERY_STATUS_DRAFT" => Ok(AppRecoveryActionStatusEnum::RECOVERYSTATUSDRAFT),
           "RECOVERY_STATUS_GENERATION_IN_PROGRESS" => Ok(AppRecoveryActionStatusEnum::RECOVERYSTATUSGENERATIONINPROGRESS),
           "RECOVERY_STATUS_GENERATION_FAILED" => Ok(AppRecoveryActionStatusEnum::RECOVERYSTATUSGENERATIONFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppRecoveryActionStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AssetModuleMetadataDeliveryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the delivery type for persistent install.
pub enum AssetModuleMetadataDeliveryTypeEnum {
    

    /// Unspecified delivery type.
    ///
    /// "UNKNOWN_DELIVERY_TYPE"
    #[serde(rename="UNKNOWN_DELIVERY_TYPE")]
    UNKNOWNDELIVERYTYPE,
    

    /// This module will always be downloaded as part of the initial install of the app.
    ///
    /// "INSTALL_TIME"
    #[serde(rename="INSTALL_TIME")]
    INSTALLTIME,
    

    /// This module is requested on-demand, which means it will not be part of the initial install, and will only be sent when requested by the client.
    ///
    /// "ON_DEMAND"
    #[serde(rename="ON_DEMAND")]
    ONDEMAND,
    

    /// This module will be downloaded immediately after initial install finishes. The app can be opened before these modules are downloaded.
    ///
    /// "FAST_FOLLOW"
    #[serde(rename="FAST_FOLLOW")]
    FASTFOLLOW,
}

impl AsRef<str> for AssetModuleMetadataDeliveryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssetModuleMetadataDeliveryTypeEnum::UNKNOWNDELIVERYTYPE => "UNKNOWN_DELIVERY_TYPE",
            AssetModuleMetadataDeliveryTypeEnum::INSTALLTIME => "INSTALL_TIME",
            AssetModuleMetadataDeliveryTypeEnum::ONDEMAND => "ON_DEMAND",
            AssetModuleMetadataDeliveryTypeEnum::FASTFOLLOW => "FAST_FOLLOW",
        }
    }
}

impl std::convert::TryFrom< &str> for AssetModuleMetadataDeliveryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_DELIVERY_TYPE" => Ok(AssetModuleMetadataDeliveryTypeEnum::UNKNOWNDELIVERYTYPE),
           "INSTALL_TIME" => Ok(AssetModuleMetadataDeliveryTypeEnum::INSTALLTIME),
           "ON_DEMAND" => Ok(AssetModuleMetadataDeliveryTypeEnum::ONDEMAND),
           "FAST_FOLLOW" => Ok(AssetModuleMetadataDeliveryTypeEnum::FASTFOLLOW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssetModuleMetadataDeliveryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AutoRenewingBasePlanTypeProrationModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The proration mode for the base plan determines what happens when a user switches to this plan from another base plan. If unspecified, defaults to CHARGE_ON_NEXT_BILLING_DATE.
pub enum AutoRenewingBasePlanTypeProrationModeEnum {
    

    /// Unspecified mode.
    ///
    /// "SUBSCRIPTION_PRORATION_MODE_UNSPECIFIED"
    #[serde(rename="SUBSCRIPTION_PRORATION_MODE_UNSPECIFIED")]
    SUBSCRIPTIONPRORATIONMODEUNSPECIFIED,
    

    /// Users will be charged for their new base plan at the end of their current billing period.
    ///
    /// "SUBSCRIPTION_PRORATION_MODE_CHARGE_ON_NEXT_BILLING_DATE"
    #[serde(rename="SUBSCRIPTION_PRORATION_MODE_CHARGE_ON_NEXT_BILLING_DATE")]
    SUBSCRIPTIONPRORATIONMODECHARGEONNEXTBILLINGDATE,
    

    /// Users will be charged for their new base plan immediately and in full. Any remaining period of their existing subscription will be used to extend the duration of the new billing plan.
    ///
    /// "SUBSCRIPTION_PRORATION_MODE_CHARGE_FULL_PRICE_IMMEDIATELY"
    #[serde(rename="SUBSCRIPTION_PRORATION_MODE_CHARGE_FULL_PRICE_IMMEDIATELY")]
    SUBSCRIPTIONPRORATIONMODECHARGEFULLPRICEIMMEDIATELY,
}

impl AsRef<str> for AutoRenewingBasePlanTypeProrationModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AutoRenewingBasePlanTypeProrationModeEnum::SUBSCRIPTIONPRORATIONMODEUNSPECIFIED => "SUBSCRIPTION_PRORATION_MODE_UNSPECIFIED",
            AutoRenewingBasePlanTypeProrationModeEnum::SUBSCRIPTIONPRORATIONMODECHARGEONNEXTBILLINGDATE => "SUBSCRIPTION_PRORATION_MODE_CHARGE_ON_NEXT_BILLING_DATE",
            AutoRenewingBasePlanTypeProrationModeEnum::SUBSCRIPTIONPRORATIONMODECHARGEFULLPRICEIMMEDIATELY => "SUBSCRIPTION_PRORATION_MODE_CHARGE_FULL_PRICE_IMMEDIATELY",
        }
    }
}

impl std::convert::TryFrom< &str> for AutoRenewingBasePlanTypeProrationModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBSCRIPTION_PRORATION_MODE_UNSPECIFIED" => Ok(AutoRenewingBasePlanTypeProrationModeEnum::SUBSCRIPTIONPRORATIONMODEUNSPECIFIED),
           "SUBSCRIPTION_PRORATION_MODE_CHARGE_ON_NEXT_BILLING_DATE" => Ok(AutoRenewingBasePlanTypeProrationModeEnum::SUBSCRIPTIONPRORATIONMODECHARGEONNEXTBILLINGDATE),
           "SUBSCRIPTION_PRORATION_MODE_CHARGE_FULL_PRICE_IMMEDIATELY" => Ok(AutoRenewingBasePlanTypeProrationModeEnum::SUBSCRIPTIONPRORATIONMODECHARGEFULLPRICEIMMEDIATELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AutoRenewingBasePlanTypeProrationModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AutoRenewingBasePlanTypeResubscribeStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether users should be able to resubscribe to this base plan in Google Play surfaces. Defaults to RESUBSCRIBE_STATE_ACTIVE if not specified.
pub enum AutoRenewingBasePlanTypeResubscribeStateEnum {
    

    /// Unspecified state.
    ///
    /// "RESUBSCRIBE_STATE_UNSPECIFIED"
    #[serde(rename="RESUBSCRIBE_STATE_UNSPECIFIED")]
    RESUBSCRIBESTATEUNSPECIFIED,
    

    /// Resubscribe is active.
    ///
    /// "RESUBSCRIBE_STATE_ACTIVE"
    #[serde(rename="RESUBSCRIBE_STATE_ACTIVE")]
    RESUBSCRIBESTATEACTIVE,
    

    /// Resubscribe is inactive.
    ///
    /// "RESUBSCRIBE_STATE_INACTIVE"
    #[serde(rename="RESUBSCRIBE_STATE_INACTIVE")]
    RESUBSCRIBESTATEINACTIVE,
}

impl AsRef<str> for AutoRenewingBasePlanTypeResubscribeStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AutoRenewingBasePlanTypeResubscribeStateEnum::RESUBSCRIBESTATEUNSPECIFIED => "RESUBSCRIBE_STATE_UNSPECIFIED",
            AutoRenewingBasePlanTypeResubscribeStateEnum::RESUBSCRIBESTATEACTIVE => "RESUBSCRIBE_STATE_ACTIVE",
            AutoRenewingBasePlanTypeResubscribeStateEnum::RESUBSCRIBESTATEINACTIVE => "RESUBSCRIBE_STATE_INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for AutoRenewingBasePlanTypeResubscribeStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESUBSCRIBE_STATE_UNSPECIFIED" => Ok(AutoRenewingBasePlanTypeResubscribeStateEnum::RESUBSCRIBESTATEUNSPECIFIED),
           "RESUBSCRIBE_STATE_ACTIVE" => Ok(AutoRenewingBasePlanTypeResubscribeStateEnum::RESUBSCRIBESTATEACTIVE),
           "RESUBSCRIBE_STATE_INACTIVE" => Ok(AutoRenewingBasePlanTypeResubscribeStateEnum::RESUBSCRIBESTATEINACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AutoRenewingBasePlanTypeResubscribeStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BasePlanStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the base plan, i.e. whether it's active. Draft and inactive base plans can be activated or deleted. Active base plans can be made inactive. Inactive base plans can be canceled. This field cannot be changed by updating the resource. Use the dedicated endpoints instead.
pub enum BasePlanStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The base plan is currently in a draft state, and hasn't been activated. It can be safely deleted at this point.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// The base plan is active and available for new subscribers.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The base plan is inactive and only available for existing subscribers.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
}

impl AsRef<str> for BasePlanStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BasePlanStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            BasePlanStateEnum::DRAFT => "DRAFT",
            BasePlanStateEnum::ACTIVE => "ACTIVE",
            BasePlanStateEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for BasePlanStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(BasePlanStateEnum::STATEUNSPECIFIED),
           "DRAFT" => Ok(BasePlanStateEnum::DRAFT),
           "ACTIVE" => Ok(BasePlanStateEnum::ACTIVE),
           "INACTIVE" => Ok(BasePlanStateEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BasePlanStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CancelSurveyResultReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason the user selected in the cancel survey.
pub enum CancelSurveyResultReasonEnum {
    

    /// Unspecified cancel survey reason.
    ///
    /// "CANCEL_SURVEY_REASON_UNSPECIFIED"
    #[serde(rename="CANCEL_SURVEY_REASON_UNSPECIFIED")]
    CANCELSURVEYREASONUNSPECIFIED,
    

    /// Not enough usage of the subscription.
    ///
    /// "CANCEL_SURVEY_REASON_NOT_ENOUGH_USAGE"
    #[serde(rename="CANCEL_SURVEY_REASON_NOT_ENOUGH_USAGE")]
    CANCELSURVEYREASONNOTENOUGHUSAGE,
    

    /// Technical issues while using the app.
    ///
    /// "CANCEL_SURVEY_REASON_TECHNICAL_ISSUES"
    #[serde(rename="CANCEL_SURVEY_REASON_TECHNICAL_ISSUES")]
    CANCELSURVEYREASONTECHNICALISSUES,
    

    /// Cost related issues.
    ///
    /// "CANCEL_SURVEY_REASON_COST_RELATED"
    #[serde(rename="CANCEL_SURVEY_REASON_COST_RELATED")]
    CANCELSURVEYREASONCOSTRELATED,
    

    /// The user found a better app.
    ///
    /// "CANCEL_SURVEY_REASON_FOUND_BETTER_APP"
    #[serde(rename="CANCEL_SURVEY_REASON_FOUND_BETTER_APP")]
    CANCELSURVEYREASONFOUNDBETTERAPP,
    

    /// Other reasons.
    ///
    /// "CANCEL_SURVEY_REASON_OTHERS"
    #[serde(rename="CANCEL_SURVEY_REASON_OTHERS")]
    CANCELSURVEYREASONOTHERS,
}

impl AsRef<str> for CancelSurveyResultReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CancelSurveyResultReasonEnum::CANCELSURVEYREASONUNSPECIFIED => "CANCEL_SURVEY_REASON_UNSPECIFIED",
            CancelSurveyResultReasonEnum::CANCELSURVEYREASONNOTENOUGHUSAGE => "CANCEL_SURVEY_REASON_NOT_ENOUGH_USAGE",
            CancelSurveyResultReasonEnum::CANCELSURVEYREASONTECHNICALISSUES => "CANCEL_SURVEY_REASON_TECHNICAL_ISSUES",
            CancelSurveyResultReasonEnum::CANCELSURVEYREASONCOSTRELATED => "CANCEL_SURVEY_REASON_COST_RELATED",
            CancelSurveyResultReasonEnum::CANCELSURVEYREASONFOUNDBETTERAPP => "CANCEL_SURVEY_REASON_FOUND_BETTER_APP",
            CancelSurveyResultReasonEnum::CANCELSURVEYREASONOTHERS => "CANCEL_SURVEY_REASON_OTHERS",
        }
    }
}

impl std::convert::TryFrom< &str> for CancelSurveyResultReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CANCEL_SURVEY_REASON_UNSPECIFIED" => Ok(CancelSurveyResultReasonEnum::CANCELSURVEYREASONUNSPECIFIED),
           "CANCEL_SURVEY_REASON_NOT_ENOUGH_USAGE" => Ok(CancelSurveyResultReasonEnum::CANCELSURVEYREASONNOTENOUGHUSAGE),
           "CANCEL_SURVEY_REASON_TECHNICAL_ISSUES" => Ok(CancelSurveyResultReasonEnum::CANCELSURVEYREASONTECHNICALISSUES),
           "CANCEL_SURVEY_REASON_COST_RELATED" => Ok(CancelSurveyResultReasonEnum::CANCELSURVEYREASONCOSTRELATED),
           "CANCEL_SURVEY_REASON_FOUND_BETTER_APP" => Ok(CancelSurveyResultReasonEnum::CANCELSURVEYREASONFOUNDBETTERAPP),
           "CANCEL_SURVEY_REASON_OTHERS" => Ok(CancelSurveyResultReasonEnum::CANCELSURVEYREASONOTHERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CancelSurveyResultReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeactivateBasePlanRequestLatencyToleranceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive.
pub enum DeactivateBasePlanRequestLatencyToleranceEnum {
    

    /// Defaults to PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED")]
    PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED,
    

    /// The update will propagate to clients within several minutes on average and up to a few hours in rare cases. Throughput is limited to 7,200 updates per app per hour.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE,
    

    /// The update will propagate to clients within 24 hours. Supports high throughput of up to 720,000 updates per app per hour using batch modification methods.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT,
}

impl AsRef<str> for DeactivateBasePlanRequestLatencyToleranceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeactivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED => "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED",
            DeactivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE",
            DeactivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT",
        }
    }
}

impl std::convert::TryFrom< &str> for DeactivateBasePlanRequestLatencyToleranceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED" => Ok(DeactivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE" => Ok(DeactivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT" => Ok(DeactivateBasePlanRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeactivateBasePlanRequestLatencyToleranceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeactivateSubscriptionOfferRequestLatencyToleranceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive.
pub enum DeactivateSubscriptionOfferRequestLatencyToleranceEnum {
    

    /// Defaults to PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED")]
    PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED,
    

    /// The update will propagate to clients within several minutes on average and up to a few hours in rare cases. Throughput is limited to 7,200 updates per app per hour.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE,
    

    /// The update will propagate to clients within 24 hours. Supports high throughput of up to 720,000 updates per app per hour using batch modification methods.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT,
}

impl AsRef<str> for DeactivateSubscriptionOfferRequestLatencyToleranceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeactivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED => "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED",
            DeactivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE",
            DeactivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT",
        }
    }
}

impl std::convert::TryFrom< &str> for DeactivateSubscriptionOfferRequestLatencyToleranceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED" => Ok(DeactivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE" => Ok(DeactivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT" => Ok(DeactivateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeactivateSubscriptionOfferRequestLatencyToleranceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeobfuscationFileSymbolTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the deobfuscation file.
pub enum DeobfuscationFileSymbolTypeEnum {
    

    /// Unspecified deobfuscation file type.
    ///
    /// "deobfuscationFileTypeUnspecified"
    #[serde(rename="deobfuscationFileTypeUnspecified")]
    DeobfuscationFileTypeUnspecified,
    

    /// Proguard deobfuscation file type.
    ///
    /// "proguard"
    #[serde(rename="proguard")]
    Proguard,
    

    /// Native debugging symbols file type.
    ///
    /// "nativeCode"
    #[serde(rename="nativeCode")]
    NativeCode,
}

impl AsRef<str> for DeobfuscationFileSymbolTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeobfuscationFileSymbolTypeEnum::DeobfuscationFileTypeUnspecified => "deobfuscationFileTypeUnspecified",
            DeobfuscationFileSymbolTypeEnum::Proguard => "proguard",
            DeobfuscationFileSymbolTypeEnum::NativeCode => "nativeCode",
        }
    }
}

impl std::convert::TryFrom< &str> for DeobfuscationFileSymbolTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "deobfuscationFileTypeUnspecified" => Ok(DeobfuscationFileSymbolTypeEnum::DeobfuscationFileTypeUnspecified),
           "proguard" => Ok(DeobfuscationFileSymbolTypeEnum::Proguard),
           "nativeCode" => Ok(DeobfuscationFileSymbolTypeEnum::NativeCode),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeobfuscationFileSymbolTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExternalSubscriptionSubscriptionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the external subscription.
pub enum ExternalSubscriptionSubscriptionTypeEnum {
    

    /// Unspecified, do not use.
    ///
    /// "SUBSCRIPTION_TYPE_UNSPECIFIED"
    #[serde(rename="SUBSCRIPTION_TYPE_UNSPECIFIED")]
    SUBSCRIPTIONTYPEUNSPECIFIED,
    

    /// This is a recurring subscription where the user is charged every billing cycle.
    ///
    /// "RECURRING"
    #[serde(rename="RECURRING")]
    RECURRING,
    

    /// This is a prepaid subscription where the user pays up front.
    ///
    /// "PREPAID"
    #[serde(rename="PREPAID")]
    PREPAID,
}

impl AsRef<str> for ExternalSubscriptionSubscriptionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExternalSubscriptionSubscriptionTypeEnum::SUBSCRIPTIONTYPEUNSPECIFIED => "SUBSCRIPTION_TYPE_UNSPECIFIED",
            ExternalSubscriptionSubscriptionTypeEnum::RECURRING => "RECURRING",
            ExternalSubscriptionSubscriptionTypeEnum::PREPAID => "PREPAID",
        }
    }
}

impl std::convert::TryFrom< &str> for ExternalSubscriptionSubscriptionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBSCRIPTION_TYPE_UNSPECIFIED" => Ok(ExternalSubscriptionSubscriptionTypeEnum::SUBSCRIPTIONTYPEUNSPECIFIED),
           "RECURRING" => Ok(ExternalSubscriptionSubscriptionTypeEnum::RECURRING),
           "PREPAID" => Ok(ExternalSubscriptionSubscriptionTypeEnum::PREPAID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExternalSubscriptionSubscriptionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExternalTransactionTransactionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the transaction.
pub enum ExternalTransactionTransactionStateEnum {
    

    /// Unspecified transaction state. Not used.
    ///
    /// "TRANSACTION_STATE_UNSPECIFIED"
    #[serde(rename="TRANSACTION_STATE_UNSPECIFIED")]
    TRANSACTIONSTATEUNSPECIFIED,
    

    /// The transaction has been successfully reported to Google.
    ///
    /// "TRANSACTION_REPORTED"
    #[serde(rename="TRANSACTION_REPORTED")]
    TRANSACTIONREPORTED,
    

    /// The transaction has been fully refunded.
    ///
    /// "TRANSACTION_CANCELED"
    #[serde(rename="TRANSACTION_CANCELED")]
    TRANSACTIONCANCELED,
}

impl AsRef<str> for ExternalTransactionTransactionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExternalTransactionTransactionStateEnum::TRANSACTIONSTATEUNSPECIFIED => "TRANSACTION_STATE_UNSPECIFIED",
            ExternalTransactionTransactionStateEnum::TRANSACTIONREPORTED => "TRANSACTION_REPORTED",
            ExternalTransactionTransactionStateEnum::TRANSACTIONCANCELED => "TRANSACTION_CANCELED",
        }
    }
}

impl std::convert::TryFrom< &str> for ExternalTransactionTransactionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSACTION_STATE_UNSPECIFIED" => Ok(ExternalTransactionTransactionStateEnum::TRANSACTIONSTATEUNSPECIFIED),
           "TRANSACTION_REPORTED" => Ok(ExternalTransactionTransactionStateEnum::TRANSACTIONREPORTED),
           "TRANSACTION_CANCELED" => Ok(ExternalTransactionTransactionStateEnum::TRANSACTIONCANCELED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExternalTransactionTransactionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GeneratedRecoveryApkRecoveryStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the recovery action corresponding to the recovery apk.
pub enum GeneratedRecoveryApkRecoveryStatusEnum {
    

    /// RecoveryStatus is unspecified.
    ///
    /// "RECOVERY_STATUS_UNSPECIFIED"
    #[serde(rename="RECOVERY_STATUS_UNSPECIFIED")]
    RECOVERYSTATUSUNSPECIFIED,
    

    /// The app recovery action has not been canceled since it has been created.
    ///
    /// "RECOVERY_STATUS_ACTIVE"
    #[serde(rename="RECOVERY_STATUS_ACTIVE")]
    RECOVERYSTATUSACTIVE,
    

    /// The recovery action has been canceled. The action cannot be resumed.
    ///
    /// "RECOVERY_STATUS_CANCELED"
    #[serde(rename="RECOVERY_STATUS_CANCELED")]
    RECOVERYSTATUSCANCELED,
    

    /// The recovery action is in the draft state and has not yet been deployed to users.
    ///
    /// "RECOVERY_STATUS_DRAFT"
    #[serde(rename="RECOVERY_STATUS_DRAFT")]
    RECOVERYSTATUSDRAFT,
    

    /// The recovery action is generating recovery apks.
    ///
    /// "RECOVERY_STATUS_GENERATION_IN_PROGRESS"
    #[serde(rename="RECOVERY_STATUS_GENERATION_IN_PROGRESS")]
    RECOVERYSTATUSGENERATIONINPROGRESS,
    

    /// The app recovery action generation has failed.
    ///
    /// "RECOVERY_STATUS_GENERATION_FAILED"
    #[serde(rename="RECOVERY_STATUS_GENERATION_FAILED")]
    RECOVERYSTATUSGENERATIONFAILED,
}

impl AsRef<str> for GeneratedRecoveryApkRecoveryStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSUNSPECIFIED => "RECOVERY_STATUS_UNSPECIFIED",
            GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSACTIVE => "RECOVERY_STATUS_ACTIVE",
            GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSCANCELED => "RECOVERY_STATUS_CANCELED",
            GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSDRAFT => "RECOVERY_STATUS_DRAFT",
            GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSGENERATIONINPROGRESS => "RECOVERY_STATUS_GENERATION_IN_PROGRESS",
            GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSGENERATIONFAILED => "RECOVERY_STATUS_GENERATION_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GeneratedRecoveryApkRecoveryStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECOVERY_STATUS_UNSPECIFIED" => Ok(GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSUNSPECIFIED),
           "RECOVERY_STATUS_ACTIVE" => Ok(GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSACTIVE),
           "RECOVERY_STATUS_CANCELED" => Ok(GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSCANCELED),
           "RECOVERY_STATUS_DRAFT" => Ok(GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSDRAFT),
           "RECOVERY_STATUS_GENERATION_IN_PROGRESS" => Ok(GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSGENERATIONINPROGRESS),
           "RECOVERY_STATUS_GENERATION_FAILED" => Ok(GeneratedRecoveryApkRecoveryStatusEnum::RECOVERYSTATUSGENERATIONFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GeneratedRecoveryApkRecoveryStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GrantAppLevelPermissionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The permissions granted to the user for this app.
pub enum GrantAppLevelPermissionsEnum {
    

    /// Unknown or unspecified permission.
    ///
    /// "APP_LEVEL_PERMISSION_UNSPECIFIED"
    #[serde(rename="APP_LEVEL_PERMISSION_UNSPECIFIED")]
    APPLEVELPERMISSIONUNSPECIFIED,
    

    /// View app information (read-only). Deprecated: Try defining a more granular capability. Otherwise, check AppLevelPermission.CAN_VIEW_NON_FINANCIAL_DATA.
    ///
    /// "CAN_ACCESS_APP"
    #[serde(rename="CAN_ACCESS_APP")]
    CANACCESSAPP,
    

    /// View financial data.
    ///
    /// "CAN_VIEW_FINANCIAL_DATA"
    #[serde(rename="CAN_VIEW_FINANCIAL_DATA")]
    CANVIEWFINANCIALDATA,
    

    /// Admin (all permissions).
    ///
    /// "CAN_MANAGE_PERMISSIONS"
    #[serde(rename="CAN_MANAGE_PERMISSIONS")]
    CANMANAGEPERMISSIONS,
    

    /// Reply to reviews.
    ///
    /// "CAN_REPLY_TO_REVIEWS"
    #[serde(rename="CAN_REPLY_TO_REVIEWS")]
    CANREPLYTOREVIEWS,
    

    /// Release to production, exclude devices, and use app signing by Google Play.
    ///
    /// "CAN_MANAGE_PUBLIC_APKS"
    #[serde(rename="CAN_MANAGE_PUBLIC_APKS")]
    CANMANAGEPUBLICAPKS,
    

    /// Release to testing tracks.
    ///
    /// "CAN_MANAGE_TRACK_APKS"
    #[serde(rename="CAN_MANAGE_TRACK_APKS")]
    CANMANAGETRACKAPKS,
    

    /// Manage testing tracks and edit tester lists.
    ///
    /// "CAN_MANAGE_TRACK_USERS"
    #[serde(rename="CAN_MANAGE_TRACK_USERS")]
    CANMANAGETRACKUSERS,
    

    /// Manage store presence.
    ///
    /// "CAN_MANAGE_PUBLIC_LISTING"
    #[serde(rename="CAN_MANAGE_PUBLIC_LISTING")]
    CANMANAGEPUBLICLISTING,
    

    /// Edit and delete draft apps.
    ///
    /// "CAN_MANAGE_DRAFT_APPS"
    #[serde(rename="CAN_MANAGE_DRAFT_APPS")]
    CANMANAGEDRAFTAPPS,
    

    /// Manage orders and subscriptions.
    ///
    /// "CAN_MANAGE_ORDERS"
    #[serde(rename="CAN_MANAGE_ORDERS")]
    CANMANAGEORDERS,
    

    /// Manage policy related pages.
    ///
    /// "CAN_MANAGE_APP_CONTENT"
    #[serde(rename="CAN_MANAGE_APP_CONTENT")]
    CANMANAGEAPPCONTENT,
    

    /// View app information (read-only).
    ///
    /// "CAN_VIEW_NON_FINANCIAL_DATA"
    #[serde(rename="CAN_VIEW_NON_FINANCIAL_DATA")]
    CANVIEWNONFINANCIALDATA,
    

    /// View app quality data such as Vitals, Crashes etc.
    ///
    /// "CAN_VIEW_APP_QUALITY"
    #[serde(rename="CAN_VIEW_APP_QUALITY")]
    CANVIEWAPPQUALITY,
    

    /// Manage the deep links setup of an app.
    ///
    /// "CAN_MANAGE_DEEPLINKS"
    #[serde(rename="CAN_MANAGE_DEEPLINKS")]
    CANMANAGEDEEPLINKS,
}

impl AsRef<str> for GrantAppLevelPermissionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GrantAppLevelPermissionsEnum::APPLEVELPERMISSIONUNSPECIFIED => "APP_LEVEL_PERMISSION_UNSPECIFIED",
            GrantAppLevelPermissionsEnum::CANACCESSAPP => "CAN_ACCESS_APP",
            GrantAppLevelPermissionsEnum::CANVIEWFINANCIALDATA => "CAN_VIEW_FINANCIAL_DATA",
            GrantAppLevelPermissionsEnum::CANMANAGEPERMISSIONS => "CAN_MANAGE_PERMISSIONS",
            GrantAppLevelPermissionsEnum::CANREPLYTOREVIEWS => "CAN_REPLY_TO_REVIEWS",
            GrantAppLevelPermissionsEnum::CANMANAGEPUBLICAPKS => "CAN_MANAGE_PUBLIC_APKS",
            GrantAppLevelPermissionsEnum::CANMANAGETRACKAPKS => "CAN_MANAGE_TRACK_APKS",
            GrantAppLevelPermissionsEnum::CANMANAGETRACKUSERS => "CAN_MANAGE_TRACK_USERS",
            GrantAppLevelPermissionsEnum::CANMANAGEPUBLICLISTING => "CAN_MANAGE_PUBLIC_LISTING",
            GrantAppLevelPermissionsEnum::CANMANAGEDRAFTAPPS => "CAN_MANAGE_DRAFT_APPS",
            GrantAppLevelPermissionsEnum::CANMANAGEORDERS => "CAN_MANAGE_ORDERS",
            GrantAppLevelPermissionsEnum::CANMANAGEAPPCONTENT => "CAN_MANAGE_APP_CONTENT",
            GrantAppLevelPermissionsEnum::CANVIEWNONFINANCIALDATA => "CAN_VIEW_NON_FINANCIAL_DATA",
            GrantAppLevelPermissionsEnum::CANVIEWAPPQUALITY => "CAN_VIEW_APP_QUALITY",
            GrantAppLevelPermissionsEnum::CANMANAGEDEEPLINKS => "CAN_MANAGE_DEEPLINKS",
        }
    }
}

impl std::convert::TryFrom< &str> for GrantAppLevelPermissionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_LEVEL_PERMISSION_UNSPECIFIED" => Ok(GrantAppLevelPermissionsEnum::APPLEVELPERMISSIONUNSPECIFIED),
           "CAN_ACCESS_APP" => Ok(GrantAppLevelPermissionsEnum::CANACCESSAPP),
           "CAN_VIEW_FINANCIAL_DATA" => Ok(GrantAppLevelPermissionsEnum::CANVIEWFINANCIALDATA),
           "CAN_MANAGE_PERMISSIONS" => Ok(GrantAppLevelPermissionsEnum::CANMANAGEPERMISSIONS),
           "CAN_REPLY_TO_REVIEWS" => Ok(GrantAppLevelPermissionsEnum::CANREPLYTOREVIEWS),
           "CAN_MANAGE_PUBLIC_APKS" => Ok(GrantAppLevelPermissionsEnum::CANMANAGEPUBLICAPKS),
           "CAN_MANAGE_TRACK_APKS" => Ok(GrantAppLevelPermissionsEnum::CANMANAGETRACKAPKS),
           "CAN_MANAGE_TRACK_USERS" => Ok(GrantAppLevelPermissionsEnum::CANMANAGETRACKUSERS),
           "CAN_MANAGE_PUBLIC_LISTING" => Ok(GrantAppLevelPermissionsEnum::CANMANAGEPUBLICLISTING),
           "CAN_MANAGE_DRAFT_APPS" => Ok(GrantAppLevelPermissionsEnum::CANMANAGEDRAFTAPPS),
           "CAN_MANAGE_ORDERS" => Ok(GrantAppLevelPermissionsEnum::CANMANAGEORDERS),
           "CAN_MANAGE_APP_CONTENT" => Ok(GrantAppLevelPermissionsEnum::CANMANAGEAPPCONTENT),
           "CAN_VIEW_NON_FINANCIAL_DATA" => Ok(GrantAppLevelPermissionsEnum::CANVIEWNONFINANCIALDATA),
           "CAN_VIEW_APP_QUALITY" => Ok(GrantAppLevelPermissionsEnum::CANVIEWAPPQUALITY),
           "CAN_MANAGE_DEEPLINKS" => Ok(GrantAppLevelPermissionsEnum::CANMANAGEDEEPLINKS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GrantAppLevelPermissionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InAppProductPurchaseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the product, e.g. a recurring subscription.
pub enum InAppProductPurchaseTypeEnum {
    

    /// Unspecified purchase type.
    ///
    /// "purchaseTypeUnspecified"
    #[serde(rename="purchaseTypeUnspecified")]
    PurchaseTypeUnspecified,
    

    /// The default product type - one time purchase.
    ///
    /// "managedUser"
    #[serde(rename="managedUser")]
    ManagedUser,
    

    /// In-app product with a recurring period.
    ///
    /// "subscription"
    #[serde(rename="subscription")]
    Subscription,
}

impl AsRef<str> for InAppProductPurchaseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InAppProductPurchaseTypeEnum::PurchaseTypeUnspecified => "purchaseTypeUnspecified",
            InAppProductPurchaseTypeEnum::ManagedUser => "managedUser",
            InAppProductPurchaseTypeEnum::Subscription => "subscription",
        }
    }
}

impl std::convert::TryFrom< &str> for InAppProductPurchaseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "purchaseTypeUnspecified" => Ok(InAppProductPurchaseTypeEnum::PurchaseTypeUnspecified),
           "managedUser" => Ok(InAppProductPurchaseTypeEnum::ManagedUser),
           "subscription" => Ok(InAppProductPurchaseTypeEnum::Subscription),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InAppProductPurchaseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InAppProductStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the product, e.g. whether it's active.
pub enum InAppProductStatusEnum {
    

    /// Unspecified status.
    ///
    /// "statusUnspecified"
    #[serde(rename="statusUnspecified")]
    StatusUnspecified,
    

    /// The product is published and active in the store.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    

    /// The product is not published and therefore inactive in the store.
    ///
    /// "inactive"
    #[serde(rename="inactive")]
    Inactive,
}

impl AsRef<str> for InAppProductStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InAppProductStatusEnum::StatusUnspecified => "statusUnspecified",
            InAppProductStatusEnum::Active => "active",
            InAppProductStatusEnum::Inactive => "inactive",
        }
    }
}

impl std::convert::TryFrom< &str> for InAppProductStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "statusUnspecified" => Ok(InAppProductStatusEnum::StatusUnspecified),
           "active" => Ok(InAppProductStatusEnum::Active),
           "inactive" => Ok(InAppProductStatusEnum::Inactive),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InAppProductStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InappproductsDeleteRequestLatencyToleranceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive.
pub enum InappproductsDeleteRequestLatencyToleranceEnum {
    

    /// Defaults to PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED")]
    PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED,
    

    /// The update will propagate to clients within several minutes on average and up to a few hours in rare cases. Throughput is limited to 7,200 updates per app per hour.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE,
    

    /// The update will propagate to clients within 24 hours. Supports high throughput of up to 720,000 updates per app per hour using batch modification methods.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT,
}

impl AsRef<str> for InappproductsDeleteRequestLatencyToleranceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InappproductsDeleteRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED => "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED",
            InappproductsDeleteRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE",
            InappproductsDeleteRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT",
        }
    }
}

impl std::convert::TryFrom< &str> for InappproductsDeleteRequestLatencyToleranceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED" => Ok(InappproductsDeleteRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE" => Ok(InappproductsDeleteRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT" => Ok(InappproductsDeleteRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InappproductsDeleteRequestLatencyToleranceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InappproductsUpdateRequestLatencyToleranceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive.
pub enum InappproductsUpdateRequestLatencyToleranceEnum {
    

    /// Defaults to PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED")]
    PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED,
    

    /// The update will propagate to clients within several minutes on average and up to a few hours in rare cases. Throughput is limited to 7,200 updates per app per hour.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE,
    

    /// The update will propagate to clients within 24 hours. Supports high throughput of up to 720,000 updates per app per hour using batch modification methods.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT,
}

impl AsRef<str> for InappproductsUpdateRequestLatencyToleranceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InappproductsUpdateRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED => "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED",
            InappproductsUpdateRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE",
            InappproductsUpdateRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT",
        }
    }
}

impl std::convert::TryFrom< &str> for InappproductsUpdateRequestLatencyToleranceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED" => Ok(InappproductsUpdateRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE" => Ok(InappproductsUpdateRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT" => Ok(InappproductsUpdateRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InappproductsUpdateRequestLatencyToleranceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Digital content or service classification for products distributed to users in the European Economic Area (EEA). The withdrawal regime under EEA consumer laws depends on this classification. Refer to the [Help Center article](https://support.google.com/googleplay/android-developer/answer/10463498) for more information.
pub enum ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum {
    
    /// "WITHDRAWAL_RIGHT_TYPE_UNSPECIFIED"
    #[serde(rename="WITHDRAWAL_RIGHT_TYPE_UNSPECIFIED")]
    WITHDRAWALRIGHTTYPEUNSPECIFIED,
    
    /// "WITHDRAWAL_RIGHT_DIGITAL_CONTENT"
    #[serde(rename="WITHDRAWAL_RIGHT_DIGITAL_CONTENT")]
    WITHDRAWALRIGHTDIGITALCONTENT,
    
    /// "WITHDRAWAL_RIGHT_SERVICE"
    #[serde(rename="WITHDRAWAL_RIGHT_SERVICE")]
    WITHDRAWALRIGHTSERVICE,
}

impl AsRef<str> for ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTTYPEUNSPECIFIED => "WITHDRAWAL_RIGHT_TYPE_UNSPECIFIED",
            ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTDIGITALCONTENT => "WITHDRAWAL_RIGHT_DIGITAL_CONTENT",
            ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTSERVICE => "WITHDRAWAL_RIGHT_SERVICE",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WITHDRAWAL_RIGHT_TYPE_UNSPECIFIED" => Ok(ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTTYPEUNSPECIFIED),
           "WITHDRAWAL_RIGHT_DIGITAL_CONTENT" => Ok(ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTDIGITALCONTENT),
           "WITHDRAWAL_RIGHT_SERVICE" => Ok(ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTSERVICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigrateBasePlanPricesRequestLatencyToleranceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive.
pub enum MigrateBasePlanPricesRequestLatencyToleranceEnum {
    

    /// Defaults to PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED")]
    PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED,
    

    /// The update will propagate to clients within several minutes on average and up to a few hours in rare cases. Throughput is limited to 7,200 updates per app per hour.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE,
    

    /// The update will propagate to clients within 24 hours. Supports high throughput of up to 720,000 updates per app per hour using batch modification methods.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT,
}

impl AsRef<str> for MigrateBasePlanPricesRequestLatencyToleranceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigrateBasePlanPricesRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED => "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED",
            MigrateBasePlanPricesRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE",
            MigrateBasePlanPricesRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT",
        }
    }
}

impl std::convert::TryFrom< &str> for MigrateBasePlanPricesRequestLatencyToleranceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED" => Ok(MigrateBasePlanPricesRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE" => Ok(MigrateBasePlanPricesRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT" => Ok(MigrateBasePlanPricesRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigrateBasePlanPricesRequestLatencyToleranceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ModuleMetadataDeliveryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the delivery type (e.g. on-demand) of the module.
pub enum ModuleMetadataDeliveryTypeEnum {
    

    /// Unspecified delivery type.
    ///
    /// "UNKNOWN_DELIVERY_TYPE"
    #[serde(rename="UNKNOWN_DELIVERY_TYPE")]
    UNKNOWNDELIVERYTYPE,
    

    /// This module will always be downloaded as part of the initial install of the app.
    ///
    /// "INSTALL_TIME"
    #[serde(rename="INSTALL_TIME")]
    INSTALLTIME,
    

    /// This module is requested on-demand, which means it will not be part of the initial install, and will only be sent when requested by the client.
    ///
    /// "ON_DEMAND"
    #[serde(rename="ON_DEMAND")]
    ONDEMAND,
    

    /// This module will be downloaded immediately after initial install finishes. The app can be opened before these modules are downloaded.
    ///
    /// "FAST_FOLLOW"
    #[serde(rename="FAST_FOLLOW")]
    FASTFOLLOW,
}

impl AsRef<str> for ModuleMetadataDeliveryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ModuleMetadataDeliveryTypeEnum::UNKNOWNDELIVERYTYPE => "UNKNOWN_DELIVERY_TYPE",
            ModuleMetadataDeliveryTypeEnum::INSTALLTIME => "INSTALL_TIME",
            ModuleMetadataDeliveryTypeEnum::ONDEMAND => "ON_DEMAND",
            ModuleMetadataDeliveryTypeEnum::FASTFOLLOW => "FAST_FOLLOW",
        }
    }
}

impl std::convert::TryFrom< &str> for ModuleMetadataDeliveryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_DELIVERY_TYPE" => Ok(ModuleMetadataDeliveryTypeEnum::UNKNOWNDELIVERYTYPE),
           "INSTALL_TIME" => Ok(ModuleMetadataDeliveryTypeEnum::INSTALLTIME),
           "ON_DEMAND" => Ok(ModuleMetadataDeliveryTypeEnum::ONDEMAND),
           "FAST_FOLLOW" => Ok(ModuleMetadataDeliveryTypeEnum::FASTFOLLOW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ModuleMetadataDeliveryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ModuleMetadataModuleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the type of this feature module.
pub enum ModuleMetadataModuleTypeEnum {
    

    /// Unknown feature module.
    ///
    /// "UNKNOWN_MODULE_TYPE"
    #[serde(rename="UNKNOWN_MODULE_TYPE")]
    UNKNOWNMODULETYPE,
    

    /// Regular feature module.
    ///
    /// "FEATURE_MODULE"
    #[serde(rename="FEATURE_MODULE")]
    FEATUREMODULE,
}

impl AsRef<str> for ModuleMetadataModuleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ModuleMetadataModuleTypeEnum::UNKNOWNMODULETYPE => "UNKNOWN_MODULE_TYPE",
            ModuleMetadataModuleTypeEnum::FEATUREMODULE => "FEATURE_MODULE",
        }
    }
}

impl std::convert::TryFrom< &str> for ModuleMetadataModuleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_MODULE_TYPE" => Ok(ModuleMetadataModuleTypeEnum::UNKNOWNMODULETYPE),
           "FEATURE_MODULE" => Ok(ModuleMetadataModuleTypeEnum::FEATUREMODULE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ModuleMetadataModuleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PrepaidBasePlanTypeTimeExtensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether users should be able to extend this prepaid base plan in Google Play surfaces. Defaults to TIME_EXTENSION_ACTIVE if not specified.
pub enum PrepaidBasePlanTypeTimeExtensionEnum {
    

    /// Unspecified state.
    ///
    /// "TIME_EXTENSION_UNSPECIFIED"
    #[serde(rename="TIME_EXTENSION_UNSPECIFIED")]
    TIMEEXTENSIONUNSPECIFIED,
    

    /// Time extension is active. Users are allowed to top-up or extend their prepaid plan.
    ///
    /// "TIME_EXTENSION_ACTIVE"
    #[serde(rename="TIME_EXTENSION_ACTIVE")]
    TIMEEXTENSIONACTIVE,
    

    /// Time extension is inactive. Users cannot top-up or extend their prepaid plan.
    ///
    /// "TIME_EXTENSION_INACTIVE"
    #[serde(rename="TIME_EXTENSION_INACTIVE")]
    TIMEEXTENSIONINACTIVE,
}

impl AsRef<str> for PrepaidBasePlanTypeTimeExtensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PrepaidBasePlanTypeTimeExtensionEnum::TIMEEXTENSIONUNSPECIFIED => "TIME_EXTENSION_UNSPECIFIED",
            PrepaidBasePlanTypeTimeExtensionEnum::TIMEEXTENSIONACTIVE => "TIME_EXTENSION_ACTIVE",
            PrepaidBasePlanTypeTimeExtensionEnum::TIMEEXTENSIONINACTIVE => "TIME_EXTENSION_INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for PrepaidBasePlanTypeTimeExtensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIME_EXTENSION_UNSPECIFIED" => Ok(PrepaidBasePlanTypeTimeExtensionEnum::TIMEEXTENSIONUNSPECIFIED),
           "TIME_EXTENSION_ACTIVE" => Ok(PrepaidBasePlanTypeTimeExtensionEnum::TIMEEXTENSIONACTIVE),
           "TIME_EXTENSION_INACTIVE" => Ok(PrepaidBasePlanTypeTimeExtensionEnum::TIMEEXTENSIONINACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PrepaidBasePlanTypeTimeExtensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RecurringExternalTransactionMigratedTransactionProgramEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Input only. Provided during the call to Create. Must only be used when migrating a subscription from manual monthly reporting to automated reporting.
pub enum RecurringExternalTransactionMigratedTransactionProgramEnum {
    

    /// Unspecified transaction program. Not used.
    ///
    /// "EXTERNAL_TRANSACTION_PROGRAM_UNSPECIFIED"
    #[serde(rename="EXTERNAL_TRANSACTION_PROGRAM_UNSPECIFIED")]
    EXTERNALTRANSACTIONPROGRAMUNSPECIFIED,
    

    /// User choice billing, where a user may choose between Google Play Billing developer-managed billing.
    ///
    /// "USER_CHOICE_BILLING"
    #[serde(rename="USER_CHOICE_BILLING")]
    USERCHOICEBILLING,
    

    /// Alternative billing only, where users may only use developer-manager billing.
    ///
    /// "ALTERNATIVE_BILLING_ONLY"
    #[serde(rename="ALTERNATIVE_BILLING_ONLY")]
    ALTERNATIVEBILLINGONLY,
}

impl AsRef<str> for RecurringExternalTransactionMigratedTransactionProgramEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RecurringExternalTransactionMigratedTransactionProgramEnum::EXTERNALTRANSACTIONPROGRAMUNSPECIFIED => "EXTERNAL_TRANSACTION_PROGRAM_UNSPECIFIED",
            RecurringExternalTransactionMigratedTransactionProgramEnum::USERCHOICEBILLING => "USER_CHOICE_BILLING",
            RecurringExternalTransactionMigratedTransactionProgramEnum::ALTERNATIVEBILLINGONLY => "ALTERNATIVE_BILLING_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for RecurringExternalTransactionMigratedTransactionProgramEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXTERNAL_TRANSACTION_PROGRAM_UNSPECIFIED" => Ok(RecurringExternalTransactionMigratedTransactionProgramEnum::EXTERNALTRANSACTIONPROGRAMUNSPECIFIED),
           "USER_CHOICE_BILLING" => Ok(RecurringExternalTransactionMigratedTransactionProgramEnum::USERCHOICEBILLING),
           "ALTERNATIVE_BILLING_ONLY" => Ok(RecurringExternalTransactionMigratedTransactionProgramEnum::ALTERNATIVEBILLINGONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RecurringExternalTransactionMigratedTransactionProgramEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegionalPriceMigrationConfigPriceIncreaseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The behavior the caller wants users to see when there is a price increase during migration. If left unset, the behavior defaults to PRICE_INCREASE_TYPE_OPT_IN. Note that the first opt-out price increase migration for each app must be initiated in Play Console.
pub enum RegionalPriceMigrationConfigPriceIncreaseTypeEnum {
    

    /// Unspecified state.
    ///
    /// "PRICE_INCREASE_TYPE_UNSPECIFIED"
    #[serde(rename="PRICE_INCREASE_TYPE_UNSPECIFIED")]
    PRICEINCREASETYPEUNSPECIFIED,
    

    /// Price increase will be presented to users on an opt-in basis.
    ///
    /// "PRICE_INCREASE_TYPE_OPT_IN"
    #[serde(rename="PRICE_INCREASE_TYPE_OPT_IN")]
    PRICEINCREASETYPEOPTIN,
    

    /// Price increase will be presented to users on an opt-out basis.
    ///
    /// "PRICE_INCREASE_TYPE_OPT_OUT"
    #[serde(rename="PRICE_INCREASE_TYPE_OPT_OUT")]
    PRICEINCREASETYPEOPTOUT,
}

impl AsRef<str> for RegionalPriceMigrationConfigPriceIncreaseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegionalPriceMigrationConfigPriceIncreaseTypeEnum::PRICEINCREASETYPEUNSPECIFIED => "PRICE_INCREASE_TYPE_UNSPECIFIED",
            RegionalPriceMigrationConfigPriceIncreaseTypeEnum::PRICEINCREASETYPEOPTIN => "PRICE_INCREASE_TYPE_OPT_IN",
            RegionalPriceMigrationConfigPriceIncreaseTypeEnum::PRICEINCREASETYPEOPTOUT => "PRICE_INCREASE_TYPE_OPT_OUT",
        }
    }
}

impl std::convert::TryFrom< &str> for RegionalPriceMigrationConfigPriceIncreaseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRICE_INCREASE_TYPE_UNSPECIFIED" => Ok(RegionalPriceMigrationConfigPriceIncreaseTypeEnum::PRICEINCREASETYPEUNSPECIFIED),
           "PRICE_INCREASE_TYPE_OPT_IN" => Ok(RegionalPriceMigrationConfigPriceIncreaseTypeEnum::PRICEINCREASETYPEOPTIN),
           "PRICE_INCREASE_TYPE_OPT_OUT" => Ok(RegionalPriceMigrationConfigPriceIncreaseTypeEnum::PRICEINCREASETYPEOPTOUT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegionalPriceMigrationConfigPriceIncreaseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegionalTaxRateInfoStreamingTaxTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// To collect communications or amusement taxes in the United States, choose the appropriate tax category. [Learn more](https://support.google.com/googleplay/android-developer/answer/10463498#streaming_tax).
pub enum RegionalTaxRateInfoStreamingTaxTypeEnum {
    

    /// No telecommunications tax collected.
    ///
    /// "STREAMING_TAX_TYPE_UNSPECIFIED"
    #[serde(rename="STREAMING_TAX_TYPE_UNSPECIFIED")]
    STREAMINGTAXTYPEUNSPECIFIED,
    

    /// US-specific telecommunications tax tier for video streaming, on demand, rentals / subscriptions / pay-per-view.
    ///
    /// "STREAMING_TAX_TYPE_TELCO_VIDEO_RENTAL"
    #[serde(rename="STREAMING_TAX_TYPE_TELCO_VIDEO_RENTAL")]
    STREAMINGTAXTYPETELCOVIDEORENTAL,
    

    /// US-specific telecommunications tax tier for video streaming of pre-recorded content like movies, tv shows.
    ///
    /// "STREAMING_TAX_TYPE_TELCO_VIDEO_SALES"
    #[serde(rename="STREAMING_TAX_TYPE_TELCO_VIDEO_SALES")]
    STREAMINGTAXTYPETELCOVIDEOSALES,
    

    /// US-specific telecommunications tax tier for video streaming of multi-channel programming.
    ///
    /// "STREAMING_TAX_TYPE_TELCO_VIDEO_MULTI_CHANNEL"
    #[serde(rename="STREAMING_TAX_TYPE_TELCO_VIDEO_MULTI_CHANNEL")]
    STREAMINGTAXTYPETELCOVIDEOMULTICHANNEL,
    

    /// US-specific telecommunications tax tier for audio streaming, rental / subscription.
    ///
    /// "STREAMING_TAX_TYPE_TELCO_AUDIO_RENTAL"
    #[serde(rename="STREAMING_TAX_TYPE_TELCO_AUDIO_RENTAL")]
    STREAMINGTAXTYPETELCOAUDIORENTAL,
    

    /// US-specific telecommunications tax tier for audio streaming, sale / permanent download.
    ///
    /// "STREAMING_TAX_TYPE_TELCO_AUDIO_SALES"
    #[serde(rename="STREAMING_TAX_TYPE_TELCO_AUDIO_SALES")]
    STREAMINGTAXTYPETELCOAUDIOSALES,
    

    /// US-specific telecommunications tax tier for multi channel audio streaming like radio.
    ///
    /// "STREAMING_TAX_TYPE_TELCO_AUDIO_MULTI_CHANNEL"
    #[serde(rename="STREAMING_TAX_TYPE_TELCO_AUDIO_MULTI_CHANNEL")]
    STREAMINGTAXTYPETELCOAUDIOMULTICHANNEL,
}

impl AsRef<str> for RegionalTaxRateInfoStreamingTaxTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPEUNSPECIFIED => "STREAMING_TAX_TYPE_UNSPECIFIED",
            RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOVIDEORENTAL => "STREAMING_TAX_TYPE_TELCO_VIDEO_RENTAL",
            RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOVIDEOSALES => "STREAMING_TAX_TYPE_TELCO_VIDEO_SALES",
            RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOVIDEOMULTICHANNEL => "STREAMING_TAX_TYPE_TELCO_VIDEO_MULTI_CHANNEL",
            RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOAUDIORENTAL => "STREAMING_TAX_TYPE_TELCO_AUDIO_RENTAL",
            RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOAUDIOSALES => "STREAMING_TAX_TYPE_TELCO_AUDIO_SALES",
            RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOAUDIOMULTICHANNEL => "STREAMING_TAX_TYPE_TELCO_AUDIO_MULTI_CHANNEL",
        }
    }
}

impl std::convert::TryFrom< &str> for RegionalTaxRateInfoStreamingTaxTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STREAMING_TAX_TYPE_UNSPECIFIED" => Ok(RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPEUNSPECIFIED),
           "STREAMING_TAX_TYPE_TELCO_VIDEO_RENTAL" => Ok(RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOVIDEORENTAL),
           "STREAMING_TAX_TYPE_TELCO_VIDEO_SALES" => Ok(RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOVIDEOSALES),
           "STREAMING_TAX_TYPE_TELCO_VIDEO_MULTI_CHANNEL" => Ok(RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOVIDEOMULTICHANNEL),
           "STREAMING_TAX_TYPE_TELCO_AUDIO_RENTAL" => Ok(RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOAUDIORENTAL),
           "STREAMING_TAX_TYPE_TELCO_AUDIO_SALES" => Ok(RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOAUDIOSALES),
           "STREAMING_TAX_TYPE_TELCO_AUDIO_MULTI_CHANNEL" => Ok(RegionalTaxRateInfoStreamingTaxTypeEnum::STREAMINGTAXTYPETELCOAUDIOMULTICHANNEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegionalTaxRateInfoStreamingTaxTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegionalTaxRateInfoTaxTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tax tier to specify reduced tax rate. Developers who sell digital news, magazines, newspapers, books, or audiobooks in various regions may be eligible for reduced tax rates. [Learn more](https://support.google.com/googleplay/android-developer/answer/10463498).
pub enum RegionalTaxRateInfoTaxTierEnum {
    
    /// "TAX_TIER_UNSPECIFIED"
    #[serde(rename="TAX_TIER_UNSPECIFIED")]
    TAXTIERUNSPECIFIED,
    
    /// "TAX_TIER_BOOKS_1"
    #[serde(rename="TAX_TIER_BOOKS_1")]
    TAXTIERBOOKS1,
    
    /// "TAX_TIER_NEWS_1"
    #[serde(rename="TAX_TIER_NEWS_1")]
    TAXTIERNEWS1,
    
    /// "TAX_TIER_NEWS_2"
    #[serde(rename="TAX_TIER_NEWS_2")]
    TAXTIERNEWS2,
    
    /// "TAX_TIER_MUSIC_OR_AUDIO_1"
    #[serde(rename="TAX_TIER_MUSIC_OR_AUDIO_1")]
    TAXTIERMUSICORAUDIO1,
    
    /// "TAX_TIER_LIVE_OR_BROADCAST_1"
    #[serde(rename="TAX_TIER_LIVE_OR_BROADCAST_1")]
    TAXTIERLIVEORBROADCAST1,
}

impl AsRef<str> for RegionalTaxRateInfoTaxTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegionalTaxRateInfoTaxTierEnum::TAXTIERUNSPECIFIED => "TAX_TIER_UNSPECIFIED",
            RegionalTaxRateInfoTaxTierEnum::TAXTIERBOOKS1 => "TAX_TIER_BOOKS_1",
            RegionalTaxRateInfoTaxTierEnum::TAXTIERNEWS1 => "TAX_TIER_NEWS_1",
            RegionalTaxRateInfoTaxTierEnum::TAXTIERNEWS2 => "TAX_TIER_NEWS_2",
            RegionalTaxRateInfoTaxTierEnum::TAXTIERMUSICORAUDIO1 => "TAX_TIER_MUSIC_OR_AUDIO_1",
            RegionalTaxRateInfoTaxTierEnum::TAXTIERLIVEORBROADCAST1 => "TAX_TIER_LIVE_OR_BROADCAST_1",
        }
    }
}

impl std::convert::TryFrom< &str> for RegionalTaxRateInfoTaxTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TAX_TIER_UNSPECIFIED" => Ok(RegionalTaxRateInfoTaxTierEnum::TAXTIERUNSPECIFIED),
           "TAX_TIER_BOOKS_1" => Ok(RegionalTaxRateInfoTaxTierEnum::TAXTIERBOOKS1),
           "TAX_TIER_NEWS_1" => Ok(RegionalTaxRateInfoTaxTierEnum::TAXTIERNEWS1),
           "TAX_TIER_NEWS_2" => Ok(RegionalTaxRateInfoTaxTierEnum::TAXTIERNEWS2),
           "TAX_TIER_MUSIC_OR_AUDIO_1" => Ok(RegionalTaxRateInfoTaxTierEnum::TAXTIERMUSICORAUDIO1),
           "TAX_TIER_LIVE_OR_BROADCAST_1" => Ok(RegionalTaxRateInfoTaxTierEnum::TAXTIERLIVEORBROADCAST1),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegionalTaxRateInfoTaxTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ScreenDensityDensityAliasEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Alias for a screen density.
pub enum ScreenDensityDensityAliasEnum {
    

    /// Unspecified screen density.
    ///
    /// "DENSITY_UNSPECIFIED"
    #[serde(rename="DENSITY_UNSPECIFIED")]
    DENSITYUNSPECIFIED,
    

    /// NODPI screen density.
    ///
    /// "NODPI"
    #[serde(rename="NODPI")]
    NODPI,
    

    /// LDPI screen density.
    ///
    /// "LDPI"
    #[serde(rename="LDPI")]
    LDPI,
    

    /// MDPI screen density.
    ///
    /// "MDPI"
    #[serde(rename="MDPI")]
    MDPI,
    

    /// TVDPI screen density.
    ///
    /// "TVDPI"
    #[serde(rename="TVDPI")]
    TVDPI,
    

    /// HDPI screen density.
    ///
    /// "HDPI"
    #[serde(rename="HDPI")]
    HDPI,
    

    /// XHDPI screen density.
    ///
    /// "XHDPI"
    #[serde(rename="XHDPI")]
    XHDPI,
    

    /// XXHDPI screen density.
    ///
    /// "XXHDPI"
    #[serde(rename="XXHDPI")]
    XXHDPI,
    

    /// XXXHDPI screen density.
    ///
    /// "XXXHDPI"
    #[serde(rename="XXXHDPI")]
    XXXHDPI,
}

impl AsRef<str> for ScreenDensityDensityAliasEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ScreenDensityDensityAliasEnum::DENSITYUNSPECIFIED => "DENSITY_UNSPECIFIED",
            ScreenDensityDensityAliasEnum::NODPI => "NODPI",
            ScreenDensityDensityAliasEnum::LDPI => "LDPI",
            ScreenDensityDensityAliasEnum::MDPI => "MDPI",
            ScreenDensityDensityAliasEnum::TVDPI => "TVDPI",
            ScreenDensityDensityAliasEnum::HDPI => "HDPI",
            ScreenDensityDensityAliasEnum::XHDPI => "XHDPI",
            ScreenDensityDensityAliasEnum::XXHDPI => "XXHDPI",
            ScreenDensityDensityAliasEnum::XXXHDPI => "XXXHDPI",
        }
    }
}

impl std::convert::TryFrom< &str> for ScreenDensityDensityAliasEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DENSITY_UNSPECIFIED" => Ok(ScreenDensityDensityAliasEnum::DENSITYUNSPECIFIED),
           "NODPI" => Ok(ScreenDensityDensityAliasEnum::NODPI),
           "LDPI" => Ok(ScreenDensityDensityAliasEnum::LDPI),
           "MDPI" => Ok(ScreenDensityDensityAliasEnum::MDPI),
           "TVDPI" => Ok(ScreenDensityDensityAliasEnum::TVDPI),
           "HDPI" => Ok(ScreenDensityDensityAliasEnum::HDPI),
           "XHDPI" => Ok(ScreenDensityDensityAliasEnum::XHDPI),
           "XXHDPI" => Ok(ScreenDensityDensityAliasEnum::XXHDPI),
           "XXXHDPI" => Ok(ScreenDensityDensityAliasEnum::XXXHDPI),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ScreenDensityDensityAliasEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SubscriptionItemPriceChangeDetailPriceChangeModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Price change mode specifies how the subscription item price is changing.
pub enum SubscriptionItemPriceChangeDetailPriceChangeModeEnum {
    

    /// Price change mode unspecified. This value should never be set.
    ///
    /// "PRICE_CHANGE_MODE_UNSPECIFIED"
    #[serde(rename="PRICE_CHANGE_MODE_UNSPECIFIED")]
    PRICECHANGEMODEUNSPECIFIED,
    

    /// If the subscription price is decreasing.
    ///
    /// "PRICE_DECREASE"
    #[serde(rename="PRICE_DECREASE")]
    PRICEDECREASE,
    

    /// If the subscription price is increasing and the user needs to accept it.
    ///
    /// "PRICE_INCREASE"
    #[serde(rename="PRICE_INCREASE")]
    PRICEINCREASE,
    

    /// If the subscription price is increasing with opt out mode.
    ///
    /// "OPT_OUT_PRICE_INCREASE"
    #[serde(rename="OPT_OUT_PRICE_INCREASE")]
    OPTOUTPRICEINCREASE,
}

impl AsRef<str> for SubscriptionItemPriceChangeDetailPriceChangeModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionItemPriceChangeDetailPriceChangeModeEnum::PRICECHANGEMODEUNSPECIFIED => "PRICE_CHANGE_MODE_UNSPECIFIED",
            SubscriptionItemPriceChangeDetailPriceChangeModeEnum::PRICEDECREASE => "PRICE_DECREASE",
            SubscriptionItemPriceChangeDetailPriceChangeModeEnum::PRICEINCREASE => "PRICE_INCREASE",
            SubscriptionItemPriceChangeDetailPriceChangeModeEnum::OPTOUTPRICEINCREASE => "OPT_OUT_PRICE_INCREASE",
        }
    }
}

impl std::convert::TryFrom< &str> for SubscriptionItemPriceChangeDetailPriceChangeModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRICE_CHANGE_MODE_UNSPECIFIED" => Ok(SubscriptionItemPriceChangeDetailPriceChangeModeEnum::PRICECHANGEMODEUNSPECIFIED),
           "PRICE_DECREASE" => Ok(SubscriptionItemPriceChangeDetailPriceChangeModeEnum::PRICEDECREASE),
           "PRICE_INCREASE" => Ok(SubscriptionItemPriceChangeDetailPriceChangeModeEnum::PRICEINCREASE),
           "OPT_OUT_PRICE_INCREASE" => Ok(SubscriptionItemPriceChangeDetailPriceChangeModeEnum::OPTOUTPRICEINCREASE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionItemPriceChangeDetailPriceChangeModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SubscriptionItemPriceChangeDetailPriceChangeStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State the price change is currently in.
pub enum SubscriptionItemPriceChangeDetailPriceChangeStateEnum {
    

    /// Price change state unspecified. This value should not be used.
    ///
    /// "PRICE_CHANGE_STATE_UNSPECIFIED"
    #[serde(rename="PRICE_CHANGE_STATE_UNSPECIFIED")]
    PRICECHANGESTATEUNSPECIFIED,
    

    /// Waiting for the user to agree for the price change.
    ///
    /// "OUTSTANDING"
    #[serde(rename="OUTSTANDING")]
    OUTSTANDING,
    

    /// The price change is confirmed to happen for the user.
    ///
    /// "CONFIRMED"
    #[serde(rename="CONFIRMED")]
    CONFIRMED,
    

    /// The price change is applied, i.e. the user has started being charged the new price.
    ///
    /// "APPLIED"
    #[serde(rename="APPLIED")]
    APPLIED,
}

impl AsRef<str> for SubscriptionItemPriceChangeDetailPriceChangeStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionItemPriceChangeDetailPriceChangeStateEnum::PRICECHANGESTATEUNSPECIFIED => "PRICE_CHANGE_STATE_UNSPECIFIED",
            SubscriptionItemPriceChangeDetailPriceChangeStateEnum::OUTSTANDING => "OUTSTANDING",
            SubscriptionItemPriceChangeDetailPriceChangeStateEnum::CONFIRMED => "CONFIRMED",
            SubscriptionItemPriceChangeDetailPriceChangeStateEnum::APPLIED => "APPLIED",
        }
    }
}

impl std::convert::TryFrom< &str> for SubscriptionItemPriceChangeDetailPriceChangeStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRICE_CHANGE_STATE_UNSPECIFIED" => Ok(SubscriptionItemPriceChangeDetailPriceChangeStateEnum::PRICECHANGESTATEUNSPECIFIED),
           "OUTSTANDING" => Ok(SubscriptionItemPriceChangeDetailPriceChangeStateEnum::OUTSTANDING),
           "CONFIRMED" => Ok(SubscriptionItemPriceChangeDetailPriceChangeStateEnum::CONFIRMED),
           "APPLIED" => Ok(SubscriptionItemPriceChangeDetailPriceChangeStateEnum::APPLIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionItemPriceChangeDetailPriceChangeStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SubscriptionOfferStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of this offer. Can be changed using Activate and Deactivate actions. NB: the base plan state supersedes this state, so an active offer may not be available if the base plan is not active.
pub enum SubscriptionOfferStateEnum {
    

    /// Default value, should never be used.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The subscription offer is not and has never been available to users.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// The subscription offer is available to new and existing users.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The subscription offer is not available to new users. Existing users retain access.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
}

impl AsRef<str> for SubscriptionOfferStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionOfferStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SubscriptionOfferStateEnum::DRAFT => "DRAFT",
            SubscriptionOfferStateEnum::ACTIVE => "ACTIVE",
            SubscriptionOfferStateEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for SubscriptionOfferStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(SubscriptionOfferStateEnum::STATEUNSPECIFIED),
           "DRAFT" => Ok(SubscriptionOfferStateEnum::DRAFT),
           "ACTIVE" => Ok(SubscriptionOfferStateEnum::ACTIVE),
           "INACTIVE" => Ok(SubscriptionOfferStateEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionOfferStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SubscriptionPurchaseV2AcknowledgementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The acknowledgement state of the subscription.
pub enum SubscriptionPurchaseV2AcknowledgementStateEnum {
    

    /// Unspecified acknowledgement state.
    ///
    /// "ACKNOWLEDGEMENT_STATE_UNSPECIFIED"
    #[serde(rename="ACKNOWLEDGEMENT_STATE_UNSPECIFIED")]
    ACKNOWLEDGEMENTSTATEUNSPECIFIED,
    

    /// The subscription is not acknowledged yet.
    ///
    /// "ACKNOWLEDGEMENT_STATE_PENDING"
    #[serde(rename="ACKNOWLEDGEMENT_STATE_PENDING")]
    ACKNOWLEDGEMENTSTATEPENDING,
    

    /// The subscription is acknowledged.
    ///
    /// "ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED"
    #[serde(rename="ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED")]
    ACKNOWLEDGEMENTSTATEACKNOWLEDGED,
}

impl AsRef<str> for SubscriptionPurchaseV2AcknowledgementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionPurchaseV2AcknowledgementStateEnum::ACKNOWLEDGEMENTSTATEUNSPECIFIED => "ACKNOWLEDGEMENT_STATE_UNSPECIFIED",
            SubscriptionPurchaseV2AcknowledgementStateEnum::ACKNOWLEDGEMENTSTATEPENDING => "ACKNOWLEDGEMENT_STATE_PENDING",
            SubscriptionPurchaseV2AcknowledgementStateEnum::ACKNOWLEDGEMENTSTATEACKNOWLEDGED => "ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED",
        }
    }
}

impl std::convert::TryFrom< &str> for SubscriptionPurchaseV2AcknowledgementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACKNOWLEDGEMENT_STATE_UNSPECIFIED" => Ok(SubscriptionPurchaseV2AcknowledgementStateEnum::ACKNOWLEDGEMENTSTATEUNSPECIFIED),
           "ACKNOWLEDGEMENT_STATE_PENDING" => Ok(SubscriptionPurchaseV2AcknowledgementStateEnum::ACKNOWLEDGEMENTSTATEPENDING),
           "ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED" => Ok(SubscriptionPurchaseV2AcknowledgementStateEnum::ACKNOWLEDGEMENTSTATEACKNOWLEDGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionPurchaseV2AcknowledgementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SubscriptionPurchaseV2SubscriptionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the subscription.
pub enum SubscriptionPurchaseV2SubscriptionStateEnum {
    

    /// Unspecified subscription state.
    ///
    /// "SUBSCRIPTION_STATE_UNSPECIFIED"
    #[serde(rename="SUBSCRIPTION_STATE_UNSPECIFIED")]
    SUBSCRIPTIONSTATEUNSPECIFIED,
    

    /// Subscription was created but awaiting payment during signup. In this state, all items are awaiting payment.
    ///
    /// "SUBSCRIPTION_STATE_PENDING"
    #[serde(rename="SUBSCRIPTION_STATE_PENDING")]
    SUBSCRIPTIONSTATEPENDING,
    

    /// Subscription is active. - (1) If the subscription is an auto renewing plan, at least one item is auto_renew_enabled and not expired. - (2) If the subscription is a prepaid plan, at least one item is not expired.
    ///
    /// "SUBSCRIPTION_STATE_ACTIVE"
    #[serde(rename="SUBSCRIPTION_STATE_ACTIVE")]
    SUBSCRIPTIONSTATEACTIVE,
    

    /// Subscription is paused. The state is only available when the subscription is an auto renewing plan. In this state, all items are in paused state.
    ///
    /// "SUBSCRIPTION_STATE_PAUSED"
    #[serde(rename="SUBSCRIPTION_STATE_PAUSED")]
    SUBSCRIPTIONSTATEPAUSED,
    

    /// Subscription is in grace period. The state is only available when the subscription is an auto renewing plan. In this state, all items are in grace period.
    ///
    /// "SUBSCRIPTION_STATE_IN_GRACE_PERIOD"
    #[serde(rename="SUBSCRIPTION_STATE_IN_GRACE_PERIOD")]
    SUBSCRIPTIONSTATEINGRACEPERIOD,
    

    /// Subscription is on hold (suspended). The state is only available when the subscription is an auto renewing plan. In this state, all items are on hold.
    ///
    /// "SUBSCRIPTION_STATE_ON_HOLD"
    #[serde(rename="SUBSCRIPTION_STATE_ON_HOLD")]
    SUBSCRIPTIONSTATEONHOLD,
    

    /// Subscription is canceled but not expired yet. The state is only available when the subscription is an auto renewing plan. All items have auto_renew_enabled set to false.
    ///
    /// "SUBSCRIPTION_STATE_CANCELED"
    #[serde(rename="SUBSCRIPTION_STATE_CANCELED")]
    SUBSCRIPTIONSTATECANCELED,
    

    /// Subscription is expired. All items have expiry_time in the past.
    ///
    /// "SUBSCRIPTION_STATE_EXPIRED"
    #[serde(rename="SUBSCRIPTION_STATE_EXPIRED")]
    SUBSCRIPTIONSTATEEXPIRED,
}

impl AsRef<str> for SubscriptionPurchaseV2SubscriptionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEUNSPECIFIED => "SUBSCRIPTION_STATE_UNSPECIFIED",
            SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEPENDING => "SUBSCRIPTION_STATE_PENDING",
            SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEACTIVE => "SUBSCRIPTION_STATE_ACTIVE",
            SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEPAUSED => "SUBSCRIPTION_STATE_PAUSED",
            SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEINGRACEPERIOD => "SUBSCRIPTION_STATE_IN_GRACE_PERIOD",
            SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEONHOLD => "SUBSCRIPTION_STATE_ON_HOLD",
            SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATECANCELED => "SUBSCRIPTION_STATE_CANCELED",
            SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEEXPIRED => "SUBSCRIPTION_STATE_EXPIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for SubscriptionPurchaseV2SubscriptionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBSCRIPTION_STATE_UNSPECIFIED" => Ok(SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEUNSPECIFIED),
           "SUBSCRIPTION_STATE_PENDING" => Ok(SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEPENDING),
           "SUBSCRIPTION_STATE_ACTIVE" => Ok(SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEACTIVE),
           "SUBSCRIPTION_STATE_PAUSED" => Ok(SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEPAUSED),
           "SUBSCRIPTION_STATE_IN_GRACE_PERIOD" => Ok(SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEINGRACEPERIOD),
           "SUBSCRIPTION_STATE_ON_HOLD" => Ok(SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEONHOLD),
           "SUBSCRIPTION_STATE_CANCELED" => Ok(SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATECANCELED),
           "SUBSCRIPTION_STATE_EXPIRED" => Ok(SubscriptionPurchaseV2SubscriptionStateEnum::SUBSCRIPTIONSTATEEXPIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionPurchaseV2SubscriptionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Digital content or service classification for products distributed to users in the European Economic Area (EEA). The withdrawal regime under EEA consumer laws depends on this classification. Refer to the [Help Center article](https://support.google.com/googleplay/android-developer/answer/10463498) for more information.
pub enum SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum {
    
    /// "WITHDRAWAL_RIGHT_TYPE_UNSPECIFIED"
    #[serde(rename="WITHDRAWAL_RIGHT_TYPE_UNSPECIFIED")]
    WITHDRAWALRIGHTTYPEUNSPECIFIED,
    
    /// "WITHDRAWAL_RIGHT_DIGITAL_CONTENT"
    #[serde(rename="WITHDRAWAL_RIGHT_DIGITAL_CONTENT")]
    WITHDRAWALRIGHTDIGITALCONTENT,
    
    /// "WITHDRAWAL_RIGHT_SERVICE"
    #[serde(rename="WITHDRAWAL_RIGHT_SERVICE")]
    WITHDRAWALRIGHTSERVICE,
}

impl AsRef<str> for SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTTYPEUNSPECIFIED => "WITHDRAWAL_RIGHT_TYPE_UNSPECIFIED",
            SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTDIGITALCONTENT => "WITHDRAWAL_RIGHT_DIGITAL_CONTENT",
            SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTSERVICE => "WITHDRAWAL_RIGHT_SERVICE",
        }
    }
}

impl std::convert::TryFrom< &str> for SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WITHDRAWAL_RIGHT_TYPE_UNSPECIFIED" => Ok(SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTTYPEUNSPECIFIED),
           "WITHDRAWAL_RIGHT_DIGITAL_CONTENT" => Ok(SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTDIGITALCONTENT),
           "WITHDRAWAL_RIGHT_SERVICE" => Ok(SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum::WITHDRAWALRIGHTSERVICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TextureCompressionFormatAliasEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Alias for texture compression format.
pub enum TextureCompressionFormatAliasEnum {
    

    /// Unspecified format.
    ///
    /// "UNSPECIFIED_TEXTURE_COMPRESSION_FORMAT"
    #[serde(rename="UNSPECIFIED_TEXTURE_COMPRESSION_FORMAT")]
    UNSPECIFIEDTEXTURECOMPRESSIONFORMAT,
    

    /// ETC1_RGB8 format.
    ///
    /// "ETC1_RGB8"
    #[serde(rename="ETC1_RGB8")]
    ETC1RGB8,
    

    /// PALETTED format.
    ///
    /// "PALETTED"
    #[serde(rename="PALETTED")]
    PALETTED,
    

    /// THREE_DC format.
    ///
    /// "THREE_DC"
    #[serde(rename="THREE_DC")]
    THREEDC,
    

    /// ATC format.
    ///
    /// "ATC"
    #[serde(rename="ATC")]
    ATC,
    

    /// LATC format.
    ///
    /// "LATC"
    #[serde(rename="LATC")]
    LATC,
    

    /// DXT1 format.
    ///
    /// "DXT1"
    #[serde(rename="DXT1")]
    DXT1,
    

    /// S3TC format.
    ///
    /// "S3TC"
    #[serde(rename="S3TC")]
    S3TC,
    

    /// PVRTC format.
    ///
    /// "PVRTC"
    #[serde(rename="PVRTC")]
    PVRTC,
    

    /// ASTC format.
    ///
    /// "ASTC"
    #[serde(rename="ASTC")]
    ASTC,
    

    /// ETC2 format.
    ///
    /// "ETC2"
    #[serde(rename="ETC2")]
    ETC2,
}

impl AsRef<str> for TextureCompressionFormatAliasEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TextureCompressionFormatAliasEnum::UNSPECIFIEDTEXTURECOMPRESSIONFORMAT => "UNSPECIFIED_TEXTURE_COMPRESSION_FORMAT",
            TextureCompressionFormatAliasEnum::ETC1RGB8 => "ETC1_RGB8",
            TextureCompressionFormatAliasEnum::PALETTED => "PALETTED",
            TextureCompressionFormatAliasEnum::THREEDC => "THREE_DC",
            TextureCompressionFormatAliasEnum::ATC => "ATC",
            TextureCompressionFormatAliasEnum::LATC => "LATC",
            TextureCompressionFormatAliasEnum::DXT1 => "DXT1",
            TextureCompressionFormatAliasEnum::S3TC => "S3TC",
            TextureCompressionFormatAliasEnum::PVRTC => "PVRTC",
            TextureCompressionFormatAliasEnum::ASTC => "ASTC",
            TextureCompressionFormatAliasEnum::ETC2 => "ETC2",
        }
    }
}

impl std::convert::TryFrom< &str> for TextureCompressionFormatAliasEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_TEXTURE_COMPRESSION_FORMAT" => Ok(TextureCompressionFormatAliasEnum::UNSPECIFIEDTEXTURECOMPRESSIONFORMAT),
           "ETC1_RGB8" => Ok(TextureCompressionFormatAliasEnum::ETC1RGB8),
           "PALETTED" => Ok(TextureCompressionFormatAliasEnum::PALETTED),
           "THREE_DC" => Ok(TextureCompressionFormatAliasEnum::THREEDC),
           "ATC" => Ok(TextureCompressionFormatAliasEnum::ATC),
           "LATC" => Ok(TextureCompressionFormatAliasEnum::LATC),
           "DXT1" => Ok(TextureCompressionFormatAliasEnum::DXT1),
           "S3TC" => Ok(TextureCompressionFormatAliasEnum::S3TC),
           "PVRTC" => Ok(TextureCompressionFormatAliasEnum::PVRTC),
           "ASTC" => Ok(TextureCompressionFormatAliasEnum::ASTC),
           "ETC2" => Ok(TextureCompressionFormatAliasEnum::ETC2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TextureCompressionFormatAliasEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TrackConfigFormFactorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Form factor of the new track. Defaults to the default track.
pub enum TrackConfigFormFactorEnum {
    

    /// Fallback value, do not use.
    ///
    /// "FORM_FACTOR_UNSPECIFIED"
    #[serde(rename="FORM_FACTOR_UNSPECIFIED")]
    FORMFACTORUNSPECIFIED,
    

    /// Default track.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// Wear form factor track.
    ///
    /// "WEAR"
    #[serde(rename="WEAR")]
    WEAR,
    

    /// Automotive form factor track.
    ///
    /// "AUTOMOTIVE"
    #[serde(rename="AUTOMOTIVE")]
    AUTOMOTIVE,
}

impl AsRef<str> for TrackConfigFormFactorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TrackConfigFormFactorEnum::FORMFACTORUNSPECIFIED => "FORM_FACTOR_UNSPECIFIED",
            TrackConfigFormFactorEnum::DEFAULT => "DEFAULT",
            TrackConfigFormFactorEnum::WEAR => "WEAR",
            TrackConfigFormFactorEnum::AUTOMOTIVE => "AUTOMOTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for TrackConfigFormFactorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORM_FACTOR_UNSPECIFIED" => Ok(TrackConfigFormFactorEnum::FORMFACTORUNSPECIFIED),
           "DEFAULT" => Ok(TrackConfigFormFactorEnum::DEFAULT),
           "WEAR" => Ok(TrackConfigFormFactorEnum::WEAR),
           "AUTOMOTIVE" => Ok(TrackConfigFormFactorEnum::AUTOMOTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TrackConfigFormFactorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TrackConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Type of the new track. Currently, the only supported value is closedTesting.
pub enum TrackConfigTypeEnum {
    

    /// Fallback value, do not use.
    ///
    /// "TRACK_TYPE_UNSPECIFIED"
    #[serde(rename="TRACK_TYPE_UNSPECIFIED")]
    TRACKTYPEUNSPECIFIED,
    

    /// Closed testing track.
    ///
    /// "CLOSED_TESTING"
    #[serde(rename="CLOSED_TESTING")]
    CLOSEDTESTING,
}

impl AsRef<str> for TrackConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TrackConfigTypeEnum::TRACKTYPEUNSPECIFIED => "TRACK_TYPE_UNSPECIFIED",
            TrackConfigTypeEnum::CLOSEDTESTING => "CLOSED_TESTING",
        }
    }
}

impl std::convert::TryFrom< &str> for TrackConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRACK_TYPE_UNSPECIFIED" => Ok(TrackConfigTypeEnum::TRACKTYPEUNSPECIFIED),
           "CLOSED_TESTING" => Ok(TrackConfigTypeEnum::CLOSEDTESTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TrackConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TrackReleaseStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the release.
pub enum TrackReleaseStatusEnum {
    

    /// Unspecified status.
    ///
    /// "statusUnspecified"
    #[serde(rename="statusUnspecified")]
    StatusUnspecified,
    

    /// The release's APKs are not being served to users.
    ///
    /// "draft"
    #[serde(rename="draft")]
    Draft,
    

    /// The release's APKs are being served to a fraction of users, determined by 'user_fraction'.
    ///
    /// "inProgress"
    #[serde(rename="inProgress")]
    InProgress,
    

    /// The release's APKs will no longer be served to users. Users who already have these APKs are unaffected.
    ///
    /// "halted"
    #[serde(rename="halted")]
    Halted,
    

    /// The release will have no further changes. Its APKs are being served to all users, unless they are eligible to APKs of a more recent release.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
}

impl AsRef<str> for TrackReleaseStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TrackReleaseStatusEnum::StatusUnspecified => "statusUnspecified",
            TrackReleaseStatusEnum::Draft => "draft",
            TrackReleaseStatusEnum::InProgress => "inProgress",
            TrackReleaseStatusEnum::Halted => "halted",
            TrackReleaseStatusEnum::Completed => "completed",
        }
    }
}

impl std::convert::TryFrom< &str> for TrackReleaseStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "statusUnspecified" => Ok(TrackReleaseStatusEnum::StatusUnspecified),
           "draft" => Ok(TrackReleaseStatusEnum::Draft),
           "inProgress" => Ok(TrackReleaseStatusEnum::InProgress),
           "halted" => Ok(TrackReleaseStatusEnum::Halted),
           "completed" => Ok(TrackReleaseStatusEnum::Completed),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TrackReleaseStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UpdateSubscriptionOfferRequestLatencyToleranceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive.
pub enum UpdateSubscriptionOfferRequestLatencyToleranceEnum {
    

    /// Defaults to PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED")]
    PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED,
    

    /// The update will propagate to clients within several minutes on average and up to a few hours in rare cases. Throughput is limited to 7,200 updates per app per hour.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE,
    

    /// The update will propagate to clients within 24 hours. Supports high throughput of up to 720,000 updates per app per hour using batch modification methods.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT,
}

impl AsRef<str> for UpdateSubscriptionOfferRequestLatencyToleranceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UpdateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED => "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED",
            UpdateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE",
            UpdateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT",
        }
    }
}

impl std::convert::TryFrom< &str> for UpdateSubscriptionOfferRequestLatencyToleranceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED" => Ok(UpdateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE" => Ok(UpdateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT" => Ok(UpdateSubscriptionOfferRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UpdateSubscriptionOfferRequestLatencyToleranceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UpdateSubscriptionRequestLatencyToleranceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive.
pub enum UpdateSubscriptionRequestLatencyToleranceEnum {
    

    /// Defaults to PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED")]
    PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED,
    

    /// The update will propagate to clients within several minutes on average and up to a few hours in rare cases. Throughput is limited to 7,200 updates per app per hour.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE,
    

    /// The update will propagate to clients within 24 hours. Supports high throughput of up to 720,000 updates per app per hour using batch modification methods.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT,
}

impl AsRef<str> for UpdateSubscriptionRequestLatencyToleranceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UpdateSubscriptionRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED => "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED",
            UpdateSubscriptionRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE",
            UpdateSubscriptionRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT",
        }
    }
}

impl std::convert::TryFrom< &str> for UpdateSubscriptionRequestLatencyToleranceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED" => Ok(UpdateSubscriptionRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE" => Ok(UpdateSubscriptionRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT" => Ok(UpdateSubscriptionRequestLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UpdateSubscriptionRequestLatencyToleranceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserAccessStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the user's access to the Play Console.
pub enum UserAccessStateEnum {
    

    /// Unknown or unspecified access state.
    ///
    /// "ACCESS_STATE_UNSPECIFIED"
    #[serde(rename="ACCESS_STATE_UNSPECIFIED")]
    ACCESSSTATEUNSPECIFIED,
    

    /// User is invited but has not yet accepted the invitation.
    ///
    /// "INVITED"
    #[serde(rename="INVITED")]
    INVITED,
    

    /// Invitation has expired.
    ///
    /// "INVITATION_EXPIRED"
    #[serde(rename="INVITATION_EXPIRED")]
    INVITATIONEXPIRED,
    

    /// User has accepted an invitation and has access to the Play Console.
    ///
    /// "ACCESS_GRANTED"
    #[serde(rename="ACCESS_GRANTED")]
    ACCESSGRANTED,
    

    /// Account access has expired.
    ///
    /// "ACCESS_EXPIRED"
    #[serde(rename="ACCESS_EXPIRED")]
    ACCESSEXPIRED,
}

impl AsRef<str> for UserAccessStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserAccessStateEnum::ACCESSSTATEUNSPECIFIED => "ACCESS_STATE_UNSPECIFIED",
            UserAccessStateEnum::INVITED => "INVITED",
            UserAccessStateEnum::INVITATIONEXPIRED => "INVITATION_EXPIRED",
            UserAccessStateEnum::ACCESSGRANTED => "ACCESS_GRANTED",
            UserAccessStateEnum::ACCESSEXPIRED => "ACCESS_EXPIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for UserAccessStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_STATE_UNSPECIFIED" => Ok(UserAccessStateEnum::ACCESSSTATEUNSPECIFIED),
           "INVITED" => Ok(UserAccessStateEnum::INVITED),
           "INVITATION_EXPIRED" => Ok(UserAccessStateEnum::INVITATIONEXPIRED),
           "ACCESS_GRANTED" => Ok(UserAccessStateEnum::ACCESSGRANTED),
           "ACCESS_EXPIRED" => Ok(UserAccessStateEnum::ACCESSEXPIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserAccessStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserDeveloperAccountPermissionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Permissions for the user which apply across the developer account.
pub enum UserDeveloperAccountPermissionsEnum {
    

    /// Unknown or unspecified permission.
    ///
    /// "DEVELOPER_LEVEL_PERMISSION_UNSPECIFIED"
    #[serde(rename="DEVELOPER_LEVEL_PERMISSION_UNSPECIFIED")]
    DEVELOPERLEVELPERMISSIONUNSPECIFIED,
    

    /// View app information and download bulk reports (read-only). Deprecated: Check CAN_VIEW_NON_FINANCIAL_DATA_GLOBAL.
    ///
    /// "CAN_SEE_ALL_APPS"
    #[serde(rename="CAN_SEE_ALL_APPS")]
    CANSEEALLAPPS,
    

    /// View financial data, orders, and cancellation survey responses.
    ///
    /// "CAN_VIEW_FINANCIAL_DATA_GLOBAL"
    #[serde(rename="CAN_VIEW_FINANCIAL_DATA_GLOBAL")]
    CANVIEWFINANCIALDATAGLOBAL,
    

    /// Admin (all permissions).
    ///
    /// "CAN_MANAGE_PERMISSIONS_GLOBAL"
    #[serde(rename="CAN_MANAGE_PERMISSIONS_GLOBAL")]
    CANMANAGEPERMISSIONSGLOBAL,
    

    /// Edit Play Games Services projects.
    ///
    /// "CAN_EDIT_GAMES_GLOBAL"
    #[serde(rename="CAN_EDIT_GAMES_GLOBAL")]
    CANEDITGAMESGLOBAL,
    

    /// Publish Play Games Services projects.
    ///
    /// "CAN_PUBLISH_GAMES_GLOBAL"
    #[serde(rename="CAN_PUBLISH_GAMES_GLOBAL")]
    CANPUBLISHGAMESGLOBAL,
    

    /// Reply to reviews.
    ///
    /// "CAN_REPLY_TO_REVIEWS_GLOBAL"
    #[serde(rename="CAN_REPLY_TO_REVIEWS_GLOBAL")]
    CANREPLYTOREVIEWSGLOBAL,
    

    /// Release to production, exclude devices, and use app signing by Google Play.
    ///
    /// "CAN_MANAGE_PUBLIC_APKS_GLOBAL"
    #[serde(rename="CAN_MANAGE_PUBLIC_APKS_GLOBAL")]
    CANMANAGEPUBLICAPKSGLOBAL,
    

    /// Release to testing tracks.
    ///
    /// "CAN_MANAGE_TRACK_APKS_GLOBAL"
    #[serde(rename="CAN_MANAGE_TRACK_APKS_GLOBAL")]
    CANMANAGETRACKAPKSGLOBAL,
    

    /// Manage testing tracks and edit tester lists.
    ///
    /// "CAN_MANAGE_TRACK_USERS_GLOBAL"
    #[serde(rename="CAN_MANAGE_TRACK_USERS_GLOBAL")]
    CANMANAGETRACKUSERSGLOBAL,
    

    /// Manage store presence.
    ///
    /// "CAN_MANAGE_PUBLIC_LISTING_GLOBAL"
    #[serde(rename="CAN_MANAGE_PUBLIC_LISTING_GLOBAL")]
    CANMANAGEPUBLICLISTINGGLOBAL,
    

    /// Create, edit, and delete draft apps.
    ///
    /// "CAN_MANAGE_DRAFT_APPS_GLOBAL"
    #[serde(rename="CAN_MANAGE_DRAFT_APPS_GLOBAL")]
    CANMANAGEDRAFTAPPSGLOBAL,
    

    /// Create and publish private apps to your organization.
    ///
    /// "CAN_CREATE_MANAGED_PLAY_APPS_GLOBAL"
    #[serde(rename="CAN_CREATE_MANAGED_PLAY_APPS_GLOBAL")]
    CANCREATEMANAGEDPLAYAPPSGLOBAL,
    

    /// Choose whether apps are public, or only available to your organization.
    ///
    /// "CAN_CHANGE_MANAGED_PLAY_SETTING_GLOBAL"
    #[serde(rename="CAN_CHANGE_MANAGED_PLAY_SETTING_GLOBAL")]
    CANCHANGEMANAGEDPLAYSETTINGGLOBAL,
    

    /// Manage orders and subscriptions.
    ///
    /// "CAN_MANAGE_ORDERS_GLOBAL"
    #[serde(rename="CAN_MANAGE_ORDERS_GLOBAL")]
    CANMANAGEORDERSGLOBAL,
    

    /// Manage policy related pages on all apps for the developer.
    ///
    /// "CAN_MANAGE_APP_CONTENT_GLOBAL"
    #[serde(rename="CAN_MANAGE_APP_CONTENT_GLOBAL")]
    CANMANAGEAPPCONTENTGLOBAL,
    

    /// View app information and download bulk reports (read-only).
    ///
    /// "CAN_VIEW_NON_FINANCIAL_DATA_GLOBAL"
    #[serde(rename="CAN_VIEW_NON_FINANCIAL_DATA_GLOBAL")]
    CANVIEWNONFINANCIALDATAGLOBAL,
    

    /// View app quality information for all apps for the developer.
    ///
    /// "CAN_VIEW_APP_QUALITY_GLOBAL"
    #[serde(rename="CAN_VIEW_APP_QUALITY_GLOBAL")]
    CANVIEWAPPQUALITYGLOBAL,
    

    /// Manage the deep links setup for all apps for the developer.
    ///
    /// "CAN_MANAGE_DEEPLINKS_GLOBAL"
    #[serde(rename="CAN_MANAGE_DEEPLINKS_GLOBAL")]
    CANMANAGEDEEPLINKSGLOBAL,
}

impl AsRef<str> for UserDeveloperAccountPermissionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserDeveloperAccountPermissionsEnum::DEVELOPERLEVELPERMISSIONUNSPECIFIED => "DEVELOPER_LEVEL_PERMISSION_UNSPECIFIED",
            UserDeveloperAccountPermissionsEnum::CANSEEALLAPPS => "CAN_SEE_ALL_APPS",
            UserDeveloperAccountPermissionsEnum::CANVIEWFINANCIALDATAGLOBAL => "CAN_VIEW_FINANCIAL_DATA_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANMANAGEPERMISSIONSGLOBAL => "CAN_MANAGE_PERMISSIONS_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANEDITGAMESGLOBAL => "CAN_EDIT_GAMES_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANPUBLISHGAMESGLOBAL => "CAN_PUBLISH_GAMES_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANREPLYTOREVIEWSGLOBAL => "CAN_REPLY_TO_REVIEWS_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANMANAGEPUBLICAPKSGLOBAL => "CAN_MANAGE_PUBLIC_APKS_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANMANAGETRACKAPKSGLOBAL => "CAN_MANAGE_TRACK_APKS_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANMANAGETRACKUSERSGLOBAL => "CAN_MANAGE_TRACK_USERS_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANMANAGEPUBLICLISTINGGLOBAL => "CAN_MANAGE_PUBLIC_LISTING_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANMANAGEDRAFTAPPSGLOBAL => "CAN_MANAGE_DRAFT_APPS_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANCREATEMANAGEDPLAYAPPSGLOBAL => "CAN_CREATE_MANAGED_PLAY_APPS_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANCHANGEMANAGEDPLAYSETTINGGLOBAL => "CAN_CHANGE_MANAGED_PLAY_SETTING_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANMANAGEORDERSGLOBAL => "CAN_MANAGE_ORDERS_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANMANAGEAPPCONTENTGLOBAL => "CAN_MANAGE_APP_CONTENT_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANVIEWNONFINANCIALDATAGLOBAL => "CAN_VIEW_NON_FINANCIAL_DATA_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANVIEWAPPQUALITYGLOBAL => "CAN_VIEW_APP_QUALITY_GLOBAL",
            UserDeveloperAccountPermissionsEnum::CANMANAGEDEEPLINKSGLOBAL => "CAN_MANAGE_DEEPLINKS_GLOBAL",
        }
    }
}

impl std::convert::TryFrom< &str> for UserDeveloperAccountPermissionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVELOPER_LEVEL_PERMISSION_UNSPECIFIED" => Ok(UserDeveloperAccountPermissionsEnum::DEVELOPERLEVELPERMISSIONUNSPECIFIED),
           "CAN_SEE_ALL_APPS" => Ok(UserDeveloperAccountPermissionsEnum::CANSEEALLAPPS),
           "CAN_VIEW_FINANCIAL_DATA_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANVIEWFINANCIALDATAGLOBAL),
           "CAN_MANAGE_PERMISSIONS_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANMANAGEPERMISSIONSGLOBAL),
           "CAN_EDIT_GAMES_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANEDITGAMESGLOBAL),
           "CAN_PUBLISH_GAMES_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANPUBLISHGAMESGLOBAL),
           "CAN_REPLY_TO_REVIEWS_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANREPLYTOREVIEWSGLOBAL),
           "CAN_MANAGE_PUBLIC_APKS_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANMANAGEPUBLICAPKSGLOBAL),
           "CAN_MANAGE_TRACK_APKS_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANMANAGETRACKAPKSGLOBAL),
           "CAN_MANAGE_TRACK_USERS_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANMANAGETRACKUSERSGLOBAL),
           "CAN_MANAGE_PUBLIC_LISTING_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANMANAGEPUBLICLISTINGGLOBAL),
           "CAN_MANAGE_DRAFT_APPS_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANMANAGEDRAFTAPPSGLOBAL),
           "CAN_CREATE_MANAGED_PLAY_APPS_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANCREATEMANAGEDPLAYAPPSGLOBAL),
           "CAN_CHANGE_MANAGED_PLAY_SETTING_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANCHANGEMANAGEDPLAYSETTINGGLOBAL),
           "CAN_MANAGE_ORDERS_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANMANAGEORDERSGLOBAL),
           "CAN_MANAGE_APP_CONTENT_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANMANAGEAPPCONTENTGLOBAL),
           "CAN_VIEW_NON_FINANCIAL_DATA_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANVIEWNONFINANCIALDATAGLOBAL),
           "CAN_VIEW_APP_QUALITY_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANVIEWAPPQUALITYGLOBAL),
           "CAN_MANAGE_DEEPLINKS_GLOBAL" => Ok(UserDeveloperAccountPermissionsEnum::CANMANAGEDEEPLINKSGLOBAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserDeveloperAccountPermissionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EditDeobfuscationFileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the deobfuscation file.
pub enum EditDeobfuscationFileTypeEnum {
    

    /// Unspecified deobfuscation file type.
    ///
    /// "deobfuscationFileTypeUnspecified"
    #[serde(rename="deobfuscationFileTypeUnspecified")]
    DeobfuscationFileTypeUnspecified,
    

    /// Proguard deobfuscation file type.
    ///
    /// "proguard"
    #[serde(rename="proguard")]
    Proguard,
    

    /// Native debugging symbols file type.
    ///
    /// "nativeCode"
    #[serde(rename="nativeCode")]
    NativeCode,
}

impl AsRef<str> for EditDeobfuscationFileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EditDeobfuscationFileTypeEnum::DeobfuscationFileTypeUnspecified => "deobfuscationFileTypeUnspecified",
            EditDeobfuscationFileTypeEnum::Proguard => "proguard",
            EditDeobfuscationFileTypeEnum::NativeCode => "nativeCode",
        }
    }
}

impl std::convert::TryFrom< &str> for EditDeobfuscationFileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "deobfuscationFileTypeUnspecified" => Ok(EditDeobfuscationFileTypeEnum::DeobfuscationFileTypeUnspecified),
           "proguard" => Ok(EditDeobfuscationFileTypeEnum::Proguard),
           "nativeCode" => Ok(EditDeobfuscationFileTypeEnum::NativeCode),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EditDeobfuscationFileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EditExpansionFileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The file type of the expansion file configuration which is being updated.
pub enum EditExpansionFileTypeEnum {
    

    /// Unspecified expansion file type.
    ///
    /// "expansionFileTypeUnspecified"
    #[serde(rename="expansionFileTypeUnspecified")]
    ExpansionFileTypeUnspecified,
    

    /// Main expansion file.
    ///
    /// "main"
    #[serde(rename="main")]
    Main,
    

    /// Patch expansion file.
    ///
    /// "patch"
    #[serde(rename="patch")]
    Patch,
}

impl AsRef<str> for EditExpansionFileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EditExpansionFileTypeEnum::ExpansionFileTypeUnspecified => "expansionFileTypeUnspecified",
            EditExpansionFileTypeEnum::Main => "main",
            EditExpansionFileTypeEnum::Patch => "patch",
        }
    }
}

impl std::convert::TryFrom< &str> for EditExpansionFileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "expansionFileTypeUnspecified" => Ok(EditExpansionFileTypeEnum::ExpansionFileTypeUnspecified),
           "main" => Ok(EditExpansionFileTypeEnum::Main),
           "patch" => Ok(EditExpansionFileTypeEnum::Patch),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EditExpansionFileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EditImageTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the Image.
pub enum EditImageTypeEnum {
    

    /// Unspecified type. Do not use.
    ///
    /// "appImageTypeUnspecified"
    #[serde(rename="appImageTypeUnspecified")]
    AppImageTypeUnspecified,
    

    /// Phone screenshot.
    ///
    /// "phoneScreenshots"
    #[serde(rename="phoneScreenshots")]
    PhoneScreenshots,
    

    /// Seven inch screenshot.
    ///
    /// "sevenInchScreenshots"
    #[serde(rename="sevenInchScreenshots")]
    SevenInchScreenshots,
    

    /// Ten inch screenshot.
    ///
    /// "tenInchScreenshots"
    #[serde(rename="tenInchScreenshots")]
    TenInchScreenshots,
    

    /// TV screenshot.
    ///
    /// "tvScreenshots"
    #[serde(rename="tvScreenshots")]
    TvScreenshots,
    

    /// Wear screenshot.
    ///
    /// "wearScreenshots"
    #[serde(rename="wearScreenshots")]
    WearScreenshots,
    

    /// Icon.
    ///
    /// "icon"
    #[serde(rename="icon")]
    Icon,
    

    /// Feature graphic.
    ///
    /// "featureGraphic"
    #[serde(rename="featureGraphic")]
    FeatureGraphic,
    

    /// TV banner.
    ///
    /// "tvBanner"
    #[serde(rename="tvBanner")]
    TvBanner,
}

impl AsRef<str> for EditImageTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EditImageTypeEnum::AppImageTypeUnspecified => "appImageTypeUnspecified",
            EditImageTypeEnum::PhoneScreenshots => "phoneScreenshots",
            EditImageTypeEnum::SevenInchScreenshots => "sevenInchScreenshots",
            EditImageTypeEnum::TenInchScreenshots => "tenInchScreenshots",
            EditImageTypeEnum::TvScreenshots => "tvScreenshots",
            EditImageTypeEnum::WearScreenshots => "wearScreenshots",
            EditImageTypeEnum::Icon => "icon",
            EditImageTypeEnum::FeatureGraphic => "featureGraphic",
            EditImageTypeEnum::TvBanner => "tvBanner",
        }
    }
}

impl std::convert::TryFrom< &str> for EditImageTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "appImageTypeUnspecified" => Ok(EditImageTypeEnum::AppImageTypeUnspecified),
           "phoneScreenshots" => Ok(EditImageTypeEnum::PhoneScreenshots),
           "sevenInchScreenshots" => Ok(EditImageTypeEnum::SevenInchScreenshots),
           "tenInchScreenshots" => Ok(EditImageTypeEnum::TenInchScreenshots),
           "tvScreenshots" => Ok(EditImageTypeEnum::TvScreenshots),
           "wearScreenshots" => Ok(EditImageTypeEnum::WearScreenshots),
           "icon" => Ok(EditImageTypeEnum::Icon),
           "featureGraphic" => Ok(EditImageTypeEnum::FeatureGraphic),
           "tvBanner" => Ok(EditImageTypeEnum::TvBanner),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EditImageTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InappproductLatencyToleranceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive.
pub enum InappproductLatencyToleranceEnum {
    

    /// Defaults to PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED")]
    PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED,
    

    /// The update will propagate to clients within several minutes on average and up to a few hours in rare cases. Throughput is limited to 7,200 updates per app per hour.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE,
    

    /// The update will propagate to clients within 24 hours. Supports high throughput of up to 720,000 updates per app per hour using batch modification methods.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT,
}

impl AsRef<str> for InappproductLatencyToleranceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InappproductLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED => "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED",
            InappproductLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE",
            InappproductLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT",
        }
    }
}

impl std::convert::TryFrom< &str> for InappproductLatencyToleranceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED" => Ok(InappproductLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE" => Ok(InappproductLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT" => Ok(InappproductLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InappproductLatencyToleranceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MonetizationLatencyToleranceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive.
pub enum MonetizationLatencyToleranceEnum {
    

    /// Defaults to PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED")]
    PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED,
    

    /// The update will propagate to clients within several minutes on average and up to a few hours in rare cases. Throughput is limited to 7,200 updates per app per hour.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE,
    

    /// The update will propagate to clients within 24 hours. Supports high throughput of up to 720,000 updates per app per hour using batch modification methods.
    ///
    /// "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT"
    #[serde(rename="PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT")]
    PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT,
}

impl AsRef<str> for MonetizationLatencyToleranceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MonetizationLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED => "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED",
            MonetizationLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE",
            MonetizationLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT => "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT",
        }
    }
}

impl std::convert::TryFrom< &str> for MonetizationLatencyToleranceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_UNSPECIFIED" => Ok(MonetizationLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCEUNSPECIFIED),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_SENSITIVE" => Ok(MonetizationLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYSENSITIVE),
           "PRODUCT_UPDATE_LATENCY_TOLERANCE_LATENCY_TOLERANT" => Ok(MonetizationLatencyToleranceEnum::PRODUCTUPDATELATENCYTOLERANCELATENCYTOLERANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MonetizationLatencyToleranceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


