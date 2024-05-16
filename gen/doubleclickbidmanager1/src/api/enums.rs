use super::*;



// region DownloadLineItemsRequestFileSpecEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// File specification (column names, types, order) in which the line items will be returned. Default to EWF.
pub enum DownloadLineItemsRequestFileSpecEnum {
    
    /// "EWF"
    #[serde(rename="EWF")]
    EWF,
}

impl AsRef<str> for DownloadLineItemsRequestFileSpecEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DownloadLineItemsRequestFileSpecEnum::EWF => "EWF",
        }
    }
}

impl std::convert::TryFrom< &str> for DownloadLineItemsRequestFileSpecEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EWF" => Ok(DownloadLineItemsRequestFileSpecEnum::EWF),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DownloadLineItemsRequestFileSpecEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DownloadLineItemsRequestFilterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter type used to filter line items to fetch.
pub enum DownloadLineItemsRequestFilterTypeEnum {
    
    /// "ADVERTISER_ID"
    #[serde(rename="ADVERTISER_ID")]
    ADVERTISERID,
    
    /// "INSERTION_ORDER_ID"
    #[serde(rename="INSERTION_ORDER_ID")]
    INSERTIONORDERID,
    
    /// "LINE_ITEM_ID"
    #[serde(rename="LINE_ITEM_ID")]
    LINEITEMID,
}

impl AsRef<str> for DownloadLineItemsRequestFilterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DownloadLineItemsRequestFilterTypeEnum::ADVERTISERID => "ADVERTISER_ID",
            DownloadLineItemsRequestFilterTypeEnum::INSERTIONORDERID => "INSERTION_ORDER_ID",
            DownloadLineItemsRequestFilterTypeEnum::LINEITEMID => "LINE_ITEM_ID",
        }
    }
}

impl std::convert::TryFrom< &str> for DownloadLineItemsRequestFilterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADVERTISER_ID" => Ok(DownloadLineItemsRequestFilterTypeEnum::ADVERTISERID),
           "INSERTION_ORDER_ID" => Ok(DownloadLineItemsRequestFilterTypeEnum::INSERTIONORDERID),
           "LINE_ITEM_ID" => Ok(DownloadLineItemsRequestFilterTypeEnum::LINEITEMID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DownloadLineItemsRequestFilterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DownloadLineItemsRequestFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Format in which the line items will be returned. Default to CSV.
pub enum DownloadLineItemsRequestFormatEnum {
    
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
}

impl AsRef<str> for DownloadLineItemsRequestFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DownloadLineItemsRequestFormatEnum::CSV => "CSV",
        }
    }
}

impl std::convert::TryFrom< &str> for DownloadLineItemsRequestFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CSV" => Ok(DownloadLineItemsRequestFormatEnum::CSV),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DownloadLineItemsRequestFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DownloadRequestFileTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// File types that will be returned. If INVENTORY_SOURCE is requested, no other file types may be requested. Acceptable values are: - "AD" - "AD_GROUP" - "CAMPAIGN" - "INSERTION_ORDER" - "INVENTORY_SOURCE" - "LINE_ITEM" 
pub enum DownloadRequestFileTypesEnum {
    
    /// "INSERTION_ORDER"
    #[serde(rename="INSERTION_ORDER")]
    INSERTIONORDER,
    
    /// "LINE_ITEM"
    #[serde(rename="LINE_ITEM")]
    LINEITEM,
    
    /// "AD_GROUP"
    #[serde(rename="AD_GROUP")]
    ADGROUP,
    
    /// "AD"
    #[serde(rename="AD")]
    AD,
    
    /// "CAMPAIGN"
    #[serde(rename="CAMPAIGN")]
    CAMPAIGN,
    
    /// "INVENTORY_SOURCE"
    #[serde(rename="INVENTORY_SOURCE")]
    INVENTORYSOURCE,
}

impl AsRef<str> for DownloadRequestFileTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DownloadRequestFileTypesEnum::INSERTIONORDER => "INSERTION_ORDER",
            DownloadRequestFileTypesEnum::LINEITEM => "LINE_ITEM",
            DownloadRequestFileTypesEnum::ADGROUP => "AD_GROUP",
            DownloadRequestFileTypesEnum::AD => "AD",
            DownloadRequestFileTypesEnum::CAMPAIGN => "CAMPAIGN",
            DownloadRequestFileTypesEnum::INVENTORYSOURCE => "INVENTORY_SOURCE",
        }
    }
}

impl std::convert::TryFrom< &str> for DownloadRequestFileTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSERTION_ORDER" => Ok(DownloadRequestFileTypesEnum::INSERTIONORDER),
           "LINE_ITEM" => Ok(DownloadRequestFileTypesEnum::LINEITEM),
           "AD_GROUP" => Ok(DownloadRequestFileTypesEnum::ADGROUP),
           "AD" => Ok(DownloadRequestFileTypesEnum::AD),
           "CAMPAIGN" => Ok(DownloadRequestFileTypesEnum::CAMPAIGN),
           "INVENTORY_SOURCE" => Ok(DownloadRequestFileTypesEnum::INVENTORYSOURCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DownloadRequestFileTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DownloadRequestFilterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter type used to filter entities to fetch. PARTNER_ID and INVENTORY_SOURCE_ID may only be used when downloading inventory sources.
pub enum DownloadRequestFilterTypeEnum {
    
    /// "ADVERTISER_ID"
    #[serde(rename="ADVERTISER_ID")]
    ADVERTISERID,
    
    /// "INSERTION_ORDER_ID"
    #[serde(rename="INSERTION_ORDER_ID")]
    INSERTIONORDERID,
    
    /// "LINE_ITEM_ID"
    #[serde(rename="LINE_ITEM_ID")]
    LINEITEMID,
    
    /// "CAMPAIGN_ID"
    #[serde(rename="CAMPAIGN_ID")]
    CAMPAIGNID,
    
    /// "INVENTORY_SOURCE_ID"
    #[serde(rename="INVENTORY_SOURCE_ID")]
    INVENTORYSOURCEID,
    
    /// "PARTNER_ID"
    #[serde(rename="PARTNER_ID")]
    PARTNERID,
}

impl AsRef<str> for DownloadRequestFilterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DownloadRequestFilterTypeEnum::ADVERTISERID => "ADVERTISER_ID",
            DownloadRequestFilterTypeEnum::INSERTIONORDERID => "INSERTION_ORDER_ID",
            DownloadRequestFilterTypeEnum::LINEITEMID => "LINE_ITEM_ID",
            DownloadRequestFilterTypeEnum::CAMPAIGNID => "CAMPAIGN_ID",
            DownloadRequestFilterTypeEnum::INVENTORYSOURCEID => "INVENTORY_SOURCE_ID",
            DownloadRequestFilterTypeEnum::PARTNERID => "PARTNER_ID",
        }
    }
}

impl std::convert::TryFrom< &str> for DownloadRequestFilterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADVERTISER_ID" => Ok(DownloadRequestFilterTypeEnum::ADVERTISERID),
           "INSERTION_ORDER_ID" => Ok(DownloadRequestFilterTypeEnum::INSERTIONORDERID),
           "LINE_ITEM_ID" => Ok(DownloadRequestFilterTypeEnum::LINEITEMID),
           "CAMPAIGN_ID" => Ok(DownloadRequestFilterTypeEnum::CAMPAIGNID),
           "INVENTORY_SOURCE_ID" => Ok(DownloadRequestFilterTypeEnum::INVENTORYSOURCEID),
           "PARTNER_ID" => Ok(DownloadRequestFilterTypeEnum::PARTNERID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DownloadRequestFilterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FilterPairTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter type.
pub enum FilterPairTypeEnum {
    
    /// "FILTER_UNKNOWN"
    #[serde(rename="FILTER_UNKNOWN")]
    FILTERUNKNOWN,
    
    /// "FILTER_DATE"
    #[serde(rename="FILTER_DATE")]
    FILTERDATE,
    
    /// "FILTER_DAY_OF_WEEK"
    #[serde(rename="FILTER_DAY_OF_WEEK")]
    FILTERDAYOFWEEK,
    
    /// "FILTER_WEEK"
    #[serde(rename="FILTER_WEEK")]
    FILTERWEEK,
    
    /// "FILTER_MONTH"
    #[serde(rename="FILTER_MONTH")]
    FILTERMONTH,
    
    /// "FILTER_YEAR"
    #[serde(rename="FILTER_YEAR")]
    FILTERYEAR,
    
    /// "FILTER_TIME_OF_DAY"
    #[serde(rename="FILTER_TIME_OF_DAY")]
    FILTERTIMEOFDAY,
    
    /// "FILTER_CONVERSION_DELAY"
    #[serde(rename="FILTER_CONVERSION_DELAY")]
    FILTERCONVERSIONDELAY,
    
    /// "FILTER_CREATIVE_ID"
    #[serde(rename="FILTER_CREATIVE_ID")]
    FILTERCREATIVEID,
    
    /// "FILTER_CREATIVE_SIZE"
    #[serde(rename="FILTER_CREATIVE_SIZE")]
    FILTERCREATIVESIZE,
    
    /// "FILTER_CREATIVE_TYPE"
    #[serde(rename="FILTER_CREATIVE_TYPE")]
    FILTERCREATIVETYPE,
    
    /// "FILTER_EXCHANGE_ID"
    #[serde(rename="FILTER_EXCHANGE_ID")]
    FILTEREXCHANGEID,
    
    /// "FILTER_AD_POSITION"
    #[serde(rename="FILTER_AD_POSITION")]
    FILTERADPOSITION,
    
    /// "FILTER_INVENTORY_SOURCE"
    #[serde(rename="FILTER_INVENTORY_SOURCE")]
    FILTERINVENTORYSOURCE,
    
    /// "FILTER_CITY"
    #[serde(rename="FILTER_CITY")]
    FILTERCITY,
    
    /// "FILTER_REGION"
    #[serde(rename="FILTER_REGION")]
    FILTERREGION,
    
    /// "FILTER_DMA"
    #[serde(rename="FILTER_DMA")]
    FILTERDMA,
    
    /// "FILTER_COUNTRY"
    #[serde(rename="FILTER_COUNTRY")]
    FILTERCOUNTRY,
    
    /// "FILTER_SITE_ID"
    #[serde(rename="FILTER_SITE_ID")]
    FILTERSITEID,
    
    /// "FILTER_CHANNEL_ID"
    #[serde(rename="FILTER_CHANNEL_ID")]
    FILTERCHANNELID,
    
    /// "FILTER_PARTNER"
    #[serde(rename="FILTER_PARTNER")]
    FILTERPARTNER,
    
    /// "FILTER_ADVERTISER"
    #[serde(rename="FILTER_ADVERTISER")]
    FILTERADVERTISER,
    
    /// "FILTER_INSERTION_ORDER"
    #[serde(rename="FILTER_INSERTION_ORDER")]
    FILTERINSERTIONORDER,
    
    /// "FILTER_LINE_ITEM"
    #[serde(rename="FILTER_LINE_ITEM")]
    FILTERLINEITEM,
    
    /// "FILTER_PARTNER_CURRENCY"
    #[serde(rename="FILTER_PARTNER_CURRENCY")]
    FILTERPARTNERCURRENCY,
    
    /// "FILTER_ADVERTISER_CURRENCY"
    #[serde(rename="FILTER_ADVERTISER_CURRENCY")]
    FILTERADVERTISERCURRENCY,
    
    /// "FILTER_ADVERTISER_TIMEZONE"
    #[serde(rename="FILTER_ADVERTISER_TIMEZONE")]
    FILTERADVERTISERTIMEZONE,
    
    /// "FILTER_LINE_ITEM_TYPE"
    #[serde(rename="FILTER_LINE_ITEM_TYPE")]
    FILTERLINEITEMTYPE,
    
    /// "FILTER_USER_LIST"
    #[serde(rename="FILTER_USER_LIST")]
    FILTERUSERLIST,
    
    /// "FILTER_USER_LIST_FIRST_PARTY"
    #[serde(rename="FILTER_USER_LIST_FIRST_PARTY")]
    FILTERUSERLISTFIRSTPARTY,
    
    /// "FILTER_USER_LIST_THIRD_PARTY"
    #[serde(rename="FILTER_USER_LIST_THIRD_PARTY")]
    FILTERUSERLISTTHIRDPARTY,
    
    /// "FILTER_TARGETED_USER_LIST"
    #[serde(rename="FILTER_TARGETED_USER_LIST")]
    FILTERTARGETEDUSERLIST,
    
    /// "FILTER_DATA_PROVIDER"
    #[serde(rename="FILTER_DATA_PROVIDER")]
    FILTERDATAPROVIDER,
    
    /// "FILTER_ORDER_ID"
    #[serde(rename="FILTER_ORDER_ID")]
    FILTERORDERID,
    
    /// "FILTER_VIDEO_PLAYER_SIZE"
    #[serde(rename="FILTER_VIDEO_PLAYER_SIZE")]
    FILTERVIDEOPLAYERSIZE,
    
    /// "FILTER_VIDEO_DURATION_SECONDS"
    #[serde(rename="FILTER_VIDEO_DURATION_SECONDS")]
    FILTERVIDEODURATIONSECONDS,
    
    /// "FILTER_KEYWORD"
    #[serde(rename="FILTER_KEYWORD")]
    FILTERKEYWORD,
    
    /// "FILTER_PAGE_CATEGORY"
    #[serde(rename="FILTER_PAGE_CATEGORY")]
    FILTERPAGECATEGORY,
    
    /// "FILTER_CAMPAIGN_DAILY_FREQUENCY"
    #[serde(rename="FILTER_CAMPAIGN_DAILY_FREQUENCY")]
    FILTERCAMPAIGNDAILYFREQUENCY,
    
    /// "FILTER_LINE_ITEM_DAILY_FREQUENCY"
    #[serde(rename="FILTER_LINE_ITEM_DAILY_FREQUENCY")]
    FILTERLINEITEMDAILYFREQUENCY,
    
    /// "FILTER_LINE_ITEM_LIFETIME_FREQUENCY"
    #[serde(rename="FILTER_LINE_ITEM_LIFETIME_FREQUENCY")]
    FILTERLINEITEMLIFETIMEFREQUENCY,
    
    /// "FILTER_OS"
    #[serde(rename="FILTER_OS")]
    FILTEROS,
    
    /// "FILTER_BROWSER"
    #[serde(rename="FILTER_BROWSER")]
    FILTERBROWSER,
    
    /// "FILTER_CARRIER"
    #[serde(rename="FILTER_CARRIER")]
    FILTERCARRIER,
    
    /// "FILTER_SITE_LANGUAGE"
    #[serde(rename="FILTER_SITE_LANGUAGE")]
    FILTERSITELANGUAGE,
    
    /// "FILTER_INVENTORY_FORMAT"
    #[serde(rename="FILTER_INVENTORY_FORMAT")]
    FILTERINVENTORYFORMAT,
    
    /// "FILTER_ZIP_CODE"
    #[serde(rename="FILTER_ZIP_CODE")]
    FILTERZIPCODE,
    
    /// "FILTER_VIDEO_RATING_TIER"
    #[serde(rename="FILTER_VIDEO_RATING_TIER")]
    FILTERVIDEORATINGTIER,
    
    /// "FILTER_VIDEO_FORMAT_SUPPORT"
    #[serde(rename="FILTER_VIDEO_FORMAT_SUPPORT")]
    FILTERVIDEOFORMATSUPPORT,
    
    /// "FILTER_VIDEO_SKIPPABLE_SUPPORT"
    #[serde(rename="FILTER_VIDEO_SKIPPABLE_SUPPORT")]
    FILTERVIDEOSKIPPABLESUPPORT,
    
    /// "FILTER_VIDEO_VPAID_SUPPORT"
    #[serde(rename="FILTER_VIDEO_VPAID_SUPPORT")]
    FILTERVIDEOVPAIDSUPPORT,
    
    /// "FILTER_VIDEO_CREATIVE_DURATION"
    #[serde(rename="FILTER_VIDEO_CREATIVE_DURATION")]
    FILTERVIDEOCREATIVEDURATION,
    
    /// "FILTER_PAGE_LAYOUT"
    #[serde(rename="FILTER_PAGE_LAYOUT")]
    FILTERPAGELAYOUT,
    
    /// "FILTER_VIDEO_AD_POSITION_IN_STREAM"
    #[serde(rename="FILTER_VIDEO_AD_POSITION_IN_STREAM")]
    FILTERVIDEOADPOSITIONINSTREAM,
    
    /// "FILTER_AGE"
    #[serde(rename="FILTER_AGE")]
    FILTERAGE,
    
    /// "FILTER_GENDER"
    #[serde(rename="FILTER_GENDER")]
    FILTERGENDER,
    
    /// "FILTER_QUARTER"
    #[serde(rename="FILTER_QUARTER")]
    FILTERQUARTER,
    
    /// "FILTER_TRUEVIEW_CONVERSION_TYPE"
    #[serde(rename="FILTER_TRUEVIEW_CONVERSION_TYPE")]
    FILTERTRUEVIEWCONVERSIONTYPE,
    
    /// "FILTER_MOBILE_GEO"
    #[serde(rename="FILTER_MOBILE_GEO")]
    FILTERMOBILEGEO,
    
    /// "FILTER_MRAID_SUPPORT"
    #[serde(rename="FILTER_MRAID_SUPPORT")]
    FILTERMRAIDSUPPORT,
    
    /// "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY"
    #[serde(rename="FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY")]
    FILTERACTIVEVIEWEXPECTEDVIEWABILITY,
    
    /// "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE"
    #[serde(rename="FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE")]
    FILTERVIDEOCREATIVEDURATIONSKIPPABLE,
    
    /// "FILTER_NIELSEN_COUNTRY_CODE"
    #[serde(rename="FILTER_NIELSEN_COUNTRY_CODE")]
    FILTERNIELSENCOUNTRYCODE,
    
    /// "FILTER_NIELSEN_DEVICE_ID"
    #[serde(rename="FILTER_NIELSEN_DEVICE_ID")]
    FILTERNIELSENDEVICEID,
    
    /// "FILTER_NIELSEN_GENDER"
    #[serde(rename="FILTER_NIELSEN_GENDER")]
    FILTERNIELSENGENDER,
    
    /// "FILTER_NIELSEN_AGE"
    #[serde(rename="FILTER_NIELSEN_AGE")]
    FILTERNIELSENAGE,
    
    /// "FILTER_INVENTORY_SOURCE_TYPE"
    #[serde(rename="FILTER_INVENTORY_SOURCE_TYPE")]
    FILTERINVENTORYSOURCETYPE,
    
    /// "FILTER_CREATIVE_WIDTH"
    #[serde(rename="FILTER_CREATIVE_WIDTH")]
    FILTERCREATIVEWIDTH,
    
    /// "FILTER_CREATIVE_HEIGHT"
    #[serde(rename="FILTER_CREATIVE_HEIGHT")]
    FILTERCREATIVEHEIGHT,
    
    /// "FILTER_DFP_ORDER_ID"
    #[serde(rename="FILTER_DFP_ORDER_ID")]
    FILTERDFPORDERID,
    
    /// "FILTER_TRUEVIEW_AGE"
    #[serde(rename="FILTER_TRUEVIEW_AGE")]
    FILTERTRUEVIEWAGE,
    
    /// "FILTER_TRUEVIEW_GENDER"
    #[serde(rename="FILTER_TRUEVIEW_GENDER")]
    FILTERTRUEVIEWGENDER,
    
    /// "FILTER_TRUEVIEW_PARENTAL_STATUS"
    #[serde(rename="FILTER_TRUEVIEW_PARENTAL_STATUS")]
    FILTERTRUEVIEWPARENTALSTATUS,
    
    /// "FILTER_TRUEVIEW_REMARKETING_LIST"
    #[serde(rename="FILTER_TRUEVIEW_REMARKETING_LIST")]
    FILTERTRUEVIEWREMARKETINGLIST,
    
    /// "FILTER_TRUEVIEW_INTEREST"
    #[serde(rename="FILTER_TRUEVIEW_INTEREST")]
    FILTERTRUEVIEWINTEREST,
    
    /// "FILTER_TRUEVIEW_AD_GROUP_ID"
    #[serde(rename="FILTER_TRUEVIEW_AD_GROUP_ID")]
    FILTERTRUEVIEWADGROUPID,
    
    /// "FILTER_TRUEVIEW_AD_GROUP_AD_ID"
    #[serde(rename="FILTER_TRUEVIEW_AD_GROUP_AD_ID")]
    FILTERTRUEVIEWADGROUPADID,
    
    /// "FILTER_TRUEVIEW_IAR_LANGUAGE"
    #[serde(rename="FILTER_TRUEVIEW_IAR_LANGUAGE")]
    FILTERTRUEVIEWIARLANGUAGE,
    
    /// "FILTER_TRUEVIEW_IAR_GENDER"
    #[serde(rename="FILTER_TRUEVIEW_IAR_GENDER")]
    FILTERTRUEVIEWIARGENDER,
    
    /// "FILTER_TRUEVIEW_IAR_AGE"
    #[serde(rename="FILTER_TRUEVIEW_IAR_AGE")]
    FILTERTRUEVIEWIARAGE,
    
    /// "FILTER_TRUEVIEW_IAR_CATEGORY"
    #[serde(rename="FILTER_TRUEVIEW_IAR_CATEGORY")]
    FILTERTRUEVIEWIARCATEGORY,
    
    /// "FILTER_TRUEVIEW_IAR_COUNTRY"
    #[serde(rename="FILTER_TRUEVIEW_IAR_COUNTRY")]
    FILTERTRUEVIEWIARCOUNTRY,
    
    /// "FILTER_TRUEVIEW_IAR_CITY"
    #[serde(rename="FILTER_TRUEVIEW_IAR_CITY")]
    FILTERTRUEVIEWIARCITY,
    
    /// "FILTER_TRUEVIEW_IAR_REGION"
    #[serde(rename="FILTER_TRUEVIEW_IAR_REGION")]
    FILTERTRUEVIEWIARREGION,
    
    /// "FILTER_TRUEVIEW_IAR_ZIPCODE"
    #[serde(rename="FILTER_TRUEVIEW_IAR_ZIPCODE")]
    FILTERTRUEVIEWIARZIPCODE,
    
    /// "FILTER_TRUEVIEW_IAR_REMARKETING_LIST"
    #[serde(rename="FILTER_TRUEVIEW_IAR_REMARKETING_LIST")]
    FILTERTRUEVIEWIARREMARKETINGLIST,
    
    /// "FILTER_TRUEVIEW_IAR_INTEREST"
    #[serde(rename="FILTER_TRUEVIEW_IAR_INTEREST")]
    FILTERTRUEVIEWIARINTEREST,
    
    /// "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS"
    #[serde(rename="FILTER_TRUEVIEW_IAR_PARENTAL_STATUS")]
    FILTERTRUEVIEWIARPARENTALSTATUS,
    
    /// "FILTER_TRUEVIEW_IAR_TIME_OF_DAY"
    #[serde(rename="FILTER_TRUEVIEW_IAR_TIME_OF_DAY")]
    FILTERTRUEVIEWIARTIMEOFDAY,
    
    /// "FILTER_TRUEVIEW_CUSTOM_AFFINITY"
    #[serde(rename="FILTER_TRUEVIEW_CUSTOM_AFFINITY")]
    FILTERTRUEVIEWCUSTOMAFFINITY,
    
    /// "FILTER_TRUEVIEW_CATEGORY"
    #[serde(rename="FILTER_TRUEVIEW_CATEGORY")]
    FILTERTRUEVIEWCATEGORY,
    
    /// "FILTER_TRUEVIEW_KEYWORD"
    #[serde(rename="FILTER_TRUEVIEW_KEYWORD")]
    FILTERTRUEVIEWKEYWORD,
    
    /// "FILTER_TRUEVIEW_PLACEMENT"
    #[serde(rename="FILTER_TRUEVIEW_PLACEMENT")]
    FILTERTRUEVIEWPLACEMENT,
    
    /// "FILTER_TRUEVIEW_URL"
    #[serde(rename="FILTER_TRUEVIEW_URL")]
    FILTERTRUEVIEWURL,
    
    /// "FILTER_TRUEVIEW_COUNTRY"
    #[serde(rename="FILTER_TRUEVIEW_COUNTRY")]
    FILTERTRUEVIEWCOUNTRY,
    
    /// "FILTER_TRUEVIEW_REGION"
    #[serde(rename="FILTER_TRUEVIEW_REGION")]
    FILTERTRUEVIEWREGION,
    
    /// "FILTER_TRUEVIEW_CITY"
    #[serde(rename="FILTER_TRUEVIEW_CITY")]
    FILTERTRUEVIEWCITY,
    
    /// "FILTER_TRUEVIEW_DMA"
    #[serde(rename="FILTER_TRUEVIEW_DMA")]
    FILTERTRUEVIEWDMA,
    
    /// "FILTER_TRUEVIEW_ZIPCODE"
    #[serde(rename="FILTER_TRUEVIEW_ZIPCODE")]
    FILTERTRUEVIEWZIPCODE,
    
    /// "FILTER_NOT_SUPPORTED"
    #[serde(rename="FILTER_NOT_SUPPORTED")]
    FILTERNOTSUPPORTED,
    
    /// "FILTER_MEDIA_PLAN"
    #[serde(rename="FILTER_MEDIA_PLAN")]
    FILTERMEDIAPLAN,
    
    /// "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL"
    #[serde(rename="FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL")]
    FILTERTRUEVIEWIARYOUTUBECHANNEL,
    
    /// "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO"
    #[serde(rename="FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO")]
    FILTERTRUEVIEWIARYOUTUBEVIDEO,
    
    /// "FILTER_SKIPPABLE_SUPPORT"
    #[serde(rename="FILTER_SKIPPABLE_SUPPORT")]
    FILTERSKIPPABLESUPPORT,
    
    /// "FILTER_COMPANION_CREATIVE_ID"
    #[serde(rename="FILTER_COMPANION_CREATIVE_ID")]
    FILTERCOMPANIONCREATIVEID,
    
    /// "FILTER_BUDGET_SEGMENT_DESCRIPTION"
    #[serde(rename="FILTER_BUDGET_SEGMENT_DESCRIPTION")]
    FILTERBUDGETSEGMENTDESCRIPTION,
    
    /// "FILTER_FLOODLIGHT_ACTIVITY_ID"
    #[serde(rename="FILTER_FLOODLIGHT_ACTIVITY_ID")]
    FILTERFLOODLIGHTACTIVITYID,
    
    /// "FILTER_DEVICE_MODEL"
    #[serde(rename="FILTER_DEVICE_MODEL")]
    FILTERDEVICEMODEL,
    
    /// "FILTER_DEVICE_MAKE"
    #[serde(rename="FILTER_DEVICE_MAKE")]
    FILTERDEVICEMAKE,
    
    /// "FILTER_DEVICE_TYPE"
    #[serde(rename="FILTER_DEVICE_TYPE")]
    FILTERDEVICETYPE,
    
    /// "FILTER_CREATIVE_ATTRIBUTE"
    #[serde(rename="FILTER_CREATIVE_ATTRIBUTE")]
    FILTERCREATIVEATTRIBUTE,
    
    /// "FILTER_INVENTORY_COMMITMENT_TYPE"
    #[serde(rename="FILTER_INVENTORY_COMMITMENT_TYPE")]
    FILTERINVENTORYCOMMITMENTTYPE,
    
    /// "FILTER_INVENTORY_RATE_TYPE"
    #[serde(rename="FILTER_INVENTORY_RATE_TYPE")]
    FILTERINVENTORYRATETYPE,
    
    /// "FILTER_INVENTORY_DELIVERY_METHOD"
    #[serde(rename="FILTER_INVENTORY_DELIVERY_METHOD")]
    FILTERINVENTORYDELIVERYMETHOD,
    
    /// "FILTER_INVENTORY_SOURCE_EXTERNAL_ID"
    #[serde(rename="FILTER_INVENTORY_SOURCE_EXTERNAL_ID")]
    FILTERINVENTORYSOURCEEXTERNALID,
    
    /// "FILTER_AUTHORIZED_SELLER_STATE"
    #[serde(rename="FILTER_AUTHORIZED_SELLER_STATE")]
    FILTERAUTHORIZEDSELLERSTATE,
    
    /// "FILTER_VIDEO_DURATION_SECONDS_RANGE"
    #[serde(rename="FILTER_VIDEO_DURATION_SECONDS_RANGE")]
    FILTERVIDEODURATIONSECONDSRANGE,
}

impl AsRef<str> for FilterPairTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FilterPairTypeEnum::FILTERUNKNOWN => "FILTER_UNKNOWN",
            FilterPairTypeEnum::FILTERDATE => "FILTER_DATE",
            FilterPairTypeEnum::FILTERDAYOFWEEK => "FILTER_DAY_OF_WEEK",
            FilterPairTypeEnum::FILTERWEEK => "FILTER_WEEK",
            FilterPairTypeEnum::FILTERMONTH => "FILTER_MONTH",
            FilterPairTypeEnum::FILTERYEAR => "FILTER_YEAR",
            FilterPairTypeEnum::FILTERTIMEOFDAY => "FILTER_TIME_OF_DAY",
            FilterPairTypeEnum::FILTERCONVERSIONDELAY => "FILTER_CONVERSION_DELAY",
            FilterPairTypeEnum::FILTERCREATIVEID => "FILTER_CREATIVE_ID",
            FilterPairTypeEnum::FILTERCREATIVESIZE => "FILTER_CREATIVE_SIZE",
            FilterPairTypeEnum::FILTERCREATIVETYPE => "FILTER_CREATIVE_TYPE",
            FilterPairTypeEnum::FILTEREXCHANGEID => "FILTER_EXCHANGE_ID",
            FilterPairTypeEnum::FILTERADPOSITION => "FILTER_AD_POSITION",
            FilterPairTypeEnum::FILTERINVENTORYSOURCE => "FILTER_INVENTORY_SOURCE",
            FilterPairTypeEnum::FILTERCITY => "FILTER_CITY",
            FilterPairTypeEnum::FILTERREGION => "FILTER_REGION",
            FilterPairTypeEnum::FILTERDMA => "FILTER_DMA",
            FilterPairTypeEnum::FILTERCOUNTRY => "FILTER_COUNTRY",
            FilterPairTypeEnum::FILTERSITEID => "FILTER_SITE_ID",
            FilterPairTypeEnum::FILTERCHANNELID => "FILTER_CHANNEL_ID",
            FilterPairTypeEnum::FILTERPARTNER => "FILTER_PARTNER",
            FilterPairTypeEnum::FILTERADVERTISER => "FILTER_ADVERTISER",
            FilterPairTypeEnum::FILTERINSERTIONORDER => "FILTER_INSERTION_ORDER",
            FilterPairTypeEnum::FILTERLINEITEM => "FILTER_LINE_ITEM",
            FilterPairTypeEnum::FILTERPARTNERCURRENCY => "FILTER_PARTNER_CURRENCY",
            FilterPairTypeEnum::FILTERADVERTISERCURRENCY => "FILTER_ADVERTISER_CURRENCY",
            FilterPairTypeEnum::FILTERADVERTISERTIMEZONE => "FILTER_ADVERTISER_TIMEZONE",
            FilterPairTypeEnum::FILTERLINEITEMTYPE => "FILTER_LINE_ITEM_TYPE",
            FilterPairTypeEnum::FILTERUSERLIST => "FILTER_USER_LIST",
            FilterPairTypeEnum::FILTERUSERLISTFIRSTPARTY => "FILTER_USER_LIST_FIRST_PARTY",
            FilterPairTypeEnum::FILTERUSERLISTTHIRDPARTY => "FILTER_USER_LIST_THIRD_PARTY",
            FilterPairTypeEnum::FILTERTARGETEDUSERLIST => "FILTER_TARGETED_USER_LIST",
            FilterPairTypeEnum::FILTERDATAPROVIDER => "FILTER_DATA_PROVIDER",
            FilterPairTypeEnum::FILTERORDERID => "FILTER_ORDER_ID",
            FilterPairTypeEnum::FILTERVIDEOPLAYERSIZE => "FILTER_VIDEO_PLAYER_SIZE",
            FilterPairTypeEnum::FILTERVIDEODURATIONSECONDS => "FILTER_VIDEO_DURATION_SECONDS",
            FilterPairTypeEnum::FILTERKEYWORD => "FILTER_KEYWORD",
            FilterPairTypeEnum::FILTERPAGECATEGORY => "FILTER_PAGE_CATEGORY",
            FilterPairTypeEnum::FILTERCAMPAIGNDAILYFREQUENCY => "FILTER_CAMPAIGN_DAILY_FREQUENCY",
            FilterPairTypeEnum::FILTERLINEITEMDAILYFREQUENCY => "FILTER_LINE_ITEM_DAILY_FREQUENCY",
            FilterPairTypeEnum::FILTERLINEITEMLIFETIMEFREQUENCY => "FILTER_LINE_ITEM_LIFETIME_FREQUENCY",
            FilterPairTypeEnum::FILTEROS => "FILTER_OS",
            FilterPairTypeEnum::FILTERBROWSER => "FILTER_BROWSER",
            FilterPairTypeEnum::FILTERCARRIER => "FILTER_CARRIER",
            FilterPairTypeEnum::FILTERSITELANGUAGE => "FILTER_SITE_LANGUAGE",
            FilterPairTypeEnum::FILTERINVENTORYFORMAT => "FILTER_INVENTORY_FORMAT",
            FilterPairTypeEnum::FILTERZIPCODE => "FILTER_ZIP_CODE",
            FilterPairTypeEnum::FILTERVIDEORATINGTIER => "FILTER_VIDEO_RATING_TIER",
            FilterPairTypeEnum::FILTERVIDEOFORMATSUPPORT => "FILTER_VIDEO_FORMAT_SUPPORT",
            FilterPairTypeEnum::FILTERVIDEOSKIPPABLESUPPORT => "FILTER_VIDEO_SKIPPABLE_SUPPORT",
            FilterPairTypeEnum::FILTERVIDEOVPAIDSUPPORT => "FILTER_VIDEO_VPAID_SUPPORT",
            FilterPairTypeEnum::FILTERVIDEOCREATIVEDURATION => "FILTER_VIDEO_CREATIVE_DURATION",
            FilterPairTypeEnum::FILTERPAGELAYOUT => "FILTER_PAGE_LAYOUT",
            FilterPairTypeEnum::FILTERVIDEOADPOSITIONINSTREAM => "FILTER_VIDEO_AD_POSITION_IN_STREAM",
            FilterPairTypeEnum::FILTERAGE => "FILTER_AGE",
            FilterPairTypeEnum::FILTERGENDER => "FILTER_GENDER",
            FilterPairTypeEnum::FILTERQUARTER => "FILTER_QUARTER",
            FilterPairTypeEnum::FILTERTRUEVIEWCONVERSIONTYPE => "FILTER_TRUEVIEW_CONVERSION_TYPE",
            FilterPairTypeEnum::FILTERMOBILEGEO => "FILTER_MOBILE_GEO",
            FilterPairTypeEnum::FILTERMRAIDSUPPORT => "FILTER_MRAID_SUPPORT",
            FilterPairTypeEnum::FILTERACTIVEVIEWEXPECTEDVIEWABILITY => "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY",
            FilterPairTypeEnum::FILTERVIDEOCREATIVEDURATIONSKIPPABLE => "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE",
            FilterPairTypeEnum::FILTERNIELSENCOUNTRYCODE => "FILTER_NIELSEN_COUNTRY_CODE",
            FilterPairTypeEnum::FILTERNIELSENDEVICEID => "FILTER_NIELSEN_DEVICE_ID",
            FilterPairTypeEnum::FILTERNIELSENGENDER => "FILTER_NIELSEN_GENDER",
            FilterPairTypeEnum::FILTERNIELSENAGE => "FILTER_NIELSEN_AGE",
            FilterPairTypeEnum::FILTERINVENTORYSOURCETYPE => "FILTER_INVENTORY_SOURCE_TYPE",
            FilterPairTypeEnum::FILTERCREATIVEWIDTH => "FILTER_CREATIVE_WIDTH",
            FilterPairTypeEnum::FILTERCREATIVEHEIGHT => "FILTER_CREATIVE_HEIGHT",
            FilterPairTypeEnum::FILTERDFPORDERID => "FILTER_DFP_ORDER_ID",
            FilterPairTypeEnum::FILTERTRUEVIEWAGE => "FILTER_TRUEVIEW_AGE",
            FilterPairTypeEnum::FILTERTRUEVIEWGENDER => "FILTER_TRUEVIEW_GENDER",
            FilterPairTypeEnum::FILTERTRUEVIEWPARENTALSTATUS => "FILTER_TRUEVIEW_PARENTAL_STATUS",
            FilterPairTypeEnum::FILTERTRUEVIEWREMARKETINGLIST => "FILTER_TRUEVIEW_REMARKETING_LIST",
            FilterPairTypeEnum::FILTERTRUEVIEWINTEREST => "FILTER_TRUEVIEW_INTEREST",
            FilterPairTypeEnum::FILTERTRUEVIEWADGROUPID => "FILTER_TRUEVIEW_AD_GROUP_ID",
            FilterPairTypeEnum::FILTERTRUEVIEWADGROUPADID => "FILTER_TRUEVIEW_AD_GROUP_AD_ID",
            FilterPairTypeEnum::FILTERTRUEVIEWIARLANGUAGE => "FILTER_TRUEVIEW_IAR_LANGUAGE",
            FilterPairTypeEnum::FILTERTRUEVIEWIARGENDER => "FILTER_TRUEVIEW_IAR_GENDER",
            FilterPairTypeEnum::FILTERTRUEVIEWIARAGE => "FILTER_TRUEVIEW_IAR_AGE",
            FilterPairTypeEnum::FILTERTRUEVIEWIARCATEGORY => "FILTER_TRUEVIEW_IAR_CATEGORY",
            FilterPairTypeEnum::FILTERTRUEVIEWIARCOUNTRY => "FILTER_TRUEVIEW_IAR_COUNTRY",
            FilterPairTypeEnum::FILTERTRUEVIEWIARCITY => "FILTER_TRUEVIEW_IAR_CITY",
            FilterPairTypeEnum::FILTERTRUEVIEWIARREGION => "FILTER_TRUEVIEW_IAR_REGION",
            FilterPairTypeEnum::FILTERTRUEVIEWIARZIPCODE => "FILTER_TRUEVIEW_IAR_ZIPCODE",
            FilterPairTypeEnum::FILTERTRUEVIEWIARREMARKETINGLIST => "FILTER_TRUEVIEW_IAR_REMARKETING_LIST",
            FilterPairTypeEnum::FILTERTRUEVIEWIARINTEREST => "FILTER_TRUEVIEW_IAR_INTEREST",
            FilterPairTypeEnum::FILTERTRUEVIEWIARPARENTALSTATUS => "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS",
            FilterPairTypeEnum::FILTERTRUEVIEWIARTIMEOFDAY => "FILTER_TRUEVIEW_IAR_TIME_OF_DAY",
            FilterPairTypeEnum::FILTERTRUEVIEWCUSTOMAFFINITY => "FILTER_TRUEVIEW_CUSTOM_AFFINITY",
            FilterPairTypeEnum::FILTERTRUEVIEWCATEGORY => "FILTER_TRUEVIEW_CATEGORY",
            FilterPairTypeEnum::FILTERTRUEVIEWKEYWORD => "FILTER_TRUEVIEW_KEYWORD",
            FilterPairTypeEnum::FILTERTRUEVIEWPLACEMENT => "FILTER_TRUEVIEW_PLACEMENT",
            FilterPairTypeEnum::FILTERTRUEVIEWURL => "FILTER_TRUEVIEW_URL",
            FilterPairTypeEnum::FILTERTRUEVIEWCOUNTRY => "FILTER_TRUEVIEW_COUNTRY",
            FilterPairTypeEnum::FILTERTRUEVIEWREGION => "FILTER_TRUEVIEW_REGION",
            FilterPairTypeEnum::FILTERTRUEVIEWCITY => "FILTER_TRUEVIEW_CITY",
            FilterPairTypeEnum::FILTERTRUEVIEWDMA => "FILTER_TRUEVIEW_DMA",
            FilterPairTypeEnum::FILTERTRUEVIEWZIPCODE => "FILTER_TRUEVIEW_ZIPCODE",
            FilterPairTypeEnum::FILTERNOTSUPPORTED => "FILTER_NOT_SUPPORTED",
            FilterPairTypeEnum::FILTERMEDIAPLAN => "FILTER_MEDIA_PLAN",
            FilterPairTypeEnum::FILTERTRUEVIEWIARYOUTUBECHANNEL => "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL",
            FilterPairTypeEnum::FILTERTRUEVIEWIARYOUTUBEVIDEO => "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO",
            FilterPairTypeEnum::FILTERSKIPPABLESUPPORT => "FILTER_SKIPPABLE_SUPPORT",
            FilterPairTypeEnum::FILTERCOMPANIONCREATIVEID => "FILTER_COMPANION_CREATIVE_ID",
            FilterPairTypeEnum::FILTERBUDGETSEGMENTDESCRIPTION => "FILTER_BUDGET_SEGMENT_DESCRIPTION",
            FilterPairTypeEnum::FILTERFLOODLIGHTACTIVITYID => "FILTER_FLOODLIGHT_ACTIVITY_ID",
            FilterPairTypeEnum::FILTERDEVICEMODEL => "FILTER_DEVICE_MODEL",
            FilterPairTypeEnum::FILTERDEVICEMAKE => "FILTER_DEVICE_MAKE",
            FilterPairTypeEnum::FILTERDEVICETYPE => "FILTER_DEVICE_TYPE",
            FilterPairTypeEnum::FILTERCREATIVEATTRIBUTE => "FILTER_CREATIVE_ATTRIBUTE",
            FilterPairTypeEnum::FILTERINVENTORYCOMMITMENTTYPE => "FILTER_INVENTORY_COMMITMENT_TYPE",
            FilterPairTypeEnum::FILTERINVENTORYRATETYPE => "FILTER_INVENTORY_RATE_TYPE",
            FilterPairTypeEnum::FILTERINVENTORYDELIVERYMETHOD => "FILTER_INVENTORY_DELIVERY_METHOD",
            FilterPairTypeEnum::FILTERINVENTORYSOURCEEXTERNALID => "FILTER_INVENTORY_SOURCE_EXTERNAL_ID",
            FilterPairTypeEnum::FILTERAUTHORIZEDSELLERSTATE => "FILTER_AUTHORIZED_SELLER_STATE",
            FilterPairTypeEnum::FILTERVIDEODURATIONSECONDSRANGE => "FILTER_VIDEO_DURATION_SECONDS_RANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for FilterPairTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FILTER_UNKNOWN" => Ok(FilterPairTypeEnum::FILTERUNKNOWN),
           "FILTER_DATE" => Ok(FilterPairTypeEnum::FILTERDATE),
           "FILTER_DAY_OF_WEEK" => Ok(FilterPairTypeEnum::FILTERDAYOFWEEK),
           "FILTER_WEEK" => Ok(FilterPairTypeEnum::FILTERWEEK),
           "FILTER_MONTH" => Ok(FilterPairTypeEnum::FILTERMONTH),
           "FILTER_YEAR" => Ok(FilterPairTypeEnum::FILTERYEAR),
           "FILTER_TIME_OF_DAY" => Ok(FilterPairTypeEnum::FILTERTIMEOFDAY),
           "FILTER_CONVERSION_DELAY" => Ok(FilterPairTypeEnum::FILTERCONVERSIONDELAY),
           "FILTER_CREATIVE_ID" => Ok(FilterPairTypeEnum::FILTERCREATIVEID),
           "FILTER_CREATIVE_SIZE" => Ok(FilterPairTypeEnum::FILTERCREATIVESIZE),
           "FILTER_CREATIVE_TYPE" => Ok(FilterPairTypeEnum::FILTERCREATIVETYPE),
           "FILTER_EXCHANGE_ID" => Ok(FilterPairTypeEnum::FILTEREXCHANGEID),
           "FILTER_AD_POSITION" => Ok(FilterPairTypeEnum::FILTERADPOSITION),
           "FILTER_INVENTORY_SOURCE" => Ok(FilterPairTypeEnum::FILTERINVENTORYSOURCE),
           "FILTER_CITY" => Ok(FilterPairTypeEnum::FILTERCITY),
           "FILTER_REGION" => Ok(FilterPairTypeEnum::FILTERREGION),
           "FILTER_DMA" => Ok(FilterPairTypeEnum::FILTERDMA),
           "FILTER_COUNTRY" => Ok(FilterPairTypeEnum::FILTERCOUNTRY),
           "FILTER_SITE_ID" => Ok(FilterPairTypeEnum::FILTERSITEID),
           "FILTER_CHANNEL_ID" => Ok(FilterPairTypeEnum::FILTERCHANNELID),
           "FILTER_PARTNER" => Ok(FilterPairTypeEnum::FILTERPARTNER),
           "FILTER_ADVERTISER" => Ok(FilterPairTypeEnum::FILTERADVERTISER),
           "FILTER_INSERTION_ORDER" => Ok(FilterPairTypeEnum::FILTERINSERTIONORDER),
           "FILTER_LINE_ITEM" => Ok(FilterPairTypeEnum::FILTERLINEITEM),
           "FILTER_PARTNER_CURRENCY" => Ok(FilterPairTypeEnum::FILTERPARTNERCURRENCY),
           "FILTER_ADVERTISER_CURRENCY" => Ok(FilterPairTypeEnum::FILTERADVERTISERCURRENCY),
           "FILTER_ADVERTISER_TIMEZONE" => Ok(FilterPairTypeEnum::FILTERADVERTISERTIMEZONE),
           "FILTER_LINE_ITEM_TYPE" => Ok(FilterPairTypeEnum::FILTERLINEITEMTYPE),
           "FILTER_USER_LIST" => Ok(FilterPairTypeEnum::FILTERUSERLIST),
           "FILTER_USER_LIST_FIRST_PARTY" => Ok(FilterPairTypeEnum::FILTERUSERLISTFIRSTPARTY),
           "FILTER_USER_LIST_THIRD_PARTY" => Ok(FilterPairTypeEnum::FILTERUSERLISTTHIRDPARTY),
           "FILTER_TARGETED_USER_LIST" => Ok(FilterPairTypeEnum::FILTERTARGETEDUSERLIST),
           "FILTER_DATA_PROVIDER" => Ok(FilterPairTypeEnum::FILTERDATAPROVIDER),
           "FILTER_ORDER_ID" => Ok(FilterPairTypeEnum::FILTERORDERID),
           "FILTER_VIDEO_PLAYER_SIZE" => Ok(FilterPairTypeEnum::FILTERVIDEOPLAYERSIZE),
           "FILTER_VIDEO_DURATION_SECONDS" => Ok(FilterPairTypeEnum::FILTERVIDEODURATIONSECONDS),
           "FILTER_KEYWORD" => Ok(FilterPairTypeEnum::FILTERKEYWORD),
           "FILTER_PAGE_CATEGORY" => Ok(FilterPairTypeEnum::FILTERPAGECATEGORY),
           "FILTER_CAMPAIGN_DAILY_FREQUENCY" => Ok(FilterPairTypeEnum::FILTERCAMPAIGNDAILYFREQUENCY),
           "FILTER_LINE_ITEM_DAILY_FREQUENCY" => Ok(FilterPairTypeEnum::FILTERLINEITEMDAILYFREQUENCY),
           "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" => Ok(FilterPairTypeEnum::FILTERLINEITEMLIFETIMEFREQUENCY),
           "FILTER_OS" => Ok(FilterPairTypeEnum::FILTEROS),
           "FILTER_BROWSER" => Ok(FilterPairTypeEnum::FILTERBROWSER),
           "FILTER_CARRIER" => Ok(FilterPairTypeEnum::FILTERCARRIER),
           "FILTER_SITE_LANGUAGE" => Ok(FilterPairTypeEnum::FILTERSITELANGUAGE),
           "FILTER_INVENTORY_FORMAT" => Ok(FilterPairTypeEnum::FILTERINVENTORYFORMAT),
           "FILTER_ZIP_CODE" => Ok(FilterPairTypeEnum::FILTERZIPCODE),
           "FILTER_VIDEO_RATING_TIER" => Ok(FilterPairTypeEnum::FILTERVIDEORATINGTIER),
           "FILTER_VIDEO_FORMAT_SUPPORT" => Ok(FilterPairTypeEnum::FILTERVIDEOFORMATSUPPORT),
           "FILTER_VIDEO_SKIPPABLE_SUPPORT" => Ok(FilterPairTypeEnum::FILTERVIDEOSKIPPABLESUPPORT),
           "FILTER_VIDEO_VPAID_SUPPORT" => Ok(FilterPairTypeEnum::FILTERVIDEOVPAIDSUPPORT),
           "FILTER_VIDEO_CREATIVE_DURATION" => Ok(FilterPairTypeEnum::FILTERVIDEOCREATIVEDURATION),
           "FILTER_PAGE_LAYOUT" => Ok(FilterPairTypeEnum::FILTERPAGELAYOUT),
           "FILTER_VIDEO_AD_POSITION_IN_STREAM" => Ok(FilterPairTypeEnum::FILTERVIDEOADPOSITIONINSTREAM),
           "FILTER_AGE" => Ok(FilterPairTypeEnum::FILTERAGE),
           "FILTER_GENDER" => Ok(FilterPairTypeEnum::FILTERGENDER),
           "FILTER_QUARTER" => Ok(FilterPairTypeEnum::FILTERQUARTER),
           "FILTER_TRUEVIEW_CONVERSION_TYPE" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWCONVERSIONTYPE),
           "FILTER_MOBILE_GEO" => Ok(FilterPairTypeEnum::FILTERMOBILEGEO),
           "FILTER_MRAID_SUPPORT" => Ok(FilterPairTypeEnum::FILTERMRAIDSUPPORT),
           "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" => Ok(FilterPairTypeEnum::FILTERACTIVEVIEWEXPECTEDVIEWABILITY),
           "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" => Ok(FilterPairTypeEnum::FILTERVIDEOCREATIVEDURATIONSKIPPABLE),
           "FILTER_NIELSEN_COUNTRY_CODE" => Ok(FilterPairTypeEnum::FILTERNIELSENCOUNTRYCODE),
           "FILTER_NIELSEN_DEVICE_ID" => Ok(FilterPairTypeEnum::FILTERNIELSENDEVICEID),
           "FILTER_NIELSEN_GENDER" => Ok(FilterPairTypeEnum::FILTERNIELSENGENDER),
           "FILTER_NIELSEN_AGE" => Ok(FilterPairTypeEnum::FILTERNIELSENAGE),
           "FILTER_INVENTORY_SOURCE_TYPE" => Ok(FilterPairTypeEnum::FILTERINVENTORYSOURCETYPE),
           "FILTER_CREATIVE_WIDTH" => Ok(FilterPairTypeEnum::FILTERCREATIVEWIDTH),
           "FILTER_CREATIVE_HEIGHT" => Ok(FilterPairTypeEnum::FILTERCREATIVEHEIGHT),
           "FILTER_DFP_ORDER_ID" => Ok(FilterPairTypeEnum::FILTERDFPORDERID),
           "FILTER_TRUEVIEW_AGE" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWAGE),
           "FILTER_TRUEVIEW_GENDER" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWGENDER),
           "FILTER_TRUEVIEW_PARENTAL_STATUS" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWPARENTALSTATUS),
           "FILTER_TRUEVIEW_REMARKETING_LIST" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWREMARKETINGLIST),
           "FILTER_TRUEVIEW_INTEREST" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWINTEREST),
           "FILTER_TRUEVIEW_AD_GROUP_ID" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWADGROUPID),
           "FILTER_TRUEVIEW_AD_GROUP_AD_ID" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWADGROUPADID),
           "FILTER_TRUEVIEW_IAR_LANGUAGE" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARLANGUAGE),
           "FILTER_TRUEVIEW_IAR_GENDER" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARGENDER),
           "FILTER_TRUEVIEW_IAR_AGE" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARAGE),
           "FILTER_TRUEVIEW_IAR_CATEGORY" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARCATEGORY),
           "FILTER_TRUEVIEW_IAR_COUNTRY" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARCOUNTRY),
           "FILTER_TRUEVIEW_IAR_CITY" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARCITY),
           "FILTER_TRUEVIEW_IAR_REGION" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARREGION),
           "FILTER_TRUEVIEW_IAR_ZIPCODE" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARZIPCODE),
           "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARREMARKETINGLIST),
           "FILTER_TRUEVIEW_IAR_INTEREST" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARINTEREST),
           "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARPARENTALSTATUS),
           "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARTIMEOFDAY),
           "FILTER_TRUEVIEW_CUSTOM_AFFINITY" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWCUSTOMAFFINITY),
           "FILTER_TRUEVIEW_CATEGORY" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWCATEGORY),
           "FILTER_TRUEVIEW_KEYWORD" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWKEYWORD),
           "FILTER_TRUEVIEW_PLACEMENT" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWPLACEMENT),
           "FILTER_TRUEVIEW_URL" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWURL),
           "FILTER_TRUEVIEW_COUNTRY" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWCOUNTRY),
           "FILTER_TRUEVIEW_REGION" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWREGION),
           "FILTER_TRUEVIEW_CITY" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWCITY),
           "FILTER_TRUEVIEW_DMA" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWDMA),
           "FILTER_TRUEVIEW_ZIPCODE" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWZIPCODE),
           "FILTER_NOT_SUPPORTED" => Ok(FilterPairTypeEnum::FILTERNOTSUPPORTED),
           "FILTER_MEDIA_PLAN" => Ok(FilterPairTypeEnum::FILTERMEDIAPLAN),
           "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARYOUTUBECHANNEL),
           "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" => Ok(FilterPairTypeEnum::FILTERTRUEVIEWIARYOUTUBEVIDEO),
           "FILTER_SKIPPABLE_SUPPORT" => Ok(FilterPairTypeEnum::FILTERSKIPPABLESUPPORT),
           "FILTER_COMPANION_CREATIVE_ID" => Ok(FilterPairTypeEnum::FILTERCOMPANIONCREATIVEID),
           "FILTER_BUDGET_SEGMENT_DESCRIPTION" => Ok(FilterPairTypeEnum::FILTERBUDGETSEGMENTDESCRIPTION),
           "FILTER_FLOODLIGHT_ACTIVITY_ID" => Ok(FilterPairTypeEnum::FILTERFLOODLIGHTACTIVITYID),
           "FILTER_DEVICE_MODEL" => Ok(FilterPairTypeEnum::FILTERDEVICEMODEL),
           "FILTER_DEVICE_MAKE" => Ok(FilterPairTypeEnum::FILTERDEVICEMAKE),
           "FILTER_DEVICE_TYPE" => Ok(FilterPairTypeEnum::FILTERDEVICETYPE),
           "FILTER_CREATIVE_ATTRIBUTE" => Ok(FilterPairTypeEnum::FILTERCREATIVEATTRIBUTE),
           "FILTER_INVENTORY_COMMITMENT_TYPE" => Ok(FilterPairTypeEnum::FILTERINVENTORYCOMMITMENTTYPE),
           "FILTER_INVENTORY_RATE_TYPE" => Ok(FilterPairTypeEnum::FILTERINVENTORYRATETYPE),
           "FILTER_INVENTORY_DELIVERY_METHOD" => Ok(FilterPairTypeEnum::FILTERINVENTORYDELIVERYMETHOD),
           "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" => Ok(FilterPairTypeEnum::FILTERINVENTORYSOURCEEXTERNALID),
           "FILTER_AUTHORIZED_SELLER_STATE" => Ok(FilterPairTypeEnum::FILTERAUTHORIZEDSELLERSTATE),
           "FILTER_VIDEO_DURATION_SECONDS_RANGE" => Ok(FilterPairTypeEnum::FILTERVIDEODURATIONSECONDSRANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FilterPairTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParameterGroupBysEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Data is grouped by the filters listed in this field.
pub enum ParameterGroupBysEnum {
    
    /// "FILTER_UNKNOWN"
    #[serde(rename="FILTER_UNKNOWN")]
    FILTERUNKNOWN,
    
    /// "FILTER_DATE"
    #[serde(rename="FILTER_DATE")]
    FILTERDATE,
    
    /// "FILTER_DAY_OF_WEEK"
    #[serde(rename="FILTER_DAY_OF_WEEK")]
    FILTERDAYOFWEEK,
    
    /// "FILTER_WEEK"
    #[serde(rename="FILTER_WEEK")]
    FILTERWEEK,
    
    /// "FILTER_MONTH"
    #[serde(rename="FILTER_MONTH")]
    FILTERMONTH,
    
    /// "FILTER_YEAR"
    #[serde(rename="FILTER_YEAR")]
    FILTERYEAR,
    
    /// "FILTER_TIME_OF_DAY"
    #[serde(rename="FILTER_TIME_OF_DAY")]
    FILTERTIMEOFDAY,
    
    /// "FILTER_CONVERSION_DELAY"
    #[serde(rename="FILTER_CONVERSION_DELAY")]
    FILTERCONVERSIONDELAY,
    
    /// "FILTER_CREATIVE_ID"
    #[serde(rename="FILTER_CREATIVE_ID")]
    FILTERCREATIVEID,
    
    /// "FILTER_CREATIVE_SIZE"
    #[serde(rename="FILTER_CREATIVE_SIZE")]
    FILTERCREATIVESIZE,
    
    /// "FILTER_CREATIVE_TYPE"
    #[serde(rename="FILTER_CREATIVE_TYPE")]
    FILTERCREATIVETYPE,
    
    /// "FILTER_EXCHANGE_ID"
    #[serde(rename="FILTER_EXCHANGE_ID")]
    FILTEREXCHANGEID,
    
    /// "FILTER_AD_POSITION"
    #[serde(rename="FILTER_AD_POSITION")]
    FILTERADPOSITION,
    
    /// "FILTER_INVENTORY_SOURCE"
    #[serde(rename="FILTER_INVENTORY_SOURCE")]
    FILTERINVENTORYSOURCE,
    
    /// "FILTER_CITY"
    #[serde(rename="FILTER_CITY")]
    FILTERCITY,
    
    /// "FILTER_REGION"
    #[serde(rename="FILTER_REGION")]
    FILTERREGION,
    
    /// "FILTER_DMA"
    #[serde(rename="FILTER_DMA")]
    FILTERDMA,
    
    /// "FILTER_COUNTRY"
    #[serde(rename="FILTER_COUNTRY")]
    FILTERCOUNTRY,
    
    /// "FILTER_SITE_ID"
    #[serde(rename="FILTER_SITE_ID")]
    FILTERSITEID,
    
    /// "FILTER_CHANNEL_ID"
    #[serde(rename="FILTER_CHANNEL_ID")]
    FILTERCHANNELID,
    
    /// "FILTER_PARTNER"
    #[serde(rename="FILTER_PARTNER")]
    FILTERPARTNER,
    
    /// "FILTER_ADVERTISER"
    #[serde(rename="FILTER_ADVERTISER")]
    FILTERADVERTISER,
    
    /// "FILTER_INSERTION_ORDER"
    #[serde(rename="FILTER_INSERTION_ORDER")]
    FILTERINSERTIONORDER,
    
    /// "FILTER_LINE_ITEM"
    #[serde(rename="FILTER_LINE_ITEM")]
    FILTERLINEITEM,
    
    /// "FILTER_PARTNER_CURRENCY"
    #[serde(rename="FILTER_PARTNER_CURRENCY")]
    FILTERPARTNERCURRENCY,
    
    /// "FILTER_ADVERTISER_CURRENCY"
    #[serde(rename="FILTER_ADVERTISER_CURRENCY")]
    FILTERADVERTISERCURRENCY,
    
    /// "FILTER_ADVERTISER_TIMEZONE"
    #[serde(rename="FILTER_ADVERTISER_TIMEZONE")]
    FILTERADVERTISERTIMEZONE,
    
    /// "FILTER_LINE_ITEM_TYPE"
    #[serde(rename="FILTER_LINE_ITEM_TYPE")]
    FILTERLINEITEMTYPE,
    
    /// "FILTER_USER_LIST"
    #[serde(rename="FILTER_USER_LIST")]
    FILTERUSERLIST,
    
    /// "FILTER_USER_LIST_FIRST_PARTY"
    #[serde(rename="FILTER_USER_LIST_FIRST_PARTY")]
    FILTERUSERLISTFIRSTPARTY,
    
    /// "FILTER_USER_LIST_THIRD_PARTY"
    #[serde(rename="FILTER_USER_LIST_THIRD_PARTY")]
    FILTERUSERLISTTHIRDPARTY,
    
    /// "FILTER_TARGETED_USER_LIST"
    #[serde(rename="FILTER_TARGETED_USER_LIST")]
    FILTERTARGETEDUSERLIST,
    
    /// "FILTER_DATA_PROVIDER"
    #[serde(rename="FILTER_DATA_PROVIDER")]
    FILTERDATAPROVIDER,
    
    /// "FILTER_ORDER_ID"
    #[serde(rename="FILTER_ORDER_ID")]
    FILTERORDERID,
    
    /// "FILTER_VIDEO_PLAYER_SIZE"
    #[serde(rename="FILTER_VIDEO_PLAYER_SIZE")]
    FILTERVIDEOPLAYERSIZE,
    
    /// "FILTER_VIDEO_DURATION_SECONDS"
    #[serde(rename="FILTER_VIDEO_DURATION_SECONDS")]
    FILTERVIDEODURATIONSECONDS,
    
    /// "FILTER_KEYWORD"
    #[serde(rename="FILTER_KEYWORD")]
    FILTERKEYWORD,
    
    /// "FILTER_PAGE_CATEGORY"
    #[serde(rename="FILTER_PAGE_CATEGORY")]
    FILTERPAGECATEGORY,
    
    /// "FILTER_CAMPAIGN_DAILY_FREQUENCY"
    #[serde(rename="FILTER_CAMPAIGN_DAILY_FREQUENCY")]
    FILTERCAMPAIGNDAILYFREQUENCY,
    
    /// "FILTER_LINE_ITEM_DAILY_FREQUENCY"
    #[serde(rename="FILTER_LINE_ITEM_DAILY_FREQUENCY")]
    FILTERLINEITEMDAILYFREQUENCY,
    
    /// "FILTER_LINE_ITEM_LIFETIME_FREQUENCY"
    #[serde(rename="FILTER_LINE_ITEM_LIFETIME_FREQUENCY")]
    FILTERLINEITEMLIFETIMEFREQUENCY,
    
    /// "FILTER_OS"
    #[serde(rename="FILTER_OS")]
    FILTEROS,
    
    /// "FILTER_BROWSER"
    #[serde(rename="FILTER_BROWSER")]
    FILTERBROWSER,
    
    /// "FILTER_CARRIER"
    #[serde(rename="FILTER_CARRIER")]
    FILTERCARRIER,
    
    /// "FILTER_SITE_LANGUAGE"
    #[serde(rename="FILTER_SITE_LANGUAGE")]
    FILTERSITELANGUAGE,
    
    /// "FILTER_INVENTORY_FORMAT"
    #[serde(rename="FILTER_INVENTORY_FORMAT")]
    FILTERINVENTORYFORMAT,
    
    /// "FILTER_ZIP_CODE"
    #[serde(rename="FILTER_ZIP_CODE")]
    FILTERZIPCODE,
    
    /// "FILTER_VIDEO_RATING_TIER"
    #[serde(rename="FILTER_VIDEO_RATING_TIER")]
    FILTERVIDEORATINGTIER,
    
    /// "FILTER_VIDEO_FORMAT_SUPPORT"
    #[serde(rename="FILTER_VIDEO_FORMAT_SUPPORT")]
    FILTERVIDEOFORMATSUPPORT,
    
    /// "FILTER_VIDEO_SKIPPABLE_SUPPORT"
    #[serde(rename="FILTER_VIDEO_SKIPPABLE_SUPPORT")]
    FILTERVIDEOSKIPPABLESUPPORT,
    
    /// "FILTER_VIDEO_VPAID_SUPPORT"
    #[serde(rename="FILTER_VIDEO_VPAID_SUPPORT")]
    FILTERVIDEOVPAIDSUPPORT,
    
    /// "FILTER_VIDEO_CREATIVE_DURATION"
    #[serde(rename="FILTER_VIDEO_CREATIVE_DURATION")]
    FILTERVIDEOCREATIVEDURATION,
    
    /// "FILTER_PAGE_LAYOUT"
    #[serde(rename="FILTER_PAGE_LAYOUT")]
    FILTERPAGELAYOUT,
    
    /// "FILTER_VIDEO_AD_POSITION_IN_STREAM"
    #[serde(rename="FILTER_VIDEO_AD_POSITION_IN_STREAM")]
    FILTERVIDEOADPOSITIONINSTREAM,
    
    /// "FILTER_AGE"
    #[serde(rename="FILTER_AGE")]
    FILTERAGE,
    
    /// "FILTER_GENDER"
    #[serde(rename="FILTER_GENDER")]
    FILTERGENDER,
    
    /// "FILTER_QUARTER"
    #[serde(rename="FILTER_QUARTER")]
    FILTERQUARTER,
    
    /// "FILTER_TRUEVIEW_CONVERSION_TYPE"
    #[serde(rename="FILTER_TRUEVIEW_CONVERSION_TYPE")]
    FILTERTRUEVIEWCONVERSIONTYPE,
    
    /// "FILTER_MOBILE_GEO"
    #[serde(rename="FILTER_MOBILE_GEO")]
    FILTERMOBILEGEO,
    
    /// "FILTER_MRAID_SUPPORT"
    #[serde(rename="FILTER_MRAID_SUPPORT")]
    FILTERMRAIDSUPPORT,
    
    /// "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY"
    #[serde(rename="FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY")]
    FILTERACTIVEVIEWEXPECTEDVIEWABILITY,
    
    /// "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE"
    #[serde(rename="FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE")]
    FILTERVIDEOCREATIVEDURATIONSKIPPABLE,
    
    /// "FILTER_NIELSEN_COUNTRY_CODE"
    #[serde(rename="FILTER_NIELSEN_COUNTRY_CODE")]
    FILTERNIELSENCOUNTRYCODE,
    
    /// "FILTER_NIELSEN_DEVICE_ID"
    #[serde(rename="FILTER_NIELSEN_DEVICE_ID")]
    FILTERNIELSENDEVICEID,
    
    /// "FILTER_NIELSEN_GENDER"
    #[serde(rename="FILTER_NIELSEN_GENDER")]
    FILTERNIELSENGENDER,
    
    /// "FILTER_NIELSEN_AGE"
    #[serde(rename="FILTER_NIELSEN_AGE")]
    FILTERNIELSENAGE,
    
    /// "FILTER_INVENTORY_SOURCE_TYPE"
    #[serde(rename="FILTER_INVENTORY_SOURCE_TYPE")]
    FILTERINVENTORYSOURCETYPE,
    
    /// "FILTER_CREATIVE_WIDTH"
    #[serde(rename="FILTER_CREATIVE_WIDTH")]
    FILTERCREATIVEWIDTH,
    
    /// "FILTER_CREATIVE_HEIGHT"
    #[serde(rename="FILTER_CREATIVE_HEIGHT")]
    FILTERCREATIVEHEIGHT,
    
    /// "FILTER_DFP_ORDER_ID"
    #[serde(rename="FILTER_DFP_ORDER_ID")]
    FILTERDFPORDERID,
    
    /// "FILTER_TRUEVIEW_AGE"
    #[serde(rename="FILTER_TRUEVIEW_AGE")]
    FILTERTRUEVIEWAGE,
    
    /// "FILTER_TRUEVIEW_GENDER"
    #[serde(rename="FILTER_TRUEVIEW_GENDER")]
    FILTERTRUEVIEWGENDER,
    
    /// "FILTER_TRUEVIEW_PARENTAL_STATUS"
    #[serde(rename="FILTER_TRUEVIEW_PARENTAL_STATUS")]
    FILTERTRUEVIEWPARENTALSTATUS,
    
    /// "FILTER_TRUEVIEW_REMARKETING_LIST"
    #[serde(rename="FILTER_TRUEVIEW_REMARKETING_LIST")]
    FILTERTRUEVIEWREMARKETINGLIST,
    
    /// "FILTER_TRUEVIEW_INTEREST"
    #[serde(rename="FILTER_TRUEVIEW_INTEREST")]
    FILTERTRUEVIEWINTEREST,
    
    /// "FILTER_TRUEVIEW_AD_GROUP_ID"
    #[serde(rename="FILTER_TRUEVIEW_AD_GROUP_ID")]
    FILTERTRUEVIEWADGROUPID,
    
    /// "FILTER_TRUEVIEW_AD_GROUP_AD_ID"
    #[serde(rename="FILTER_TRUEVIEW_AD_GROUP_AD_ID")]
    FILTERTRUEVIEWADGROUPADID,
    
    /// "FILTER_TRUEVIEW_IAR_LANGUAGE"
    #[serde(rename="FILTER_TRUEVIEW_IAR_LANGUAGE")]
    FILTERTRUEVIEWIARLANGUAGE,
    
    /// "FILTER_TRUEVIEW_IAR_GENDER"
    #[serde(rename="FILTER_TRUEVIEW_IAR_GENDER")]
    FILTERTRUEVIEWIARGENDER,
    
    /// "FILTER_TRUEVIEW_IAR_AGE"
    #[serde(rename="FILTER_TRUEVIEW_IAR_AGE")]
    FILTERTRUEVIEWIARAGE,
    
    /// "FILTER_TRUEVIEW_IAR_CATEGORY"
    #[serde(rename="FILTER_TRUEVIEW_IAR_CATEGORY")]
    FILTERTRUEVIEWIARCATEGORY,
    
    /// "FILTER_TRUEVIEW_IAR_COUNTRY"
    #[serde(rename="FILTER_TRUEVIEW_IAR_COUNTRY")]
    FILTERTRUEVIEWIARCOUNTRY,
    
    /// "FILTER_TRUEVIEW_IAR_CITY"
    #[serde(rename="FILTER_TRUEVIEW_IAR_CITY")]
    FILTERTRUEVIEWIARCITY,
    
    /// "FILTER_TRUEVIEW_IAR_REGION"
    #[serde(rename="FILTER_TRUEVIEW_IAR_REGION")]
    FILTERTRUEVIEWIARREGION,
    
    /// "FILTER_TRUEVIEW_IAR_ZIPCODE"
    #[serde(rename="FILTER_TRUEVIEW_IAR_ZIPCODE")]
    FILTERTRUEVIEWIARZIPCODE,
    
    /// "FILTER_TRUEVIEW_IAR_REMARKETING_LIST"
    #[serde(rename="FILTER_TRUEVIEW_IAR_REMARKETING_LIST")]
    FILTERTRUEVIEWIARREMARKETINGLIST,
    
    /// "FILTER_TRUEVIEW_IAR_INTEREST"
    #[serde(rename="FILTER_TRUEVIEW_IAR_INTEREST")]
    FILTERTRUEVIEWIARINTEREST,
    
    /// "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS"
    #[serde(rename="FILTER_TRUEVIEW_IAR_PARENTAL_STATUS")]
    FILTERTRUEVIEWIARPARENTALSTATUS,
    
    /// "FILTER_TRUEVIEW_IAR_TIME_OF_DAY"
    #[serde(rename="FILTER_TRUEVIEW_IAR_TIME_OF_DAY")]
    FILTERTRUEVIEWIARTIMEOFDAY,
    
    /// "FILTER_TRUEVIEW_CUSTOM_AFFINITY"
    #[serde(rename="FILTER_TRUEVIEW_CUSTOM_AFFINITY")]
    FILTERTRUEVIEWCUSTOMAFFINITY,
    
    /// "FILTER_TRUEVIEW_CATEGORY"
    #[serde(rename="FILTER_TRUEVIEW_CATEGORY")]
    FILTERTRUEVIEWCATEGORY,
    
    /// "FILTER_TRUEVIEW_KEYWORD"
    #[serde(rename="FILTER_TRUEVIEW_KEYWORD")]
    FILTERTRUEVIEWKEYWORD,
    
    /// "FILTER_TRUEVIEW_PLACEMENT"
    #[serde(rename="FILTER_TRUEVIEW_PLACEMENT")]
    FILTERTRUEVIEWPLACEMENT,
    
    /// "FILTER_TRUEVIEW_URL"
    #[serde(rename="FILTER_TRUEVIEW_URL")]
    FILTERTRUEVIEWURL,
    
    /// "FILTER_TRUEVIEW_COUNTRY"
    #[serde(rename="FILTER_TRUEVIEW_COUNTRY")]
    FILTERTRUEVIEWCOUNTRY,
    
    /// "FILTER_TRUEVIEW_REGION"
    #[serde(rename="FILTER_TRUEVIEW_REGION")]
    FILTERTRUEVIEWREGION,
    
    /// "FILTER_TRUEVIEW_CITY"
    #[serde(rename="FILTER_TRUEVIEW_CITY")]
    FILTERTRUEVIEWCITY,
    
    /// "FILTER_TRUEVIEW_DMA"
    #[serde(rename="FILTER_TRUEVIEW_DMA")]
    FILTERTRUEVIEWDMA,
    
    /// "FILTER_TRUEVIEW_ZIPCODE"
    #[serde(rename="FILTER_TRUEVIEW_ZIPCODE")]
    FILTERTRUEVIEWZIPCODE,
    
    /// "FILTER_NOT_SUPPORTED"
    #[serde(rename="FILTER_NOT_SUPPORTED")]
    FILTERNOTSUPPORTED,
    
    /// "FILTER_MEDIA_PLAN"
    #[serde(rename="FILTER_MEDIA_PLAN")]
    FILTERMEDIAPLAN,
    
    /// "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL"
    #[serde(rename="FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL")]
    FILTERTRUEVIEWIARYOUTUBECHANNEL,
    
    /// "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO"
    #[serde(rename="FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO")]
    FILTERTRUEVIEWIARYOUTUBEVIDEO,
    
    /// "FILTER_SKIPPABLE_SUPPORT"
    #[serde(rename="FILTER_SKIPPABLE_SUPPORT")]
    FILTERSKIPPABLESUPPORT,
    
    /// "FILTER_COMPANION_CREATIVE_ID"
    #[serde(rename="FILTER_COMPANION_CREATIVE_ID")]
    FILTERCOMPANIONCREATIVEID,
    
    /// "FILTER_BUDGET_SEGMENT_DESCRIPTION"
    #[serde(rename="FILTER_BUDGET_SEGMENT_DESCRIPTION")]
    FILTERBUDGETSEGMENTDESCRIPTION,
    
    /// "FILTER_FLOODLIGHT_ACTIVITY_ID"
    #[serde(rename="FILTER_FLOODLIGHT_ACTIVITY_ID")]
    FILTERFLOODLIGHTACTIVITYID,
    
    /// "FILTER_DEVICE_MODEL"
    #[serde(rename="FILTER_DEVICE_MODEL")]
    FILTERDEVICEMODEL,
    
    /// "FILTER_DEVICE_MAKE"
    #[serde(rename="FILTER_DEVICE_MAKE")]
    FILTERDEVICEMAKE,
    
    /// "FILTER_DEVICE_TYPE"
    #[serde(rename="FILTER_DEVICE_TYPE")]
    FILTERDEVICETYPE,
    
    /// "FILTER_CREATIVE_ATTRIBUTE"
    #[serde(rename="FILTER_CREATIVE_ATTRIBUTE")]
    FILTERCREATIVEATTRIBUTE,
    
    /// "FILTER_INVENTORY_COMMITMENT_TYPE"
    #[serde(rename="FILTER_INVENTORY_COMMITMENT_TYPE")]
    FILTERINVENTORYCOMMITMENTTYPE,
    
    /// "FILTER_INVENTORY_RATE_TYPE"
    #[serde(rename="FILTER_INVENTORY_RATE_TYPE")]
    FILTERINVENTORYRATETYPE,
    
    /// "FILTER_INVENTORY_DELIVERY_METHOD"
    #[serde(rename="FILTER_INVENTORY_DELIVERY_METHOD")]
    FILTERINVENTORYDELIVERYMETHOD,
    
    /// "FILTER_INVENTORY_SOURCE_EXTERNAL_ID"
    #[serde(rename="FILTER_INVENTORY_SOURCE_EXTERNAL_ID")]
    FILTERINVENTORYSOURCEEXTERNALID,
    
    /// "FILTER_AUTHORIZED_SELLER_STATE"
    #[serde(rename="FILTER_AUTHORIZED_SELLER_STATE")]
    FILTERAUTHORIZEDSELLERSTATE,
    
    /// "FILTER_VIDEO_DURATION_SECONDS_RANGE"
    #[serde(rename="FILTER_VIDEO_DURATION_SECONDS_RANGE")]
    FILTERVIDEODURATIONSECONDSRANGE,
}

impl AsRef<str> for ParameterGroupBysEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParameterGroupBysEnum::FILTERUNKNOWN => "FILTER_UNKNOWN",
            ParameterGroupBysEnum::FILTERDATE => "FILTER_DATE",
            ParameterGroupBysEnum::FILTERDAYOFWEEK => "FILTER_DAY_OF_WEEK",
            ParameterGroupBysEnum::FILTERWEEK => "FILTER_WEEK",
            ParameterGroupBysEnum::FILTERMONTH => "FILTER_MONTH",
            ParameterGroupBysEnum::FILTERYEAR => "FILTER_YEAR",
            ParameterGroupBysEnum::FILTERTIMEOFDAY => "FILTER_TIME_OF_DAY",
            ParameterGroupBysEnum::FILTERCONVERSIONDELAY => "FILTER_CONVERSION_DELAY",
            ParameterGroupBysEnum::FILTERCREATIVEID => "FILTER_CREATIVE_ID",
            ParameterGroupBysEnum::FILTERCREATIVESIZE => "FILTER_CREATIVE_SIZE",
            ParameterGroupBysEnum::FILTERCREATIVETYPE => "FILTER_CREATIVE_TYPE",
            ParameterGroupBysEnum::FILTEREXCHANGEID => "FILTER_EXCHANGE_ID",
            ParameterGroupBysEnum::FILTERADPOSITION => "FILTER_AD_POSITION",
            ParameterGroupBysEnum::FILTERINVENTORYSOURCE => "FILTER_INVENTORY_SOURCE",
            ParameterGroupBysEnum::FILTERCITY => "FILTER_CITY",
            ParameterGroupBysEnum::FILTERREGION => "FILTER_REGION",
            ParameterGroupBysEnum::FILTERDMA => "FILTER_DMA",
            ParameterGroupBysEnum::FILTERCOUNTRY => "FILTER_COUNTRY",
            ParameterGroupBysEnum::FILTERSITEID => "FILTER_SITE_ID",
            ParameterGroupBysEnum::FILTERCHANNELID => "FILTER_CHANNEL_ID",
            ParameterGroupBysEnum::FILTERPARTNER => "FILTER_PARTNER",
            ParameterGroupBysEnum::FILTERADVERTISER => "FILTER_ADVERTISER",
            ParameterGroupBysEnum::FILTERINSERTIONORDER => "FILTER_INSERTION_ORDER",
            ParameterGroupBysEnum::FILTERLINEITEM => "FILTER_LINE_ITEM",
            ParameterGroupBysEnum::FILTERPARTNERCURRENCY => "FILTER_PARTNER_CURRENCY",
            ParameterGroupBysEnum::FILTERADVERTISERCURRENCY => "FILTER_ADVERTISER_CURRENCY",
            ParameterGroupBysEnum::FILTERADVERTISERTIMEZONE => "FILTER_ADVERTISER_TIMEZONE",
            ParameterGroupBysEnum::FILTERLINEITEMTYPE => "FILTER_LINE_ITEM_TYPE",
            ParameterGroupBysEnum::FILTERUSERLIST => "FILTER_USER_LIST",
            ParameterGroupBysEnum::FILTERUSERLISTFIRSTPARTY => "FILTER_USER_LIST_FIRST_PARTY",
            ParameterGroupBysEnum::FILTERUSERLISTTHIRDPARTY => "FILTER_USER_LIST_THIRD_PARTY",
            ParameterGroupBysEnum::FILTERTARGETEDUSERLIST => "FILTER_TARGETED_USER_LIST",
            ParameterGroupBysEnum::FILTERDATAPROVIDER => "FILTER_DATA_PROVIDER",
            ParameterGroupBysEnum::FILTERORDERID => "FILTER_ORDER_ID",
            ParameterGroupBysEnum::FILTERVIDEOPLAYERSIZE => "FILTER_VIDEO_PLAYER_SIZE",
            ParameterGroupBysEnum::FILTERVIDEODURATIONSECONDS => "FILTER_VIDEO_DURATION_SECONDS",
            ParameterGroupBysEnum::FILTERKEYWORD => "FILTER_KEYWORD",
            ParameterGroupBysEnum::FILTERPAGECATEGORY => "FILTER_PAGE_CATEGORY",
            ParameterGroupBysEnum::FILTERCAMPAIGNDAILYFREQUENCY => "FILTER_CAMPAIGN_DAILY_FREQUENCY",
            ParameterGroupBysEnum::FILTERLINEITEMDAILYFREQUENCY => "FILTER_LINE_ITEM_DAILY_FREQUENCY",
            ParameterGroupBysEnum::FILTERLINEITEMLIFETIMEFREQUENCY => "FILTER_LINE_ITEM_LIFETIME_FREQUENCY",
            ParameterGroupBysEnum::FILTEROS => "FILTER_OS",
            ParameterGroupBysEnum::FILTERBROWSER => "FILTER_BROWSER",
            ParameterGroupBysEnum::FILTERCARRIER => "FILTER_CARRIER",
            ParameterGroupBysEnum::FILTERSITELANGUAGE => "FILTER_SITE_LANGUAGE",
            ParameterGroupBysEnum::FILTERINVENTORYFORMAT => "FILTER_INVENTORY_FORMAT",
            ParameterGroupBysEnum::FILTERZIPCODE => "FILTER_ZIP_CODE",
            ParameterGroupBysEnum::FILTERVIDEORATINGTIER => "FILTER_VIDEO_RATING_TIER",
            ParameterGroupBysEnum::FILTERVIDEOFORMATSUPPORT => "FILTER_VIDEO_FORMAT_SUPPORT",
            ParameterGroupBysEnum::FILTERVIDEOSKIPPABLESUPPORT => "FILTER_VIDEO_SKIPPABLE_SUPPORT",
            ParameterGroupBysEnum::FILTERVIDEOVPAIDSUPPORT => "FILTER_VIDEO_VPAID_SUPPORT",
            ParameterGroupBysEnum::FILTERVIDEOCREATIVEDURATION => "FILTER_VIDEO_CREATIVE_DURATION",
            ParameterGroupBysEnum::FILTERPAGELAYOUT => "FILTER_PAGE_LAYOUT",
            ParameterGroupBysEnum::FILTERVIDEOADPOSITIONINSTREAM => "FILTER_VIDEO_AD_POSITION_IN_STREAM",
            ParameterGroupBysEnum::FILTERAGE => "FILTER_AGE",
            ParameterGroupBysEnum::FILTERGENDER => "FILTER_GENDER",
            ParameterGroupBysEnum::FILTERQUARTER => "FILTER_QUARTER",
            ParameterGroupBysEnum::FILTERTRUEVIEWCONVERSIONTYPE => "FILTER_TRUEVIEW_CONVERSION_TYPE",
            ParameterGroupBysEnum::FILTERMOBILEGEO => "FILTER_MOBILE_GEO",
            ParameterGroupBysEnum::FILTERMRAIDSUPPORT => "FILTER_MRAID_SUPPORT",
            ParameterGroupBysEnum::FILTERACTIVEVIEWEXPECTEDVIEWABILITY => "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY",
            ParameterGroupBysEnum::FILTERVIDEOCREATIVEDURATIONSKIPPABLE => "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE",
            ParameterGroupBysEnum::FILTERNIELSENCOUNTRYCODE => "FILTER_NIELSEN_COUNTRY_CODE",
            ParameterGroupBysEnum::FILTERNIELSENDEVICEID => "FILTER_NIELSEN_DEVICE_ID",
            ParameterGroupBysEnum::FILTERNIELSENGENDER => "FILTER_NIELSEN_GENDER",
            ParameterGroupBysEnum::FILTERNIELSENAGE => "FILTER_NIELSEN_AGE",
            ParameterGroupBysEnum::FILTERINVENTORYSOURCETYPE => "FILTER_INVENTORY_SOURCE_TYPE",
            ParameterGroupBysEnum::FILTERCREATIVEWIDTH => "FILTER_CREATIVE_WIDTH",
            ParameterGroupBysEnum::FILTERCREATIVEHEIGHT => "FILTER_CREATIVE_HEIGHT",
            ParameterGroupBysEnum::FILTERDFPORDERID => "FILTER_DFP_ORDER_ID",
            ParameterGroupBysEnum::FILTERTRUEVIEWAGE => "FILTER_TRUEVIEW_AGE",
            ParameterGroupBysEnum::FILTERTRUEVIEWGENDER => "FILTER_TRUEVIEW_GENDER",
            ParameterGroupBysEnum::FILTERTRUEVIEWPARENTALSTATUS => "FILTER_TRUEVIEW_PARENTAL_STATUS",
            ParameterGroupBysEnum::FILTERTRUEVIEWREMARKETINGLIST => "FILTER_TRUEVIEW_REMARKETING_LIST",
            ParameterGroupBysEnum::FILTERTRUEVIEWINTEREST => "FILTER_TRUEVIEW_INTEREST",
            ParameterGroupBysEnum::FILTERTRUEVIEWADGROUPID => "FILTER_TRUEVIEW_AD_GROUP_ID",
            ParameterGroupBysEnum::FILTERTRUEVIEWADGROUPADID => "FILTER_TRUEVIEW_AD_GROUP_AD_ID",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARLANGUAGE => "FILTER_TRUEVIEW_IAR_LANGUAGE",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARGENDER => "FILTER_TRUEVIEW_IAR_GENDER",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARAGE => "FILTER_TRUEVIEW_IAR_AGE",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARCATEGORY => "FILTER_TRUEVIEW_IAR_CATEGORY",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARCOUNTRY => "FILTER_TRUEVIEW_IAR_COUNTRY",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARCITY => "FILTER_TRUEVIEW_IAR_CITY",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARREGION => "FILTER_TRUEVIEW_IAR_REGION",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARZIPCODE => "FILTER_TRUEVIEW_IAR_ZIPCODE",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARREMARKETINGLIST => "FILTER_TRUEVIEW_IAR_REMARKETING_LIST",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARINTEREST => "FILTER_TRUEVIEW_IAR_INTEREST",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARPARENTALSTATUS => "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARTIMEOFDAY => "FILTER_TRUEVIEW_IAR_TIME_OF_DAY",
            ParameterGroupBysEnum::FILTERTRUEVIEWCUSTOMAFFINITY => "FILTER_TRUEVIEW_CUSTOM_AFFINITY",
            ParameterGroupBysEnum::FILTERTRUEVIEWCATEGORY => "FILTER_TRUEVIEW_CATEGORY",
            ParameterGroupBysEnum::FILTERTRUEVIEWKEYWORD => "FILTER_TRUEVIEW_KEYWORD",
            ParameterGroupBysEnum::FILTERTRUEVIEWPLACEMENT => "FILTER_TRUEVIEW_PLACEMENT",
            ParameterGroupBysEnum::FILTERTRUEVIEWURL => "FILTER_TRUEVIEW_URL",
            ParameterGroupBysEnum::FILTERTRUEVIEWCOUNTRY => "FILTER_TRUEVIEW_COUNTRY",
            ParameterGroupBysEnum::FILTERTRUEVIEWREGION => "FILTER_TRUEVIEW_REGION",
            ParameterGroupBysEnum::FILTERTRUEVIEWCITY => "FILTER_TRUEVIEW_CITY",
            ParameterGroupBysEnum::FILTERTRUEVIEWDMA => "FILTER_TRUEVIEW_DMA",
            ParameterGroupBysEnum::FILTERTRUEVIEWZIPCODE => "FILTER_TRUEVIEW_ZIPCODE",
            ParameterGroupBysEnum::FILTERNOTSUPPORTED => "FILTER_NOT_SUPPORTED",
            ParameterGroupBysEnum::FILTERMEDIAPLAN => "FILTER_MEDIA_PLAN",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARYOUTUBECHANNEL => "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL",
            ParameterGroupBysEnum::FILTERTRUEVIEWIARYOUTUBEVIDEO => "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO",
            ParameterGroupBysEnum::FILTERSKIPPABLESUPPORT => "FILTER_SKIPPABLE_SUPPORT",
            ParameterGroupBysEnum::FILTERCOMPANIONCREATIVEID => "FILTER_COMPANION_CREATIVE_ID",
            ParameterGroupBysEnum::FILTERBUDGETSEGMENTDESCRIPTION => "FILTER_BUDGET_SEGMENT_DESCRIPTION",
            ParameterGroupBysEnum::FILTERFLOODLIGHTACTIVITYID => "FILTER_FLOODLIGHT_ACTIVITY_ID",
            ParameterGroupBysEnum::FILTERDEVICEMODEL => "FILTER_DEVICE_MODEL",
            ParameterGroupBysEnum::FILTERDEVICEMAKE => "FILTER_DEVICE_MAKE",
            ParameterGroupBysEnum::FILTERDEVICETYPE => "FILTER_DEVICE_TYPE",
            ParameterGroupBysEnum::FILTERCREATIVEATTRIBUTE => "FILTER_CREATIVE_ATTRIBUTE",
            ParameterGroupBysEnum::FILTERINVENTORYCOMMITMENTTYPE => "FILTER_INVENTORY_COMMITMENT_TYPE",
            ParameterGroupBysEnum::FILTERINVENTORYRATETYPE => "FILTER_INVENTORY_RATE_TYPE",
            ParameterGroupBysEnum::FILTERINVENTORYDELIVERYMETHOD => "FILTER_INVENTORY_DELIVERY_METHOD",
            ParameterGroupBysEnum::FILTERINVENTORYSOURCEEXTERNALID => "FILTER_INVENTORY_SOURCE_EXTERNAL_ID",
            ParameterGroupBysEnum::FILTERAUTHORIZEDSELLERSTATE => "FILTER_AUTHORIZED_SELLER_STATE",
            ParameterGroupBysEnum::FILTERVIDEODURATIONSECONDSRANGE => "FILTER_VIDEO_DURATION_SECONDS_RANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for ParameterGroupBysEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FILTER_UNKNOWN" => Ok(ParameterGroupBysEnum::FILTERUNKNOWN),
           "FILTER_DATE" => Ok(ParameterGroupBysEnum::FILTERDATE),
           "FILTER_DAY_OF_WEEK" => Ok(ParameterGroupBysEnum::FILTERDAYOFWEEK),
           "FILTER_WEEK" => Ok(ParameterGroupBysEnum::FILTERWEEK),
           "FILTER_MONTH" => Ok(ParameterGroupBysEnum::FILTERMONTH),
           "FILTER_YEAR" => Ok(ParameterGroupBysEnum::FILTERYEAR),
           "FILTER_TIME_OF_DAY" => Ok(ParameterGroupBysEnum::FILTERTIMEOFDAY),
           "FILTER_CONVERSION_DELAY" => Ok(ParameterGroupBysEnum::FILTERCONVERSIONDELAY),
           "FILTER_CREATIVE_ID" => Ok(ParameterGroupBysEnum::FILTERCREATIVEID),
           "FILTER_CREATIVE_SIZE" => Ok(ParameterGroupBysEnum::FILTERCREATIVESIZE),
           "FILTER_CREATIVE_TYPE" => Ok(ParameterGroupBysEnum::FILTERCREATIVETYPE),
           "FILTER_EXCHANGE_ID" => Ok(ParameterGroupBysEnum::FILTEREXCHANGEID),
           "FILTER_AD_POSITION" => Ok(ParameterGroupBysEnum::FILTERADPOSITION),
           "FILTER_INVENTORY_SOURCE" => Ok(ParameterGroupBysEnum::FILTERINVENTORYSOURCE),
           "FILTER_CITY" => Ok(ParameterGroupBysEnum::FILTERCITY),
           "FILTER_REGION" => Ok(ParameterGroupBysEnum::FILTERREGION),
           "FILTER_DMA" => Ok(ParameterGroupBysEnum::FILTERDMA),
           "FILTER_COUNTRY" => Ok(ParameterGroupBysEnum::FILTERCOUNTRY),
           "FILTER_SITE_ID" => Ok(ParameterGroupBysEnum::FILTERSITEID),
           "FILTER_CHANNEL_ID" => Ok(ParameterGroupBysEnum::FILTERCHANNELID),
           "FILTER_PARTNER" => Ok(ParameterGroupBysEnum::FILTERPARTNER),
           "FILTER_ADVERTISER" => Ok(ParameterGroupBysEnum::FILTERADVERTISER),
           "FILTER_INSERTION_ORDER" => Ok(ParameterGroupBysEnum::FILTERINSERTIONORDER),
           "FILTER_LINE_ITEM" => Ok(ParameterGroupBysEnum::FILTERLINEITEM),
           "FILTER_PARTNER_CURRENCY" => Ok(ParameterGroupBysEnum::FILTERPARTNERCURRENCY),
           "FILTER_ADVERTISER_CURRENCY" => Ok(ParameterGroupBysEnum::FILTERADVERTISERCURRENCY),
           "FILTER_ADVERTISER_TIMEZONE" => Ok(ParameterGroupBysEnum::FILTERADVERTISERTIMEZONE),
           "FILTER_LINE_ITEM_TYPE" => Ok(ParameterGroupBysEnum::FILTERLINEITEMTYPE),
           "FILTER_USER_LIST" => Ok(ParameterGroupBysEnum::FILTERUSERLIST),
           "FILTER_USER_LIST_FIRST_PARTY" => Ok(ParameterGroupBysEnum::FILTERUSERLISTFIRSTPARTY),
           "FILTER_USER_LIST_THIRD_PARTY" => Ok(ParameterGroupBysEnum::FILTERUSERLISTTHIRDPARTY),
           "FILTER_TARGETED_USER_LIST" => Ok(ParameterGroupBysEnum::FILTERTARGETEDUSERLIST),
           "FILTER_DATA_PROVIDER" => Ok(ParameterGroupBysEnum::FILTERDATAPROVIDER),
           "FILTER_ORDER_ID" => Ok(ParameterGroupBysEnum::FILTERORDERID),
           "FILTER_VIDEO_PLAYER_SIZE" => Ok(ParameterGroupBysEnum::FILTERVIDEOPLAYERSIZE),
           "FILTER_VIDEO_DURATION_SECONDS" => Ok(ParameterGroupBysEnum::FILTERVIDEODURATIONSECONDS),
           "FILTER_KEYWORD" => Ok(ParameterGroupBysEnum::FILTERKEYWORD),
           "FILTER_PAGE_CATEGORY" => Ok(ParameterGroupBysEnum::FILTERPAGECATEGORY),
           "FILTER_CAMPAIGN_DAILY_FREQUENCY" => Ok(ParameterGroupBysEnum::FILTERCAMPAIGNDAILYFREQUENCY),
           "FILTER_LINE_ITEM_DAILY_FREQUENCY" => Ok(ParameterGroupBysEnum::FILTERLINEITEMDAILYFREQUENCY),
           "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" => Ok(ParameterGroupBysEnum::FILTERLINEITEMLIFETIMEFREQUENCY),
           "FILTER_OS" => Ok(ParameterGroupBysEnum::FILTEROS),
           "FILTER_BROWSER" => Ok(ParameterGroupBysEnum::FILTERBROWSER),
           "FILTER_CARRIER" => Ok(ParameterGroupBysEnum::FILTERCARRIER),
           "FILTER_SITE_LANGUAGE" => Ok(ParameterGroupBysEnum::FILTERSITELANGUAGE),
           "FILTER_INVENTORY_FORMAT" => Ok(ParameterGroupBysEnum::FILTERINVENTORYFORMAT),
           "FILTER_ZIP_CODE" => Ok(ParameterGroupBysEnum::FILTERZIPCODE),
           "FILTER_VIDEO_RATING_TIER" => Ok(ParameterGroupBysEnum::FILTERVIDEORATINGTIER),
           "FILTER_VIDEO_FORMAT_SUPPORT" => Ok(ParameterGroupBysEnum::FILTERVIDEOFORMATSUPPORT),
           "FILTER_VIDEO_SKIPPABLE_SUPPORT" => Ok(ParameterGroupBysEnum::FILTERVIDEOSKIPPABLESUPPORT),
           "FILTER_VIDEO_VPAID_SUPPORT" => Ok(ParameterGroupBysEnum::FILTERVIDEOVPAIDSUPPORT),
           "FILTER_VIDEO_CREATIVE_DURATION" => Ok(ParameterGroupBysEnum::FILTERVIDEOCREATIVEDURATION),
           "FILTER_PAGE_LAYOUT" => Ok(ParameterGroupBysEnum::FILTERPAGELAYOUT),
           "FILTER_VIDEO_AD_POSITION_IN_STREAM" => Ok(ParameterGroupBysEnum::FILTERVIDEOADPOSITIONINSTREAM),
           "FILTER_AGE" => Ok(ParameterGroupBysEnum::FILTERAGE),
           "FILTER_GENDER" => Ok(ParameterGroupBysEnum::FILTERGENDER),
           "FILTER_QUARTER" => Ok(ParameterGroupBysEnum::FILTERQUARTER),
           "FILTER_TRUEVIEW_CONVERSION_TYPE" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWCONVERSIONTYPE),
           "FILTER_MOBILE_GEO" => Ok(ParameterGroupBysEnum::FILTERMOBILEGEO),
           "FILTER_MRAID_SUPPORT" => Ok(ParameterGroupBysEnum::FILTERMRAIDSUPPORT),
           "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" => Ok(ParameterGroupBysEnum::FILTERACTIVEVIEWEXPECTEDVIEWABILITY),
           "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" => Ok(ParameterGroupBysEnum::FILTERVIDEOCREATIVEDURATIONSKIPPABLE),
           "FILTER_NIELSEN_COUNTRY_CODE" => Ok(ParameterGroupBysEnum::FILTERNIELSENCOUNTRYCODE),
           "FILTER_NIELSEN_DEVICE_ID" => Ok(ParameterGroupBysEnum::FILTERNIELSENDEVICEID),
           "FILTER_NIELSEN_GENDER" => Ok(ParameterGroupBysEnum::FILTERNIELSENGENDER),
           "FILTER_NIELSEN_AGE" => Ok(ParameterGroupBysEnum::FILTERNIELSENAGE),
           "FILTER_INVENTORY_SOURCE_TYPE" => Ok(ParameterGroupBysEnum::FILTERINVENTORYSOURCETYPE),
           "FILTER_CREATIVE_WIDTH" => Ok(ParameterGroupBysEnum::FILTERCREATIVEWIDTH),
           "FILTER_CREATIVE_HEIGHT" => Ok(ParameterGroupBysEnum::FILTERCREATIVEHEIGHT),
           "FILTER_DFP_ORDER_ID" => Ok(ParameterGroupBysEnum::FILTERDFPORDERID),
           "FILTER_TRUEVIEW_AGE" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWAGE),
           "FILTER_TRUEVIEW_GENDER" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWGENDER),
           "FILTER_TRUEVIEW_PARENTAL_STATUS" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWPARENTALSTATUS),
           "FILTER_TRUEVIEW_REMARKETING_LIST" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWREMARKETINGLIST),
           "FILTER_TRUEVIEW_INTEREST" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWINTEREST),
           "FILTER_TRUEVIEW_AD_GROUP_ID" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWADGROUPID),
           "FILTER_TRUEVIEW_AD_GROUP_AD_ID" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWADGROUPADID),
           "FILTER_TRUEVIEW_IAR_LANGUAGE" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARLANGUAGE),
           "FILTER_TRUEVIEW_IAR_GENDER" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARGENDER),
           "FILTER_TRUEVIEW_IAR_AGE" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARAGE),
           "FILTER_TRUEVIEW_IAR_CATEGORY" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARCATEGORY),
           "FILTER_TRUEVIEW_IAR_COUNTRY" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARCOUNTRY),
           "FILTER_TRUEVIEW_IAR_CITY" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARCITY),
           "FILTER_TRUEVIEW_IAR_REGION" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARREGION),
           "FILTER_TRUEVIEW_IAR_ZIPCODE" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARZIPCODE),
           "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARREMARKETINGLIST),
           "FILTER_TRUEVIEW_IAR_INTEREST" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARINTEREST),
           "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARPARENTALSTATUS),
           "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARTIMEOFDAY),
           "FILTER_TRUEVIEW_CUSTOM_AFFINITY" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWCUSTOMAFFINITY),
           "FILTER_TRUEVIEW_CATEGORY" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWCATEGORY),
           "FILTER_TRUEVIEW_KEYWORD" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWKEYWORD),
           "FILTER_TRUEVIEW_PLACEMENT" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWPLACEMENT),
           "FILTER_TRUEVIEW_URL" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWURL),
           "FILTER_TRUEVIEW_COUNTRY" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWCOUNTRY),
           "FILTER_TRUEVIEW_REGION" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWREGION),
           "FILTER_TRUEVIEW_CITY" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWCITY),
           "FILTER_TRUEVIEW_DMA" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWDMA),
           "FILTER_TRUEVIEW_ZIPCODE" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWZIPCODE),
           "FILTER_NOT_SUPPORTED" => Ok(ParameterGroupBysEnum::FILTERNOTSUPPORTED),
           "FILTER_MEDIA_PLAN" => Ok(ParameterGroupBysEnum::FILTERMEDIAPLAN),
           "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARYOUTUBECHANNEL),
           "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" => Ok(ParameterGroupBysEnum::FILTERTRUEVIEWIARYOUTUBEVIDEO),
           "FILTER_SKIPPABLE_SUPPORT" => Ok(ParameterGroupBysEnum::FILTERSKIPPABLESUPPORT),
           "FILTER_COMPANION_CREATIVE_ID" => Ok(ParameterGroupBysEnum::FILTERCOMPANIONCREATIVEID),
           "FILTER_BUDGET_SEGMENT_DESCRIPTION" => Ok(ParameterGroupBysEnum::FILTERBUDGETSEGMENTDESCRIPTION),
           "FILTER_FLOODLIGHT_ACTIVITY_ID" => Ok(ParameterGroupBysEnum::FILTERFLOODLIGHTACTIVITYID),
           "FILTER_DEVICE_MODEL" => Ok(ParameterGroupBysEnum::FILTERDEVICEMODEL),
           "FILTER_DEVICE_MAKE" => Ok(ParameterGroupBysEnum::FILTERDEVICEMAKE),
           "FILTER_DEVICE_TYPE" => Ok(ParameterGroupBysEnum::FILTERDEVICETYPE),
           "FILTER_CREATIVE_ATTRIBUTE" => Ok(ParameterGroupBysEnum::FILTERCREATIVEATTRIBUTE),
           "FILTER_INVENTORY_COMMITMENT_TYPE" => Ok(ParameterGroupBysEnum::FILTERINVENTORYCOMMITMENTTYPE),
           "FILTER_INVENTORY_RATE_TYPE" => Ok(ParameterGroupBysEnum::FILTERINVENTORYRATETYPE),
           "FILTER_INVENTORY_DELIVERY_METHOD" => Ok(ParameterGroupBysEnum::FILTERINVENTORYDELIVERYMETHOD),
           "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" => Ok(ParameterGroupBysEnum::FILTERINVENTORYSOURCEEXTERNALID),
           "FILTER_AUTHORIZED_SELLER_STATE" => Ok(ParameterGroupBysEnum::FILTERAUTHORIZEDSELLERSTATE),
           "FILTER_VIDEO_DURATION_SECONDS_RANGE" => Ok(ParameterGroupBysEnum::FILTERVIDEODURATIONSECONDSRANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParameterGroupBysEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParameterMetricsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Metrics to include as columns in your report.
pub enum ParameterMetricsEnum {
    
    /// "METRIC_UNKNOWN"
    #[serde(rename="METRIC_UNKNOWN")]
    METRICUNKNOWN,
    
    /// "METRIC_IMPRESSIONS"
    #[serde(rename="METRIC_IMPRESSIONS")]
    METRICIMPRESSIONS,
    
    /// "METRIC_CLICKS"
    #[serde(rename="METRIC_CLICKS")]
    METRICCLICKS,
    
    /// "METRIC_LAST_IMPRESSIONS"
    #[serde(rename="METRIC_LAST_IMPRESSIONS")]
    METRICLASTIMPRESSIONS,
    
    /// "METRIC_LAST_CLICKS"
    #[serde(rename="METRIC_LAST_CLICKS")]
    METRICLASTCLICKS,
    
    /// "METRIC_TOTAL_CONVERSIONS"
    #[serde(rename="METRIC_TOTAL_CONVERSIONS")]
    METRICTOTALCONVERSIONS,
    
    /// "METRIC_MEDIA_COST_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_COST_ADVERTISER")]
    METRICMEDIACOSTADVERTISER,
    
    /// "METRIC_MEDIA_COST_USD"
    #[serde(rename="METRIC_MEDIA_COST_USD")]
    METRICMEDIACOSTUSD,
    
    /// "METRIC_MEDIA_COST_PARTNER"
    #[serde(rename="METRIC_MEDIA_COST_PARTNER")]
    METRICMEDIACOSTPARTNER,
    
    /// "METRIC_DATA_COST_ADVERTISER"
    #[serde(rename="METRIC_DATA_COST_ADVERTISER")]
    METRICDATACOSTADVERTISER,
    
    /// "METRIC_DATA_COST_USD"
    #[serde(rename="METRIC_DATA_COST_USD")]
    METRICDATACOSTUSD,
    
    /// "METRIC_DATA_COST_PARTNER"
    #[serde(rename="METRIC_DATA_COST_PARTNER")]
    METRICDATACOSTPARTNER,
    
    /// "METRIC_CPM_FEE1_ADVERTISER"
    #[serde(rename="METRIC_CPM_FEE1_ADVERTISER")]
    METRICCPMFEE1ADVERTISER,
    
    /// "METRIC_CPM_FEE1_USD"
    #[serde(rename="METRIC_CPM_FEE1_USD")]
    METRICCPMFEE1USD,
    
    /// "METRIC_CPM_FEE1_PARTNER"
    #[serde(rename="METRIC_CPM_FEE1_PARTNER")]
    METRICCPMFEE1PARTNER,
    
    /// "METRIC_CPM_FEE2_ADVERTISER"
    #[serde(rename="METRIC_CPM_FEE2_ADVERTISER")]
    METRICCPMFEE2ADVERTISER,
    
    /// "METRIC_CPM_FEE2_USD"
    #[serde(rename="METRIC_CPM_FEE2_USD")]
    METRICCPMFEE2USD,
    
    /// "METRIC_CPM_FEE2_PARTNER"
    #[serde(rename="METRIC_CPM_FEE2_PARTNER")]
    METRICCPMFEE2PARTNER,
    
    /// "METRIC_MEDIA_FEE1_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_FEE1_ADVERTISER")]
    METRICMEDIAFEE1ADVERTISER,
    
    /// "METRIC_MEDIA_FEE1_USD"
    #[serde(rename="METRIC_MEDIA_FEE1_USD")]
    METRICMEDIAFEE1USD,
    
    /// "METRIC_MEDIA_FEE1_PARTNER"
    #[serde(rename="METRIC_MEDIA_FEE1_PARTNER")]
    METRICMEDIAFEE1PARTNER,
    
    /// "METRIC_MEDIA_FEE2_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_FEE2_ADVERTISER")]
    METRICMEDIAFEE2ADVERTISER,
    
    /// "METRIC_MEDIA_FEE2_USD"
    #[serde(rename="METRIC_MEDIA_FEE2_USD")]
    METRICMEDIAFEE2USD,
    
    /// "METRIC_MEDIA_FEE2_PARTNER"
    #[serde(rename="METRIC_MEDIA_FEE2_PARTNER")]
    METRICMEDIAFEE2PARTNER,
    
    /// "METRIC_REVENUE_ADVERTISER"
    #[serde(rename="METRIC_REVENUE_ADVERTISER")]
    METRICREVENUEADVERTISER,
    
    /// "METRIC_REVENUE_USD"
    #[serde(rename="METRIC_REVENUE_USD")]
    METRICREVENUEUSD,
    
    /// "METRIC_REVENUE_PARTNER"
    #[serde(rename="METRIC_REVENUE_PARTNER")]
    METRICREVENUEPARTNER,
    
    /// "METRIC_PROFIT_ADVERTISER"
    #[serde(rename="METRIC_PROFIT_ADVERTISER")]
    METRICPROFITADVERTISER,
    
    /// "METRIC_PROFIT_USD"
    #[serde(rename="METRIC_PROFIT_USD")]
    METRICPROFITUSD,
    
    /// "METRIC_PROFIT_PARTNER"
    #[serde(rename="METRIC_PROFIT_PARTNER")]
    METRICPROFITPARTNER,
    
    /// "METRIC_PROFIT_MARGIN"
    #[serde(rename="METRIC_PROFIT_MARGIN")]
    METRICPROFITMARGIN,
    
    /// "METRIC_TOTAL_MEDIA_COST_USD"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_USD")]
    METRICTOTALMEDIACOSTUSD,
    
    /// "METRIC_TOTAL_MEDIA_COST_PARTNER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_PARTNER")]
    METRICTOTALMEDIACOSTPARTNER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ADVERTISER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ADVERTISER")]
    METRICTOTALMEDIACOSTADVERTISER,
    
    /// "METRIC_BILLABLE_COST_USD"
    #[serde(rename="METRIC_BILLABLE_COST_USD")]
    METRICBILLABLECOSTUSD,
    
    /// "METRIC_BILLABLE_COST_PARTNER"
    #[serde(rename="METRIC_BILLABLE_COST_PARTNER")]
    METRICBILLABLECOSTPARTNER,
    
    /// "METRIC_BILLABLE_COST_ADVERTISER"
    #[serde(rename="METRIC_BILLABLE_COST_ADVERTISER")]
    METRICBILLABLECOSTADVERTISER,
    
    /// "METRIC_PLATFORM_FEE_USD"
    #[serde(rename="METRIC_PLATFORM_FEE_USD")]
    METRICPLATFORMFEEUSD,
    
    /// "METRIC_PLATFORM_FEE_PARTNER"
    #[serde(rename="METRIC_PLATFORM_FEE_PARTNER")]
    METRICPLATFORMFEEPARTNER,
    
    /// "METRIC_PLATFORM_FEE_ADVERTISER"
    #[serde(rename="METRIC_PLATFORM_FEE_ADVERTISER")]
    METRICPLATFORMFEEADVERTISER,
    
    /// "METRIC_VIDEO_COMPLETION_RATE"
    #[serde(rename="METRIC_VIDEO_COMPLETION_RATE")]
    METRICVIDEOCOMPLETIONRATE,
    
    /// "METRIC_PROFIT_ECPM_ADVERTISER"
    #[serde(rename="METRIC_PROFIT_ECPM_ADVERTISER")]
    METRICPROFITECPMADVERTISER,
    
    /// "METRIC_PROFIT_ECPM_USD"
    #[serde(rename="METRIC_PROFIT_ECPM_USD")]
    METRICPROFITECPMUSD,
    
    /// "METRIC_PROFIT_ECPM_PARTNER"
    #[serde(rename="METRIC_PROFIT_ECPM_PARTNER")]
    METRICPROFITECPMPARTNER,
    
    /// "METRIC_REVENUE_ECPM_ADVERTISER"
    #[serde(rename="METRIC_REVENUE_ECPM_ADVERTISER")]
    METRICREVENUEECPMADVERTISER,
    
    /// "METRIC_REVENUE_ECPM_USD"
    #[serde(rename="METRIC_REVENUE_ECPM_USD")]
    METRICREVENUEECPMUSD,
    
    /// "METRIC_REVENUE_ECPM_PARTNER"
    #[serde(rename="METRIC_REVENUE_ECPM_PARTNER")]
    METRICREVENUEECPMPARTNER,
    
    /// "METRIC_REVENUE_ECPC_ADVERTISER"
    #[serde(rename="METRIC_REVENUE_ECPC_ADVERTISER")]
    METRICREVENUEECPCADVERTISER,
    
    /// "METRIC_REVENUE_ECPC_USD"
    #[serde(rename="METRIC_REVENUE_ECPC_USD")]
    METRICREVENUEECPCUSD,
    
    /// "METRIC_REVENUE_ECPC_PARTNER"
    #[serde(rename="METRIC_REVENUE_ECPC_PARTNER")]
    METRICREVENUEECPCPARTNER,
    
    /// "METRIC_REVENUE_ECPA_ADVERTISER"
    #[serde(rename="METRIC_REVENUE_ECPA_ADVERTISER")]
    METRICREVENUEECPAADVERTISER,
    
    /// "METRIC_REVENUE_ECPA_USD"
    #[serde(rename="METRIC_REVENUE_ECPA_USD")]
    METRICREVENUEECPAUSD,
    
    /// "METRIC_REVENUE_ECPA_PARTNER"
    #[serde(rename="METRIC_REVENUE_ECPA_PARTNER")]
    METRICREVENUEECPAPARTNER,
    
    /// "METRIC_REVENUE_ECPAPV_ADVERTISER"
    #[serde(rename="METRIC_REVENUE_ECPAPV_ADVERTISER")]
    METRICREVENUEECPAPVADVERTISER,
    
    /// "METRIC_REVENUE_ECPAPV_USD"
    #[serde(rename="METRIC_REVENUE_ECPAPV_USD")]
    METRICREVENUEECPAPVUSD,
    
    /// "METRIC_REVENUE_ECPAPV_PARTNER"
    #[serde(rename="METRIC_REVENUE_ECPAPV_PARTNER")]
    METRICREVENUEECPAPVPARTNER,
    
    /// "METRIC_REVENUE_ECPAPC_ADVERTISER"
    #[serde(rename="METRIC_REVENUE_ECPAPC_ADVERTISER")]
    METRICREVENUEECPAPCADVERTISER,
    
    /// "METRIC_REVENUE_ECPAPC_USD"
    #[serde(rename="METRIC_REVENUE_ECPAPC_USD")]
    METRICREVENUEECPAPCUSD,
    
    /// "METRIC_REVENUE_ECPAPC_PARTNER"
    #[serde(rename="METRIC_REVENUE_ECPAPC_PARTNER")]
    METRICREVENUEECPAPCPARTNER,
    
    /// "METRIC_MEDIA_COST_ECPM_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_COST_ECPM_ADVERTISER")]
    METRICMEDIACOSTECPMADVERTISER,
    
    /// "METRIC_MEDIA_COST_ECPM_USD"
    #[serde(rename="METRIC_MEDIA_COST_ECPM_USD")]
    METRICMEDIACOSTECPMUSD,
    
    /// "METRIC_MEDIA_COST_ECPM_PARTNER"
    #[serde(rename="METRIC_MEDIA_COST_ECPM_PARTNER")]
    METRICMEDIACOSTECPMPARTNER,
    
    /// "METRIC_MEDIA_COST_ECPC_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_COST_ECPC_ADVERTISER")]
    METRICMEDIACOSTECPCADVERTISER,
    
    /// "METRIC_MEDIA_COST_ECPC_USD"
    #[serde(rename="METRIC_MEDIA_COST_ECPC_USD")]
    METRICMEDIACOSTECPCUSD,
    
    /// "METRIC_MEDIA_COST_ECPC_PARTNER"
    #[serde(rename="METRIC_MEDIA_COST_ECPC_PARTNER")]
    METRICMEDIACOSTECPCPARTNER,
    
    /// "METRIC_MEDIA_COST_ECPA_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_COST_ECPA_ADVERTISER")]
    METRICMEDIACOSTECPAADVERTISER,
    
    /// "METRIC_MEDIA_COST_ECPA_USD"
    #[serde(rename="METRIC_MEDIA_COST_ECPA_USD")]
    METRICMEDIACOSTECPAUSD,
    
    /// "METRIC_MEDIA_COST_ECPA_PARTNER"
    #[serde(rename="METRIC_MEDIA_COST_ECPA_PARTNER")]
    METRICMEDIACOSTECPAPARTNER,
    
    /// "METRIC_MEDIA_COST_ECPAPV_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_COST_ECPAPV_ADVERTISER")]
    METRICMEDIACOSTECPAPVADVERTISER,
    
    /// "METRIC_MEDIA_COST_ECPAPV_USD"
    #[serde(rename="METRIC_MEDIA_COST_ECPAPV_USD")]
    METRICMEDIACOSTECPAPVUSD,
    
    /// "METRIC_MEDIA_COST_ECPAPV_PARTNER"
    #[serde(rename="METRIC_MEDIA_COST_ECPAPV_PARTNER")]
    METRICMEDIACOSTECPAPVPARTNER,
    
    /// "METRIC_MEDIA_COST_ECPAPC_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_COST_ECPAPC_ADVERTISER")]
    METRICMEDIACOSTECPAPCADVERTISER,
    
    /// "METRIC_MEDIA_COST_ECPAPC_USD"
    #[serde(rename="METRIC_MEDIA_COST_ECPAPC_USD")]
    METRICMEDIACOSTECPAPCUSD,
    
    /// "METRIC_MEDIA_COST_ECPAPC_PARTNER"
    #[serde(rename="METRIC_MEDIA_COST_ECPAPC_PARTNER")]
    METRICMEDIACOSTECPAPCPARTNER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPM_ADVERTISER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPM_ADVERTISER")]
    METRICTOTALMEDIACOSTECPMADVERTISER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPM_USD"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPM_USD")]
    METRICTOTALMEDIACOSTECPMUSD,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPM_PARTNER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPM_PARTNER")]
    METRICTOTALMEDIACOSTECPMPARTNER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPC_ADVERTISER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPC_ADVERTISER")]
    METRICTOTALMEDIACOSTECPCADVERTISER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPC_USD"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPC_USD")]
    METRICTOTALMEDIACOSTECPCUSD,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPC_PARTNER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPC_PARTNER")]
    METRICTOTALMEDIACOSTECPCPARTNER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPA_ADVERTISER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPA_ADVERTISER")]
    METRICTOTALMEDIACOSTECPAADVERTISER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPA_USD"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPA_USD")]
    METRICTOTALMEDIACOSTECPAUSD,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPA_PARTNER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPA_PARTNER")]
    METRICTOTALMEDIACOSTECPAPARTNER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPAPV_ADVERTISER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPAPV_ADVERTISER")]
    METRICTOTALMEDIACOSTECPAPVADVERTISER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPAPV_USD"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPAPV_USD")]
    METRICTOTALMEDIACOSTECPAPVUSD,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPAPV_PARTNER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPAPV_PARTNER")]
    METRICTOTALMEDIACOSTECPAPVPARTNER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPAPC_ADVERTISER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPAPC_ADVERTISER")]
    METRICTOTALMEDIACOSTECPAPCADVERTISER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPAPC_USD"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPAPC_USD")]
    METRICTOTALMEDIACOSTECPAPCUSD,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPAPC_PARTNER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPAPC_PARTNER")]
    METRICTOTALMEDIACOSTECPAPCPARTNER,
    
    /// "METRIC_RICH_MEDIA_VIDEO_PLAYS"
    #[serde(rename="METRIC_RICH_MEDIA_VIDEO_PLAYS")]
    METRICRICHMEDIAVIDEOPLAYS,
    
    /// "METRIC_RICH_MEDIA_VIDEO_COMPLETIONS"
    #[serde(rename="METRIC_RICH_MEDIA_VIDEO_COMPLETIONS")]
    METRICRICHMEDIAVIDEOCOMPLETIONS,
    
    /// "METRIC_RICH_MEDIA_VIDEO_PAUSES"
    #[serde(rename="METRIC_RICH_MEDIA_VIDEO_PAUSES")]
    METRICRICHMEDIAVIDEOPAUSES,
    
    /// "METRIC_RICH_MEDIA_VIDEO_MUTES"
    #[serde(rename="METRIC_RICH_MEDIA_VIDEO_MUTES")]
    METRICRICHMEDIAVIDEOMUTES,
    
    /// "METRIC_RICH_MEDIA_VIDEO_MIDPOINTS"
    #[serde(rename="METRIC_RICH_MEDIA_VIDEO_MIDPOINTS")]
    METRICRICHMEDIAVIDEOMIDPOINTS,
    
    /// "METRIC_RICH_MEDIA_VIDEO_FULL_SCREENS"
    #[serde(rename="METRIC_RICH_MEDIA_VIDEO_FULL_SCREENS")]
    METRICRICHMEDIAVIDEOFULLSCREENS,
    
    /// "METRIC_RICH_MEDIA_VIDEO_FIRST_QUARTILE_COMPLETES"
    #[serde(rename="METRIC_RICH_MEDIA_VIDEO_FIRST_QUARTILE_COMPLETES")]
    METRICRICHMEDIAVIDEOFIRSTQUARTILECOMPLETES,
    
    /// "METRIC_RICH_MEDIA_VIDEO_THIRD_QUARTILE_COMPLETES"
    #[serde(rename="METRIC_RICH_MEDIA_VIDEO_THIRD_QUARTILE_COMPLETES")]
    METRICRICHMEDIAVIDEOTHIRDQUARTILECOMPLETES,
    
    /// "METRIC_CLICK_TO_POST_CLICK_CONVERSION_RATE"
    #[serde(rename="METRIC_CLICK_TO_POST_CLICK_CONVERSION_RATE")]
    METRICCLICKTOPOSTCLICKCONVERSIONRATE,
    
    /// "METRIC_IMPRESSIONS_TO_CONVERSION_RATE"
    #[serde(rename="METRIC_IMPRESSIONS_TO_CONVERSION_RATE")]
    METRICIMPRESSIONSTOCONVERSIONRATE,
    
    /// "METRIC_CONVERSIONS_PER_MILLE"
    #[serde(rename="METRIC_CONVERSIONS_PER_MILLE")]
    METRICCONVERSIONSPERMILLE,
    
    /// "METRIC_CTR"
    #[serde(rename="METRIC_CTR")]
    METRICCTR,
    
    /// "METRIC_BID_REQUESTS"
    #[serde(rename="METRIC_BID_REQUESTS")]
    METRICBIDREQUESTS,
    
    /// "METRIC_UNIQUE_VISITORS_COOKIES"
    #[serde(rename="METRIC_UNIQUE_VISITORS_COOKIES")]
    METRICUNIQUEVISITORSCOOKIES,
    
    /// "METRIC_REVENUE_ECPCV_ADVERTISER"
    #[serde(rename="METRIC_REVENUE_ECPCV_ADVERTISER")]
    METRICREVENUEECPCVADVERTISER,
    
    /// "METRIC_REVENUE_ECPCV_USD"
    #[serde(rename="METRIC_REVENUE_ECPCV_USD")]
    METRICREVENUEECPCVUSD,
    
    /// "METRIC_REVENUE_ECPCV_PARTNER"
    #[serde(rename="METRIC_REVENUE_ECPCV_PARTNER")]
    METRICREVENUEECPCVPARTNER,
    
    /// "METRIC_MEDIA_COST_ECPCV_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_COST_ECPCV_ADVERTISER")]
    METRICMEDIACOSTECPCVADVERTISER,
    
    /// "METRIC_MEDIA_COST_ECPCV_USD"
    #[serde(rename="METRIC_MEDIA_COST_ECPCV_USD")]
    METRICMEDIACOSTECPCVUSD,
    
    /// "METRIC_MEDIA_COST_ECPCV_PARTNER"
    #[serde(rename="METRIC_MEDIA_COST_ECPCV_PARTNER")]
    METRICMEDIACOSTECPCVPARTNER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPCV_ADVERTISER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPCV_ADVERTISER")]
    METRICTOTALMEDIACOSTECPCVADVERTISER,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPCV_USD"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPCV_USD")]
    METRICTOTALMEDIACOSTECPCVUSD,
    
    /// "METRIC_TOTAL_MEDIA_COST_ECPCV_PARTNER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_ECPCV_PARTNER")]
    METRICTOTALMEDIACOSTECPCVPARTNER,
    
    /// "METRIC_RICH_MEDIA_VIDEO_SKIPS"
    #[serde(rename="METRIC_RICH_MEDIA_VIDEO_SKIPS")]
    METRICRICHMEDIAVIDEOSKIPS,
    
    /// "METRIC_FEE2_ADVERTISER"
    #[serde(rename="METRIC_FEE2_ADVERTISER")]
    METRICFEE2ADVERTISER,
    
    /// "METRIC_FEE2_USD"
    #[serde(rename="METRIC_FEE2_USD")]
    METRICFEE2USD,
    
    /// "METRIC_FEE2_PARTNER"
    #[serde(rename="METRIC_FEE2_PARTNER")]
    METRICFEE2PARTNER,
    
    /// "METRIC_FEE3_ADVERTISER"
    #[serde(rename="METRIC_FEE3_ADVERTISER")]
    METRICFEE3ADVERTISER,
    
    /// "METRIC_FEE3_USD"
    #[serde(rename="METRIC_FEE3_USD")]
    METRICFEE3USD,
    
    /// "METRIC_FEE3_PARTNER"
    #[serde(rename="METRIC_FEE3_PARTNER")]
    METRICFEE3PARTNER,
    
    /// "METRIC_FEE4_ADVERTISER"
    #[serde(rename="METRIC_FEE4_ADVERTISER")]
    METRICFEE4ADVERTISER,
    
    /// "METRIC_FEE4_USD"
    #[serde(rename="METRIC_FEE4_USD")]
    METRICFEE4USD,
    
    /// "METRIC_FEE4_PARTNER"
    #[serde(rename="METRIC_FEE4_PARTNER")]
    METRICFEE4PARTNER,
    
    /// "METRIC_FEE5_ADVERTISER"
    #[serde(rename="METRIC_FEE5_ADVERTISER")]
    METRICFEE5ADVERTISER,
    
    /// "METRIC_FEE5_USD"
    #[serde(rename="METRIC_FEE5_USD")]
    METRICFEE5USD,
    
    /// "METRIC_FEE5_PARTNER"
    #[serde(rename="METRIC_FEE5_PARTNER")]
    METRICFEE5PARTNER,
    
    /// "METRIC_FEE6_ADVERTISER"
    #[serde(rename="METRIC_FEE6_ADVERTISER")]
    METRICFEE6ADVERTISER,
    
    /// "METRIC_FEE6_USD"
    #[serde(rename="METRIC_FEE6_USD")]
    METRICFEE6USD,
    
    /// "METRIC_FEE6_PARTNER"
    #[serde(rename="METRIC_FEE6_PARTNER")]
    METRICFEE6PARTNER,
    
    /// "METRIC_FEE7_ADVERTISER"
    #[serde(rename="METRIC_FEE7_ADVERTISER")]
    METRICFEE7ADVERTISER,
    
    /// "METRIC_FEE7_USD"
    #[serde(rename="METRIC_FEE7_USD")]
    METRICFEE7USD,
    
    /// "METRIC_FEE7_PARTNER"
    #[serde(rename="METRIC_FEE7_PARTNER")]
    METRICFEE7PARTNER,
    
    /// "METRIC_FEE8_ADVERTISER"
    #[serde(rename="METRIC_FEE8_ADVERTISER")]
    METRICFEE8ADVERTISER,
    
    /// "METRIC_FEE8_USD"
    #[serde(rename="METRIC_FEE8_USD")]
    METRICFEE8USD,
    
    /// "METRIC_FEE8_PARTNER"
    #[serde(rename="METRIC_FEE8_PARTNER")]
    METRICFEE8PARTNER,
    
    /// "METRIC_FEE9_ADVERTISER"
    #[serde(rename="METRIC_FEE9_ADVERTISER")]
    METRICFEE9ADVERTISER,
    
    /// "METRIC_FEE9_USD"
    #[serde(rename="METRIC_FEE9_USD")]
    METRICFEE9USD,
    
    /// "METRIC_FEE9_PARTNER"
    #[serde(rename="METRIC_FEE9_PARTNER")]
    METRICFEE9PARTNER,
    
    /// "METRIC_FEE10_ADVERTISER"
    #[serde(rename="METRIC_FEE10_ADVERTISER")]
    METRICFEE10ADVERTISER,
    
    /// "METRIC_FEE10_USD"
    #[serde(rename="METRIC_FEE10_USD")]
    METRICFEE10USD,
    
    /// "METRIC_FEE10_PARTNER"
    #[serde(rename="METRIC_FEE10_PARTNER")]
    METRICFEE10PARTNER,
    
    /// "METRIC_FEE11_ADVERTISER"
    #[serde(rename="METRIC_FEE11_ADVERTISER")]
    METRICFEE11ADVERTISER,
    
    /// "METRIC_FEE11_USD"
    #[serde(rename="METRIC_FEE11_USD")]
    METRICFEE11USD,
    
    /// "METRIC_FEE11_PARTNER"
    #[serde(rename="METRIC_FEE11_PARTNER")]
    METRICFEE11PARTNER,
    
    /// "METRIC_FEE12_ADVERTISER"
    #[serde(rename="METRIC_FEE12_ADVERTISER")]
    METRICFEE12ADVERTISER,
    
    /// "METRIC_FEE12_USD"
    #[serde(rename="METRIC_FEE12_USD")]
    METRICFEE12USD,
    
    /// "METRIC_FEE12_PARTNER"
    #[serde(rename="METRIC_FEE12_PARTNER")]
    METRICFEE12PARTNER,
    
    /// "METRIC_FEE13_ADVERTISER"
    #[serde(rename="METRIC_FEE13_ADVERTISER")]
    METRICFEE13ADVERTISER,
    
    /// "METRIC_FEE13_USD"
    #[serde(rename="METRIC_FEE13_USD")]
    METRICFEE13USD,
    
    /// "METRIC_FEE13_PARTNER"
    #[serde(rename="METRIC_FEE13_PARTNER")]
    METRICFEE13PARTNER,
    
    /// "METRIC_FEE14_ADVERTISER"
    #[serde(rename="METRIC_FEE14_ADVERTISER")]
    METRICFEE14ADVERTISER,
    
    /// "METRIC_FEE14_USD"
    #[serde(rename="METRIC_FEE14_USD")]
    METRICFEE14USD,
    
    /// "METRIC_FEE14_PARTNER"
    #[serde(rename="METRIC_FEE14_PARTNER")]
    METRICFEE14PARTNER,
    
    /// "METRIC_FEE15_ADVERTISER"
    #[serde(rename="METRIC_FEE15_ADVERTISER")]
    METRICFEE15ADVERTISER,
    
    /// "METRIC_FEE15_USD"
    #[serde(rename="METRIC_FEE15_USD")]
    METRICFEE15USD,
    
    /// "METRIC_FEE15_PARTNER"
    #[serde(rename="METRIC_FEE15_PARTNER")]
    METRICFEE15PARTNER,
    
    /// "METRIC_CPM_FEE3_ADVERTISER"
    #[serde(rename="METRIC_CPM_FEE3_ADVERTISER")]
    METRICCPMFEE3ADVERTISER,
    
    /// "METRIC_CPM_FEE3_USD"
    #[serde(rename="METRIC_CPM_FEE3_USD")]
    METRICCPMFEE3USD,
    
    /// "METRIC_CPM_FEE3_PARTNER"
    #[serde(rename="METRIC_CPM_FEE3_PARTNER")]
    METRICCPMFEE3PARTNER,
    
    /// "METRIC_CPM_FEE4_ADVERTISER"
    #[serde(rename="METRIC_CPM_FEE4_ADVERTISER")]
    METRICCPMFEE4ADVERTISER,
    
    /// "METRIC_CPM_FEE4_USD"
    #[serde(rename="METRIC_CPM_FEE4_USD")]
    METRICCPMFEE4USD,
    
    /// "METRIC_CPM_FEE4_PARTNER"
    #[serde(rename="METRIC_CPM_FEE4_PARTNER")]
    METRICCPMFEE4PARTNER,
    
    /// "METRIC_CPM_FEE5_ADVERTISER"
    #[serde(rename="METRIC_CPM_FEE5_ADVERTISER")]
    METRICCPMFEE5ADVERTISER,
    
    /// "METRIC_CPM_FEE5_USD"
    #[serde(rename="METRIC_CPM_FEE5_USD")]
    METRICCPMFEE5USD,
    
    /// "METRIC_CPM_FEE5_PARTNER"
    #[serde(rename="METRIC_CPM_FEE5_PARTNER")]
    METRICCPMFEE5PARTNER,
    
    /// "METRIC_MEDIA_FEE3_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_FEE3_ADVERTISER")]
    METRICMEDIAFEE3ADVERTISER,
    
    /// "METRIC_MEDIA_FEE3_USD"
    #[serde(rename="METRIC_MEDIA_FEE3_USD")]
    METRICMEDIAFEE3USD,
    
    /// "METRIC_MEDIA_FEE3_PARTNER"
    #[serde(rename="METRIC_MEDIA_FEE3_PARTNER")]
    METRICMEDIAFEE3PARTNER,
    
    /// "METRIC_MEDIA_FEE4_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_FEE4_ADVERTISER")]
    METRICMEDIAFEE4ADVERTISER,
    
    /// "METRIC_MEDIA_FEE4_USD"
    #[serde(rename="METRIC_MEDIA_FEE4_USD")]
    METRICMEDIAFEE4USD,
    
    /// "METRIC_MEDIA_FEE4_PARTNER"
    #[serde(rename="METRIC_MEDIA_FEE4_PARTNER")]
    METRICMEDIAFEE4PARTNER,
    
    /// "METRIC_MEDIA_FEE5_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_FEE5_ADVERTISER")]
    METRICMEDIAFEE5ADVERTISER,
    
    /// "METRIC_MEDIA_FEE5_USD"
    #[serde(rename="METRIC_MEDIA_FEE5_USD")]
    METRICMEDIAFEE5USD,
    
    /// "METRIC_MEDIA_FEE5_PARTNER"
    #[serde(rename="METRIC_MEDIA_FEE5_PARTNER")]
    METRICMEDIAFEE5PARTNER,
    
    /// "METRIC_VIDEO_COMPANION_IMPRESSIONS"
    #[serde(rename="METRIC_VIDEO_COMPANION_IMPRESSIONS")]
    METRICVIDEOCOMPANIONIMPRESSIONS,
    
    /// "METRIC_VIDEO_COMPANION_CLICKS"
    #[serde(rename="METRIC_VIDEO_COMPANION_CLICKS")]
    METRICVIDEOCOMPANIONCLICKS,
    
    /// "METRIC_FEE16_ADVERTISER"
    #[serde(rename="METRIC_FEE16_ADVERTISER")]
    METRICFEE16ADVERTISER,
    
    /// "METRIC_FEE16_USD"
    #[serde(rename="METRIC_FEE16_USD")]
    METRICFEE16USD,
    
    /// "METRIC_FEE16_PARTNER"
    #[serde(rename="METRIC_FEE16_PARTNER")]
    METRICFEE16PARTNER,
    
    /// "METRIC_FEE17_ADVERTISER"
    #[serde(rename="METRIC_FEE17_ADVERTISER")]
    METRICFEE17ADVERTISER,
    
    /// "METRIC_FEE17_USD"
    #[serde(rename="METRIC_FEE17_USD")]
    METRICFEE17USD,
    
    /// "METRIC_FEE17_PARTNER"
    #[serde(rename="METRIC_FEE17_PARTNER")]
    METRICFEE17PARTNER,
    
    /// "METRIC_FEE18_ADVERTISER"
    #[serde(rename="METRIC_FEE18_ADVERTISER")]
    METRICFEE18ADVERTISER,
    
    /// "METRIC_FEE18_USD"
    #[serde(rename="METRIC_FEE18_USD")]
    METRICFEE18USD,
    
    /// "METRIC_FEE18_PARTNER"
    #[serde(rename="METRIC_FEE18_PARTNER")]
    METRICFEE18PARTNER,
    
    /// "METRIC_TRUEVIEW_VIEWS"
    #[serde(rename="METRIC_TRUEVIEW_VIEWS")]
    METRICTRUEVIEWVIEWS,
    
    /// "METRIC_TRUEVIEW_UNIQUE_VIEWERS"
    #[serde(rename="METRIC_TRUEVIEW_UNIQUE_VIEWERS")]
    METRICTRUEVIEWUNIQUEVIEWERS,
    
    /// "METRIC_TRUEVIEW_EARNED_VIEWS"
    #[serde(rename="METRIC_TRUEVIEW_EARNED_VIEWS")]
    METRICTRUEVIEWEARNEDVIEWS,
    
    /// "METRIC_TRUEVIEW_EARNED_SUBSCRIBERS"
    #[serde(rename="METRIC_TRUEVIEW_EARNED_SUBSCRIBERS")]
    METRICTRUEVIEWEARNEDSUBSCRIBERS,
    
    /// "METRIC_TRUEVIEW_EARNED_PLAYLIST_ADDITIONS"
    #[serde(rename="METRIC_TRUEVIEW_EARNED_PLAYLIST_ADDITIONS")]
    METRICTRUEVIEWEARNEDPLAYLISTADDITIONS,
    
    /// "METRIC_TRUEVIEW_EARNED_LIKES"
    #[serde(rename="METRIC_TRUEVIEW_EARNED_LIKES")]
    METRICTRUEVIEWEARNEDLIKES,
    
    /// "METRIC_TRUEVIEW_EARNED_SHARES"
    #[serde(rename="METRIC_TRUEVIEW_EARNED_SHARES")]
    METRICTRUEVIEWEARNEDSHARES,
    
    /// "METRIC_TRUEVIEW_IMPRESSION_SHARE"
    #[serde(rename="METRIC_TRUEVIEW_IMPRESSION_SHARE")]
    METRICTRUEVIEWIMPRESSIONSHARE,
    
    /// "METRIC_TRUEVIEW_LOST_IS_BUDGET"
    #[serde(rename="METRIC_TRUEVIEW_LOST_IS_BUDGET")]
    METRICTRUEVIEWLOSTISBUDGET,
    
    /// "METRIC_TRUEVIEW_LOST_IS_RANK"
    #[serde(rename="METRIC_TRUEVIEW_LOST_IS_RANK")]
    METRICTRUEVIEWLOSTISRANK,
    
    /// "METRIC_TRUEVIEW_VIEW_THROUGH_CONVERSION"
    #[serde(rename="METRIC_TRUEVIEW_VIEW_THROUGH_CONVERSION")]
    METRICTRUEVIEWVIEWTHROUGHCONVERSION,
    
    /// "METRIC_TRUEVIEW_CONVERSION_MANY_PER_VIEW"
    #[serde(rename="METRIC_TRUEVIEW_CONVERSION_MANY_PER_VIEW")]
    METRICTRUEVIEWCONVERSIONMANYPERVIEW,
    
    /// "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUE"
    #[serde(rename="METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUE")]
    METRICTRUEVIEWTOTALCONVERSIONVALUE,
    
    /// "METRIC_TRUEVIEW_VIEW_RATE"
    #[serde(rename="METRIC_TRUEVIEW_VIEW_RATE")]
    METRICTRUEVIEWVIEWRATE,
    
    /// "METRIC_TRUEVIEW_CONVERSION_RATE_ONE_PER_VIEW"
    #[serde(rename="METRIC_TRUEVIEW_CONVERSION_RATE_ONE_PER_VIEW")]
    METRICTRUEVIEWCONVERSIONRATEONEPERVIEW,
    
    /// "METRIC_TRUEVIEW_CPV_ADVERTISER"
    #[serde(rename="METRIC_TRUEVIEW_CPV_ADVERTISER")]
    METRICTRUEVIEWCPVADVERTISER,
    
    /// "METRIC_TRUEVIEW_CPV_USD"
    #[serde(rename="METRIC_TRUEVIEW_CPV_USD")]
    METRICTRUEVIEWCPVUSD,
    
    /// "METRIC_TRUEVIEW_CPV_PARTNER"
    #[serde(rename="METRIC_TRUEVIEW_CPV_PARTNER")]
    METRICTRUEVIEWCPVPARTNER,
    
    /// "METRIC_FEE19_ADVERTISER"
    #[serde(rename="METRIC_FEE19_ADVERTISER")]
    METRICFEE19ADVERTISER,
    
    /// "METRIC_FEE19_USD"
    #[serde(rename="METRIC_FEE19_USD")]
    METRICFEE19USD,
    
    /// "METRIC_FEE19_PARTNER"
    #[serde(rename="METRIC_FEE19_PARTNER")]
    METRICFEE19PARTNER,
    
    /// "METRIC_TEA_TRUEVIEW_IMPRESSIONS"
    #[serde(rename="METRIC_TEA_TRUEVIEW_IMPRESSIONS")]
    METRICTEATRUEVIEWIMPRESSIONS,
    
    /// "METRIC_TEA_TRUEVIEW_UNIQUE_COOKIES"
    #[serde(rename="METRIC_TEA_TRUEVIEW_UNIQUE_COOKIES")]
    METRICTEATRUEVIEWUNIQUECOOKIES,
    
    /// "METRIC_FEE20_ADVERTISER"
    #[serde(rename="METRIC_FEE20_ADVERTISER")]
    METRICFEE20ADVERTISER,
    
    /// "METRIC_FEE20_USD"
    #[serde(rename="METRIC_FEE20_USD")]
    METRICFEE20USD,
    
    /// "METRIC_FEE20_PARTNER"
    #[serde(rename="METRIC_FEE20_PARTNER")]
    METRICFEE20PARTNER,
    
    /// "METRIC_FEE21_ADVERTISER"
    #[serde(rename="METRIC_FEE21_ADVERTISER")]
    METRICFEE21ADVERTISER,
    
    /// "METRIC_FEE21_USD"
    #[serde(rename="METRIC_FEE21_USD")]
    METRICFEE21USD,
    
    /// "METRIC_FEE21_PARTNER"
    #[serde(rename="METRIC_FEE21_PARTNER")]
    METRICFEE21PARTNER,
    
    /// "METRIC_FEE22_ADVERTISER"
    #[serde(rename="METRIC_FEE22_ADVERTISER")]
    METRICFEE22ADVERTISER,
    
    /// "METRIC_FEE22_USD"
    #[serde(rename="METRIC_FEE22_USD")]
    METRICFEE22USD,
    
    /// "METRIC_FEE22_PARTNER"
    #[serde(rename="METRIC_FEE22_PARTNER")]
    METRICFEE22PARTNER,
    
    /// "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_ADVERTISER"
    #[serde(rename="METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_ADVERTISER")]
    METRICTRUEVIEWTOTALCONVERSIONVALUESADVERTISER,
    
    /// "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_USD"
    #[serde(rename="METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_USD")]
    METRICTRUEVIEWTOTALCONVERSIONVALUESUSD,
    
    /// "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_PARTNER"
    #[serde(rename="METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_PARTNER")]
    METRICTRUEVIEWTOTALCONVERSIONVALUESPARTNER,
    
    /// "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_ADVERTISER"
    #[serde(rename="METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_ADVERTISER")]
    METRICTRUEVIEWCONVERSIONCOSTMANYPERVIEWADVERTISER,
    
    /// "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_USD"
    #[serde(rename="METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_USD")]
    METRICTRUEVIEWCONVERSIONCOSTMANYPERVIEWUSD,
    
    /// "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_PARTNER"
    #[serde(rename="METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_PARTNER")]
    METRICTRUEVIEWCONVERSIONCOSTMANYPERVIEWPARTNER,
    
    /// "METRIC_PROFIT_VIEWABLE_ECPM_ADVERTISER"
    #[serde(rename="METRIC_PROFIT_VIEWABLE_ECPM_ADVERTISER")]
    METRICPROFITVIEWABLEECPMADVERTISER,
    
    /// "METRIC_PROFIT_VIEWABLE_ECPM_USD"
    #[serde(rename="METRIC_PROFIT_VIEWABLE_ECPM_USD")]
    METRICPROFITVIEWABLEECPMUSD,
    
    /// "METRIC_PROFIT_VIEWABLE_ECPM_PARTNER"
    #[serde(rename="METRIC_PROFIT_VIEWABLE_ECPM_PARTNER")]
    METRICPROFITVIEWABLEECPMPARTNER,
    
    /// "METRIC_REVENUE_VIEWABLE_ECPM_ADVERTISER"
    #[serde(rename="METRIC_REVENUE_VIEWABLE_ECPM_ADVERTISER")]
    METRICREVENUEVIEWABLEECPMADVERTISER,
    
    /// "METRIC_REVENUE_VIEWABLE_ECPM_USD"
    #[serde(rename="METRIC_REVENUE_VIEWABLE_ECPM_USD")]
    METRICREVENUEVIEWABLEECPMUSD,
    
    /// "METRIC_REVENUE_VIEWABLE_ECPM_PARTNER"
    #[serde(rename="METRIC_REVENUE_VIEWABLE_ECPM_PARTNER")]
    METRICREVENUEVIEWABLEECPMPARTNER,
    
    /// "METRIC_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER"
    #[serde(rename="METRIC_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER")]
    METRICMEDIACOSTVIEWABLEECPMADVERTISER,
    
    /// "METRIC_MEDIA_COST_VIEWABLE_ECPM_USD"
    #[serde(rename="METRIC_MEDIA_COST_VIEWABLE_ECPM_USD")]
    METRICMEDIACOSTVIEWABLEECPMUSD,
    
    /// "METRIC_MEDIA_COST_VIEWABLE_ECPM_PARTNER"
    #[serde(rename="METRIC_MEDIA_COST_VIEWABLE_ECPM_PARTNER")]
    METRICMEDIACOSTVIEWABLEECPMPARTNER,
    
    /// "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER")]
    METRICTOTALMEDIACOSTVIEWABLEECPMADVERTISER,
    
    /// "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_USD"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_USD")]
    METRICTOTALMEDIACOSTVIEWABLEECPMUSD,
    
    /// "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_PARTNER"
    #[serde(rename="METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_PARTNER")]
    METRICTOTALMEDIACOSTVIEWABLEECPMPARTNER,
    
    /// "METRIC_TRUEVIEW_ENGAGEMENTS"
    #[serde(rename="METRIC_TRUEVIEW_ENGAGEMENTS")]
    METRICTRUEVIEWENGAGEMENTS,
    
    /// "METRIC_TRUEVIEW_ENGAGEMENT_RATE"
    #[serde(rename="METRIC_TRUEVIEW_ENGAGEMENT_RATE")]
    METRICTRUEVIEWENGAGEMENTRATE,
    
    /// "METRIC_TRUEVIEW_AVERAGE_CPE_ADVERTISER"
    #[serde(rename="METRIC_TRUEVIEW_AVERAGE_CPE_ADVERTISER")]
    METRICTRUEVIEWAVERAGECPEADVERTISER,
    
    /// "METRIC_TRUEVIEW_AVERAGE_CPE_USD"
    #[serde(rename="METRIC_TRUEVIEW_AVERAGE_CPE_USD")]
    METRICTRUEVIEWAVERAGECPEUSD,
    
    /// "METRIC_TRUEVIEW_AVERAGE_CPE_PARTNER"
    #[serde(rename="METRIC_TRUEVIEW_AVERAGE_CPE_PARTNER")]
    METRICTRUEVIEWAVERAGECPEPARTNER,
    
    /// "METRIC_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS"
    #[serde(rename="METRIC_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS")]
    METRICACTIVEVIEWVIEWABLEIMPRESSIONS,
    
    /// "METRIC_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS"
    #[serde(rename="METRIC_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS")]
    METRICACTIVEVIEWELIGIBLEIMPRESSIONS,
    
    /// "METRIC_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS"
    #[serde(rename="METRIC_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS")]
    METRICACTIVEVIEWMEASURABLEIMPRESSIONS,
    
    /// "METRIC_ACTIVE_VIEW_PCT_MEASURABLE_IMPRESSIONS"
    #[serde(rename="METRIC_ACTIVE_VIEW_PCT_MEASURABLE_IMPRESSIONS")]
    METRICACTIVEVIEWPCTMEASURABLEIMPRESSIONS,
    
    /// "METRIC_ACTIVE_VIEW_PCT_VIEWABLE_IMPRESSIONS"
    #[serde(rename="METRIC_ACTIVE_VIEW_PCT_VIEWABLE_IMPRESSIONS")]
    METRICACTIVEVIEWPCTVIEWABLEIMPRESSIONS,
    
    /// "METRIC_ACTIVE_VIEW_AVERAGE_VIEWABLE_TIME"
    #[serde(rename="METRIC_ACTIVE_VIEW_AVERAGE_VIEWABLE_TIME")]
    METRICACTIVEVIEWAVERAGEVIEWABLETIME,
    
    /// "METRIC_ACTIVE_VIEW_UNMEASURABLE_IMPRESSIONS"
    #[serde(rename="METRIC_ACTIVE_VIEW_UNMEASURABLE_IMPRESSIONS")]
    METRICACTIVEVIEWUNMEASURABLEIMPRESSIONS,
    
    /// "METRIC_ACTIVE_VIEW_UNVIEWABLE_IMPRESSIONS"
    #[serde(rename="METRIC_ACTIVE_VIEW_UNVIEWABLE_IMPRESSIONS")]
    METRICACTIVEVIEWUNVIEWABLEIMPRESSIONS,
    
    /// "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNMEASURABLE"
    #[serde(rename="METRIC_ACTIVE_VIEW_DISTRIBUTION_UNMEASURABLE")]
    METRICACTIVEVIEWDISTRIBUTIONUNMEASURABLE,
    
    /// "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNVIEWABLE"
    #[serde(rename="METRIC_ACTIVE_VIEW_DISTRIBUTION_UNVIEWABLE")]
    METRICACTIVEVIEWDISTRIBUTIONUNVIEWABLE,
    
    /// "METRIC_ACTIVE_VIEW_DISTRIBUTION_VIEWABLE"
    #[serde(rename="METRIC_ACTIVE_VIEW_DISTRIBUTION_VIEWABLE")]
    METRICACTIVEVIEWDISTRIBUTIONVIEWABLE,
    
    /// "METRIC_ACTIVE_VIEW_PERCENT_VIEWABLE_FOR_TIME_THRESHOLD"
    #[serde(rename="METRIC_ACTIVE_VIEW_PERCENT_VIEWABLE_FOR_TIME_THRESHOLD")]
    METRICACTIVEVIEWPERCENTVIEWABLEFORTIMETHRESHOLD,
    
    /// "METRIC_ACTIVE_VIEW_VIEWABLE_FOR_TIME_THRESHOLD"
    #[serde(rename="METRIC_ACTIVE_VIEW_VIEWABLE_FOR_TIME_THRESHOLD")]
    METRICACTIVEVIEWVIEWABLEFORTIMETHRESHOLD,
    
    /// "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_AT_START"
    #[serde(rename="METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_AT_START")]
    METRICACTIVEVIEWPERCENTVISIBLEATSTART,
    
    /// "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_FIRST_QUAR"
    #[serde(rename="METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_FIRST_QUAR")]
    METRICACTIVEVIEWPERCENTVISIBLEFIRSTQUAR,
    
    /// "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_SECOND_QUAR"
    #[serde(rename="METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_SECOND_QUAR")]
    METRICACTIVEVIEWPERCENTVISIBLESECONDQUAR,
    
    /// "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_THIRD_QUAR"
    #[serde(rename="METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_THIRD_QUAR")]
    METRICACTIVEVIEWPERCENTVISIBLETHIRDQUAR,
    
    /// "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_ON_COMPLETE"
    #[serde(rename="METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_ON_COMPLETE")]
    METRICACTIVEVIEWPERCENTVISIBLEONCOMPLETE,
    
    /// "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_AT_START"
    #[serde(rename="METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_AT_START")]
    METRICACTIVEVIEWPERCENTAUDIBLEVISIBLEATSTART,
    
    /// "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_FIRST_QUAR"
    #[serde(rename="METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_FIRST_QUAR")]
    METRICACTIVEVIEWPERCENTAUDIBLEVISIBLEFIRSTQUAR,
    
    /// "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_SECOND_QUAR"
    #[serde(rename="METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_SECOND_QUAR")]
    METRICACTIVEVIEWPERCENTAUDIBLEVISIBLESECONDQUAR,
    
    /// "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_THIRD_QUAR"
    #[serde(rename="METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_THIRD_QUAR")]
    METRICACTIVEVIEWPERCENTAUDIBLEVISIBLETHIRDQUAR,
    
    /// "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_ON_COMPLETE"
    #[serde(rename="METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_ON_COMPLETE")]
    METRICACTIVEVIEWPERCENTAUDIBLEVISIBLEONCOMPLETE,
    
    /// "METRIC_ACTIVE_VIEW_AUDIBLE_VISIBLE_ON_COMPLETE_IMPRESSIONS"
    #[serde(rename="METRIC_ACTIVE_VIEW_AUDIBLE_VISIBLE_ON_COMPLETE_IMPRESSIONS")]
    METRICACTIVEVIEWAUDIBLEVISIBLEONCOMPLETEIMPRESSIONS,
    
    /// "METRIC_VIEWABLE_BID_REQUESTS"
    #[serde(rename="METRIC_VIEWABLE_BID_REQUESTS")]
    METRICVIEWABLEBIDREQUESTS,
    
    /// "METRIC_COOKIE_REACH_IMPRESSION_REACH"
    #[serde(rename="METRIC_COOKIE_REACH_IMPRESSION_REACH")]
    METRICCOOKIEREACHIMPRESSIONREACH,
    
    /// "METRIC_COOKIE_REACH_AVERAGE_IMPRESSION_FREQUENCY"
    #[serde(rename="METRIC_COOKIE_REACH_AVERAGE_IMPRESSION_FREQUENCY")]
    METRICCOOKIEREACHAVERAGEIMPRESSIONFREQUENCY,
    
    /// "METRIC_DBM_ENGAGEMENT_RATE"
    #[serde(rename="METRIC_DBM_ENGAGEMENT_RATE")]
    METRICDBMENGAGEMENTRATE,
    
    /// "METRIC_RICH_MEDIA_SCROLLS"
    #[serde(rename="METRIC_RICH_MEDIA_SCROLLS")]
    METRICRICHMEDIASCROLLS,
    
    /// "METRIC_CM_POST_VIEW_REVENUE"
    #[serde(rename="METRIC_CM_POST_VIEW_REVENUE")]
    METRICCMPOSTVIEWREVENUE,
    
    /// "METRIC_CM_POST_CLICK_REVENUE"
    #[serde(rename="METRIC_CM_POST_CLICK_REVENUE")]
    METRICCMPOSTCLICKREVENUE,
    
    /// "METRIC_FLOODLIGHT_IMPRESSIONS"
    #[serde(rename="METRIC_FLOODLIGHT_IMPRESSIONS")]
    METRICFLOODLIGHTIMPRESSIONS,
    
    /// "METRIC_BILLABLE_IMPRESSIONS"
    #[serde(rename="METRIC_BILLABLE_IMPRESSIONS")]
    METRICBILLABLEIMPRESSIONS,
}

impl AsRef<str> for ParameterMetricsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParameterMetricsEnum::METRICUNKNOWN => "METRIC_UNKNOWN",
            ParameterMetricsEnum::METRICIMPRESSIONS => "METRIC_IMPRESSIONS",
            ParameterMetricsEnum::METRICCLICKS => "METRIC_CLICKS",
            ParameterMetricsEnum::METRICLASTIMPRESSIONS => "METRIC_LAST_IMPRESSIONS",
            ParameterMetricsEnum::METRICLASTCLICKS => "METRIC_LAST_CLICKS",
            ParameterMetricsEnum::METRICTOTALCONVERSIONS => "METRIC_TOTAL_CONVERSIONS",
            ParameterMetricsEnum::METRICMEDIACOSTADVERTISER => "METRIC_MEDIA_COST_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIACOSTUSD => "METRIC_MEDIA_COST_USD",
            ParameterMetricsEnum::METRICMEDIACOSTPARTNER => "METRIC_MEDIA_COST_PARTNER",
            ParameterMetricsEnum::METRICDATACOSTADVERTISER => "METRIC_DATA_COST_ADVERTISER",
            ParameterMetricsEnum::METRICDATACOSTUSD => "METRIC_DATA_COST_USD",
            ParameterMetricsEnum::METRICDATACOSTPARTNER => "METRIC_DATA_COST_PARTNER",
            ParameterMetricsEnum::METRICCPMFEE1ADVERTISER => "METRIC_CPM_FEE1_ADVERTISER",
            ParameterMetricsEnum::METRICCPMFEE1USD => "METRIC_CPM_FEE1_USD",
            ParameterMetricsEnum::METRICCPMFEE1PARTNER => "METRIC_CPM_FEE1_PARTNER",
            ParameterMetricsEnum::METRICCPMFEE2ADVERTISER => "METRIC_CPM_FEE2_ADVERTISER",
            ParameterMetricsEnum::METRICCPMFEE2USD => "METRIC_CPM_FEE2_USD",
            ParameterMetricsEnum::METRICCPMFEE2PARTNER => "METRIC_CPM_FEE2_PARTNER",
            ParameterMetricsEnum::METRICMEDIAFEE1ADVERTISER => "METRIC_MEDIA_FEE1_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIAFEE1USD => "METRIC_MEDIA_FEE1_USD",
            ParameterMetricsEnum::METRICMEDIAFEE1PARTNER => "METRIC_MEDIA_FEE1_PARTNER",
            ParameterMetricsEnum::METRICMEDIAFEE2ADVERTISER => "METRIC_MEDIA_FEE2_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIAFEE2USD => "METRIC_MEDIA_FEE2_USD",
            ParameterMetricsEnum::METRICMEDIAFEE2PARTNER => "METRIC_MEDIA_FEE2_PARTNER",
            ParameterMetricsEnum::METRICREVENUEADVERTISER => "METRIC_REVENUE_ADVERTISER",
            ParameterMetricsEnum::METRICREVENUEUSD => "METRIC_REVENUE_USD",
            ParameterMetricsEnum::METRICREVENUEPARTNER => "METRIC_REVENUE_PARTNER",
            ParameterMetricsEnum::METRICPROFITADVERTISER => "METRIC_PROFIT_ADVERTISER",
            ParameterMetricsEnum::METRICPROFITUSD => "METRIC_PROFIT_USD",
            ParameterMetricsEnum::METRICPROFITPARTNER => "METRIC_PROFIT_PARTNER",
            ParameterMetricsEnum::METRICPROFITMARGIN => "METRIC_PROFIT_MARGIN",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTUSD => "METRIC_TOTAL_MEDIA_COST_USD",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTPARTNER => "METRIC_TOTAL_MEDIA_COST_PARTNER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTADVERTISER => "METRIC_TOTAL_MEDIA_COST_ADVERTISER",
            ParameterMetricsEnum::METRICBILLABLECOSTUSD => "METRIC_BILLABLE_COST_USD",
            ParameterMetricsEnum::METRICBILLABLECOSTPARTNER => "METRIC_BILLABLE_COST_PARTNER",
            ParameterMetricsEnum::METRICBILLABLECOSTADVERTISER => "METRIC_BILLABLE_COST_ADVERTISER",
            ParameterMetricsEnum::METRICPLATFORMFEEUSD => "METRIC_PLATFORM_FEE_USD",
            ParameterMetricsEnum::METRICPLATFORMFEEPARTNER => "METRIC_PLATFORM_FEE_PARTNER",
            ParameterMetricsEnum::METRICPLATFORMFEEADVERTISER => "METRIC_PLATFORM_FEE_ADVERTISER",
            ParameterMetricsEnum::METRICVIDEOCOMPLETIONRATE => "METRIC_VIDEO_COMPLETION_RATE",
            ParameterMetricsEnum::METRICPROFITECPMADVERTISER => "METRIC_PROFIT_ECPM_ADVERTISER",
            ParameterMetricsEnum::METRICPROFITECPMUSD => "METRIC_PROFIT_ECPM_USD",
            ParameterMetricsEnum::METRICPROFITECPMPARTNER => "METRIC_PROFIT_ECPM_PARTNER",
            ParameterMetricsEnum::METRICREVENUEECPMADVERTISER => "METRIC_REVENUE_ECPM_ADVERTISER",
            ParameterMetricsEnum::METRICREVENUEECPMUSD => "METRIC_REVENUE_ECPM_USD",
            ParameterMetricsEnum::METRICREVENUEECPMPARTNER => "METRIC_REVENUE_ECPM_PARTNER",
            ParameterMetricsEnum::METRICREVENUEECPCADVERTISER => "METRIC_REVENUE_ECPC_ADVERTISER",
            ParameterMetricsEnum::METRICREVENUEECPCUSD => "METRIC_REVENUE_ECPC_USD",
            ParameterMetricsEnum::METRICREVENUEECPCPARTNER => "METRIC_REVENUE_ECPC_PARTNER",
            ParameterMetricsEnum::METRICREVENUEECPAADVERTISER => "METRIC_REVENUE_ECPA_ADVERTISER",
            ParameterMetricsEnum::METRICREVENUEECPAUSD => "METRIC_REVENUE_ECPA_USD",
            ParameterMetricsEnum::METRICREVENUEECPAPARTNER => "METRIC_REVENUE_ECPA_PARTNER",
            ParameterMetricsEnum::METRICREVENUEECPAPVADVERTISER => "METRIC_REVENUE_ECPAPV_ADVERTISER",
            ParameterMetricsEnum::METRICREVENUEECPAPVUSD => "METRIC_REVENUE_ECPAPV_USD",
            ParameterMetricsEnum::METRICREVENUEECPAPVPARTNER => "METRIC_REVENUE_ECPAPV_PARTNER",
            ParameterMetricsEnum::METRICREVENUEECPAPCADVERTISER => "METRIC_REVENUE_ECPAPC_ADVERTISER",
            ParameterMetricsEnum::METRICREVENUEECPAPCUSD => "METRIC_REVENUE_ECPAPC_USD",
            ParameterMetricsEnum::METRICREVENUEECPAPCPARTNER => "METRIC_REVENUE_ECPAPC_PARTNER",
            ParameterMetricsEnum::METRICMEDIACOSTECPMADVERTISER => "METRIC_MEDIA_COST_ECPM_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIACOSTECPMUSD => "METRIC_MEDIA_COST_ECPM_USD",
            ParameterMetricsEnum::METRICMEDIACOSTECPMPARTNER => "METRIC_MEDIA_COST_ECPM_PARTNER",
            ParameterMetricsEnum::METRICMEDIACOSTECPCADVERTISER => "METRIC_MEDIA_COST_ECPC_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIACOSTECPCUSD => "METRIC_MEDIA_COST_ECPC_USD",
            ParameterMetricsEnum::METRICMEDIACOSTECPCPARTNER => "METRIC_MEDIA_COST_ECPC_PARTNER",
            ParameterMetricsEnum::METRICMEDIACOSTECPAADVERTISER => "METRIC_MEDIA_COST_ECPA_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIACOSTECPAUSD => "METRIC_MEDIA_COST_ECPA_USD",
            ParameterMetricsEnum::METRICMEDIACOSTECPAPARTNER => "METRIC_MEDIA_COST_ECPA_PARTNER",
            ParameterMetricsEnum::METRICMEDIACOSTECPAPVADVERTISER => "METRIC_MEDIA_COST_ECPAPV_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIACOSTECPAPVUSD => "METRIC_MEDIA_COST_ECPAPV_USD",
            ParameterMetricsEnum::METRICMEDIACOSTECPAPVPARTNER => "METRIC_MEDIA_COST_ECPAPV_PARTNER",
            ParameterMetricsEnum::METRICMEDIACOSTECPAPCADVERTISER => "METRIC_MEDIA_COST_ECPAPC_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIACOSTECPAPCUSD => "METRIC_MEDIA_COST_ECPAPC_USD",
            ParameterMetricsEnum::METRICMEDIACOSTECPAPCPARTNER => "METRIC_MEDIA_COST_ECPAPC_PARTNER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPMADVERTISER => "METRIC_TOTAL_MEDIA_COST_ECPM_ADVERTISER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPMUSD => "METRIC_TOTAL_MEDIA_COST_ECPM_USD",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPMPARTNER => "METRIC_TOTAL_MEDIA_COST_ECPM_PARTNER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCADVERTISER => "METRIC_TOTAL_MEDIA_COST_ECPC_ADVERTISER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCUSD => "METRIC_TOTAL_MEDIA_COST_ECPC_USD",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCPARTNER => "METRIC_TOTAL_MEDIA_COST_ECPC_PARTNER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAADVERTISER => "METRIC_TOTAL_MEDIA_COST_ECPA_ADVERTISER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAUSD => "METRIC_TOTAL_MEDIA_COST_ECPA_USD",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPARTNER => "METRIC_TOTAL_MEDIA_COST_ECPA_PARTNER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPVADVERTISER => "METRIC_TOTAL_MEDIA_COST_ECPAPV_ADVERTISER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPVUSD => "METRIC_TOTAL_MEDIA_COST_ECPAPV_USD",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPVPARTNER => "METRIC_TOTAL_MEDIA_COST_ECPAPV_PARTNER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPCADVERTISER => "METRIC_TOTAL_MEDIA_COST_ECPAPC_ADVERTISER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPCUSD => "METRIC_TOTAL_MEDIA_COST_ECPAPC_USD",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPCPARTNER => "METRIC_TOTAL_MEDIA_COST_ECPAPC_PARTNER",
            ParameterMetricsEnum::METRICRICHMEDIAVIDEOPLAYS => "METRIC_RICH_MEDIA_VIDEO_PLAYS",
            ParameterMetricsEnum::METRICRICHMEDIAVIDEOCOMPLETIONS => "METRIC_RICH_MEDIA_VIDEO_COMPLETIONS",
            ParameterMetricsEnum::METRICRICHMEDIAVIDEOPAUSES => "METRIC_RICH_MEDIA_VIDEO_PAUSES",
            ParameterMetricsEnum::METRICRICHMEDIAVIDEOMUTES => "METRIC_RICH_MEDIA_VIDEO_MUTES",
            ParameterMetricsEnum::METRICRICHMEDIAVIDEOMIDPOINTS => "METRIC_RICH_MEDIA_VIDEO_MIDPOINTS",
            ParameterMetricsEnum::METRICRICHMEDIAVIDEOFULLSCREENS => "METRIC_RICH_MEDIA_VIDEO_FULL_SCREENS",
            ParameterMetricsEnum::METRICRICHMEDIAVIDEOFIRSTQUARTILECOMPLETES => "METRIC_RICH_MEDIA_VIDEO_FIRST_QUARTILE_COMPLETES",
            ParameterMetricsEnum::METRICRICHMEDIAVIDEOTHIRDQUARTILECOMPLETES => "METRIC_RICH_MEDIA_VIDEO_THIRD_QUARTILE_COMPLETES",
            ParameterMetricsEnum::METRICCLICKTOPOSTCLICKCONVERSIONRATE => "METRIC_CLICK_TO_POST_CLICK_CONVERSION_RATE",
            ParameterMetricsEnum::METRICIMPRESSIONSTOCONVERSIONRATE => "METRIC_IMPRESSIONS_TO_CONVERSION_RATE",
            ParameterMetricsEnum::METRICCONVERSIONSPERMILLE => "METRIC_CONVERSIONS_PER_MILLE",
            ParameterMetricsEnum::METRICCTR => "METRIC_CTR",
            ParameterMetricsEnum::METRICBIDREQUESTS => "METRIC_BID_REQUESTS",
            ParameterMetricsEnum::METRICUNIQUEVISITORSCOOKIES => "METRIC_UNIQUE_VISITORS_COOKIES",
            ParameterMetricsEnum::METRICREVENUEECPCVADVERTISER => "METRIC_REVENUE_ECPCV_ADVERTISER",
            ParameterMetricsEnum::METRICREVENUEECPCVUSD => "METRIC_REVENUE_ECPCV_USD",
            ParameterMetricsEnum::METRICREVENUEECPCVPARTNER => "METRIC_REVENUE_ECPCV_PARTNER",
            ParameterMetricsEnum::METRICMEDIACOSTECPCVADVERTISER => "METRIC_MEDIA_COST_ECPCV_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIACOSTECPCVUSD => "METRIC_MEDIA_COST_ECPCV_USD",
            ParameterMetricsEnum::METRICMEDIACOSTECPCVPARTNER => "METRIC_MEDIA_COST_ECPCV_PARTNER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCVADVERTISER => "METRIC_TOTAL_MEDIA_COST_ECPCV_ADVERTISER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCVUSD => "METRIC_TOTAL_MEDIA_COST_ECPCV_USD",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCVPARTNER => "METRIC_TOTAL_MEDIA_COST_ECPCV_PARTNER",
            ParameterMetricsEnum::METRICRICHMEDIAVIDEOSKIPS => "METRIC_RICH_MEDIA_VIDEO_SKIPS",
            ParameterMetricsEnum::METRICFEE2ADVERTISER => "METRIC_FEE2_ADVERTISER",
            ParameterMetricsEnum::METRICFEE2USD => "METRIC_FEE2_USD",
            ParameterMetricsEnum::METRICFEE2PARTNER => "METRIC_FEE2_PARTNER",
            ParameterMetricsEnum::METRICFEE3ADVERTISER => "METRIC_FEE3_ADVERTISER",
            ParameterMetricsEnum::METRICFEE3USD => "METRIC_FEE3_USD",
            ParameterMetricsEnum::METRICFEE3PARTNER => "METRIC_FEE3_PARTNER",
            ParameterMetricsEnum::METRICFEE4ADVERTISER => "METRIC_FEE4_ADVERTISER",
            ParameterMetricsEnum::METRICFEE4USD => "METRIC_FEE4_USD",
            ParameterMetricsEnum::METRICFEE4PARTNER => "METRIC_FEE4_PARTNER",
            ParameterMetricsEnum::METRICFEE5ADVERTISER => "METRIC_FEE5_ADVERTISER",
            ParameterMetricsEnum::METRICFEE5USD => "METRIC_FEE5_USD",
            ParameterMetricsEnum::METRICFEE5PARTNER => "METRIC_FEE5_PARTNER",
            ParameterMetricsEnum::METRICFEE6ADVERTISER => "METRIC_FEE6_ADVERTISER",
            ParameterMetricsEnum::METRICFEE6USD => "METRIC_FEE6_USD",
            ParameterMetricsEnum::METRICFEE6PARTNER => "METRIC_FEE6_PARTNER",
            ParameterMetricsEnum::METRICFEE7ADVERTISER => "METRIC_FEE7_ADVERTISER",
            ParameterMetricsEnum::METRICFEE7USD => "METRIC_FEE7_USD",
            ParameterMetricsEnum::METRICFEE7PARTNER => "METRIC_FEE7_PARTNER",
            ParameterMetricsEnum::METRICFEE8ADVERTISER => "METRIC_FEE8_ADVERTISER",
            ParameterMetricsEnum::METRICFEE8USD => "METRIC_FEE8_USD",
            ParameterMetricsEnum::METRICFEE8PARTNER => "METRIC_FEE8_PARTNER",
            ParameterMetricsEnum::METRICFEE9ADVERTISER => "METRIC_FEE9_ADVERTISER",
            ParameterMetricsEnum::METRICFEE9USD => "METRIC_FEE9_USD",
            ParameterMetricsEnum::METRICFEE9PARTNER => "METRIC_FEE9_PARTNER",
            ParameterMetricsEnum::METRICFEE10ADVERTISER => "METRIC_FEE10_ADVERTISER",
            ParameterMetricsEnum::METRICFEE10USD => "METRIC_FEE10_USD",
            ParameterMetricsEnum::METRICFEE10PARTNER => "METRIC_FEE10_PARTNER",
            ParameterMetricsEnum::METRICFEE11ADVERTISER => "METRIC_FEE11_ADVERTISER",
            ParameterMetricsEnum::METRICFEE11USD => "METRIC_FEE11_USD",
            ParameterMetricsEnum::METRICFEE11PARTNER => "METRIC_FEE11_PARTNER",
            ParameterMetricsEnum::METRICFEE12ADVERTISER => "METRIC_FEE12_ADVERTISER",
            ParameterMetricsEnum::METRICFEE12USD => "METRIC_FEE12_USD",
            ParameterMetricsEnum::METRICFEE12PARTNER => "METRIC_FEE12_PARTNER",
            ParameterMetricsEnum::METRICFEE13ADVERTISER => "METRIC_FEE13_ADVERTISER",
            ParameterMetricsEnum::METRICFEE13USD => "METRIC_FEE13_USD",
            ParameterMetricsEnum::METRICFEE13PARTNER => "METRIC_FEE13_PARTNER",
            ParameterMetricsEnum::METRICFEE14ADVERTISER => "METRIC_FEE14_ADVERTISER",
            ParameterMetricsEnum::METRICFEE14USD => "METRIC_FEE14_USD",
            ParameterMetricsEnum::METRICFEE14PARTNER => "METRIC_FEE14_PARTNER",
            ParameterMetricsEnum::METRICFEE15ADVERTISER => "METRIC_FEE15_ADVERTISER",
            ParameterMetricsEnum::METRICFEE15USD => "METRIC_FEE15_USD",
            ParameterMetricsEnum::METRICFEE15PARTNER => "METRIC_FEE15_PARTNER",
            ParameterMetricsEnum::METRICCPMFEE3ADVERTISER => "METRIC_CPM_FEE3_ADVERTISER",
            ParameterMetricsEnum::METRICCPMFEE3USD => "METRIC_CPM_FEE3_USD",
            ParameterMetricsEnum::METRICCPMFEE3PARTNER => "METRIC_CPM_FEE3_PARTNER",
            ParameterMetricsEnum::METRICCPMFEE4ADVERTISER => "METRIC_CPM_FEE4_ADVERTISER",
            ParameterMetricsEnum::METRICCPMFEE4USD => "METRIC_CPM_FEE4_USD",
            ParameterMetricsEnum::METRICCPMFEE4PARTNER => "METRIC_CPM_FEE4_PARTNER",
            ParameterMetricsEnum::METRICCPMFEE5ADVERTISER => "METRIC_CPM_FEE5_ADVERTISER",
            ParameterMetricsEnum::METRICCPMFEE5USD => "METRIC_CPM_FEE5_USD",
            ParameterMetricsEnum::METRICCPMFEE5PARTNER => "METRIC_CPM_FEE5_PARTNER",
            ParameterMetricsEnum::METRICMEDIAFEE3ADVERTISER => "METRIC_MEDIA_FEE3_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIAFEE3USD => "METRIC_MEDIA_FEE3_USD",
            ParameterMetricsEnum::METRICMEDIAFEE3PARTNER => "METRIC_MEDIA_FEE3_PARTNER",
            ParameterMetricsEnum::METRICMEDIAFEE4ADVERTISER => "METRIC_MEDIA_FEE4_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIAFEE4USD => "METRIC_MEDIA_FEE4_USD",
            ParameterMetricsEnum::METRICMEDIAFEE4PARTNER => "METRIC_MEDIA_FEE4_PARTNER",
            ParameterMetricsEnum::METRICMEDIAFEE5ADVERTISER => "METRIC_MEDIA_FEE5_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIAFEE5USD => "METRIC_MEDIA_FEE5_USD",
            ParameterMetricsEnum::METRICMEDIAFEE5PARTNER => "METRIC_MEDIA_FEE5_PARTNER",
            ParameterMetricsEnum::METRICVIDEOCOMPANIONIMPRESSIONS => "METRIC_VIDEO_COMPANION_IMPRESSIONS",
            ParameterMetricsEnum::METRICVIDEOCOMPANIONCLICKS => "METRIC_VIDEO_COMPANION_CLICKS",
            ParameterMetricsEnum::METRICFEE16ADVERTISER => "METRIC_FEE16_ADVERTISER",
            ParameterMetricsEnum::METRICFEE16USD => "METRIC_FEE16_USD",
            ParameterMetricsEnum::METRICFEE16PARTNER => "METRIC_FEE16_PARTNER",
            ParameterMetricsEnum::METRICFEE17ADVERTISER => "METRIC_FEE17_ADVERTISER",
            ParameterMetricsEnum::METRICFEE17USD => "METRIC_FEE17_USD",
            ParameterMetricsEnum::METRICFEE17PARTNER => "METRIC_FEE17_PARTNER",
            ParameterMetricsEnum::METRICFEE18ADVERTISER => "METRIC_FEE18_ADVERTISER",
            ParameterMetricsEnum::METRICFEE18USD => "METRIC_FEE18_USD",
            ParameterMetricsEnum::METRICFEE18PARTNER => "METRIC_FEE18_PARTNER",
            ParameterMetricsEnum::METRICTRUEVIEWVIEWS => "METRIC_TRUEVIEW_VIEWS",
            ParameterMetricsEnum::METRICTRUEVIEWUNIQUEVIEWERS => "METRIC_TRUEVIEW_UNIQUE_VIEWERS",
            ParameterMetricsEnum::METRICTRUEVIEWEARNEDVIEWS => "METRIC_TRUEVIEW_EARNED_VIEWS",
            ParameterMetricsEnum::METRICTRUEVIEWEARNEDSUBSCRIBERS => "METRIC_TRUEVIEW_EARNED_SUBSCRIBERS",
            ParameterMetricsEnum::METRICTRUEVIEWEARNEDPLAYLISTADDITIONS => "METRIC_TRUEVIEW_EARNED_PLAYLIST_ADDITIONS",
            ParameterMetricsEnum::METRICTRUEVIEWEARNEDLIKES => "METRIC_TRUEVIEW_EARNED_LIKES",
            ParameterMetricsEnum::METRICTRUEVIEWEARNEDSHARES => "METRIC_TRUEVIEW_EARNED_SHARES",
            ParameterMetricsEnum::METRICTRUEVIEWIMPRESSIONSHARE => "METRIC_TRUEVIEW_IMPRESSION_SHARE",
            ParameterMetricsEnum::METRICTRUEVIEWLOSTISBUDGET => "METRIC_TRUEVIEW_LOST_IS_BUDGET",
            ParameterMetricsEnum::METRICTRUEVIEWLOSTISRANK => "METRIC_TRUEVIEW_LOST_IS_RANK",
            ParameterMetricsEnum::METRICTRUEVIEWVIEWTHROUGHCONVERSION => "METRIC_TRUEVIEW_VIEW_THROUGH_CONVERSION",
            ParameterMetricsEnum::METRICTRUEVIEWCONVERSIONMANYPERVIEW => "METRIC_TRUEVIEW_CONVERSION_MANY_PER_VIEW",
            ParameterMetricsEnum::METRICTRUEVIEWTOTALCONVERSIONVALUE => "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUE",
            ParameterMetricsEnum::METRICTRUEVIEWVIEWRATE => "METRIC_TRUEVIEW_VIEW_RATE",
            ParameterMetricsEnum::METRICTRUEVIEWCONVERSIONRATEONEPERVIEW => "METRIC_TRUEVIEW_CONVERSION_RATE_ONE_PER_VIEW",
            ParameterMetricsEnum::METRICTRUEVIEWCPVADVERTISER => "METRIC_TRUEVIEW_CPV_ADVERTISER",
            ParameterMetricsEnum::METRICTRUEVIEWCPVUSD => "METRIC_TRUEVIEW_CPV_USD",
            ParameterMetricsEnum::METRICTRUEVIEWCPVPARTNER => "METRIC_TRUEVIEW_CPV_PARTNER",
            ParameterMetricsEnum::METRICFEE19ADVERTISER => "METRIC_FEE19_ADVERTISER",
            ParameterMetricsEnum::METRICFEE19USD => "METRIC_FEE19_USD",
            ParameterMetricsEnum::METRICFEE19PARTNER => "METRIC_FEE19_PARTNER",
            ParameterMetricsEnum::METRICTEATRUEVIEWIMPRESSIONS => "METRIC_TEA_TRUEVIEW_IMPRESSIONS",
            ParameterMetricsEnum::METRICTEATRUEVIEWUNIQUECOOKIES => "METRIC_TEA_TRUEVIEW_UNIQUE_COOKIES",
            ParameterMetricsEnum::METRICFEE20ADVERTISER => "METRIC_FEE20_ADVERTISER",
            ParameterMetricsEnum::METRICFEE20USD => "METRIC_FEE20_USD",
            ParameterMetricsEnum::METRICFEE20PARTNER => "METRIC_FEE20_PARTNER",
            ParameterMetricsEnum::METRICFEE21ADVERTISER => "METRIC_FEE21_ADVERTISER",
            ParameterMetricsEnum::METRICFEE21USD => "METRIC_FEE21_USD",
            ParameterMetricsEnum::METRICFEE21PARTNER => "METRIC_FEE21_PARTNER",
            ParameterMetricsEnum::METRICFEE22ADVERTISER => "METRIC_FEE22_ADVERTISER",
            ParameterMetricsEnum::METRICFEE22USD => "METRIC_FEE22_USD",
            ParameterMetricsEnum::METRICFEE22PARTNER => "METRIC_FEE22_PARTNER",
            ParameterMetricsEnum::METRICTRUEVIEWTOTALCONVERSIONVALUESADVERTISER => "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_ADVERTISER",
            ParameterMetricsEnum::METRICTRUEVIEWTOTALCONVERSIONVALUESUSD => "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_USD",
            ParameterMetricsEnum::METRICTRUEVIEWTOTALCONVERSIONVALUESPARTNER => "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_PARTNER",
            ParameterMetricsEnum::METRICTRUEVIEWCONVERSIONCOSTMANYPERVIEWADVERTISER => "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_ADVERTISER",
            ParameterMetricsEnum::METRICTRUEVIEWCONVERSIONCOSTMANYPERVIEWUSD => "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_USD",
            ParameterMetricsEnum::METRICTRUEVIEWCONVERSIONCOSTMANYPERVIEWPARTNER => "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_PARTNER",
            ParameterMetricsEnum::METRICPROFITVIEWABLEECPMADVERTISER => "METRIC_PROFIT_VIEWABLE_ECPM_ADVERTISER",
            ParameterMetricsEnum::METRICPROFITVIEWABLEECPMUSD => "METRIC_PROFIT_VIEWABLE_ECPM_USD",
            ParameterMetricsEnum::METRICPROFITVIEWABLEECPMPARTNER => "METRIC_PROFIT_VIEWABLE_ECPM_PARTNER",
            ParameterMetricsEnum::METRICREVENUEVIEWABLEECPMADVERTISER => "METRIC_REVENUE_VIEWABLE_ECPM_ADVERTISER",
            ParameterMetricsEnum::METRICREVENUEVIEWABLEECPMUSD => "METRIC_REVENUE_VIEWABLE_ECPM_USD",
            ParameterMetricsEnum::METRICREVENUEVIEWABLEECPMPARTNER => "METRIC_REVENUE_VIEWABLE_ECPM_PARTNER",
            ParameterMetricsEnum::METRICMEDIACOSTVIEWABLEECPMADVERTISER => "METRIC_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER",
            ParameterMetricsEnum::METRICMEDIACOSTVIEWABLEECPMUSD => "METRIC_MEDIA_COST_VIEWABLE_ECPM_USD",
            ParameterMetricsEnum::METRICMEDIACOSTVIEWABLEECPMPARTNER => "METRIC_MEDIA_COST_VIEWABLE_ECPM_PARTNER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTVIEWABLEECPMADVERTISER => "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTVIEWABLEECPMUSD => "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_USD",
            ParameterMetricsEnum::METRICTOTALMEDIACOSTVIEWABLEECPMPARTNER => "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_PARTNER",
            ParameterMetricsEnum::METRICTRUEVIEWENGAGEMENTS => "METRIC_TRUEVIEW_ENGAGEMENTS",
            ParameterMetricsEnum::METRICTRUEVIEWENGAGEMENTRATE => "METRIC_TRUEVIEW_ENGAGEMENT_RATE",
            ParameterMetricsEnum::METRICTRUEVIEWAVERAGECPEADVERTISER => "METRIC_TRUEVIEW_AVERAGE_CPE_ADVERTISER",
            ParameterMetricsEnum::METRICTRUEVIEWAVERAGECPEUSD => "METRIC_TRUEVIEW_AVERAGE_CPE_USD",
            ParameterMetricsEnum::METRICTRUEVIEWAVERAGECPEPARTNER => "METRIC_TRUEVIEW_AVERAGE_CPE_PARTNER",
            ParameterMetricsEnum::METRICACTIVEVIEWVIEWABLEIMPRESSIONS => "METRIC_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS",
            ParameterMetricsEnum::METRICACTIVEVIEWELIGIBLEIMPRESSIONS => "METRIC_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS",
            ParameterMetricsEnum::METRICACTIVEVIEWMEASURABLEIMPRESSIONS => "METRIC_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS",
            ParameterMetricsEnum::METRICACTIVEVIEWPCTMEASURABLEIMPRESSIONS => "METRIC_ACTIVE_VIEW_PCT_MEASURABLE_IMPRESSIONS",
            ParameterMetricsEnum::METRICACTIVEVIEWPCTVIEWABLEIMPRESSIONS => "METRIC_ACTIVE_VIEW_PCT_VIEWABLE_IMPRESSIONS",
            ParameterMetricsEnum::METRICACTIVEVIEWAVERAGEVIEWABLETIME => "METRIC_ACTIVE_VIEW_AVERAGE_VIEWABLE_TIME",
            ParameterMetricsEnum::METRICACTIVEVIEWUNMEASURABLEIMPRESSIONS => "METRIC_ACTIVE_VIEW_UNMEASURABLE_IMPRESSIONS",
            ParameterMetricsEnum::METRICACTIVEVIEWUNVIEWABLEIMPRESSIONS => "METRIC_ACTIVE_VIEW_UNVIEWABLE_IMPRESSIONS",
            ParameterMetricsEnum::METRICACTIVEVIEWDISTRIBUTIONUNMEASURABLE => "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNMEASURABLE",
            ParameterMetricsEnum::METRICACTIVEVIEWDISTRIBUTIONUNVIEWABLE => "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNVIEWABLE",
            ParameterMetricsEnum::METRICACTIVEVIEWDISTRIBUTIONVIEWABLE => "METRIC_ACTIVE_VIEW_DISTRIBUTION_VIEWABLE",
            ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVIEWABLEFORTIMETHRESHOLD => "METRIC_ACTIVE_VIEW_PERCENT_VIEWABLE_FOR_TIME_THRESHOLD",
            ParameterMetricsEnum::METRICACTIVEVIEWVIEWABLEFORTIMETHRESHOLD => "METRIC_ACTIVE_VIEW_VIEWABLE_FOR_TIME_THRESHOLD",
            ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVISIBLEATSTART => "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_AT_START",
            ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVISIBLEFIRSTQUAR => "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_FIRST_QUAR",
            ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVISIBLESECONDQUAR => "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_SECOND_QUAR",
            ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVISIBLETHIRDQUAR => "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_THIRD_QUAR",
            ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVISIBLEONCOMPLETE => "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_ON_COMPLETE",
            ParameterMetricsEnum::METRICACTIVEVIEWPERCENTAUDIBLEVISIBLEATSTART => "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_AT_START",
            ParameterMetricsEnum::METRICACTIVEVIEWPERCENTAUDIBLEVISIBLEFIRSTQUAR => "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_FIRST_QUAR",
            ParameterMetricsEnum::METRICACTIVEVIEWPERCENTAUDIBLEVISIBLESECONDQUAR => "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_SECOND_QUAR",
            ParameterMetricsEnum::METRICACTIVEVIEWPERCENTAUDIBLEVISIBLETHIRDQUAR => "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_THIRD_QUAR",
            ParameterMetricsEnum::METRICACTIVEVIEWPERCENTAUDIBLEVISIBLEONCOMPLETE => "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_ON_COMPLETE",
            ParameterMetricsEnum::METRICACTIVEVIEWAUDIBLEVISIBLEONCOMPLETEIMPRESSIONS => "METRIC_ACTIVE_VIEW_AUDIBLE_VISIBLE_ON_COMPLETE_IMPRESSIONS",
            ParameterMetricsEnum::METRICVIEWABLEBIDREQUESTS => "METRIC_VIEWABLE_BID_REQUESTS",
            ParameterMetricsEnum::METRICCOOKIEREACHIMPRESSIONREACH => "METRIC_COOKIE_REACH_IMPRESSION_REACH",
            ParameterMetricsEnum::METRICCOOKIEREACHAVERAGEIMPRESSIONFREQUENCY => "METRIC_COOKIE_REACH_AVERAGE_IMPRESSION_FREQUENCY",
            ParameterMetricsEnum::METRICDBMENGAGEMENTRATE => "METRIC_DBM_ENGAGEMENT_RATE",
            ParameterMetricsEnum::METRICRICHMEDIASCROLLS => "METRIC_RICH_MEDIA_SCROLLS",
            ParameterMetricsEnum::METRICCMPOSTVIEWREVENUE => "METRIC_CM_POST_VIEW_REVENUE",
            ParameterMetricsEnum::METRICCMPOSTCLICKREVENUE => "METRIC_CM_POST_CLICK_REVENUE",
            ParameterMetricsEnum::METRICFLOODLIGHTIMPRESSIONS => "METRIC_FLOODLIGHT_IMPRESSIONS",
            ParameterMetricsEnum::METRICBILLABLEIMPRESSIONS => "METRIC_BILLABLE_IMPRESSIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for ParameterMetricsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_UNKNOWN" => Ok(ParameterMetricsEnum::METRICUNKNOWN),
           "METRIC_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICIMPRESSIONS),
           "METRIC_CLICKS" => Ok(ParameterMetricsEnum::METRICCLICKS),
           "METRIC_LAST_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICLASTIMPRESSIONS),
           "METRIC_LAST_CLICKS" => Ok(ParameterMetricsEnum::METRICLASTCLICKS),
           "METRIC_TOTAL_CONVERSIONS" => Ok(ParameterMetricsEnum::METRICTOTALCONVERSIONS),
           "METRIC_MEDIA_COST_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTADVERTISER),
           "METRIC_MEDIA_COST_USD" => Ok(ParameterMetricsEnum::METRICMEDIACOSTUSD),
           "METRIC_MEDIA_COST_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTPARTNER),
           "METRIC_DATA_COST_ADVERTISER" => Ok(ParameterMetricsEnum::METRICDATACOSTADVERTISER),
           "METRIC_DATA_COST_USD" => Ok(ParameterMetricsEnum::METRICDATACOSTUSD),
           "METRIC_DATA_COST_PARTNER" => Ok(ParameterMetricsEnum::METRICDATACOSTPARTNER),
           "METRIC_CPM_FEE1_ADVERTISER" => Ok(ParameterMetricsEnum::METRICCPMFEE1ADVERTISER),
           "METRIC_CPM_FEE1_USD" => Ok(ParameterMetricsEnum::METRICCPMFEE1USD),
           "METRIC_CPM_FEE1_PARTNER" => Ok(ParameterMetricsEnum::METRICCPMFEE1PARTNER),
           "METRIC_CPM_FEE2_ADVERTISER" => Ok(ParameterMetricsEnum::METRICCPMFEE2ADVERTISER),
           "METRIC_CPM_FEE2_USD" => Ok(ParameterMetricsEnum::METRICCPMFEE2USD),
           "METRIC_CPM_FEE2_PARTNER" => Ok(ParameterMetricsEnum::METRICCPMFEE2PARTNER),
           "METRIC_MEDIA_FEE1_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIAFEE1ADVERTISER),
           "METRIC_MEDIA_FEE1_USD" => Ok(ParameterMetricsEnum::METRICMEDIAFEE1USD),
           "METRIC_MEDIA_FEE1_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIAFEE1PARTNER),
           "METRIC_MEDIA_FEE2_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIAFEE2ADVERTISER),
           "METRIC_MEDIA_FEE2_USD" => Ok(ParameterMetricsEnum::METRICMEDIAFEE2USD),
           "METRIC_MEDIA_FEE2_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIAFEE2PARTNER),
           "METRIC_REVENUE_ADVERTISER" => Ok(ParameterMetricsEnum::METRICREVENUEADVERTISER),
           "METRIC_REVENUE_USD" => Ok(ParameterMetricsEnum::METRICREVENUEUSD),
           "METRIC_REVENUE_PARTNER" => Ok(ParameterMetricsEnum::METRICREVENUEPARTNER),
           "METRIC_PROFIT_ADVERTISER" => Ok(ParameterMetricsEnum::METRICPROFITADVERTISER),
           "METRIC_PROFIT_USD" => Ok(ParameterMetricsEnum::METRICPROFITUSD),
           "METRIC_PROFIT_PARTNER" => Ok(ParameterMetricsEnum::METRICPROFITPARTNER),
           "METRIC_PROFIT_MARGIN" => Ok(ParameterMetricsEnum::METRICPROFITMARGIN),
           "METRIC_TOTAL_MEDIA_COST_USD" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTUSD),
           "METRIC_TOTAL_MEDIA_COST_PARTNER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTPARTNER),
           "METRIC_TOTAL_MEDIA_COST_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTADVERTISER),
           "METRIC_BILLABLE_COST_USD" => Ok(ParameterMetricsEnum::METRICBILLABLECOSTUSD),
           "METRIC_BILLABLE_COST_PARTNER" => Ok(ParameterMetricsEnum::METRICBILLABLECOSTPARTNER),
           "METRIC_BILLABLE_COST_ADVERTISER" => Ok(ParameterMetricsEnum::METRICBILLABLECOSTADVERTISER),
           "METRIC_PLATFORM_FEE_USD" => Ok(ParameterMetricsEnum::METRICPLATFORMFEEUSD),
           "METRIC_PLATFORM_FEE_PARTNER" => Ok(ParameterMetricsEnum::METRICPLATFORMFEEPARTNER),
           "METRIC_PLATFORM_FEE_ADVERTISER" => Ok(ParameterMetricsEnum::METRICPLATFORMFEEADVERTISER),
           "METRIC_VIDEO_COMPLETION_RATE" => Ok(ParameterMetricsEnum::METRICVIDEOCOMPLETIONRATE),
           "METRIC_PROFIT_ECPM_ADVERTISER" => Ok(ParameterMetricsEnum::METRICPROFITECPMADVERTISER),
           "METRIC_PROFIT_ECPM_USD" => Ok(ParameterMetricsEnum::METRICPROFITECPMUSD),
           "METRIC_PROFIT_ECPM_PARTNER" => Ok(ParameterMetricsEnum::METRICPROFITECPMPARTNER),
           "METRIC_REVENUE_ECPM_ADVERTISER" => Ok(ParameterMetricsEnum::METRICREVENUEECPMADVERTISER),
           "METRIC_REVENUE_ECPM_USD" => Ok(ParameterMetricsEnum::METRICREVENUEECPMUSD),
           "METRIC_REVENUE_ECPM_PARTNER" => Ok(ParameterMetricsEnum::METRICREVENUEECPMPARTNER),
           "METRIC_REVENUE_ECPC_ADVERTISER" => Ok(ParameterMetricsEnum::METRICREVENUEECPCADVERTISER),
           "METRIC_REVENUE_ECPC_USD" => Ok(ParameterMetricsEnum::METRICREVENUEECPCUSD),
           "METRIC_REVENUE_ECPC_PARTNER" => Ok(ParameterMetricsEnum::METRICREVENUEECPCPARTNER),
           "METRIC_REVENUE_ECPA_ADVERTISER" => Ok(ParameterMetricsEnum::METRICREVENUEECPAADVERTISER),
           "METRIC_REVENUE_ECPA_USD" => Ok(ParameterMetricsEnum::METRICREVENUEECPAUSD),
           "METRIC_REVENUE_ECPA_PARTNER" => Ok(ParameterMetricsEnum::METRICREVENUEECPAPARTNER),
           "METRIC_REVENUE_ECPAPV_ADVERTISER" => Ok(ParameterMetricsEnum::METRICREVENUEECPAPVADVERTISER),
           "METRIC_REVENUE_ECPAPV_USD" => Ok(ParameterMetricsEnum::METRICREVENUEECPAPVUSD),
           "METRIC_REVENUE_ECPAPV_PARTNER" => Ok(ParameterMetricsEnum::METRICREVENUEECPAPVPARTNER),
           "METRIC_REVENUE_ECPAPC_ADVERTISER" => Ok(ParameterMetricsEnum::METRICREVENUEECPAPCADVERTISER),
           "METRIC_REVENUE_ECPAPC_USD" => Ok(ParameterMetricsEnum::METRICREVENUEECPAPCUSD),
           "METRIC_REVENUE_ECPAPC_PARTNER" => Ok(ParameterMetricsEnum::METRICREVENUEECPAPCPARTNER),
           "METRIC_MEDIA_COST_ECPM_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPMADVERTISER),
           "METRIC_MEDIA_COST_ECPM_USD" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPMUSD),
           "METRIC_MEDIA_COST_ECPM_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPMPARTNER),
           "METRIC_MEDIA_COST_ECPC_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPCADVERTISER),
           "METRIC_MEDIA_COST_ECPC_USD" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPCUSD),
           "METRIC_MEDIA_COST_ECPC_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPCPARTNER),
           "METRIC_MEDIA_COST_ECPA_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPAADVERTISER),
           "METRIC_MEDIA_COST_ECPA_USD" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPAUSD),
           "METRIC_MEDIA_COST_ECPA_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPAPARTNER),
           "METRIC_MEDIA_COST_ECPAPV_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPAPVADVERTISER),
           "METRIC_MEDIA_COST_ECPAPV_USD" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPAPVUSD),
           "METRIC_MEDIA_COST_ECPAPV_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPAPVPARTNER),
           "METRIC_MEDIA_COST_ECPAPC_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPAPCADVERTISER),
           "METRIC_MEDIA_COST_ECPAPC_USD" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPAPCUSD),
           "METRIC_MEDIA_COST_ECPAPC_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPAPCPARTNER),
           "METRIC_TOTAL_MEDIA_COST_ECPM_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPMADVERTISER),
           "METRIC_TOTAL_MEDIA_COST_ECPM_USD" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPMUSD),
           "METRIC_TOTAL_MEDIA_COST_ECPM_PARTNER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPMPARTNER),
           "METRIC_TOTAL_MEDIA_COST_ECPC_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCADVERTISER),
           "METRIC_TOTAL_MEDIA_COST_ECPC_USD" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCUSD),
           "METRIC_TOTAL_MEDIA_COST_ECPC_PARTNER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCPARTNER),
           "METRIC_TOTAL_MEDIA_COST_ECPA_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAADVERTISER),
           "METRIC_TOTAL_MEDIA_COST_ECPA_USD" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAUSD),
           "METRIC_TOTAL_MEDIA_COST_ECPA_PARTNER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPARTNER),
           "METRIC_TOTAL_MEDIA_COST_ECPAPV_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPVADVERTISER),
           "METRIC_TOTAL_MEDIA_COST_ECPAPV_USD" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPVUSD),
           "METRIC_TOTAL_MEDIA_COST_ECPAPV_PARTNER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPVPARTNER),
           "METRIC_TOTAL_MEDIA_COST_ECPAPC_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPCADVERTISER),
           "METRIC_TOTAL_MEDIA_COST_ECPAPC_USD" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPCUSD),
           "METRIC_TOTAL_MEDIA_COST_ECPAPC_PARTNER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPAPCPARTNER),
           "METRIC_RICH_MEDIA_VIDEO_PLAYS" => Ok(ParameterMetricsEnum::METRICRICHMEDIAVIDEOPLAYS),
           "METRIC_RICH_MEDIA_VIDEO_COMPLETIONS" => Ok(ParameterMetricsEnum::METRICRICHMEDIAVIDEOCOMPLETIONS),
           "METRIC_RICH_MEDIA_VIDEO_PAUSES" => Ok(ParameterMetricsEnum::METRICRICHMEDIAVIDEOPAUSES),
           "METRIC_RICH_MEDIA_VIDEO_MUTES" => Ok(ParameterMetricsEnum::METRICRICHMEDIAVIDEOMUTES),
           "METRIC_RICH_MEDIA_VIDEO_MIDPOINTS" => Ok(ParameterMetricsEnum::METRICRICHMEDIAVIDEOMIDPOINTS),
           "METRIC_RICH_MEDIA_VIDEO_FULL_SCREENS" => Ok(ParameterMetricsEnum::METRICRICHMEDIAVIDEOFULLSCREENS),
           "METRIC_RICH_MEDIA_VIDEO_FIRST_QUARTILE_COMPLETES" => Ok(ParameterMetricsEnum::METRICRICHMEDIAVIDEOFIRSTQUARTILECOMPLETES),
           "METRIC_RICH_MEDIA_VIDEO_THIRD_QUARTILE_COMPLETES" => Ok(ParameterMetricsEnum::METRICRICHMEDIAVIDEOTHIRDQUARTILECOMPLETES),
           "METRIC_CLICK_TO_POST_CLICK_CONVERSION_RATE" => Ok(ParameterMetricsEnum::METRICCLICKTOPOSTCLICKCONVERSIONRATE),
           "METRIC_IMPRESSIONS_TO_CONVERSION_RATE" => Ok(ParameterMetricsEnum::METRICIMPRESSIONSTOCONVERSIONRATE),
           "METRIC_CONVERSIONS_PER_MILLE" => Ok(ParameterMetricsEnum::METRICCONVERSIONSPERMILLE),
           "METRIC_CTR" => Ok(ParameterMetricsEnum::METRICCTR),
           "METRIC_BID_REQUESTS" => Ok(ParameterMetricsEnum::METRICBIDREQUESTS),
           "METRIC_UNIQUE_VISITORS_COOKIES" => Ok(ParameterMetricsEnum::METRICUNIQUEVISITORSCOOKIES),
           "METRIC_REVENUE_ECPCV_ADVERTISER" => Ok(ParameterMetricsEnum::METRICREVENUEECPCVADVERTISER),
           "METRIC_REVENUE_ECPCV_USD" => Ok(ParameterMetricsEnum::METRICREVENUEECPCVUSD),
           "METRIC_REVENUE_ECPCV_PARTNER" => Ok(ParameterMetricsEnum::METRICREVENUEECPCVPARTNER),
           "METRIC_MEDIA_COST_ECPCV_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPCVADVERTISER),
           "METRIC_MEDIA_COST_ECPCV_USD" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPCVUSD),
           "METRIC_MEDIA_COST_ECPCV_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTECPCVPARTNER),
           "METRIC_TOTAL_MEDIA_COST_ECPCV_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCVADVERTISER),
           "METRIC_TOTAL_MEDIA_COST_ECPCV_USD" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCVUSD),
           "METRIC_TOTAL_MEDIA_COST_ECPCV_PARTNER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTECPCVPARTNER),
           "METRIC_RICH_MEDIA_VIDEO_SKIPS" => Ok(ParameterMetricsEnum::METRICRICHMEDIAVIDEOSKIPS),
           "METRIC_FEE2_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE2ADVERTISER),
           "METRIC_FEE2_USD" => Ok(ParameterMetricsEnum::METRICFEE2USD),
           "METRIC_FEE2_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE2PARTNER),
           "METRIC_FEE3_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE3ADVERTISER),
           "METRIC_FEE3_USD" => Ok(ParameterMetricsEnum::METRICFEE3USD),
           "METRIC_FEE3_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE3PARTNER),
           "METRIC_FEE4_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE4ADVERTISER),
           "METRIC_FEE4_USD" => Ok(ParameterMetricsEnum::METRICFEE4USD),
           "METRIC_FEE4_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE4PARTNER),
           "METRIC_FEE5_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE5ADVERTISER),
           "METRIC_FEE5_USD" => Ok(ParameterMetricsEnum::METRICFEE5USD),
           "METRIC_FEE5_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE5PARTNER),
           "METRIC_FEE6_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE6ADVERTISER),
           "METRIC_FEE6_USD" => Ok(ParameterMetricsEnum::METRICFEE6USD),
           "METRIC_FEE6_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE6PARTNER),
           "METRIC_FEE7_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE7ADVERTISER),
           "METRIC_FEE7_USD" => Ok(ParameterMetricsEnum::METRICFEE7USD),
           "METRIC_FEE7_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE7PARTNER),
           "METRIC_FEE8_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE8ADVERTISER),
           "METRIC_FEE8_USD" => Ok(ParameterMetricsEnum::METRICFEE8USD),
           "METRIC_FEE8_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE8PARTNER),
           "METRIC_FEE9_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE9ADVERTISER),
           "METRIC_FEE9_USD" => Ok(ParameterMetricsEnum::METRICFEE9USD),
           "METRIC_FEE9_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE9PARTNER),
           "METRIC_FEE10_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE10ADVERTISER),
           "METRIC_FEE10_USD" => Ok(ParameterMetricsEnum::METRICFEE10USD),
           "METRIC_FEE10_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE10PARTNER),
           "METRIC_FEE11_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE11ADVERTISER),
           "METRIC_FEE11_USD" => Ok(ParameterMetricsEnum::METRICFEE11USD),
           "METRIC_FEE11_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE11PARTNER),
           "METRIC_FEE12_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE12ADVERTISER),
           "METRIC_FEE12_USD" => Ok(ParameterMetricsEnum::METRICFEE12USD),
           "METRIC_FEE12_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE12PARTNER),
           "METRIC_FEE13_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE13ADVERTISER),
           "METRIC_FEE13_USD" => Ok(ParameterMetricsEnum::METRICFEE13USD),
           "METRIC_FEE13_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE13PARTNER),
           "METRIC_FEE14_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE14ADVERTISER),
           "METRIC_FEE14_USD" => Ok(ParameterMetricsEnum::METRICFEE14USD),
           "METRIC_FEE14_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE14PARTNER),
           "METRIC_FEE15_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE15ADVERTISER),
           "METRIC_FEE15_USD" => Ok(ParameterMetricsEnum::METRICFEE15USD),
           "METRIC_FEE15_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE15PARTNER),
           "METRIC_CPM_FEE3_ADVERTISER" => Ok(ParameterMetricsEnum::METRICCPMFEE3ADVERTISER),
           "METRIC_CPM_FEE3_USD" => Ok(ParameterMetricsEnum::METRICCPMFEE3USD),
           "METRIC_CPM_FEE3_PARTNER" => Ok(ParameterMetricsEnum::METRICCPMFEE3PARTNER),
           "METRIC_CPM_FEE4_ADVERTISER" => Ok(ParameterMetricsEnum::METRICCPMFEE4ADVERTISER),
           "METRIC_CPM_FEE4_USD" => Ok(ParameterMetricsEnum::METRICCPMFEE4USD),
           "METRIC_CPM_FEE4_PARTNER" => Ok(ParameterMetricsEnum::METRICCPMFEE4PARTNER),
           "METRIC_CPM_FEE5_ADVERTISER" => Ok(ParameterMetricsEnum::METRICCPMFEE5ADVERTISER),
           "METRIC_CPM_FEE5_USD" => Ok(ParameterMetricsEnum::METRICCPMFEE5USD),
           "METRIC_CPM_FEE5_PARTNER" => Ok(ParameterMetricsEnum::METRICCPMFEE5PARTNER),
           "METRIC_MEDIA_FEE3_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIAFEE3ADVERTISER),
           "METRIC_MEDIA_FEE3_USD" => Ok(ParameterMetricsEnum::METRICMEDIAFEE3USD),
           "METRIC_MEDIA_FEE3_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIAFEE3PARTNER),
           "METRIC_MEDIA_FEE4_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIAFEE4ADVERTISER),
           "METRIC_MEDIA_FEE4_USD" => Ok(ParameterMetricsEnum::METRICMEDIAFEE4USD),
           "METRIC_MEDIA_FEE4_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIAFEE4PARTNER),
           "METRIC_MEDIA_FEE5_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIAFEE5ADVERTISER),
           "METRIC_MEDIA_FEE5_USD" => Ok(ParameterMetricsEnum::METRICMEDIAFEE5USD),
           "METRIC_MEDIA_FEE5_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIAFEE5PARTNER),
           "METRIC_VIDEO_COMPANION_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICVIDEOCOMPANIONIMPRESSIONS),
           "METRIC_VIDEO_COMPANION_CLICKS" => Ok(ParameterMetricsEnum::METRICVIDEOCOMPANIONCLICKS),
           "METRIC_FEE16_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE16ADVERTISER),
           "METRIC_FEE16_USD" => Ok(ParameterMetricsEnum::METRICFEE16USD),
           "METRIC_FEE16_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE16PARTNER),
           "METRIC_FEE17_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE17ADVERTISER),
           "METRIC_FEE17_USD" => Ok(ParameterMetricsEnum::METRICFEE17USD),
           "METRIC_FEE17_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE17PARTNER),
           "METRIC_FEE18_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE18ADVERTISER),
           "METRIC_FEE18_USD" => Ok(ParameterMetricsEnum::METRICFEE18USD),
           "METRIC_FEE18_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE18PARTNER),
           "METRIC_TRUEVIEW_VIEWS" => Ok(ParameterMetricsEnum::METRICTRUEVIEWVIEWS),
           "METRIC_TRUEVIEW_UNIQUE_VIEWERS" => Ok(ParameterMetricsEnum::METRICTRUEVIEWUNIQUEVIEWERS),
           "METRIC_TRUEVIEW_EARNED_VIEWS" => Ok(ParameterMetricsEnum::METRICTRUEVIEWEARNEDVIEWS),
           "METRIC_TRUEVIEW_EARNED_SUBSCRIBERS" => Ok(ParameterMetricsEnum::METRICTRUEVIEWEARNEDSUBSCRIBERS),
           "METRIC_TRUEVIEW_EARNED_PLAYLIST_ADDITIONS" => Ok(ParameterMetricsEnum::METRICTRUEVIEWEARNEDPLAYLISTADDITIONS),
           "METRIC_TRUEVIEW_EARNED_LIKES" => Ok(ParameterMetricsEnum::METRICTRUEVIEWEARNEDLIKES),
           "METRIC_TRUEVIEW_EARNED_SHARES" => Ok(ParameterMetricsEnum::METRICTRUEVIEWEARNEDSHARES),
           "METRIC_TRUEVIEW_IMPRESSION_SHARE" => Ok(ParameterMetricsEnum::METRICTRUEVIEWIMPRESSIONSHARE),
           "METRIC_TRUEVIEW_LOST_IS_BUDGET" => Ok(ParameterMetricsEnum::METRICTRUEVIEWLOSTISBUDGET),
           "METRIC_TRUEVIEW_LOST_IS_RANK" => Ok(ParameterMetricsEnum::METRICTRUEVIEWLOSTISRANK),
           "METRIC_TRUEVIEW_VIEW_THROUGH_CONVERSION" => Ok(ParameterMetricsEnum::METRICTRUEVIEWVIEWTHROUGHCONVERSION),
           "METRIC_TRUEVIEW_CONVERSION_MANY_PER_VIEW" => Ok(ParameterMetricsEnum::METRICTRUEVIEWCONVERSIONMANYPERVIEW),
           "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUE" => Ok(ParameterMetricsEnum::METRICTRUEVIEWTOTALCONVERSIONVALUE),
           "METRIC_TRUEVIEW_VIEW_RATE" => Ok(ParameterMetricsEnum::METRICTRUEVIEWVIEWRATE),
           "METRIC_TRUEVIEW_CONVERSION_RATE_ONE_PER_VIEW" => Ok(ParameterMetricsEnum::METRICTRUEVIEWCONVERSIONRATEONEPERVIEW),
           "METRIC_TRUEVIEW_CPV_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTRUEVIEWCPVADVERTISER),
           "METRIC_TRUEVIEW_CPV_USD" => Ok(ParameterMetricsEnum::METRICTRUEVIEWCPVUSD),
           "METRIC_TRUEVIEW_CPV_PARTNER" => Ok(ParameterMetricsEnum::METRICTRUEVIEWCPVPARTNER),
           "METRIC_FEE19_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE19ADVERTISER),
           "METRIC_FEE19_USD" => Ok(ParameterMetricsEnum::METRICFEE19USD),
           "METRIC_FEE19_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE19PARTNER),
           "METRIC_TEA_TRUEVIEW_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICTEATRUEVIEWIMPRESSIONS),
           "METRIC_TEA_TRUEVIEW_UNIQUE_COOKIES" => Ok(ParameterMetricsEnum::METRICTEATRUEVIEWUNIQUECOOKIES),
           "METRIC_FEE20_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE20ADVERTISER),
           "METRIC_FEE20_USD" => Ok(ParameterMetricsEnum::METRICFEE20USD),
           "METRIC_FEE20_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE20PARTNER),
           "METRIC_FEE21_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE21ADVERTISER),
           "METRIC_FEE21_USD" => Ok(ParameterMetricsEnum::METRICFEE21USD),
           "METRIC_FEE21_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE21PARTNER),
           "METRIC_FEE22_ADVERTISER" => Ok(ParameterMetricsEnum::METRICFEE22ADVERTISER),
           "METRIC_FEE22_USD" => Ok(ParameterMetricsEnum::METRICFEE22USD),
           "METRIC_FEE22_PARTNER" => Ok(ParameterMetricsEnum::METRICFEE22PARTNER),
           "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTRUEVIEWTOTALCONVERSIONVALUESADVERTISER),
           "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_USD" => Ok(ParameterMetricsEnum::METRICTRUEVIEWTOTALCONVERSIONVALUESUSD),
           "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_PARTNER" => Ok(ParameterMetricsEnum::METRICTRUEVIEWTOTALCONVERSIONVALUESPARTNER),
           "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTRUEVIEWCONVERSIONCOSTMANYPERVIEWADVERTISER),
           "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_USD" => Ok(ParameterMetricsEnum::METRICTRUEVIEWCONVERSIONCOSTMANYPERVIEWUSD),
           "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_PARTNER" => Ok(ParameterMetricsEnum::METRICTRUEVIEWCONVERSIONCOSTMANYPERVIEWPARTNER),
           "METRIC_PROFIT_VIEWABLE_ECPM_ADVERTISER" => Ok(ParameterMetricsEnum::METRICPROFITVIEWABLEECPMADVERTISER),
           "METRIC_PROFIT_VIEWABLE_ECPM_USD" => Ok(ParameterMetricsEnum::METRICPROFITVIEWABLEECPMUSD),
           "METRIC_PROFIT_VIEWABLE_ECPM_PARTNER" => Ok(ParameterMetricsEnum::METRICPROFITVIEWABLEECPMPARTNER),
           "METRIC_REVENUE_VIEWABLE_ECPM_ADVERTISER" => Ok(ParameterMetricsEnum::METRICREVENUEVIEWABLEECPMADVERTISER),
           "METRIC_REVENUE_VIEWABLE_ECPM_USD" => Ok(ParameterMetricsEnum::METRICREVENUEVIEWABLEECPMUSD),
           "METRIC_REVENUE_VIEWABLE_ECPM_PARTNER" => Ok(ParameterMetricsEnum::METRICREVENUEVIEWABLEECPMPARTNER),
           "METRIC_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTVIEWABLEECPMADVERTISER),
           "METRIC_MEDIA_COST_VIEWABLE_ECPM_USD" => Ok(ParameterMetricsEnum::METRICMEDIACOSTVIEWABLEECPMUSD),
           "METRIC_MEDIA_COST_VIEWABLE_ECPM_PARTNER" => Ok(ParameterMetricsEnum::METRICMEDIACOSTVIEWABLEECPMPARTNER),
           "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTVIEWABLEECPMADVERTISER),
           "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_USD" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTVIEWABLEECPMUSD),
           "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_PARTNER" => Ok(ParameterMetricsEnum::METRICTOTALMEDIACOSTVIEWABLEECPMPARTNER),
           "METRIC_TRUEVIEW_ENGAGEMENTS" => Ok(ParameterMetricsEnum::METRICTRUEVIEWENGAGEMENTS),
           "METRIC_TRUEVIEW_ENGAGEMENT_RATE" => Ok(ParameterMetricsEnum::METRICTRUEVIEWENGAGEMENTRATE),
           "METRIC_TRUEVIEW_AVERAGE_CPE_ADVERTISER" => Ok(ParameterMetricsEnum::METRICTRUEVIEWAVERAGECPEADVERTISER),
           "METRIC_TRUEVIEW_AVERAGE_CPE_USD" => Ok(ParameterMetricsEnum::METRICTRUEVIEWAVERAGECPEUSD),
           "METRIC_TRUEVIEW_AVERAGE_CPE_PARTNER" => Ok(ParameterMetricsEnum::METRICTRUEVIEWAVERAGECPEPARTNER),
           "METRIC_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWVIEWABLEIMPRESSIONS),
           "METRIC_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWELIGIBLEIMPRESSIONS),
           "METRIC_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWMEASURABLEIMPRESSIONS),
           "METRIC_ACTIVE_VIEW_PCT_MEASURABLE_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPCTMEASURABLEIMPRESSIONS),
           "METRIC_ACTIVE_VIEW_PCT_VIEWABLE_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPCTVIEWABLEIMPRESSIONS),
           "METRIC_ACTIVE_VIEW_AVERAGE_VIEWABLE_TIME" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWAVERAGEVIEWABLETIME),
           "METRIC_ACTIVE_VIEW_UNMEASURABLE_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWUNMEASURABLEIMPRESSIONS),
           "METRIC_ACTIVE_VIEW_UNVIEWABLE_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWUNVIEWABLEIMPRESSIONS),
           "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNMEASURABLE" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWDISTRIBUTIONUNMEASURABLE),
           "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNVIEWABLE" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWDISTRIBUTIONUNVIEWABLE),
           "METRIC_ACTIVE_VIEW_DISTRIBUTION_VIEWABLE" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWDISTRIBUTIONVIEWABLE),
           "METRIC_ACTIVE_VIEW_PERCENT_VIEWABLE_FOR_TIME_THRESHOLD" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVIEWABLEFORTIMETHRESHOLD),
           "METRIC_ACTIVE_VIEW_VIEWABLE_FOR_TIME_THRESHOLD" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWVIEWABLEFORTIMETHRESHOLD),
           "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_AT_START" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVISIBLEATSTART),
           "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_FIRST_QUAR" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVISIBLEFIRSTQUAR),
           "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_SECOND_QUAR" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVISIBLESECONDQUAR),
           "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_THIRD_QUAR" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVISIBLETHIRDQUAR),
           "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_ON_COMPLETE" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPERCENTVISIBLEONCOMPLETE),
           "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_AT_START" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPERCENTAUDIBLEVISIBLEATSTART),
           "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_FIRST_QUAR" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPERCENTAUDIBLEVISIBLEFIRSTQUAR),
           "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_SECOND_QUAR" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPERCENTAUDIBLEVISIBLESECONDQUAR),
           "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_THIRD_QUAR" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPERCENTAUDIBLEVISIBLETHIRDQUAR),
           "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_ON_COMPLETE" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWPERCENTAUDIBLEVISIBLEONCOMPLETE),
           "METRIC_ACTIVE_VIEW_AUDIBLE_VISIBLE_ON_COMPLETE_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICACTIVEVIEWAUDIBLEVISIBLEONCOMPLETEIMPRESSIONS),
           "METRIC_VIEWABLE_BID_REQUESTS" => Ok(ParameterMetricsEnum::METRICVIEWABLEBIDREQUESTS),
           "METRIC_COOKIE_REACH_IMPRESSION_REACH" => Ok(ParameterMetricsEnum::METRICCOOKIEREACHIMPRESSIONREACH),
           "METRIC_COOKIE_REACH_AVERAGE_IMPRESSION_FREQUENCY" => Ok(ParameterMetricsEnum::METRICCOOKIEREACHAVERAGEIMPRESSIONFREQUENCY),
           "METRIC_DBM_ENGAGEMENT_RATE" => Ok(ParameterMetricsEnum::METRICDBMENGAGEMENTRATE),
           "METRIC_RICH_MEDIA_SCROLLS" => Ok(ParameterMetricsEnum::METRICRICHMEDIASCROLLS),
           "METRIC_CM_POST_VIEW_REVENUE" => Ok(ParameterMetricsEnum::METRICCMPOSTVIEWREVENUE),
           "METRIC_CM_POST_CLICK_REVENUE" => Ok(ParameterMetricsEnum::METRICCMPOSTCLICKREVENUE),
           "METRIC_FLOODLIGHT_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICFLOODLIGHTIMPRESSIONS),
           "METRIC_BILLABLE_IMPRESSIONS" => Ok(ParameterMetricsEnum::METRICBILLABLEIMPRESSIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParameterMetricsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParameterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Report type.
pub enum ParameterTypeEnum {
    
    /// "TYPE_GENERAL"
    #[serde(rename="TYPE_GENERAL")]
    TYPEGENERAL,
    
    /// "TYPE_AUDIENCE_PERFORMANCE"
    #[serde(rename="TYPE_AUDIENCE_PERFORMANCE")]
    TYPEAUDIENCEPERFORMANCE,
    
    /// "TYPE_INVENTORY_AVAILABILITY"
    #[serde(rename="TYPE_INVENTORY_AVAILABILITY")]
    TYPEINVENTORYAVAILABILITY,
    
    /// "TYPE_KEYWORD"
    #[serde(rename="TYPE_KEYWORD")]
    TYPEKEYWORD,
    
    /// "TYPE_PIXEL_LOAD"
    #[serde(rename="TYPE_PIXEL_LOAD")]
    TYPEPIXELLOAD,
    
    /// "TYPE_AUDIENCE_COMPOSITION"
    #[serde(rename="TYPE_AUDIENCE_COMPOSITION")]
    TYPEAUDIENCECOMPOSITION,
    
    /// "TYPE_CROSS_PARTNER"
    #[serde(rename="TYPE_CROSS_PARTNER")]
    TYPECROSSPARTNER,
    
    /// "TYPE_PAGE_CATEGORY"
    #[serde(rename="TYPE_PAGE_CATEGORY")]
    TYPEPAGECATEGORY,
    
    /// "TYPE_THIRD_PARTY_DATA_PROVIDER"
    #[serde(rename="TYPE_THIRD_PARTY_DATA_PROVIDER")]
    TYPETHIRDPARTYDATAPROVIDER,
    
    /// "TYPE_CROSS_PARTNER_THIRD_PARTY_DATA_PROVIDER"
    #[serde(rename="TYPE_CROSS_PARTNER_THIRD_PARTY_DATA_PROVIDER")]
    TYPECROSSPARTNERTHIRDPARTYDATAPROVIDER,
    
    /// "TYPE_CLIENT_SAFE"
    #[serde(rename="TYPE_CLIENT_SAFE")]
    TYPECLIENTSAFE,
    
    /// "TYPE_ORDER_ID"
    #[serde(rename="TYPE_ORDER_ID")]
    TYPEORDERID,
    
    /// "TYPE_FEE"
    #[serde(rename="TYPE_FEE")]
    TYPEFEE,
    
    /// "TYPE_CROSS_FEE"
    #[serde(rename="TYPE_CROSS_FEE")]
    TYPECROSSFEE,
    
    /// "TYPE_ACTIVE_GRP"
    #[serde(rename="TYPE_ACTIVE_GRP")]
    TYPEACTIVEGRP,
    
    /// "TYPE_YOUTUBE_VERTICAL"
    #[serde(rename="TYPE_YOUTUBE_VERTICAL")]
    TYPEYOUTUBEVERTICAL,
    
    /// "TYPE_COMSCORE_VCE"
    #[serde(rename="TYPE_COMSCORE_VCE")]
    TYPECOMSCOREVCE,
    
    /// "TYPE_TRUEVIEW"
    #[serde(rename="TYPE_TRUEVIEW")]
    TYPETRUEVIEW,
    
    /// "TYPE_NIELSEN_AUDIENCE_PROFILE"
    #[serde(rename="TYPE_NIELSEN_AUDIENCE_PROFILE")]
    TYPENIELSENAUDIENCEPROFILE,
    
    /// "TYPE_NIELSEN_DAILY_REACH_BUILD"
    #[serde(rename="TYPE_NIELSEN_DAILY_REACH_BUILD")]
    TYPENIELSENDAILYREACHBUILD,
    
    /// "TYPE_NIELSEN_SITE"
    #[serde(rename="TYPE_NIELSEN_SITE")]
    TYPENIELSENSITE,
    
    /// "TYPE_REACH_AND_FREQUENCY"
    #[serde(rename="TYPE_REACH_AND_FREQUENCY")]
    TYPEREACHANDFREQUENCY,
    
    /// "TYPE_ESTIMATED_CONVERSION"
    #[serde(rename="TYPE_ESTIMATED_CONVERSION")]
    TYPEESTIMATEDCONVERSION,
    
    /// "TYPE_VERIFICATION"
    #[serde(rename="TYPE_VERIFICATION")]
    TYPEVERIFICATION,
    
    /// "TYPE_TRUEVIEW_IAR"
    #[serde(rename="TYPE_TRUEVIEW_IAR")]
    TYPETRUEVIEWIAR,
    
    /// "TYPE_NIELSEN_ONLINE_GLOBAL_MARKET"
    #[serde(rename="TYPE_NIELSEN_ONLINE_GLOBAL_MARKET")]
    TYPENIELSENONLINEGLOBALMARKET,
    
    /// "TYPE_PETRA_NIELSEN_AUDIENCE_PROFILE"
    #[serde(rename="TYPE_PETRA_NIELSEN_AUDIENCE_PROFILE")]
    TYPEPETRANIELSENAUDIENCEPROFILE,
    
    /// "TYPE_PETRA_NIELSEN_DAILY_REACH_BUILD"
    #[serde(rename="TYPE_PETRA_NIELSEN_DAILY_REACH_BUILD")]
    TYPEPETRANIELSENDAILYREACHBUILD,
    
    /// "TYPE_PETRA_NIELSEN_ONLINE_GLOBAL_MARKET"
    #[serde(rename="TYPE_PETRA_NIELSEN_ONLINE_GLOBAL_MARKET")]
    TYPEPETRANIELSENONLINEGLOBALMARKET,
    
    /// "TYPE_NOT_SUPPORTED"
    #[serde(rename="TYPE_NOT_SUPPORTED")]
    TYPENOTSUPPORTED,
    
    /// "TYPE_REACH_AUDIENCE"
    #[serde(rename="TYPE_REACH_AUDIENCE")]
    TYPEREACHAUDIENCE,
    
    /// "TYPE_LINEAR_TV_SEARCH_LIFT"
    #[serde(rename="TYPE_LINEAR_TV_SEARCH_LIFT")]
    TYPELINEARTVSEARCHLIFT,
}

impl AsRef<str> for ParameterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParameterTypeEnum::TYPEGENERAL => "TYPE_GENERAL",
            ParameterTypeEnum::TYPEAUDIENCEPERFORMANCE => "TYPE_AUDIENCE_PERFORMANCE",
            ParameterTypeEnum::TYPEINVENTORYAVAILABILITY => "TYPE_INVENTORY_AVAILABILITY",
            ParameterTypeEnum::TYPEKEYWORD => "TYPE_KEYWORD",
            ParameterTypeEnum::TYPEPIXELLOAD => "TYPE_PIXEL_LOAD",
            ParameterTypeEnum::TYPEAUDIENCECOMPOSITION => "TYPE_AUDIENCE_COMPOSITION",
            ParameterTypeEnum::TYPECROSSPARTNER => "TYPE_CROSS_PARTNER",
            ParameterTypeEnum::TYPEPAGECATEGORY => "TYPE_PAGE_CATEGORY",
            ParameterTypeEnum::TYPETHIRDPARTYDATAPROVIDER => "TYPE_THIRD_PARTY_DATA_PROVIDER",
            ParameterTypeEnum::TYPECROSSPARTNERTHIRDPARTYDATAPROVIDER => "TYPE_CROSS_PARTNER_THIRD_PARTY_DATA_PROVIDER",
            ParameterTypeEnum::TYPECLIENTSAFE => "TYPE_CLIENT_SAFE",
            ParameterTypeEnum::TYPEORDERID => "TYPE_ORDER_ID",
            ParameterTypeEnum::TYPEFEE => "TYPE_FEE",
            ParameterTypeEnum::TYPECROSSFEE => "TYPE_CROSS_FEE",
            ParameterTypeEnum::TYPEACTIVEGRP => "TYPE_ACTIVE_GRP",
            ParameterTypeEnum::TYPEYOUTUBEVERTICAL => "TYPE_YOUTUBE_VERTICAL",
            ParameterTypeEnum::TYPECOMSCOREVCE => "TYPE_COMSCORE_VCE",
            ParameterTypeEnum::TYPETRUEVIEW => "TYPE_TRUEVIEW",
            ParameterTypeEnum::TYPENIELSENAUDIENCEPROFILE => "TYPE_NIELSEN_AUDIENCE_PROFILE",
            ParameterTypeEnum::TYPENIELSENDAILYREACHBUILD => "TYPE_NIELSEN_DAILY_REACH_BUILD",
            ParameterTypeEnum::TYPENIELSENSITE => "TYPE_NIELSEN_SITE",
            ParameterTypeEnum::TYPEREACHANDFREQUENCY => "TYPE_REACH_AND_FREQUENCY",
            ParameterTypeEnum::TYPEESTIMATEDCONVERSION => "TYPE_ESTIMATED_CONVERSION",
            ParameterTypeEnum::TYPEVERIFICATION => "TYPE_VERIFICATION",
            ParameterTypeEnum::TYPETRUEVIEWIAR => "TYPE_TRUEVIEW_IAR",
            ParameterTypeEnum::TYPENIELSENONLINEGLOBALMARKET => "TYPE_NIELSEN_ONLINE_GLOBAL_MARKET",
            ParameterTypeEnum::TYPEPETRANIELSENAUDIENCEPROFILE => "TYPE_PETRA_NIELSEN_AUDIENCE_PROFILE",
            ParameterTypeEnum::TYPEPETRANIELSENDAILYREACHBUILD => "TYPE_PETRA_NIELSEN_DAILY_REACH_BUILD",
            ParameterTypeEnum::TYPEPETRANIELSENONLINEGLOBALMARKET => "TYPE_PETRA_NIELSEN_ONLINE_GLOBAL_MARKET",
            ParameterTypeEnum::TYPENOTSUPPORTED => "TYPE_NOT_SUPPORTED",
            ParameterTypeEnum::TYPEREACHAUDIENCE => "TYPE_REACH_AUDIENCE",
            ParameterTypeEnum::TYPELINEARTVSEARCHLIFT => "TYPE_LINEAR_TV_SEARCH_LIFT",
        }
    }
}

impl std::convert::TryFrom< &str> for ParameterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_GENERAL" => Ok(ParameterTypeEnum::TYPEGENERAL),
           "TYPE_AUDIENCE_PERFORMANCE" => Ok(ParameterTypeEnum::TYPEAUDIENCEPERFORMANCE),
           "TYPE_INVENTORY_AVAILABILITY" => Ok(ParameterTypeEnum::TYPEINVENTORYAVAILABILITY),
           "TYPE_KEYWORD" => Ok(ParameterTypeEnum::TYPEKEYWORD),
           "TYPE_PIXEL_LOAD" => Ok(ParameterTypeEnum::TYPEPIXELLOAD),
           "TYPE_AUDIENCE_COMPOSITION" => Ok(ParameterTypeEnum::TYPEAUDIENCECOMPOSITION),
           "TYPE_CROSS_PARTNER" => Ok(ParameterTypeEnum::TYPECROSSPARTNER),
           "TYPE_PAGE_CATEGORY" => Ok(ParameterTypeEnum::TYPEPAGECATEGORY),
           "TYPE_THIRD_PARTY_DATA_PROVIDER" => Ok(ParameterTypeEnum::TYPETHIRDPARTYDATAPROVIDER),
           "TYPE_CROSS_PARTNER_THIRD_PARTY_DATA_PROVIDER" => Ok(ParameterTypeEnum::TYPECROSSPARTNERTHIRDPARTYDATAPROVIDER),
           "TYPE_CLIENT_SAFE" => Ok(ParameterTypeEnum::TYPECLIENTSAFE),
           "TYPE_ORDER_ID" => Ok(ParameterTypeEnum::TYPEORDERID),
           "TYPE_FEE" => Ok(ParameterTypeEnum::TYPEFEE),
           "TYPE_CROSS_FEE" => Ok(ParameterTypeEnum::TYPECROSSFEE),
           "TYPE_ACTIVE_GRP" => Ok(ParameterTypeEnum::TYPEACTIVEGRP),
           "TYPE_YOUTUBE_VERTICAL" => Ok(ParameterTypeEnum::TYPEYOUTUBEVERTICAL),
           "TYPE_COMSCORE_VCE" => Ok(ParameterTypeEnum::TYPECOMSCOREVCE),
           "TYPE_TRUEVIEW" => Ok(ParameterTypeEnum::TYPETRUEVIEW),
           "TYPE_NIELSEN_AUDIENCE_PROFILE" => Ok(ParameterTypeEnum::TYPENIELSENAUDIENCEPROFILE),
           "TYPE_NIELSEN_DAILY_REACH_BUILD" => Ok(ParameterTypeEnum::TYPENIELSENDAILYREACHBUILD),
           "TYPE_NIELSEN_SITE" => Ok(ParameterTypeEnum::TYPENIELSENSITE),
           "TYPE_REACH_AND_FREQUENCY" => Ok(ParameterTypeEnum::TYPEREACHANDFREQUENCY),
           "TYPE_ESTIMATED_CONVERSION" => Ok(ParameterTypeEnum::TYPEESTIMATEDCONVERSION),
           "TYPE_VERIFICATION" => Ok(ParameterTypeEnum::TYPEVERIFICATION),
           "TYPE_TRUEVIEW_IAR" => Ok(ParameterTypeEnum::TYPETRUEVIEWIAR),
           "TYPE_NIELSEN_ONLINE_GLOBAL_MARKET" => Ok(ParameterTypeEnum::TYPENIELSENONLINEGLOBALMARKET),
           "TYPE_PETRA_NIELSEN_AUDIENCE_PROFILE" => Ok(ParameterTypeEnum::TYPEPETRANIELSENAUDIENCEPROFILE),
           "TYPE_PETRA_NIELSEN_DAILY_REACH_BUILD" => Ok(ParameterTypeEnum::TYPEPETRANIELSENDAILYREACHBUILD),
           "TYPE_PETRA_NIELSEN_ONLINE_GLOBAL_MARKET" => Ok(ParameterTypeEnum::TYPEPETRANIELSENONLINEGLOBALMARKET),
           "TYPE_NOT_SUPPORTED" => Ok(ParameterTypeEnum::TYPENOTSUPPORTED),
           "TYPE_REACH_AUDIENCE" => Ok(ParameterTypeEnum::TYPEREACHAUDIENCE),
           "TYPE_LINEAR_TV_SEARCH_LIFT" => Ok(ParameterTypeEnum::TYPELINEARTVSEARCHLIFT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParameterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueryMetadataDataRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Range of report data.
pub enum QueryMetadataDataRangeEnum {
    
    /// "CUSTOM_DATES"
    #[serde(rename="CUSTOM_DATES")]
    CUSTOMDATES,
    
    /// "CURRENT_DAY"
    #[serde(rename="CURRENT_DAY")]
    CURRENTDAY,
    
    /// "PREVIOUS_DAY"
    #[serde(rename="PREVIOUS_DAY")]
    PREVIOUSDAY,
    
    /// "WEEK_TO_DATE"
    #[serde(rename="WEEK_TO_DATE")]
    WEEKTODATE,
    
    /// "MONTH_TO_DATE"
    #[serde(rename="MONTH_TO_DATE")]
    MONTHTODATE,
    
    /// "QUARTER_TO_DATE"
    #[serde(rename="QUARTER_TO_DATE")]
    QUARTERTODATE,
    
    /// "YEAR_TO_DATE"
    #[serde(rename="YEAR_TO_DATE")]
    YEARTODATE,
    
    /// "PREVIOUS_WEEK"
    #[serde(rename="PREVIOUS_WEEK")]
    PREVIOUSWEEK,
    
    /// "PREVIOUS_HALF_MONTH"
    #[serde(rename="PREVIOUS_HALF_MONTH")]
    PREVIOUSHALFMONTH,
    
    /// "PREVIOUS_MONTH"
    #[serde(rename="PREVIOUS_MONTH")]
    PREVIOUSMONTH,
    
    /// "PREVIOUS_QUARTER"
    #[serde(rename="PREVIOUS_QUARTER")]
    PREVIOUSQUARTER,
    
    /// "PREVIOUS_YEAR"
    #[serde(rename="PREVIOUS_YEAR")]
    PREVIOUSYEAR,
    
    /// "LAST_7_DAYS"
    #[serde(rename="LAST_7_DAYS")]
    LAST7DAYS,
    
    /// "LAST_30_DAYS"
    #[serde(rename="LAST_30_DAYS")]
    LAST30DAYS,
    
    /// "LAST_90_DAYS"
    #[serde(rename="LAST_90_DAYS")]
    LAST90DAYS,
    
    /// "LAST_365_DAYS"
    #[serde(rename="LAST_365_DAYS")]
    LAST365DAYS,
    
    /// "ALL_TIME"
    #[serde(rename="ALL_TIME")]
    ALLTIME,
    
    /// "LAST_14_DAYS"
    #[serde(rename="LAST_14_DAYS")]
    LAST14DAYS,
    
    /// "TYPE_NOT_SUPPORTED"
    #[serde(rename="TYPE_NOT_SUPPORTED")]
    TYPENOTSUPPORTED,
}

impl AsRef<str> for QueryMetadataDataRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueryMetadataDataRangeEnum::CUSTOMDATES => "CUSTOM_DATES",
            QueryMetadataDataRangeEnum::CURRENTDAY => "CURRENT_DAY",
            QueryMetadataDataRangeEnum::PREVIOUSDAY => "PREVIOUS_DAY",
            QueryMetadataDataRangeEnum::WEEKTODATE => "WEEK_TO_DATE",
            QueryMetadataDataRangeEnum::MONTHTODATE => "MONTH_TO_DATE",
            QueryMetadataDataRangeEnum::QUARTERTODATE => "QUARTER_TO_DATE",
            QueryMetadataDataRangeEnum::YEARTODATE => "YEAR_TO_DATE",
            QueryMetadataDataRangeEnum::PREVIOUSWEEK => "PREVIOUS_WEEK",
            QueryMetadataDataRangeEnum::PREVIOUSHALFMONTH => "PREVIOUS_HALF_MONTH",
            QueryMetadataDataRangeEnum::PREVIOUSMONTH => "PREVIOUS_MONTH",
            QueryMetadataDataRangeEnum::PREVIOUSQUARTER => "PREVIOUS_QUARTER",
            QueryMetadataDataRangeEnum::PREVIOUSYEAR => "PREVIOUS_YEAR",
            QueryMetadataDataRangeEnum::LAST7DAYS => "LAST_7_DAYS",
            QueryMetadataDataRangeEnum::LAST30DAYS => "LAST_30_DAYS",
            QueryMetadataDataRangeEnum::LAST90DAYS => "LAST_90_DAYS",
            QueryMetadataDataRangeEnum::LAST365DAYS => "LAST_365_DAYS",
            QueryMetadataDataRangeEnum::ALLTIME => "ALL_TIME",
            QueryMetadataDataRangeEnum::LAST14DAYS => "LAST_14_DAYS",
            QueryMetadataDataRangeEnum::TYPENOTSUPPORTED => "TYPE_NOT_SUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for QueryMetadataDataRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CUSTOM_DATES" => Ok(QueryMetadataDataRangeEnum::CUSTOMDATES),
           "CURRENT_DAY" => Ok(QueryMetadataDataRangeEnum::CURRENTDAY),
           "PREVIOUS_DAY" => Ok(QueryMetadataDataRangeEnum::PREVIOUSDAY),
           "WEEK_TO_DATE" => Ok(QueryMetadataDataRangeEnum::WEEKTODATE),
           "MONTH_TO_DATE" => Ok(QueryMetadataDataRangeEnum::MONTHTODATE),
           "QUARTER_TO_DATE" => Ok(QueryMetadataDataRangeEnum::QUARTERTODATE),
           "YEAR_TO_DATE" => Ok(QueryMetadataDataRangeEnum::YEARTODATE),
           "PREVIOUS_WEEK" => Ok(QueryMetadataDataRangeEnum::PREVIOUSWEEK),
           "PREVIOUS_HALF_MONTH" => Ok(QueryMetadataDataRangeEnum::PREVIOUSHALFMONTH),
           "PREVIOUS_MONTH" => Ok(QueryMetadataDataRangeEnum::PREVIOUSMONTH),
           "PREVIOUS_QUARTER" => Ok(QueryMetadataDataRangeEnum::PREVIOUSQUARTER),
           "PREVIOUS_YEAR" => Ok(QueryMetadataDataRangeEnum::PREVIOUSYEAR),
           "LAST_7_DAYS" => Ok(QueryMetadataDataRangeEnum::LAST7DAYS),
           "LAST_30_DAYS" => Ok(QueryMetadataDataRangeEnum::LAST30DAYS),
           "LAST_90_DAYS" => Ok(QueryMetadataDataRangeEnum::LAST90DAYS),
           "LAST_365_DAYS" => Ok(QueryMetadataDataRangeEnum::LAST365DAYS),
           "ALL_TIME" => Ok(QueryMetadataDataRangeEnum::ALLTIME),
           "LAST_14_DAYS" => Ok(QueryMetadataDataRangeEnum::LAST14DAYS),
           "TYPE_NOT_SUPPORTED" => Ok(QueryMetadataDataRangeEnum::TYPENOTSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueryMetadataDataRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueryMetadataFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Format of the generated report.
pub enum QueryMetadataFormatEnum {
    
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
    
    /// "EXCEL_CSV"
    #[serde(rename="EXCEL_CSV")]
    EXCELCSV,
    
    /// "XLSX"
    #[serde(rename="XLSX")]
    XLSX,
}

impl AsRef<str> for QueryMetadataFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueryMetadataFormatEnum::CSV => "CSV",
            QueryMetadataFormatEnum::EXCELCSV => "EXCEL_CSV",
            QueryMetadataFormatEnum::XLSX => "XLSX",
        }
    }
}

impl std::convert::TryFrom< &str> for QueryMetadataFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CSV" => Ok(QueryMetadataFormatEnum::CSV),
           "EXCEL_CSV" => Ok(QueryMetadataFormatEnum::EXCELCSV),
           "XLSX" => Ok(QueryMetadataFormatEnum::XLSX),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueryMetadataFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueryScheduleFrequencyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How often the query is run.
pub enum QueryScheduleFrequencyEnum {
    
    /// "ONE_TIME"
    #[serde(rename="ONE_TIME")]
    ONETIME,
    
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
    
    /// "WEEKLY"
    #[serde(rename="WEEKLY")]
    WEEKLY,
    
    /// "SEMI_MONTHLY"
    #[serde(rename="SEMI_MONTHLY")]
    SEMIMONTHLY,
    
    /// "MONTHLY"
    #[serde(rename="MONTHLY")]
    MONTHLY,
    
    /// "QUARTERLY"
    #[serde(rename="QUARTERLY")]
    QUARTERLY,
}

impl AsRef<str> for QueryScheduleFrequencyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueryScheduleFrequencyEnum::ONETIME => "ONE_TIME",
            QueryScheduleFrequencyEnum::DAILY => "DAILY",
            QueryScheduleFrequencyEnum::WEEKLY => "WEEKLY",
            QueryScheduleFrequencyEnum::SEMIMONTHLY => "SEMI_MONTHLY",
            QueryScheduleFrequencyEnum::MONTHLY => "MONTHLY",
            QueryScheduleFrequencyEnum::QUARTERLY => "QUARTERLY",
        }
    }
}

impl std::convert::TryFrom< &str> for QueryScheduleFrequencyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ONE_TIME" => Ok(QueryScheduleFrequencyEnum::ONETIME),
           "DAILY" => Ok(QueryScheduleFrequencyEnum::DAILY),
           "WEEKLY" => Ok(QueryScheduleFrequencyEnum::WEEKLY),
           "SEMI_MONTHLY" => Ok(QueryScheduleFrequencyEnum::SEMIMONTHLY),
           "MONTHLY" => Ok(QueryScheduleFrequencyEnum::MONTHLY),
           "QUARTERLY" => Ok(QueryScheduleFrequencyEnum::QUARTERLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueryScheduleFrequencyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportFailureErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Error code that shows why the report was not created.
pub enum ReportFailureErrorCodeEnum {
    
    /// "AUTHENTICATION_ERROR"
    #[serde(rename="AUTHENTICATION_ERROR")]
    AUTHENTICATIONERROR,
    
    /// "UNAUTHORIZED_API_ACCESS"
    #[serde(rename="UNAUTHORIZED_API_ACCESS")]
    UNAUTHORIZEDAPIACCESS,
    
    /// "SERVER_ERROR"
    #[serde(rename="SERVER_ERROR")]
    SERVERERROR,
    
    /// "VALIDATION_ERROR"
    #[serde(rename="VALIDATION_ERROR")]
    VALIDATIONERROR,
    
    /// "REPORTING_FATAL_ERROR"
    #[serde(rename="REPORTING_FATAL_ERROR")]
    REPORTINGFATALERROR,
    
    /// "REPORTING_TRANSIENT_ERROR"
    #[serde(rename="REPORTING_TRANSIENT_ERROR")]
    REPORTINGTRANSIENTERROR,
    
    /// "REPORTING_IMCOMPATIBLE_METRICS"
    #[serde(rename="REPORTING_IMCOMPATIBLE_METRICS")]
    REPORTINGIMCOMPATIBLEMETRICS,
    
    /// "REPORTING_ILLEGAL_FILENAME"
    #[serde(rename="REPORTING_ILLEGAL_FILENAME")]
    REPORTINGILLEGALFILENAME,
    
    /// "REPORTING_QUERY_NOT_FOUND"
    #[serde(rename="REPORTING_QUERY_NOT_FOUND")]
    REPORTINGQUERYNOTFOUND,
    
    /// "REPORTING_BUCKET_NOT_FOUND"
    #[serde(rename="REPORTING_BUCKET_NOT_FOUND")]
    REPORTINGBUCKETNOTFOUND,
    
    /// "REPORTING_CREATE_BUCKET_FAILED"
    #[serde(rename="REPORTING_CREATE_BUCKET_FAILED")]
    REPORTINGCREATEBUCKETFAILED,
    
    /// "REPORTING_DELETE_BUCKET_FAILED"
    #[serde(rename="REPORTING_DELETE_BUCKET_FAILED")]
    REPORTINGDELETEBUCKETFAILED,
    
    /// "REPORTING_UPDATE_BUCKET_PERMISSION_FAILED"
    #[serde(rename="REPORTING_UPDATE_BUCKET_PERMISSION_FAILED")]
    REPORTINGUPDATEBUCKETPERMISSIONFAILED,
    
    /// "REPORTING_WRITE_BUCKET_OBJECT_FAILED"
    #[serde(rename="REPORTING_WRITE_BUCKET_OBJECT_FAILED")]
    REPORTINGWRITEBUCKETOBJECTFAILED,
    
    /// "DEPRECATED_REPORTING_INVALID_QUERY"
    #[serde(rename="DEPRECATED_REPORTING_INVALID_QUERY")]
    DEPRECATEDREPORTINGINVALIDQUERY,
    
    /// "REPORTING_INVALID_QUERY_TOO_MANY_UNFILTERED_LARGE_GROUP_BYS"
    #[serde(rename="REPORTING_INVALID_QUERY_TOO_MANY_UNFILTERED_LARGE_GROUP_BYS")]
    REPORTINGINVALIDQUERYTOOMANYUNFILTEREDLARGEGROUPBYS,
    
    /// "REPORTING_INVALID_QUERY_TITLE_MISSING"
    #[serde(rename="REPORTING_INVALID_QUERY_TITLE_MISSING")]
    REPORTINGINVALIDQUERYTITLEMISSING,
    
    /// "REPORTING_INVALID_QUERY_MISSING_PARTNER_AND_ADVERTISER_FILTERS"
    #[serde(rename="REPORTING_INVALID_QUERY_MISSING_PARTNER_AND_ADVERTISER_FILTERS")]
    REPORTINGINVALIDQUERYMISSINGPARTNERANDADVERTISERFILTERS,
}

impl AsRef<str> for ReportFailureErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportFailureErrorCodeEnum::AUTHENTICATIONERROR => "AUTHENTICATION_ERROR",
            ReportFailureErrorCodeEnum::UNAUTHORIZEDAPIACCESS => "UNAUTHORIZED_API_ACCESS",
            ReportFailureErrorCodeEnum::SERVERERROR => "SERVER_ERROR",
            ReportFailureErrorCodeEnum::VALIDATIONERROR => "VALIDATION_ERROR",
            ReportFailureErrorCodeEnum::REPORTINGFATALERROR => "REPORTING_FATAL_ERROR",
            ReportFailureErrorCodeEnum::REPORTINGTRANSIENTERROR => "REPORTING_TRANSIENT_ERROR",
            ReportFailureErrorCodeEnum::REPORTINGIMCOMPATIBLEMETRICS => "REPORTING_IMCOMPATIBLE_METRICS",
            ReportFailureErrorCodeEnum::REPORTINGILLEGALFILENAME => "REPORTING_ILLEGAL_FILENAME",
            ReportFailureErrorCodeEnum::REPORTINGQUERYNOTFOUND => "REPORTING_QUERY_NOT_FOUND",
            ReportFailureErrorCodeEnum::REPORTINGBUCKETNOTFOUND => "REPORTING_BUCKET_NOT_FOUND",
            ReportFailureErrorCodeEnum::REPORTINGCREATEBUCKETFAILED => "REPORTING_CREATE_BUCKET_FAILED",
            ReportFailureErrorCodeEnum::REPORTINGDELETEBUCKETFAILED => "REPORTING_DELETE_BUCKET_FAILED",
            ReportFailureErrorCodeEnum::REPORTINGUPDATEBUCKETPERMISSIONFAILED => "REPORTING_UPDATE_BUCKET_PERMISSION_FAILED",
            ReportFailureErrorCodeEnum::REPORTINGWRITEBUCKETOBJECTFAILED => "REPORTING_WRITE_BUCKET_OBJECT_FAILED",
            ReportFailureErrorCodeEnum::DEPRECATEDREPORTINGINVALIDQUERY => "DEPRECATED_REPORTING_INVALID_QUERY",
            ReportFailureErrorCodeEnum::REPORTINGINVALIDQUERYTOOMANYUNFILTEREDLARGEGROUPBYS => "REPORTING_INVALID_QUERY_TOO_MANY_UNFILTERED_LARGE_GROUP_BYS",
            ReportFailureErrorCodeEnum::REPORTINGINVALIDQUERYTITLEMISSING => "REPORTING_INVALID_QUERY_TITLE_MISSING",
            ReportFailureErrorCodeEnum::REPORTINGINVALIDQUERYMISSINGPARTNERANDADVERTISERFILTERS => "REPORTING_INVALID_QUERY_MISSING_PARTNER_AND_ADVERTISER_FILTERS",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportFailureErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTHENTICATION_ERROR" => Ok(ReportFailureErrorCodeEnum::AUTHENTICATIONERROR),
           "UNAUTHORIZED_API_ACCESS" => Ok(ReportFailureErrorCodeEnum::UNAUTHORIZEDAPIACCESS),
           "SERVER_ERROR" => Ok(ReportFailureErrorCodeEnum::SERVERERROR),
           "VALIDATION_ERROR" => Ok(ReportFailureErrorCodeEnum::VALIDATIONERROR),
           "REPORTING_FATAL_ERROR" => Ok(ReportFailureErrorCodeEnum::REPORTINGFATALERROR),
           "REPORTING_TRANSIENT_ERROR" => Ok(ReportFailureErrorCodeEnum::REPORTINGTRANSIENTERROR),
           "REPORTING_IMCOMPATIBLE_METRICS" => Ok(ReportFailureErrorCodeEnum::REPORTINGIMCOMPATIBLEMETRICS),
           "REPORTING_ILLEGAL_FILENAME" => Ok(ReportFailureErrorCodeEnum::REPORTINGILLEGALFILENAME),
           "REPORTING_QUERY_NOT_FOUND" => Ok(ReportFailureErrorCodeEnum::REPORTINGQUERYNOTFOUND),
           "REPORTING_BUCKET_NOT_FOUND" => Ok(ReportFailureErrorCodeEnum::REPORTINGBUCKETNOTFOUND),
           "REPORTING_CREATE_BUCKET_FAILED" => Ok(ReportFailureErrorCodeEnum::REPORTINGCREATEBUCKETFAILED),
           "REPORTING_DELETE_BUCKET_FAILED" => Ok(ReportFailureErrorCodeEnum::REPORTINGDELETEBUCKETFAILED),
           "REPORTING_UPDATE_BUCKET_PERMISSION_FAILED" => Ok(ReportFailureErrorCodeEnum::REPORTINGUPDATEBUCKETPERMISSIONFAILED),
           "REPORTING_WRITE_BUCKET_OBJECT_FAILED" => Ok(ReportFailureErrorCodeEnum::REPORTINGWRITEBUCKETOBJECTFAILED),
           "DEPRECATED_REPORTING_INVALID_QUERY" => Ok(ReportFailureErrorCodeEnum::DEPRECATEDREPORTINGINVALIDQUERY),
           "REPORTING_INVALID_QUERY_TOO_MANY_UNFILTERED_LARGE_GROUP_BYS" => Ok(ReportFailureErrorCodeEnum::REPORTINGINVALIDQUERYTOOMANYUNFILTEREDLARGEGROUPBYS),
           "REPORTING_INVALID_QUERY_TITLE_MISSING" => Ok(ReportFailureErrorCodeEnum::REPORTINGINVALIDQUERYTITLEMISSING),
           "REPORTING_INVALID_QUERY_MISSING_PARTNER_AND_ADVERTISER_FILTERS" => Ok(ReportFailureErrorCodeEnum::REPORTINGINVALIDQUERYMISSINGPARTNERANDADVERTISERFILTERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportFailureErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportStatusFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The file type of the report.
pub enum ReportStatusFormatEnum {
    
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
    
    /// "EXCEL_CSV"
    #[serde(rename="EXCEL_CSV")]
    EXCELCSV,
    
    /// "XLSX"
    #[serde(rename="XLSX")]
    XLSX,
}

impl AsRef<str> for ReportStatusFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportStatusFormatEnum::CSV => "CSV",
            ReportStatusFormatEnum::EXCELCSV => "EXCEL_CSV",
            ReportStatusFormatEnum::XLSX => "XLSX",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportStatusFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CSV" => Ok(ReportStatusFormatEnum::CSV),
           "EXCEL_CSV" => Ok(ReportStatusFormatEnum::EXCELCSV),
           "XLSX" => Ok(ReportStatusFormatEnum::XLSX),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportStatusFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the report.
pub enum ReportStatusStateEnum {
    
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
    
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for ReportStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportStatusStateEnum::RUNNING => "RUNNING",
            ReportStatusStateEnum::DONE => "DONE",
            ReportStatusStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RUNNING" => Ok(ReportStatusStateEnum::RUNNING),
           "DONE" => Ok(ReportStatusStateEnum::DONE),
           "FAILED" => Ok(ReportStatusStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RunQueryRequestDataRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Report data range used to generate the report.
pub enum RunQueryRequestDataRangeEnum {
    
    /// "CUSTOM_DATES"
    #[serde(rename="CUSTOM_DATES")]
    CUSTOMDATES,
    
    /// "CURRENT_DAY"
    #[serde(rename="CURRENT_DAY")]
    CURRENTDAY,
    
    /// "PREVIOUS_DAY"
    #[serde(rename="PREVIOUS_DAY")]
    PREVIOUSDAY,
    
    /// "WEEK_TO_DATE"
    #[serde(rename="WEEK_TO_DATE")]
    WEEKTODATE,
    
    /// "MONTH_TO_DATE"
    #[serde(rename="MONTH_TO_DATE")]
    MONTHTODATE,
    
    /// "QUARTER_TO_DATE"
    #[serde(rename="QUARTER_TO_DATE")]
    QUARTERTODATE,
    
    /// "YEAR_TO_DATE"
    #[serde(rename="YEAR_TO_DATE")]
    YEARTODATE,
    
    /// "PREVIOUS_WEEK"
    #[serde(rename="PREVIOUS_WEEK")]
    PREVIOUSWEEK,
    
    /// "PREVIOUS_HALF_MONTH"
    #[serde(rename="PREVIOUS_HALF_MONTH")]
    PREVIOUSHALFMONTH,
    
    /// "PREVIOUS_MONTH"
    #[serde(rename="PREVIOUS_MONTH")]
    PREVIOUSMONTH,
    
    /// "PREVIOUS_QUARTER"
    #[serde(rename="PREVIOUS_QUARTER")]
    PREVIOUSQUARTER,
    
    /// "PREVIOUS_YEAR"
    #[serde(rename="PREVIOUS_YEAR")]
    PREVIOUSYEAR,
    
    /// "LAST_7_DAYS"
    #[serde(rename="LAST_7_DAYS")]
    LAST7DAYS,
    
    /// "LAST_30_DAYS"
    #[serde(rename="LAST_30_DAYS")]
    LAST30DAYS,
    
    /// "LAST_90_DAYS"
    #[serde(rename="LAST_90_DAYS")]
    LAST90DAYS,
    
    /// "LAST_365_DAYS"
    #[serde(rename="LAST_365_DAYS")]
    LAST365DAYS,
    
    /// "ALL_TIME"
    #[serde(rename="ALL_TIME")]
    ALLTIME,
    
    /// "LAST_14_DAYS"
    #[serde(rename="LAST_14_DAYS")]
    LAST14DAYS,
    
    /// "TYPE_NOT_SUPPORTED"
    #[serde(rename="TYPE_NOT_SUPPORTED")]
    TYPENOTSUPPORTED,
}

impl AsRef<str> for RunQueryRequestDataRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RunQueryRequestDataRangeEnum::CUSTOMDATES => "CUSTOM_DATES",
            RunQueryRequestDataRangeEnum::CURRENTDAY => "CURRENT_DAY",
            RunQueryRequestDataRangeEnum::PREVIOUSDAY => "PREVIOUS_DAY",
            RunQueryRequestDataRangeEnum::WEEKTODATE => "WEEK_TO_DATE",
            RunQueryRequestDataRangeEnum::MONTHTODATE => "MONTH_TO_DATE",
            RunQueryRequestDataRangeEnum::QUARTERTODATE => "QUARTER_TO_DATE",
            RunQueryRequestDataRangeEnum::YEARTODATE => "YEAR_TO_DATE",
            RunQueryRequestDataRangeEnum::PREVIOUSWEEK => "PREVIOUS_WEEK",
            RunQueryRequestDataRangeEnum::PREVIOUSHALFMONTH => "PREVIOUS_HALF_MONTH",
            RunQueryRequestDataRangeEnum::PREVIOUSMONTH => "PREVIOUS_MONTH",
            RunQueryRequestDataRangeEnum::PREVIOUSQUARTER => "PREVIOUS_QUARTER",
            RunQueryRequestDataRangeEnum::PREVIOUSYEAR => "PREVIOUS_YEAR",
            RunQueryRequestDataRangeEnum::LAST7DAYS => "LAST_7_DAYS",
            RunQueryRequestDataRangeEnum::LAST30DAYS => "LAST_30_DAYS",
            RunQueryRequestDataRangeEnum::LAST90DAYS => "LAST_90_DAYS",
            RunQueryRequestDataRangeEnum::LAST365DAYS => "LAST_365_DAYS",
            RunQueryRequestDataRangeEnum::ALLTIME => "ALL_TIME",
            RunQueryRequestDataRangeEnum::LAST14DAYS => "LAST_14_DAYS",
            RunQueryRequestDataRangeEnum::TYPENOTSUPPORTED => "TYPE_NOT_SUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for RunQueryRequestDataRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CUSTOM_DATES" => Ok(RunQueryRequestDataRangeEnum::CUSTOMDATES),
           "CURRENT_DAY" => Ok(RunQueryRequestDataRangeEnum::CURRENTDAY),
           "PREVIOUS_DAY" => Ok(RunQueryRequestDataRangeEnum::PREVIOUSDAY),
           "WEEK_TO_DATE" => Ok(RunQueryRequestDataRangeEnum::WEEKTODATE),
           "MONTH_TO_DATE" => Ok(RunQueryRequestDataRangeEnum::MONTHTODATE),
           "QUARTER_TO_DATE" => Ok(RunQueryRequestDataRangeEnum::QUARTERTODATE),
           "YEAR_TO_DATE" => Ok(RunQueryRequestDataRangeEnum::YEARTODATE),
           "PREVIOUS_WEEK" => Ok(RunQueryRequestDataRangeEnum::PREVIOUSWEEK),
           "PREVIOUS_HALF_MONTH" => Ok(RunQueryRequestDataRangeEnum::PREVIOUSHALFMONTH),
           "PREVIOUS_MONTH" => Ok(RunQueryRequestDataRangeEnum::PREVIOUSMONTH),
           "PREVIOUS_QUARTER" => Ok(RunQueryRequestDataRangeEnum::PREVIOUSQUARTER),
           "PREVIOUS_YEAR" => Ok(RunQueryRequestDataRangeEnum::PREVIOUSYEAR),
           "LAST_7_DAYS" => Ok(RunQueryRequestDataRangeEnum::LAST7DAYS),
           "LAST_30_DAYS" => Ok(RunQueryRequestDataRangeEnum::LAST30DAYS),
           "LAST_90_DAYS" => Ok(RunQueryRequestDataRangeEnum::LAST90DAYS),
           "LAST_365_DAYS" => Ok(RunQueryRequestDataRangeEnum::LAST365DAYS),
           "ALL_TIME" => Ok(RunQueryRequestDataRangeEnum::ALLTIME),
           "LAST_14_DAYS" => Ok(RunQueryRequestDataRangeEnum::LAST14DAYS),
           "TYPE_NOT_SUPPORTED" => Ok(RunQueryRequestDataRangeEnum::TYPENOTSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RunQueryRequestDataRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UploadLineItemsRequestFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Format the line items are in. Default to CSV.
pub enum UploadLineItemsRequestFormatEnum {
    
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
}

impl AsRef<str> for UploadLineItemsRequestFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UploadLineItemsRequestFormatEnum::CSV => "CSV",
        }
    }
}

impl std::convert::TryFrom< &str> for UploadLineItemsRequestFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CSV" => Ok(UploadLineItemsRequestFormatEnum::CSV),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UploadLineItemsRequestFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


