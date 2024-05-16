use super::*;



// region GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates whether _this binding_ provides the specified permission to the specified principal for the specified resource. This field does _not_ indicate whether the principal actually has the permission for the resource. There might be another binding that overrides this binding. To determine whether the principal actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse.
pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum {
    

    /// Default value. This value is unused.
    ///
    /// "ACCESS_STATE_UNSPECIFIED"
    #[serde(rename="ACCESS_STATE_UNSPECIFIED")]
    ACCESSSTATEUNSPECIFIED,
    

    /// The principal has the permission.
    ///
    /// "GRANTED"
    #[serde(rename="GRANTED")]
    GRANTED,
    

    /// The principal does not have the permission.
    ///
    /// "NOT_GRANTED"
    #[serde(rename="NOT_GRANTED")]
    NOTGRANTED,
    

    /// The principal has the permission only if a condition expression evaluates to `true`.
    ///
    /// "UNKNOWN_CONDITIONAL"
    #[serde(rename="UNKNOWN_CONDITIONAL")]
    UNKNOWNCONDITIONAL,
    

    /// The sender of the request does not have access to all of the policies that Policy Troubleshooter needs to evaluate.
    ///
    /// "UNKNOWN_INFO_DENIED"
    #[serde(rename="UNKNOWN_INFO_DENIED")]
    UNKNOWNINFODENIED,
}

impl AsRef<str> for GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum::ACCESSSTATEUNSPECIFIED => "ACCESS_STATE_UNSPECIFIED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum::GRANTED => "GRANTED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum::NOTGRANTED => "NOT_GRANTED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum::UNKNOWNCONDITIONAL => "UNKNOWN_CONDITIONAL",
            GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum::UNKNOWNINFODENIED => "UNKNOWN_INFO_DENIED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_STATE_UNSPECIFIED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum::ACCESSSTATEUNSPECIFIED),
           "GRANTED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum::GRANTED),
           "NOT_GRANTED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum::NOTGRANTED),
           "UNKNOWN_CONDITIONAL" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum::UNKNOWNCONDITIONAL),
           "UNKNOWN_INFO_DENIED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum::UNKNOWNINFODENIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicytroubleshooterV1BindingExplanationAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicytroubleshooterV1BindingExplanationRelevanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The relevance of this binding to the overall determination for the entire policy.
pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationRelevanceEnum {
    

    /// Default value. This value is unused.
    ///
    /// "HEURISTIC_RELEVANCE_UNSPECIFIED"
    #[serde(rename="HEURISTIC_RELEVANCE_UNSPECIFIED")]
    HEURISTICRELEVANCEUNSPECIFIED,
    

    /// The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination.
    ///
    /// "NORMAL"
    #[serde(rename="NORMAL")]
    NORMAL,
    

    /// The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
}

impl AsRef<str> for GoogleCloudPolicytroubleshooterV1BindingExplanationRelevanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicytroubleshooterV1BindingExplanationRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED => "HEURISTIC_RELEVANCE_UNSPECIFIED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationRelevanceEnum::NORMAL => "NORMAL",
            GoogleCloudPolicytroubleshooterV1BindingExplanationRelevanceEnum::HIGH => "HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicytroubleshooterV1BindingExplanationRelevanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEURISTIC_RELEVANCE_UNSPECIFIED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED),
           "NORMAL" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationRelevanceEnum::NORMAL),
           "HIGH" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationRelevanceEnum::HIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicytroubleshooterV1BindingExplanationRelevanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the role granted by this binding contains the specified permission.
pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum {
    

    /// Default value. This value is unused.
    ///
    /// "ROLE_PERMISSION_UNSPECIFIED"
    #[serde(rename="ROLE_PERMISSION_UNSPECIFIED")]
    ROLEPERMISSIONUNSPECIFIED,
    

    /// The permission is included in the role.
    ///
    /// "ROLE_PERMISSION_INCLUDED"
    #[serde(rename="ROLE_PERMISSION_INCLUDED")]
    ROLEPERMISSIONINCLUDED,
    

    /// The permission is not included in the role.
    ///
    /// "ROLE_PERMISSION_NOT_INCLUDED"
    #[serde(rename="ROLE_PERMISSION_NOT_INCLUDED")]
    ROLEPERMISSIONNOTINCLUDED,
    

    /// The sender of the request is not allowed to access the binding.
    ///
    /// "ROLE_PERMISSION_UNKNOWN_INFO_DENIED"
    #[serde(rename="ROLE_PERMISSION_UNKNOWN_INFO_DENIED")]
    ROLEPERMISSIONUNKNOWNINFODENIED,
}

impl AsRef<str> for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONUNSPECIFIED => "ROLE_PERMISSION_UNSPECIFIED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONINCLUDED => "ROLE_PERMISSION_INCLUDED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONNOTINCLUDED => "ROLE_PERMISSION_NOT_INCLUDED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONUNKNOWNINFODENIED => "ROLE_PERMISSION_UNKNOWN_INFO_DENIED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_PERMISSION_UNSPECIFIED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONUNSPECIFIED),
           "ROLE_PERMISSION_INCLUDED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONINCLUDED),
           "ROLE_PERMISSION_NOT_INCLUDED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONNOTINCLUDED),
           "ROLE_PERMISSION_UNKNOWN_INFO_DENIED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONUNKNOWNINFODENIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The relevance of the permission's existence, or nonexistence, in the role to the overall determination for the entire policy.
pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevanceEnum {
    

    /// Default value. This value is unused.
    ///
    /// "HEURISTIC_RELEVANCE_UNSPECIFIED"
    #[serde(rename="HEURISTIC_RELEVANCE_UNSPECIFIED")]
    HEURISTICRELEVANCEUNSPECIFIED,
    

    /// The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination.
    ///
    /// "NORMAL"
    #[serde(rename="NORMAL")]
    NORMAL,
    

    /// The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
}

impl AsRef<str> for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED => "HEURISTIC_RELEVANCE_UNSPECIFIED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevanceEnum::NORMAL => "NORMAL",
            GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevanceEnum::HIGH => "HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEURISTIC_RELEVANCE_UNSPECIFIED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED),
           "NORMAL" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevanceEnum::NORMAL),
           "HIGH" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevanceEnum::HIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the binding includes the principal.
pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum {
    

    /// Default value. This value is unused.
    ///
    /// "MEMBERSHIP_UNSPECIFIED"
    #[serde(rename="MEMBERSHIP_UNSPECIFIED")]
    MEMBERSHIPUNSPECIFIED,
    

    /// The binding includes the principal. The principal can be included directly or indirectly. For example: * A principal is included directly if that principal is listed in the binding. * A principal is included indirectly if that principal is in a Google group or Google Workspace domain that is listed in the binding.
    ///
    /// "MEMBERSHIP_INCLUDED"
    #[serde(rename="MEMBERSHIP_INCLUDED")]
    MEMBERSHIPINCLUDED,
    

    /// The binding does not include the principal.
    ///
    /// "MEMBERSHIP_NOT_INCLUDED"
    #[serde(rename="MEMBERSHIP_NOT_INCLUDED")]
    MEMBERSHIPNOTINCLUDED,
    

    /// The sender of the request is not allowed to access the binding.
    ///
    /// "MEMBERSHIP_UNKNOWN_INFO_DENIED"
    #[serde(rename="MEMBERSHIP_UNKNOWN_INFO_DENIED")]
    MEMBERSHIPUNKNOWNINFODENIED,
    

    /// The principal is an unsupported type. Only Google Accounts and service accounts are supported.
    ///
    /// "MEMBERSHIP_UNKNOWN_UNSUPPORTED"
    #[serde(rename="MEMBERSHIP_UNKNOWN_UNSUPPORTED")]
    MEMBERSHIPUNKNOWNUNSUPPORTED,
}

impl AsRef<str> for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNSPECIFIED => "MEMBERSHIP_UNSPECIFIED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPINCLUDED => "MEMBERSHIP_INCLUDED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPNOTINCLUDED => "MEMBERSHIP_NOT_INCLUDED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNKNOWNINFODENIED => "MEMBERSHIP_UNKNOWN_INFO_DENIED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNKNOWNUNSUPPORTED => "MEMBERSHIP_UNKNOWN_UNSUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEMBERSHIP_UNSPECIFIED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNSPECIFIED),
           "MEMBERSHIP_INCLUDED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPINCLUDED),
           "MEMBERSHIP_NOT_INCLUDED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPNOTINCLUDED),
           "MEMBERSHIP_UNKNOWN_INFO_DENIED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNKNOWNINFODENIED),
           "MEMBERSHIP_UNKNOWN_UNSUPPORTED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNKNOWNUNSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembershipEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The relevance of the principal's status to the overall determination for the binding.
pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevanceEnum {
    

    /// Default value. This value is unused.
    ///
    /// "HEURISTIC_RELEVANCE_UNSPECIFIED"
    #[serde(rename="HEURISTIC_RELEVANCE_UNSPECIFIED")]
    HEURISTICRELEVANCEUNSPECIFIED,
    

    /// The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination.
    ///
    /// "NORMAL"
    #[serde(rename="NORMAL")]
    NORMAL,
    

    /// The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
}

impl AsRef<str> for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED => "HEURISTIC_RELEVANCE_UNSPECIFIED",
            GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevanceEnum::NORMAL => "NORMAL",
            GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevanceEnum::HIGH => "HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEURISTIC_RELEVANCE_UNSPECIFIED" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED),
           "NORMAL" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevanceEnum::NORMAL),
           "HIGH" => Ok(GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevanceEnum::HIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether _this policy_ provides the specified permission to the specified principal for the specified resource. This field does _not_ indicate whether the principal actually has the permission for the resource. There might be another policy that overrides this policy. To determine whether the principal actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse.
pub enum GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum {
    

    /// Default value. This value is unused.
    ///
    /// "ACCESS_STATE_UNSPECIFIED"
    #[serde(rename="ACCESS_STATE_UNSPECIFIED")]
    ACCESSSTATEUNSPECIFIED,
    

    /// The principal has the permission.
    ///
    /// "GRANTED"
    #[serde(rename="GRANTED")]
    GRANTED,
    

    /// The principal does not have the permission.
    ///
    /// "NOT_GRANTED"
    #[serde(rename="NOT_GRANTED")]
    NOTGRANTED,
    

    /// The principal has the permission only if a condition expression evaluates to `true`.
    ///
    /// "UNKNOWN_CONDITIONAL"
    #[serde(rename="UNKNOWN_CONDITIONAL")]
    UNKNOWNCONDITIONAL,
    

    /// The sender of the request does not have access to all of the policies that Policy Troubleshooter needs to evaluate.
    ///
    /// "UNKNOWN_INFO_DENIED"
    #[serde(rename="UNKNOWN_INFO_DENIED")]
    UNKNOWNINFODENIED,
}

impl AsRef<str> for GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum::ACCESSSTATEUNSPECIFIED => "ACCESS_STATE_UNSPECIFIED",
            GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum::GRANTED => "GRANTED",
            GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum::NOTGRANTED => "NOT_GRANTED",
            GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum::UNKNOWNCONDITIONAL => "UNKNOWN_CONDITIONAL",
            GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum::UNKNOWNINFODENIED => "UNKNOWN_INFO_DENIED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_STATE_UNSPECIFIED" => Ok(GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum::ACCESSSTATEUNSPECIFIED),
           "GRANTED" => Ok(GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum::GRANTED),
           "NOT_GRANTED" => Ok(GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum::NOTGRANTED),
           "UNKNOWN_CONDITIONAL" => Ok(GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum::UNKNOWNCONDITIONAL),
           "UNKNOWN_INFO_DENIED" => Ok(GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum::UNKNOWNINFODENIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The relevance of this policy to the overall determination in the TroubleshootIamPolicyResponse. If the sender of the request does not have access to the policy, this field is omitted.
pub enum GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevanceEnum {
    

    /// Default value. This value is unused.
    ///
    /// "HEURISTIC_RELEVANCE_UNSPECIFIED"
    #[serde(rename="HEURISTIC_RELEVANCE_UNSPECIFIED")]
    HEURISTICRELEVANCEUNSPECIFIED,
    

    /// The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination.
    ///
    /// "NORMAL"
    #[serde(rename="NORMAL")]
    NORMAL,
    

    /// The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
}

impl AsRef<str> for GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED => "HEURISTIC_RELEVANCE_UNSPECIFIED",
            GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevanceEnum::NORMAL => "NORMAL",
            GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevanceEnum::HIGH => "HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEURISTIC_RELEVANCE_UNSPECIFIED" => Ok(GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED),
           "NORMAL" => Ok(GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevanceEnum::NORMAL),
           "HIGH" => Ok(GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevanceEnum::HIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the principal has the specified permission for the specified resource, based on evaluating all of the applicable IAM policies.
pub enum GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum {
    

    /// Default value. This value is unused.
    ///
    /// "ACCESS_STATE_UNSPECIFIED"
    #[serde(rename="ACCESS_STATE_UNSPECIFIED")]
    ACCESSSTATEUNSPECIFIED,
    

    /// The principal has the permission.
    ///
    /// "GRANTED"
    #[serde(rename="GRANTED")]
    GRANTED,
    

    /// The principal does not have the permission.
    ///
    /// "NOT_GRANTED"
    #[serde(rename="NOT_GRANTED")]
    NOTGRANTED,
    

    /// The principal has the permission only if a condition expression evaluates to `true`.
    ///
    /// "UNKNOWN_CONDITIONAL"
    #[serde(rename="UNKNOWN_CONDITIONAL")]
    UNKNOWNCONDITIONAL,
    

    /// The sender of the request does not have access to all of the policies that Policy Troubleshooter needs to evaluate.
    ///
    /// "UNKNOWN_INFO_DENIED"
    #[serde(rename="UNKNOWN_INFO_DENIED")]
    UNKNOWNINFODENIED,
}

impl AsRef<str> for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum::ACCESSSTATEUNSPECIFIED => "ACCESS_STATE_UNSPECIFIED",
            GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum::GRANTED => "GRANTED",
            GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum::NOTGRANTED => "NOT_GRANTED",
            GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum::UNKNOWNCONDITIONAL => "UNKNOWN_CONDITIONAL",
            GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum::UNKNOWNINFODENIED => "UNKNOWN_INFO_DENIED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_STATE_UNSPECIFIED" => Ok(GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum::ACCESSSTATEUNSPECIFIED),
           "GRANTED" => Ok(GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum::GRANTED),
           "NOT_GRANTED" => Ok(GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum::NOTGRANTED),
           "UNKNOWN_CONDITIONAL" => Ok(GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum::UNKNOWNCONDITIONAL),
           "UNKNOWN_INFO_DENIED" => Ok(GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum::UNKNOWNINFODENIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccessEnum {
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


