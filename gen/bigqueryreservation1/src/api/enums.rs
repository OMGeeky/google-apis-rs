use super::*;



// region AssignmentJobTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Which type of jobs will use the reservation.
pub enum AssignmentJobTypeEnum {
    

    /// Invalid type. Requests with this value will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`.
    ///
    /// "JOB_TYPE_UNSPECIFIED"
    #[serde(rename="JOB_TYPE_UNSPECIFIED")]
    JOBTYPEUNSPECIFIED,
    

    /// Pipeline (load/export) jobs from the project will use the reservation.
    ///
    /// "PIPELINE"
    #[serde(rename="PIPELINE")]
    PIPELINE,
    

    /// Query jobs from the project will use the reservation.
    ///
    /// "QUERY"
    #[serde(rename="QUERY")]
    QUERY,
    

    /// BigQuery ML jobs that use services external to BigQuery for model training. These jobs will not utilize idle slots from other reservations.
    ///
    /// "ML_EXTERNAL"
    #[serde(rename="ML_EXTERNAL")]
    MLEXTERNAL,
    

    /// Background jobs that BigQuery runs for the customers in the background.
    ///
    /// "BACKGROUND"
    #[serde(rename="BACKGROUND")]
    BACKGROUND,
    

    /// Continuous SQL jobs will use this reservation. Reservations with continuous assignments cannot be mixed with non-continuous assignments.
    ///
    /// "CONTINUOUS"
    #[serde(rename="CONTINUOUS")]
    CONTINUOUS,
}

impl AsRef<str> for AssignmentJobTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssignmentJobTypeEnum::JOBTYPEUNSPECIFIED => "JOB_TYPE_UNSPECIFIED",
            AssignmentJobTypeEnum::PIPELINE => "PIPELINE",
            AssignmentJobTypeEnum::QUERY => "QUERY",
            AssignmentJobTypeEnum::MLEXTERNAL => "ML_EXTERNAL",
            AssignmentJobTypeEnum::BACKGROUND => "BACKGROUND",
            AssignmentJobTypeEnum::CONTINUOUS => "CONTINUOUS",
        }
    }
}

impl std::convert::TryFrom< &str> for AssignmentJobTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JOB_TYPE_UNSPECIFIED" => Ok(AssignmentJobTypeEnum::JOBTYPEUNSPECIFIED),
           "PIPELINE" => Ok(AssignmentJobTypeEnum::PIPELINE),
           "QUERY" => Ok(AssignmentJobTypeEnum::QUERY),
           "ML_EXTERNAL" => Ok(AssignmentJobTypeEnum::MLEXTERNAL),
           "BACKGROUND" => Ok(AssignmentJobTypeEnum::BACKGROUND),
           "CONTINUOUS" => Ok(AssignmentJobTypeEnum::CONTINUOUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssignmentJobTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AssignmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the assignment.
pub enum AssignmentStateEnum {
    

    /// Invalid state value.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Queries from assignee will be executed as on-demand, if related assignment is pending.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Assignment is ready.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for AssignmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssignmentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AssignmentStateEnum::PENDING => "PENDING",
            AssignmentStateEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for AssignmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AssignmentStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(AssignmentStateEnum::PENDING),
           "ACTIVE" => Ok(AssignmentStateEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssignmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CapacityCommitmentEditionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Edition of the capacity commitment.
pub enum CapacityCommitmentEditionEnum {
    

    /// Default value, which will be treated as ENTERPRISE.
    ///
    /// "EDITION_UNSPECIFIED"
    #[serde(rename="EDITION_UNSPECIFIED")]
    EDITIONUNSPECIFIED,
    

    /// Standard edition.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// Enterprise edition.
    ///
    /// "ENTERPRISE"
    #[serde(rename="ENTERPRISE")]
    ENTERPRISE,
    

    /// Enterprise plus edition.
    ///
    /// "ENTERPRISE_PLUS"
    #[serde(rename="ENTERPRISE_PLUS")]
    ENTERPRISEPLUS,
}

impl AsRef<str> for CapacityCommitmentEditionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CapacityCommitmentEditionEnum::EDITIONUNSPECIFIED => "EDITION_UNSPECIFIED",
            CapacityCommitmentEditionEnum::STANDARD => "STANDARD",
            CapacityCommitmentEditionEnum::ENTERPRISE => "ENTERPRISE",
            CapacityCommitmentEditionEnum::ENTERPRISEPLUS => "ENTERPRISE_PLUS",
        }
    }
}

impl std::convert::TryFrom< &str> for CapacityCommitmentEditionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EDITION_UNSPECIFIED" => Ok(CapacityCommitmentEditionEnum::EDITIONUNSPECIFIED),
           "STANDARD" => Ok(CapacityCommitmentEditionEnum::STANDARD),
           "ENTERPRISE" => Ok(CapacityCommitmentEditionEnum::ENTERPRISE),
           "ENTERPRISE_PLUS" => Ok(CapacityCommitmentEditionEnum::ENTERPRISEPLUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CapacityCommitmentEditionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CapacityCommitmentPlanEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Capacity commitment commitment plan.
pub enum CapacityCommitmentPlanEnum {
    

    /// Invalid plan value. Requests with this value will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`.
    ///
    /// "COMMITMENT_PLAN_UNSPECIFIED"
    #[serde(rename="COMMITMENT_PLAN_UNSPECIFIED")]
    COMMITMENTPLANUNSPECIFIED,
    

    /// Flex commitments have committed period of 1 minute after becoming ACTIVE. After that, they are not in a committed period anymore and can be removed any time.
    ///
    /// "FLEX"
    #[serde(rename="FLEX")]
    FLEX,
    

    /// Same as FLEX, should only be used if flat-rate commitments are still available.
    ///
    /// "FLEX_FLAT_RATE"
    #[serde(rename="FLEX_FLAT_RATE")]
    FLEXFLATRATE,
    

    /// Trial commitments have a committed period of 182 days after becoming ACTIVE. After that, they are converted to a new commitment based on the `renewal_plan`. Default `renewal_plan` for Trial commitment is Flex so that it can be deleted right after committed period ends.
    ///
    /// "TRIAL"
    #[serde(rename="TRIAL")]
    TRIAL,
    

    /// Monthly commitments have a committed period of 30 days after becoming ACTIVE. After that, they are not in a committed period anymore and can be removed any time.
    ///
    /// "MONTHLY"
    #[serde(rename="MONTHLY")]
    MONTHLY,
    

    /// Same as MONTHLY, should only be used if flat-rate commitments are still available.
    ///
    /// "MONTHLY_FLAT_RATE"
    #[serde(rename="MONTHLY_FLAT_RATE")]
    MONTHLYFLATRATE,
    

    /// Annual commitments have a committed period of 365 days after becoming ACTIVE. After that they are converted to a new commitment based on the renewal_plan.
    ///
    /// "ANNUAL"
    #[serde(rename="ANNUAL")]
    ANNUAL,
    

    /// Same as ANNUAL, should only be used if flat-rate commitments are still available.
    ///
    /// "ANNUAL_FLAT_RATE"
    #[serde(rename="ANNUAL_FLAT_RATE")]
    ANNUALFLATRATE,
    

    /// 3-year commitments have a committed period of 1095(3 * 365) days after becoming ACTIVE. After that they are converted to a new commitment based on the renewal_plan.
    ///
    /// "THREE_YEAR"
    #[serde(rename="THREE_YEAR")]
    THREEYEAR,
    

    /// Should only be used for `renewal_plan` and is only meaningful if edition is specified to values other than EDITION_UNSPECIFIED. Otherwise CreateCapacityCommitmentRequest or UpdateCapacityCommitmentRequest will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. If the renewal_plan is NONE, capacity commitment will be removed at the end of its commitment period.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
}

impl AsRef<str> for CapacityCommitmentPlanEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CapacityCommitmentPlanEnum::COMMITMENTPLANUNSPECIFIED => "COMMITMENT_PLAN_UNSPECIFIED",
            CapacityCommitmentPlanEnum::FLEX => "FLEX",
            CapacityCommitmentPlanEnum::FLEXFLATRATE => "FLEX_FLAT_RATE",
            CapacityCommitmentPlanEnum::TRIAL => "TRIAL",
            CapacityCommitmentPlanEnum::MONTHLY => "MONTHLY",
            CapacityCommitmentPlanEnum::MONTHLYFLATRATE => "MONTHLY_FLAT_RATE",
            CapacityCommitmentPlanEnum::ANNUAL => "ANNUAL",
            CapacityCommitmentPlanEnum::ANNUALFLATRATE => "ANNUAL_FLAT_RATE",
            CapacityCommitmentPlanEnum::THREEYEAR => "THREE_YEAR",
            CapacityCommitmentPlanEnum::NONE => "NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for CapacityCommitmentPlanEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMMITMENT_PLAN_UNSPECIFIED" => Ok(CapacityCommitmentPlanEnum::COMMITMENTPLANUNSPECIFIED),
           "FLEX" => Ok(CapacityCommitmentPlanEnum::FLEX),
           "FLEX_FLAT_RATE" => Ok(CapacityCommitmentPlanEnum::FLEXFLATRATE),
           "TRIAL" => Ok(CapacityCommitmentPlanEnum::TRIAL),
           "MONTHLY" => Ok(CapacityCommitmentPlanEnum::MONTHLY),
           "MONTHLY_FLAT_RATE" => Ok(CapacityCommitmentPlanEnum::MONTHLYFLATRATE),
           "ANNUAL" => Ok(CapacityCommitmentPlanEnum::ANNUAL),
           "ANNUAL_FLAT_RATE" => Ok(CapacityCommitmentPlanEnum::ANNUALFLATRATE),
           "THREE_YEAR" => Ok(CapacityCommitmentPlanEnum::THREEYEAR),
           "NONE" => Ok(CapacityCommitmentPlanEnum::NONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CapacityCommitmentPlanEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CapacityCommitmentRenewalPlanEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The plan this capacity commitment is converted to after commitment_end_time passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for ANNUAL and TRIAL commitments.
pub enum CapacityCommitmentRenewalPlanEnum {
    

    /// Invalid plan value. Requests with this value will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`.
    ///
    /// "COMMITMENT_PLAN_UNSPECIFIED"
    #[serde(rename="COMMITMENT_PLAN_UNSPECIFIED")]
    COMMITMENTPLANUNSPECIFIED,
    

    /// Flex commitments have committed period of 1 minute after becoming ACTIVE. After that, they are not in a committed period anymore and can be removed any time.
    ///
    /// "FLEX"
    #[serde(rename="FLEX")]
    FLEX,
    

    /// Same as FLEX, should only be used if flat-rate commitments are still available.
    ///
    /// "FLEX_FLAT_RATE"
    #[serde(rename="FLEX_FLAT_RATE")]
    FLEXFLATRATE,
    

    /// Trial commitments have a committed period of 182 days after becoming ACTIVE. After that, they are converted to a new commitment based on the `renewal_plan`. Default `renewal_plan` for Trial commitment is Flex so that it can be deleted right after committed period ends.
    ///
    /// "TRIAL"
    #[serde(rename="TRIAL")]
    TRIAL,
    

    /// Monthly commitments have a committed period of 30 days after becoming ACTIVE. After that, they are not in a committed period anymore and can be removed any time.
    ///
    /// "MONTHLY"
    #[serde(rename="MONTHLY")]
    MONTHLY,
    

    /// Same as MONTHLY, should only be used if flat-rate commitments are still available.
    ///
    /// "MONTHLY_FLAT_RATE"
    #[serde(rename="MONTHLY_FLAT_RATE")]
    MONTHLYFLATRATE,
    

    /// Annual commitments have a committed period of 365 days after becoming ACTIVE. After that they are converted to a new commitment based on the renewal_plan.
    ///
    /// "ANNUAL"
    #[serde(rename="ANNUAL")]
    ANNUAL,
    

    /// Same as ANNUAL, should only be used if flat-rate commitments are still available.
    ///
    /// "ANNUAL_FLAT_RATE"
    #[serde(rename="ANNUAL_FLAT_RATE")]
    ANNUALFLATRATE,
    

    /// 3-year commitments have a committed period of 1095(3 * 365) days after becoming ACTIVE. After that they are converted to a new commitment based on the renewal_plan.
    ///
    /// "THREE_YEAR"
    #[serde(rename="THREE_YEAR")]
    THREEYEAR,
    

    /// Should only be used for `renewal_plan` and is only meaningful if edition is specified to values other than EDITION_UNSPECIFIED. Otherwise CreateCapacityCommitmentRequest or UpdateCapacityCommitmentRequest will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. If the renewal_plan is NONE, capacity commitment will be removed at the end of its commitment period.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
}

impl AsRef<str> for CapacityCommitmentRenewalPlanEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CapacityCommitmentRenewalPlanEnum::COMMITMENTPLANUNSPECIFIED => "COMMITMENT_PLAN_UNSPECIFIED",
            CapacityCommitmentRenewalPlanEnum::FLEX => "FLEX",
            CapacityCommitmentRenewalPlanEnum::FLEXFLATRATE => "FLEX_FLAT_RATE",
            CapacityCommitmentRenewalPlanEnum::TRIAL => "TRIAL",
            CapacityCommitmentRenewalPlanEnum::MONTHLY => "MONTHLY",
            CapacityCommitmentRenewalPlanEnum::MONTHLYFLATRATE => "MONTHLY_FLAT_RATE",
            CapacityCommitmentRenewalPlanEnum::ANNUAL => "ANNUAL",
            CapacityCommitmentRenewalPlanEnum::ANNUALFLATRATE => "ANNUAL_FLAT_RATE",
            CapacityCommitmentRenewalPlanEnum::THREEYEAR => "THREE_YEAR",
            CapacityCommitmentRenewalPlanEnum::NONE => "NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for CapacityCommitmentRenewalPlanEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMMITMENT_PLAN_UNSPECIFIED" => Ok(CapacityCommitmentRenewalPlanEnum::COMMITMENTPLANUNSPECIFIED),
           "FLEX" => Ok(CapacityCommitmentRenewalPlanEnum::FLEX),
           "FLEX_FLAT_RATE" => Ok(CapacityCommitmentRenewalPlanEnum::FLEXFLATRATE),
           "TRIAL" => Ok(CapacityCommitmentRenewalPlanEnum::TRIAL),
           "MONTHLY" => Ok(CapacityCommitmentRenewalPlanEnum::MONTHLY),
           "MONTHLY_FLAT_RATE" => Ok(CapacityCommitmentRenewalPlanEnum::MONTHLYFLATRATE),
           "ANNUAL" => Ok(CapacityCommitmentRenewalPlanEnum::ANNUAL),
           "ANNUAL_FLAT_RATE" => Ok(CapacityCommitmentRenewalPlanEnum::ANNUALFLATRATE),
           "THREE_YEAR" => Ok(CapacityCommitmentRenewalPlanEnum::THREEYEAR),
           "NONE" => Ok(CapacityCommitmentRenewalPlanEnum::NONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CapacityCommitmentRenewalPlanEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CapacityCommitmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the commitment.
pub enum CapacityCommitmentStateEnum {
    

    /// Invalid state value.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Capacity commitment is pending provisioning. Pending capacity commitment does not contribute to the project's slot_capacity.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Once slots are provisioned, capacity commitment becomes active. slot_count is added to the project's slot_capacity.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Capacity commitment is failed to be activated by the backend.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for CapacityCommitmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CapacityCommitmentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CapacityCommitmentStateEnum::PENDING => "PENDING",
            CapacityCommitmentStateEnum::ACTIVE => "ACTIVE",
            CapacityCommitmentStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for CapacityCommitmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CapacityCommitmentStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(CapacityCommitmentStateEnum::PENDING),
           "ACTIVE" => Ok(CapacityCommitmentStateEnum::ACTIVE),
           "FAILED" => Ok(CapacityCommitmentStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CapacityCommitmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReservationEditionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Edition of the reservation.
pub enum ReservationEditionEnum {
    

    /// Default value, which will be treated as ENTERPRISE.
    ///
    /// "EDITION_UNSPECIFIED"
    #[serde(rename="EDITION_UNSPECIFIED")]
    EDITIONUNSPECIFIED,
    

    /// Standard edition.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// Enterprise edition.
    ///
    /// "ENTERPRISE"
    #[serde(rename="ENTERPRISE")]
    ENTERPRISE,
    

    /// Enterprise plus edition.
    ///
    /// "ENTERPRISE_PLUS"
    #[serde(rename="ENTERPRISE_PLUS")]
    ENTERPRISEPLUS,
}

impl AsRef<str> for ReservationEditionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReservationEditionEnum::EDITIONUNSPECIFIED => "EDITION_UNSPECIFIED",
            ReservationEditionEnum::STANDARD => "STANDARD",
            ReservationEditionEnum::ENTERPRISE => "ENTERPRISE",
            ReservationEditionEnum::ENTERPRISEPLUS => "ENTERPRISE_PLUS",
        }
    }
}

impl std::convert::TryFrom< &str> for ReservationEditionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EDITION_UNSPECIFIED" => Ok(ReservationEditionEnum::EDITIONUNSPECIFIED),
           "STANDARD" => Ok(ReservationEditionEnum::STANDARD),
           "ENTERPRISE" => Ok(ReservationEditionEnum::ENTERPRISE),
           "ENTERPRISE_PLUS" => Ok(ReservationEditionEnum::ENTERPRISEPLUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReservationEditionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


