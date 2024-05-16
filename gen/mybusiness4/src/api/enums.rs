use super::*;



// region AccountPermissionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the PermissionLevel the caller has for this account.
pub enum AccountPermissionLevelEnum {
    

    /// Not specified.
    ///
    /// "PERMISSION_LEVEL_UNSPECIFIED"
    #[serde(rename="PERMISSION_LEVEL_UNSPECIFIED")]
    PERMISSIONLEVELUNSPECIFIED,
    

    /// The user has owner level permission.
    ///
    /// "OWNER_LEVEL"
    #[serde(rename="OWNER_LEVEL")]
    OWNERLEVEL,
    

    /// The user has member level permission.
    ///
    /// "MEMBER_LEVEL"
    #[serde(rename="MEMBER_LEVEL")]
    MEMBERLEVEL,
}

impl AsRef<str> for AccountPermissionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountPermissionLevelEnum::PERMISSIONLEVELUNSPECIFIED => "PERMISSION_LEVEL_UNSPECIFIED",
            AccountPermissionLevelEnum::OWNERLEVEL => "OWNER_LEVEL",
            AccountPermissionLevelEnum::MEMBERLEVEL => "MEMBER_LEVEL",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountPermissionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERMISSION_LEVEL_UNSPECIFIED" => Ok(AccountPermissionLevelEnum::PERMISSIONLEVELUNSPECIFIED),
           "OWNER_LEVEL" => Ok(AccountPermissionLevelEnum::OWNERLEVEL),
           "MEMBER_LEVEL" => Ok(AccountPermissionLevelEnum::MEMBERLEVEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountPermissionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the AccountRole
the caller has for this account.
pub enum AccountRoleEnum {
    

    /// Not specified.
    ///
    /// "ACCOUNT_ROLE_UNSPECIFIED"
    #[serde(rename="ACCOUNT_ROLE_UNSPECIFIED")]
    ACCOUNTROLEUNSPECIFIED,
    

    /// The user owns this account. (Displays as 'Primary Owner' in UI).
    ///
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
    

    /// The user is a co-owner of the account. (Displays as 'Owner' in UI).
    ///
    /// "CO_OWNER"
    #[serde(rename="CO_OWNER")]
    COOWNER,
    

    /// The user can manage this account.
    ///
    /// "MANAGER"
    #[serde(rename="MANAGER")]
    MANAGER,
    

    /// The user can manage social (Google+) pages for the account.
(Displays as 'Site Manager' in UI).
    ///
    /// "COMMUNITY_MANAGER"
    #[serde(rename="COMMUNITY_MANAGER")]
    COMMUNITYMANAGER,
}

impl AsRef<str> for AccountRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountRoleEnum::ACCOUNTROLEUNSPECIFIED => "ACCOUNT_ROLE_UNSPECIFIED",
            AccountRoleEnum::OWNER => "OWNER",
            AccountRoleEnum::COOWNER => "CO_OWNER",
            AccountRoleEnum::MANAGER => "MANAGER",
            AccountRoleEnum::COMMUNITYMANAGER => "COMMUNITY_MANAGER",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCOUNT_ROLE_UNSPECIFIED" => Ok(AccountRoleEnum::ACCOUNTROLEUNSPECIFIED),
           "OWNER" => Ok(AccountRoleEnum::OWNER),
           "CO_OWNER" => Ok(AccountRoleEnum::COOWNER),
           "MANAGER" => Ok(AccountRoleEnum::MANAGER),
           "COMMUNITY_MANAGER" => Ok(AccountRoleEnum::COMMUNITYMANAGER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the AccountType
of this account.
pub enum AccountTypeEnum {
    

    /// Not specified.
    ///
    /// "ACCOUNT_TYPE_UNSPECIFIED"
    #[serde(rename="ACCOUNT_TYPE_UNSPECIFIED")]
    ACCOUNTTYPEUNSPECIFIED,
    

    /// An end-user account.
    ///
    /// "PERSONAL"
    #[serde(rename="PERSONAL")]
    PERSONAL,
    

    /// A group of Locations. For more information, see the [help center article]
(https://support.google.com/business/answer/6085326)
    ///
    /// "LOCATION_GROUP"
    #[serde(rename="LOCATION_GROUP")]
    LOCATIONGROUP,
    

    /// A User Group for segregating organization staff in groups. For more
information, see the [help center
article](https://support.google.com/business/answer/7655731)
    ///
    /// "USER_GROUP"
    #[serde(rename="USER_GROUP")]
    USERGROUP,
    

    /// An organization representing a company. For more information, see the [help
center article](https://support.google.com/business/answer/7663063)
    ///
    /// "ORGANIZATION"
    #[serde(rename="ORGANIZATION")]
    ORGANIZATION,
}

impl AsRef<str> for AccountTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountTypeEnum::ACCOUNTTYPEUNSPECIFIED => "ACCOUNT_TYPE_UNSPECIFIED",
            AccountTypeEnum::PERSONAL => "PERSONAL",
            AccountTypeEnum::LOCATIONGROUP => "LOCATION_GROUP",
            AccountTypeEnum::USERGROUP => "USER_GROUP",
            AccountTypeEnum::ORGANIZATION => "ORGANIZATION",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCOUNT_TYPE_UNSPECIFIED" => Ok(AccountTypeEnum::ACCOUNTTYPEUNSPECIFIED),
           "PERSONAL" => Ok(AccountTypeEnum::PERSONAL),
           "LOCATION_GROUP" => Ok(AccountTypeEnum::LOCATIONGROUP),
           "USER_GROUP" => Ok(AccountTypeEnum::USERGROUP),
           "ORGANIZATION" => Ok(AccountTypeEnum::ORGANIZATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountStateStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If verified, future locations that are created are automatically
connected to Google Maps, and have Google+ pages created, without requiring
moderation.
pub enum AccountStateStatusEnum {
    

    /// Not specified.
    ///
    /// "ACCOUNT_STATUS_UNSPECIFIED"
    #[serde(rename="ACCOUNT_STATUS_UNSPECIFIED")]
    ACCOUNTSTATUSUNSPECIFIED,
    

    /// Verified account.
    ///
    /// "VERIFIED"
    #[serde(rename="VERIFIED")]
    VERIFIED,
    

    /// Account that is not verified, and verification has not been requested.
    ///
    /// "UNVERIFIED"
    #[serde(rename="UNVERIFIED")]
    UNVERIFIED,
    

    /// Account that is not verified, but verification has been requested.
    ///
    /// "VERIFICATION_REQUESTED"
    #[serde(rename="VERIFICATION_REQUESTED")]
    VERIFICATIONREQUESTED,
}

impl AsRef<str> for AccountStateStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountStateStatusEnum::ACCOUNTSTATUSUNSPECIFIED => "ACCOUNT_STATUS_UNSPECIFIED",
            AccountStateStatusEnum::VERIFIED => "VERIFIED",
            AccountStateStatusEnum::UNVERIFIED => "UNVERIFIED",
            AccountStateStatusEnum::VERIFICATIONREQUESTED => "VERIFICATION_REQUESTED",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountStateStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCOUNT_STATUS_UNSPECIFIED" => Ok(AccountStateStatusEnum::ACCOUNTSTATUSUNSPECIFIED),
           "VERIFIED" => Ok(AccountStateStatusEnum::VERIFIED),
           "UNVERIFIED" => Ok(AccountStateStatusEnum::UNVERIFIED),
           "VERIFICATION_REQUESTED" => Ok(AccountStateStatusEnum::VERIFICATIONREQUESTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountStateStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountStateVettedStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the account is vetted by Google. A vetted account is able
to verify locations via the VETTED_PARTNER method.
pub enum AccountStateVettedStatusEnum {
    

    /// Not Specified
    ///
    /// "VETTED_STATUS_UNSPECIFIED"
    #[serde(rename="VETTED_STATUS_UNSPECIFIED")]
    VETTEDSTATUSUNSPECIFIED,
    

    /// The account is not vetted by Google.
    ///
    /// "NOT_VETTED"
    #[serde(rename="NOT_VETTED")]
    NOTVETTED,
    

    /// The account is vetted by Google and in a valid state.
An account is automatically vetted if it has direct access to a vetted
group account.
    ///
    /// "VETTED"
    #[serde(rename="VETTED")]
    VETTED,
    

    /// The account is vetted but in an invalid state. The account will behave
like an unvetted account.
    ///
    /// "INVALID"
    #[serde(rename="INVALID")]
    INVALID,
}

impl AsRef<str> for AccountStateVettedStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountStateVettedStatusEnum::VETTEDSTATUSUNSPECIFIED => "VETTED_STATUS_UNSPECIFIED",
            AccountStateVettedStatusEnum::NOTVETTED => "NOT_VETTED",
            AccountStateVettedStatusEnum::VETTED => "VETTED",
            AccountStateVettedStatusEnum::INVALID => "INVALID",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountStateVettedStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VETTED_STATUS_UNSPECIFIED" => Ok(AccountStateVettedStatusEnum::VETTEDSTATUSUNSPECIFIED),
           "NOT_VETTED" => Ok(AccountStateVettedStatusEnum::NOTVETTED),
           "VETTED" => Ok(AccountStateVettedStatusEnum::VETTED),
           "INVALID" => Ok(AccountStateVettedStatusEnum::INVALID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountStateVettedStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdminRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the AdminRole that this
admin uses with the specified Account
or Location resource.
pub enum AdminRoleEnum {
    

    /// Not specified.
    ///
    /// "ADMIN_ROLE_UNSPECIFIED"
    #[serde(rename="ADMIN_ROLE_UNSPECIFIED")]
    ADMINROLEUNSPECIFIED,
    

    /// The admin has owner-level access and is the primary owner. (Displays as
'Primary Owner' in UI).
    ///
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
    

    /// The admin has owner-level access. (Displays as 'Owner' in UI).
    ///
    /// "CO_OWNER"
    #[serde(rename="CO_OWNER")]
    COOWNER,
    

    /// The admin has managerial access.
    ///
    /// "MANAGER"
    #[serde(rename="MANAGER")]
    MANAGER,
    

    /// The admin can manage social (Google+) pages. (Displays as 'Site Manager'
in UI).
    ///
    /// "COMMUNITY_MANAGER"
    #[serde(rename="COMMUNITY_MANAGER")]
    COMMUNITYMANAGER,
}

impl AsRef<str> for AdminRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdminRoleEnum::ADMINROLEUNSPECIFIED => "ADMIN_ROLE_UNSPECIFIED",
            AdminRoleEnum::OWNER => "OWNER",
            AdminRoleEnum::COOWNER => "CO_OWNER",
            AdminRoleEnum::MANAGER => "MANAGER",
            AdminRoleEnum::COMMUNITYMANAGER => "COMMUNITY_MANAGER",
        }
    }
}

impl std::convert::TryFrom< &str> for AdminRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADMIN_ROLE_UNSPECIFIED" => Ok(AdminRoleEnum::ADMINROLEUNSPECIFIED),
           "OWNER" => Ok(AdminRoleEnum::OWNER),
           "CO_OWNER" => Ok(AdminRoleEnum::COOWNER),
           "MANAGER" => Ok(AdminRoleEnum::MANAGER),
           "COMMUNITY_MANAGER" => Ok(AdminRoleEnum::COMMUNITYMANAGER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdminRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AttributeValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of value that this attribute contains. This should be
used to determine how to interpret the value.
pub enum AttributeValueTypeEnum {
    

    /// Not specified.
    ///
    /// "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED"
    #[serde(rename="ATTRIBUTE_VALUE_TYPE_UNSPECIFIED")]
    ATTRIBUTEVALUETYPEUNSPECIFIED,
    

    /// The values for this attribute are boolean values.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// The attribute has a predetermined list of available values that can be
used. Metadata for this attribute will list these values.
    ///
    /// "ENUM"
    #[serde(rename="ENUM")]
    ENUM,
    

    /// The values for this attribute are URLs.
    ///
    /// "URL"
    #[serde(rename="URL")]
    URL,
    

    /// The attribute value is an enum with multiple possible values that can be
explicitly set or unset.
    ///
    /// "REPEATED_ENUM"
    #[serde(rename="REPEATED_ENUM")]
    REPEATEDENUM,
}

impl AsRef<str> for AttributeValueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AttributeValueTypeEnum::ATTRIBUTEVALUETYPEUNSPECIFIED => "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED",
            AttributeValueTypeEnum::BOOL => "BOOL",
            AttributeValueTypeEnum::ENUM => "ENUM",
            AttributeValueTypeEnum::URL => "URL",
            AttributeValueTypeEnum::REPEATEDENUM => "REPEATED_ENUM",
        }
    }
}

impl std::convert::TryFrom< &str> for AttributeValueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED" => Ok(AttributeValueTypeEnum::ATTRIBUTEVALUETYPEUNSPECIFIED),
           "BOOL" => Ok(AttributeValueTypeEnum::BOOL),
           "ENUM" => Ok(AttributeValueTypeEnum::ENUM),
           "URL" => Ok(AttributeValueTypeEnum::URL),
           "REPEATED_ENUM" => Ok(AttributeValueTypeEnum::REPEATEDENUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AttributeValueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AttributeMetadataValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The value type for the attribute. Values set and retrieved should be
expected to be of this type.
pub enum AttributeMetadataValueTypeEnum {
    

    /// Not specified.
    ///
    /// "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED"
    #[serde(rename="ATTRIBUTE_VALUE_TYPE_UNSPECIFIED")]
    ATTRIBUTEVALUETYPEUNSPECIFIED,
    

    /// The values for this attribute are boolean values.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// The attribute has a predetermined list of available values that can be
used. Metadata for this attribute will list these values.
    ///
    /// "ENUM"
    #[serde(rename="ENUM")]
    ENUM,
    

    /// The values for this attribute are URLs.
    ///
    /// "URL"
    #[serde(rename="URL")]
    URL,
    

    /// The attribute value is an enum with multiple possible values that can be
explicitly set or unset.
    ///
    /// "REPEATED_ENUM"
    #[serde(rename="REPEATED_ENUM")]
    REPEATEDENUM,
}

impl AsRef<str> for AttributeMetadataValueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AttributeMetadataValueTypeEnum::ATTRIBUTEVALUETYPEUNSPECIFIED => "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED",
            AttributeMetadataValueTypeEnum::BOOL => "BOOL",
            AttributeMetadataValueTypeEnum::ENUM => "ENUM",
            AttributeMetadataValueTypeEnum::URL => "URL",
            AttributeMetadataValueTypeEnum::REPEATEDENUM => "REPEATED_ENUM",
        }
    }
}

impl std::convert::TryFrom< &str> for AttributeMetadataValueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED" => Ok(AttributeMetadataValueTypeEnum::ATTRIBUTEVALUETYPEUNSPECIFIED),
           "BOOL" => Ok(AttributeMetadataValueTypeEnum::BOOL),
           "ENUM" => Ok(AttributeMetadataValueTypeEnum::ENUM),
           "URL" => Ok(AttributeMetadataValueTypeEnum::URL),
           "REPEATED_ENUM" => Ok(AttributeMetadataValueTypeEnum::REPEATEDENUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AttributeMetadataValueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuthorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of user the author is.
pub enum AuthorTypeEnum {
    

    /// This should not be used.
    ///
    /// "AUTHOR_TYPE_UNSPECIFIED"
    #[serde(rename="AUTHOR_TYPE_UNSPECIFIED")]
    AUTHORTYPEUNSPECIFIED,
    

    /// A regular user.
    ///
    /// "REGULAR_USER"
    #[serde(rename="REGULAR_USER")]
    REGULARUSER,
    

    /// A Local Guide
    ///
    /// "LOCAL_GUIDE"
    #[serde(rename="LOCAL_GUIDE")]
    LOCALGUIDE,
    

    /// The owner/manager of the location
    ///
    /// "MERCHANT"
    #[serde(rename="MERCHANT")]
    MERCHANT,
}

impl AsRef<str> for AuthorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthorTypeEnum::AUTHORTYPEUNSPECIFIED => "AUTHOR_TYPE_UNSPECIFIED",
            AuthorTypeEnum::REGULARUSER => "REGULAR_USER",
            AuthorTypeEnum::LOCALGUIDE => "LOCAL_GUIDE",
            AuthorTypeEnum::MERCHANT => "MERCHANT",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTHOR_TYPE_UNSPECIFIED" => Ok(AuthorTypeEnum::AUTHORTYPEUNSPECIFIED),
           "REGULAR_USER" => Ok(AuthorTypeEnum::REGULARUSER),
           "LOCAL_GUIDE" => Ok(AuthorTypeEnum::LOCALGUIDE),
           "MERCHANT" => Ok(AuthorTypeEnum::MERCHANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CallToActionActionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of action that will be performed.
pub enum CallToActionActionTypeEnum {
    

    /// Type unspecified.
    ///
    /// "ACTION_TYPE_UNSPECIFIED"
    #[serde(rename="ACTION_TYPE_UNSPECIFIED")]
    ACTIONTYPEUNSPECIFIED,
    

    /// This post wants a user to book an appointment/table/etc.
    ///
    /// "BOOK"
    #[serde(rename="BOOK")]
    BOOK,
    

    /// This post wants a user to order something.
    ///
    /// "ORDER"
    #[serde(rename="ORDER")]
    ORDER,
    

    /// This post wants a user to browse a product catalog.
    ///
    /// "SHOP"
    #[serde(rename="SHOP")]
    SHOP,
    

    /// This post wants a user to learn more (at their website).
    ///
    /// "LEARN_MORE"
    #[serde(rename="LEARN_MORE")]
    LEARNMORE,
    

    /// This post wants a user to register/sign up/join something.
    ///
    /// "SIGN_UP"
    #[serde(rename="SIGN_UP")]
    SIGNUP,
    

    /// Deprecated. Use `OFFER` in `LocalPostTopicType` to create a post with
offer content.
    ///
    /// "GET_OFFER"
    #[serde(rename="GET_OFFER")]
    GETOFFER,
    

    /// This post wants a user to call the business.
    ///
    /// "CALL"
    #[serde(rename="CALL")]
    CALL,
}

impl AsRef<str> for CallToActionActionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CallToActionActionTypeEnum::ACTIONTYPEUNSPECIFIED => "ACTION_TYPE_UNSPECIFIED",
            CallToActionActionTypeEnum::BOOK => "BOOK",
            CallToActionActionTypeEnum::ORDER => "ORDER",
            CallToActionActionTypeEnum::SHOP => "SHOP",
            CallToActionActionTypeEnum::LEARNMORE => "LEARN_MORE",
            CallToActionActionTypeEnum::SIGNUP => "SIGN_UP",
            CallToActionActionTypeEnum::GETOFFER => "GET_OFFER",
            CallToActionActionTypeEnum::CALL => "CALL",
        }
    }
}

impl std::convert::TryFrom< &str> for CallToActionActionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_TYPE_UNSPECIFIED" => Ok(CallToActionActionTypeEnum::ACTIONTYPEUNSPECIFIED),
           "BOOK" => Ok(CallToActionActionTypeEnum::BOOK),
           "ORDER" => Ok(CallToActionActionTypeEnum::ORDER),
           "SHOP" => Ok(CallToActionActionTypeEnum::SHOP),
           "LEARN_MORE" => Ok(CallToActionActionTypeEnum::LEARNMORE),
           "SIGN_UP" => Ok(CallToActionActionTypeEnum::SIGNUP),
           "GET_OFFER" => Ok(CallToActionActionTypeEnum::GETOFFER),
           "CALL" => Ok(CallToActionActionTypeEnum::CALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CallToActionActionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DimensionalMetricValueMetricOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The option that requested this dimensional value.
pub enum DimensionalMetricValueMetricOptionEnum {
    

    /// No metric option specified. Will default to AGGREGATED_TOTAL in a request.
    ///
    /// "METRIC_OPTION_UNSPECIFIED"
    #[serde(rename="METRIC_OPTION_UNSPECIFIED")]
    METRICOPTIONUNSPECIFIED,
    

    /// Return values aggregated over the entire time frame.
This is the default value.
    ///
    /// "AGGREGATED_TOTAL"
    #[serde(rename="AGGREGATED_TOTAL")]
    AGGREGATEDTOTAL,
    

    /// Return daily timestamped values across time range.
    ///
    /// "AGGREGATED_DAILY"
    #[serde(rename="AGGREGATED_DAILY")]
    AGGREGATEDDAILY,
    

    /// Values will be returned as a breakdown by day of the week.
Only valid for ACTIONS_PHONE.
    ///
    /// "BREAKDOWN_DAY_OF_WEEK"
    #[serde(rename="BREAKDOWN_DAY_OF_WEEK")]
    BREAKDOWNDAYOFWEEK,
    

    /// Values will be returned as a breakdown by hour of the day.
Only valid for ACTIONS_PHONE.
    ///
    /// "BREAKDOWN_HOUR_OF_DAY"
    #[serde(rename="BREAKDOWN_HOUR_OF_DAY")]
    BREAKDOWNHOUROFDAY,
}

impl AsRef<str> for DimensionalMetricValueMetricOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DimensionalMetricValueMetricOptionEnum::METRICOPTIONUNSPECIFIED => "METRIC_OPTION_UNSPECIFIED",
            DimensionalMetricValueMetricOptionEnum::AGGREGATEDTOTAL => "AGGREGATED_TOTAL",
            DimensionalMetricValueMetricOptionEnum::AGGREGATEDDAILY => "AGGREGATED_DAILY",
            DimensionalMetricValueMetricOptionEnum::BREAKDOWNDAYOFWEEK => "BREAKDOWN_DAY_OF_WEEK",
            DimensionalMetricValueMetricOptionEnum::BREAKDOWNHOUROFDAY => "BREAKDOWN_HOUR_OF_DAY",
        }
    }
}

impl std::convert::TryFrom< &str> for DimensionalMetricValueMetricOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_OPTION_UNSPECIFIED" => Ok(DimensionalMetricValueMetricOptionEnum::METRICOPTIONUNSPECIFIED),
           "AGGREGATED_TOTAL" => Ok(DimensionalMetricValueMetricOptionEnum::AGGREGATEDTOTAL),
           "AGGREGATED_DAILY" => Ok(DimensionalMetricValueMetricOptionEnum::AGGREGATEDDAILY),
           "BREAKDOWN_DAY_OF_WEEK" => Ok(DimensionalMetricValueMetricOptionEnum::BREAKDOWNDAYOFWEEK),
           "BREAKDOWN_HOUR_OF_DAY" => Ok(DimensionalMetricValueMetricOptionEnum::BREAKDOWNHOUROFDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DimensionalMetricValueMetricOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DrivingDirectionMetricsRequestNumDaysEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The number of days to aggregate data for. Results returned will
be available data over the last number of requested days.
Valid values are 7, 30, and 90.
pub enum DrivingDirectionMetricsRequestNumDaysEnum {
    

    /// 7 days. This is the default value.
    ///
    /// "SEVEN"
    #[serde(rename="SEVEN")]
    SEVEN,
    

    /// 30 days.
    ///
    /// "THIRTY"
    #[serde(rename="THIRTY")]
    THIRTY,
    

    /// 90 days.
    ///
    /// "NINETY"
    #[serde(rename="NINETY")]
    NINETY,
}

impl AsRef<str> for DrivingDirectionMetricsRequestNumDaysEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DrivingDirectionMetricsRequestNumDaysEnum::SEVEN => "SEVEN",
            DrivingDirectionMetricsRequestNumDaysEnum::THIRTY => "THIRTY",
            DrivingDirectionMetricsRequestNumDaysEnum::NINETY => "NINETY",
        }
    }
}

impl std::convert::TryFrom< &str> for DrivingDirectionMetricsRequestNumDaysEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVEN" => Ok(DrivingDirectionMetricsRequestNumDaysEnum::SEVEN),
           "THIRTY" => Ok(DrivingDirectionMetricsRequestNumDaysEnum::THIRTY),
           "NINETY" => Ok(DrivingDirectionMetricsRequestNumDaysEnum::NINETY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DrivingDirectionMetricsRequestNumDaysEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DuplicateAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the user has access to the location it duplicates.
pub enum DuplicateAccessEnum {
    

    /// Not specified.
    ///
    /// "ACCESS_UNSPECIFIED"
    #[serde(rename="ACCESS_UNSPECIFIED")]
    ACCESSUNSPECIFIED,
    

    /// Unable to determine whether the user has access to the location that
it duplicates.
    ///
    /// "ACCESS_UNKNOWN"
    #[serde(rename="ACCESS_UNKNOWN")]
    ACCESSUNKNOWN,
    

    /// User has access to the location that it duplicates.
    ///
    /// "ALLOWED"
    #[serde(rename="ALLOWED")]
    ALLOWED,
    

    /// User doesn't have access to the location that it duplicates.
    ///
    /// "INSUFFICIENT"
    #[serde(rename="INSUFFICIENT")]
    INSUFFICIENT,
}

impl AsRef<str> for DuplicateAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DuplicateAccessEnum::ACCESSUNSPECIFIED => "ACCESS_UNSPECIFIED",
            DuplicateAccessEnum::ACCESSUNKNOWN => "ACCESS_UNKNOWN",
            DuplicateAccessEnum::ALLOWED => "ALLOWED",
            DuplicateAccessEnum::INSUFFICIENT => "INSUFFICIENT",
        }
    }
}

impl std::convert::TryFrom< &str> for DuplicateAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_UNSPECIFIED" => Ok(DuplicateAccessEnum::ACCESSUNSPECIFIED),
           "ACCESS_UNKNOWN" => Ok(DuplicateAccessEnum::ACCESSUNKNOWN),
           "ALLOWED" => Ok(DuplicateAccessEnum::ALLOWED),
           "INSUFFICIENT" => Ok(DuplicateAccessEnum::INSUFFICIENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DuplicateAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InvitationRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The invited role on the account.
pub enum InvitationRoleEnum {
    

    /// Not specified.
    ///
    /// "ADMIN_ROLE_UNSPECIFIED"
    #[serde(rename="ADMIN_ROLE_UNSPECIFIED")]
    ADMINROLEUNSPECIFIED,
    

    /// The admin has owner-level access and is the primary owner. (Displays as
'Primary Owner' in UI).
    ///
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
    

    /// The admin has owner-level access. (Displays as 'Owner' in UI).
    ///
    /// "CO_OWNER"
    #[serde(rename="CO_OWNER")]
    COOWNER,
    

    /// The admin has managerial access.
    ///
    /// "MANAGER"
    #[serde(rename="MANAGER")]
    MANAGER,
    

    /// The admin can manage social (Google+) pages. (Displays as 'Site Manager'
in UI).
    ///
    /// "COMMUNITY_MANAGER"
    #[serde(rename="COMMUNITY_MANAGER")]
    COMMUNITYMANAGER,
}

impl AsRef<str> for InvitationRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InvitationRoleEnum::ADMINROLEUNSPECIFIED => "ADMIN_ROLE_UNSPECIFIED",
            InvitationRoleEnum::OWNER => "OWNER",
            InvitationRoleEnum::COOWNER => "CO_OWNER",
            InvitationRoleEnum::MANAGER => "MANAGER",
            InvitationRoleEnum::COMMUNITYMANAGER => "COMMUNITY_MANAGER",
        }
    }
}

impl std::convert::TryFrom< &str> for InvitationRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADMIN_ROLE_UNSPECIFIED" => Ok(InvitationRoleEnum::ADMINROLEUNSPECIFIED),
           "OWNER" => Ok(InvitationRoleEnum::OWNER),
           "CO_OWNER" => Ok(InvitationRoleEnum::COOWNER),
           "MANAGER" => Ok(InvitationRoleEnum::MANAGER),
           "COMMUNITY_MANAGER" => Ok(InvitationRoleEnum::COMMUNITYMANAGER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InvitationRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LocalPostAlertTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of alert the post is created for. This field is only applicable
for posts of topic_type Alert, and behaves as a sub-type of Alerts.
pub enum LocalPostAlertTypeEnum {
    

    /// No alert is specified.
    ///
    /// "ALERT_TYPE_UNSPECIFIED"
    #[serde(rename="ALERT_TYPE_UNSPECIFIED")]
    ALERTTYPEUNSPECIFIED,
    

    /// Alerts related to the 2019 Coronavirus Disease pandemic. Covid posts only
support a summary field and a call to action field. When these alerts are
no longer relevant, new Alert post creation for type COVID-19 will be
disabled. However, merchant will still be able to manage their existing
COVID-19 posts.
    ///
    /// "COVID_19"
    #[serde(rename="COVID_19")]
    COVID19,
}

impl AsRef<str> for LocalPostAlertTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LocalPostAlertTypeEnum::ALERTTYPEUNSPECIFIED => "ALERT_TYPE_UNSPECIFIED",
            LocalPostAlertTypeEnum::COVID19 => "COVID_19",
        }
    }
}

impl std::convert::TryFrom< &str> for LocalPostAlertTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALERT_TYPE_UNSPECIFIED" => Ok(LocalPostAlertTypeEnum::ALERTTYPEUNSPECIFIED),
           "COVID_19" => Ok(LocalPostAlertTypeEnum::COVID19),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LocalPostAlertTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LocalPostStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the post, indicating what part of its lifecycle
it is in.
pub enum LocalPostStateEnum {
    

    /// State not specified.
    ///
    /// "LOCAL_POST_STATE_UNSPECIFIED"
    #[serde(rename="LOCAL_POST_STATE_UNSPECIFIED")]
    LOCALPOSTSTATEUNSPECIFIED,
    

    /// This post was rejected due to content policy violation.
    ///
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// This post is published and is currently appearing in search results.
    ///
    /// "LIVE"
    #[serde(rename="LIVE")]
    LIVE,
    

    /// This post is being processed and is not appearing in search results.
    ///
    /// "PROCESSING"
    #[serde(rename="PROCESSING")]
    PROCESSING,
}

impl AsRef<str> for LocalPostStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LocalPostStateEnum::LOCALPOSTSTATEUNSPECIFIED => "LOCAL_POST_STATE_UNSPECIFIED",
            LocalPostStateEnum::REJECTED => "REJECTED",
            LocalPostStateEnum::LIVE => "LIVE",
            LocalPostStateEnum::PROCESSING => "PROCESSING",
        }
    }
}

impl std::convert::TryFrom< &str> for LocalPostStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOCAL_POST_STATE_UNSPECIFIED" => Ok(LocalPostStateEnum::LOCALPOSTSTATEUNSPECIFIED),
           "REJECTED" => Ok(LocalPostStateEnum::REJECTED),
           "LIVE" => Ok(LocalPostStateEnum::LIVE),
           "PROCESSING" => Ok(LocalPostStateEnum::PROCESSING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LocalPostStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LocalPostTopicTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The topic type of the post: standard, event, offer, or alert.
pub enum LocalPostTopicTypeEnum {
    

    /// No post type is specified.
    ///
    /// "LOCAL_POST_TOPIC_TYPE_UNSPECIFIED"
    #[serde(rename="LOCAL_POST_TOPIC_TYPE_UNSPECIFIED")]
    LOCALPOSTTOPICTYPEUNSPECIFIED,
    

    /// Post contains basic information, like summary and images.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// Post contains basic information and an event.
    ///
    /// "EVENT"
    #[serde(rename="EVENT")]
    EVENT,
    

    /// Post contains basic information, an event and offer related content (e.g.
coupon code)
    ///
    /// "OFFER"
    #[serde(rename="OFFER")]
    OFFER,
    

    /// High-priority, and timely announcements related to an ongoing event. These
types of posts are not always available for authoring.
    ///
    /// "ALERT"
    #[serde(rename="ALERT")]
    ALERT,
}

impl AsRef<str> for LocalPostTopicTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LocalPostTopicTypeEnum::LOCALPOSTTOPICTYPEUNSPECIFIED => "LOCAL_POST_TOPIC_TYPE_UNSPECIFIED",
            LocalPostTopicTypeEnum::STANDARD => "STANDARD",
            LocalPostTopicTypeEnum::EVENT => "EVENT",
            LocalPostTopicTypeEnum::OFFER => "OFFER",
            LocalPostTopicTypeEnum::ALERT => "ALERT",
        }
    }
}

impl std::convert::TryFrom< &str> for LocalPostTopicTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOCAL_POST_TOPIC_TYPE_UNSPECIFIED" => Ok(LocalPostTopicTypeEnum::LOCALPOSTTOPICTYPEUNSPECIFIED),
           "STANDARD" => Ok(LocalPostTopicTypeEnum::STANDARD),
           "EVENT" => Ok(LocalPostTopicTypeEnum::EVENT),
           "OFFER" => Ok(LocalPostTopicTypeEnum::OFFER),
           "ALERT" => Ok(LocalPostTopicTypeEnum::ALERT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LocalPostTopicTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LocationAssociationCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The category that this location photo belongs to.
pub enum LocationAssociationCategoryEnum {
    

    /// Unspecified category.
    ///
    /// "CATEGORY_UNSPECIFIED"
    #[serde(rename="CATEGORY_UNSPECIFIED")]
    CATEGORYUNSPECIFIED,
    

    /// Cover photo. A location has only one cover photo.
    ///
    /// "COVER"
    #[serde(rename="COVER")]
    COVER,
    

    /// Profile photo. A location has only one profile photo.
    ///
    /// "PROFILE"
    #[serde(rename="PROFILE")]
    PROFILE,
    

    /// Logo photo.
    ///
    /// "LOGO"
    #[serde(rename="LOGO")]
    LOGO,
    

    /// Exterior media.
    ///
    /// "EXTERIOR"
    #[serde(rename="EXTERIOR")]
    EXTERIOR,
    

    /// Interior media.
    ///
    /// "INTERIOR"
    #[serde(rename="INTERIOR")]
    INTERIOR,
    

    /// Product media.
    ///
    /// "PRODUCT"
    #[serde(rename="PRODUCT")]
    PRODUCT,
    

    /// 'At-work' media.
    ///
    /// "AT_WORK"
    #[serde(rename="AT_WORK")]
    ATWORK,
    

    /// Food and drink media.
    ///
    /// "FOOD_AND_DRINK"
    #[serde(rename="FOOD_AND_DRINK")]
    FOODANDDRINK,
    

    /// Menu media.
    ///
    /// "MENU"
    #[serde(rename="MENU")]
    MENU,
    

    /// Common area media.
    ///
    /// "COMMON_AREA"
    #[serde(rename="COMMON_AREA")]
    COMMONAREA,
    

    /// Rooms media.
    ///
    /// "ROOMS"
    #[serde(rename="ROOMS")]
    ROOMS,
    

    /// Teams media.
    ///
    /// "TEAMS"
    #[serde(rename="TEAMS")]
    TEAMS,
    

    /// Additional, uncategorized media.
    ///
    /// "ADDITIONAL"
    #[serde(rename="ADDITIONAL")]
    ADDITIONAL,
}

impl AsRef<str> for LocationAssociationCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LocationAssociationCategoryEnum::CATEGORYUNSPECIFIED => "CATEGORY_UNSPECIFIED",
            LocationAssociationCategoryEnum::COVER => "COVER",
            LocationAssociationCategoryEnum::PROFILE => "PROFILE",
            LocationAssociationCategoryEnum::LOGO => "LOGO",
            LocationAssociationCategoryEnum::EXTERIOR => "EXTERIOR",
            LocationAssociationCategoryEnum::INTERIOR => "INTERIOR",
            LocationAssociationCategoryEnum::PRODUCT => "PRODUCT",
            LocationAssociationCategoryEnum::ATWORK => "AT_WORK",
            LocationAssociationCategoryEnum::FOODANDDRINK => "FOOD_AND_DRINK",
            LocationAssociationCategoryEnum::MENU => "MENU",
            LocationAssociationCategoryEnum::COMMONAREA => "COMMON_AREA",
            LocationAssociationCategoryEnum::ROOMS => "ROOMS",
            LocationAssociationCategoryEnum::TEAMS => "TEAMS",
            LocationAssociationCategoryEnum::ADDITIONAL => "ADDITIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for LocationAssociationCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CATEGORY_UNSPECIFIED" => Ok(LocationAssociationCategoryEnum::CATEGORYUNSPECIFIED),
           "COVER" => Ok(LocationAssociationCategoryEnum::COVER),
           "PROFILE" => Ok(LocationAssociationCategoryEnum::PROFILE),
           "LOGO" => Ok(LocationAssociationCategoryEnum::LOGO),
           "EXTERIOR" => Ok(LocationAssociationCategoryEnum::EXTERIOR),
           "INTERIOR" => Ok(LocationAssociationCategoryEnum::INTERIOR),
           "PRODUCT" => Ok(LocationAssociationCategoryEnum::PRODUCT),
           "AT_WORK" => Ok(LocationAssociationCategoryEnum::ATWORK),
           "FOOD_AND_DRINK" => Ok(LocationAssociationCategoryEnum::FOODANDDRINK),
           "MENU" => Ok(LocationAssociationCategoryEnum::MENU),
           "COMMON_AREA" => Ok(LocationAssociationCategoryEnum::COMMONAREA),
           "ROOMS" => Ok(LocationAssociationCategoryEnum::ROOMS),
           "TEAMS" => Ok(LocationAssociationCategoryEnum::TEAMS),
           "ADDITIONAL" => Ok(LocationAssociationCategoryEnum::ADDITIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LocationAssociationCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediaItemMediaFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The format of this media item. Must be set when the media item is created,
and is read-only on all other requests. Cannot be updated.
pub enum MediaItemMediaFormatEnum {
    

    /// Format unspecified.
    ///
    /// "MEDIA_FORMAT_UNSPECIFIED"
    #[serde(rename="MEDIA_FORMAT_UNSPECIFIED")]
    MEDIAFORMATUNSPECIFIED,
    

    /// Media item is a photo. In this version, only photos are supported.
    ///
    /// "PHOTO"
    #[serde(rename="PHOTO")]
    PHOTO,
    

    /// Media item is a video.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
}

impl AsRef<str> for MediaItemMediaFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediaItemMediaFormatEnum::MEDIAFORMATUNSPECIFIED => "MEDIA_FORMAT_UNSPECIFIED",
            MediaItemMediaFormatEnum::PHOTO => "PHOTO",
            MediaItemMediaFormatEnum::VIDEO => "VIDEO",
        }
    }
}

impl std::convert::TryFrom< &str> for MediaItemMediaFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEDIA_FORMAT_UNSPECIFIED" => Ok(MediaItemMediaFormatEnum::MEDIAFORMATUNSPECIFIED),
           "PHOTO" => Ok(MediaItemMediaFormatEnum::PHOTO),
           "VIDEO" => Ok(MediaItemMediaFormatEnum::VIDEO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediaItemMediaFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricRequestMetricEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The requested metric.
pub enum MetricRequestMetricEnum {
    

    /// No metric specified.
    ///
    /// "METRIC_UNSPECIFIED"
    #[serde(rename="METRIC_UNSPECIFIED")]
    METRICUNSPECIFIED,
    

    /// Shorthand to request all available metrics. Which metrics are included in
ALL varies, and depends on the resource for which insights are being
requested.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// The number of times the resource was shown when searching for the location
directly.
    ///
    /// "QUERIES_DIRECT"
    #[serde(rename="QUERIES_DIRECT")]
    QUERIESDIRECT,
    

    /// The number of times the resource was shown as a result of a categorical
search (for example, restaurant).
    ///
    /// "QUERIES_INDIRECT"
    #[serde(rename="QUERIES_INDIRECT")]
    QUERIESINDIRECT,
    

    /// The number of times a resource was shown as a result of a search for the
chain it belongs to, or a brand it sells. For example, Starbucks, Adidas.
This is a subset of QUERIES_INDIRECT.
    ///
    /// "QUERIES_CHAIN"
    #[serde(rename="QUERIES_CHAIN")]
    QUERIESCHAIN,
    

    /// The number of times the resource was viewed on Google Maps.
    ///
    /// "VIEWS_MAPS"
    #[serde(rename="VIEWS_MAPS")]
    VIEWSMAPS,
    

    /// The number of times the resource was viewed on Google Search.
    ///
    /// "VIEWS_SEARCH"
    #[serde(rename="VIEWS_SEARCH")]
    VIEWSSEARCH,
    

    /// The number of times the website was clicked.
    ///
    /// "ACTIONS_WEBSITE"
    #[serde(rename="ACTIONS_WEBSITE")]
    ACTIONSWEBSITE,
    

    /// The number of times the phone number was clicked.
    ///
    /// "ACTIONS_PHONE"
    #[serde(rename="ACTIONS_PHONE")]
    ACTIONSPHONE,
    

    /// The number of times driving directions were requested.
    ///
    /// "ACTIONS_DRIVING_DIRECTIONS"
    #[serde(rename="ACTIONS_DRIVING_DIRECTIONS")]
    ACTIONSDRIVINGDIRECTIONS,
    

    /// The number of views on media items uploaded by the merchant.
    ///
    /// "PHOTOS_VIEWS_MERCHANT"
    #[serde(rename="PHOTOS_VIEWS_MERCHANT")]
    PHOTOSVIEWSMERCHANT,
    

    /// The number of views on media items uploaded by customers.
    ///
    /// "PHOTOS_VIEWS_CUSTOMERS"
    #[serde(rename="PHOTOS_VIEWS_CUSTOMERS")]
    PHOTOSVIEWSCUSTOMERS,
    

    /// The total number of media items that are currently live that have been
uploaded by the merchant.
    ///
    /// "PHOTOS_COUNT_MERCHANT"
    #[serde(rename="PHOTOS_COUNT_MERCHANT")]
    PHOTOSCOUNTMERCHANT,
    

    /// The total number of media items that are currently live that have been
uploaded by customers.
    ///
    /// "PHOTOS_COUNT_CUSTOMERS"
    #[serde(rename="PHOTOS_COUNT_CUSTOMERS")]
    PHOTOSCOUNTCUSTOMERS,
    

    /// The number of times the local post was viewed on Google Search.
    ///
    /// "LOCAL_POST_VIEWS_SEARCH"
    #[serde(rename="LOCAL_POST_VIEWS_SEARCH")]
    LOCALPOSTVIEWSSEARCH,
    

    /// The number of times the call to action button was clicked on Google.
    ///
    /// "LOCAL_POST_ACTIONS_CALL_TO_ACTION"
    #[serde(rename="LOCAL_POST_ACTIONS_CALL_TO_ACTION")]
    LOCALPOSTACTIONSCALLTOACTION,
}

impl AsRef<str> for MetricRequestMetricEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricRequestMetricEnum::METRICUNSPECIFIED => "METRIC_UNSPECIFIED",
            MetricRequestMetricEnum::ALL => "ALL",
            MetricRequestMetricEnum::QUERIESDIRECT => "QUERIES_DIRECT",
            MetricRequestMetricEnum::QUERIESINDIRECT => "QUERIES_INDIRECT",
            MetricRequestMetricEnum::QUERIESCHAIN => "QUERIES_CHAIN",
            MetricRequestMetricEnum::VIEWSMAPS => "VIEWS_MAPS",
            MetricRequestMetricEnum::VIEWSSEARCH => "VIEWS_SEARCH",
            MetricRequestMetricEnum::ACTIONSWEBSITE => "ACTIONS_WEBSITE",
            MetricRequestMetricEnum::ACTIONSPHONE => "ACTIONS_PHONE",
            MetricRequestMetricEnum::ACTIONSDRIVINGDIRECTIONS => "ACTIONS_DRIVING_DIRECTIONS",
            MetricRequestMetricEnum::PHOTOSVIEWSMERCHANT => "PHOTOS_VIEWS_MERCHANT",
            MetricRequestMetricEnum::PHOTOSVIEWSCUSTOMERS => "PHOTOS_VIEWS_CUSTOMERS",
            MetricRequestMetricEnum::PHOTOSCOUNTMERCHANT => "PHOTOS_COUNT_MERCHANT",
            MetricRequestMetricEnum::PHOTOSCOUNTCUSTOMERS => "PHOTOS_COUNT_CUSTOMERS",
            MetricRequestMetricEnum::LOCALPOSTVIEWSSEARCH => "LOCAL_POST_VIEWS_SEARCH",
            MetricRequestMetricEnum::LOCALPOSTACTIONSCALLTOACTION => "LOCAL_POST_ACTIONS_CALL_TO_ACTION",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricRequestMetricEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_UNSPECIFIED" => Ok(MetricRequestMetricEnum::METRICUNSPECIFIED),
           "ALL" => Ok(MetricRequestMetricEnum::ALL),
           "QUERIES_DIRECT" => Ok(MetricRequestMetricEnum::QUERIESDIRECT),
           "QUERIES_INDIRECT" => Ok(MetricRequestMetricEnum::QUERIESINDIRECT),
           "QUERIES_CHAIN" => Ok(MetricRequestMetricEnum::QUERIESCHAIN),
           "VIEWS_MAPS" => Ok(MetricRequestMetricEnum::VIEWSMAPS),
           "VIEWS_SEARCH" => Ok(MetricRequestMetricEnum::VIEWSSEARCH),
           "ACTIONS_WEBSITE" => Ok(MetricRequestMetricEnum::ACTIONSWEBSITE),
           "ACTIONS_PHONE" => Ok(MetricRequestMetricEnum::ACTIONSPHONE),
           "ACTIONS_DRIVING_DIRECTIONS" => Ok(MetricRequestMetricEnum::ACTIONSDRIVINGDIRECTIONS),
           "PHOTOS_VIEWS_MERCHANT" => Ok(MetricRequestMetricEnum::PHOTOSVIEWSMERCHANT),
           "PHOTOS_VIEWS_CUSTOMERS" => Ok(MetricRequestMetricEnum::PHOTOSVIEWSCUSTOMERS),
           "PHOTOS_COUNT_MERCHANT" => Ok(MetricRequestMetricEnum::PHOTOSCOUNTMERCHANT),
           "PHOTOS_COUNT_CUSTOMERS" => Ok(MetricRequestMetricEnum::PHOTOSCOUNTCUSTOMERS),
           "LOCAL_POST_VIEWS_SEARCH" => Ok(MetricRequestMetricEnum::LOCALPOSTVIEWSSEARCH),
           "LOCAL_POST_ACTIONS_CALL_TO_ACTION" => Ok(MetricRequestMetricEnum::LOCALPOSTACTIONSCALLTOACTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricRequestMetricEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricRequestOptionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the values should appear when returned.
pub enum MetricRequestOptionsEnum {
    

    /// No metric option specified. Will default to AGGREGATED_TOTAL in a request.
    ///
    /// "METRIC_OPTION_UNSPECIFIED"
    #[serde(rename="METRIC_OPTION_UNSPECIFIED")]
    METRICOPTIONUNSPECIFIED,
    

    /// Return values aggregated over the entire time frame.
This is the default value.
    ///
    /// "AGGREGATED_TOTAL"
    #[serde(rename="AGGREGATED_TOTAL")]
    AGGREGATEDTOTAL,
    

    /// Return daily timestamped values across time range.
    ///
    /// "AGGREGATED_DAILY"
    #[serde(rename="AGGREGATED_DAILY")]
    AGGREGATEDDAILY,
    

    /// Values will be returned as a breakdown by day of the week.
Only valid for ACTIONS_PHONE.
    ///
    /// "BREAKDOWN_DAY_OF_WEEK"
    #[serde(rename="BREAKDOWN_DAY_OF_WEEK")]
    BREAKDOWNDAYOFWEEK,
    

    /// Values will be returned as a breakdown by hour of the day.
Only valid for ACTIONS_PHONE.
    ///
    /// "BREAKDOWN_HOUR_OF_DAY"
    #[serde(rename="BREAKDOWN_HOUR_OF_DAY")]
    BREAKDOWNHOUROFDAY,
}

impl AsRef<str> for MetricRequestOptionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricRequestOptionsEnum::METRICOPTIONUNSPECIFIED => "METRIC_OPTION_UNSPECIFIED",
            MetricRequestOptionsEnum::AGGREGATEDTOTAL => "AGGREGATED_TOTAL",
            MetricRequestOptionsEnum::AGGREGATEDDAILY => "AGGREGATED_DAILY",
            MetricRequestOptionsEnum::BREAKDOWNDAYOFWEEK => "BREAKDOWN_DAY_OF_WEEK",
            MetricRequestOptionsEnum::BREAKDOWNHOUROFDAY => "BREAKDOWN_HOUR_OF_DAY",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricRequestOptionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_OPTION_UNSPECIFIED" => Ok(MetricRequestOptionsEnum::METRICOPTIONUNSPECIFIED),
           "AGGREGATED_TOTAL" => Ok(MetricRequestOptionsEnum::AGGREGATEDTOTAL),
           "AGGREGATED_DAILY" => Ok(MetricRequestOptionsEnum::AGGREGATEDDAILY),
           "BREAKDOWN_DAY_OF_WEEK" => Ok(MetricRequestOptionsEnum::BREAKDOWNDAYOFWEEK),
           "BREAKDOWN_HOUR_OF_DAY" => Ok(MetricRequestOptionsEnum::BREAKDOWNHOUROFDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricRequestOptionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricValueMetricEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The metric for which the value applies.
pub enum MetricValueMetricEnum {
    

    /// No metric specified.
    ///
    /// "METRIC_UNSPECIFIED"
    #[serde(rename="METRIC_UNSPECIFIED")]
    METRICUNSPECIFIED,
    

    /// Shorthand to request all available metrics. Which metrics are included in
ALL varies, and depends on the resource for which insights are being
requested.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// The number of times the resource was shown when searching for the location
directly.
    ///
    /// "QUERIES_DIRECT"
    #[serde(rename="QUERIES_DIRECT")]
    QUERIESDIRECT,
    

    /// The number of times the resource was shown as a result of a categorical
search (for example, restaurant).
    ///
    /// "QUERIES_INDIRECT"
    #[serde(rename="QUERIES_INDIRECT")]
    QUERIESINDIRECT,
    

    /// The number of times a resource was shown as a result of a search for the
chain it belongs to, or a brand it sells. For example, Starbucks, Adidas.
This is a subset of QUERIES_INDIRECT.
    ///
    /// "QUERIES_CHAIN"
    #[serde(rename="QUERIES_CHAIN")]
    QUERIESCHAIN,
    

    /// The number of times the resource was viewed on Google Maps.
    ///
    /// "VIEWS_MAPS"
    #[serde(rename="VIEWS_MAPS")]
    VIEWSMAPS,
    

    /// The number of times the resource was viewed on Google Search.
    ///
    /// "VIEWS_SEARCH"
    #[serde(rename="VIEWS_SEARCH")]
    VIEWSSEARCH,
    

    /// The number of times the website was clicked.
    ///
    /// "ACTIONS_WEBSITE"
    #[serde(rename="ACTIONS_WEBSITE")]
    ACTIONSWEBSITE,
    

    /// The number of times the phone number was clicked.
    ///
    /// "ACTIONS_PHONE"
    #[serde(rename="ACTIONS_PHONE")]
    ACTIONSPHONE,
    

    /// The number of times driving directions were requested.
    ///
    /// "ACTIONS_DRIVING_DIRECTIONS"
    #[serde(rename="ACTIONS_DRIVING_DIRECTIONS")]
    ACTIONSDRIVINGDIRECTIONS,
    

    /// The number of views on media items uploaded by the merchant.
    ///
    /// "PHOTOS_VIEWS_MERCHANT"
    #[serde(rename="PHOTOS_VIEWS_MERCHANT")]
    PHOTOSVIEWSMERCHANT,
    

    /// The number of views on media items uploaded by customers.
    ///
    /// "PHOTOS_VIEWS_CUSTOMERS"
    #[serde(rename="PHOTOS_VIEWS_CUSTOMERS")]
    PHOTOSVIEWSCUSTOMERS,
    

    /// The total number of media items that are currently live that have been
uploaded by the merchant.
    ///
    /// "PHOTOS_COUNT_MERCHANT"
    #[serde(rename="PHOTOS_COUNT_MERCHANT")]
    PHOTOSCOUNTMERCHANT,
    

    /// The total number of media items that are currently live that have been
uploaded by customers.
    ///
    /// "PHOTOS_COUNT_CUSTOMERS"
    #[serde(rename="PHOTOS_COUNT_CUSTOMERS")]
    PHOTOSCOUNTCUSTOMERS,
    

    /// The number of times the local post was viewed on Google Search.
    ///
    /// "LOCAL_POST_VIEWS_SEARCH"
    #[serde(rename="LOCAL_POST_VIEWS_SEARCH")]
    LOCALPOSTVIEWSSEARCH,
    

    /// The number of times the call to action button was clicked on Google.
    ///
    /// "LOCAL_POST_ACTIONS_CALL_TO_ACTION"
    #[serde(rename="LOCAL_POST_ACTIONS_CALL_TO_ACTION")]
    LOCALPOSTACTIONSCALLTOACTION,
}

impl AsRef<str> for MetricValueMetricEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricValueMetricEnum::METRICUNSPECIFIED => "METRIC_UNSPECIFIED",
            MetricValueMetricEnum::ALL => "ALL",
            MetricValueMetricEnum::QUERIESDIRECT => "QUERIES_DIRECT",
            MetricValueMetricEnum::QUERIESINDIRECT => "QUERIES_INDIRECT",
            MetricValueMetricEnum::QUERIESCHAIN => "QUERIES_CHAIN",
            MetricValueMetricEnum::VIEWSMAPS => "VIEWS_MAPS",
            MetricValueMetricEnum::VIEWSSEARCH => "VIEWS_SEARCH",
            MetricValueMetricEnum::ACTIONSWEBSITE => "ACTIONS_WEBSITE",
            MetricValueMetricEnum::ACTIONSPHONE => "ACTIONS_PHONE",
            MetricValueMetricEnum::ACTIONSDRIVINGDIRECTIONS => "ACTIONS_DRIVING_DIRECTIONS",
            MetricValueMetricEnum::PHOTOSVIEWSMERCHANT => "PHOTOS_VIEWS_MERCHANT",
            MetricValueMetricEnum::PHOTOSVIEWSCUSTOMERS => "PHOTOS_VIEWS_CUSTOMERS",
            MetricValueMetricEnum::PHOTOSCOUNTMERCHANT => "PHOTOS_COUNT_MERCHANT",
            MetricValueMetricEnum::PHOTOSCOUNTCUSTOMERS => "PHOTOS_COUNT_CUSTOMERS",
            MetricValueMetricEnum::LOCALPOSTVIEWSSEARCH => "LOCAL_POST_VIEWS_SEARCH",
            MetricValueMetricEnum::LOCALPOSTACTIONSCALLTOACTION => "LOCAL_POST_ACTIONS_CALL_TO_ACTION",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricValueMetricEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_UNSPECIFIED" => Ok(MetricValueMetricEnum::METRICUNSPECIFIED),
           "ALL" => Ok(MetricValueMetricEnum::ALL),
           "QUERIES_DIRECT" => Ok(MetricValueMetricEnum::QUERIESDIRECT),
           "QUERIES_INDIRECT" => Ok(MetricValueMetricEnum::QUERIESINDIRECT),
           "QUERIES_CHAIN" => Ok(MetricValueMetricEnum::QUERIESCHAIN),
           "VIEWS_MAPS" => Ok(MetricValueMetricEnum::VIEWSMAPS),
           "VIEWS_SEARCH" => Ok(MetricValueMetricEnum::VIEWSSEARCH),
           "ACTIONS_WEBSITE" => Ok(MetricValueMetricEnum::ACTIONSWEBSITE),
           "ACTIONS_PHONE" => Ok(MetricValueMetricEnum::ACTIONSPHONE),
           "ACTIONS_DRIVING_DIRECTIONS" => Ok(MetricValueMetricEnum::ACTIONSDRIVINGDIRECTIONS),
           "PHOTOS_VIEWS_MERCHANT" => Ok(MetricValueMetricEnum::PHOTOSVIEWSMERCHANT),
           "PHOTOS_VIEWS_CUSTOMERS" => Ok(MetricValueMetricEnum::PHOTOSVIEWSCUSTOMERS),
           "PHOTOS_COUNT_MERCHANT" => Ok(MetricValueMetricEnum::PHOTOSCOUNTMERCHANT),
           "PHOTOS_COUNT_CUSTOMERS" => Ok(MetricValueMetricEnum::PHOTOSCOUNTCUSTOMERS),
           "LOCAL_POST_VIEWS_SEARCH" => Ok(MetricValueMetricEnum::LOCALPOSTVIEWSSEARCH),
           "LOCAL_POST_ACTIONS_CALL_TO_ACTION" => Ok(MetricValueMetricEnum::LOCALPOSTACTIONSCALLTOACTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricValueMetricEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NotificationNotificationTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The types of notifications that will be sent to the Cloud Pub/Sub topic.
At least one must be specified. To stop receiving notifications entirely,
use DeleteNotifications.
pub enum NotificationNotificationTypesEnum {
    

    /// No notification type. Will not match any notifications.
    ///
    /// "NOTIFICATION_TYPE_UNSPECIFIED"
    #[serde(rename="NOTIFICATION_TYPE_UNSPECIFIED")]
    NOTIFICATIONTYPEUNSPECIFIED,
    

    /// The location has Google updates for review. The location_name field on the
notification will provide the resource name of the location with Google
updates.
    ///
    /// "GOOGLE_UPDATE"
    #[serde(rename="GOOGLE_UPDATE")]
    GOOGLEUPDATE,
    

    /// A new review has been added to the location. The review_name field on the
notification will provide the resource name of the review that was added,
and location_name will have the location's resource name.
    ///
    /// "NEW_REVIEW"
    #[serde(rename="NEW_REVIEW")]
    NEWREVIEW,
    

    /// A review on the location has been updated. The review_name field on the
notification will provide the resource name of the review that was added,
and location_name will have the location's resource name.
    ///
    /// "UPDATED_REVIEW"
    #[serde(rename="UPDATED_REVIEW")]
    UPDATEDREVIEW,
    

    /// A new media item has been added to the location by a Google Maps user.
The notification will provide the resource name of the new media item.
    ///
    /// "NEW_CUSTOMER_MEDIA"
    #[serde(rename="NEW_CUSTOMER_MEDIA")]
    NEWCUSTOMERMEDIA,
    

    /// A new question is added to the location. The notification will provide the
resource name of question.
    ///
    /// "NEW_QUESTION"
    #[serde(rename="NEW_QUESTION")]
    NEWQUESTION,
    

    /// A question of the location is updated. The notification will provide the
resource name of question.
    ///
    /// "UPDATED_QUESTION"
    #[serde(rename="UPDATED_QUESTION")]
    UPDATEDQUESTION,
    

    /// A new answer is added to the location. The notification will provide the
resource name of question and answer.
    ///
    /// "NEW_ANSWER"
    #[serde(rename="NEW_ANSWER")]
    NEWANSWER,
    

    /// An answer of the location is updated.  The notification will provide the
resource name of question and answer.
    ///
    /// "UPDATED_ANSWER"
    #[serde(rename="UPDATED_ANSWER")]
    UPDATEDANSWER,
    

    /// The LocationState of the Location was updated. The Notification will
contain the field mask of the updated LocationState fields.
    ///
    /// "UPDATED_LOCATION_STATE"
    #[serde(rename="UPDATED_LOCATION_STATE")]
    UPDATEDLOCATIONSTATE,
}

impl AsRef<str> for NotificationNotificationTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NotificationNotificationTypesEnum::NOTIFICATIONTYPEUNSPECIFIED => "NOTIFICATION_TYPE_UNSPECIFIED",
            NotificationNotificationTypesEnum::GOOGLEUPDATE => "GOOGLE_UPDATE",
            NotificationNotificationTypesEnum::NEWREVIEW => "NEW_REVIEW",
            NotificationNotificationTypesEnum::UPDATEDREVIEW => "UPDATED_REVIEW",
            NotificationNotificationTypesEnum::NEWCUSTOMERMEDIA => "NEW_CUSTOMER_MEDIA",
            NotificationNotificationTypesEnum::NEWQUESTION => "NEW_QUESTION",
            NotificationNotificationTypesEnum::UPDATEDQUESTION => "UPDATED_QUESTION",
            NotificationNotificationTypesEnum::NEWANSWER => "NEW_ANSWER",
            NotificationNotificationTypesEnum::UPDATEDANSWER => "UPDATED_ANSWER",
            NotificationNotificationTypesEnum::UPDATEDLOCATIONSTATE => "UPDATED_LOCATION_STATE",
        }
    }
}

impl std::convert::TryFrom< &str> for NotificationNotificationTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOTIFICATION_TYPE_UNSPECIFIED" => Ok(NotificationNotificationTypesEnum::NOTIFICATIONTYPEUNSPECIFIED),
           "GOOGLE_UPDATE" => Ok(NotificationNotificationTypesEnum::GOOGLEUPDATE),
           "NEW_REVIEW" => Ok(NotificationNotificationTypesEnum::NEWREVIEW),
           "UPDATED_REVIEW" => Ok(NotificationNotificationTypesEnum::UPDATEDREVIEW),
           "NEW_CUSTOMER_MEDIA" => Ok(NotificationNotificationTypesEnum::NEWCUSTOMERMEDIA),
           "NEW_QUESTION" => Ok(NotificationNotificationTypesEnum::NEWQUESTION),
           "UPDATED_QUESTION" => Ok(NotificationNotificationTypesEnum::UPDATEDQUESTION),
           "NEW_ANSWER" => Ok(NotificationNotificationTypesEnum::NEWANSWER),
           "UPDATED_ANSWER" => Ok(NotificationNotificationTypesEnum::UPDATEDANSWER),
           "UPDATED_LOCATION_STATE" => Ok(NotificationNotificationTypesEnum::UPDATEDLOCATIONSTATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NotificationNotificationTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OpenInfoStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether or not the Location is currently open for business.
All locations are open by default, unless updated to be closed.
pub enum OpenInfoStatusEnum {
    

    /// Not specified.
    ///
    /// "OPEN_FOR_BUSINESS_UNSPECIFIED"
    #[serde(rename="OPEN_FOR_BUSINESS_UNSPECIFIED")]
    OPENFORBUSINESSUNSPECIFIED,
    

    /// Indicates that the location is open.
    ///
    /// "OPEN"
    #[serde(rename="OPEN")]
    OPEN,
    

    /// Indicates that the location has been permanently closed.
    ///
    /// "CLOSED_PERMANENTLY"
    #[serde(rename="CLOSED_PERMANENTLY")]
    CLOSEDPERMANENTLY,
    

    /// Indicates that the location has been temporarily closed.
This value may only be applied to published locations (i.e.
location_state.is_published = true).
When updating the status field to this value, clients are required to set
the `update_mask` explicitly to `open_info.status`. No other update masks
can be set during this update call. This is a temporary restriction which
will be relaxed soon.
    ///
    /// "CLOSED_TEMPORARILY"
    #[serde(rename="CLOSED_TEMPORARILY")]
    CLOSEDTEMPORARILY,
}

impl AsRef<str> for OpenInfoStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OpenInfoStatusEnum::OPENFORBUSINESSUNSPECIFIED => "OPEN_FOR_BUSINESS_UNSPECIFIED",
            OpenInfoStatusEnum::OPEN => "OPEN",
            OpenInfoStatusEnum::CLOSEDPERMANENTLY => "CLOSED_PERMANENTLY",
            OpenInfoStatusEnum::CLOSEDTEMPORARILY => "CLOSED_TEMPORARILY",
        }
    }
}

impl std::convert::TryFrom< &str> for OpenInfoStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPEN_FOR_BUSINESS_UNSPECIFIED" => Ok(OpenInfoStatusEnum::OPENFORBUSINESSUNSPECIFIED),
           "OPEN" => Ok(OpenInfoStatusEnum::OPEN),
           "CLOSED_PERMANENTLY" => Ok(OpenInfoStatusEnum::CLOSEDPERMANENTLY),
           "CLOSED_TEMPORARILY" => Ok(OpenInfoStatusEnum::CLOSEDTEMPORARILY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OpenInfoStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportGoogleLocationRequestReportReasonBadLocationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason for which the user is reporting this location when the issue
is with the location itself.
pub enum ReportGoogleLocationRequestReportReasonBadLocationEnum {
    

    /// Not specified.
    ///
    /// "BAD_LOCATION_REASON_UNSPECIFIED"
    #[serde(rename="BAD_LOCATION_REASON_UNSPECIFIED")]
    BADLOCATIONREASONUNSPECIFIED,
    

    /// The recommended location is not an actual location.
    ///
    /// "NOT_A_LOCATION"
    #[serde(rename="NOT_A_LOCATION")]
    NOTALOCATION,
    

    /// The recommended location is permanently closed.
    ///
    /// "PERMANENTLY_CLOSED"
    #[serde(rename="PERMANENTLY_CLOSED")]
    PERMANENTLYCLOSED,
    

    /// The recommended location does not exist.
    ///
    /// "DOES_NOT_EXIST"
    #[serde(rename="DOES_NOT_EXIST")]
    DOESNOTEXIST,
    

    /// The recommended location is spam, fake or offensive.
    ///
    /// "SPAM"
    #[serde(rename="SPAM")]
    SPAM,
    

    /// The recommended location is a private place or home.
    ///
    /// "NOT_A_BUSINESS"
    #[serde(rename="NOT_A_BUSINESS")]
    NOTABUSINESS,
    

    /// The recommended location has moved to a new location.

Should be accompanied by a `report_reason_elaboration` specifying the new
address.
    ///
    /// "MOVED"
    #[serde(rename="MOVED")]
    MOVED,
    

    /// The recommended location is a duplicate of another location.
    ///
    /// "DUPLICATE"
    #[serde(rename="DUPLICATE")]
    DUPLICATE,
}

impl AsRef<str> for ReportGoogleLocationRequestReportReasonBadLocationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportGoogleLocationRequestReportReasonBadLocationEnum::BADLOCATIONREASONUNSPECIFIED => "BAD_LOCATION_REASON_UNSPECIFIED",
            ReportGoogleLocationRequestReportReasonBadLocationEnum::NOTALOCATION => "NOT_A_LOCATION",
            ReportGoogleLocationRequestReportReasonBadLocationEnum::PERMANENTLYCLOSED => "PERMANENTLY_CLOSED",
            ReportGoogleLocationRequestReportReasonBadLocationEnum::DOESNOTEXIST => "DOES_NOT_EXIST",
            ReportGoogleLocationRequestReportReasonBadLocationEnum::SPAM => "SPAM",
            ReportGoogleLocationRequestReportReasonBadLocationEnum::NOTABUSINESS => "NOT_A_BUSINESS",
            ReportGoogleLocationRequestReportReasonBadLocationEnum::MOVED => "MOVED",
            ReportGoogleLocationRequestReportReasonBadLocationEnum::DUPLICATE => "DUPLICATE",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportGoogleLocationRequestReportReasonBadLocationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BAD_LOCATION_REASON_UNSPECIFIED" => Ok(ReportGoogleLocationRequestReportReasonBadLocationEnum::BADLOCATIONREASONUNSPECIFIED),
           "NOT_A_LOCATION" => Ok(ReportGoogleLocationRequestReportReasonBadLocationEnum::NOTALOCATION),
           "PERMANENTLY_CLOSED" => Ok(ReportGoogleLocationRequestReportReasonBadLocationEnum::PERMANENTLYCLOSED),
           "DOES_NOT_EXIST" => Ok(ReportGoogleLocationRequestReportReasonBadLocationEnum::DOESNOTEXIST),
           "SPAM" => Ok(ReportGoogleLocationRequestReportReasonBadLocationEnum::SPAM),
           "NOT_A_BUSINESS" => Ok(ReportGoogleLocationRequestReportReasonBadLocationEnum::NOTABUSINESS),
           "MOVED" => Ok(ReportGoogleLocationRequestReportReasonBadLocationEnum::MOVED),
           "DUPLICATE" => Ok(ReportGoogleLocationRequestReportReasonBadLocationEnum::DUPLICATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportGoogleLocationRequestReportReasonBadLocationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportGoogleLocationRequestReportReasonBadRecommendationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason for which the user is reporting this location when the issue
is with the recommendation. This report is useful if the location has
been recommended to the GMB account.
pub enum ReportGoogleLocationRequestReportReasonBadRecommendationEnum {
    

    /// Not specified.
    ///
    /// "BAD_RECOMMENDATION_REASON_UNSPECIFIED"
    #[serde(rename="BAD_RECOMMENDATION_REASON_UNSPECIFIED")]
    BADRECOMMENDATIONREASONUNSPECIFIED,
    

    /// The recommended location is not a store front.
    ///
    /// "NOT_A_STORE_FRONT"
    #[serde(rename="NOT_A_STORE_FRONT")]
    NOTASTOREFRONT,
    

    /// The recommended location doesn't belong to the chain suggested in
the `chain_display_name` in the returned location.

Should be accompanied by a `report_reason_elaboration` specifying the
name of the correct chain.
    ///
    /// "NOT_PART_OF_SUGGESTED_CHAIN"
    #[serde(rename="NOT_PART_OF_SUGGESTED_CHAIN")]
    NOTPARTOFSUGGESTEDCHAIN,
    

    /// The recommended location is not relevant to the user.

Should be accompanied by a `report_reason_elaboration` for why the
recommendation is irrelevant.
    ///
    /// "IRRELEVANT"
    #[serde(rename="IRRELEVANT")]
    IRRELEVANT,
}

impl AsRef<str> for ReportGoogleLocationRequestReportReasonBadRecommendationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportGoogleLocationRequestReportReasonBadRecommendationEnum::BADRECOMMENDATIONREASONUNSPECIFIED => "BAD_RECOMMENDATION_REASON_UNSPECIFIED",
            ReportGoogleLocationRequestReportReasonBadRecommendationEnum::NOTASTOREFRONT => "NOT_A_STORE_FRONT",
            ReportGoogleLocationRequestReportReasonBadRecommendationEnum::NOTPARTOFSUGGESTEDCHAIN => "NOT_PART_OF_SUGGESTED_CHAIN",
            ReportGoogleLocationRequestReportReasonBadRecommendationEnum::IRRELEVANT => "IRRELEVANT",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportGoogleLocationRequestReportReasonBadRecommendationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BAD_RECOMMENDATION_REASON_UNSPECIFIED" => Ok(ReportGoogleLocationRequestReportReasonBadRecommendationEnum::BADRECOMMENDATIONREASONUNSPECIFIED),
           "NOT_A_STORE_FRONT" => Ok(ReportGoogleLocationRequestReportReasonBadRecommendationEnum::NOTASTOREFRONT),
           "NOT_PART_OF_SUGGESTED_CHAIN" => Ok(ReportGoogleLocationRequestReportReasonBadRecommendationEnum::NOTPARTOFSUGGESTEDCHAIN),
           "IRRELEVANT" => Ok(ReportGoogleLocationRequestReportReasonBadRecommendationEnum::IRRELEVANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportGoogleLocationRequestReportReasonBadRecommendationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReviewStarRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The star rating of the review.
pub enum ReviewStarRatingEnum {
    

    /// Not specified.
    ///
    /// "STAR_RATING_UNSPECIFIED"
    #[serde(rename="STAR_RATING_UNSPECIFIED")]
    STARRATINGUNSPECIFIED,
    

    /// One star out of a maximum of five.
    ///
    /// "ONE"
    #[serde(rename="ONE")]
    ONE,
    

    /// Two stars out of a maximum of five.
    ///
    /// "TWO"
    #[serde(rename="TWO")]
    TWO,
    

    /// Three stars out of a maximum of five.
    ///
    /// "THREE"
    #[serde(rename="THREE")]
    THREE,
    

    /// Four stars out of a maximum of five.
    ///
    /// "FOUR"
    #[serde(rename="FOUR")]
    FOUR,
    

    /// The maximum star rating.
    ///
    /// "FIVE"
    #[serde(rename="FIVE")]
    FIVE,
}

impl AsRef<str> for ReviewStarRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReviewStarRatingEnum::STARRATINGUNSPECIFIED => "STAR_RATING_UNSPECIFIED",
            ReviewStarRatingEnum::ONE => "ONE",
            ReviewStarRatingEnum::TWO => "TWO",
            ReviewStarRatingEnum::THREE => "THREE",
            ReviewStarRatingEnum::FOUR => "FOUR",
            ReviewStarRatingEnum::FIVE => "FIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ReviewStarRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STAR_RATING_UNSPECIFIED" => Ok(ReviewStarRatingEnum::STARRATINGUNSPECIFIED),
           "ONE" => Ok(ReviewStarRatingEnum::ONE),
           "TWO" => Ok(ReviewStarRatingEnum::TWO),
           "THREE" => Ok(ReviewStarRatingEnum::THREE),
           "FOUR" => Ok(ReviewStarRatingEnum::FOUR),
           "FIVE" => Ok(ReviewStarRatingEnum::FIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReviewStarRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SectionSectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Type of the current price list section. Default value is FOOD.
pub enum SectionSectionTypeEnum {
    

    /// Not specified.
    ///
    /// "SECTION_TYPE_UNSPECIFIED"
    #[serde(rename="SECTION_TYPE_UNSPECIFIED")]
    SECTIONTYPEUNSPECIFIED,
    

    /// Section contains items that represent food.
    ///
    /// "FOOD"
    #[serde(rename="FOOD")]
    FOOD,
    

    /// Section contains items that represent services.
    ///
    /// "SERVICES"
    #[serde(rename="SERVICES")]
    SERVICES,
}

impl AsRef<str> for SectionSectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SectionSectionTypeEnum::SECTIONTYPEUNSPECIFIED => "SECTION_TYPE_UNSPECIFIED",
            SectionSectionTypeEnum::FOOD => "FOOD",
            SectionSectionTypeEnum::SERVICES => "SERVICES",
        }
    }
}

impl std::convert::TryFrom< &str> for SectionSectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_TYPE_UNSPECIFIED" => Ok(SectionSectionTypeEnum::SECTIONTYPEUNSPECIFIED),
           "FOOD" => Ok(SectionSectionTypeEnum::FOOD),
           "SERVICES" => Ok(SectionSectionTypeEnum::SERVICES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SectionSectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceAreaBusinesBusinessTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the type of the service area
business.
pub enum ServiceAreaBusinesBusinessTypeEnum {
    

    /// Output only. Not specified.
    ///
    /// "BUSINESS_TYPE_UNSPECIFIED"
    #[serde(rename="BUSINESS_TYPE_UNSPECIFIED")]
    BUSINESSTYPEUNSPECIFIED,
    

    /// Offers service only in the surrounding area (not at the business
address).
    ///
    /// "CUSTOMER_LOCATION_ONLY"
    #[serde(rename="CUSTOMER_LOCATION_ONLY")]
    CUSTOMERLOCATIONONLY,
    

    /// Offers service at the business address and the surrounding area.
    ///
    /// "CUSTOMER_AND_BUSINESS_LOCATION"
    #[serde(rename="CUSTOMER_AND_BUSINESS_LOCATION")]
    CUSTOMERANDBUSINESSLOCATION,
}

impl AsRef<str> for ServiceAreaBusinesBusinessTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceAreaBusinesBusinessTypeEnum::BUSINESSTYPEUNSPECIFIED => "BUSINESS_TYPE_UNSPECIFIED",
            ServiceAreaBusinesBusinessTypeEnum::CUSTOMERLOCATIONONLY => "CUSTOMER_LOCATION_ONLY",
            ServiceAreaBusinesBusinessTypeEnum::CUSTOMERANDBUSINESSLOCATION => "CUSTOMER_AND_BUSINESS_LOCATION",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceAreaBusinesBusinessTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUSINESS_TYPE_UNSPECIFIED" => Ok(ServiceAreaBusinesBusinessTypeEnum::BUSINESSTYPEUNSPECIFIED),
           "CUSTOMER_LOCATION_ONLY" => Ok(ServiceAreaBusinesBusinessTypeEnum::CUSTOMERLOCATIONONLY),
           "CUSTOMER_AND_BUSINESS_LOCATION" => Ok(ServiceAreaBusinesBusinessTypeEnum::CUSTOMERANDBUSINESSLOCATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceAreaBusinesBusinessTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TimeDimensionDayOfWeekEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The day of the week ("MONDAY" to "SUNDAY") this value corresponds to.
Set for BREAKDOWN_DAY_OF_WEEK option.
pub enum TimeDimensionDayOfWeekEnum {
    

    /// The day of the week is unspecified.
    ///
    /// "DAY_OF_WEEK_UNSPECIFIED"
    #[serde(rename="DAY_OF_WEEK_UNSPECIFIED")]
    DAYOFWEEKUNSPECIFIED,
    

    /// Monday
    ///
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    

    /// Tuesday
    ///
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    

    /// Wednesday
    ///
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    

    /// Thursday
    ///
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    

    /// Friday
    ///
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    

    /// Saturday
    ///
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
    

    /// Sunday
    ///
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for TimeDimensionDayOfWeekEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TimeDimensionDayOfWeekEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            TimeDimensionDayOfWeekEnum::MONDAY => "MONDAY",
            TimeDimensionDayOfWeekEnum::TUESDAY => "TUESDAY",
            TimeDimensionDayOfWeekEnum::WEDNESDAY => "WEDNESDAY",
            TimeDimensionDayOfWeekEnum::THURSDAY => "THURSDAY",
            TimeDimensionDayOfWeekEnum::FRIDAY => "FRIDAY",
            TimeDimensionDayOfWeekEnum::SATURDAY => "SATURDAY",
            TimeDimensionDayOfWeekEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for TimeDimensionDayOfWeekEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(TimeDimensionDayOfWeekEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(TimeDimensionDayOfWeekEnum::MONDAY),
           "TUESDAY" => Ok(TimeDimensionDayOfWeekEnum::TUESDAY),
           "WEDNESDAY" => Ok(TimeDimensionDayOfWeekEnum::WEDNESDAY),
           "THURSDAY" => Ok(TimeDimensionDayOfWeekEnum::THURSDAY),
           "FRIDAY" => Ok(TimeDimensionDayOfWeekEnum::FRIDAY),
           "SATURDAY" => Ok(TimeDimensionDayOfWeekEnum::SATURDAY),
           "SUNDAY" => Ok(TimeDimensionDayOfWeekEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TimeDimensionDayOfWeekEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TimePeriodCloseDayEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the day of the week this period ends
on.
pub enum TimePeriodCloseDayEnum {
    

    /// The day of the week is unspecified.
    ///
    /// "DAY_OF_WEEK_UNSPECIFIED"
    #[serde(rename="DAY_OF_WEEK_UNSPECIFIED")]
    DAYOFWEEKUNSPECIFIED,
    

    /// Monday
    ///
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    

    /// Tuesday
    ///
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    

    /// Wednesday
    ///
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    

    /// Thursday
    ///
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    

    /// Friday
    ///
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    

    /// Saturday
    ///
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
    

    /// Sunday
    ///
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for TimePeriodCloseDayEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TimePeriodCloseDayEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            TimePeriodCloseDayEnum::MONDAY => "MONDAY",
            TimePeriodCloseDayEnum::TUESDAY => "TUESDAY",
            TimePeriodCloseDayEnum::WEDNESDAY => "WEDNESDAY",
            TimePeriodCloseDayEnum::THURSDAY => "THURSDAY",
            TimePeriodCloseDayEnum::FRIDAY => "FRIDAY",
            TimePeriodCloseDayEnum::SATURDAY => "SATURDAY",
            TimePeriodCloseDayEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for TimePeriodCloseDayEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(TimePeriodCloseDayEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(TimePeriodCloseDayEnum::MONDAY),
           "TUESDAY" => Ok(TimePeriodCloseDayEnum::TUESDAY),
           "WEDNESDAY" => Ok(TimePeriodCloseDayEnum::WEDNESDAY),
           "THURSDAY" => Ok(TimePeriodCloseDayEnum::THURSDAY),
           "FRIDAY" => Ok(TimePeriodCloseDayEnum::FRIDAY),
           "SATURDAY" => Ok(TimePeriodCloseDayEnum::SATURDAY),
           "SUNDAY" => Ok(TimePeriodCloseDayEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TimePeriodCloseDayEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TimePeriodOpenDayEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the day of the week this period starts
on.
pub enum TimePeriodOpenDayEnum {
    

    /// The day of the week is unspecified.
    ///
    /// "DAY_OF_WEEK_UNSPECIFIED"
    #[serde(rename="DAY_OF_WEEK_UNSPECIFIED")]
    DAYOFWEEKUNSPECIFIED,
    

    /// Monday
    ///
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    

    /// Tuesday
    ///
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    

    /// Wednesday
    ///
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    

    /// Thursday
    ///
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    

    /// Friday
    ///
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    

    /// Saturday
    ///
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
    

    /// Sunday
    ///
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for TimePeriodOpenDayEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TimePeriodOpenDayEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            TimePeriodOpenDayEnum::MONDAY => "MONDAY",
            TimePeriodOpenDayEnum::TUESDAY => "TUESDAY",
            TimePeriodOpenDayEnum::WEDNESDAY => "WEDNESDAY",
            TimePeriodOpenDayEnum::THURSDAY => "THURSDAY",
            TimePeriodOpenDayEnum::FRIDAY => "FRIDAY",
            TimePeriodOpenDayEnum::SATURDAY => "SATURDAY",
            TimePeriodOpenDayEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for TimePeriodOpenDayEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(TimePeriodOpenDayEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(TimePeriodOpenDayEnum::MONDAY),
           "TUESDAY" => Ok(TimePeriodOpenDayEnum::TUESDAY),
           "WEDNESDAY" => Ok(TimePeriodOpenDayEnum::WEDNESDAY),
           "THURSDAY" => Ok(TimePeriodOpenDayEnum::THURSDAY),
           "FRIDAY" => Ok(TimePeriodOpenDayEnum::FRIDAY),
           "SATURDAY" => Ok(TimePeriodOpenDayEnum::SATURDAY),
           "SUNDAY" => Ok(TimePeriodOpenDayEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TimePeriodOpenDayEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VerificationMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The method of the verification.
pub enum VerificationMethodEnum {
    

    /// Default value, will result in errors.
    ///
    /// "VERIFICATION_METHOD_UNSPECIFIED"
    #[serde(rename="VERIFICATION_METHOD_UNSPECIFIED")]
    VERIFICATIONMETHODUNSPECIFIED,
    

    /// Send a postcard with a verification PIN to a specific mailing address.
The PIN is used to complete verification with Google.
    ///
    /// "ADDRESS"
    #[serde(rename="ADDRESS")]
    ADDRESS,
    

    /// Send an email with a verification PIN to a specific email address.
The PIN is used to complete verification with Google.
    ///
    /// "EMAIL"
    #[serde(rename="EMAIL")]
    EMAIL,
    

    /// Make a phone call with a verification PIN to a specific phone number.
The PIN is used to complete verification with Google.
    ///
    /// "PHONE_CALL"
    #[serde(rename="PHONE_CALL")]
    PHONECALL,
    

    /// Send an SMS with a verification PIN to a specific phone number.
The PIN is used to complete verification with Google.
    ///
    /// "SMS"
    #[serde(rename="SMS")]
    SMS,
    

    /// Verify the location without additional user action. This option may not be
available for all locations.
    ///
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
    

    /// Used for My Business Provider partners. This option may not be available
for all locations.
    ///
    /// "VETTED_PARTNER"
    #[serde(rename="VETTED_PARTNER")]
    VETTEDPARTNER,
}

impl AsRef<str> for VerificationMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VerificationMethodEnum::VERIFICATIONMETHODUNSPECIFIED => "VERIFICATION_METHOD_UNSPECIFIED",
            VerificationMethodEnum::ADDRESS => "ADDRESS",
            VerificationMethodEnum::EMAIL => "EMAIL",
            VerificationMethodEnum::PHONECALL => "PHONE_CALL",
            VerificationMethodEnum::SMS => "SMS",
            VerificationMethodEnum::AUTO => "AUTO",
            VerificationMethodEnum::VETTEDPARTNER => "VETTED_PARTNER",
        }
    }
}

impl std::convert::TryFrom< &str> for VerificationMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERIFICATION_METHOD_UNSPECIFIED" => Ok(VerificationMethodEnum::VERIFICATIONMETHODUNSPECIFIED),
           "ADDRESS" => Ok(VerificationMethodEnum::ADDRESS),
           "EMAIL" => Ok(VerificationMethodEnum::EMAIL),
           "PHONE_CALL" => Ok(VerificationMethodEnum::PHONECALL),
           "SMS" => Ok(VerificationMethodEnum::SMS),
           "AUTO" => Ok(VerificationMethodEnum::AUTO),
           "VETTED_PARTNER" => Ok(VerificationMethodEnum::VETTEDPARTNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VerificationMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VerificationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the verification.
pub enum VerificationStateEnum {
    

    /// Default value, will result in errors.
    ///
    /// "VERIFICATION_STATE_UNSPECIFIED"
    #[serde(rename="VERIFICATION_STATE_UNSPECIFIED")]
    VERIFICATIONSTATEUNSPECIFIED,
    

    /// The verification is pending.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The verification is completed.
    ///
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// The verification is failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for VerificationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VerificationStateEnum::VERIFICATIONSTATEUNSPECIFIED => "VERIFICATION_STATE_UNSPECIFIED",
            VerificationStateEnum::PENDING => "PENDING",
            VerificationStateEnum::COMPLETED => "COMPLETED",
            VerificationStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for VerificationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERIFICATION_STATE_UNSPECIFIED" => Ok(VerificationStateEnum::VERIFICATIONSTATEUNSPECIFIED),
           "PENDING" => Ok(VerificationStateEnum::PENDING),
           "COMPLETED" => Ok(VerificationStateEnum::COMPLETED),
           "FAILED" => Ok(VerificationStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VerificationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VerificationOptionVerificationMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Method to verify the location.
pub enum VerificationOptionVerificationMethodEnum {
    

    /// Default value, will result in errors.
    ///
    /// "VERIFICATION_METHOD_UNSPECIFIED"
    #[serde(rename="VERIFICATION_METHOD_UNSPECIFIED")]
    VERIFICATIONMETHODUNSPECIFIED,
    

    /// Send a postcard with a verification PIN to a specific mailing address.
The PIN is used to complete verification with Google.
    ///
    /// "ADDRESS"
    #[serde(rename="ADDRESS")]
    ADDRESS,
    

    /// Send an email with a verification PIN to a specific email address.
The PIN is used to complete verification with Google.
    ///
    /// "EMAIL"
    #[serde(rename="EMAIL")]
    EMAIL,
    

    /// Make a phone call with a verification PIN to a specific phone number.
The PIN is used to complete verification with Google.
    ///
    /// "PHONE_CALL"
    #[serde(rename="PHONE_CALL")]
    PHONECALL,
    

    /// Send an SMS with a verification PIN to a specific phone number.
The PIN is used to complete verification with Google.
    ///
    /// "SMS"
    #[serde(rename="SMS")]
    SMS,
    

    /// Verify the location without additional user action. This option may not be
available for all locations.
    ///
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
    

    /// Used for My Business Provider partners. This option may not be available
for all locations.
    ///
    /// "VETTED_PARTNER"
    #[serde(rename="VETTED_PARTNER")]
    VETTEDPARTNER,
}

impl AsRef<str> for VerificationOptionVerificationMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VerificationOptionVerificationMethodEnum::VERIFICATIONMETHODUNSPECIFIED => "VERIFICATION_METHOD_UNSPECIFIED",
            VerificationOptionVerificationMethodEnum::ADDRESS => "ADDRESS",
            VerificationOptionVerificationMethodEnum::EMAIL => "EMAIL",
            VerificationOptionVerificationMethodEnum::PHONECALL => "PHONE_CALL",
            VerificationOptionVerificationMethodEnum::SMS => "SMS",
            VerificationOptionVerificationMethodEnum::AUTO => "AUTO",
            VerificationOptionVerificationMethodEnum::VETTEDPARTNER => "VETTED_PARTNER",
        }
    }
}

impl std::convert::TryFrom< &str> for VerificationOptionVerificationMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERIFICATION_METHOD_UNSPECIFIED" => Ok(VerificationOptionVerificationMethodEnum::VERIFICATIONMETHODUNSPECIFIED),
           "ADDRESS" => Ok(VerificationOptionVerificationMethodEnum::ADDRESS),
           "EMAIL" => Ok(VerificationOptionVerificationMethodEnum::EMAIL),
           "PHONE_CALL" => Ok(VerificationOptionVerificationMethodEnum::PHONECALL),
           "SMS" => Ok(VerificationOptionVerificationMethodEnum::SMS),
           "AUTO" => Ok(VerificationOptionVerificationMethodEnum::AUTO),
           "VETTED_PARTNER" => Ok(VerificationOptionVerificationMethodEnum::VETTEDPARTNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VerificationOptionVerificationMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VerifyLocationRequestMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Verification method.
pub enum VerifyLocationRequestMethodEnum {
    

    /// Default value, will result in errors.
    ///
    /// "VERIFICATION_METHOD_UNSPECIFIED"
    #[serde(rename="VERIFICATION_METHOD_UNSPECIFIED")]
    VERIFICATIONMETHODUNSPECIFIED,
    

    /// Send a postcard with a verification PIN to a specific mailing address.
The PIN is used to complete verification with Google.
    ///
    /// "ADDRESS"
    #[serde(rename="ADDRESS")]
    ADDRESS,
    

    /// Send an email with a verification PIN to a specific email address.
The PIN is used to complete verification with Google.
    ///
    /// "EMAIL"
    #[serde(rename="EMAIL")]
    EMAIL,
    

    /// Make a phone call with a verification PIN to a specific phone number.
The PIN is used to complete verification with Google.
    ///
    /// "PHONE_CALL"
    #[serde(rename="PHONE_CALL")]
    PHONECALL,
    

    /// Send an SMS with a verification PIN to a specific phone number.
The PIN is used to complete verification with Google.
    ///
    /// "SMS"
    #[serde(rename="SMS")]
    SMS,
    

    /// Verify the location without additional user action. This option may not be
available for all locations.
    ///
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
    

    /// Used for My Business Provider partners. This option may not be available
for all locations.
    ///
    /// "VETTED_PARTNER"
    #[serde(rename="VETTED_PARTNER")]
    VETTEDPARTNER,
}

impl AsRef<str> for VerifyLocationRequestMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VerifyLocationRequestMethodEnum::VERIFICATIONMETHODUNSPECIFIED => "VERIFICATION_METHOD_UNSPECIFIED",
            VerifyLocationRequestMethodEnum::ADDRESS => "ADDRESS",
            VerifyLocationRequestMethodEnum::EMAIL => "EMAIL",
            VerifyLocationRequestMethodEnum::PHONECALL => "PHONE_CALL",
            VerifyLocationRequestMethodEnum::SMS => "SMS",
            VerifyLocationRequestMethodEnum::AUTO => "AUTO",
            VerifyLocationRequestMethodEnum::VETTEDPARTNER => "VETTED_PARTNER",
        }
    }
}

impl std::convert::TryFrom< &str> for VerifyLocationRequestMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERIFICATION_METHOD_UNSPECIFIED" => Ok(VerifyLocationRequestMethodEnum::VERIFICATIONMETHODUNSPECIFIED),
           "ADDRESS" => Ok(VerifyLocationRequestMethodEnum::ADDRESS),
           "EMAIL" => Ok(VerifyLocationRequestMethodEnum::EMAIL),
           "PHONE_CALL" => Ok(VerifyLocationRequestMethodEnum::PHONECALL),
           "SMS" => Ok(VerifyLocationRequestMethodEnum::SMS),
           "AUTO" => Ok(VerifyLocationRequestMethodEnum::AUTO),
           "VETTED_PARTNER" => Ok(VerifyLocationRequestMethodEnum::VETTEDPARTNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VerifyLocationRequestMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountTargetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies which target types should appear in the response.
pub enum AccountTargetTypeEnum {
    

    /// no description found
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// no description found
    ///
    /// "ACCOUNTS_ONLY"
    #[serde(rename="ACCOUNTS_ONLY")]
    ACCOUNTSONLY,
    

    /// no description found
    ///
    /// "LOCATIONS_ONLY"
    #[serde(rename="LOCATIONS_ONLY")]
    LOCATIONSONLY,
}

impl AsRef<str> for AccountTargetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountTargetTypeEnum::ALL => "ALL",
            AccountTargetTypeEnum::ACCOUNTSONLY => "ACCOUNTS_ONLY",
            AccountTargetTypeEnum::LOCATIONSONLY => "LOCATIONS_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountTargetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL" => Ok(AccountTargetTypeEnum::ALL),
           "ACCOUNTS_ONLY" => Ok(AccountTargetTypeEnum::ACCOUNTSONLY),
           "LOCATIONS_ONLY" => Ok(AccountTargetTypeEnum::LOCATIONSONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountTargetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


