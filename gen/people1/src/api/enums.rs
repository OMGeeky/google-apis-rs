use super::*;



// region AgeRangeTypeAgeRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The age range.
pub enum AgeRangeTypeAgeRangeEnum {
    

    /// Unspecified.
    ///
    /// "AGE_RANGE_UNSPECIFIED"
    #[serde(rename="AGE_RANGE_UNSPECIFIED")]
    AGERANGEUNSPECIFIED,
    

    /// Younger than eighteen.
    ///
    /// "LESS_THAN_EIGHTEEN"
    #[serde(rename="LESS_THAN_EIGHTEEN")]
    LESSTHANEIGHTEEN,
    

    /// Between eighteen and twenty.
    ///
    /// "EIGHTEEN_TO_TWENTY"
    #[serde(rename="EIGHTEEN_TO_TWENTY")]
    EIGHTEENTOTWENTY,
    

    /// Twenty-one and older.
    ///
    /// "TWENTY_ONE_OR_OLDER"
    #[serde(rename="TWENTY_ONE_OR_OLDER")]
    TWENTYONEOROLDER,
}

impl AsRef<str> for AgeRangeTypeAgeRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AgeRangeTypeAgeRangeEnum::AGERANGEUNSPECIFIED => "AGE_RANGE_UNSPECIFIED",
            AgeRangeTypeAgeRangeEnum::LESSTHANEIGHTEEN => "LESS_THAN_EIGHTEEN",
            AgeRangeTypeAgeRangeEnum::EIGHTEENTOTWENTY => "EIGHTEEN_TO_TWENTY",
            AgeRangeTypeAgeRangeEnum::TWENTYONEOROLDER => "TWENTY_ONE_OR_OLDER",
        }
    }
}

impl std::convert::TryFrom< &str> for AgeRangeTypeAgeRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AGE_RANGE_UNSPECIFIED" => Ok(AgeRangeTypeAgeRangeEnum::AGERANGEUNSPECIFIED),
           "LESS_THAN_EIGHTEEN" => Ok(AgeRangeTypeAgeRangeEnum::LESSTHANEIGHTEEN),
           "EIGHTEEN_TO_TWENTY" => Ok(AgeRangeTypeAgeRangeEnum::EIGHTEENTOTWENTY),
           "TWENTY_ONE_OR_OLDER" => Ok(AgeRangeTypeAgeRangeEnum::TWENTYONEOROLDER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AgeRangeTypeAgeRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchCreateContactsRequestSourcesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A mask of what source types to return in the post mutate read. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.
pub enum BatchCreateContactsRequestSourcesEnum {
    

    /// Unspecified.
    ///
    /// "READ_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="READ_SOURCE_TYPE_UNSPECIFIED")]
    READSOURCETYPEUNSPECIFIED,
    

    /// Returns SourceType.ACCOUNT, SourceType.DOMAIN_PROFILE, and SourceType.PROFILE.
    ///
    /// "READ_SOURCE_TYPE_PROFILE"
    #[serde(rename="READ_SOURCE_TYPE_PROFILE")]
    READSOURCETYPEPROFILE,
    

    /// Returns SourceType.CONTACT.
    ///
    /// "READ_SOURCE_TYPE_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_CONTACT")]
    READSOURCETYPECONTACT,
    

    /// Returns SourceType.DOMAIN_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_DOMAIN_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_DOMAIN_CONTACT")]
    READSOURCETYPEDOMAINCONTACT,
    

    /// Returns SourceType.OTHER_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_OTHER_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_OTHER_CONTACT")]
    READSOURCETYPEOTHERCONTACT,
}

impl AsRef<str> for BatchCreateContactsRequestSourcesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchCreateContactsRequestSourcesEnum::READSOURCETYPEUNSPECIFIED => "READ_SOURCE_TYPE_UNSPECIFIED",
            BatchCreateContactsRequestSourcesEnum::READSOURCETYPEPROFILE => "READ_SOURCE_TYPE_PROFILE",
            BatchCreateContactsRequestSourcesEnum::READSOURCETYPECONTACT => "READ_SOURCE_TYPE_CONTACT",
            BatchCreateContactsRequestSourcesEnum::READSOURCETYPEDOMAINCONTACT => "READ_SOURCE_TYPE_DOMAIN_CONTACT",
            BatchCreateContactsRequestSourcesEnum::READSOURCETYPEOTHERCONTACT => "READ_SOURCE_TYPE_OTHER_CONTACT",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchCreateContactsRequestSourcesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "READ_SOURCE_TYPE_UNSPECIFIED" => Ok(BatchCreateContactsRequestSourcesEnum::READSOURCETYPEUNSPECIFIED),
           "READ_SOURCE_TYPE_PROFILE" => Ok(BatchCreateContactsRequestSourcesEnum::READSOURCETYPEPROFILE),
           "READ_SOURCE_TYPE_CONTACT" => Ok(BatchCreateContactsRequestSourcesEnum::READSOURCETYPECONTACT),
           "READ_SOURCE_TYPE_DOMAIN_CONTACT" => Ok(BatchCreateContactsRequestSourcesEnum::READSOURCETYPEDOMAINCONTACT),
           "READ_SOURCE_TYPE_OTHER_CONTACT" => Ok(BatchCreateContactsRequestSourcesEnum::READSOURCETYPEOTHERCONTACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchCreateContactsRequestSourcesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchUpdateContactsRequestSourcesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.
pub enum BatchUpdateContactsRequestSourcesEnum {
    

    /// Unspecified.
    ///
    /// "READ_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="READ_SOURCE_TYPE_UNSPECIFIED")]
    READSOURCETYPEUNSPECIFIED,
    

    /// Returns SourceType.ACCOUNT, SourceType.DOMAIN_PROFILE, and SourceType.PROFILE.
    ///
    /// "READ_SOURCE_TYPE_PROFILE"
    #[serde(rename="READ_SOURCE_TYPE_PROFILE")]
    READSOURCETYPEPROFILE,
    

    /// Returns SourceType.CONTACT.
    ///
    /// "READ_SOURCE_TYPE_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_CONTACT")]
    READSOURCETYPECONTACT,
    

    /// Returns SourceType.DOMAIN_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_DOMAIN_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_DOMAIN_CONTACT")]
    READSOURCETYPEDOMAINCONTACT,
    

    /// Returns SourceType.OTHER_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_OTHER_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_OTHER_CONTACT")]
    READSOURCETYPEOTHERCONTACT,
}

impl AsRef<str> for BatchUpdateContactsRequestSourcesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchUpdateContactsRequestSourcesEnum::READSOURCETYPEUNSPECIFIED => "READ_SOURCE_TYPE_UNSPECIFIED",
            BatchUpdateContactsRequestSourcesEnum::READSOURCETYPEPROFILE => "READ_SOURCE_TYPE_PROFILE",
            BatchUpdateContactsRequestSourcesEnum::READSOURCETYPECONTACT => "READ_SOURCE_TYPE_CONTACT",
            BatchUpdateContactsRequestSourcesEnum::READSOURCETYPEDOMAINCONTACT => "READ_SOURCE_TYPE_DOMAIN_CONTACT",
            BatchUpdateContactsRequestSourcesEnum::READSOURCETYPEOTHERCONTACT => "READ_SOURCE_TYPE_OTHER_CONTACT",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchUpdateContactsRequestSourcesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "READ_SOURCE_TYPE_UNSPECIFIED" => Ok(BatchUpdateContactsRequestSourcesEnum::READSOURCETYPEUNSPECIFIED),
           "READ_SOURCE_TYPE_PROFILE" => Ok(BatchUpdateContactsRequestSourcesEnum::READSOURCETYPEPROFILE),
           "READ_SOURCE_TYPE_CONTACT" => Ok(BatchUpdateContactsRequestSourcesEnum::READSOURCETYPECONTACT),
           "READ_SOURCE_TYPE_DOMAIN_CONTACT" => Ok(BatchUpdateContactsRequestSourcesEnum::READSOURCETYPEDOMAINCONTACT),
           "READ_SOURCE_TYPE_OTHER_CONTACT" => Ok(BatchUpdateContactsRequestSourcesEnum::READSOURCETYPEOTHERCONTACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchUpdateContactsRequestSourcesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BiographyContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The content type of the biography.
pub enum BiographyContentTypeEnum {
    

    /// Unspecified.
    ///
    /// "CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_TYPE_UNSPECIFIED")]
    CONTENTTYPEUNSPECIFIED,
    

    /// Plain text.
    ///
    /// "TEXT_PLAIN"
    #[serde(rename="TEXT_PLAIN")]
    TEXTPLAIN,
    

    /// HTML text.
    ///
    /// "TEXT_HTML"
    #[serde(rename="TEXT_HTML")]
    TEXTHTML,
}

impl AsRef<str> for BiographyContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BiographyContentTypeEnum::CONTENTTYPEUNSPECIFIED => "CONTENT_TYPE_UNSPECIFIED",
            BiographyContentTypeEnum::TEXTPLAIN => "TEXT_PLAIN",
            BiographyContentTypeEnum::TEXTHTML => "TEXT_HTML",
        }
    }
}

impl std::convert::TryFrom< &str> for BiographyContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_TYPE_UNSPECIFIED" => Ok(BiographyContentTypeEnum::CONTENTTYPEUNSPECIFIED),
           "TEXT_PLAIN" => Ok(BiographyContentTypeEnum::TEXTPLAIN),
           "TEXT_HTML" => Ok(BiographyContentTypeEnum::TEXTHTML),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BiographyContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContactGroupGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The contact group type.
pub enum ContactGroupGroupTypeEnum {
    

    /// Unspecified.
    ///
    /// "GROUP_TYPE_UNSPECIFIED"
    #[serde(rename="GROUP_TYPE_UNSPECIFIED")]
    GROUPTYPEUNSPECIFIED,
    

    /// User defined contact group.
    ///
    /// "USER_CONTACT_GROUP"
    #[serde(rename="USER_CONTACT_GROUP")]
    USERCONTACTGROUP,
    

    /// System defined contact group.
    ///
    /// "SYSTEM_CONTACT_GROUP"
    #[serde(rename="SYSTEM_CONTACT_GROUP")]
    SYSTEMCONTACTGROUP,
}

impl AsRef<str> for ContactGroupGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContactGroupGroupTypeEnum::GROUPTYPEUNSPECIFIED => "GROUP_TYPE_UNSPECIFIED",
            ContactGroupGroupTypeEnum::USERCONTACTGROUP => "USER_CONTACT_GROUP",
            ContactGroupGroupTypeEnum::SYSTEMCONTACTGROUP => "SYSTEM_CONTACT_GROUP",
        }
    }
}

impl std::convert::TryFrom< &str> for ContactGroupGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GROUP_TYPE_UNSPECIFIED" => Ok(ContactGroupGroupTypeEnum::GROUPTYPEUNSPECIFIED),
           "USER_CONTACT_GROUP" => Ok(ContactGroupGroupTypeEnum::USERCONTACTGROUP),
           "SYSTEM_CONTACT_GROUP" => Ok(ContactGroupGroupTypeEnum::SYSTEMCONTACTGROUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContactGroupGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CopyOtherContactToMyContactsGroupRequestSourcesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.
pub enum CopyOtherContactToMyContactsGroupRequestSourcesEnum {
    

    /// Unspecified.
    ///
    /// "READ_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="READ_SOURCE_TYPE_UNSPECIFIED")]
    READSOURCETYPEUNSPECIFIED,
    

    /// Returns SourceType.ACCOUNT, SourceType.DOMAIN_PROFILE, and SourceType.PROFILE.
    ///
    /// "READ_SOURCE_TYPE_PROFILE"
    #[serde(rename="READ_SOURCE_TYPE_PROFILE")]
    READSOURCETYPEPROFILE,
    

    /// Returns SourceType.CONTACT.
    ///
    /// "READ_SOURCE_TYPE_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_CONTACT")]
    READSOURCETYPECONTACT,
    

    /// Returns SourceType.DOMAIN_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_DOMAIN_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_DOMAIN_CONTACT")]
    READSOURCETYPEDOMAINCONTACT,
    

    /// Returns SourceType.OTHER_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_OTHER_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_OTHER_CONTACT")]
    READSOURCETYPEOTHERCONTACT,
}

impl AsRef<str> for CopyOtherContactToMyContactsGroupRequestSourcesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CopyOtherContactToMyContactsGroupRequestSourcesEnum::READSOURCETYPEUNSPECIFIED => "READ_SOURCE_TYPE_UNSPECIFIED",
            CopyOtherContactToMyContactsGroupRequestSourcesEnum::READSOURCETYPEPROFILE => "READ_SOURCE_TYPE_PROFILE",
            CopyOtherContactToMyContactsGroupRequestSourcesEnum::READSOURCETYPECONTACT => "READ_SOURCE_TYPE_CONTACT",
            CopyOtherContactToMyContactsGroupRequestSourcesEnum::READSOURCETYPEDOMAINCONTACT => "READ_SOURCE_TYPE_DOMAIN_CONTACT",
            CopyOtherContactToMyContactsGroupRequestSourcesEnum::READSOURCETYPEOTHERCONTACT => "READ_SOURCE_TYPE_OTHER_CONTACT",
        }
    }
}

impl std::convert::TryFrom< &str> for CopyOtherContactToMyContactsGroupRequestSourcesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "READ_SOURCE_TYPE_UNSPECIFIED" => Ok(CopyOtherContactToMyContactsGroupRequestSourcesEnum::READSOURCETYPEUNSPECIFIED),
           "READ_SOURCE_TYPE_PROFILE" => Ok(CopyOtherContactToMyContactsGroupRequestSourcesEnum::READSOURCETYPEPROFILE),
           "READ_SOURCE_TYPE_CONTACT" => Ok(CopyOtherContactToMyContactsGroupRequestSourcesEnum::READSOURCETYPECONTACT),
           "READ_SOURCE_TYPE_DOMAIN_CONTACT" => Ok(CopyOtherContactToMyContactsGroupRequestSourcesEnum::READSOURCETYPEDOMAINCONTACT),
           "READ_SOURCE_TYPE_OTHER_CONTACT" => Ok(CopyOtherContactToMyContactsGroupRequestSourcesEnum::READSOURCETYPEOTHERCONTACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CopyOtherContactToMyContactsGroupRequestSourcesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MiscKeywordTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The miscellaneous keyword type.
pub enum MiscKeywordTypeEnum {
    

    /// Unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Outlook field for billing information.
    ///
    /// "OUTLOOK_BILLING_INFORMATION"
    #[serde(rename="OUTLOOK_BILLING_INFORMATION")]
    OUTLOOKBILLINGINFORMATION,
    

    /// Outlook field for directory server.
    ///
    /// "OUTLOOK_DIRECTORY_SERVER"
    #[serde(rename="OUTLOOK_DIRECTORY_SERVER")]
    OUTLOOKDIRECTORYSERVER,
    

    /// Outlook field for keyword.
    ///
    /// "OUTLOOK_KEYWORD"
    #[serde(rename="OUTLOOK_KEYWORD")]
    OUTLOOKKEYWORD,
    

    /// Outlook field for mileage.
    ///
    /// "OUTLOOK_MILEAGE"
    #[serde(rename="OUTLOOK_MILEAGE")]
    OUTLOOKMILEAGE,
    

    /// Outlook field for priority.
    ///
    /// "OUTLOOK_PRIORITY"
    #[serde(rename="OUTLOOK_PRIORITY")]
    OUTLOOKPRIORITY,
    

    /// Outlook field for sensitivity.
    ///
    /// "OUTLOOK_SENSITIVITY"
    #[serde(rename="OUTLOOK_SENSITIVITY")]
    OUTLOOKSENSITIVITY,
    

    /// Outlook field for subject.
    ///
    /// "OUTLOOK_SUBJECT"
    #[serde(rename="OUTLOOK_SUBJECT")]
    OUTLOOKSUBJECT,
    

    /// Outlook field for user.
    ///
    /// "OUTLOOK_USER"
    #[serde(rename="OUTLOOK_USER")]
    OUTLOOKUSER,
    

    /// Home.
    ///
    /// "HOME"
    #[serde(rename="HOME")]
    HOME,
    

    /// Work.
    ///
    /// "WORK"
    #[serde(rename="WORK")]
    WORK,
    

    /// Other.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for MiscKeywordTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MiscKeywordTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            MiscKeywordTypeEnum::OUTLOOKBILLINGINFORMATION => "OUTLOOK_BILLING_INFORMATION",
            MiscKeywordTypeEnum::OUTLOOKDIRECTORYSERVER => "OUTLOOK_DIRECTORY_SERVER",
            MiscKeywordTypeEnum::OUTLOOKKEYWORD => "OUTLOOK_KEYWORD",
            MiscKeywordTypeEnum::OUTLOOKMILEAGE => "OUTLOOK_MILEAGE",
            MiscKeywordTypeEnum::OUTLOOKPRIORITY => "OUTLOOK_PRIORITY",
            MiscKeywordTypeEnum::OUTLOOKSENSITIVITY => "OUTLOOK_SENSITIVITY",
            MiscKeywordTypeEnum::OUTLOOKSUBJECT => "OUTLOOK_SUBJECT",
            MiscKeywordTypeEnum::OUTLOOKUSER => "OUTLOOK_USER",
            MiscKeywordTypeEnum::HOME => "HOME",
            MiscKeywordTypeEnum::WORK => "WORK",
            MiscKeywordTypeEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for MiscKeywordTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(MiscKeywordTypeEnum::TYPEUNSPECIFIED),
           "OUTLOOK_BILLING_INFORMATION" => Ok(MiscKeywordTypeEnum::OUTLOOKBILLINGINFORMATION),
           "OUTLOOK_DIRECTORY_SERVER" => Ok(MiscKeywordTypeEnum::OUTLOOKDIRECTORYSERVER),
           "OUTLOOK_KEYWORD" => Ok(MiscKeywordTypeEnum::OUTLOOKKEYWORD),
           "OUTLOOK_MILEAGE" => Ok(MiscKeywordTypeEnum::OUTLOOKMILEAGE),
           "OUTLOOK_PRIORITY" => Ok(MiscKeywordTypeEnum::OUTLOOKPRIORITY),
           "OUTLOOK_SENSITIVITY" => Ok(MiscKeywordTypeEnum::OUTLOOKSENSITIVITY),
           "OUTLOOK_SUBJECT" => Ok(MiscKeywordTypeEnum::OUTLOOKSUBJECT),
           "OUTLOOK_USER" => Ok(MiscKeywordTypeEnum::OUTLOOKUSER),
           "HOME" => Ok(MiscKeywordTypeEnum::HOME),
           "WORK" => Ok(MiscKeywordTypeEnum::WORK),
           "OTHER" => Ok(MiscKeywordTypeEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MiscKeywordTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NicknameTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the nickname.
pub enum NicknameTypeEnum {
    

    /// Generic nickname.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// Maiden name or birth family name. Used when the person's family name has changed as a result of marriage.
    ///
    /// "MAIDEN_NAME"
    #[serde(rename="MAIDEN_NAME")]
    MAIDENNAME,
    

    /// Initials.
    ///
    /// "INITIALS"
    #[serde(rename="INITIALS")]
    INITIALS,
    

    /// Google+ profile nickname.
    ///
    /// "GPLUS"
    #[serde(rename="GPLUS")]
    GPLUS,
    

    /// A professional affiliation or other name; for example, `Dr. Smith.`
    ///
    /// "OTHER_NAME"
    #[serde(rename="OTHER_NAME")]
    OTHERNAME,
    

    /// Alternate name person is known by.
    ///
    /// "ALTERNATE_NAME"
    #[serde(rename="ALTERNATE_NAME")]
    ALTERNATENAME,
    

    /// A shorter version of the person's name.
    ///
    /// "SHORT_NAME"
    #[serde(rename="SHORT_NAME")]
    SHORTNAME,
}

impl AsRef<str> for NicknameTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NicknameTypeEnum::DEFAULT => "DEFAULT",
            NicknameTypeEnum::MAIDENNAME => "MAIDEN_NAME",
            NicknameTypeEnum::INITIALS => "INITIALS",
            NicknameTypeEnum::GPLUS => "GPLUS",
            NicknameTypeEnum::OTHERNAME => "OTHER_NAME",
            NicknameTypeEnum::ALTERNATENAME => "ALTERNATE_NAME",
            NicknameTypeEnum::SHORTNAME => "SHORT_NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for NicknameTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(NicknameTypeEnum::DEFAULT),
           "MAIDEN_NAME" => Ok(NicknameTypeEnum::MAIDENNAME),
           "INITIALS" => Ok(NicknameTypeEnum::INITIALS),
           "GPLUS" => Ok(NicknameTypeEnum::GPLUS),
           "OTHER_NAME" => Ok(NicknameTypeEnum::OTHERNAME),
           "ALTERNATE_NAME" => Ok(NicknameTypeEnum::ALTERNATENAME),
           "SHORT_NAME" => Ok(NicknameTypeEnum::SHORTNAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NicknameTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonAgeRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. **DEPRECATED** (Please use `person.ageRanges` instead) The person's age range.
pub enum PersonAgeRangeEnum {
    

    /// Unspecified.
    ///
    /// "AGE_RANGE_UNSPECIFIED"
    #[serde(rename="AGE_RANGE_UNSPECIFIED")]
    AGERANGEUNSPECIFIED,
    

    /// Younger than eighteen.
    ///
    /// "LESS_THAN_EIGHTEEN"
    #[serde(rename="LESS_THAN_EIGHTEEN")]
    LESSTHANEIGHTEEN,
    

    /// Between eighteen and twenty.
    ///
    /// "EIGHTEEN_TO_TWENTY"
    #[serde(rename="EIGHTEEN_TO_TWENTY")]
    EIGHTEENTOTWENTY,
    

    /// Twenty-one and older.
    ///
    /// "TWENTY_ONE_OR_OLDER"
    #[serde(rename="TWENTY_ONE_OR_OLDER")]
    TWENTYONEOROLDER,
}

impl AsRef<str> for PersonAgeRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonAgeRangeEnum::AGERANGEUNSPECIFIED => "AGE_RANGE_UNSPECIFIED",
            PersonAgeRangeEnum::LESSTHANEIGHTEEN => "LESS_THAN_EIGHTEEN",
            PersonAgeRangeEnum::EIGHTEENTOTWENTY => "EIGHTEEN_TO_TWENTY",
            PersonAgeRangeEnum::TWENTYONEOROLDER => "TWENTY_ONE_OR_OLDER",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonAgeRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AGE_RANGE_UNSPECIFIED" => Ok(PersonAgeRangeEnum::AGERANGEUNSPECIFIED),
           "LESS_THAN_EIGHTEEN" => Ok(PersonAgeRangeEnum::LESSTHANEIGHTEEN),
           "EIGHTEEN_TO_TWENTY" => Ok(PersonAgeRangeEnum::EIGHTEENTOTWENTY),
           "TWENTY_ONE_OR_OLDER" => Ok(PersonAgeRangeEnum::TWENTYONEOROLDER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonAgeRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonMetadataObjectTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. **DEPRECATED** (Please use `person.metadata.sources.profileMetadata.objectType` instead) The type of the person object.
pub enum PersonMetadataObjectTypeEnum {
    

    /// Unspecified.
    ///
    /// "OBJECT_TYPE_UNSPECIFIED"
    #[serde(rename="OBJECT_TYPE_UNSPECIFIED")]
    OBJECTTYPEUNSPECIFIED,
    

    /// Person.
    ///
    /// "PERSON"
    #[serde(rename="PERSON")]
    PERSON,
    

    /// [Currents Page.](https://workspace.google.com/products/currents/)
    ///
    /// "PAGE"
    #[serde(rename="PAGE")]
    PAGE,
}

impl AsRef<str> for PersonMetadataObjectTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonMetadataObjectTypeEnum::OBJECTTYPEUNSPECIFIED => "OBJECT_TYPE_UNSPECIFIED",
            PersonMetadataObjectTypeEnum::PERSON => "PERSON",
            PersonMetadataObjectTypeEnum::PAGE => "PAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonMetadataObjectTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OBJECT_TYPE_UNSPECIFIED" => Ok(PersonMetadataObjectTypeEnum::OBJECTTYPEUNSPECIFIED),
           "PERSON" => Ok(PersonMetadataObjectTypeEnum::PERSON),
           "PAGE" => Ok(PersonMetadataObjectTypeEnum::PAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonMetadataObjectTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProfileMetadataObjectTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The profile object type.
pub enum ProfileMetadataObjectTypeEnum {
    

    /// Unspecified.
    ///
    /// "OBJECT_TYPE_UNSPECIFIED"
    #[serde(rename="OBJECT_TYPE_UNSPECIFIED")]
    OBJECTTYPEUNSPECIFIED,
    

    /// Person.
    ///
    /// "PERSON"
    #[serde(rename="PERSON")]
    PERSON,
    

    /// [Currents Page.](https://workspace.google.com/products/currents/)
    ///
    /// "PAGE"
    #[serde(rename="PAGE")]
    PAGE,
}

impl AsRef<str> for ProfileMetadataObjectTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProfileMetadataObjectTypeEnum::OBJECTTYPEUNSPECIFIED => "OBJECT_TYPE_UNSPECIFIED",
            ProfileMetadataObjectTypeEnum::PERSON => "PERSON",
            ProfileMetadataObjectTypeEnum::PAGE => "PAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProfileMetadataObjectTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OBJECT_TYPE_UNSPECIFIED" => Ok(ProfileMetadataObjectTypeEnum::OBJECTTYPEUNSPECIFIED),
           "PERSON" => Ok(ProfileMetadataObjectTypeEnum::PERSON),
           "PAGE" => Ok(ProfileMetadataObjectTypeEnum::PAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProfileMetadataObjectTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProfileMetadataUserTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The user types.
pub enum ProfileMetadataUserTypesEnum {
    

    /// The user type is not known.
    ///
    /// "USER_TYPE_UNKNOWN"
    #[serde(rename="USER_TYPE_UNKNOWN")]
    USERTYPEUNKNOWN,
    

    /// The user is a Google user.
    ///
    /// "GOOGLE_USER"
    #[serde(rename="GOOGLE_USER")]
    GOOGLEUSER,
    

    /// The user is a Currents user.
    ///
    /// "GPLUS_USER"
    #[serde(rename="GPLUS_USER")]
    GPLUSUSER,
    

    /// The user is a Google Workspace user.
    ///
    /// "GOOGLE_APPS_USER"
    #[serde(rename="GOOGLE_APPS_USER")]
    GOOGLEAPPSUSER,
}

impl AsRef<str> for ProfileMetadataUserTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProfileMetadataUserTypesEnum::USERTYPEUNKNOWN => "USER_TYPE_UNKNOWN",
            ProfileMetadataUserTypesEnum::GOOGLEUSER => "GOOGLE_USER",
            ProfileMetadataUserTypesEnum::GPLUSUSER => "GPLUS_USER",
            ProfileMetadataUserTypesEnum::GOOGLEAPPSUSER => "GOOGLE_APPS_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for ProfileMetadataUserTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_TYPE_UNKNOWN" => Ok(ProfileMetadataUserTypesEnum::USERTYPEUNKNOWN),
           "GOOGLE_USER" => Ok(ProfileMetadataUserTypesEnum::GOOGLEUSER),
           "GPLUS_USER" => Ok(ProfileMetadataUserTypesEnum::GPLUSUSER),
           "GOOGLE_APPS_USER" => Ok(ProfileMetadataUserTypesEnum::GOOGLEAPPSUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProfileMetadataUserTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The source type.
pub enum SourceTypeEnum {
    

    /// Unspecified.
    ///
    /// "SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="SOURCE_TYPE_UNSPECIFIED")]
    SOURCETYPEUNSPECIFIED,
    

    /// [Google Account](https://accounts.google.com).
    ///
    /// "ACCOUNT"
    #[serde(rename="ACCOUNT")]
    ACCOUNT,
    

    /// [Google profile](https://profiles.google.com). You can view the profile at [https://profiles.google.com/](https://profiles.google.com/){id}, where {id} is the source id.
    ///
    /// "PROFILE"
    #[serde(rename="PROFILE")]
    PROFILE,
    

    /// [Google Workspace domain profile](https://support.google.com/a/answer/1628008).
    ///
    /// "DOMAIN_PROFILE"
    #[serde(rename="DOMAIN_PROFILE")]
    DOMAINPROFILE,
    

    /// [Google contact](https://contacts.google.com). You can view the contact at [https://contact.google.com/](https://contact.google.com/){id}, where {id} is the source id.
    ///
    /// "CONTACT"
    #[serde(rename="CONTACT")]
    CONTACT,
    

    /// [Google "Other contact"](https://contacts.google.com/other).
    ///
    /// "OTHER_CONTACT"
    #[serde(rename="OTHER_CONTACT")]
    OTHERCONTACT,
    

    /// [Google Workspace domain shared contact](https://support.google.com/a/answer/9281635).
    ///
    /// "DOMAIN_CONTACT"
    #[serde(rename="DOMAIN_CONTACT")]
    DOMAINCONTACT,
}

impl AsRef<str> for SourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SourceTypeEnum::SOURCETYPEUNSPECIFIED => "SOURCE_TYPE_UNSPECIFIED",
            SourceTypeEnum::ACCOUNT => "ACCOUNT",
            SourceTypeEnum::PROFILE => "PROFILE",
            SourceTypeEnum::DOMAINPROFILE => "DOMAIN_PROFILE",
            SourceTypeEnum::CONTACT => "CONTACT",
            SourceTypeEnum::OTHERCONTACT => "OTHER_CONTACT",
            SourceTypeEnum::DOMAINCONTACT => "DOMAIN_CONTACT",
        }
    }
}

impl std::convert::TryFrom< &str> for SourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SOURCE_TYPE_UNSPECIFIED" => Ok(SourceTypeEnum::SOURCETYPEUNSPECIFIED),
           "ACCOUNT" => Ok(SourceTypeEnum::ACCOUNT),
           "PROFILE" => Ok(SourceTypeEnum::PROFILE),
           "DOMAIN_PROFILE" => Ok(SourceTypeEnum::DOMAINPROFILE),
           "CONTACT" => Ok(SourceTypeEnum::CONTACT),
           "OTHER_CONTACT" => Ok(SourceTypeEnum::OTHERCONTACT),
           "DOMAIN_CONTACT" => Ok(SourceTypeEnum::DOMAINCONTACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UpdateContactPhotoRequestSourcesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.
pub enum UpdateContactPhotoRequestSourcesEnum {
    

    /// Unspecified.
    ///
    /// "READ_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="READ_SOURCE_TYPE_UNSPECIFIED")]
    READSOURCETYPEUNSPECIFIED,
    

    /// Returns SourceType.ACCOUNT, SourceType.DOMAIN_PROFILE, and SourceType.PROFILE.
    ///
    /// "READ_SOURCE_TYPE_PROFILE"
    #[serde(rename="READ_SOURCE_TYPE_PROFILE")]
    READSOURCETYPEPROFILE,
    

    /// Returns SourceType.CONTACT.
    ///
    /// "READ_SOURCE_TYPE_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_CONTACT")]
    READSOURCETYPECONTACT,
    

    /// Returns SourceType.DOMAIN_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_DOMAIN_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_DOMAIN_CONTACT")]
    READSOURCETYPEDOMAINCONTACT,
    

    /// Returns SourceType.OTHER_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_OTHER_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_OTHER_CONTACT")]
    READSOURCETYPEOTHERCONTACT,
}

impl AsRef<str> for UpdateContactPhotoRequestSourcesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UpdateContactPhotoRequestSourcesEnum::READSOURCETYPEUNSPECIFIED => "READ_SOURCE_TYPE_UNSPECIFIED",
            UpdateContactPhotoRequestSourcesEnum::READSOURCETYPEPROFILE => "READ_SOURCE_TYPE_PROFILE",
            UpdateContactPhotoRequestSourcesEnum::READSOURCETYPECONTACT => "READ_SOURCE_TYPE_CONTACT",
            UpdateContactPhotoRequestSourcesEnum::READSOURCETYPEDOMAINCONTACT => "READ_SOURCE_TYPE_DOMAIN_CONTACT",
            UpdateContactPhotoRequestSourcesEnum::READSOURCETYPEOTHERCONTACT => "READ_SOURCE_TYPE_OTHER_CONTACT",
        }
    }
}

impl std::convert::TryFrom< &str> for UpdateContactPhotoRequestSourcesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "READ_SOURCE_TYPE_UNSPECIFIED" => Ok(UpdateContactPhotoRequestSourcesEnum::READSOURCETYPEUNSPECIFIED),
           "READ_SOURCE_TYPE_PROFILE" => Ok(UpdateContactPhotoRequestSourcesEnum::READSOURCETYPEPROFILE),
           "READ_SOURCE_TYPE_CONTACT" => Ok(UpdateContactPhotoRequestSourcesEnum::READSOURCETYPECONTACT),
           "READ_SOURCE_TYPE_DOMAIN_CONTACT" => Ok(UpdateContactPhotoRequestSourcesEnum::READSOURCETYPEDOMAINCONTACT),
           "READ_SOURCE_TYPE_OTHER_CONTACT" => Ok(UpdateContactPhotoRequestSourcesEnum::READSOURCETYPEOTHERCONTACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UpdateContactPhotoRequestSourcesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OtherContactSourcesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT if not set. Possible values for this field are: * READ_SOURCE_TYPE_CONTACT * READ_SOURCE_TYPE_CONTACT,READ_SOURCE_TYPE_PROFILE Specifying READ_SOURCE_TYPE_PROFILE without specifying READ_SOURCE_TYPE_CONTACT is not permitted.
pub enum OtherContactSourcesEnum {
    

    /// Unspecified.
    ///
    /// "READ_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="READ_SOURCE_TYPE_UNSPECIFIED")]
    READSOURCETYPEUNSPECIFIED,
    

    /// Returns SourceType.ACCOUNT, SourceType.DOMAIN_PROFILE, and SourceType.PROFILE.
    ///
    /// "READ_SOURCE_TYPE_PROFILE"
    #[serde(rename="READ_SOURCE_TYPE_PROFILE")]
    READSOURCETYPEPROFILE,
    

    /// Returns SourceType.CONTACT.
    ///
    /// "READ_SOURCE_TYPE_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_CONTACT")]
    READSOURCETYPECONTACT,
    

    /// Returns SourceType.DOMAIN_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_DOMAIN_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_DOMAIN_CONTACT")]
    READSOURCETYPEDOMAINCONTACT,
    

    /// Returns SourceType.OTHER_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_OTHER_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_OTHER_CONTACT")]
    READSOURCETYPEOTHERCONTACT,
}

impl AsRef<str> for OtherContactSourcesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OtherContactSourcesEnum::READSOURCETYPEUNSPECIFIED => "READ_SOURCE_TYPE_UNSPECIFIED",
            OtherContactSourcesEnum::READSOURCETYPEPROFILE => "READ_SOURCE_TYPE_PROFILE",
            OtherContactSourcesEnum::READSOURCETYPECONTACT => "READ_SOURCE_TYPE_CONTACT",
            OtherContactSourcesEnum::READSOURCETYPEDOMAINCONTACT => "READ_SOURCE_TYPE_DOMAIN_CONTACT",
            OtherContactSourcesEnum::READSOURCETYPEOTHERCONTACT => "READ_SOURCE_TYPE_OTHER_CONTACT",
        }
    }
}

impl std::convert::TryFrom< &str> for OtherContactSourcesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "READ_SOURCE_TYPE_UNSPECIFIED" => Ok(OtherContactSourcesEnum::READSOURCETYPEUNSPECIFIED),
           "READ_SOURCE_TYPE_PROFILE" => Ok(OtherContactSourcesEnum::READSOURCETYPEPROFILE),
           "READ_SOURCE_TYPE_CONTACT" => Ok(OtherContactSourcesEnum::READSOURCETYPECONTACT),
           "READ_SOURCE_TYPE_DOMAIN_CONTACT" => Ok(OtherContactSourcesEnum::READSOURCETYPEDOMAINCONTACT),
           "READ_SOURCE_TYPE_OTHER_CONTACT" => Ok(OtherContactSourcesEnum::READSOURCETYPEOTHERCONTACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OtherContactSourcesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The order in which the connections should be sorted. Defaults to `LAST_MODIFIED_ASCENDING`.
pub enum PersonSortOrderEnum {
    

    /// Sort people by when they were changed; older entries first.
    ///
    /// "LAST_MODIFIED_ASCENDING"
    #[serde(rename="LAST_MODIFIED_ASCENDING")]
    LASTMODIFIEDASCENDING,
    

    /// Sort people by when they were changed; newer entries first.
    ///
    /// "LAST_MODIFIED_DESCENDING"
    #[serde(rename="LAST_MODIFIED_DESCENDING")]
    LASTMODIFIEDDESCENDING,
    

    /// Sort people by first name.
    ///
    /// "FIRST_NAME_ASCENDING"
    #[serde(rename="FIRST_NAME_ASCENDING")]
    FIRSTNAMEASCENDING,
    

    /// Sort people by last name.
    ///
    /// "LAST_NAME_ASCENDING"
    #[serde(rename="LAST_NAME_ASCENDING")]
    LASTNAMEASCENDING,
}

impl AsRef<str> for PersonSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonSortOrderEnum::LASTMODIFIEDASCENDING => "LAST_MODIFIED_ASCENDING",
            PersonSortOrderEnum::LASTMODIFIEDDESCENDING => "LAST_MODIFIED_DESCENDING",
            PersonSortOrderEnum::FIRSTNAMEASCENDING => "FIRST_NAME_ASCENDING",
            PersonSortOrderEnum::LASTNAMEASCENDING => "LAST_NAME_ASCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAST_MODIFIED_ASCENDING" => Ok(PersonSortOrderEnum::LASTMODIFIEDASCENDING),
           "LAST_MODIFIED_DESCENDING" => Ok(PersonSortOrderEnum::LASTMODIFIEDDESCENDING),
           "FIRST_NAME_ASCENDING" => Ok(PersonSortOrderEnum::FIRSTNAMEASCENDING),
           "LAST_NAME_ASCENDING" => Ok(PersonSortOrderEnum::LASTNAMEASCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonSourcesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.
pub enum PersonSourcesEnum {
    

    /// Unspecified.
    ///
    /// "READ_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="READ_SOURCE_TYPE_UNSPECIFIED")]
    READSOURCETYPEUNSPECIFIED,
    

    /// Returns SourceType.ACCOUNT, SourceType.DOMAIN_PROFILE, and SourceType.PROFILE.
    ///
    /// "READ_SOURCE_TYPE_PROFILE"
    #[serde(rename="READ_SOURCE_TYPE_PROFILE")]
    READSOURCETYPEPROFILE,
    

    /// Returns SourceType.CONTACT.
    ///
    /// "READ_SOURCE_TYPE_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_CONTACT")]
    READSOURCETYPECONTACT,
    

    /// Returns SourceType.DOMAIN_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_DOMAIN_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_DOMAIN_CONTACT")]
    READSOURCETYPEDOMAINCONTACT,
    

    /// Returns SourceType.OTHER_CONTACT.
    ///
    /// "READ_SOURCE_TYPE_OTHER_CONTACT"
    #[serde(rename="READ_SOURCE_TYPE_OTHER_CONTACT")]
    READSOURCETYPEOTHERCONTACT,
}

impl AsRef<str> for PersonSourcesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonSourcesEnum::READSOURCETYPEUNSPECIFIED => "READ_SOURCE_TYPE_UNSPECIFIED",
            PersonSourcesEnum::READSOURCETYPEPROFILE => "READ_SOURCE_TYPE_PROFILE",
            PersonSourcesEnum::READSOURCETYPECONTACT => "READ_SOURCE_TYPE_CONTACT",
            PersonSourcesEnum::READSOURCETYPEDOMAINCONTACT => "READ_SOURCE_TYPE_DOMAIN_CONTACT",
            PersonSourcesEnum::READSOURCETYPEOTHERCONTACT => "READ_SOURCE_TYPE_OTHER_CONTACT",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonSourcesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "READ_SOURCE_TYPE_UNSPECIFIED" => Ok(PersonSourcesEnum::READSOURCETYPEUNSPECIFIED),
           "READ_SOURCE_TYPE_PROFILE" => Ok(PersonSourcesEnum::READSOURCETYPEPROFILE),
           "READ_SOURCE_TYPE_CONTACT" => Ok(PersonSourcesEnum::READSOURCETYPECONTACT),
           "READ_SOURCE_TYPE_DOMAIN_CONTACT" => Ok(PersonSourcesEnum::READSOURCETYPEDOMAINCONTACT),
           "READ_SOURCE_TYPE_OTHER_CONTACT" => Ok(PersonSourcesEnum::READSOURCETYPEOTHERCONTACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonSourcesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonMergeSourcesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Additional data to merge into the directory sources if they are connected through verified join keys such as email addresses or phone numbers.
pub enum PersonMergeSourcesEnum {
    

    /// Unspecified.
    ///
    /// "DIRECTORY_MERGE_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="DIRECTORY_MERGE_SOURCE_TYPE_UNSPECIFIED")]
    DIRECTORYMERGESOURCETYPEUNSPECIFIED,
    

    /// User owned contact.
    ///
    /// "DIRECTORY_MERGE_SOURCE_TYPE_CONTACT"
    #[serde(rename="DIRECTORY_MERGE_SOURCE_TYPE_CONTACT")]
    DIRECTORYMERGESOURCETYPECONTACT,
}

impl AsRef<str> for PersonMergeSourcesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonMergeSourcesEnum::DIRECTORYMERGESOURCETYPEUNSPECIFIED => "DIRECTORY_MERGE_SOURCE_TYPE_UNSPECIFIED",
            PersonMergeSourcesEnum::DIRECTORYMERGESOURCETYPECONTACT => "DIRECTORY_MERGE_SOURCE_TYPE_CONTACT",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonMergeSourcesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIRECTORY_MERGE_SOURCE_TYPE_UNSPECIFIED" => Ok(PersonMergeSourcesEnum::DIRECTORYMERGESOURCETYPEUNSPECIFIED),
           "DIRECTORY_MERGE_SOURCE_TYPE_CONTACT" => Ok(PersonMergeSourcesEnum::DIRECTORYMERGESOURCETYPECONTACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonMergeSourcesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


