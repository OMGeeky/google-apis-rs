use super::*;



// region GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The evaluation behavior of this constraint in the absence of a policy.
pub enum GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum {
    

    /// This is only used for distinguishing unset values and should never be used.
    ///
    /// "CONSTRAINT_DEFAULT_UNSPECIFIED"
    #[serde(rename="CONSTRAINT_DEFAULT_UNSPECIFIED")]
    CONSTRAINTDEFAULTUNSPECIFIED,
    

    /// Indicate that all values are allowed for list constraints. Indicate that enforcement is off for boolean constraints.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// Indicate that all values are denied for list constraints. Indicate that enforcement is on for boolean constraints.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum::CONSTRAINTDEFAULTUNSPECIFIED => "CONSTRAINT_DEFAULT_UNSPECIFIED",
            GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum::ALLOW => "ALLOW",
            GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONSTRAINT_DEFAULT_UNSPECIFIED" => Ok(GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum::CONSTRAINTDEFAULTUNSPECIFIED),
           "ALLOW" => Ok(GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum::ALLOW),
           "DENY" => Ok(GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintActionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allow or deny type.
pub enum GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintActionTypeEnum {
    

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

impl AsRef<str> for GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintActionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintActionTypeEnum::ACTIONTYPEUNSPECIFIED => "ACTION_TYPE_UNSPECIFIED",
            GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintActionTypeEnum::ALLOW => "ALLOW",
            GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintActionTypeEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintActionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintActionTypeEnum::ACTIONTYPEUNSPECIFIED),
           "ALLOW" => Ok(GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintActionTypeEnum::ALLOW),
           "DENY" => Ok(GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintActionTypeEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintActionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// All the operations being applied for this constraint.
pub enum GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum {
    

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

impl AsRef<str> for GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum::METHODTYPEUNSPECIFIED => "METHOD_TYPE_UNSPECIFIED",
            GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum::CREATE => "CREATE",
            GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum::UPDATE => "UPDATE",
            GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum::DELETE => "DELETE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METHOD_TYPE_UNSPECIFIED" => Ok(GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum::METHODTYPEUNSPECIFIED),
           "CREATE" => Ok(GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum::CREATE),
           "UPDATE" => Ok(GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum::UPDATE),
           "DELETE" => Ok(GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum::DELETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudOrgpolicyV2ConstraintGoogleDefinedCustomConstraintMethodTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


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


