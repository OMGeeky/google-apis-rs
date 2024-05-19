use super::*;



// region CreativeDealsStatusFilterEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// When specified, only creatives having the given deals status are returned.
pub enum CreativeDealsStatusFilterEnum {
    

    /// Creatives which have been approved for serving on deals.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    

    /// Creatives which have been conditionally approved for serving on deals.
    ///
    /// "conditionally_approved"
    #[serde(rename="conditionally_approved")]
    ConditionallyApproved,
    

    /// Creatives which have been disapproved for serving on deals.
    ///
    /// "disapproved"
    #[serde(rename="disapproved")]
    Disapproved,
    

    /// Creatives whose deals status is not yet checked.
    ///
    /// "not_checked"
    #[serde(rename="not_checked")]
    NotChecked,
}

impl AsRef<str> for CreativeDealsStatusFilterEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeDealsStatusFilterEnum::Approved => "approved",
            CreativeDealsStatusFilterEnum::ConditionallyApproved => "conditionally_approved",
            CreativeDealsStatusFilterEnum::Disapproved => "disapproved",
            CreativeDealsStatusFilterEnum::NotChecked => "not_checked",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeDealsStatusFilterEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "approved" => Ok(CreativeDealsStatusFilterEnum::Approved),
           "conditionally_approved" => Ok(CreativeDealsStatusFilterEnum::ConditionallyApproved),
           "disapproved" => Ok(CreativeDealsStatusFilterEnum::Disapproved),
           "not_checked" => Ok(CreativeDealsStatusFilterEnum::NotChecked),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeDealsStatusFilterEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeOpenAuctionStatusFilterEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// When specified, only creatives having the given open auction status are returned.
pub enum CreativeOpenAuctionStatusFilterEnum {
    

    /// Creatives which have been approved for serving on the open auction.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    

    /// Creatives which have been conditionally approved for serving on the open auction.
    ///
    /// "conditionally_approved"
    #[serde(rename="conditionally_approved")]
    ConditionallyApproved,
    

    /// Creatives which have been disapproved for serving on the open auction.
    ///
    /// "disapproved"
    #[serde(rename="disapproved")]
    Disapproved,
    

    /// Creatives whose open auction status is not yet checked.
    ///
    /// "not_checked"
    #[serde(rename="not_checked")]
    NotChecked,
}

impl AsRef<str> for CreativeOpenAuctionStatusFilterEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeOpenAuctionStatusFilterEnum::Approved => "approved",
            CreativeOpenAuctionStatusFilterEnum::ConditionallyApproved => "conditionally_approved",
            CreativeOpenAuctionStatusFilterEnum::Disapproved => "disapproved",
            CreativeOpenAuctionStatusFilterEnum::NotChecked => "not_checked",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeOpenAuctionStatusFilterEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "approved" => Ok(CreativeOpenAuctionStatusFilterEnum::Approved),
           "conditionally_approved" => Ok(CreativeOpenAuctionStatusFilterEnum::ConditionallyApproved),
           "disapproved" => Ok(CreativeOpenAuctionStatusFilterEnum::Disapproved),
           "not_checked" => Ok(CreativeOpenAuctionStatusFilterEnum::NotChecked),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeOpenAuctionStatusFilterEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProposalUpdateActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The proposed action to take on the proposal. This field is required and it must be set when updating a proposal.
pub enum ProposalUpdateActionEnum {
    
    /// "accept"
    #[serde(rename="accept")]
    Accept,
    
    /// "cancel"
    #[serde(rename="cancel")]
    Cancel,
    
    /// "propose"
    #[serde(rename="propose")]
    Propose,
    
    /// "proposeAndAccept"
    #[serde(rename="proposeAndAccept")]
    ProposeAndAccept,
    
    /// "unknownAction"
    #[serde(rename="unknownAction")]
    UnknownAction,
    
    /// "updateNonTerms"
    #[serde(rename="updateNonTerms")]
    UpdateNonTerms,
}

impl AsRef<str> for ProposalUpdateActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProposalUpdateActionEnum::Accept => "accept",
            ProposalUpdateActionEnum::Cancel => "cancel",
            ProposalUpdateActionEnum::Propose => "propose",
            ProposalUpdateActionEnum::ProposeAndAccept => "proposeAndAccept",
            ProposalUpdateActionEnum::UnknownAction => "unknownAction",
            ProposalUpdateActionEnum::UpdateNonTerms => "updateNonTerms",
        }
    }
}

impl std::convert::TryFrom< &str> for ProposalUpdateActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "accept" => Ok(ProposalUpdateActionEnum::Accept),
           "cancel" => Ok(ProposalUpdateActionEnum::Cancel),
           "propose" => Ok(ProposalUpdateActionEnum::Propose),
           "proposeAndAccept" => Ok(ProposalUpdateActionEnum::ProposeAndAccept),
           "unknownAction" => Ok(ProposalUpdateActionEnum::UnknownAction),
           "updateNonTerms" => Ok(ProposalUpdateActionEnum::UpdateNonTerms),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProposalUpdateActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


