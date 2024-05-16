use super::*;



// region OrderreturnOrderByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Return the results in the specified order.
pub enum OrderreturnOrderByEnum {
    
    /// "returnCreationTimeAsc"
    #[serde(rename="returnCreationTimeAsc")]
    ReturnCreationTimeAsc,
    
    /// "returnCreationTimeDesc"
    #[serde(rename="returnCreationTimeDesc")]
    ReturnCreationTimeDesc,
}

impl AsRef<str> for OrderreturnOrderByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderreturnOrderByEnum::ReturnCreationTimeAsc => "returnCreationTimeAsc",
            OrderreturnOrderByEnum::ReturnCreationTimeDesc => "returnCreationTimeDesc",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderreturnOrderByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "returnCreationTimeAsc" => Ok(OrderreturnOrderByEnum::ReturnCreationTimeAsc),
           "returnCreationTimeDesc" => Ok(OrderreturnOrderByEnum::ReturnCreationTimeDesc),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderreturnOrderByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderTemplateNameEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The name of the template to retrieve.
pub enum OrderTemplateNameEnum {
    
    /// "template1"
    #[serde(rename="template1")]
    Template1,
    
    /// "template1a"
    #[serde(rename="template1a")]
    Template1a,
    
    /// "template1b"
    #[serde(rename="template1b")]
    Template1b,
    
    /// "template2"
    #[serde(rename="template2")]
    Template2,
}

impl AsRef<str> for OrderTemplateNameEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderTemplateNameEnum::Template1 => "template1",
            OrderTemplateNameEnum::Template1a => "template1a",
            OrderTemplateNameEnum::Template1b => "template1b",
            OrderTemplateNameEnum::Template2 => "template2",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderTemplateNameEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "template1" => Ok(OrderTemplateNameEnum::Template1),
           "template1a" => Ok(OrderTemplateNameEnum::Template1a),
           "template1b" => Ok(OrderTemplateNameEnum::Template1b),
           "template2" => Ok(OrderTemplateNameEnum::Template2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderTemplateNameEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderOrderByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ordering of the returned list. The only supported value are placedDate desc and placedDate asc for now, which returns orders sorted by placement date. "placedDate desc" stands for listing orders by placement date, from oldest to most recent. "placedDate asc" stands for listing orders by placement date, from most recent to oldest. In future releases we'll support other sorting criteria.
pub enum OrderOrderByEnum {
    
    /// "placedDate asc"
    #[serde(rename="placedDate asc")]
    PlacedDateAsc,
    
    /// "placedDate desc"
    #[serde(rename="placedDate desc")]
    PlacedDateDesc,
}

impl AsRef<str> for OrderOrderByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderOrderByEnum::PlacedDateAsc => "placedDate asc",
            OrderOrderByEnum::PlacedDateDesc => "placedDate desc",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderOrderByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "placedDate asc" => Ok(OrderOrderByEnum::PlacedDateAsc),
           "placedDate desc" => Ok(OrderOrderByEnum::PlacedDateDesc),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderOrderByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderStatusesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Obtains orders that match any of the specified statuses. Multiple values can be specified with comma separation. Additionally, please note that active is a shortcut for pendingShipment and partiallyShipped, and completed is a shortcut for shipped , partiallyDelivered, delivered, partiallyReturned, returned, and canceled.
pub enum OrderStatusesEnum {
    
    /// "active"
    #[serde(rename="active")]
    Active,
    
    /// "canceled"
    #[serde(rename="canceled")]
    Canceled,
    
    /// "completed"
    #[serde(rename="completed")]
    Completed,
    
    /// "delivered"
    #[serde(rename="delivered")]
    Delivered,
    
    /// "inProgress"
    #[serde(rename="inProgress")]
    InProgress,
    
    /// "partiallyDelivered"
    #[serde(rename="partiallyDelivered")]
    PartiallyDelivered,
    
    /// "partiallyReturned"
    #[serde(rename="partiallyReturned")]
    PartiallyReturned,
    
    /// "partiallyShipped"
    #[serde(rename="partiallyShipped")]
    PartiallyShipped,
    
    /// "pendingShipment"
    #[serde(rename="pendingShipment")]
    PendingShipment,
    
    /// "returned"
    #[serde(rename="returned")]
    Returned,
    
    /// "shipped"
    #[serde(rename="shipped")]
    Shipped,
}

impl AsRef<str> for OrderStatusesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderStatusesEnum::Active => "active",
            OrderStatusesEnum::Canceled => "canceled",
            OrderStatusesEnum::Completed => "completed",
            OrderStatusesEnum::Delivered => "delivered",
            OrderStatusesEnum::InProgress => "inProgress",
            OrderStatusesEnum::PartiallyDelivered => "partiallyDelivered",
            OrderStatusesEnum::PartiallyReturned => "partiallyReturned",
            OrderStatusesEnum::PartiallyShipped => "partiallyShipped",
            OrderStatusesEnum::PendingShipment => "pendingShipment",
            OrderStatusesEnum::Returned => "returned",
            OrderStatusesEnum::Shipped => "shipped",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderStatusesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "active" => Ok(OrderStatusesEnum::Active),
           "canceled" => Ok(OrderStatusesEnum::Canceled),
           "completed" => Ok(OrderStatusesEnum::Completed),
           "delivered" => Ok(OrderStatusesEnum::Delivered),
           "inProgress" => Ok(OrderStatusesEnum::InProgress),
           "partiallyDelivered" => Ok(OrderStatusesEnum::PartiallyDelivered),
           "partiallyReturned" => Ok(OrderStatusesEnum::PartiallyReturned),
           "partiallyShipped" => Ok(OrderStatusesEnum::PartiallyShipped),
           "pendingShipment" => Ok(OrderStatusesEnum::PendingShipment),
           "returned" => Ok(OrderStatusesEnum::Returned),
           "shipped" => Ok(OrderStatusesEnum::Shipped),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderStatusesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


