use super::*;



// region NotificationSettingNotificationTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The types of notifications that will be sent to the Pub/Sub topic. To stop receiving notifications entirely, use NotificationSettings.UpdateNotificationSetting with an empty notification_types or set the pubsub_topic to an empty string.
pub enum NotificationSettingNotificationTypesEnum {
    

    /// No notification type. Will not match any notifications.
    ///
    /// "NOTIFICATION_TYPE_UNSPECIFIED"
    #[serde(rename="NOTIFICATION_TYPE_UNSPECIFIED")]
    NOTIFICATIONTYPEUNSPECIFIED,
    

    /// The location has Google updates for review. The location_name field on the notification will provide the resource name of the location with Google updates.
    ///
    /// "GOOGLE_UPDATE"
    #[serde(rename="GOOGLE_UPDATE")]
    GOOGLEUPDATE,
    

    /// A new review has been added to the location. The review_name field on the notification will provide the resource name of the review that was added, and location_name will have the location's resource name.
    ///
    /// "NEW_REVIEW"
    #[serde(rename="NEW_REVIEW")]
    NEWREVIEW,
    

    /// A review on the location has been updated. The review_name field on the notification will provide the resource name of the review that was added, and location_name will have the location's resource name.
    ///
    /// "UPDATED_REVIEW"
    #[serde(rename="UPDATED_REVIEW")]
    UPDATEDREVIEW,
    

    /// A new media item has been added to the location by a Google Maps user. The notification will provide the resource name of the new media item.
    ///
    /// "NEW_CUSTOMER_MEDIA"
    #[serde(rename="NEW_CUSTOMER_MEDIA")]
    NEWCUSTOMERMEDIA,
    

    /// A new question is added to the location. The notification will provide the resource name of question.
    ///
    /// "NEW_QUESTION"
    #[serde(rename="NEW_QUESTION")]
    NEWQUESTION,
    

    /// A question of the location is updated. The notification will provide the resource name of question.
    ///
    /// "UPDATED_QUESTION"
    #[serde(rename="UPDATED_QUESTION")]
    UPDATEDQUESTION,
    

    /// A new answer is added to the location. The notification will provide the resource name of question and answer.
    ///
    /// "NEW_ANSWER"
    #[serde(rename="NEW_ANSWER")]
    NEWANSWER,
    

    /// An answer of the location is updated. The notification will provide the resource name of question and answer.
    ///
    /// "UPDATED_ANSWER"
    #[serde(rename="UPDATED_ANSWER")]
    UPDATEDANSWER,
    

    /// Indicates whether there is a change in location metadata's duplicate location field.
    ///
    /// "DUPLICATE_LOCATION"
    #[serde(rename="DUPLICATE_LOCATION")]
    DUPLICATELOCATION,
    

    /// Deprecated: Migrate the existing usages of this value to the more expanded "VOICE_OF_MERCHANT_UPDATED".
    ///
    /// "LOSS_OF_VOICE_OF_MERCHANT"
    #[serde(rename="LOSS_OF_VOICE_OF_MERCHANT")]
    LOSSOFVOICEOFMERCHANT,
    

    /// Indicates whether the location has an update in Voice of Merchant (VOM) status. VOM dictates whether the location is in good standing and the merchant has control over the business on Google. Any edits made to the location will propagate to Maps after passing the review phase. Call GetVoiceOfMerchantState rpc for more details.
    ///
    /// "VOICE_OF_MERCHANT_UPDATED"
    #[serde(rename="VOICE_OF_MERCHANT_UPDATED")]
    VOICEOFMERCHANTUPDATED,
}

impl AsRef<str> for NotificationSettingNotificationTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NotificationSettingNotificationTypesEnum::NOTIFICATIONTYPEUNSPECIFIED => "NOTIFICATION_TYPE_UNSPECIFIED",
            NotificationSettingNotificationTypesEnum::GOOGLEUPDATE => "GOOGLE_UPDATE",
            NotificationSettingNotificationTypesEnum::NEWREVIEW => "NEW_REVIEW",
            NotificationSettingNotificationTypesEnum::UPDATEDREVIEW => "UPDATED_REVIEW",
            NotificationSettingNotificationTypesEnum::NEWCUSTOMERMEDIA => "NEW_CUSTOMER_MEDIA",
            NotificationSettingNotificationTypesEnum::NEWQUESTION => "NEW_QUESTION",
            NotificationSettingNotificationTypesEnum::UPDATEDQUESTION => "UPDATED_QUESTION",
            NotificationSettingNotificationTypesEnum::NEWANSWER => "NEW_ANSWER",
            NotificationSettingNotificationTypesEnum::UPDATEDANSWER => "UPDATED_ANSWER",
            NotificationSettingNotificationTypesEnum::DUPLICATELOCATION => "DUPLICATE_LOCATION",
            NotificationSettingNotificationTypesEnum::LOSSOFVOICEOFMERCHANT => "LOSS_OF_VOICE_OF_MERCHANT",
            NotificationSettingNotificationTypesEnum::VOICEOFMERCHANTUPDATED => "VOICE_OF_MERCHANT_UPDATED",
        }
    }
}

impl std::convert::TryFrom< &str> for NotificationSettingNotificationTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOTIFICATION_TYPE_UNSPECIFIED" => Ok(NotificationSettingNotificationTypesEnum::NOTIFICATIONTYPEUNSPECIFIED),
           "GOOGLE_UPDATE" => Ok(NotificationSettingNotificationTypesEnum::GOOGLEUPDATE),
           "NEW_REVIEW" => Ok(NotificationSettingNotificationTypesEnum::NEWREVIEW),
           "UPDATED_REVIEW" => Ok(NotificationSettingNotificationTypesEnum::UPDATEDREVIEW),
           "NEW_CUSTOMER_MEDIA" => Ok(NotificationSettingNotificationTypesEnum::NEWCUSTOMERMEDIA),
           "NEW_QUESTION" => Ok(NotificationSettingNotificationTypesEnum::NEWQUESTION),
           "UPDATED_QUESTION" => Ok(NotificationSettingNotificationTypesEnum::UPDATEDQUESTION),
           "NEW_ANSWER" => Ok(NotificationSettingNotificationTypesEnum::NEWANSWER),
           "UPDATED_ANSWER" => Ok(NotificationSettingNotificationTypesEnum::UPDATEDANSWER),
           "DUPLICATE_LOCATION" => Ok(NotificationSettingNotificationTypesEnum::DUPLICATELOCATION),
           "LOSS_OF_VOICE_OF_MERCHANT" => Ok(NotificationSettingNotificationTypesEnum::LOSSOFVOICEOFMERCHANT),
           "VOICE_OF_MERCHANT_UPDATED" => Ok(NotificationSettingNotificationTypesEnum::VOICEOFMERCHANTUPDATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NotificationSettingNotificationTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


