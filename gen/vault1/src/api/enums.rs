use super::*;



// region CalendarExportOptionExportFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The file format for exported text messages.
pub enum CalendarExportOptionExportFormatEnum {
    

    /// No export format specified.
    ///
    /// "EXPORT_FORMAT_UNSPECIFIED"
    #[serde(rename="EXPORT_FORMAT_UNSPECIFIED")]
    EXPORTFORMATUNSPECIFIED,
    

    /// Export as MBOX. Only available for Gmail, Groups, Hangouts and Voice.
    ///
    /// "MBOX"
    #[serde(rename="MBOX")]
    MBOX,
    

    /// Export as PST. Only available for Gmail, Groups, Hangouts, Voice and Calendar.
    ///
    /// "PST"
    #[serde(rename="PST")]
    PST,
    

    /// Export as ICS. Only available for Calendar.
    ///
    /// "ICS"
    #[serde(rename="ICS")]
    ICS,
}

impl AsRef<str> for CalendarExportOptionExportFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CalendarExportOptionExportFormatEnum::EXPORTFORMATUNSPECIFIED => "EXPORT_FORMAT_UNSPECIFIED",
            CalendarExportOptionExportFormatEnum::MBOX => "MBOX",
            CalendarExportOptionExportFormatEnum::PST => "PST",
            CalendarExportOptionExportFormatEnum::ICS => "ICS",
        }
    }
}

impl std::convert::TryFrom< &str> for CalendarExportOptionExportFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPORT_FORMAT_UNSPECIFIED" => Ok(CalendarExportOptionExportFormatEnum::EXPORTFORMATUNSPECIFIED),
           "MBOX" => Ok(CalendarExportOptionExportFormatEnum::MBOX),
           "PST" => Ok(CalendarExportOptionExportFormatEnum::PST),
           "ICS" => Ok(CalendarExportOptionExportFormatEnum::ICS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CalendarExportOptionExportFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CalendarOptionResponseStatusesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Matches only events for which the custodian gave one of these responses. If the set is empty or contains ATTENDEE_RESPONSE_UNSPECIFIED there will be no filtering on responses.
pub enum CalendarOptionResponseStatusesEnum {
    

    /// Attendee response unspecified. If this is set no filtering on responses will be done, all other attendee responses that are part of the query options are ignored.
    ///
    /// "ATTENDEE_RESPONSE_UNSPECIFIED"
    #[serde(rename="ATTENDEE_RESPONSE_UNSPECIFIED")]
    ATTENDEERESPONSEUNSPECIFIED,
    

    /// The participant has been invited but has not responded yet.
    ///
    /// "ATTENDEE_RESPONSE_NEEDS_ACTION"
    #[serde(rename="ATTENDEE_RESPONSE_NEEDS_ACTION")]
    ATTENDEERESPONSENEEDSACTION,
    

    /// The participant plans to attend.
    ///
    /// "ATTENDEE_RESPONSE_ACCEPTED"
    #[serde(rename="ATTENDEE_RESPONSE_ACCEPTED")]
    ATTENDEERESPONSEACCEPTED,
    

    /// The participant does not plan to attend.
    ///
    /// "ATTENDEE_RESPONSE_DECLINED"
    #[serde(rename="ATTENDEE_RESPONSE_DECLINED")]
    ATTENDEERESPONSEDECLINED,
    

    /// The participant expects to possibly attend.
    ///
    /// "ATTENDEE_RESPONSE_TENTATIVE"
    #[serde(rename="ATTENDEE_RESPONSE_TENTATIVE")]
    ATTENDEERESPONSETENTATIVE,
}

impl AsRef<str> for CalendarOptionResponseStatusesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CalendarOptionResponseStatusesEnum::ATTENDEERESPONSEUNSPECIFIED => "ATTENDEE_RESPONSE_UNSPECIFIED",
            CalendarOptionResponseStatusesEnum::ATTENDEERESPONSENEEDSACTION => "ATTENDEE_RESPONSE_NEEDS_ACTION",
            CalendarOptionResponseStatusesEnum::ATTENDEERESPONSEACCEPTED => "ATTENDEE_RESPONSE_ACCEPTED",
            CalendarOptionResponseStatusesEnum::ATTENDEERESPONSEDECLINED => "ATTENDEE_RESPONSE_DECLINED",
            CalendarOptionResponseStatusesEnum::ATTENDEERESPONSETENTATIVE => "ATTENDEE_RESPONSE_TENTATIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for CalendarOptionResponseStatusesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTENDEE_RESPONSE_UNSPECIFIED" => Ok(CalendarOptionResponseStatusesEnum::ATTENDEERESPONSEUNSPECIFIED),
           "ATTENDEE_RESPONSE_NEEDS_ACTION" => Ok(CalendarOptionResponseStatusesEnum::ATTENDEERESPONSENEEDSACTION),
           "ATTENDEE_RESPONSE_ACCEPTED" => Ok(CalendarOptionResponseStatusesEnum::ATTENDEERESPONSEACCEPTED),
           "ATTENDEE_RESPONSE_DECLINED" => Ok(CalendarOptionResponseStatusesEnum::ATTENDEERESPONSEDECLINED),
           "ATTENDEE_RESPONSE_TENTATIVE" => Ok(CalendarOptionResponseStatusesEnum::ATTENDEERESPONSETENTATIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CalendarOptionResponseStatusesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CountArtifactsRequestViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sets the granularity of the count results.
pub enum CountArtifactsRequestViewEnum {
    

    /// Default. Same as **TOTAL_COUNT**.
    ///
    /// "COUNT_RESULT_VIEW_UNSPECIFIED"
    #[serde(rename="COUNT_RESULT_VIEW_UNSPECIFIED")]
    COUNTRESULTVIEWUNSPECIFIED,
    

    /// Response includes counts of the total accounts, queried accounts, matching accounts, non-queryable accounts, and queried account errors.
    ///
    /// "TOTAL_COUNT"
    #[serde(rename="TOTAL_COUNT")]
    TOTALCOUNT,
    

    /// Response includes the same details as **TOTAL_COUNT**, plus additional account breakdown.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
}

impl AsRef<str> for CountArtifactsRequestViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CountArtifactsRequestViewEnum::COUNTRESULTVIEWUNSPECIFIED => "COUNT_RESULT_VIEW_UNSPECIFIED",
            CountArtifactsRequestViewEnum::TOTALCOUNT => "TOTAL_COUNT",
            CountArtifactsRequestViewEnum::ALL => "ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for CountArtifactsRequestViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COUNT_RESULT_VIEW_UNSPECIFIED" => Ok(CountArtifactsRequestViewEnum::COUNTRESULTVIEWUNSPECIFIED),
           "TOTAL_COUNT" => Ok(CountArtifactsRequestViewEnum::TOTALCOUNT),
           "ALL" => Ok(CountArtifactsRequestViewEnum::ALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CountArtifactsRequestViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DriveOptionClientSideEncryptedOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Set whether the results include only content encrypted with [Google Workspace Client-side encryption](https://support.google.com/a?p=cse_ov) content, only unencrypted content, or both. Defaults to both. Currently supported for Drive.
pub enum DriveOptionClientSideEncryptedOptionEnum {
    

    /// Encryption status unspecified. Results include both client-side encrypted and non-encrypted content.
    ///
    /// "CLIENT_SIDE_ENCRYPTED_OPTION_UNSPECIFIED"
    #[serde(rename="CLIENT_SIDE_ENCRYPTED_OPTION_UNSPECIFIED")]
    CLIENTSIDEENCRYPTEDOPTIONUNSPECIFIED,
    

    /// Include both client-side encrypted and unencrypted content in results.
    ///
    /// "CLIENT_SIDE_ENCRYPTED_OPTION_ANY"
    #[serde(rename="CLIENT_SIDE_ENCRYPTED_OPTION_ANY")]
    CLIENTSIDEENCRYPTEDOPTIONANY,
    

    /// Include client-side encrypted content only.
    ///
    /// "CLIENT_SIDE_ENCRYPTED_OPTION_ENCRYPTED"
    #[serde(rename="CLIENT_SIDE_ENCRYPTED_OPTION_ENCRYPTED")]
    CLIENTSIDEENCRYPTEDOPTIONENCRYPTED,
    

    /// Include unencrypted content only.
    ///
    /// "CLIENT_SIDE_ENCRYPTED_OPTION_UNENCRYPTED"
    #[serde(rename="CLIENT_SIDE_ENCRYPTED_OPTION_UNENCRYPTED")]
    CLIENTSIDEENCRYPTEDOPTIONUNENCRYPTED,
}

impl AsRef<str> for DriveOptionClientSideEncryptedOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DriveOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONUNSPECIFIED => "CLIENT_SIDE_ENCRYPTED_OPTION_UNSPECIFIED",
            DriveOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONANY => "CLIENT_SIDE_ENCRYPTED_OPTION_ANY",
            DriveOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONENCRYPTED => "CLIENT_SIDE_ENCRYPTED_OPTION_ENCRYPTED",
            DriveOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONUNENCRYPTED => "CLIENT_SIDE_ENCRYPTED_OPTION_UNENCRYPTED",
        }
    }
}

impl std::convert::TryFrom< &str> for DriveOptionClientSideEncryptedOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLIENT_SIDE_ENCRYPTED_OPTION_UNSPECIFIED" => Ok(DriveOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONUNSPECIFIED),
           "CLIENT_SIDE_ENCRYPTED_OPTION_ANY" => Ok(DriveOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONANY),
           "CLIENT_SIDE_ENCRYPTED_OPTION_ENCRYPTED" => Ok(DriveOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONENCRYPTED),
           "CLIENT_SIDE_ENCRYPTED_OPTION_UNENCRYPTED" => Ok(DriveOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONUNENCRYPTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DriveOptionClientSideEncryptedOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExportStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The status of the export.
pub enum ExportStatusEnum {
    

    /// The status is unspecified.
    ///
    /// "EXPORT_STATUS_UNSPECIFIED"
    #[serde(rename="EXPORT_STATUS_UNSPECIFIED")]
    EXPORTSTATUSUNSPECIFIED,
    

    /// The export completed.
    ///
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// The export failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The export is in progress.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
}

impl AsRef<str> for ExportStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExportStatusEnum::EXPORTSTATUSUNSPECIFIED => "EXPORT_STATUS_UNSPECIFIED",
            ExportStatusEnum::COMPLETED => "COMPLETED",
            ExportStatusEnum::FAILED => "FAILED",
            ExportStatusEnum::INPROGRESS => "IN_PROGRESS",
        }
    }
}

impl std::convert::TryFrom< &str> for ExportStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPORT_STATUS_UNSPECIFIED" => Ok(ExportStatusEnum::EXPORTSTATUSUNSPECIFIED),
           "COMPLETED" => Ok(ExportStatusEnum::COMPLETED),
           "FAILED" => Ok(ExportStatusEnum::FAILED),
           "IN_PROGRESS" => Ok(ExportStatusEnum::INPROGRESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExportStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExportOptionRegionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The requested data region for the export.
pub enum ExportOptionRegionEnum {
    

    /// The region is unspecified. Defaults to ANY.
    ///
    /// "EXPORT_REGION_UNSPECIFIED"
    #[serde(rename="EXPORT_REGION_UNSPECIFIED")]
    EXPORTREGIONUNSPECIFIED,
    

    /// Any region.
    ///
    /// "ANY"
    #[serde(rename="ANY")]
    ANY,
    

    /// United States region.
    ///
    /// "US"
    #[serde(rename="US")]
    US,
    

    /// Europe region.
    ///
    /// "EUROPE"
    #[serde(rename="EUROPE")]
    EUROPE,
}

impl AsRef<str> for ExportOptionRegionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExportOptionRegionEnum::EXPORTREGIONUNSPECIFIED => "EXPORT_REGION_UNSPECIFIED",
            ExportOptionRegionEnum::ANY => "ANY",
            ExportOptionRegionEnum::US => "US",
            ExportOptionRegionEnum::EUROPE => "EUROPE",
        }
    }
}

impl std::convert::TryFrom< &str> for ExportOptionRegionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPORT_REGION_UNSPECIFIED" => Ok(ExportOptionRegionEnum::EXPORTREGIONUNSPECIFIED),
           "ANY" => Ok(ExportOptionRegionEnum::ANY),
           "US" => Ok(ExportOptionRegionEnum::US),
           "EUROPE" => Ok(ExportOptionRegionEnum::EUROPE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExportOptionRegionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GroupsExportOptionExportFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The file format for exported messages.
pub enum GroupsExportOptionExportFormatEnum {
    

    /// No export format specified.
    ///
    /// "EXPORT_FORMAT_UNSPECIFIED"
    #[serde(rename="EXPORT_FORMAT_UNSPECIFIED")]
    EXPORTFORMATUNSPECIFIED,
    

    /// Export as MBOX. Only available for Gmail, Groups, Hangouts and Voice.
    ///
    /// "MBOX"
    #[serde(rename="MBOX")]
    MBOX,
    

    /// Export as PST. Only available for Gmail, Groups, Hangouts, Voice and Calendar.
    ///
    /// "PST"
    #[serde(rename="PST")]
    PST,
    

    /// Export as ICS. Only available for Calendar.
    ///
    /// "ICS"
    #[serde(rename="ICS")]
    ICS,
}

impl AsRef<str> for GroupsExportOptionExportFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GroupsExportOptionExportFormatEnum::EXPORTFORMATUNSPECIFIED => "EXPORT_FORMAT_UNSPECIFIED",
            GroupsExportOptionExportFormatEnum::MBOX => "MBOX",
            GroupsExportOptionExportFormatEnum::PST => "PST",
            GroupsExportOptionExportFormatEnum::ICS => "ICS",
        }
    }
}

impl std::convert::TryFrom< &str> for GroupsExportOptionExportFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPORT_FORMAT_UNSPECIFIED" => Ok(GroupsExportOptionExportFormatEnum::EXPORTFORMATUNSPECIFIED),
           "MBOX" => Ok(GroupsExportOptionExportFormatEnum::MBOX),
           "PST" => Ok(GroupsExportOptionExportFormatEnum::PST),
           "ICS" => Ok(GroupsExportOptionExportFormatEnum::ICS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GroupsExportOptionExportFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HangoutsChatExportOptionExportFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The file format for exported messages.
pub enum HangoutsChatExportOptionExportFormatEnum {
    

    /// No export format specified.
    ///
    /// "EXPORT_FORMAT_UNSPECIFIED"
    #[serde(rename="EXPORT_FORMAT_UNSPECIFIED")]
    EXPORTFORMATUNSPECIFIED,
    

    /// Export as MBOX. Only available for Gmail, Groups, Hangouts and Voice.
    ///
    /// "MBOX"
    #[serde(rename="MBOX")]
    MBOX,
    

    /// Export as PST. Only available for Gmail, Groups, Hangouts, Voice and Calendar.
    ///
    /// "PST"
    #[serde(rename="PST")]
    PST,
    

    /// Export as ICS. Only available for Calendar.
    ///
    /// "ICS"
    #[serde(rename="ICS")]
    ICS,
}

impl AsRef<str> for HangoutsChatExportOptionExportFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HangoutsChatExportOptionExportFormatEnum::EXPORTFORMATUNSPECIFIED => "EXPORT_FORMAT_UNSPECIFIED",
            HangoutsChatExportOptionExportFormatEnum::MBOX => "MBOX",
            HangoutsChatExportOptionExportFormatEnum::PST => "PST",
            HangoutsChatExportOptionExportFormatEnum::ICS => "ICS",
        }
    }
}

impl std::convert::TryFrom< &str> for HangoutsChatExportOptionExportFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPORT_FORMAT_UNSPECIFIED" => Ok(HangoutsChatExportOptionExportFormatEnum::EXPORTFORMATUNSPECIFIED),
           "MBOX" => Ok(HangoutsChatExportOptionExportFormatEnum::MBOX),
           "PST" => Ok(HangoutsChatExportOptionExportFormatEnum::PST),
           "ICS" => Ok(HangoutsChatExportOptionExportFormatEnum::ICS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HangoutsChatExportOptionExportFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HeldVoiceQueryCoveredDataEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of data types covered by the hold. Should be non-empty. Order does not matter and duplicates are ignored.
pub enum HeldVoiceQueryCoveredDataEnum {
    

    /// Covered data unspecified.
    ///
    /// "COVERED_DATA_UNSPECIFIED"
    #[serde(rename="COVERED_DATA_UNSPECIFIED")]
    COVEREDDATAUNSPECIFIED,
    

    /// Voice text messages.
    ///
    /// "TEXT_MESSAGES"
    #[serde(rename="TEXT_MESSAGES")]
    TEXTMESSAGES,
    

    /// Voicemails and their transcripts.
    ///
    /// "VOICEMAILS"
    #[serde(rename="VOICEMAILS")]
    VOICEMAILS,
    

    /// Call logs.
    ///
    /// "CALL_LOGS"
    #[serde(rename="CALL_LOGS")]
    CALLLOGS,
}

impl AsRef<str> for HeldVoiceQueryCoveredDataEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HeldVoiceQueryCoveredDataEnum::COVEREDDATAUNSPECIFIED => "COVERED_DATA_UNSPECIFIED",
            HeldVoiceQueryCoveredDataEnum::TEXTMESSAGES => "TEXT_MESSAGES",
            HeldVoiceQueryCoveredDataEnum::VOICEMAILS => "VOICEMAILS",
            HeldVoiceQueryCoveredDataEnum::CALLLOGS => "CALL_LOGS",
        }
    }
}

impl std::convert::TryFrom< &str> for HeldVoiceQueryCoveredDataEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COVERED_DATA_UNSPECIFIED" => Ok(HeldVoiceQueryCoveredDataEnum::COVEREDDATAUNSPECIFIED),
           "TEXT_MESSAGES" => Ok(HeldVoiceQueryCoveredDataEnum::TEXTMESSAGES),
           "VOICEMAILS" => Ok(HeldVoiceQueryCoveredDataEnum::VOICEMAILS),
           "CALL_LOGS" => Ok(HeldVoiceQueryCoveredDataEnum::CALLLOGS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HeldVoiceQueryCoveredDataEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HoldCorpusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The service to be searched.
pub enum HoldCorpusEnum {
    

    /// No service specified.
    ///
    /// "CORPUS_TYPE_UNSPECIFIED"
    #[serde(rename="CORPUS_TYPE_UNSPECIFIED")]
    CORPUSTYPEUNSPECIFIED,
    

    /// Drive, including Meet and Sites.
    ///
    /// "DRIVE"
    #[serde(rename="DRIVE")]
    DRIVE,
    

    /// For search, Gmail and classic Hangouts. For holds, Gmail only.
    ///
    /// "MAIL"
    #[serde(rename="MAIL")]
    MAIL,
    

    /// Groups.
    ///
    /// "GROUPS"
    #[serde(rename="GROUPS")]
    GROUPS,
    

    /// For export, Google Chat only. For holds, Google Chat and classic Hangouts.
    ///
    /// "HANGOUTS_CHAT"
    #[serde(rename="HANGOUTS_CHAT")]
    HANGOUTSCHAT,
    

    /// Google Voice.
    ///
    /// "VOICE"
    #[serde(rename="VOICE")]
    VOICE,
    

    /// Calendar.
    ///
    /// "CALENDAR"
    #[serde(rename="CALENDAR")]
    CALENDAR,
}

impl AsRef<str> for HoldCorpusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HoldCorpusEnum::CORPUSTYPEUNSPECIFIED => "CORPUS_TYPE_UNSPECIFIED",
            HoldCorpusEnum::DRIVE => "DRIVE",
            HoldCorpusEnum::MAIL => "MAIL",
            HoldCorpusEnum::GROUPS => "GROUPS",
            HoldCorpusEnum::HANGOUTSCHAT => "HANGOUTS_CHAT",
            HoldCorpusEnum::VOICE => "VOICE",
            HoldCorpusEnum::CALENDAR => "CALENDAR",
        }
    }
}

impl std::convert::TryFrom< &str> for HoldCorpusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CORPUS_TYPE_UNSPECIFIED" => Ok(HoldCorpusEnum::CORPUSTYPEUNSPECIFIED),
           "DRIVE" => Ok(HoldCorpusEnum::DRIVE),
           "MAIL" => Ok(HoldCorpusEnum::MAIL),
           "GROUPS" => Ok(HoldCorpusEnum::GROUPS),
           "HANGOUTS_CHAT" => Ok(HoldCorpusEnum::HANGOUTSCHAT),
           "VOICE" => Ok(HoldCorpusEnum::VOICE),
           "CALENDAR" => Ok(HoldCorpusEnum::CALENDAR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HoldCorpusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MailExportOptionExportFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The file format for exported messages.
pub enum MailExportOptionExportFormatEnum {
    

    /// No export format specified.
    ///
    /// "EXPORT_FORMAT_UNSPECIFIED"
    #[serde(rename="EXPORT_FORMAT_UNSPECIFIED")]
    EXPORTFORMATUNSPECIFIED,
    

    /// Export as MBOX. Only available for Gmail, Groups, Hangouts and Voice.
    ///
    /// "MBOX"
    #[serde(rename="MBOX")]
    MBOX,
    

    /// Export as PST. Only available for Gmail, Groups, Hangouts, Voice and Calendar.
    ///
    /// "PST"
    #[serde(rename="PST")]
    PST,
    

    /// Export as ICS. Only available for Calendar.
    ///
    /// "ICS"
    #[serde(rename="ICS")]
    ICS,
}

impl AsRef<str> for MailExportOptionExportFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MailExportOptionExportFormatEnum::EXPORTFORMATUNSPECIFIED => "EXPORT_FORMAT_UNSPECIFIED",
            MailExportOptionExportFormatEnum::MBOX => "MBOX",
            MailExportOptionExportFormatEnum::PST => "PST",
            MailExportOptionExportFormatEnum::ICS => "ICS",
        }
    }
}

impl std::convert::TryFrom< &str> for MailExportOptionExportFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPORT_FORMAT_UNSPECIFIED" => Ok(MailExportOptionExportFormatEnum::EXPORTFORMATUNSPECIFIED),
           "MBOX" => Ok(MailExportOptionExportFormatEnum::MBOX),
           "PST" => Ok(MailExportOptionExportFormatEnum::PST),
           "ICS" => Ok(MailExportOptionExportFormatEnum::ICS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MailExportOptionExportFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MailOptionClientSideEncryptedOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether the results should include encrypted content, unencrypted content, or both. Defaults to including both.
pub enum MailOptionClientSideEncryptedOptionEnum {
    

    /// Encryption status unspecified. Results include both client-side encrypted and non-encrypted content.
    ///
    /// "CLIENT_SIDE_ENCRYPTED_OPTION_UNSPECIFIED"
    #[serde(rename="CLIENT_SIDE_ENCRYPTED_OPTION_UNSPECIFIED")]
    CLIENTSIDEENCRYPTEDOPTIONUNSPECIFIED,
    

    /// Include both client-side encrypted and unencrypted content in results.
    ///
    /// "CLIENT_SIDE_ENCRYPTED_OPTION_ANY"
    #[serde(rename="CLIENT_SIDE_ENCRYPTED_OPTION_ANY")]
    CLIENTSIDEENCRYPTEDOPTIONANY,
    

    /// Include client-side encrypted content only.
    ///
    /// "CLIENT_SIDE_ENCRYPTED_OPTION_ENCRYPTED"
    #[serde(rename="CLIENT_SIDE_ENCRYPTED_OPTION_ENCRYPTED")]
    CLIENTSIDEENCRYPTEDOPTIONENCRYPTED,
    

    /// Include unencrypted content only.
    ///
    /// "CLIENT_SIDE_ENCRYPTED_OPTION_UNENCRYPTED"
    #[serde(rename="CLIENT_SIDE_ENCRYPTED_OPTION_UNENCRYPTED")]
    CLIENTSIDEENCRYPTEDOPTIONUNENCRYPTED,
}

impl AsRef<str> for MailOptionClientSideEncryptedOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MailOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONUNSPECIFIED => "CLIENT_SIDE_ENCRYPTED_OPTION_UNSPECIFIED",
            MailOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONANY => "CLIENT_SIDE_ENCRYPTED_OPTION_ANY",
            MailOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONENCRYPTED => "CLIENT_SIDE_ENCRYPTED_OPTION_ENCRYPTED",
            MailOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONUNENCRYPTED => "CLIENT_SIDE_ENCRYPTED_OPTION_UNENCRYPTED",
        }
    }
}

impl std::convert::TryFrom< &str> for MailOptionClientSideEncryptedOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLIENT_SIDE_ENCRYPTED_OPTION_UNSPECIFIED" => Ok(MailOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONUNSPECIFIED),
           "CLIENT_SIDE_ENCRYPTED_OPTION_ANY" => Ok(MailOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONANY),
           "CLIENT_SIDE_ENCRYPTED_OPTION_ENCRYPTED" => Ok(MailOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONENCRYPTED),
           "CLIENT_SIDE_ENCRYPTED_OPTION_UNENCRYPTED" => Ok(MailOptionClientSideEncryptedOptionEnum::CLIENTSIDEENCRYPTEDOPTIONUNENCRYPTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MailOptionClientSideEncryptedOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MatterStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the matter.
pub enum MatterStateEnum {
    

    /// The matter has no specified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The matter is open.
    ///
    /// "OPEN"
    #[serde(rename="OPEN")]
    OPEN,
    

    /// The matter is closed.
    ///
    /// "CLOSED"
    #[serde(rename="CLOSED")]
    CLOSED,
    

    /// The matter is deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for MatterStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MatterStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            MatterStateEnum::OPEN => "OPEN",
            MatterStateEnum::CLOSED => "CLOSED",
            MatterStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for MatterStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(MatterStateEnum::STATEUNSPECIFIED),
           "OPEN" => Ok(MatterStateEnum::OPEN),
           "CLOSED" => Ok(MatterStateEnum::CLOSED),
           "DELETED" => Ok(MatterStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MatterStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MatterPermissionRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The user's role for the matter.
pub enum MatterPermissionRoleEnum {
    

    /// No role assigned.
    ///
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    

    /// A collaborator on the matter.
    ///
    /// "COLLABORATOR"
    #[serde(rename="COLLABORATOR")]
    COLLABORATOR,
    

    /// The owner of the matter.
    ///
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
}

impl AsRef<str> for MatterPermissionRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MatterPermissionRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            MatterPermissionRoleEnum::COLLABORATOR => "COLLABORATOR",
            MatterPermissionRoleEnum::OWNER => "OWNER",
        }
    }
}

impl std::convert::TryFrom< &str> for MatterPermissionRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(MatterPermissionRoleEnum::ROLEUNSPECIFIED),
           "COLLABORATOR" => Ok(MatterPermissionRoleEnum::COLLABORATOR),
           "OWNER" => Ok(MatterPermissionRoleEnum::OWNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MatterPermissionRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueryCorpusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Google Workspace service to search.
pub enum QueryCorpusEnum {
    

    /// No service specified.
    ///
    /// "CORPUS_TYPE_UNSPECIFIED"
    #[serde(rename="CORPUS_TYPE_UNSPECIFIED")]
    CORPUSTYPEUNSPECIFIED,
    

    /// Drive, including Meet and Sites.
    ///
    /// "DRIVE"
    #[serde(rename="DRIVE")]
    DRIVE,
    

    /// For search, Gmail and classic Hangouts. For holds, Gmail only.
    ///
    /// "MAIL"
    #[serde(rename="MAIL")]
    MAIL,
    

    /// Groups.
    ///
    /// "GROUPS"
    #[serde(rename="GROUPS")]
    GROUPS,
    

    /// For export, Google Chat only. For holds, Google Chat and classic Hangouts.
    ///
    /// "HANGOUTS_CHAT"
    #[serde(rename="HANGOUTS_CHAT")]
    HANGOUTSCHAT,
    

    /// Google Voice.
    ///
    /// "VOICE"
    #[serde(rename="VOICE")]
    VOICE,
    

    /// Calendar.
    ///
    /// "CALENDAR"
    #[serde(rename="CALENDAR")]
    CALENDAR,
}

impl AsRef<str> for QueryCorpusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueryCorpusEnum::CORPUSTYPEUNSPECIFIED => "CORPUS_TYPE_UNSPECIFIED",
            QueryCorpusEnum::DRIVE => "DRIVE",
            QueryCorpusEnum::MAIL => "MAIL",
            QueryCorpusEnum::GROUPS => "GROUPS",
            QueryCorpusEnum::HANGOUTSCHAT => "HANGOUTS_CHAT",
            QueryCorpusEnum::VOICE => "VOICE",
            QueryCorpusEnum::CALENDAR => "CALENDAR",
        }
    }
}

impl std::convert::TryFrom< &str> for QueryCorpusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CORPUS_TYPE_UNSPECIFIED" => Ok(QueryCorpusEnum::CORPUSTYPEUNSPECIFIED),
           "DRIVE" => Ok(QueryCorpusEnum::DRIVE),
           "MAIL" => Ok(QueryCorpusEnum::MAIL),
           "GROUPS" => Ok(QueryCorpusEnum::GROUPS),
           "HANGOUTS_CHAT" => Ok(QueryCorpusEnum::HANGOUTSCHAT),
           "VOICE" => Ok(QueryCorpusEnum::VOICE),
           "CALENDAR" => Ok(QueryCorpusEnum::CALENDAR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueryCorpusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueryDataScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The data source to search.
pub enum QueryDataScopeEnum {
    

    /// No data source specified.
    ///
    /// "DATA_SCOPE_UNSPECIFIED"
    #[serde(rename="DATA_SCOPE_UNSPECIFIED")]
    DATASCOPEUNSPECIFIED,
    

    /// All available data.
    ///
    /// "ALL_DATA"
    #[serde(rename="ALL_DATA")]
    ALLDATA,
    

    /// Only data on hold.
    ///
    /// "HELD_DATA"
    #[serde(rename="HELD_DATA")]
    HELDDATA,
    

    /// Only data not yet processed by Vault. (Gmail and Groups only)
    ///
    /// "UNPROCESSED_DATA"
    #[serde(rename="UNPROCESSED_DATA")]
    UNPROCESSEDDATA,
}

impl AsRef<str> for QueryDataScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueryDataScopeEnum::DATASCOPEUNSPECIFIED => "DATA_SCOPE_UNSPECIFIED",
            QueryDataScopeEnum::ALLDATA => "ALL_DATA",
            QueryDataScopeEnum::HELDDATA => "HELD_DATA",
            QueryDataScopeEnum::UNPROCESSEDDATA => "UNPROCESSED_DATA",
        }
    }
}

impl std::convert::TryFrom< &str> for QueryDataScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_SCOPE_UNSPECIFIED" => Ok(QueryDataScopeEnum::DATASCOPEUNSPECIFIED),
           "ALL_DATA" => Ok(QueryDataScopeEnum::ALLDATA),
           "HELD_DATA" => Ok(QueryDataScopeEnum::HELDDATA),
           "UNPROCESSED_DATA" => Ok(QueryDataScopeEnum::UNPROCESSEDDATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueryDataScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueryMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The entity to search. This field replaces **searchMethod** to support shared drives. When **searchMethod** is **TEAM_DRIVE**, the response of this field is **SHARED_DRIVE**.
pub enum QueryMethodEnum {
    

    /// A search method must be specified or else it is rejected.
    ///
    /// "SEARCH_METHOD_UNSPECIFIED"
    #[serde(rename="SEARCH_METHOD_UNSPECIFIED")]
    SEARCHMETHODUNSPECIFIED,
    

    /// Search the data of the accounts specified in [AccountInfo](https://developers.google.com/vault/reference/rest/v1/Query#accountinfo).
    ///
    /// "ACCOUNT"
    #[serde(rename="ACCOUNT")]
    ACCOUNT,
    

    /// Search the data of all accounts in the organizational unit specified in [OrgUnitInfo](https://developers.google.com/vault/reference/rest/v1/Query#orgunitinfo).
    ///
    /// "ORG_UNIT"
    #[serde(rename="ORG_UNIT")]
    ORGUNIT,
    

    /// Search the data in the Team Drive specified in **team_drive_info**.
    ///
    /// "TEAM_DRIVE"
    #[serde(rename="TEAM_DRIVE")]
    TEAMDRIVE,
    

    /// Search the data of all accounts in the organization. Supported only for Gmail. When specified, you don't need to specify **AccountInfo** or **OrgUnitInfo**.
    ///
    /// "ENTIRE_ORG"
    #[serde(rename="ENTIRE_ORG")]
    ENTIREORG,
    

    /// Search messages in the Chat spaces specified in [HangoutsChatInfo](https://developers.google.com/vault/reference/rest/v1/Query#hangoutschatinfo).
    ///
    /// "ROOM"
    #[serde(rename="ROOM")]
    ROOM,
    

    /// Search for sites by the published site URLs specified in [SitesUrlInfo](https://developers.google.com/vault/reference/rest/v1/Query#sitesurlinfo).
    ///
    /// "SITES_URL"
    #[serde(rename="SITES_URL")]
    SITESURL,
    

    /// Search the files in the shared drives specified in [SharedDriveInfo](https://developers.google.com/vault/reference/rest/v1/Query#shareddriveinfo).
    ///
    /// "SHARED_DRIVE"
    #[serde(rename="SHARED_DRIVE")]
    SHAREDDRIVE,
}

impl AsRef<str> for QueryMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueryMethodEnum::SEARCHMETHODUNSPECIFIED => "SEARCH_METHOD_UNSPECIFIED",
            QueryMethodEnum::ACCOUNT => "ACCOUNT",
            QueryMethodEnum::ORGUNIT => "ORG_UNIT",
            QueryMethodEnum::TEAMDRIVE => "TEAM_DRIVE",
            QueryMethodEnum::ENTIREORG => "ENTIRE_ORG",
            QueryMethodEnum::ROOM => "ROOM",
            QueryMethodEnum::SITESURL => "SITES_URL",
            QueryMethodEnum::SHAREDDRIVE => "SHARED_DRIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for QueryMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEARCH_METHOD_UNSPECIFIED" => Ok(QueryMethodEnum::SEARCHMETHODUNSPECIFIED),
           "ACCOUNT" => Ok(QueryMethodEnum::ACCOUNT),
           "ORG_UNIT" => Ok(QueryMethodEnum::ORGUNIT),
           "TEAM_DRIVE" => Ok(QueryMethodEnum::TEAMDRIVE),
           "ENTIRE_ORG" => Ok(QueryMethodEnum::ENTIREORG),
           "ROOM" => Ok(QueryMethodEnum::ROOM),
           "SITES_URL" => Ok(QueryMethodEnum::SITESURL),
           "SHARED_DRIVE" => Ok(QueryMethodEnum::SHAREDDRIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueryMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QuerySearchMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The search method to use.
pub enum QuerySearchMethodEnum {
    

    /// A search method must be specified or else it is rejected.
    ///
    /// "SEARCH_METHOD_UNSPECIFIED"
    #[serde(rename="SEARCH_METHOD_UNSPECIFIED")]
    SEARCHMETHODUNSPECIFIED,
    

    /// Search the data of the accounts specified in [AccountInfo](https://developers.google.com/vault/reference/rest/v1/Query#accountinfo).
    ///
    /// "ACCOUNT"
    #[serde(rename="ACCOUNT")]
    ACCOUNT,
    

    /// Search the data of all accounts in the organizational unit specified in [OrgUnitInfo](https://developers.google.com/vault/reference/rest/v1/Query#orgunitinfo).
    ///
    /// "ORG_UNIT"
    #[serde(rename="ORG_UNIT")]
    ORGUNIT,
    

    /// Search the data in the Team Drive specified in **team_drive_info**.
    ///
    /// "TEAM_DRIVE"
    #[serde(rename="TEAM_DRIVE")]
    TEAMDRIVE,
    

    /// Search the data of all accounts in the organization. Supported only for Gmail. When specified, you don't need to specify **AccountInfo** or **OrgUnitInfo**.
    ///
    /// "ENTIRE_ORG"
    #[serde(rename="ENTIRE_ORG")]
    ENTIREORG,
    

    /// Search messages in the Chat spaces specified in [HangoutsChatInfo](https://developers.google.com/vault/reference/rest/v1/Query#hangoutschatinfo).
    ///
    /// "ROOM"
    #[serde(rename="ROOM")]
    ROOM,
    

    /// Search for sites by the published site URLs specified in [SitesUrlInfo](https://developers.google.com/vault/reference/rest/v1/Query#sitesurlinfo).
    ///
    /// "SITES_URL"
    #[serde(rename="SITES_URL")]
    SITESURL,
    

    /// Search the files in the shared drives specified in [SharedDriveInfo](https://developers.google.com/vault/reference/rest/v1/Query#shareddriveinfo).
    ///
    /// "SHARED_DRIVE"
    #[serde(rename="SHARED_DRIVE")]
    SHAREDDRIVE,
}

impl AsRef<str> for QuerySearchMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QuerySearchMethodEnum::SEARCHMETHODUNSPECIFIED => "SEARCH_METHOD_UNSPECIFIED",
            QuerySearchMethodEnum::ACCOUNT => "ACCOUNT",
            QuerySearchMethodEnum::ORGUNIT => "ORG_UNIT",
            QuerySearchMethodEnum::TEAMDRIVE => "TEAM_DRIVE",
            QuerySearchMethodEnum::ENTIREORG => "ENTIRE_ORG",
            QuerySearchMethodEnum::ROOM => "ROOM",
            QuerySearchMethodEnum::SITESURL => "SITES_URL",
            QuerySearchMethodEnum::SHAREDDRIVE => "SHARED_DRIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for QuerySearchMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEARCH_METHOD_UNSPECIFIED" => Ok(QuerySearchMethodEnum::SEARCHMETHODUNSPECIFIED),
           "ACCOUNT" => Ok(QuerySearchMethodEnum::ACCOUNT),
           "ORG_UNIT" => Ok(QuerySearchMethodEnum::ORGUNIT),
           "TEAM_DRIVE" => Ok(QuerySearchMethodEnum::TEAMDRIVE),
           "ENTIRE_ORG" => Ok(QuerySearchMethodEnum::ENTIREORG),
           "ROOM" => Ok(QuerySearchMethodEnum::ROOM),
           "SITES_URL" => Ok(QuerySearchMethodEnum::SITESURL),
           "SHARED_DRIVE" => Ok(QuerySearchMethodEnum::SHAREDDRIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QuerySearchMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VoiceExportOptionExportFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The file format for exported text messages.
pub enum VoiceExportOptionExportFormatEnum {
    

    /// No export format specified.
    ///
    /// "EXPORT_FORMAT_UNSPECIFIED"
    #[serde(rename="EXPORT_FORMAT_UNSPECIFIED")]
    EXPORTFORMATUNSPECIFIED,
    

    /// Export as MBOX. Only available for Gmail, Groups, Hangouts and Voice.
    ///
    /// "MBOX"
    #[serde(rename="MBOX")]
    MBOX,
    

    /// Export as PST. Only available for Gmail, Groups, Hangouts, Voice and Calendar.
    ///
    /// "PST"
    #[serde(rename="PST")]
    PST,
    

    /// Export as ICS. Only available for Calendar.
    ///
    /// "ICS"
    #[serde(rename="ICS")]
    ICS,
}

impl AsRef<str> for VoiceExportOptionExportFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VoiceExportOptionExportFormatEnum::EXPORTFORMATUNSPECIFIED => "EXPORT_FORMAT_UNSPECIFIED",
            VoiceExportOptionExportFormatEnum::MBOX => "MBOX",
            VoiceExportOptionExportFormatEnum::PST => "PST",
            VoiceExportOptionExportFormatEnum::ICS => "ICS",
        }
    }
}

impl std::convert::TryFrom< &str> for VoiceExportOptionExportFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPORT_FORMAT_UNSPECIFIED" => Ok(VoiceExportOptionExportFormatEnum::EXPORTFORMATUNSPECIFIED),
           "MBOX" => Ok(VoiceExportOptionExportFormatEnum::MBOX),
           "PST" => Ok(VoiceExportOptionExportFormatEnum::PST),
           "ICS" => Ok(VoiceExportOptionExportFormatEnum::ICS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VoiceExportOptionExportFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VoiceOptionCoveredDataEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Datatypes to search
pub enum VoiceOptionCoveredDataEnum {
    

    /// Covered data unspecified.
    ///
    /// "COVERED_DATA_UNSPECIFIED"
    #[serde(rename="COVERED_DATA_UNSPECIFIED")]
    COVEREDDATAUNSPECIFIED,
    

    /// Voice text messages.
    ///
    /// "TEXT_MESSAGES"
    #[serde(rename="TEXT_MESSAGES")]
    TEXTMESSAGES,
    

    /// Voicemails and their transcripts.
    ///
    /// "VOICEMAILS"
    #[serde(rename="VOICEMAILS")]
    VOICEMAILS,
    

    /// Call logs.
    ///
    /// "CALL_LOGS"
    #[serde(rename="CALL_LOGS")]
    CALLLOGS,
}

impl AsRef<str> for VoiceOptionCoveredDataEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VoiceOptionCoveredDataEnum::COVEREDDATAUNSPECIFIED => "COVERED_DATA_UNSPECIFIED",
            VoiceOptionCoveredDataEnum::TEXTMESSAGES => "TEXT_MESSAGES",
            VoiceOptionCoveredDataEnum::VOICEMAILS => "VOICEMAILS",
            VoiceOptionCoveredDataEnum::CALLLOGS => "CALL_LOGS",
        }
    }
}

impl std::convert::TryFrom< &str> for VoiceOptionCoveredDataEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COVERED_DATA_UNSPECIFIED" => Ok(VoiceOptionCoveredDataEnum::COVEREDDATAUNSPECIFIED),
           "TEXT_MESSAGES" => Ok(VoiceOptionCoveredDataEnum::TEXTMESSAGES),
           "VOICEMAILS" => Ok(VoiceOptionCoveredDataEnum::VOICEMAILS),
           "CALL_LOGS" => Ok(VoiceOptionCoveredDataEnum::CALLLOGS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VoiceOptionCoveredDataEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MatterViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how much information about the matter to return in response.
pub enum MatterViewEnum {
    

    /// The amount of detail is unspecified. Same as **BASIC**.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// Returns the matter ID, name, description, and state. Default choice.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Returns the basic details and a list of matter owners and collaborators (see [MatterPermissions](https://developers.google.com/vault/reference/rest/v1/matters#matterpermission)).
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for MatterViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MatterViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            MatterViewEnum::BASIC => "BASIC",
            MatterViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for MatterViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(MatterViewEnum::VIEWUNSPECIFIED),
           "BASIC" => Ok(MatterViewEnum::BASIC),
           "FULL" => Ok(MatterViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MatterViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MatterStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If set, lists only matters with the specified state. The default lists matters of all states.
pub enum MatterStateEnum {
    

    /// The matter has no specified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The matter is open.
    ///
    /// "OPEN"
    #[serde(rename="OPEN")]
    OPEN,
    

    /// The matter is closed.
    ///
    /// "CLOSED"
    #[serde(rename="CLOSED")]
    CLOSED,
    

    /// The matter is deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for MatterStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MatterStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            MatterStateEnum::OPEN => "OPEN",
            MatterStateEnum::CLOSED => "CLOSED",
            MatterStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for MatterStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(MatterStateEnum::STATEUNSPECIFIED),
           "OPEN" => Ok(MatterStateEnum::OPEN),
           "CLOSED" => Ok(MatterStateEnum::CLOSED),
           "DELETED" => Ok(MatterStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MatterStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


