use super::*;



// region GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Level of the catalog at which events are uploaded. See https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels for more details.
pub enum GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum {
    

    /// Unknown value - should never be used.
    ///
    /// "CATALOG_ITEM_LEVEL_UNSPECIFIED"
    #[serde(rename="CATALOG_ITEM_LEVEL_UNSPECIFIED")]
    CATALOGITEMLEVELUNSPECIFIED,
    

    /// Catalog items are at variant level.
    ///
    /// "VARIANT"
    #[serde(rename="VARIANT")]
    VARIANT,
    

    /// Catalog items are at master level.
    ///
    /// "MASTER"
    #[serde(rename="MASTER")]
    MASTER,
}

impl AsRef<str> for GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum::CATALOGITEMLEVELUNSPECIFIED => "CATALOG_ITEM_LEVEL_UNSPECIFIED",
            GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum::VARIANT => "VARIANT",
            GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum::MASTER => "MASTER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CATALOG_ITEM_LEVEL_UNSPECIFIED" => Ok(GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum::CATALOGITEMLEVELUNSPECIFIED),
           "VARIANT" => Ok(GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum::VARIANT),
           "MASTER" => Ok(GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum::MASTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Level of the catalog at which predictions are made. See https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels for more details.
pub enum GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum {
    

    /// Unknown value - should never be used.
    ///
    /// "CATALOG_ITEM_LEVEL_UNSPECIFIED"
    #[serde(rename="CATALOG_ITEM_LEVEL_UNSPECIFIED")]
    CATALOGITEMLEVELUNSPECIFIED,
    

    /// Catalog items are at variant level.
    ///
    /// "VARIANT"
    #[serde(rename="VARIANT")]
    VARIANT,
    

    /// Catalog items are at master level.
    ///
    /// "MASTER"
    #[serde(rename="MASTER")]
    MASTER,
}

impl AsRef<str> for GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum::CATALOGITEMLEVELUNSPECIFIED => "CATALOG_ITEM_LEVEL_UNSPECIFIED",
            GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum::VARIANT => "VARIANT",
            GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum::MASTER => "MASTER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CATALOG_ITEM_LEVEL_UNSPECIFIED" => Ok(GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum::CATALOGITEMLEVELUNSPECIFIED),
           "VARIANT" => Ok(GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum::VARIANT),
           "MASTER" => Ok(GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum::MASTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Online stock state of the catalog item. Default is `IN_STOCK`.
pub enum GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum {
    

    /// Default item stock status. Should never be used.
    ///
    /// "STOCK_STATE_UNSPECIFIED"
    #[serde(rename="STOCK_STATE_UNSPECIFIED")]
    STOCKSTATEUNSPECIFIED,
    

    /// Item in stock.
    ///
    /// "IN_STOCK"
    #[serde(rename="IN_STOCK")]
    INSTOCK,
    

    /// Item out of stock.
    ///
    /// "OUT_OF_STOCK"
    #[serde(rename="OUT_OF_STOCK")]
    OUTOFSTOCK,
    

    /// Item that is in pre-order state.
    ///
    /// "PREORDER"
    #[serde(rename="PREORDER")]
    PREORDER,
    

    /// Item that is back-ordered (i.e. temporarily out of stock).
    ///
    /// "BACKORDER"
    #[serde(rename="BACKORDER")]
    BACKORDER,
}

impl AsRef<str> for GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum::STOCKSTATEUNSPECIFIED => "STOCK_STATE_UNSPECIFIED",
            GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum::INSTOCK => "IN_STOCK",
            GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum::OUTOFSTOCK => "OUT_OF_STOCK",
            GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum::PREORDER => "PREORDER",
            GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum::BACKORDER => "BACKORDER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STOCK_STATE_UNSPECIFIED" => Ok(GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum::STOCKSTATEUNSPECIFIED),
           "IN_STOCK" => Ok(GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum::INSTOCK),
           "OUT_OF_STOCK" => Ok(GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum::OUTOFSTOCK),
           "PREORDER" => Ok(GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum::PREORDER),
           "BACKORDER" => Ok(GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum::BACKORDER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Item stock state. If provided, this overrides the stock state in Catalog for items in this event.
pub enum GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum {
    

    /// Default item stock status. Should never be used.
    ///
    /// "STOCK_STATE_UNSPECIFIED"
    #[serde(rename="STOCK_STATE_UNSPECIFIED")]
    STOCKSTATEUNSPECIFIED,
    

    /// Item in stock.
    ///
    /// "IN_STOCK"
    #[serde(rename="IN_STOCK")]
    INSTOCK,
    

    /// Item out of stock.
    ///
    /// "OUT_OF_STOCK"
    #[serde(rename="OUT_OF_STOCK")]
    OUTOFSTOCK,
    

    /// Item that is in pre-order state.
    ///
    /// "PREORDER"
    #[serde(rename="PREORDER")]
    PREORDER,
    

    /// Item that is back-ordered (i.e. temporarily out of stock).
    ///
    /// "BACKORDER"
    #[serde(rename="BACKORDER")]
    BACKORDER,
}

impl AsRef<str> for GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum::STOCKSTATEUNSPECIFIED => "STOCK_STATE_UNSPECIFIED",
            GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum::INSTOCK => "IN_STOCK",
            GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum::OUTOFSTOCK => "OUT_OF_STOCK",
            GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum::PREORDER => "PREORDER",
            GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum::BACKORDER => "BACKORDER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STOCK_STATE_UNSPECIFIED" => Ok(GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum::STOCKSTATEUNSPECIFIED),
           "IN_STOCK" => Ok(GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum::INSTOCK),
           "OUT_OF_STOCK" => Ok(GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum::OUTOFSTOCK),
           "PREORDER" => Ok(GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum::PREORDER),
           "BACKORDER" => Ok(GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum::BACKORDER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the catalog rejoin to define the scope and range of the user events to be rejoined with catalog items.
pub enum GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum {
    

    /// Rejoin catalogs with all events including both joined events and unjoined events.
    ///
    /// "USER_EVENT_REJOIN_SCOPE_UNSPECIFIED"
    #[serde(rename="USER_EVENT_REJOIN_SCOPE_UNSPECIFIED")]
    USEREVENTREJOINSCOPEUNSPECIFIED,
    

    /// Only rejoin catalogs with joined events.
    ///
    /// "JOINED_EVENTS"
    #[serde(rename="JOINED_EVENTS")]
    JOINEDEVENTS,
    

    /// Only rejoin catalogs with unjoined events.
    ///
    /// "UNJOINED_EVENTS"
    #[serde(rename="UNJOINED_EVENTS")]
    UNJOINEDEVENTS,
}

impl AsRef<str> for GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum::USEREVENTREJOINSCOPEUNSPECIFIED => "USER_EVENT_REJOIN_SCOPE_UNSPECIFIED",
            GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum::JOINEDEVENTS => "JOINED_EVENTS",
            GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum::UNJOINEDEVENTS => "UNJOINED_EVENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_EVENT_REJOIN_SCOPE_UNSPECIFIED" => Ok(GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum::USEREVENTREJOINSCOPEUNSPECIFIED),
           "JOINED_EVENTS" => Ok(GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum::JOINEDEVENTS),
           "UNJOINED_EVENTS" => Ok(GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum::UNJOINEDEVENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. This field should *not* be set when using JavaScript pixel or the Recommendations AI Tag. Defaults to `EVENT_SOURCE_UNSPECIFIED`.
pub enum GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum {
    

    /// Unspecified event source.
    ///
    /// "EVENT_SOURCE_UNSPECIFIED"
    #[serde(rename="EVENT_SOURCE_UNSPECIFIED")]
    EVENTSOURCEUNSPECIFIED,
    

    /// The event is ingested via a javascript pixel or Recommendations AI Tag through automl datalayer or JS Macros.
    ///
    /// "AUTOML"
    #[serde(rename="AUTOML")]
    AUTOML,
    

    /// The event is ingested via Recommendations AI Tag through Enhanced Ecommerce datalayer.
    ///
    /// "ECOMMERCE"
    #[serde(rename="ECOMMERCE")]
    ECOMMERCE,
    

    /// The event is ingested via Import user events API.
    ///
    /// "BATCH_UPLOAD"
    #[serde(rename="BATCH_UPLOAD")]
    BATCHUPLOAD,
}

impl AsRef<str> for GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum::EVENTSOURCEUNSPECIFIED => "EVENT_SOURCE_UNSPECIFIED",
            GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum::AUTOML => "AUTOML",
            GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum::ECOMMERCE => "ECOMMERCE",
            GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum::BATCHUPLOAD => "BATCH_UPLOAD",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_SOURCE_UNSPECIFIED" => Ok(GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum::EVENTSOURCEUNSPECIFIED),
           "AUTOML" => Ok(GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum::AUTOML),
           "ECOMMERCE" => Ok(GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum::ECOMMERCE),
           "BATCH_UPLOAD" => Ok(GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum::BATCHUPLOAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


