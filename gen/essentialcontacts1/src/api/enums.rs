use super::*;



// region GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The categories of notifications that the contact will receive communications for.
pub enum GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum {
    

    /// Notification category is unrecognized or unspecified.
    ///
    /// "NOTIFICATION_CATEGORY_UNSPECIFIED"
    #[serde(rename="NOTIFICATION_CATEGORY_UNSPECIFIED")]
    NOTIFICATIONCATEGORYUNSPECIFIED,
    

    /// All notifications related to the resource, including notifications pertaining to categories added in the future.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// Notifications related to imminent account suspension.
    ///
    /// "SUSPENSION"
    #[serde(rename="SUSPENSION")]
    SUSPENSION,
    

    /// Notifications related to security/privacy incidents, notifications, and vulnerabilities.
    ///
    /// "SECURITY"
    #[serde(rename="SECURITY")]
    SECURITY,
    

    /// Notifications related to technical events and issues such as outages, errors, or bugs.
    ///
    /// "TECHNICAL"
    #[serde(rename="TECHNICAL")]
    TECHNICAL,
    

    /// Notifications related to billing and payments notifications, price updates, errors, or credits.
    ///
    /// "BILLING"
    #[serde(rename="BILLING")]
    BILLING,
    

    /// Notifications related to enforcement actions, regulatory compliance, or government notices.
    ///
    /// "LEGAL"
    #[serde(rename="LEGAL")]
    LEGAL,
    

    /// Notifications related to new versions, product terms updates, or deprecations.
    ///
    /// "PRODUCT_UPDATES"
    #[serde(rename="PRODUCT_UPDATES")]
    PRODUCTUPDATES,
    

    /// Child category of TECHNICAL. If assigned, technical incident notifications will go to these contacts instead of TECHNICAL.
    ///
    /// "TECHNICAL_INCIDENTS"
    #[serde(rename="TECHNICAL_INCIDENTS")]
    TECHNICALINCIDENTS,
}

impl AsRef<str> for GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::NOTIFICATIONCATEGORYUNSPECIFIED => "NOTIFICATION_CATEGORY_UNSPECIFIED",
            GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::ALL => "ALL",
            GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::SUSPENSION => "SUSPENSION",
            GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::SECURITY => "SECURITY",
            GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::TECHNICAL => "TECHNICAL",
            GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::BILLING => "BILLING",
            GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::LEGAL => "LEGAL",
            GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::PRODUCTUPDATES => "PRODUCT_UPDATES",
            GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::TECHNICALINCIDENTS => "TECHNICAL_INCIDENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOTIFICATION_CATEGORY_UNSPECIFIED" => Ok(GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::NOTIFICATIONCATEGORYUNSPECIFIED),
           "ALL" => Ok(GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::ALL),
           "SUSPENSION" => Ok(GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::SUSPENSION),
           "SECURITY" => Ok(GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::SECURITY),
           "TECHNICAL" => Ok(GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::TECHNICAL),
           "BILLING" => Ok(GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::BILLING),
           "LEGAL" => Ok(GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::LEGAL),
           "PRODUCT_UPDATES" => Ok(GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::PRODUCTUPDATES),
           "TECHNICAL_INCIDENTS" => Ok(GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum::TECHNICALINCIDENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudEssentialcontactsV1ContactValidationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The validity of the contact. A contact is considered valid if it is the correct recipient for notifications for a particular resource.
pub enum GoogleCloudEssentialcontactsV1ContactValidationStateEnum {
    

    /// The validation state is unknown or unspecified.
    ///
    /// "VALIDATION_STATE_UNSPECIFIED"
    #[serde(rename="VALIDATION_STATE_UNSPECIFIED")]
    VALIDATIONSTATEUNSPECIFIED,
    

    /// The contact is marked as valid. This is usually done manually by the contact admin. All new contacts begin in the valid state.
    ///
    /// "VALID"
    #[serde(rename="VALID")]
    VALID,
    

    /// The contact is considered invalid. This may become the state if the contact's email is found to be unreachable.
    ///
    /// "INVALID"
    #[serde(rename="INVALID")]
    INVALID,
}

impl AsRef<str> for GoogleCloudEssentialcontactsV1ContactValidationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudEssentialcontactsV1ContactValidationStateEnum::VALIDATIONSTATEUNSPECIFIED => "VALIDATION_STATE_UNSPECIFIED",
            GoogleCloudEssentialcontactsV1ContactValidationStateEnum::VALID => "VALID",
            GoogleCloudEssentialcontactsV1ContactValidationStateEnum::INVALID => "INVALID",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudEssentialcontactsV1ContactValidationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VALIDATION_STATE_UNSPECIFIED" => Ok(GoogleCloudEssentialcontactsV1ContactValidationStateEnum::VALIDATIONSTATEUNSPECIFIED),
           "VALID" => Ok(GoogleCloudEssentialcontactsV1ContactValidationStateEnum::VALID),
           "INVALID" => Ok(GoogleCloudEssentialcontactsV1ContactValidationStateEnum::INVALID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudEssentialcontactsV1ContactValidationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The notification category to send the test message for. All contacts must be subscribed to this category.
pub enum GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum {
    

    /// Notification category is unrecognized or unspecified.
    ///
    /// "NOTIFICATION_CATEGORY_UNSPECIFIED"
    #[serde(rename="NOTIFICATION_CATEGORY_UNSPECIFIED")]
    NOTIFICATIONCATEGORYUNSPECIFIED,
    

    /// All notifications related to the resource, including notifications pertaining to categories added in the future.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// Notifications related to imminent account suspension.
    ///
    /// "SUSPENSION"
    #[serde(rename="SUSPENSION")]
    SUSPENSION,
    

    /// Notifications related to security/privacy incidents, notifications, and vulnerabilities.
    ///
    /// "SECURITY"
    #[serde(rename="SECURITY")]
    SECURITY,
    

    /// Notifications related to technical events and issues such as outages, errors, or bugs.
    ///
    /// "TECHNICAL"
    #[serde(rename="TECHNICAL")]
    TECHNICAL,
    

    /// Notifications related to billing and payments notifications, price updates, errors, or credits.
    ///
    /// "BILLING"
    #[serde(rename="BILLING")]
    BILLING,
    

    /// Notifications related to enforcement actions, regulatory compliance, or government notices.
    ///
    /// "LEGAL"
    #[serde(rename="LEGAL")]
    LEGAL,
    

    /// Notifications related to new versions, product terms updates, or deprecations.
    ///
    /// "PRODUCT_UPDATES"
    #[serde(rename="PRODUCT_UPDATES")]
    PRODUCTUPDATES,
    

    /// Child category of TECHNICAL. If assigned, technical incident notifications will go to these contacts instead of TECHNICAL.
    ///
    /// "TECHNICAL_INCIDENTS"
    #[serde(rename="TECHNICAL_INCIDENTS")]
    TECHNICALINCIDENTS,
}

impl AsRef<str> for GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::NOTIFICATIONCATEGORYUNSPECIFIED => "NOTIFICATION_CATEGORY_UNSPECIFIED",
            GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::ALL => "ALL",
            GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::SUSPENSION => "SUSPENSION",
            GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::SECURITY => "SECURITY",
            GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::TECHNICAL => "TECHNICAL",
            GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::BILLING => "BILLING",
            GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::LEGAL => "LEGAL",
            GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::PRODUCTUPDATES => "PRODUCT_UPDATES",
            GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::TECHNICALINCIDENTS => "TECHNICAL_INCIDENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOTIFICATION_CATEGORY_UNSPECIFIED" => Ok(GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::NOTIFICATIONCATEGORYUNSPECIFIED),
           "ALL" => Ok(GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::ALL),
           "SUSPENSION" => Ok(GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::SUSPENSION),
           "SECURITY" => Ok(GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::SECURITY),
           "TECHNICAL" => Ok(GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::TECHNICAL),
           "BILLING" => Ok(GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::BILLING),
           "LEGAL" => Ok(GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::LEGAL),
           "PRODUCT_UPDATES" => Ok(GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::PRODUCTUPDATES),
           "TECHNICAL_INCIDENTS" => Ok(GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum::TECHNICALINCIDENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FolderNotificationCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The categories of notifications to compute contacts for. If ALL is included in this list, contacts subscribed to any notification category will be returned.
pub enum FolderNotificationCategoriesEnum {
    

    /// Notification category is unrecognized or unspecified.
    ///
    /// "NOTIFICATION_CATEGORY_UNSPECIFIED"
    #[serde(rename="NOTIFICATION_CATEGORY_UNSPECIFIED")]
    NOTIFICATIONCATEGORYUNSPECIFIED,
    

    /// All notifications related to the resource, including notifications pertaining to categories added in the future.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// Notifications related to imminent account suspension.
    ///
    /// "SUSPENSION"
    #[serde(rename="SUSPENSION")]
    SUSPENSION,
    

    /// Notifications related to security/privacy incidents, notifications, and vulnerabilities.
    ///
    /// "SECURITY"
    #[serde(rename="SECURITY")]
    SECURITY,
    

    /// Notifications related to technical events and issues such as outages, errors, or bugs.
    ///
    /// "TECHNICAL"
    #[serde(rename="TECHNICAL")]
    TECHNICAL,
    

    /// Notifications related to billing and payments notifications, price updates, errors, or credits.
    ///
    /// "BILLING"
    #[serde(rename="BILLING")]
    BILLING,
    

    /// Notifications related to enforcement actions, regulatory compliance, or government notices.
    ///
    /// "LEGAL"
    #[serde(rename="LEGAL")]
    LEGAL,
    

    /// Notifications related to new versions, product terms updates, or deprecations.
    ///
    /// "PRODUCT_UPDATES"
    #[serde(rename="PRODUCT_UPDATES")]
    PRODUCTUPDATES,
    

    /// Child category of TECHNICAL. If assigned, technical incident notifications will go to these contacts instead of TECHNICAL.
    ///
    /// "TECHNICAL_INCIDENTS"
    #[serde(rename="TECHNICAL_INCIDENTS")]
    TECHNICALINCIDENTS,
}

impl AsRef<str> for FolderNotificationCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FolderNotificationCategoriesEnum::NOTIFICATIONCATEGORYUNSPECIFIED => "NOTIFICATION_CATEGORY_UNSPECIFIED",
            FolderNotificationCategoriesEnum::ALL => "ALL",
            FolderNotificationCategoriesEnum::SUSPENSION => "SUSPENSION",
            FolderNotificationCategoriesEnum::SECURITY => "SECURITY",
            FolderNotificationCategoriesEnum::TECHNICAL => "TECHNICAL",
            FolderNotificationCategoriesEnum::BILLING => "BILLING",
            FolderNotificationCategoriesEnum::LEGAL => "LEGAL",
            FolderNotificationCategoriesEnum::PRODUCTUPDATES => "PRODUCT_UPDATES",
            FolderNotificationCategoriesEnum::TECHNICALINCIDENTS => "TECHNICAL_INCIDENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for FolderNotificationCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOTIFICATION_CATEGORY_UNSPECIFIED" => Ok(FolderNotificationCategoriesEnum::NOTIFICATIONCATEGORYUNSPECIFIED),
           "ALL" => Ok(FolderNotificationCategoriesEnum::ALL),
           "SUSPENSION" => Ok(FolderNotificationCategoriesEnum::SUSPENSION),
           "SECURITY" => Ok(FolderNotificationCategoriesEnum::SECURITY),
           "TECHNICAL" => Ok(FolderNotificationCategoriesEnum::TECHNICAL),
           "BILLING" => Ok(FolderNotificationCategoriesEnum::BILLING),
           "LEGAL" => Ok(FolderNotificationCategoriesEnum::LEGAL),
           "PRODUCT_UPDATES" => Ok(FolderNotificationCategoriesEnum::PRODUCTUPDATES),
           "TECHNICAL_INCIDENTS" => Ok(FolderNotificationCategoriesEnum::TECHNICALINCIDENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FolderNotificationCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationNotificationCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The categories of notifications to compute contacts for. If ALL is included in this list, contacts subscribed to any notification category will be returned.
pub enum OrganizationNotificationCategoriesEnum {
    

    /// Notification category is unrecognized or unspecified.
    ///
    /// "NOTIFICATION_CATEGORY_UNSPECIFIED"
    #[serde(rename="NOTIFICATION_CATEGORY_UNSPECIFIED")]
    NOTIFICATIONCATEGORYUNSPECIFIED,
    

    /// All notifications related to the resource, including notifications pertaining to categories added in the future.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// Notifications related to imminent account suspension.
    ///
    /// "SUSPENSION"
    #[serde(rename="SUSPENSION")]
    SUSPENSION,
    

    /// Notifications related to security/privacy incidents, notifications, and vulnerabilities.
    ///
    /// "SECURITY"
    #[serde(rename="SECURITY")]
    SECURITY,
    

    /// Notifications related to technical events and issues such as outages, errors, or bugs.
    ///
    /// "TECHNICAL"
    #[serde(rename="TECHNICAL")]
    TECHNICAL,
    

    /// Notifications related to billing and payments notifications, price updates, errors, or credits.
    ///
    /// "BILLING"
    #[serde(rename="BILLING")]
    BILLING,
    

    /// Notifications related to enforcement actions, regulatory compliance, or government notices.
    ///
    /// "LEGAL"
    #[serde(rename="LEGAL")]
    LEGAL,
    

    /// Notifications related to new versions, product terms updates, or deprecations.
    ///
    /// "PRODUCT_UPDATES"
    #[serde(rename="PRODUCT_UPDATES")]
    PRODUCTUPDATES,
    

    /// Child category of TECHNICAL. If assigned, technical incident notifications will go to these contacts instead of TECHNICAL.
    ///
    /// "TECHNICAL_INCIDENTS"
    #[serde(rename="TECHNICAL_INCIDENTS")]
    TECHNICALINCIDENTS,
}

impl AsRef<str> for OrganizationNotificationCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationNotificationCategoriesEnum::NOTIFICATIONCATEGORYUNSPECIFIED => "NOTIFICATION_CATEGORY_UNSPECIFIED",
            OrganizationNotificationCategoriesEnum::ALL => "ALL",
            OrganizationNotificationCategoriesEnum::SUSPENSION => "SUSPENSION",
            OrganizationNotificationCategoriesEnum::SECURITY => "SECURITY",
            OrganizationNotificationCategoriesEnum::TECHNICAL => "TECHNICAL",
            OrganizationNotificationCategoriesEnum::BILLING => "BILLING",
            OrganizationNotificationCategoriesEnum::LEGAL => "LEGAL",
            OrganizationNotificationCategoriesEnum::PRODUCTUPDATES => "PRODUCT_UPDATES",
            OrganizationNotificationCategoriesEnum::TECHNICALINCIDENTS => "TECHNICAL_INCIDENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationNotificationCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOTIFICATION_CATEGORY_UNSPECIFIED" => Ok(OrganizationNotificationCategoriesEnum::NOTIFICATIONCATEGORYUNSPECIFIED),
           "ALL" => Ok(OrganizationNotificationCategoriesEnum::ALL),
           "SUSPENSION" => Ok(OrganizationNotificationCategoriesEnum::SUSPENSION),
           "SECURITY" => Ok(OrganizationNotificationCategoriesEnum::SECURITY),
           "TECHNICAL" => Ok(OrganizationNotificationCategoriesEnum::TECHNICAL),
           "BILLING" => Ok(OrganizationNotificationCategoriesEnum::BILLING),
           "LEGAL" => Ok(OrganizationNotificationCategoriesEnum::LEGAL),
           "PRODUCT_UPDATES" => Ok(OrganizationNotificationCategoriesEnum::PRODUCTUPDATES),
           "TECHNICAL_INCIDENTS" => Ok(OrganizationNotificationCategoriesEnum::TECHNICALINCIDENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationNotificationCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectNotificationCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The categories of notifications to compute contacts for. If ALL is included in this list, contacts subscribed to any notification category will be returned.
pub enum ProjectNotificationCategoriesEnum {
    

    /// Notification category is unrecognized or unspecified.
    ///
    /// "NOTIFICATION_CATEGORY_UNSPECIFIED"
    #[serde(rename="NOTIFICATION_CATEGORY_UNSPECIFIED")]
    NOTIFICATIONCATEGORYUNSPECIFIED,
    

    /// All notifications related to the resource, including notifications pertaining to categories added in the future.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// Notifications related to imminent account suspension.
    ///
    /// "SUSPENSION"
    #[serde(rename="SUSPENSION")]
    SUSPENSION,
    

    /// Notifications related to security/privacy incidents, notifications, and vulnerabilities.
    ///
    /// "SECURITY"
    #[serde(rename="SECURITY")]
    SECURITY,
    

    /// Notifications related to technical events and issues such as outages, errors, or bugs.
    ///
    /// "TECHNICAL"
    #[serde(rename="TECHNICAL")]
    TECHNICAL,
    

    /// Notifications related to billing and payments notifications, price updates, errors, or credits.
    ///
    /// "BILLING"
    #[serde(rename="BILLING")]
    BILLING,
    

    /// Notifications related to enforcement actions, regulatory compliance, or government notices.
    ///
    /// "LEGAL"
    #[serde(rename="LEGAL")]
    LEGAL,
    

    /// Notifications related to new versions, product terms updates, or deprecations.
    ///
    /// "PRODUCT_UPDATES"
    #[serde(rename="PRODUCT_UPDATES")]
    PRODUCTUPDATES,
    

    /// Child category of TECHNICAL. If assigned, technical incident notifications will go to these contacts instead of TECHNICAL.
    ///
    /// "TECHNICAL_INCIDENTS"
    #[serde(rename="TECHNICAL_INCIDENTS")]
    TECHNICALINCIDENTS,
}

impl AsRef<str> for ProjectNotificationCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectNotificationCategoriesEnum::NOTIFICATIONCATEGORYUNSPECIFIED => "NOTIFICATION_CATEGORY_UNSPECIFIED",
            ProjectNotificationCategoriesEnum::ALL => "ALL",
            ProjectNotificationCategoriesEnum::SUSPENSION => "SUSPENSION",
            ProjectNotificationCategoriesEnum::SECURITY => "SECURITY",
            ProjectNotificationCategoriesEnum::TECHNICAL => "TECHNICAL",
            ProjectNotificationCategoriesEnum::BILLING => "BILLING",
            ProjectNotificationCategoriesEnum::LEGAL => "LEGAL",
            ProjectNotificationCategoriesEnum::PRODUCTUPDATES => "PRODUCT_UPDATES",
            ProjectNotificationCategoriesEnum::TECHNICALINCIDENTS => "TECHNICAL_INCIDENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectNotificationCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOTIFICATION_CATEGORY_UNSPECIFIED" => Ok(ProjectNotificationCategoriesEnum::NOTIFICATIONCATEGORYUNSPECIFIED),
           "ALL" => Ok(ProjectNotificationCategoriesEnum::ALL),
           "SUSPENSION" => Ok(ProjectNotificationCategoriesEnum::SUSPENSION),
           "SECURITY" => Ok(ProjectNotificationCategoriesEnum::SECURITY),
           "TECHNICAL" => Ok(ProjectNotificationCategoriesEnum::TECHNICAL),
           "BILLING" => Ok(ProjectNotificationCategoriesEnum::BILLING),
           "LEGAL" => Ok(ProjectNotificationCategoriesEnum::LEGAL),
           "PRODUCT_UPDATES" => Ok(ProjectNotificationCategoriesEnum::PRODUCTUPDATES),
           "TECHNICAL_INCIDENTS" => Ok(ProjectNotificationCategoriesEnum::TECHNICALINCIDENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectNotificationCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


