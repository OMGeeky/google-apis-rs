use super::*;



// region BucketProjectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Set of properties to return. Defaults to full.
pub enum BucketProjectionEnum {
    

    /// Include all properties.
    ///
    /// "full"
    #[serde(rename="full")]
    Full,
    

    /// Omit owner, acl and defaultObjectAcl properties.
    ///
    /// "noAcl"
    #[serde(rename="noAcl")]
    NoAcl,
}

impl AsRef<str> for BucketProjectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BucketProjectionEnum::Full => "full",
            BucketProjectionEnum::NoAcl => "noAcl",
        }
    }
}

impl std::convert::TryFrom< &str> for BucketProjectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "full" => Ok(BucketProjectionEnum::Full),
           "noAcl" => Ok(BucketProjectionEnum::NoAcl),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BucketProjectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BucketPredefinedAclEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Apply a predefined set of access controls to this bucket.
pub enum BucketPredefinedAclEnum {
    

    /// Project team owners get OWNER access, and allAuthenticatedUsers get READER access.
    ///
    /// "authenticatedRead"
    #[serde(rename="authenticatedRead")]
    AuthenticatedRead,
    

    /// Project team owners get OWNER access.
    ///
    /// "private"
    #[serde(rename="private")]
    Private,
    

    /// Project team members get access according to their roles.
    ///
    /// "projectPrivate"
    #[serde(rename="projectPrivate")]
    ProjectPrivate,
    

    /// Project team owners get OWNER access, and allUsers get READER access.
    ///
    /// "publicRead"
    #[serde(rename="publicRead")]
    PublicRead,
    

    /// Project team owners get OWNER access, and allUsers get WRITER access.
    ///
    /// "publicReadWrite"
    #[serde(rename="publicReadWrite")]
    PublicReadWrite,
}

impl AsRef<str> for BucketPredefinedAclEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BucketPredefinedAclEnum::AuthenticatedRead => "authenticatedRead",
            BucketPredefinedAclEnum::Private => "private",
            BucketPredefinedAclEnum::ProjectPrivate => "projectPrivate",
            BucketPredefinedAclEnum::PublicRead => "publicRead",
            BucketPredefinedAclEnum::PublicReadWrite => "publicReadWrite",
        }
    }
}

impl std::convert::TryFrom< &str> for BucketPredefinedAclEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "authenticatedRead" => Ok(BucketPredefinedAclEnum::AuthenticatedRead),
           "private" => Ok(BucketPredefinedAclEnum::Private),
           "projectPrivate" => Ok(BucketPredefinedAclEnum::ProjectPrivate),
           "publicRead" => Ok(BucketPredefinedAclEnum::PublicRead),
           "publicReadWrite" => Ok(BucketPredefinedAclEnum::PublicReadWrite),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BucketPredefinedAclEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BucketPredefinedDefaultObjectAclEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Apply a predefined set of default object access controls to this bucket.
pub enum BucketPredefinedDefaultObjectAclEnum {
    

    /// Object owner gets OWNER access, and allAuthenticatedUsers get READER access.
    ///
    /// "authenticatedRead"
    #[serde(rename="authenticatedRead")]
    AuthenticatedRead,
    

    /// Object owner gets OWNER access, and project team owners get OWNER access.
    ///
    /// "bucketOwnerFullControl"
    #[serde(rename="bucketOwnerFullControl")]
    BucketOwnerFullControl,
    

    /// Object owner gets OWNER access, and project team owners get READER access.
    ///
    /// "bucketOwnerRead"
    #[serde(rename="bucketOwnerRead")]
    BucketOwnerRead,
    

    /// Object owner gets OWNER access.
    ///
    /// "private"
    #[serde(rename="private")]
    Private,
    

    /// Object owner gets OWNER access, and project team members get access according to their roles.
    ///
    /// "projectPrivate"
    #[serde(rename="projectPrivate")]
    ProjectPrivate,
    

    /// Object owner gets OWNER access, and allUsers get READER access.
    ///
    /// "publicRead"
    #[serde(rename="publicRead")]
    PublicRead,
}

impl AsRef<str> for BucketPredefinedDefaultObjectAclEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BucketPredefinedDefaultObjectAclEnum::AuthenticatedRead => "authenticatedRead",
            BucketPredefinedDefaultObjectAclEnum::BucketOwnerFullControl => "bucketOwnerFullControl",
            BucketPredefinedDefaultObjectAclEnum::BucketOwnerRead => "bucketOwnerRead",
            BucketPredefinedDefaultObjectAclEnum::Private => "private",
            BucketPredefinedDefaultObjectAclEnum::ProjectPrivate => "projectPrivate",
            BucketPredefinedDefaultObjectAclEnum::PublicRead => "publicRead",
        }
    }
}

impl std::convert::TryFrom< &str> for BucketPredefinedDefaultObjectAclEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "authenticatedRead" => Ok(BucketPredefinedDefaultObjectAclEnum::AuthenticatedRead),
           "bucketOwnerFullControl" => Ok(BucketPredefinedDefaultObjectAclEnum::BucketOwnerFullControl),
           "bucketOwnerRead" => Ok(BucketPredefinedDefaultObjectAclEnum::BucketOwnerRead),
           "private" => Ok(BucketPredefinedDefaultObjectAclEnum::Private),
           "projectPrivate" => Ok(BucketPredefinedDefaultObjectAclEnum::ProjectPrivate),
           "publicRead" => Ok(BucketPredefinedDefaultObjectAclEnum::PublicRead),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BucketPredefinedDefaultObjectAclEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ObjectDestinationPredefinedAclEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Apply a predefined set of access controls to the destination object.
pub enum ObjectDestinationPredefinedAclEnum {
    

    /// Object owner gets OWNER access, and allAuthenticatedUsers get READER access.
    ///
    /// "authenticatedRead"
    #[serde(rename="authenticatedRead")]
    AuthenticatedRead,
    

    /// Object owner gets OWNER access, and project team owners get OWNER access.
    ///
    /// "bucketOwnerFullControl"
    #[serde(rename="bucketOwnerFullControl")]
    BucketOwnerFullControl,
    

    /// Object owner gets OWNER access, and project team owners get READER access.
    ///
    /// "bucketOwnerRead"
    #[serde(rename="bucketOwnerRead")]
    BucketOwnerRead,
    

    /// Object owner gets OWNER access.
    ///
    /// "private"
    #[serde(rename="private")]
    Private,
    

    /// Object owner gets OWNER access, and project team members get access according to their roles.
    ///
    /// "projectPrivate"
    #[serde(rename="projectPrivate")]
    ProjectPrivate,
    

    /// Object owner gets OWNER access, and allUsers get READER access.
    ///
    /// "publicRead"
    #[serde(rename="publicRead")]
    PublicRead,
}

impl AsRef<str> for ObjectDestinationPredefinedAclEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ObjectDestinationPredefinedAclEnum::AuthenticatedRead => "authenticatedRead",
            ObjectDestinationPredefinedAclEnum::BucketOwnerFullControl => "bucketOwnerFullControl",
            ObjectDestinationPredefinedAclEnum::BucketOwnerRead => "bucketOwnerRead",
            ObjectDestinationPredefinedAclEnum::Private => "private",
            ObjectDestinationPredefinedAclEnum::ProjectPrivate => "projectPrivate",
            ObjectDestinationPredefinedAclEnum::PublicRead => "publicRead",
        }
    }
}

impl std::convert::TryFrom< &str> for ObjectDestinationPredefinedAclEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "authenticatedRead" => Ok(ObjectDestinationPredefinedAclEnum::AuthenticatedRead),
           "bucketOwnerFullControl" => Ok(ObjectDestinationPredefinedAclEnum::BucketOwnerFullControl),
           "bucketOwnerRead" => Ok(ObjectDestinationPredefinedAclEnum::BucketOwnerRead),
           "private" => Ok(ObjectDestinationPredefinedAclEnum::Private),
           "projectPrivate" => Ok(ObjectDestinationPredefinedAclEnum::ProjectPrivate),
           "publicRead" => Ok(ObjectDestinationPredefinedAclEnum::PublicRead),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ObjectDestinationPredefinedAclEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ObjectProjectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Set of properties to return. Defaults to noAcl.
pub enum ObjectProjectionEnum {
    

    /// Include all properties.
    ///
    /// "full"
    #[serde(rename="full")]
    Full,
    

    /// Omit the owner, acl property.
    ///
    /// "noAcl"
    #[serde(rename="noAcl")]
    NoAcl,
}

impl AsRef<str> for ObjectProjectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ObjectProjectionEnum::Full => "full",
            ObjectProjectionEnum::NoAcl => "noAcl",
        }
    }
}

impl std::convert::TryFrom< &str> for ObjectProjectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "full" => Ok(ObjectProjectionEnum::Full),
           "noAcl" => Ok(ObjectProjectionEnum::NoAcl),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ObjectProjectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ObjectPredefinedAclEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Apply a predefined set of access controls to this object.
pub enum ObjectPredefinedAclEnum {
    

    /// Object owner gets OWNER access, and allAuthenticatedUsers get READER access.
    ///
    /// "authenticatedRead"
    #[serde(rename="authenticatedRead")]
    AuthenticatedRead,
    

    /// Object owner gets OWNER access, and project team owners get OWNER access.
    ///
    /// "bucketOwnerFullControl"
    #[serde(rename="bucketOwnerFullControl")]
    BucketOwnerFullControl,
    

    /// Object owner gets OWNER access, and project team owners get READER access.
    ///
    /// "bucketOwnerRead"
    #[serde(rename="bucketOwnerRead")]
    BucketOwnerRead,
    

    /// Object owner gets OWNER access.
    ///
    /// "private"
    #[serde(rename="private")]
    Private,
    

    /// Object owner gets OWNER access, and project team members get access according to their roles.
    ///
    /// "projectPrivate"
    #[serde(rename="projectPrivate")]
    ProjectPrivate,
    

    /// Object owner gets OWNER access, and allUsers get READER access.
    ///
    /// "publicRead"
    #[serde(rename="publicRead")]
    PublicRead,
}

impl AsRef<str> for ObjectPredefinedAclEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ObjectPredefinedAclEnum::AuthenticatedRead => "authenticatedRead",
            ObjectPredefinedAclEnum::BucketOwnerFullControl => "bucketOwnerFullControl",
            ObjectPredefinedAclEnum::BucketOwnerRead => "bucketOwnerRead",
            ObjectPredefinedAclEnum::Private => "private",
            ObjectPredefinedAclEnum::ProjectPrivate => "projectPrivate",
            ObjectPredefinedAclEnum::PublicRead => "publicRead",
        }
    }
}

impl std::convert::TryFrom< &str> for ObjectPredefinedAclEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "authenticatedRead" => Ok(ObjectPredefinedAclEnum::AuthenticatedRead),
           "bucketOwnerFullControl" => Ok(ObjectPredefinedAclEnum::BucketOwnerFullControl),
           "bucketOwnerRead" => Ok(ObjectPredefinedAclEnum::BucketOwnerRead),
           "private" => Ok(ObjectPredefinedAclEnum::Private),
           "projectPrivate" => Ok(ObjectPredefinedAclEnum::ProjectPrivate),
           "publicRead" => Ok(ObjectPredefinedAclEnum::PublicRead),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ObjectPredefinedAclEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


