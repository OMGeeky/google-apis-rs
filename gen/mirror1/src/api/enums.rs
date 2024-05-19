use super::*;



// region TimelineOrderByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the order in which timeline items are returned.
pub enum TimelineOrderByEnum {
    

    /// Results will be ordered by displayTime (default). This is the same ordering as is used in the timeline on the device.
    ///
    /// "displayTime"
    #[serde(rename="displayTime")]
    DisplayTime,
    

    /// Results will be ordered by the time at which they were last written to the data store.
    ///
    /// "writeTime"
    #[serde(rename="writeTime")]
    WriteTime,
}

impl AsRef<str> for TimelineOrderByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TimelineOrderByEnum::DisplayTime => "displayTime",
            TimelineOrderByEnum::WriteTime => "writeTime",
        }
    }
}

impl std::convert::TryFrom< &str> for TimelineOrderByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "displayTime" => Ok(TimelineOrderByEnum::DisplayTime),
           "writeTime" => Ok(TimelineOrderByEnum::WriteTime),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TimelineOrderByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


