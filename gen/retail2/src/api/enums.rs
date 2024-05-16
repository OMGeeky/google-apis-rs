use super::*;



// region GoogleCloudRetailV2AttributesConfigAttributeConfigLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The AttributeConfigLevel used for this catalog.
pub enum GoogleCloudRetailV2AttributesConfigAttributeConfigLevelEnum {
    

    /// Value used when unset. In this case, server behavior defaults to CATALOG_LEVEL_ATTRIBUTE_CONFIG.
    ///
    /// "ATTRIBUTE_CONFIG_LEVEL_UNSPECIFIED"
    #[serde(rename="ATTRIBUTE_CONFIG_LEVEL_UNSPECIFIED")]
    ATTRIBUTECONFIGLEVELUNSPECIFIED,
    

    /// At this level, we honor the attribute configurations set in Product.attributes.
    ///
    /// "PRODUCT_LEVEL_ATTRIBUTE_CONFIG"
    #[serde(rename="PRODUCT_LEVEL_ATTRIBUTE_CONFIG")]
    PRODUCTLEVELATTRIBUTECONFIG,
    

    /// At this level, we honor the attribute configurations set in CatalogConfig.attribute_configs.
    ///
    /// "CATALOG_LEVEL_ATTRIBUTE_CONFIG"
    #[serde(rename="CATALOG_LEVEL_ATTRIBUTE_CONFIG")]
    CATALOGLEVELATTRIBUTECONFIG,
}

impl AsRef<str> for GoogleCloudRetailV2AttributesConfigAttributeConfigLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2AttributesConfigAttributeConfigLevelEnum::ATTRIBUTECONFIGLEVELUNSPECIFIED => "ATTRIBUTE_CONFIG_LEVEL_UNSPECIFIED",
            GoogleCloudRetailV2AttributesConfigAttributeConfigLevelEnum::PRODUCTLEVELATTRIBUTECONFIG => "PRODUCT_LEVEL_ATTRIBUTE_CONFIG",
            GoogleCloudRetailV2AttributesConfigAttributeConfigLevelEnum::CATALOGLEVELATTRIBUTECONFIG => "CATALOG_LEVEL_ATTRIBUTE_CONFIG",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2AttributesConfigAttributeConfigLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTRIBUTE_CONFIG_LEVEL_UNSPECIFIED" => Ok(GoogleCloudRetailV2AttributesConfigAttributeConfigLevelEnum::ATTRIBUTECONFIGLEVELUNSPECIFIED),
           "PRODUCT_LEVEL_ATTRIBUTE_CONFIG" => Ok(GoogleCloudRetailV2AttributesConfigAttributeConfigLevelEnum::PRODUCTLEVELATTRIBUTECONFIG),
           "CATALOG_LEVEL_ATTRIBUTE_CONFIG" => Ok(GoogleCloudRetailV2AttributesConfigAttributeConfigLevelEnum::CATALOGLEVELATTRIBUTECONFIG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2AttributesConfigAttributeConfigLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2CatalogAttributeDynamicFacetableOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If DYNAMIC_FACETABLE_ENABLED, attribute values are available for dynamic facet. Could only be DYNAMIC_FACETABLE_DISABLED if CatalogAttribute.indexable_option is INDEXABLE_DISABLED. Otherwise, an INVALID_ARGUMENT error is returned. Must be specified, otherwise throws INVALID_FORMAT error.
pub enum GoogleCloudRetailV2CatalogAttributeDynamicFacetableOptionEnum {
    

    /// Value used when unset.
    ///
    /// "DYNAMIC_FACETABLE_OPTION_UNSPECIFIED"
    #[serde(rename="DYNAMIC_FACETABLE_OPTION_UNSPECIFIED")]
    DYNAMICFACETABLEOPTIONUNSPECIFIED,
    

    /// Dynamic facetable option enabled for an attribute.
    ///
    /// "DYNAMIC_FACETABLE_ENABLED"
    #[serde(rename="DYNAMIC_FACETABLE_ENABLED")]
    DYNAMICFACETABLEENABLED,
    

    /// Dynamic facetable option disabled for an attribute.
    ///
    /// "DYNAMIC_FACETABLE_DISABLED"
    #[serde(rename="DYNAMIC_FACETABLE_DISABLED")]
    DYNAMICFACETABLEDISABLED,
}

impl AsRef<str> for GoogleCloudRetailV2CatalogAttributeDynamicFacetableOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2CatalogAttributeDynamicFacetableOptionEnum::DYNAMICFACETABLEOPTIONUNSPECIFIED => "DYNAMIC_FACETABLE_OPTION_UNSPECIFIED",
            GoogleCloudRetailV2CatalogAttributeDynamicFacetableOptionEnum::DYNAMICFACETABLEENABLED => "DYNAMIC_FACETABLE_ENABLED",
            GoogleCloudRetailV2CatalogAttributeDynamicFacetableOptionEnum::DYNAMICFACETABLEDISABLED => "DYNAMIC_FACETABLE_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2CatalogAttributeDynamicFacetableOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DYNAMIC_FACETABLE_OPTION_UNSPECIFIED" => Ok(GoogleCloudRetailV2CatalogAttributeDynamicFacetableOptionEnum::DYNAMICFACETABLEOPTIONUNSPECIFIED),
           "DYNAMIC_FACETABLE_ENABLED" => Ok(GoogleCloudRetailV2CatalogAttributeDynamicFacetableOptionEnum::DYNAMICFACETABLEENABLED),
           "DYNAMIC_FACETABLE_DISABLED" => Ok(GoogleCloudRetailV2CatalogAttributeDynamicFacetableOptionEnum::DYNAMICFACETABLEDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2CatalogAttributeDynamicFacetableOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2CatalogAttributeExactSearchableOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If EXACT_SEARCHABLE_ENABLED, attribute values will be exact searchable. This property only applies to textual custom attributes and requires indexable set to enabled to enable exact-searchable. If unset, the server behavior defaults to EXACT_SEARCHABLE_DISABLED.
pub enum GoogleCloudRetailV2CatalogAttributeExactSearchableOptionEnum {
    

    /// Value used when unset.
    ///
    /// "EXACT_SEARCHABLE_OPTION_UNSPECIFIED"
    #[serde(rename="EXACT_SEARCHABLE_OPTION_UNSPECIFIED")]
    EXACTSEARCHABLEOPTIONUNSPECIFIED,
    

    /// Exact searchable option enabled for an attribute.
    ///
    /// "EXACT_SEARCHABLE_ENABLED"
    #[serde(rename="EXACT_SEARCHABLE_ENABLED")]
    EXACTSEARCHABLEENABLED,
    

    /// Exact searchable option disabled for an attribute.
    ///
    /// "EXACT_SEARCHABLE_DISABLED"
    #[serde(rename="EXACT_SEARCHABLE_DISABLED")]
    EXACTSEARCHABLEDISABLED,
}

impl AsRef<str> for GoogleCloudRetailV2CatalogAttributeExactSearchableOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2CatalogAttributeExactSearchableOptionEnum::EXACTSEARCHABLEOPTIONUNSPECIFIED => "EXACT_SEARCHABLE_OPTION_UNSPECIFIED",
            GoogleCloudRetailV2CatalogAttributeExactSearchableOptionEnum::EXACTSEARCHABLEENABLED => "EXACT_SEARCHABLE_ENABLED",
            GoogleCloudRetailV2CatalogAttributeExactSearchableOptionEnum::EXACTSEARCHABLEDISABLED => "EXACT_SEARCHABLE_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2CatalogAttributeExactSearchableOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXACT_SEARCHABLE_OPTION_UNSPECIFIED" => Ok(GoogleCloudRetailV2CatalogAttributeExactSearchableOptionEnum::EXACTSEARCHABLEOPTIONUNSPECIFIED),
           "EXACT_SEARCHABLE_ENABLED" => Ok(GoogleCloudRetailV2CatalogAttributeExactSearchableOptionEnum::EXACTSEARCHABLEENABLED),
           "EXACT_SEARCHABLE_DISABLED" => Ok(GoogleCloudRetailV2CatalogAttributeExactSearchableOptionEnum::EXACTSEARCHABLEDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2CatalogAttributeExactSearchableOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2CatalogAttributeIndexableOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// When AttributesConfig.attribute_config_level is CATALOG_LEVEL_ATTRIBUTE_CONFIG, if INDEXABLE_ENABLED attribute values are indexed so that it can be filtered, faceted, or boosted in SearchService.Search. Must be specified when AttributesConfig.attribute_config_level is CATALOG_LEVEL_ATTRIBUTE_CONFIG, otherwise throws INVALID_FORMAT error.
pub enum GoogleCloudRetailV2CatalogAttributeIndexableOptionEnum {
    

    /// Value used when unset.
    ///
    /// "INDEXABLE_OPTION_UNSPECIFIED"
    #[serde(rename="INDEXABLE_OPTION_UNSPECIFIED")]
    INDEXABLEOPTIONUNSPECIFIED,
    

    /// Indexable option enabled for an attribute.
    ///
    /// "INDEXABLE_ENABLED"
    #[serde(rename="INDEXABLE_ENABLED")]
    INDEXABLEENABLED,
    

    /// Indexable option disabled for an attribute.
    ///
    /// "INDEXABLE_DISABLED"
    #[serde(rename="INDEXABLE_DISABLED")]
    INDEXABLEDISABLED,
}

impl AsRef<str> for GoogleCloudRetailV2CatalogAttributeIndexableOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2CatalogAttributeIndexableOptionEnum::INDEXABLEOPTIONUNSPECIFIED => "INDEXABLE_OPTION_UNSPECIFIED",
            GoogleCloudRetailV2CatalogAttributeIndexableOptionEnum::INDEXABLEENABLED => "INDEXABLE_ENABLED",
            GoogleCloudRetailV2CatalogAttributeIndexableOptionEnum::INDEXABLEDISABLED => "INDEXABLE_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2CatalogAttributeIndexableOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INDEXABLE_OPTION_UNSPECIFIED" => Ok(GoogleCloudRetailV2CatalogAttributeIndexableOptionEnum::INDEXABLEOPTIONUNSPECIFIED),
           "INDEXABLE_ENABLED" => Ok(GoogleCloudRetailV2CatalogAttributeIndexableOptionEnum::INDEXABLEENABLED),
           "INDEXABLE_DISABLED" => Ok(GoogleCloudRetailV2CatalogAttributeIndexableOptionEnum::INDEXABLEDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2CatalogAttributeIndexableOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2CatalogAttributeRetrievableOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If RETRIEVABLE_ENABLED, attribute values are retrievable in the search results. If unset, the server behavior defaults to RETRIEVABLE_DISABLED.
pub enum GoogleCloudRetailV2CatalogAttributeRetrievableOptionEnum {
    

    /// Value used when unset.
    ///
    /// "RETRIEVABLE_OPTION_UNSPECIFIED"
    #[serde(rename="RETRIEVABLE_OPTION_UNSPECIFIED")]
    RETRIEVABLEOPTIONUNSPECIFIED,
    

    /// Retrievable option enabled for an attribute.
    ///
    /// "RETRIEVABLE_ENABLED"
    #[serde(rename="RETRIEVABLE_ENABLED")]
    RETRIEVABLEENABLED,
    

    /// Retrievable option disabled for an attribute.
    ///
    /// "RETRIEVABLE_DISABLED"
    #[serde(rename="RETRIEVABLE_DISABLED")]
    RETRIEVABLEDISABLED,
}

impl AsRef<str> for GoogleCloudRetailV2CatalogAttributeRetrievableOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2CatalogAttributeRetrievableOptionEnum::RETRIEVABLEOPTIONUNSPECIFIED => "RETRIEVABLE_OPTION_UNSPECIFIED",
            GoogleCloudRetailV2CatalogAttributeRetrievableOptionEnum::RETRIEVABLEENABLED => "RETRIEVABLE_ENABLED",
            GoogleCloudRetailV2CatalogAttributeRetrievableOptionEnum::RETRIEVABLEDISABLED => "RETRIEVABLE_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2CatalogAttributeRetrievableOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RETRIEVABLE_OPTION_UNSPECIFIED" => Ok(GoogleCloudRetailV2CatalogAttributeRetrievableOptionEnum::RETRIEVABLEOPTIONUNSPECIFIED),
           "RETRIEVABLE_ENABLED" => Ok(GoogleCloudRetailV2CatalogAttributeRetrievableOptionEnum::RETRIEVABLEENABLED),
           "RETRIEVABLE_DISABLED" => Ok(GoogleCloudRetailV2CatalogAttributeRetrievableOptionEnum::RETRIEVABLEDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2CatalogAttributeRetrievableOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2CatalogAttributeSearchableOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// When AttributesConfig.attribute_config_level is CATALOG_LEVEL_ATTRIBUTE_CONFIG, if SEARCHABLE_ENABLED, attribute values are searchable by text queries in SearchService.Search. If SEARCHABLE_ENABLED but attribute type is numerical, attribute values will not be searchable by text queries in SearchService.Search, as there are no text values associated to numerical attributes. Must be specified, when AttributesConfig.attribute_config_level is CATALOG_LEVEL_ATTRIBUTE_CONFIG, otherwise throws INVALID_FORMAT error.
pub enum GoogleCloudRetailV2CatalogAttributeSearchableOptionEnum {
    

    /// Value used when unset.
    ///
    /// "SEARCHABLE_OPTION_UNSPECIFIED"
    #[serde(rename="SEARCHABLE_OPTION_UNSPECIFIED")]
    SEARCHABLEOPTIONUNSPECIFIED,
    

    /// Searchable option enabled for an attribute.
    ///
    /// "SEARCHABLE_ENABLED"
    #[serde(rename="SEARCHABLE_ENABLED")]
    SEARCHABLEENABLED,
    

    /// Searchable option disabled for an attribute.
    ///
    /// "SEARCHABLE_DISABLED"
    #[serde(rename="SEARCHABLE_DISABLED")]
    SEARCHABLEDISABLED,
}

impl AsRef<str> for GoogleCloudRetailV2CatalogAttributeSearchableOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2CatalogAttributeSearchableOptionEnum::SEARCHABLEOPTIONUNSPECIFIED => "SEARCHABLE_OPTION_UNSPECIFIED",
            GoogleCloudRetailV2CatalogAttributeSearchableOptionEnum::SEARCHABLEENABLED => "SEARCHABLE_ENABLED",
            GoogleCloudRetailV2CatalogAttributeSearchableOptionEnum::SEARCHABLEDISABLED => "SEARCHABLE_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2CatalogAttributeSearchableOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEARCHABLE_OPTION_UNSPECIFIED" => Ok(GoogleCloudRetailV2CatalogAttributeSearchableOptionEnum::SEARCHABLEOPTIONUNSPECIFIED),
           "SEARCHABLE_ENABLED" => Ok(GoogleCloudRetailV2CatalogAttributeSearchableOptionEnum::SEARCHABLEENABLED),
           "SEARCHABLE_DISABLED" => Ok(GoogleCloudRetailV2CatalogAttributeSearchableOptionEnum::SEARCHABLEDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2CatalogAttributeSearchableOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2CatalogAttributeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of this attribute. This is derived from the attribute in Product.attributes.
pub enum GoogleCloudRetailV2CatalogAttributeTypeEnum {
    

    /// The type of the attribute is unknown. Used when type cannot be derived from attribute that is not in_use.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Textual attribute.
    ///
    /// "TEXTUAL"
    #[serde(rename="TEXTUAL")]
    TEXTUAL,
    

    /// Numerical attribute.
    ///
    /// "NUMERICAL"
    #[serde(rename="NUMERICAL")]
    NUMERICAL,
}

impl AsRef<str> for GoogleCloudRetailV2CatalogAttributeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2CatalogAttributeTypeEnum::UNKNOWN => "UNKNOWN",
            GoogleCloudRetailV2CatalogAttributeTypeEnum::TEXTUAL => "TEXTUAL",
            GoogleCloudRetailV2CatalogAttributeTypeEnum::NUMERICAL => "NUMERICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2CatalogAttributeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(GoogleCloudRetailV2CatalogAttributeTypeEnum::UNKNOWN),
           "TEXTUAL" => Ok(GoogleCloudRetailV2CatalogAttributeTypeEnum::TEXTUAL),
           "NUMERICAL" => Ok(GoogleCloudRetailV2CatalogAttributeTypeEnum::NUMERICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2CatalogAttributeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ControlSearchSolutionUseCaseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the use case for the control. Affects what condition fields can be set. Only settable by search controls. Will default to SEARCH_SOLUTION_USE_CASE_SEARCH if not specified. Currently only allow one search_solution_use_case per control.
pub enum GoogleCloudRetailV2ControlSearchSolutionUseCaseEnum {
    

    /// The value when it's unspecified. In this case, server behavior defaults to SEARCH_SOLUTION_USE_CASE_SEARCH.
    ///
    /// "SEARCH_SOLUTION_USE_CASE_UNSPECIFIED"
    #[serde(rename="SEARCH_SOLUTION_USE_CASE_UNSPECIFIED")]
    SEARCHSOLUTIONUSECASEUNSPECIFIED,
    

    /// Search use case. Expects the traffic has a non-empty query.
    ///
    /// "SEARCH_SOLUTION_USE_CASE_SEARCH"
    #[serde(rename="SEARCH_SOLUTION_USE_CASE_SEARCH")]
    SEARCHSOLUTIONUSECASESEARCH,
    

    /// Browse use case. Expects the traffic has an empty query.
    ///
    /// "SEARCH_SOLUTION_USE_CASE_BROWSE"
    #[serde(rename="SEARCH_SOLUTION_USE_CASE_BROWSE")]
    SEARCHSOLUTIONUSECASEBROWSE,
}

impl AsRef<str> for GoogleCloudRetailV2ControlSearchSolutionUseCaseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ControlSearchSolutionUseCaseEnum::SEARCHSOLUTIONUSECASEUNSPECIFIED => "SEARCH_SOLUTION_USE_CASE_UNSPECIFIED",
            GoogleCloudRetailV2ControlSearchSolutionUseCaseEnum::SEARCHSOLUTIONUSECASESEARCH => "SEARCH_SOLUTION_USE_CASE_SEARCH",
            GoogleCloudRetailV2ControlSearchSolutionUseCaseEnum::SEARCHSOLUTIONUSECASEBROWSE => "SEARCH_SOLUTION_USE_CASE_BROWSE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ControlSearchSolutionUseCaseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEARCH_SOLUTION_USE_CASE_UNSPECIFIED" => Ok(GoogleCloudRetailV2ControlSearchSolutionUseCaseEnum::SEARCHSOLUTIONUSECASEUNSPECIFIED),
           "SEARCH_SOLUTION_USE_CASE_SEARCH" => Ok(GoogleCloudRetailV2ControlSearchSolutionUseCaseEnum::SEARCHSOLUTIONUSECASESEARCH),
           "SEARCH_SOLUTION_USE_CASE_BROWSE" => Ok(GoogleCloudRetailV2ControlSearchSolutionUseCaseEnum::SEARCHSOLUTIONUSECASEBROWSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ControlSearchSolutionUseCaseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ControlSolutionTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The solution types that the control is used for. Currently we support setting only one type of solution at creation time. Only `SOLUTION_TYPE_SEARCH` value is supported at the moment. If no solution type is provided at creation time, will default to SOLUTION_TYPE_SEARCH.
pub enum GoogleCloudRetailV2ControlSolutionTypesEnum {
    

    /// Default value.
    ///
    /// "SOLUTION_TYPE_UNSPECIFIED"
    #[serde(rename="SOLUTION_TYPE_UNSPECIFIED")]
    SOLUTIONTYPEUNSPECIFIED,
    

    /// Used for Recommendations AI.
    ///
    /// "SOLUTION_TYPE_RECOMMENDATION"
    #[serde(rename="SOLUTION_TYPE_RECOMMENDATION")]
    SOLUTIONTYPERECOMMENDATION,
    

    /// Used for Retail Search.
    ///
    /// "SOLUTION_TYPE_SEARCH"
    #[serde(rename="SOLUTION_TYPE_SEARCH")]
    SOLUTIONTYPESEARCH,
}

impl AsRef<str> for GoogleCloudRetailV2ControlSolutionTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ControlSolutionTypesEnum::SOLUTIONTYPEUNSPECIFIED => "SOLUTION_TYPE_UNSPECIFIED",
            GoogleCloudRetailV2ControlSolutionTypesEnum::SOLUTIONTYPERECOMMENDATION => "SOLUTION_TYPE_RECOMMENDATION",
            GoogleCloudRetailV2ControlSolutionTypesEnum::SOLUTIONTYPESEARCH => "SOLUTION_TYPE_SEARCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ControlSolutionTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SOLUTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudRetailV2ControlSolutionTypesEnum::SOLUTIONTYPEUNSPECIFIED),
           "SOLUTION_TYPE_RECOMMENDATION" => Ok(GoogleCloudRetailV2ControlSolutionTypesEnum::SOLUTIONTYPERECOMMENDATION),
           "SOLUTION_TYPE_SEARCH" => Ok(GoogleCloudRetailV2ControlSolutionTypesEnum::SOLUTIONTYPESEARCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ControlSolutionTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ImportProductsRequestReconciliationModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The mode of reconciliation between existing products and the products to be imported. Defaults to ReconciliationMode.INCREMENTAL.
pub enum GoogleCloudRetailV2ImportProductsRequestReconciliationModeEnum {
    

    /// Defaults to INCREMENTAL.
    ///
    /// "RECONCILIATION_MODE_UNSPECIFIED"
    #[serde(rename="RECONCILIATION_MODE_UNSPECIFIED")]
    RECONCILIATIONMODEUNSPECIFIED,
    

    /// Inserts new products or updates existing products.
    ///
    /// "INCREMENTAL"
    #[serde(rename="INCREMENTAL")]
    INCREMENTAL,
    

    /// Calculates diff and replaces the entire product dataset. Existing products may be deleted if they are not present in the source location.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for GoogleCloudRetailV2ImportProductsRequestReconciliationModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ImportProductsRequestReconciliationModeEnum::RECONCILIATIONMODEUNSPECIFIED => "RECONCILIATION_MODE_UNSPECIFIED",
            GoogleCloudRetailV2ImportProductsRequestReconciliationModeEnum::INCREMENTAL => "INCREMENTAL",
            GoogleCloudRetailV2ImportProductsRequestReconciliationModeEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ImportProductsRequestReconciliationModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECONCILIATION_MODE_UNSPECIFIED" => Ok(GoogleCloudRetailV2ImportProductsRequestReconciliationModeEnum::RECONCILIATIONMODEUNSPECIFIED),
           "INCREMENTAL" => Ok(GoogleCloudRetailV2ImportProductsRequestReconciliationModeEnum::INCREMENTAL),
           "FULL" => Ok(GoogleCloudRetailV2ImportProductsRequestReconciliationModeEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ImportProductsRequestReconciliationModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ModelDataStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of data requirements for this model: `DATA_OK` and `DATA_ERROR`. Recommendation model cannot be trained if the data is in `DATA_ERROR` state. Recommendation model can have `DATA_ERROR` state even if serving state is `ACTIVE`: models were trained successfully before, but cannot be refreshed because model no longer has sufficient data for training.
pub enum GoogleCloudRetailV2ModelDataStateEnum {
    

    /// Unspecified default value, should never be explicitly set.
    ///
    /// "DATA_STATE_UNSPECIFIED"
    #[serde(rename="DATA_STATE_UNSPECIFIED")]
    DATASTATEUNSPECIFIED,
    

    /// The model has sufficient training data.
    ///
    /// "DATA_OK"
    #[serde(rename="DATA_OK")]
    DATAOK,
    

    /// The model does not have sufficient training data. Error messages can be queried via Stackdriver.
    ///
    /// "DATA_ERROR"
    #[serde(rename="DATA_ERROR")]
    DATAERROR,
}

impl AsRef<str> for GoogleCloudRetailV2ModelDataStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ModelDataStateEnum::DATASTATEUNSPECIFIED => "DATA_STATE_UNSPECIFIED",
            GoogleCloudRetailV2ModelDataStateEnum::DATAOK => "DATA_OK",
            GoogleCloudRetailV2ModelDataStateEnum::DATAERROR => "DATA_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ModelDataStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_STATE_UNSPECIFIED" => Ok(GoogleCloudRetailV2ModelDataStateEnum::DATASTATEUNSPECIFIED),
           "DATA_OK" => Ok(GoogleCloudRetailV2ModelDataStateEnum::DATAOK),
           "DATA_ERROR" => Ok(GoogleCloudRetailV2ModelDataStateEnum::DATAERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ModelDataStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ModelFilteringOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. If `RECOMMENDATIONS_FILTERING_ENABLED`, recommendation filtering by attributes is enabled for the model.
pub enum GoogleCloudRetailV2ModelFilteringOptionEnum {
    

    /// Value used when unset. In this case, server behavior defaults to RECOMMENDATIONS_FILTERING_DISABLED.
    ///
    /// "RECOMMENDATIONS_FILTERING_OPTION_UNSPECIFIED"
    #[serde(rename="RECOMMENDATIONS_FILTERING_OPTION_UNSPECIFIED")]
    RECOMMENDATIONSFILTERINGOPTIONUNSPECIFIED,
    

    /// Recommendation filtering is disabled.
    ///
    /// "RECOMMENDATIONS_FILTERING_DISABLED"
    #[serde(rename="RECOMMENDATIONS_FILTERING_DISABLED")]
    RECOMMENDATIONSFILTERINGDISABLED,
    

    /// Recommendation filtering is enabled.
    ///
    /// "RECOMMENDATIONS_FILTERING_ENABLED"
    #[serde(rename="RECOMMENDATIONS_FILTERING_ENABLED")]
    RECOMMENDATIONSFILTERINGENABLED,
}

impl AsRef<str> for GoogleCloudRetailV2ModelFilteringOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ModelFilteringOptionEnum::RECOMMENDATIONSFILTERINGOPTIONUNSPECIFIED => "RECOMMENDATIONS_FILTERING_OPTION_UNSPECIFIED",
            GoogleCloudRetailV2ModelFilteringOptionEnum::RECOMMENDATIONSFILTERINGDISABLED => "RECOMMENDATIONS_FILTERING_DISABLED",
            GoogleCloudRetailV2ModelFilteringOptionEnum::RECOMMENDATIONSFILTERINGENABLED => "RECOMMENDATIONS_FILTERING_ENABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ModelFilteringOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECOMMENDATIONS_FILTERING_OPTION_UNSPECIFIED" => Ok(GoogleCloudRetailV2ModelFilteringOptionEnum::RECOMMENDATIONSFILTERINGOPTIONUNSPECIFIED),
           "RECOMMENDATIONS_FILTERING_DISABLED" => Ok(GoogleCloudRetailV2ModelFilteringOptionEnum::RECOMMENDATIONSFILTERINGDISABLED),
           "RECOMMENDATIONS_FILTERING_ENABLED" => Ok(GoogleCloudRetailV2ModelFilteringOptionEnum::RECOMMENDATIONSFILTERINGENABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ModelFilteringOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ModelPeriodicTuningStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The state of periodic tuning. The period we use is 3 months - to do a one-off tune earlier use the `TuneModel` method. Default value is `PERIODIC_TUNING_ENABLED`.
pub enum GoogleCloudRetailV2ModelPeriodicTuningStateEnum {
    

    /// Unspecified default value, should never be explicitly set.
    ///
    /// "PERIODIC_TUNING_STATE_UNSPECIFIED"
    #[serde(rename="PERIODIC_TUNING_STATE_UNSPECIFIED")]
    PERIODICTUNINGSTATEUNSPECIFIED,
    

    /// The model has periodic tuning disabled. Tuning can be reenabled by calling the `EnableModelPeriodicTuning` method or by calling the `TuneModel` method.
    ///
    /// "PERIODIC_TUNING_DISABLED"
    #[serde(rename="PERIODIC_TUNING_DISABLED")]
    PERIODICTUNINGDISABLED,
    

    /// The model cannot be tuned with periodic tuning OR the `TuneModel` method. Hide the options in customer UI and reject any requests through the backend self serve API.
    ///
    /// "ALL_TUNING_DISABLED"
    #[serde(rename="ALL_TUNING_DISABLED")]
    ALLTUNINGDISABLED,
    

    /// The model has periodic tuning enabled. Tuning can be disabled by calling the `DisableModelPeriodicTuning` method.
    ///
    /// "PERIODIC_TUNING_ENABLED"
    #[serde(rename="PERIODIC_TUNING_ENABLED")]
    PERIODICTUNINGENABLED,
}

impl AsRef<str> for GoogleCloudRetailV2ModelPeriodicTuningStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ModelPeriodicTuningStateEnum::PERIODICTUNINGSTATEUNSPECIFIED => "PERIODIC_TUNING_STATE_UNSPECIFIED",
            GoogleCloudRetailV2ModelPeriodicTuningStateEnum::PERIODICTUNINGDISABLED => "PERIODIC_TUNING_DISABLED",
            GoogleCloudRetailV2ModelPeriodicTuningStateEnum::ALLTUNINGDISABLED => "ALL_TUNING_DISABLED",
            GoogleCloudRetailV2ModelPeriodicTuningStateEnum::PERIODICTUNINGENABLED => "PERIODIC_TUNING_ENABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ModelPeriodicTuningStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERIODIC_TUNING_STATE_UNSPECIFIED" => Ok(GoogleCloudRetailV2ModelPeriodicTuningStateEnum::PERIODICTUNINGSTATEUNSPECIFIED),
           "PERIODIC_TUNING_DISABLED" => Ok(GoogleCloudRetailV2ModelPeriodicTuningStateEnum::PERIODICTUNINGDISABLED),
           "ALL_TUNING_DISABLED" => Ok(GoogleCloudRetailV2ModelPeriodicTuningStateEnum::ALLTUNINGDISABLED),
           "PERIODIC_TUNING_ENABLED" => Ok(GoogleCloudRetailV2ModelPeriodicTuningStateEnum::PERIODICTUNINGENABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ModelPeriodicTuningStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ModelServingStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The serving state of the model: `ACTIVE`, `NOT_ACTIVE`.
pub enum GoogleCloudRetailV2ModelServingStateEnum {
    

    /// Unspecified serving state.
    ///
    /// "SERVING_STATE_UNSPECIFIED"
    #[serde(rename="SERVING_STATE_UNSPECIFIED")]
    SERVINGSTATEUNSPECIFIED,
    

    /// The model is not serving.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// The model is serving and can be queried.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The model is trained on tuned hyperparameters and can be queried.
    ///
    /// "TUNED"
    #[serde(rename="TUNED")]
    TUNED,
}

impl AsRef<str> for GoogleCloudRetailV2ModelServingStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ModelServingStateEnum::SERVINGSTATEUNSPECIFIED => "SERVING_STATE_UNSPECIFIED",
            GoogleCloudRetailV2ModelServingStateEnum::INACTIVE => "INACTIVE",
            GoogleCloudRetailV2ModelServingStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudRetailV2ModelServingStateEnum::TUNED => "TUNED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ModelServingStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVING_STATE_UNSPECIFIED" => Ok(GoogleCloudRetailV2ModelServingStateEnum::SERVINGSTATEUNSPECIFIED),
           "INACTIVE" => Ok(GoogleCloudRetailV2ModelServingStateEnum::INACTIVE),
           "ACTIVE" => Ok(GoogleCloudRetailV2ModelServingStateEnum::ACTIVE),
           "TUNED" => Ok(GoogleCloudRetailV2ModelServingStateEnum::TUNED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ModelServingStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ModelTrainingStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The training state that the model is in (e.g. `TRAINING` or `PAUSED`). Since part of the cost of running the service is frequency of training - this can be used to determine when to train model in order to control cost. If not specified: the default value for `CreateModel` method is `TRAINING`. The default value for `UpdateModel` method is to keep the state the same as before.
pub enum GoogleCloudRetailV2ModelTrainingStateEnum {
    

    /// Unspecified training state.
    ///
    /// "TRAINING_STATE_UNSPECIFIED"
    #[serde(rename="TRAINING_STATE_UNSPECIFIED")]
    TRAININGSTATEUNSPECIFIED,
    

    /// The model training is paused.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// The model is training.
    ///
    /// "TRAINING"
    #[serde(rename="TRAINING")]
    TRAINING,
}

impl AsRef<str> for GoogleCloudRetailV2ModelTrainingStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ModelTrainingStateEnum::TRAININGSTATEUNSPECIFIED => "TRAINING_STATE_UNSPECIFIED",
            GoogleCloudRetailV2ModelTrainingStateEnum::PAUSED => "PAUSED",
            GoogleCloudRetailV2ModelTrainingStateEnum::TRAINING => "TRAINING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ModelTrainingStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRAINING_STATE_UNSPECIFIED" => Ok(GoogleCloudRetailV2ModelTrainingStateEnum::TRAININGSTATEUNSPECIFIED),
           "PAUSED" => Ok(GoogleCloudRetailV2ModelTrainingStateEnum::PAUSED),
           "TRAINING" => Ok(GoogleCloudRetailV2ModelTrainingStateEnum::TRAINING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ModelTrainingStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ModelFrequentlyBoughtTogetherFeaturesConfigContextProductsTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies the context of the model when it is used in predict requests. Can only be set for the `frequently-bought-together` type. If it isn't specified, it defaults to MULTIPLE_CONTEXT_PRODUCTS.
pub enum GoogleCloudRetailV2ModelFrequentlyBoughtTogetherFeaturesConfigContextProductsTypeEnum {
    

    /// Unspecified default value, should never be explicitly set. Defaults to MULTIPLE_CONTEXT_PRODUCTS.
    ///
    /// "CONTEXT_PRODUCTS_TYPE_UNSPECIFIED"
    #[serde(rename="CONTEXT_PRODUCTS_TYPE_UNSPECIFIED")]
    CONTEXTPRODUCTSTYPEUNSPECIFIED,
    

    /// Use only a single product as context for the recommendation. Typically used on pages like add-to-cart or product details.
    ///
    /// "SINGLE_CONTEXT_PRODUCT"
    #[serde(rename="SINGLE_CONTEXT_PRODUCT")]
    SINGLECONTEXTPRODUCT,
    

    /// Use one or multiple products as context for the recommendation. Typically used on shopping cart pages.
    ///
    /// "MULTIPLE_CONTEXT_PRODUCTS"
    #[serde(rename="MULTIPLE_CONTEXT_PRODUCTS")]
    MULTIPLECONTEXTPRODUCTS,
}

impl AsRef<str> for GoogleCloudRetailV2ModelFrequentlyBoughtTogetherFeaturesConfigContextProductsTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ModelFrequentlyBoughtTogetherFeaturesConfigContextProductsTypeEnum::CONTEXTPRODUCTSTYPEUNSPECIFIED => "CONTEXT_PRODUCTS_TYPE_UNSPECIFIED",
            GoogleCloudRetailV2ModelFrequentlyBoughtTogetherFeaturesConfigContextProductsTypeEnum::SINGLECONTEXTPRODUCT => "SINGLE_CONTEXT_PRODUCT",
            GoogleCloudRetailV2ModelFrequentlyBoughtTogetherFeaturesConfigContextProductsTypeEnum::MULTIPLECONTEXTPRODUCTS => "MULTIPLE_CONTEXT_PRODUCTS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ModelFrequentlyBoughtTogetherFeaturesConfigContextProductsTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTEXT_PRODUCTS_TYPE_UNSPECIFIED" => Ok(GoogleCloudRetailV2ModelFrequentlyBoughtTogetherFeaturesConfigContextProductsTypeEnum::CONTEXTPRODUCTSTYPEUNSPECIFIED),
           "SINGLE_CONTEXT_PRODUCT" => Ok(GoogleCloudRetailV2ModelFrequentlyBoughtTogetherFeaturesConfigContextProductsTypeEnum::SINGLECONTEXTPRODUCT),
           "MULTIPLE_CONTEXT_PRODUCTS" => Ok(GoogleCloudRetailV2ModelFrequentlyBoughtTogetherFeaturesConfigContextProductsTypeEnum::MULTIPLECONTEXTPRODUCTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ModelFrequentlyBoughtTogetherFeaturesConfigContextProductsTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ProductAvailabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The online availability of the Product. Default to Availability.IN_STOCK. Corresponding properties: Google Merchant Center property [availability](https://support.google.com/merchants/answer/6324448). Schema.org property [Offer.availability](https://schema.org/availability).
pub enum GoogleCloudRetailV2ProductAvailabilityEnum {
    

    /// Default product availability. Default to Availability.IN_STOCK if unset.
    ///
    /// "AVAILABILITY_UNSPECIFIED"
    #[serde(rename="AVAILABILITY_UNSPECIFIED")]
    AVAILABILITYUNSPECIFIED,
    

    /// Product in stock.
    ///
    /// "IN_STOCK"
    #[serde(rename="IN_STOCK")]
    INSTOCK,
    

    /// Product out of stock.
    ///
    /// "OUT_OF_STOCK"
    #[serde(rename="OUT_OF_STOCK")]
    OUTOFSTOCK,
    

    /// Product that is in pre-order state.
    ///
    /// "PREORDER"
    #[serde(rename="PREORDER")]
    PREORDER,
    

    /// Product that is back-ordered (i.e. temporarily out of stock).
    ///
    /// "BACKORDER"
    #[serde(rename="BACKORDER")]
    BACKORDER,
}

impl AsRef<str> for GoogleCloudRetailV2ProductAvailabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ProductAvailabilityEnum::AVAILABILITYUNSPECIFIED => "AVAILABILITY_UNSPECIFIED",
            GoogleCloudRetailV2ProductAvailabilityEnum::INSTOCK => "IN_STOCK",
            GoogleCloudRetailV2ProductAvailabilityEnum::OUTOFSTOCK => "OUT_OF_STOCK",
            GoogleCloudRetailV2ProductAvailabilityEnum::PREORDER => "PREORDER",
            GoogleCloudRetailV2ProductAvailabilityEnum::BACKORDER => "BACKORDER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ProductAvailabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AVAILABILITY_UNSPECIFIED" => Ok(GoogleCloudRetailV2ProductAvailabilityEnum::AVAILABILITYUNSPECIFIED),
           "IN_STOCK" => Ok(GoogleCloudRetailV2ProductAvailabilityEnum::INSTOCK),
           "OUT_OF_STOCK" => Ok(GoogleCloudRetailV2ProductAvailabilityEnum::OUTOFSTOCK),
           "PREORDER" => Ok(GoogleCloudRetailV2ProductAvailabilityEnum::PREORDER),
           "BACKORDER" => Ok(GoogleCloudRetailV2ProductAvailabilityEnum::BACKORDER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ProductAvailabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ProductTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The type of the product. Default to Catalog.product_level_config.ingestion_product_type if unset.
pub enum GoogleCloudRetailV2ProductTypeEnum {
    

    /// Default value. Default to Catalog.product_level_config.ingestion_product_type if unset.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The primary type. As the primary unit for predicting, indexing and search serving, a Type.PRIMARY Product is grouped with multiple Type.VARIANT Products.
    ///
    /// "PRIMARY"
    #[serde(rename="PRIMARY")]
    PRIMARY,
    

    /// The variant type. Type.VARIANT Products usually share some common attributes on the same Type.PRIMARY Products, but they have variant attributes like different colors, sizes and prices, etc.
    ///
    /// "VARIANT"
    #[serde(rename="VARIANT")]
    VARIANT,
    

    /// The collection type. Collection products are bundled Type.PRIMARY Products or Type.VARIANT Products that are sold together, such as a jewelry set with necklaces, earrings and rings, etc.
    ///
    /// "COLLECTION"
    #[serde(rename="COLLECTION")]
    COLLECTION,
}

impl AsRef<str> for GoogleCloudRetailV2ProductTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ProductTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudRetailV2ProductTypeEnum::PRIMARY => "PRIMARY",
            GoogleCloudRetailV2ProductTypeEnum::VARIANT => "VARIANT",
            GoogleCloudRetailV2ProductTypeEnum::COLLECTION => "COLLECTION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ProductTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudRetailV2ProductTypeEnum::TYPEUNSPECIFIED),
           "PRIMARY" => Ok(GoogleCloudRetailV2ProductTypeEnum::PRIMARY),
           "VARIANT" => Ok(GoogleCloudRetailV2ProductTypeEnum::VARIANT),
           "COLLECTION" => Ok(GoogleCloudRetailV2ProductTypeEnum::COLLECTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ProductTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the user event rejoin to define the scope and range of the user events to be rejoined with the latest product catalog. Defaults to `USER_EVENT_REJOIN_SCOPE_UNSPECIFIED` if this field is not set, or set to an invalid integer value.
pub enum GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum {
    

    /// Rejoin all events with the latest product catalog, including both joined events and unjoined events.
    ///
    /// "USER_EVENT_REJOIN_SCOPE_UNSPECIFIED"
    #[serde(rename="USER_EVENT_REJOIN_SCOPE_UNSPECIFIED")]
    USEREVENTREJOINSCOPEUNSPECIFIED,
    

    /// Only rejoin joined events with the latest product catalog.
    ///
    /// "JOINED_EVENTS"
    #[serde(rename="JOINED_EVENTS")]
    JOINEDEVENTS,
    

    /// Only rejoin unjoined events with the latest product catalog.
    ///
    /// "UNJOINED_EVENTS"
    #[serde(rename="UNJOINED_EVENTS")]
    UNJOINEDEVENTS,
}

impl AsRef<str> for GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum::USEREVENTREJOINSCOPEUNSPECIFIED => "USER_EVENT_REJOIN_SCOPE_UNSPECIFIED",
            GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum::JOINEDEVENTS => "JOINED_EVENTS",
            GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum::UNJOINEDEVENTS => "UNJOINED_EVENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_EVENT_REJOIN_SCOPE_UNSPECIFIED" => Ok(GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum::USEREVENTREJOINSCOPEUNSPECIFIED),
           "JOINED_EVENTS" => Ok(GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum::JOINEDEVENTS),
           "UNJOINED_EVENTS" => Ok(GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum::UNJOINEDEVENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2SearchRequestSearchModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The search mode of the search request. If not specified, a single search request triggers both product search and faceted search.
pub enum GoogleCloudRetailV2SearchRequestSearchModeEnum {
    

    /// Default value. In this case both product search and faceted search will be performed. Both SearchResponse.SearchResult and SearchResponse.Facet will be returned.
    ///
    /// "SEARCH_MODE_UNSPECIFIED"
    #[serde(rename="SEARCH_MODE_UNSPECIFIED")]
    SEARCHMODEUNSPECIFIED,
    

    /// Only product search will be performed. The faceted search will be disabled. Only SearchResponse.SearchResult will be returned. SearchResponse.Facet will not be returned, even if SearchRequest.facet_specs or SearchRequest.dynamic_facet_spec is set.
    ///
    /// "PRODUCT_SEARCH_ONLY"
    #[serde(rename="PRODUCT_SEARCH_ONLY")]
    PRODUCTSEARCHONLY,
    

    /// Only faceted search will be performed. The product search will be disabled. When in this mode, one or both of SearchRequest.facet_specs and SearchRequest.dynamic_facet_spec should be set. Otherwise, an INVALID_ARGUMENT error is returned. Only SearchResponse.Facet will be returned. SearchResponse.SearchResult will not be returned.
    ///
    /// "FACETED_SEARCH_ONLY"
    #[serde(rename="FACETED_SEARCH_ONLY")]
    FACETEDSEARCHONLY,
}

impl AsRef<str> for GoogleCloudRetailV2SearchRequestSearchModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2SearchRequestSearchModeEnum::SEARCHMODEUNSPECIFIED => "SEARCH_MODE_UNSPECIFIED",
            GoogleCloudRetailV2SearchRequestSearchModeEnum::PRODUCTSEARCHONLY => "PRODUCT_SEARCH_ONLY",
            GoogleCloudRetailV2SearchRequestSearchModeEnum::FACETEDSEARCHONLY => "FACETED_SEARCH_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2SearchRequestSearchModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEARCH_MODE_UNSPECIFIED" => Ok(GoogleCloudRetailV2SearchRequestSearchModeEnum::SEARCHMODEUNSPECIFIED),
           "PRODUCT_SEARCH_ONLY" => Ok(GoogleCloudRetailV2SearchRequestSearchModeEnum::PRODUCTSEARCHONLY),
           "FACETED_SEARCH_ONLY" => Ok(GoogleCloudRetailV2SearchRequestSearchModeEnum::FACETEDSEARCHONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2SearchRequestSearchModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2SearchRequestDynamicFacetSpecModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mode of the DynamicFacet feature. Defaults to Mode.DISABLED if it's unset.
pub enum GoogleCloudRetailV2SearchRequestDynamicFacetSpecModeEnum {
    

    /// Default value.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// Disable Dynamic Facet.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Automatic mode built by Google Retail Search.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
}

impl AsRef<str> for GoogleCloudRetailV2SearchRequestDynamicFacetSpecModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2SearchRequestDynamicFacetSpecModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            GoogleCloudRetailV2SearchRequestDynamicFacetSpecModeEnum::DISABLED => "DISABLED",
            GoogleCloudRetailV2SearchRequestDynamicFacetSpecModeEnum::ENABLED => "ENABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2SearchRequestDynamicFacetSpecModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(GoogleCloudRetailV2SearchRequestDynamicFacetSpecModeEnum::MODEUNSPECIFIED),
           "DISABLED" => Ok(GoogleCloudRetailV2SearchRequestDynamicFacetSpecModeEnum::DISABLED),
           "ENABLED" => Ok(GoogleCloudRetailV2SearchRequestDynamicFacetSpecModeEnum::ENABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2SearchRequestDynamicFacetSpecModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2SearchRequestPersonalizationSpecModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defaults to Mode.AUTO.
pub enum GoogleCloudRetailV2SearchRequestPersonalizationSpecModeEnum {
    

    /// Default value. In this case, server behavior defaults to Mode.AUTO.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// Let CRS decide whether to use personalization based on quality of user event data.
    ///
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
    

    /// Disable personalization.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for GoogleCloudRetailV2SearchRequestPersonalizationSpecModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2SearchRequestPersonalizationSpecModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            GoogleCloudRetailV2SearchRequestPersonalizationSpecModeEnum::AUTO => "AUTO",
            GoogleCloudRetailV2SearchRequestPersonalizationSpecModeEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2SearchRequestPersonalizationSpecModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(GoogleCloudRetailV2SearchRequestPersonalizationSpecModeEnum::MODEUNSPECIFIED),
           "AUTO" => Ok(GoogleCloudRetailV2SearchRequestPersonalizationSpecModeEnum::AUTO),
           "DISABLED" => Ok(GoogleCloudRetailV2SearchRequestPersonalizationSpecModeEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2SearchRequestPersonalizationSpecModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2SearchRequestQueryExpansionSpecConditionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The condition under which query expansion should occur. Default to Condition.DISABLED.
pub enum GoogleCloudRetailV2SearchRequestQueryExpansionSpecConditionEnum {
    

    /// Unspecified query expansion condition. In this case, server behavior defaults to Condition.DISABLED.
    ///
    /// "CONDITION_UNSPECIFIED"
    #[serde(rename="CONDITION_UNSPECIFIED")]
    CONDITIONUNSPECIFIED,
    

    /// Disabled query expansion. Only the exact search query is used, even if SearchResponse.total_size is zero.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Automatic query expansion built by Google Retail Search.
    ///
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
}

impl AsRef<str> for GoogleCloudRetailV2SearchRequestQueryExpansionSpecConditionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2SearchRequestQueryExpansionSpecConditionEnum::CONDITIONUNSPECIFIED => "CONDITION_UNSPECIFIED",
            GoogleCloudRetailV2SearchRequestQueryExpansionSpecConditionEnum::DISABLED => "DISABLED",
            GoogleCloudRetailV2SearchRequestQueryExpansionSpecConditionEnum::AUTO => "AUTO",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2SearchRequestQueryExpansionSpecConditionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONDITION_UNSPECIFIED" => Ok(GoogleCloudRetailV2SearchRequestQueryExpansionSpecConditionEnum::CONDITIONUNSPECIFIED),
           "DISABLED" => Ok(GoogleCloudRetailV2SearchRequestQueryExpansionSpecConditionEnum::DISABLED),
           "AUTO" => Ok(GoogleCloudRetailV2SearchRequestQueryExpansionSpecConditionEnum::AUTO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2SearchRequestQueryExpansionSpecConditionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2SearchRequestSpellCorrectionSpecModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The mode under which spell correction should take effect to replace the original search query. Default to Mode.AUTO.
pub enum GoogleCloudRetailV2SearchRequestSpellCorrectionSpecModeEnum {
    

    /// Unspecified spell correction mode. In this case, server behavior defaults to Mode.AUTO.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// Google Retail Search will try to find a spell suggestion if there is any and put in the SearchResponse.corrected_query. The spell suggestion will not be used as the search query.
    ///
    /// "SUGGESTION_ONLY"
    #[serde(rename="SUGGESTION_ONLY")]
    SUGGESTIONONLY,
    

    /// Automatic spell correction built by Google Retail Search. Search will be based on the corrected query if found.
    ///
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
}

impl AsRef<str> for GoogleCloudRetailV2SearchRequestSpellCorrectionSpecModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2SearchRequestSpellCorrectionSpecModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            GoogleCloudRetailV2SearchRequestSpellCorrectionSpecModeEnum::SUGGESTIONONLY => "SUGGESTION_ONLY",
            GoogleCloudRetailV2SearchRequestSpellCorrectionSpecModeEnum::AUTO => "AUTO",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2SearchRequestSpellCorrectionSpecModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(GoogleCloudRetailV2SearchRequestSpellCorrectionSpecModeEnum::MODEUNSPECIFIED),
           "SUGGESTION_ONLY" => Ok(GoogleCloudRetailV2SearchRequestSpellCorrectionSpecModeEnum::SUGGESTIONONLY),
           "AUTO" => Ok(GoogleCloudRetailV2SearchRequestSpellCorrectionSpecModeEnum::AUTO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2SearchRequestSpellCorrectionSpecModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ServingConfigDiversityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// What kind of diversity to use - data driven or rule based. If unset, the server behavior defaults to RULE_BASED_DIVERSITY.
pub enum GoogleCloudRetailV2ServingConfigDiversityTypeEnum {
    

    /// Default value.
    ///
    /// "DIVERSITY_TYPE_UNSPECIFIED"
    #[serde(rename="DIVERSITY_TYPE_UNSPECIFIED")]
    DIVERSITYTYPEUNSPECIFIED,
    

    /// Rule based diversity.
    ///
    /// "RULE_BASED_DIVERSITY"
    #[serde(rename="RULE_BASED_DIVERSITY")]
    RULEBASEDDIVERSITY,
    

    /// Data driven diversity.
    ///
    /// "DATA_DRIVEN_DIVERSITY"
    #[serde(rename="DATA_DRIVEN_DIVERSITY")]
    DATADRIVENDIVERSITY,
}

impl AsRef<str> for GoogleCloudRetailV2ServingConfigDiversityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ServingConfigDiversityTypeEnum::DIVERSITYTYPEUNSPECIFIED => "DIVERSITY_TYPE_UNSPECIFIED",
            GoogleCloudRetailV2ServingConfigDiversityTypeEnum::RULEBASEDDIVERSITY => "RULE_BASED_DIVERSITY",
            GoogleCloudRetailV2ServingConfigDiversityTypeEnum::DATADRIVENDIVERSITY => "DATA_DRIVEN_DIVERSITY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ServingConfigDiversityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIVERSITY_TYPE_UNSPECIFIED" => Ok(GoogleCloudRetailV2ServingConfigDiversityTypeEnum::DIVERSITYTYPEUNSPECIFIED),
           "RULE_BASED_DIVERSITY" => Ok(GoogleCloudRetailV2ServingConfigDiversityTypeEnum::RULEBASEDDIVERSITY),
           "DATA_DRIVEN_DIVERSITY" => Ok(GoogleCloudRetailV2ServingConfigDiversityTypeEnum::DATADRIVENDIVERSITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ServingConfigDiversityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRetailV2ServingConfigSolutionTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. Specifies the solution types that a serving config can be associated with. Currently we support setting only one type of solution.
pub enum GoogleCloudRetailV2ServingConfigSolutionTypesEnum {
    

    /// Default value.
    ///
    /// "SOLUTION_TYPE_UNSPECIFIED"
    #[serde(rename="SOLUTION_TYPE_UNSPECIFIED")]
    SOLUTIONTYPEUNSPECIFIED,
    

    /// Used for Recommendations AI.
    ///
    /// "SOLUTION_TYPE_RECOMMENDATION"
    #[serde(rename="SOLUTION_TYPE_RECOMMENDATION")]
    SOLUTIONTYPERECOMMENDATION,
    

    /// Used for Retail Search.
    ///
    /// "SOLUTION_TYPE_SEARCH"
    #[serde(rename="SOLUTION_TYPE_SEARCH")]
    SOLUTIONTYPESEARCH,
}

impl AsRef<str> for GoogleCloudRetailV2ServingConfigSolutionTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRetailV2ServingConfigSolutionTypesEnum::SOLUTIONTYPEUNSPECIFIED => "SOLUTION_TYPE_UNSPECIFIED",
            GoogleCloudRetailV2ServingConfigSolutionTypesEnum::SOLUTIONTYPERECOMMENDATION => "SOLUTION_TYPE_RECOMMENDATION",
            GoogleCloudRetailV2ServingConfigSolutionTypesEnum::SOLUTIONTYPESEARCH => "SOLUTION_TYPE_SEARCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRetailV2ServingConfigSolutionTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SOLUTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudRetailV2ServingConfigSolutionTypesEnum::SOLUTIONTYPEUNSPECIFIED),
           "SOLUTION_TYPE_RECOMMENDATION" => Ok(GoogleCloudRetailV2ServingConfigSolutionTypesEnum::SOLUTIONTYPERECOMMENDATION),
           "SOLUTION_TYPE_SEARCH" => Ok(GoogleCloudRetailV2ServingConfigSolutionTypesEnum::SOLUTIONTYPESEARCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRetailV2ServingConfigSolutionTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


