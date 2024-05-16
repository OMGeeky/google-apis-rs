use super::*;



// region AccountPermissionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the permission level the user has for this account.
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
/// Output only. Specifies the AccountRole of this account.
pub enum AccountRoleEnum {
    

    /// Not specified.
    ///
    /// "ACCOUNT_ROLE_UNSPECIFIED"
    #[serde(rename="ACCOUNT_ROLE_UNSPECIFIED")]
    ACCOUNTROLEUNSPECIFIED,
    

    /// The user is the primary owner this account.
    ///
    /// "PRIMARY_OWNER"
    #[serde(rename="PRIMARY_OWNER")]
    PRIMARYOWNER,
    

    /// The user owner of the account.
    ///
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
    

    /// The user can manage this account.
    ///
    /// "MANAGER"
    #[serde(rename="MANAGER")]
    MANAGER,
    

    /// The user can manage a limited set of features for the account.
    ///
    /// "SITE_MANAGER"
    #[serde(rename="SITE_MANAGER")]
    SITEMANAGER,
}

impl AsRef<str> for AccountRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountRoleEnum::ACCOUNTROLEUNSPECIFIED => "ACCOUNT_ROLE_UNSPECIFIED",
            AccountRoleEnum::PRIMARYOWNER => "PRIMARY_OWNER",
            AccountRoleEnum::OWNER => "OWNER",
            AccountRoleEnum::MANAGER => "MANAGER",
            AccountRoleEnum::SITEMANAGER => "SITE_MANAGER",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCOUNT_ROLE_UNSPECIFIED" => Ok(AccountRoleEnum::ACCOUNTROLEUNSPECIFIED),
           "PRIMARY_OWNER" => Ok(AccountRoleEnum::PRIMARYOWNER),
           "OWNER" => Ok(AccountRoleEnum::OWNER),
           "MANAGER" => Ok(AccountRoleEnum::MANAGER),
           "SITE_MANAGER" => Ok(AccountRoleEnum::SITEMANAGER),
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
/// Required. Contains the type of account. Accounts of type PERSONAL and ORGANIZATION cannot be created using this API.
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
    

    /// A group of Locations. For more information, see the [help center article] (https://support.google.com/business/answer/6085326)
    ///
    /// "LOCATION_GROUP"
    #[serde(rename="LOCATION_GROUP")]
    LOCATIONGROUP,
    

    /// A User Group for segregating organization staff in groups. For more information, see the [help center article](https://support.google.com/business/answer/7655731)
    ///
    /// "USER_GROUP"
    #[serde(rename="USER_GROUP")]
    USERGROUP,
    

    /// An organization representing a company. For more information, see the [help center article](https://support.google.com/business/answer/7663063)
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


// region AccountVerificationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. If verified, future locations that are created are automatically connected to Google Maps, and have Google+ pages created, without requiring moderation.
pub enum AccountVerificationStateEnum {
    

    /// Not specified.
    ///
    /// "VERIFICATION_STATE_UNSPECIFIED"
    #[serde(rename="VERIFICATION_STATE_UNSPECIFIED")]
    VERIFICATIONSTATEUNSPECIFIED,
    

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

impl AsRef<str> for AccountVerificationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountVerificationStateEnum::VERIFICATIONSTATEUNSPECIFIED => "VERIFICATION_STATE_UNSPECIFIED",
            AccountVerificationStateEnum::VERIFIED => "VERIFIED",
            AccountVerificationStateEnum::UNVERIFIED => "UNVERIFIED",
            AccountVerificationStateEnum::VERIFICATIONREQUESTED => "VERIFICATION_REQUESTED",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountVerificationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERIFICATION_STATE_UNSPECIFIED" => Ok(AccountVerificationStateEnum::VERIFICATIONSTATEUNSPECIFIED),
           "VERIFIED" => Ok(AccountVerificationStateEnum::VERIFIED),
           "UNVERIFIED" => Ok(AccountVerificationStateEnum::UNVERIFIED),
           "VERIFICATION_REQUESTED" => Ok(AccountVerificationStateEnum::VERIFICATIONREQUESTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountVerificationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountVettedStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Indicates whether the account is vetted by Google. A vetted account is able to verify locations via the VETTED_PARTNER method.
pub enum AccountVettedStateEnum {
    

    /// Not Specified
    ///
    /// "VETTED_STATE_UNSPECIFIED"
    #[serde(rename="VETTED_STATE_UNSPECIFIED")]
    VETTEDSTATEUNSPECIFIED,
    

    /// The account is not vetted by Google.
    ///
    /// "NOT_VETTED"
    #[serde(rename="NOT_VETTED")]
    NOTVETTED,
    

    /// The account is vetted by Google and in a valid state. An account is automatically vetted if it has direct access to a vetted group account.
    ///
    /// "VETTED"
    #[serde(rename="VETTED")]
    VETTED,
    

    /// The account is vetted but in an invalid state. The account will behave like an unvetted account.
    ///
    /// "INVALID"
    #[serde(rename="INVALID")]
    INVALID,
}

impl AsRef<str> for AccountVettedStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountVettedStateEnum::VETTEDSTATEUNSPECIFIED => "VETTED_STATE_UNSPECIFIED",
            AccountVettedStateEnum::NOTVETTED => "NOT_VETTED",
            AccountVettedStateEnum::VETTED => "VETTED",
            AccountVettedStateEnum::INVALID => "INVALID",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountVettedStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VETTED_STATE_UNSPECIFIED" => Ok(AccountVettedStateEnum::VETTEDSTATEUNSPECIFIED),
           "NOT_VETTED" => Ok(AccountVettedStateEnum::NOTVETTED),
           "VETTED" => Ok(AccountVettedStateEnum::VETTED),
           "INVALID" => Ok(AccountVettedStateEnum::INVALID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountVettedStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdminRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Specifies the role that this admin uses with the specified Account or Location.
pub enum AdminRoleEnum {
    

    /// Not specified.
    ///
    /// "ADMIN_ROLE_UNSPECIFIED"
    #[serde(rename="ADMIN_ROLE_UNSPECIFIED")]
    ADMINROLEUNSPECIFIED,
    

    /// The admin has owner-level access and is the primary owner. (Displays as 'Primary Owner' in UI).
    ///
    /// "PRIMARY_OWNER"
    #[serde(rename="PRIMARY_OWNER")]
    PRIMARYOWNER,
    

    /// The admin has owner-level access. (Displays as 'Owner' in UI).
    ///
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
    

    /// The admin has managerial access.
    ///
    /// "MANAGER"
    #[serde(rename="MANAGER")]
    MANAGER,
    

    /// The admin can manage social (Google+) pages. (Displays as 'Site Manager' in UI). This API doesn't allow creating an account admin with a SITE_MANAGER role.
    ///
    /// "SITE_MANAGER"
    #[serde(rename="SITE_MANAGER")]
    SITEMANAGER,
}

impl AsRef<str> for AdminRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdminRoleEnum::ADMINROLEUNSPECIFIED => "ADMIN_ROLE_UNSPECIFIED",
            AdminRoleEnum::PRIMARYOWNER => "PRIMARY_OWNER",
            AdminRoleEnum::OWNER => "OWNER",
            AdminRoleEnum::MANAGER => "MANAGER",
            AdminRoleEnum::SITEMANAGER => "SITE_MANAGER",
        }
    }
}

impl std::convert::TryFrom< &str> for AdminRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADMIN_ROLE_UNSPECIFIED" => Ok(AdminRoleEnum::ADMINROLEUNSPECIFIED),
           "PRIMARY_OWNER" => Ok(AdminRoleEnum::PRIMARYOWNER),
           "OWNER" => Ok(AdminRoleEnum::OWNER),
           "MANAGER" => Ok(AdminRoleEnum::MANAGER),
           "SITE_MANAGER" => Ok(AdminRoleEnum::SITEMANAGER),
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


// region InvitationRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The invited role on the account.
pub enum InvitationRoleEnum {
    

    /// Not specified.
    ///
    /// "ADMIN_ROLE_UNSPECIFIED"
    #[serde(rename="ADMIN_ROLE_UNSPECIFIED")]
    ADMINROLEUNSPECIFIED,
    

    /// The admin has owner-level access and is the primary owner. (Displays as 'Primary Owner' in UI).
    ///
    /// "PRIMARY_OWNER"
    #[serde(rename="PRIMARY_OWNER")]
    PRIMARYOWNER,
    

    /// The admin has owner-level access. (Displays as 'Owner' in UI).
    ///
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
    

    /// The admin has managerial access.
    ///
    /// "MANAGER"
    #[serde(rename="MANAGER")]
    MANAGER,
    

    /// The admin can manage social (Google+) pages. (Displays as 'Site Manager' in UI). This API doesn't allow creating an account admin with a SITE_MANAGER role.
    ///
    /// "SITE_MANAGER"
    #[serde(rename="SITE_MANAGER")]
    SITEMANAGER,
}

impl AsRef<str> for InvitationRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InvitationRoleEnum::ADMINROLEUNSPECIFIED => "ADMIN_ROLE_UNSPECIFIED",
            InvitationRoleEnum::PRIMARYOWNER => "PRIMARY_OWNER",
            InvitationRoleEnum::OWNER => "OWNER",
            InvitationRoleEnum::MANAGER => "MANAGER",
            InvitationRoleEnum::SITEMANAGER => "SITE_MANAGER",
        }
    }
}

impl std::convert::TryFrom< &str> for InvitationRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADMIN_ROLE_UNSPECIFIED" => Ok(InvitationRoleEnum::ADMINROLEUNSPECIFIED),
           "PRIMARY_OWNER" => Ok(InvitationRoleEnum::PRIMARYOWNER),
           "OWNER" => Ok(InvitationRoleEnum::OWNER),
           "MANAGER" => Ok(InvitationRoleEnum::MANAGER),
           "SITE_MANAGER" => Ok(InvitationRoleEnum::SITEMANAGER),
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


// region InvitationTargetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies which target types should appear in the response.
pub enum InvitationTargetTypeEnum {
    

    /// Set when target type is unspecified.
    ///
    /// "TARGET_TYPE_UNSPECIFIED"
    #[serde(rename="TARGET_TYPE_UNSPECIFIED")]
    TARGETTYPEUNSPECIFIED,
    

    /// List invitations only for targets of type Account.
    ///
    /// "ACCOUNTS_ONLY"
    #[serde(rename="ACCOUNTS_ONLY")]
    ACCOUNTSONLY,
    

    /// List invitations only for targets of type Location.
    ///
    /// "LOCATIONS_ONLY"
    #[serde(rename="LOCATIONS_ONLY")]
    LOCATIONSONLY,
}

impl AsRef<str> for InvitationTargetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InvitationTargetTypeEnum::TARGETTYPEUNSPECIFIED => "TARGET_TYPE_UNSPECIFIED",
            InvitationTargetTypeEnum::ACCOUNTSONLY => "ACCOUNTS_ONLY",
            InvitationTargetTypeEnum::LOCATIONSONLY => "LOCATIONS_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for InvitationTargetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGET_TYPE_UNSPECIFIED" => Ok(InvitationTargetTypeEnum::TARGETTYPEUNSPECIFIED),
           "ACCOUNTS_ONLY" => Ok(InvitationTargetTypeEnum::ACCOUNTSONLY),
           "LOCATIONS_ONLY" => Ok(InvitationTargetTypeEnum::LOCATIONSONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InvitationTargetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


