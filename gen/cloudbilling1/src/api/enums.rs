use super::*;



// region AggregationInfoAggregationIntervalEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum AggregationInfoAggregationIntervalEnum {
    
    /// "AGGREGATION_INTERVAL_UNSPECIFIED"
    #[serde(rename="AGGREGATION_INTERVAL_UNSPECIFIED")]
    AGGREGATIONINTERVALUNSPECIFIED,
    
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
    
    /// "MONTHLY"
    #[serde(rename="MONTHLY")]
    MONTHLY,
}

impl AsRef<str> for AggregationInfoAggregationIntervalEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AggregationInfoAggregationIntervalEnum::AGGREGATIONINTERVALUNSPECIFIED => "AGGREGATION_INTERVAL_UNSPECIFIED",
            AggregationInfoAggregationIntervalEnum::DAILY => "DAILY",
            AggregationInfoAggregationIntervalEnum::MONTHLY => "MONTHLY",
        }
    }
}

impl std::convert::TryFrom< &str> for AggregationInfoAggregationIntervalEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AGGREGATION_INTERVAL_UNSPECIFIED" => Ok(AggregationInfoAggregationIntervalEnum::AGGREGATIONINTERVALUNSPECIFIED),
           "DAILY" => Ok(AggregationInfoAggregationIntervalEnum::DAILY),
           "MONTHLY" => Ok(AggregationInfoAggregationIntervalEnum::MONTHLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AggregationInfoAggregationIntervalEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AggregationInfoAggregationLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum AggregationInfoAggregationLevelEnum {
    
    /// "AGGREGATION_LEVEL_UNSPECIFIED"
    #[serde(rename="AGGREGATION_LEVEL_UNSPECIFIED")]
    AGGREGATIONLEVELUNSPECIFIED,
    
    /// "ACCOUNT"
    #[serde(rename="ACCOUNT")]
    ACCOUNT,
    
    /// "PROJECT"
    #[serde(rename="PROJECT")]
    PROJECT,
}

impl AsRef<str> for AggregationInfoAggregationLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AggregationInfoAggregationLevelEnum::AGGREGATIONLEVELUNSPECIFIED => "AGGREGATION_LEVEL_UNSPECIFIED",
            AggregationInfoAggregationLevelEnum::ACCOUNT => "ACCOUNT",
            AggregationInfoAggregationLevelEnum::PROJECT => "PROJECT",
        }
    }
}

impl std::convert::TryFrom< &str> for AggregationInfoAggregationLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AGGREGATION_LEVEL_UNSPECIFIED" => Ok(AggregationInfoAggregationLevelEnum::AGGREGATIONLEVELUNSPECIFIED),
           "ACCOUNT" => Ok(AggregationInfoAggregationLevelEnum::ACCOUNT),
           "PROJECT" => Ok(AggregationInfoAggregationLevelEnum::PROJECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AggregationInfoAggregationLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuditLogConfigLogTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The log type that this config enables.
pub enum AuditLogConfigLogTypeEnum {
    

    /// Default case. Should never be this.
    ///
    /// "LOG_TYPE_UNSPECIFIED"
    #[serde(rename="LOG_TYPE_UNSPECIFIED")]
    LOGTYPEUNSPECIFIED,
    

    /// Admin reads. Example: CloudIAM getIamPolicy
    ///
    /// "ADMIN_READ"
    #[serde(rename="ADMIN_READ")]
    ADMINREAD,
    

    /// Data writes. Example: CloudSQL Users create
    ///
    /// "DATA_WRITE"
    #[serde(rename="DATA_WRITE")]
    DATAWRITE,
    

    /// Data reads. Example: CloudSQL Users list
    ///
    /// "DATA_READ"
    #[serde(rename="DATA_READ")]
    DATAREAD,
}

impl AsRef<str> for AuditLogConfigLogTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            AuditLogConfigLogTypeEnum::ADMINREAD => "ADMIN_READ",
            AuditLogConfigLogTypeEnum::DATAWRITE => "DATA_WRITE",
            AuditLogConfigLogTypeEnum::DATAREAD => "DATA_READ",
        }
    }
}

impl std::convert::TryFrom< &str> for AuditLogConfigLogTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED),
           "ADMIN_READ" => Ok(AuditLogConfigLogTypeEnum::ADMINREAD),
           "DATA_WRITE" => Ok(AuditLogConfigLogTypeEnum::DATAWRITE),
           "DATA_READ" => Ok(AuditLogConfigLogTypeEnum::DATAREAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuditLogConfigLogTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GeoTaxonomyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of Geo Taxonomy: GLOBAL, REGIONAL, or MULTI_REGIONAL.
pub enum GeoTaxonomyTypeEnum {
    

    /// The type is not specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The sku is global in nature, e.g. a license sku. Global skus are available in all regions, and so have an empty region list.
    ///
    /// "GLOBAL"
    #[serde(rename="GLOBAL")]
    GLOBAL,
    

    /// The sku is available in a specific region, e.g. "us-west2".
    ///
    /// "REGIONAL"
    #[serde(rename="REGIONAL")]
    REGIONAL,
    

    /// The sku is associated with multiple regions, e.g. "us-west2" and "us-east1".
    ///
    /// "MULTI_REGIONAL"
    #[serde(rename="MULTI_REGIONAL")]
    MULTIREGIONAL,
}

impl AsRef<str> for GeoTaxonomyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GeoTaxonomyTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GeoTaxonomyTypeEnum::GLOBAL => "GLOBAL",
            GeoTaxonomyTypeEnum::REGIONAL => "REGIONAL",
            GeoTaxonomyTypeEnum::MULTIREGIONAL => "MULTI_REGIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GeoTaxonomyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GeoTaxonomyTypeEnum::TYPEUNSPECIFIED),
           "GLOBAL" => Ok(GeoTaxonomyTypeEnum::GLOBAL),
           "REGIONAL" => Ok(GeoTaxonomyTypeEnum::REGIONAL),
           "MULTI_REGIONAL" => Ok(GeoTaxonomyTypeEnum::MULTIREGIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GeoTaxonomyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


