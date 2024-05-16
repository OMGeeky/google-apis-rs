use super::*;



// region AutoForwardingDispositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state that a message should be left in after it has been forwarded.
pub enum AutoForwardingDispositionEnum {
    

    /// Unspecified disposition.
    ///
    /// "dispositionUnspecified"
    #[serde(rename="dispositionUnspecified")]
    DispositionUnspecified,
    

    /// Leave the message in the `INBOX`.
    ///
    /// "leaveInInbox"
    #[serde(rename="leaveInInbox")]
    LeaveInInbox,
    

    /// Archive the message.
    ///
    /// "archive"
    #[serde(rename="archive")]
    Archive,
    

    /// Move the message to the `TRASH`.
    ///
    /// "trash"
    #[serde(rename="trash")]
    Trash,
    

    /// Leave the message in the `INBOX` and mark it as read.
    ///
    /// "markRead"
    #[serde(rename="markRead")]
    MarkRead,
}

impl AsRef<str> for AutoForwardingDispositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AutoForwardingDispositionEnum::DispositionUnspecified => "dispositionUnspecified",
            AutoForwardingDispositionEnum::LeaveInInbox => "leaveInInbox",
            AutoForwardingDispositionEnum::Archive => "archive",
            AutoForwardingDispositionEnum::Trash => "trash",
            AutoForwardingDispositionEnum::MarkRead => "markRead",
        }
    }
}

impl std::convert::TryFrom< &str> for AutoForwardingDispositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "dispositionUnspecified" => Ok(AutoForwardingDispositionEnum::DispositionUnspecified),
           "leaveInInbox" => Ok(AutoForwardingDispositionEnum::LeaveInInbox),
           "archive" => Ok(AutoForwardingDispositionEnum::Archive),
           "trash" => Ok(AutoForwardingDispositionEnum::Trash),
           "markRead" => Ok(AutoForwardingDispositionEnum::MarkRead),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AutoForwardingDispositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CseKeyPairEnablementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the key pair.
pub enum CseKeyPairEnablementStateEnum {
    

    /// The current state of the key pair is not set. The key pair is neither turned on nor turned off.
    ///
    /// "stateUnspecified"
    #[serde(rename="stateUnspecified")]
    StateUnspecified,
    

    /// The key pair is turned on. For any email messages that this key pair encrypts, Gmail decrypts the messages and signs any outgoing mail with the private key. To turn on a key pair, use the EnableCseKeyPair method.
    ///
    /// "enabled"
    #[serde(rename="enabled")]
    Enabled,
    

    /// The key pair is turned off. Authenticated users cannot decrypt email messages nor sign outgoing messages. If a key pair is turned off for more than 30 days, you can permanently delete it. To turn off a key pair, use the DisableCseKeyPair method.
    ///
    /// "disabled"
    #[serde(rename="disabled")]
    Disabled,
}

impl AsRef<str> for CseKeyPairEnablementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CseKeyPairEnablementStateEnum::StateUnspecified => "stateUnspecified",
            CseKeyPairEnablementStateEnum::Enabled => "enabled",
            CseKeyPairEnablementStateEnum::Disabled => "disabled",
        }
    }
}

impl std::convert::TryFrom< &str> for CseKeyPairEnablementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "stateUnspecified" => Ok(CseKeyPairEnablementStateEnum::StateUnspecified),
           "enabled" => Ok(CseKeyPairEnablementStateEnum::Enabled),
           "disabled" => Ok(CseKeyPairEnablementStateEnum::Disabled),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CseKeyPairEnablementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DelegateVerificationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether this address has been verified and can act as a delegate for the account. Read-only.
pub enum DelegateVerificationStatusEnum {
    

    /// Unspecified verification status.
    ///
    /// "verificationStatusUnspecified"
    #[serde(rename="verificationStatusUnspecified")]
    VerificationStatusUnspecified,
    

    /// The address can act a delegate for the account.
    ///
    /// "accepted"
    #[serde(rename="accepted")]
    Accepted,
    

    /// A verification request was mailed to the address, and the owner has not yet accepted it.
    ///
    /// "pending"
    #[serde(rename="pending")]
    Pending,
    

    /// A verification request was mailed to the address, and the owner rejected it.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
    

    /// A verification request was mailed to the address, and it expired without verification.
    ///
    /// "expired"
    #[serde(rename="expired")]
    Expired,
}

impl AsRef<str> for DelegateVerificationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DelegateVerificationStatusEnum::VerificationStatusUnspecified => "verificationStatusUnspecified",
            DelegateVerificationStatusEnum::Accepted => "accepted",
            DelegateVerificationStatusEnum::Pending => "pending",
            DelegateVerificationStatusEnum::Rejected => "rejected",
            DelegateVerificationStatusEnum::Expired => "expired",
        }
    }
}

impl std::convert::TryFrom< &str> for DelegateVerificationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "verificationStatusUnspecified" => Ok(DelegateVerificationStatusEnum::VerificationStatusUnspecified),
           "accepted" => Ok(DelegateVerificationStatusEnum::Accepted),
           "pending" => Ok(DelegateVerificationStatusEnum::Pending),
           "rejected" => Ok(DelegateVerificationStatusEnum::Rejected),
           "expired" => Ok(DelegateVerificationStatusEnum::Expired),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DelegateVerificationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FilterCriterionSizeComparisonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the message size in bytes should be in relation to the size field.
pub enum FilterCriterionSizeComparisonEnum {
    
    /// "unspecified"
    #[serde(rename="unspecified")]
    Unspecified,
    

    /// Find messages smaller than the given size.
    ///
    /// "smaller"
    #[serde(rename="smaller")]
    Smaller,
    

    /// Find messages larger than the given size.
    ///
    /// "larger"
    #[serde(rename="larger")]
    Larger,
}

impl AsRef<str> for FilterCriterionSizeComparisonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FilterCriterionSizeComparisonEnum::Unspecified => "unspecified",
            FilterCriterionSizeComparisonEnum::Smaller => "smaller",
            FilterCriterionSizeComparisonEnum::Larger => "larger",
        }
    }
}

impl std::convert::TryFrom< &str> for FilterCriterionSizeComparisonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unspecified" => Ok(FilterCriterionSizeComparisonEnum::Unspecified),
           "smaller" => Ok(FilterCriterionSizeComparisonEnum::Smaller),
           "larger" => Ok(FilterCriterionSizeComparisonEnum::Larger),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FilterCriterionSizeComparisonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ForwardingAddresVerificationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether this address has been verified and is usable for forwarding. Read-only.
pub enum ForwardingAddresVerificationStatusEnum {
    

    /// Unspecified verification status.
    ///
    /// "verificationStatusUnspecified"
    #[serde(rename="verificationStatusUnspecified")]
    VerificationStatusUnspecified,
    

    /// The address is ready to use for forwarding.
    ///
    /// "accepted"
    #[serde(rename="accepted")]
    Accepted,
    

    /// The address is awaiting verification by the owner.
    ///
    /// "pending"
    #[serde(rename="pending")]
    Pending,
}

impl AsRef<str> for ForwardingAddresVerificationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ForwardingAddresVerificationStatusEnum::VerificationStatusUnspecified => "verificationStatusUnspecified",
            ForwardingAddresVerificationStatusEnum::Accepted => "accepted",
            ForwardingAddresVerificationStatusEnum::Pending => "pending",
        }
    }
}

impl std::convert::TryFrom< &str> for ForwardingAddresVerificationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "verificationStatusUnspecified" => Ok(ForwardingAddresVerificationStatusEnum::VerificationStatusUnspecified),
           "accepted" => Ok(ForwardingAddresVerificationStatusEnum::Accepted),
           "pending" => Ok(ForwardingAddresVerificationStatusEnum::Pending),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ForwardingAddresVerificationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImapSettingExpungeBehaviorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The action that will be executed on a message when it is marked as deleted and expunged from the last visible IMAP folder.
pub enum ImapSettingExpungeBehaviorEnum {
    

    /// Unspecified behavior.
    ///
    /// "expungeBehaviorUnspecified"
    #[serde(rename="expungeBehaviorUnspecified")]
    ExpungeBehaviorUnspecified,
    

    /// Archive messages marked as deleted.
    ///
    /// "archive"
    #[serde(rename="archive")]
    Archive,
    

    /// Move messages marked as deleted to the trash.
    ///
    /// "trash"
    #[serde(rename="trash")]
    Trash,
    

    /// Immediately and permanently delete messages marked as deleted. The expunged messages cannot be recovered.
    ///
    /// "deleteForever"
    #[serde(rename="deleteForever")]
    DeleteForever,
}

impl AsRef<str> for ImapSettingExpungeBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImapSettingExpungeBehaviorEnum::ExpungeBehaviorUnspecified => "expungeBehaviorUnspecified",
            ImapSettingExpungeBehaviorEnum::Archive => "archive",
            ImapSettingExpungeBehaviorEnum::Trash => "trash",
            ImapSettingExpungeBehaviorEnum::DeleteForever => "deleteForever",
        }
    }
}

impl std::convert::TryFrom< &str> for ImapSettingExpungeBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "expungeBehaviorUnspecified" => Ok(ImapSettingExpungeBehaviorEnum::ExpungeBehaviorUnspecified),
           "archive" => Ok(ImapSettingExpungeBehaviorEnum::Archive),
           "trash" => Ok(ImapSettingExpungeBehaviorEnum::Trash),
           "deleteForever" => Ok(ImapSettingExpungeBehaviorEnum::DeleteForever),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImapSettingExpungeBehaviorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LabelLabelListVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The visibility of the label in the label list in the Gmail web interface.
pub enum LabelLabelListVisibilityEnum {
    

    /// Show the label in the label list.
    ///
    /// "labelShow"
    #[serde(rename="labelShow")]
    LabelShow,
    

    /// Show the label if there are any unread messages with that label.
    ///
    /// "labelShowIfUnread"
    #[serde(rename="labelShowIfUnread")]
    LabelShowIfUnread,
    

    /// Do not show the label in the label list.
    ///
    /// "labelHide"
    #[serde(rename="labelHide")]
    LabelHide,
}

impl AsRef<str> for LabelLabelListVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LabelLabelListVisibilityEnum::LabelShow => "labelShow",
            LabelLabelListVisibilityEnum::LabelShowIfUnread => "labelShowIfUnread",
            LabelLabelListVisibilityEnum::LabelHide => "labelHide",
        }
    }
}

impl std::convert::TryFrom< &str> for LabelLabelListVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "labelShow" => Ok(LabelLabelListVisibilityEnum::LabelShow),
           "labelShowIfUnread" => Ok(LabelLabelListVisibilityEnum::LabelShowIfUnread),
           "labelHide" => Ok(LabelLabelListVisibilityEnum::LabelHide),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LabelLabelListVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LabelMessageListVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The visibility of messages with this label in the message list in the Gmail web interface.
pub enum LabelMessageListVisibilityEnum {
    

    /// Show the label in the message list.
    ///
    /// "show"
    #[serde(rename="show")]
    Show,
    

    /// Do not show the label in the message list.
    ///
    /// "hide"
    #[serde(rename="hide")]
    Hide,
}

impl AsRef<str> for LabelMessageListVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LabelMessageListVisibilityEnum::Show => "show",
            LabelMessageListVisibilityEnum::Hide => "hide",
        }
    }
}

impl std::convert::TryFrom< &str> for LabelMessageListVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "show" => Ok(LabelMessageListVisibilityEnum::Show),
           "hide" => Ok(LabelMessageListVisibilityEnum::Hide),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LabelMessageListVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LabelTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads.
pub enum LabelTypeEnum {
    

    /// Labels created by Gmail.
    ///
    /// "system"
    #[serde(rename="system")]
    System,
    

    /// Custom labels created by the user or application.
    ///
    /// "user"
    #[serde(rename="user")]
    User,
}

impl AsRef<str> for LabelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LabelTypeEnum::System => "system",
            LabelTypeEnum::User => "user",
        }
    }
}

impl std::convert::TryFrom< &str> for LabelTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "system" => Ok(LabelTypeEnum::System),
           "user" => Ok(LabelTypeEnum::User),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LabelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PopSettingAccessWindowEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The range of messages which are accessible via POP.
pub enum PopSettingAccessWindowEnum {
    

    /// Unspecified range.
    ///
    /// "accessWindowUnspecified"
    #[serde(rename="accessWindowUnspecified")]
    AccessWindowUnspecified,
    

    /// Indicates that no messages are accessible via POP.
    ///
    /// "disabled"
    #[serde(rename="disabled")]
    Disabled,
    

    /// Indicates that unfetched messages received after some past point in time are accessible via POP.
    ///
    /// "fromNowOn"
    #[serde(rename="fromNowOn")]
    FromNowOn,
    

    /// Indicates that all unfetched messages are accessible via POP.
    ///
    /// "allMail"
    #[serde(rename="allMail")]
    AllMail,
}

impl AsRef<str> for PopSettingAccessWindowEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PopSettingAccessWindowEnum::AccessWindowUnspecified => "accessWindowUnspecified",
            PopSettingAccessWindowEnum::Disabled => "disabled",
            PopSettingAccessWindowEnum::FromNowOn => "fromNowOn",
            PopSettingAccessWindowEnum::AllMail => "allMail",
        }
    }
}

impl std::convert::TryFrom< &str> for PopSettingAccessWindowEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "accessWindowUnspecified" => Ok(PopSettingAccessWindowEnum::AccessWindowUnspecified),
           "disabled" => Ok(PopSettingAccessWindowEnum::Disabled),
           "fromNowOn" => Ok(PopSettingAccessWindowEnum::FromNowOn),
           "allMail" => Ok(PopSettingAccessWindowEnum::AllMail),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PopSettingAccessWindowEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PopSettingDispositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The action that will be executed on a message after it has been fetched via POP.
pub enum PopSettingDispositionEnum {
    

    /// Unspecified disposition.
    ///
    /// "dispositionUnspecified"
    #[serde(rename="dispositionUnspecified")]
    DispositionUnspecified,
    

    /// Leave the message in the `INBOX`.
    ///
    /// "leaveInInbox"
    #[serde(rename="leaveInInbox")]
    LeaveInInbox,
    

    /// Archive the message.
    ///
    /// "archive"
    #[serde(rename="archive")]
    Archive,
    

    /// Move the message to the `TRASH`.
    ///
    /// "trash"
    #[serde(rename="trash")]
    Trash,
    

    /// Leave the message in the `INBOX` and mark it as read.
    ///
    /// "markRead"
    #[serde(rename="markRead")]
    MarkRead,
}

impl AsRef<str> for PopSettingDispositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PopSettingDispositionEnum::DispositionUnspecified => "dispositionUnspecified",
            PopSettingDispositionEnum::LeaveInInbox => "leaveInInbox",
            PopSettingDispositionEnum::Archive => "archive",
            PopSettingDispositionEnum::Trash => "trash",
            PopSettingDispositionEnum::MarkRead => "markRead",
        }
    }
}

impl std::convert::TryFrom< &str> for PopSettingDispositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "dispositionUnspecified" => Ok(PopSettingDispositionEnum::DispositionUnspecified),
           "leaveInInbox" => Ok(PopSettingDispositionEnum::LeaveInInbox),
           "archive" => Ok(PopSettingDispositionEnum::Archive),
           "trash" => Ok(PopSettingDispositionEnum::Trash),
           "markRead" => Ok(PopSettingDispositionEnum::MarkRead),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PopSettingDispositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SendAVerificationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether this address has been verified for use as a send-as alias. Read-only. This setting only applies to custom "from" aliases.
pub enum SendAVerificationStatusEnum {
    

    /// Unspecified verification status.
    ///
    /// "verificationStatusUnspecified"
    #[serde(rename="verificationStatusUnspecified")]
    VerificationStatusUnspecified,
    

    /// The address is ready to use as a send-as alias.
    ///
    /// "accepted"
    #[serde(rename="accepted")]
    Accepted,
    

    /// The address is awaiting verification by the owner.
    ///
    /// "pending"
    #[serde(rename="pending")]
    Pending,
}

impl AsRef<str> for SendAVerificationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SendAVerificationStatusEnum::VerificationStatusUnspecified => "verificationStatusUnspecified",
            SendAVerificationStatusEnum::Accepted => "accepted",
            SendAVerificationStatusEnum::Pending => "pending",
        }
    }
}

impl std::convert::TryFrom< &str> for SendAVerificationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "verificationStatusUnspecified" => Ok(SendAVerificationStatusEnum::VerificationStatusUnspecified),
           "accepted" => Ok(SendAVerificationStatusEnum::Accepted),
           "pending" => Ok(SendAVerificationStatusEnum::Pending),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SendAVerificationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SmtpMsaSecurityModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The protocol that will be used to secure communication with the SMTP service. Required.
pub enum SmtpMsaSecurityModeEnum {
    

    /// Unspecified security mode.
    ///
    /// "securityModeUnspecified"
    #[serde(rename="securityModeUnspecified")]
    SecurityModeUnspecified,
    

    /// Communication with the remote SMTP service is unsecured. Requires port 25.
    ///
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// Communication with the remote SMTP service is secured using SSL.
    ///
    /// "ssl"
    #[serde(rename="ssl")]
    Ssl,
    

    /// Communication with the remote SMTP service is secured using STARTTLS.
    ///
    /// "starttls"
    #[serde(rename="starttls")]
    Starttls,
}

impl AsRef<str> for SmtpMsaSecurityModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SmtpMsaSecurityModeEnum::SecurityModeUnspecified => "securityModeUnspecified",
            SmtpMsaSecurityModeEnum::None => "none",
            SmtpMsaSecurityModeEnum::Ssl => "ssl",
            SmtpMsaSecurityModeEnum::Starttls => "starttls",
        }
    }
}

impl std::convert::TryFrom< &str> for SmtpMsaSecurityModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "securityModeUnspecified" => Ok(SmtpMsaSecurityModeEnum::SecurityModeUnspecified),
           "none" => Ok(SmtpMsaSecurityModeEnum::None),
           "ssl" => Ok(SmtpMsaSecurityModeEnum::Ssl),
           "starttls" => Ok(SmtpMsaSecurityModeEnum::Starttls),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SmtpMsaSecurityModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WatchRequestLabelFilterActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filtering behavior of `labelIds list` specified. This field is deprecated because it caused incorrect behavior in some cases; use `label_filter_behavior` instead.
pub enum WatchRequestLabelFilterActionEnum {
    

    /// Only get push notifications for message changes relating to labelIds specified.
    ///
    /// "include"
    #[serde(rename="include")]
    Include,
    

    /// Get push notifications for all message changes except those relating to labelIds specified.
    ///
    /// "exclude"
    #[serde(rename="exclude")]
    Exclude,
}

impl AsRef<str> for WatchRequestLabelFilterActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WatchRequestLabelFilterActionEnum::Include => "include",
            WatchRequestLabelFilterActionEnum::Exclude => "exclude",
        }
    }
}

impl std::convert::TryFrom< &str> for WatchRequestLabelFilterActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "include" => Ok(WatchRequestLabelFilterActionEnum::Include),
           "exclude" => Ok(WatchRequestLabelFilterActionEnum::Exclude),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WatchRequestLabelFilterActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WatchRequestLabelFilterBehaviorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filtering behavior of `labelIds list` specified. This field replaces `label_filter_action`; if set, `label_filter_action` is ignored.
pub enum WatchRequestLabelFilterBehaviorEnum {
    

    /// Only get push notifications for message changes relating to labelIds specified.
    ///
    /// "include"
    #[serde(rename="include")]
    Include,
    

    /// Get push notifications for all message changes except those relating to labelIds specified.
    ///
    /// "exclude"
    #[serde(rename="exclude")]
    Exclude,
}

impl AsRef<str> for WatchRequestLabelFilterBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WatchRequestLabelFilterBehaviorEnum::Include => "include",
            WatchRequestLabelFilterBehaviorEnum::Exclude => "exclude",
        }
    }
}

impl std::convert::TryFrom< &str> for WatchRequestLabelFilterBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "include" => Ok(WatchRequestLabelFilterBehaviorEnum::Include),
           "exclude" => Ok(WatchRequestLabelFilterBehaviorEnum::Exclude),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WatchRequestLabelFilterBehaviorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The format to return the messages in.
pub enum UserFormatEnum {
    

    /// Returns the full email message data with body content parsed in the `payload` field; the `raw` field is not used. Format cannot be used when accessing the api using the gmail.metadata scope.
    ///
    /// "full"
    #[serde(rename="full")]
    Full,
    

    /// Returns only email message IDs, labels, and email headers.
    ///
    /// "metadata"
    #[serde(rename="metadata")]
    Metadata,
    

    /// Returns only email message IDs and labels; does not return the email headers, body, or payload.
    ///
    /// "minimal"
    #[serde(rename="minimal")]
    Minimal,
}

impl AsRef<str> for UserFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserFormatEnum::Full => "full",
            UserFormatEnum::Metadata => "metadata",
            UserFormatEnum::Minimal => "minimal",
        }
    }
}

impl std::convert::TryFrom< &str> for UserFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "full" => Ok(UserFormatEnum::Full),
           "metadata" => Ok(UserFormatEnum::Metadata),
           "minimal" => Ok(UserFormatEnum::Minimal),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for UserFormatEnum {
    fn default() -> UserFormatEnum {
        UserFormatEnum::Full
    }
}

// endregion


// region UserHistoryTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// History types to be returned by the function
pub enum UserHistoryTypesEnum {
    
    /// "messageAdded"
    #[serde(rename="messageAdded")]
    MessageAdded,
    
    /// "messageDeleted"
    #[serde(rename="messageDeleted")]
    MessageDeleted,
    
    /// "labelAdded"
    #[serde(rename="labelAdded")]
    LabelAdded,
    
    /// "labelRemoved"
    #[serde(rename="labelRemoved")]
    LabelRemoved,
}

impl AsRef<str> for UserHistoryTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserHistoryTypesEnum::MessageAdded => "messageAdded",
            UserHistoryTypesEnum::MessageDeleted => "messageDeleted",
            UserHistoryTypesEnum::LabelAdded => "labelAdded",
            UserHistoryTypesEnum::LabelRemoved => "labelRemoved",
        }
    }
}

impl std::convert::TryFrom< &str> for UserHistoryTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "messageAdded" => Ok(UserHistoryTypesEnum::MessageAdded),
           "messageDeleted" => Ok(UserHistoryTypesEnum::MessageDeleted),
           "labelAdded" => Ok(UserHistoryTypesEnum::LabelAdded),
           "labelRemoved" => Ok(UserHistoryTypesEnum::LabelRemoved),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserHistoryTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserInternalDateSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Source for Gmail's internal date of the message.
pub enum UserInternalDateSourceEnum {
    

    /// Internal message date set to current time when received by Gmail.
    ///
    /// "receivedTime"
    #[serde(rename="receivedTime")]
    ReceivedTime,
    

    /// Internal message time based on 'Date' header in email, when valid.
    ///
    /// "dateHeader"
    #[serde(rename="dateHeader")]
    DateHeader,
}

impl AsRef<str> for UserInternalDateSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserInternalDateSourceEnum::ReceivedTime => "receivedTime",
            UserInternalDateSourceEnum::DateHeader => "dateHeader",
        }
    }
}

impl std::convert::TryFrom< &str> for UserInternalDateSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "receivedTime" => Ok(UserInternalDateSourceEnum::ReceivedTime),
           "dateHeader" => Ok(UserInternalDateSourceEnum::DateHeader),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserInternalDateSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for UserInternalDateSourceEnum {
    fn default() -> UserInternalDateSourceEnum {
        UserInternalDateSourceEnum::ReceivedTime
    }
}

// endregion


