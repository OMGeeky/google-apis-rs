use super::*;



// region AndroidConfigPriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Message priority. Can take "normal" and "high" values. For more information, see [Setting the priority of a message](https://goo.gl/GjONJv).
pub enum AndroidConfigPriorityEnum {
    

    /// Default priority for data messages. Normal priority messages won't open network connections on a sleeping device, and their delivery may be delayed to conserve the battery. For less time-sensitive messages, such as notifications of new email or other data to sync, choose normal delivery priority.
    ///
    /// "NORMAL"
    #[serde(rename="NORMAL")]
    NORMAL,
    

    /// Default priority for notification messages. FCM attempts to deliver high priority messages immediately, allowing the FCM service to wake a sleeping device when possible and open a network connection to your app server. Apps with instant messaging, chat, or voice call alerts, for example, generally need to open a network connection and make sure FCM delivers the message to the device without delay. Set high priority if the message is time-critical and requires the user's immediate interaction, but beware that setting your messages to high priority contributes more to battery drain compared with normal priority messages.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
}

impl AsRef<str> for AndroidConfigPriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AndroidConfigPriorityEnum::NORMAL => "NORMAL",
            AndroidConfigPriorityEnum::HIGH => "HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for AndroidConfigPriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NORMAL" => Ok(AndroidConfigPriorityEnum::NORMAL),
           "HIGH" => Ok(AndroidConfigPriorityEnum::HIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AndroidConfigPriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AndroidNotificationNotificationPriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Set the relative priority for this notification. Priority is an indication of how much of the user's attention should be consumed by this notification. Low-priority notifications may be hidden from the user in certain situations, while the user might be interrupted for a higher-priority notification. The effect of setting the same priorities may differ slightly on different platforms. Note this priority differs from `AndroidMessagePriority`. This priority is processed by the client after the message has been delivered, whereas [AndroidMessagePriority](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidmessagepriority) is an FCM concept that controls when the message is delivered.
pub enum AndroidNotificationNotificationPriorityEnum {
    

    /// If priority is unspecified, notification priority is set to `PRIORITY_DEFAULT`.
    ///
    /// "PRIORITY_UNSPECIFIED"
    #[serde(rename="PRIORITY_UNSPECIFIED")]
    PRIORITYUNSPECIFIED,
    

    /// Lowest notification priority. Notifications with this `PRIORITY_MIN` might not be shown to the user except under special circumstances, such as detailed notification logs.
    ///
    /// "PRIORITY_MIN"
    #[serde(rename="PRIORITY_MIN")]
    PRIORITYMIN,
    

    /// Lower notification priority. The UI may choose to show the notifications smaller, or at a different position in the list, compared with notifications with `PRIORITY_DEFAULT`.
    ///
    /// "PRIORITY_LOW"
    #[serde(rename="PRIORITY_LOW")]
    PRIORITYLOW,
    

    /// Default notification priority. If the application does not prioritize its own notifications, use this value for all notifications.
    ///
    /// "PRIORITY_DEFAULT"
    #[serde(rename="PRIORITY_DEFAULT")]
    PRIORITYDEFAULT,
    

    /// Higher notification priority. Use this for more important notifications or alerts. The UI may choose to show these notifications larger, or at a different position in the notification lists, compared with notifications with `PRIORITY_DEFAULT`.
    ///
    /// "PRIORITY_HIGH"
    #[serde(rename="PRIORITY_HIGH")]
    PRIORITYHIGH,
    

    /// Highest notification priority. Use this for the application's most important items that require the user's prompt attention or input.
    ///
    /// "PRIORITY_MAX"
    #[serde(rename="PRIORITY_MAX")]
    PRIORITYMAX,
}

impl AsRef<str> for AndroidNotificationNotificationPriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AndroidNotificationNotificationPriorityEnum::PRIORITYUNSPECIFIED => "PRIORITY_UNSPECIFIED",
            AndroidNotificationNotificationPriorityEnum::PRIORITYMIN => "PRIORITY_MIN",
            AndroidNotificationNotificationPriorityEnum::PRIORITYLOW => "PRIORITY_LOW",
            AndroidNotificationNotificationPriorityEnum::PRIORITYDEFAULT => "PRIORITY_DEFAULT",
            AndroidNotificationNotificationPriorityEnum::PRIORITYHIGH => "PRIORITY_HIGH",
            AndroidNotificationNotificationPriorityEnum::PRIORITYMAX => "PRIORITY_MAX",
        }
    }
}

impl std::convert::TryFrom< &str> for AndroidNotificationNotificationPriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIORITY_UNSPECIFIED" => Ok(AndroidNotificationNotificationPriorityEnum::PRIORITYUNSPECIFIED),
           "PRIORITY_MIN" => Ok(AndroidNotificationNotificationPriorityEnum::PRIORITYMIN),
           "PRIORITY_LOW" => Ok(AndroidNotificationNotificationPriorityEnum::PRIORITYLOW),
           "PRIORITY_DEFAULT" => Ok(AndroidNotificationNotificationPriorityEnum::PRIORITYDEFAULT),
           "PRIORITY_HIGH" => Ok(AndroidNotificationNotificationPriorityEnum::PRIORITYHIGH),
           "PRIORITY_MAX" => Ok(AndroidNotificationNotificationPriorityEnum::PRIORITYMAX),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AndroidNotificationNotificationPriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AndroidNotificationProxyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Setting to control when a notification may be proxied.
pub enum AndroidNotificationProxyEnum {
    

    /// If unspecified, default to `Proxy.IF_PRIORITY_LOWERED`.
    ///
    /// "PROXY_UNSPECIFIED"
    #[serde(rename="PROXY_UNSPECIFIED")]
    PROXYUNSPECIFIED,
    

    /// Try to proxy this notification.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// Do not proxy this notification.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
    

    /// Only try to proxy this notification if its `AndroidMessagePriority` was lowered from `HIGH` to `NORMAL` on the device.
    ///
    /// "IF_PRIORITY_LOWERED"
    #[serde(rename="IF_PRIORITY_LOWERED")]
    IFPRIORITYLOWERED,
}

impl AsRef<str> for AndroidNotificationProxyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AndroidNotificationProxyEnum::PROXYUNSPECIFIED => "PROXY_UNSPECIFIED",
            AndroidNotificationProxyEnum::ALLOW => "ALLOW",
            AndroidNotificationProxyEnum::DENY => "DENY",
            AndroidNotificationProxyEnum::IFPRIORITYLOWERED => "IF_PRIORITY_LOWERED",
        }
    }
}

impl std::convert::TryFrom< &str> for AndroidNotificationProxyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROXY_UNSPECIFIED" => Ok(AndroidNotificationProxyEnum::PROXYUNSPECIFIED),
           "ALLOW" => Ok(AndroidNotificationProxyEnum::ALLOW),
           "DENY" => Ok(AndroidNotificationProxyEnum::DENY),
           "IF_PRIORITY_LOWERED" => Ok(AndroidNotificationProxyEnum::IFPRIORITYLOWERED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AndroidNotificationProxyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AndroidNotificationVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Set the [Notification.visibility](https://developer.android.com/reference/android/app/Notification.html#visibility) of the notification.
pub enum AndroidNotificationVisibilityEnum {
    

    /// If unspecified, default to `Visibility.PRIVATE`.
    ///
    /// "VISIBILITY_UNSPECIFIED"
    #[serde(rename="VISIBILITY_UNSPECIFIED")]
    VISIBILITYUNSPECIFIED,
    

    /// Show this notification on all lockscreens, but conceal sensitive or private information on secure lockscreens.
    ///
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
    

    /// Show this notification in its entirety on all lockscreens.
    ///
    /// "PUBLIC"
    #[serde(rename="PUBLIC")]
    PUBLIC,
    

    /// Do not reveal any part of this notification on a secure lockscreen.
    ///
    /// "SECRET"
    #[serde(rename="SECRET")]
    SECRET,
}

impl AsRef<str> for AndroidNotificationVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AndroidNotificationVisibilityEnum::VISIBILITYUNSPECIFIED => "VISIBILITY_UNSPECIFIED",
            AndroidNotificationVisibilityEnum::PRIVATE => "PRIVATE",
            AndroidNotificationVisibilityEnum::PUBLIC => "PUBLIC",
            AndroidNotificationVisibilityEnum::SECRET => "SECRET",
        }
    }
}

impl std::convert::TryFrom< &str> for AndroidNotificationVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VISIBILITY_UNSPECIFIED" => Ok(AndroidNotificationVisibilityEnum::VISIBILITYUNSPECIFIED),
           "PRIVATE" => Ok(AndroidNotificationVisibilityEnum::PRIVATE),
           "PUBLIC" => Ok(AndroidNotificationVisibilityEnum::PUBLIC),
           "SECRET" => Ok(AndroidNotificationVisibilityEnum::SECRET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AndroidNotificationVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


