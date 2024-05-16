use super::*;



// region JobProgressEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Job progress
pub enum JobProgressEnum {
    

    /// Completed
    ///
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// In progress
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// Not accepted
    ///
    /// "NOT_ACCEPTED"
    #[serde(rename="NOT_ACCEPTED")]
    NOTACCEPTED,
    

    /// Not started
    ///
    /// "NOT_STARTED"
    #[serde(rename="NOT_STARTED")]
    NOTSTARTED,
    

    /// Obsolete
    ///
    /// "OBSOLETE"
    #[serde(rename="OBSOLETE")]
    OBSOLETE,
}

impl AsRef<str> for JobProgressEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobProgressEnum::COMPLETED => "COMPLETED",
            JobProgressEnum::INPROGRESS => "IN_PROGRESS",
            JobProgressEnum::NOTACCEPTED => "NOT_ACCEPTED",
            JobProgressEnum::NOTSTARTED => "NOT_STARTED",
            JobProgressEnum::OBSOLETE => "OBSOLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for JobProgressEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPLETED" => Ok(JobProgressEnum::COMPLETED),
           "IN_PROGRESS" => Ok(JobProgressEnum::INPROGRESS),
           "NOT_ACCEPTED" => Ok(JobProgressEnum::NOTACCEPTED),
           "NOT_STARTED" => Ok(JobProgressEnum::NOTSTARTED),
           "OBSOLETE" => Ok(JobProgressEnum::OBSOLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobProgressEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


