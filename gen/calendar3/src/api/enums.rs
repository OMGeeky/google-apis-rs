use super::*;



// region CalendarListMinAccessRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The minimum access role for the user in the returned entries. Optional. The default is no restriction.
pub enum CalendarListMinAccessRoleEnum {
    

    /// The user can read free/busy information.
    ///
    /// "freeBusyReader"
    #[serde(rename="freeBusyReader")]
    FreeBusyReader,
    

    /// The user can read and modify events and access control lists.
    ///
    /// "owner"
    #[serde(rename="owner")]
    Owner,
    

    /// The user can read events that are not private.
    ///
    /// "reader"
    #[serde(rename="reader")]
    Reader,
    

    /// The user can read and modify events.
    ///
    /// "writer"
    #[serde(rename="writer")]
    Writer,
}

impl AsRef<str> for CalendarListMinAccessRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CalendarListMinAccessRoleEnum::FreeBusyReader => "freeBusyReader",
            CalendarListMinAccessRoleEnum::Owner => "owner",
            CalendarListMinAccessRoleEnum::Reader => "reader",
            CalendarListMinAccessRoleEnum::Writer => "writer",
        }
    }
}

impl std::convert::TryFrom< &str> for CalendarListMinAccessRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "freeBusyReader" => Ok(CalendarListMinAccessRoleEnum::FreeBusyReader),
           "owner" => Ok(CalendarListMinAccessRoleEnum::Owner),
           "reader" => Ok(CalendarListMinAccessRoleEnum::Reader),
           "writer" => Ok(CalendarListMinAccessRoleEnum::Writer),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CalendarListMinAccessRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventSendUpdatesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Guests who should receive notifications about the event update (for example, title changes, etc.).
pub enum EventSendUpdatesEnum {
    

    /// Notifications are sent to all guests.
    ///
    /// "all"
    #[serde(rename="all")]
    All,
    

    /// Notifications are sent to non-Google Calendar guests only.
    ///
    /// "externalOnly"
    #[serde(rename="externalOnly")]
    ExternalOnly,
    

    /// No notifications are sent. For calendar migration tasks, consider using the Events.import method instead.
    ///
    /// "none"
    #[serde(rename="none")]
    None,
}

impl AsRef<str> for EventSendUpdatesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventSendUpdatesEnum::All => "all",
            EventSendUpdatesEnum::ExternalOnly => "externalOnly",
            EventSendUpdatesEnum::None => "none",
        }
    }
}

impl std::convert::TryFrom< &str> for EventSendUpdatesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "all" => Ok(EventSendUpdatesEnum::All),
           "externalOnly" => Ok(EventSendUpdatesEnum::ExternalOnly),
           "none" => Ok(EventSendUpdatesEnum::None),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventSendUpdatesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventEventTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Event types to return. Optional. This parameter can be repeated multiple times to return events of different types. If unset, returns all event types.
pub enum EventEventTypesEnum {
    

    /// Regular events.
    ///
    /// "default"
    #[serde(rename="default")]
    Default,
    

    /// Focus time events.
    ///
    /// "focusTime"
    #[serde(rename="focusTime")]
    FocusTime,
    

    /// Out of office events.
    ///
    /// "outOfOffice"
    #[serde(rename="outOfOffice")]
    OutOfOffice,
    

    /// Working location events.
    ///
    /// "workingLocation"
    #[serde(rename="workingLocation")]
    WorkingLocation,
}

impl AsRef<str> for EventEventTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventEventTypesEnum::Default => "default",
            EventEventTypesEnum::FocusTime => "focusTime",
            EventEventTypesEnum::OutOfOffice => "outOfOffice",
            EventEventTypesEnum::WorkingLocation => "workingLocation",
        }
    }
}

impl std::convert::TryFrom< &str> for EventEventTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "default" => Ok(EventEventTypesEnum::Default),
           "focusTime" => Ok(EventEventTypesEnum::FocusTime),
           "outOfOffice" => Ok(EventEventTypesEnum::OutOfOffice),
           "workingLocation" => Ok(EventEventTypesEnum::WorkingLocation),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventEventTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventOrderByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The order of the events returned in the result. Optional. The default is an unspecified, stable order.
pub enum EventOrderByEnum {
    

    /// Order by the start date/time (ascending). This is only available when querying single events (i.e. the parameter singleEvents is True)
    ///
    /// "startTime"
    #[serde(rename="startTime")]
    StartTime,
    

    /// Order by last modification time (ascending).
    ///
    /// "updated"
    #[serde(rename="updated")]
    Updated,
}

impl AsRef<str> for EventOrderByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventOrderByEnum::StartTime => "startTime",
            EventOrderByEnum::Updated => "updated",
        }
    }
}

impl std::convert::TryFrom< &str> for EventOrderByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "startTime" => Ok(EventOrderByEnum::StartTime),
           "updated" => Ok(EventOrderByEnum::Updated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventOrderByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


