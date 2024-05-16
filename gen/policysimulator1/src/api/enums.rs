use super::*;



// region GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allow or deny type.
pub enum GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum {
    

    /// Unspecified. Results in an error.
    ///
    /// "ACTION_TYPE_UNSPECIFIED"
    #[serde(rename="ACTION_TYPE_UNSPECIFIED")]
    ACTIONTYPEUNSPECIFIED,
    

    /// Allowed action type.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// Deny action type.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum::ACTIONTYPEUNSPECIFIED => "ACTION_TYPE_UNSPECIFIED",
            GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum::ALLOW => "ALLOW",
            GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum::ACTIONTYPEUNSPECIFIED),
           "ALLOW" => Ok(GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum::ALLOW),
           "DENY" => Ok(GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// All the operations being applied for this constraint.
pub enum GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum {
    

    /// Unspecified. Results in an error.
    ///
    /// "METHOD_TYPE_UNSPECIFIED"
    #[serde(rename="METHOD_TYPE_UNSPECIFIED")]
    METHODTYPEUNSPECIFIED,
    

    /// Constraint applied when creating the resource.
    ///
    /// "CREATE"
    #[serde(rename="CREATE")]
    CREATE,
    

    /// Constraint applied when updating the resource.
    ///
    /// "UPDATE"
    #[serde(rename="UPDATE")]
    UPDATE,
    

    /// Constraint applied when deleting the resource. Not supported yet.
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
}

impl AsRef<str> for GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum::METHODTYPEUNSPECIFIED => "METHOD_TYPE_UNSPECIFIED",
            GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum::CREATE => "CREATE",
            GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum::UPDATE => "UPDATE",
            GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum::DELETE => "DELETE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METHOD_TYPE_UNSPECIFIED" => Ok(GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum::METHODTYPEUNSPECIFIED),
           "CREATE" => Ok(GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum::CREATE),
           "UPDATE" => Ok(GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum::UPDATE),
           "DELETE" => Ok(GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum::DELETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the principal's access, specified in the AccessState field, changed between the current (baseline) policies and proposed (simulated) policies.
pub enum GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum {
    

    /// Default value. This value is unused.
    ///
    /// "ACCESS_CHANGE_TYPE_UNSPECIFIED"
    #[serde(rename="ACCESS_CHANGE_TYPE_UNSPECIFIED")]
    ACCESSCHANGETYPEUNSPECIFIED,
    

    /// The principal's access did not change. This includes the case where both baseline and simulated are UNKNOWN, but the unknown information is equivalent.
    ///
    /// "NO_CHANGE"
    #[serde(rename="NO_CHANGE")]
    NOCHANGE,
    

    /// The principal's access under both the current policies and the proposed policies is `UNKNOWN`, but the unknown information differs between them.
    ///
    /// "UNKNOWN_CHANGE"
    #[serde(rename="UNKNOWN_CHANGE")]
    UNKNOWNCHANGE,
    

    /// The principal had access under the current policies (`GRANTED`), but will no longer have access after the proposed changes (`NOT_GRANTED`).
    ///
    /// "ACCESS_REVOKED"
    #[serde(rename="ACCESS_REVOKED")]
    ACCESSREVOKED,
    

    /// The principal did not have access under the current policies (`NOT_GRANTED`), but will have access after the proposed changes (`GRANTED`).
    ///
    /// "ACCESS_GAINED"
    #[serde(rename="ACCESS_GAINED")]
    ACCESSGAINED,
    

    /// This result can occur for the following reasons: * The principal had access under the current policies (`GRANTED`), but their access after the proposed changes is `UNKNOWN`. * The principal's access under the current policies is `UNKNOWN`, but they will not have access after the proposed changes (`NOT_GRANTED`).
    ///
    /// "ACCESS_MAYBE_REVOKED"
    #[serde(rename="ACCESS_MAYBE_REVOKED")]
    ACCESSMAYBEREVOKED,
    

    /// This result can occur for the following reasons: * The principal did not have access under the current policies (`NOT_GRANTED`), but their access after the proposed changes is `UNKNOWN`. * The principal's access under the current policies is `UNKNOWN`, but they will have access after the proposed changes (`GRANTED`).
    ///
    /// "ACCESS_MAYBE_GAINED"
    #[serde(rename="ACCESS_MAYBE_GAINED")]
    ACCESSMAYBEGAINED,
}

impl AsRef<str> for GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::ACCESSCHANGETYPEUNSPECIFIED => "ACCESS_CHANGE_TYPE_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::NOCHANGE => "NO_CHANGE",
            GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::UNKNOWNCHANGE => "UNKNOWN_CHANGE",
            GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::ACCESSREVOKED => "ACCESS_REVOKED",
            GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::ACCESSGAINED => "ACCESS_GAINED",
            GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::ACCESSMAYBEREVOKED => "ACCESS_MAYBE_REVOKED",
            GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::ACCESSMAYBEGAINED => "ACCESS_MAYBE_GAINED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_CHANGE_TYPE_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::ACCESSCHANGETYPEUNSPECIFIED),
           "NO_CHANGE" => Ok(GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::NOCHANGE),
           "UNKNOWN_CHANGE" => Ok(GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::UNKNOWNCHANGE),
           "ACCESS_REVOKED" => Ok(GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::ACCESSREVOKED),
           "ACCESS_GAINED" => Ok(GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::ACCESSGAINED),
           "ACCESS_MAYBE_REVOKED" => Ok(GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::ACCESSMAYBEREVOKED),
           "ACCESS_MAYBE_GAINED" => Ok(GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum::ACCESSMAYBEGAINED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates whether _this binding_ provides the specified permission to the specified principal for the specified resource. This field does _not_ indicate whether the principal actually has the permission for the resource. There might be another binding that overrides this binding. To determine whether the principal actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse.
pub enum GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum {
    

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
    

    /// The user who created the Replay does not have access to all of the policies that Policy Simulator needs to evaluate.
    ///
    /// "UNKNOWN_INFO_DENIED"
    #[serde(rename="UNKNOWN_INFO_DENIED")]
    UNKNOWNINFODENIED,
}

impl AsRef<str> for GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum::ACCESSSTATEUNSPECIFIED => "ACCESS_STATE_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum::GRANTED => "GRANTED",
            GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum::NOTGRANTED => "NOT_GRANTED",
            GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum::UNKNOWNCONDITIONAL => "UNKNOWN_CONDITIONAL",
            GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum::UNKNOWNINFODENIED => "UNKNOWN_INFO_DENIED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_STATE_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum::ACCESSSTATEUNSPECIFIED),
           "GRANTED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum::GRANTED),
           "NOT_GRANTED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum::NOTGRANTED),
           "UNKNOWN_CONDITIONAL" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum::UNKNOWNCONDITIONAL),
           "UNKNOWN_INFO_DENIED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum::UNKNOWNINFODENIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The relevance of this binding to the overall determination for the entire policy.
pub enum GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum {
    

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

impl AsRef<str> for GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED => "HEURISTIC_RELEVANCE_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum::NORMAL => "NORMAL",
            GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum::HIGH => "HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEURISTIC_RELEVANCE_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED),
           "NORMAL" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum::NORMAL),
           "HIGH" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum::HIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the role granted by this binding contains the specified permission.
pub enum GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum {
    

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
    

    /// The user who created the Replay is not allowed to access the binding.
    ///
    /// "ROLE_PERMISSION_UNKNOWN_INFO_DENIED"
    #[serde(rename="ROLE_PERMISSION_UNKNOWN_INFO_DENIED")]
    ROLEPERMISSIONUNKNOWNINFODENIED,
}

impl AsRef<str> for GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONUNSPECIFIED => "ROLE_PERMISSION_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONINCLUDED => "ROLE_PERMISSION_INCLUDED",
            GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONNOTINCLUDED => "ROLE_PERMISSION_NOT_INCLUDED",
            GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONUNKNOWNINFODENIED => "ROLE_PERMISSION_UNKNOWN_INFO_DENIED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_PERMISSION_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONUNSPECIFIED),
           "ROLE_PERMISSION_INCLUDED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONINCLUDED),
           "ROLE_PERMISSION_NOT_INCLUDED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONNOTINCLUDED),
           "ROLE_PERMISSION_UNKNOWN_INFO_DENIED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum::ROLEPERMISSIONUNKNOWNINFODENIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The relevance of the permission's existence, or nonexistence, in the role to the overall determination for the entire policy.
pub enum GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum {
    

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

impl AsRef<str> for GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED => "HEURISTIC_RELEVANCE_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum::NORMAL => "NORMAL",
            GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum::HIGH => "HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEURISTIC_RELEVANCE_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED),
           "NORMAL" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum::NORMAL),
           "HIGH" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum::HIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the binding includes the principal.
pub enum GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum {
    

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
    

    /// The user who created the Replay is not allowed to access the binding.
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

impl AsRef<str> for GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNSPECIFIED => "MEMBERSHIP_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPINCLUDED => "MEMBERSHIP_INCLUDED",
            GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPNOTINCLUDED => "MEMBERSHIP_NOT_INCLUDED",
            GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNKNOWNINFODENIED => "MEMBERSHIP_UNKNOWN_INFO_DENIED",
            GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNKNOWNUNSUPPORTED => "MEMBERSHIP_UNKNOWN_UNSUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEMBERSHIP_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNSPECIFIED),
           "MEMBERSHIP_INCLUDED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPINCLUDED),
           "MEMBERSHIP_NOT_INCLUDED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPNOTINCLUDED),
           "MEMBERSHIP_UNKNOWN_INFO_DENIED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNKNOWNINFODENIED),
           "MEMBERSHIP_UNKNOWN_UNSUPPORTED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum::MEMBERSHIPUNKNOWNUNSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The relevance of the principal's status to the overall determination for the binding.
pub enum GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum {
    

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

impl AsRef<str> for GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED => "HEURISTIC_RELEVANCE_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum::NORMAL => "NORMAL",
            GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum::HIGH => "HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEURISTIC_RELEVANCE_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED),
           "NORMAL" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum::NORMAL),
           "HIGH" => Ok(GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum::HIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the principal in the access tuple has permission to access the resource in the access tuple under the given policies.
pub enum GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum {
    

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
    

    /// The user who created the Replay does not have access to all of the policies that Policy Simulator needs to evaluate.
    ///
    /// "UNKNOWN_INFO_DENIED"
    #[serde(rename="UNKNOWN_INFO_DENIED")]
    UNKNOWNINFODENIED,
}

impl AsRef<str> for GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum::ACCESSSTATEUNSPECIFIED => "ACCESS_STATE_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum::GRANTED => "GRANTED",
            GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum::NOTGRANTED => "NOT_GRANTED",
            GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum::UNKNOWNCONDITIONAL => "UNKNOWN_CONDITIONAL",
            GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum::UNKNOWNINFODENIED => "UNKNOWN_INFO_DENIED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_STATE_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum::ACCESSSTATEUNSPECIFIED),
           "GRANTED" => Ok(GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum::GRANTED),
           "NOT_GRANTED" => Ok(GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum::NOTGRANTED),
           "UNKNOWN_CONDITIONAL" => Ok(GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum::UNKNOWNCONDITIONAL),
           "UNKNOWN_INFO_DENIED" => Ok(GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum::UNKNOWNINFODENIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether _this policy_ provides the specified permission to the specified principal for the specified resource. This field does _not_ indicate whether the principal actually has the permission for the resource. There might be another policy that overrides this policy. To determine whether the principal actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse.
pub enum GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum {
    

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
    

    /// The user who created the Replay does not have access to all of the policies that Policy Simulator needs to evaluate.
    ///
    /// "UNKNOWN_INFO_DENIED"
    #[serde(rename="UNKNOWN_INFO_DENIED")]
    UNKNOWNINFODENIED,
}

impl AsRef<str> for GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum::ACCESSSTATEUNSPECIFIED => "ACCESS_STATE_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum::GRANTED => "GRANTED",
            GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum::NOTGRANTED => "NOT_GRANTED",
            GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum::UNKNOWNCONDITIONAL => "UNKNOWN_CONDITIONAL",
            GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum::UNKNOWNINFODENIED => "UNKNOWN_INFO_DENIED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_STATE_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum::ACCESSSTATEUNSPECIFIED),
           "GRANTED" => Ok(GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum::GRANTED),
           "NOT_GRANTED" => Ok(GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum::NOTGRANTED),
           "UNKNOWN_CONDITIONAL" => Ok(GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum::UNKNOWNCONDITIONAL),
           "UNKNOWN_INFO_DENIED" => Ok(GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum::UNKNOWNINFODENIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The relevance of this policy to the overall determination in the TroubleshootIamPolicyResponse. If the user who created the Replay does not have access to the policy, this field is omitted.
pub enum GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum {
    

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

impl AsRef<str> for GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED => "HEURISTIC_RELEVANCE_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum::NORMAL => "NORMAL",
            GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum::HIGH => "HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEURISTIC_RELEVANCE_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum::HEURISTICRELEVANCEUNSPECIFIED),
           "NORMAL" => Ok(GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum::NORMAL),
           "HIGH" => Ok(GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum::HIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the `OrgPolicyViolationsPreview`.
pub enum GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum {
    

    /// The state is unspecified.
    ///
    /// "PREVIEW_STATE_UNSPECIFIED"
    #[serde(rename="PREVIEW_STATE_UNSPECIFIED")]
    PREVIEWSTATEUNSPECIFIED,
    

    /// The OrgPolicyViolationsPreview has not been created yet.
    ///
    /// "PREVIEW_PENDING"
    #[serde(rename="PREVIEW_PENDING")]
    PREVIEWPENDING,
    

    /// The OrgPolicyViolationsPreview is currently being created.
    ///
    /// "PREVIEW_RUNNING"
    #[serde(rename="PREVIEW_RUNNING")]
    PREVIEWRUNNING,
    

    /// The OrgPolicyViolationsPreview creation finished successfully.
    ///
    /// "PREVIEW_SUCCEEDED"
    #[serde(rename="PREVIEW_SUCCEEDED")]
    PREVIEWSUCCEEDED,
    

    /// The OrgPolicyViolationsPreview creation failed with an error.
    ///
    /// "PREVIEW_FAILED"
    #[serde(rename="PREVIEW_FAILED")]
    PREVIEWFAILED,
}

impl AsRef<str> for GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum::PREVIEWSTATEUNSPECIFIED => "PREVIEW_STATE_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum::PREVIEWPENDING => "PREVIEW_PENDING",
            GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum::PREVIEWRUNNING => "PREVIEW_RUNNING",
            GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum::PREVIEWSUCCEEDED => "PREVIEW_SUCCEEDED",
            GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum::PREVIEWFAILED => "PREVIEW_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PREVIEW_STATE_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum::PREVIEWSTATEUNSPECIFIED),
           "PREVIEW_PENDING" => Ok(GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum::PREVIEWPENDING),
           "PREVIEW_RUNNING" => Ok(GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum::PREVIEWRUNNING),
           "PREVIEW_SUCCEEDED" => Ok(GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum::PREVIEWSUCCEEDED),
           "PREVIEW_FAILED" => Ok(GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum::PREVIEWFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1ReplayStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the `Replay`.
pub enum GoogleCloudPolicysimulatorV1ReplayStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The `Replay` has not started yet.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The `Replay` is currently running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The `Replay` has successfully completed.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The `Replay` has finished with an error.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for GoogleCloudPolicysimulatorV1ReplayStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1ReplayStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1ReplayStateEnum::PENDING => "PENDING",
            GoogleCloudPolicysimulatorV1ReplayStateEnum::RUNNING => "RUNNING",
            GoogleCloudPolicysimulatorV1ReplayStateEnum::SUCCEEDED => "SUCCEEDED",
            GoogleCloudPolicysimulatorV1ReplayStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1ReplayStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1ReplayStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(GoogleCloudPolicysimulatorV1ReplayStateEnum::PENDING),
           "RUNNING" => Ok(GoogleCloudPolicysimulatorV1ReplayStateEnum::RUNNING),
           "SUCCEEDED" => Ok(GoogleCloudPolicysimulatorV1ReplayStateEnum::SUCCEEDED),
           "FAILED" => Ok(GoogleCloudPolicysimulatorV1ReplayStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1ReplayStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPolicysimulatorV1ReplayConfigLogSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The logs to use as input for the Replay.
pub enum GoogleCloudPolicysimulatorV1ReplayConfigLogSourceEnum {
    

    /// An unspecified log source. If the log source is unspecified, the Replay defaults to using `RECENT_ACCESSES`.
    ///
    /// "LOG_SOURCE_UNSPECIFIED"
    #[serde(rename="LOG_SOURCE_UNSPECIFIED")]
    LOGSOURCEUNSPECIFIED,
    

    /// All access logs from the last 90 days. These logs may not include logs from the most recent 7 days.
    ///
    /// "RECENT_ACCESSES"
    #[serde(rename="RECENT_ACCESSES")]
    RECENTACCESSES,
}

impl AsRef<str> for GoogleCloudPolicysimulatorV1ReplayConfigLogSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPolicysimulatorV1ReplayConfigLogSourceEnum::LOGSOURCEUNSPECIFIED => "LOG_SOURCE_UNSPECIFIED",
            GoogleCloudPolicysimulatorV1ReplayConfigLogSourceEnum::RECENTACCESSES => "RECENT_ACCESSES",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPolicysimulatorV1ReplayConfigLogSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_SOURCE_UNSPECIFIED" => Ok(GoogleCloudPolicysimulatorV1ReplayConfigLogSourceEnum::LOGSOURCEUNSPECIFIED),
           "RECENT_ACCESSES" => Ok(GoogleCloudPolicysimulatorV1ReplayConfigLogSourceEnum::RECENTACCESSES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPolicysimulatorV1ReplayConfigLogSourceEnum {
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


