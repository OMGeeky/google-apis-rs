use super::*;



// region SubscriptionDeletionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the subscription is to be fully cancelled or downgraded
pub enum SubscriptionDeletionTypeEnum {
    

    /// Cancels the subscription immediately
    ///
    /// "cancel"
    #[serde(rename="cancel")]
    Cancel,
    

    /// Downgrades a Google Apps for Business subscription to Google Apps
    ///
    /// "downgrade"
    #[serde(rename="downgrade")]
    Downgrade,
    

    /// Suspends the subscriptions for 4 days before cancelling it
    ///
    /// "suspend"
    #[serde(rename="suspend")]
    Suspend,
    

    /// Transfers a subscription directly to Google
    ///
    /// "transfer_to_direct"
    #[serde(rename="transfer_to_direct")]
    TransferToDirect,
}

impl AsRef<str> for SubscriptionDeletionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionDeletionTypeEnum::Cancel => "cancel",
            SubscriptionDeletionTypeEnum::Downgrade => "downgrade",
            SubscriptionDeletionTypeEnum::Suspend => "suspend",
            SubscriptionDeletionTypeEnum::TransferToDirect => "transfer_to_direct",
        }
    }
}

impl std::convert::TryFrom< &str> for SubscriptionDeletionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "cancel" => Ok(SubscriptionDeletionTypeEnum::Cancel),
           "downgrade" => Ok(SubscriptionDeletionTypeEnum::Downgrade),
           "suspend" => Ok(SubscriptionDeletionTypeEnum::Suspend),
           "transfer_to_direct" => Ok(SubscriptionDeletionTypeEnum::TransferToDirect),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionDeletionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


