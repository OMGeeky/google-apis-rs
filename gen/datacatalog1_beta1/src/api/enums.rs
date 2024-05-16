use super::*;



// region GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The table source type.
pub enum GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum {
    

    /// Default unknown type.
    ///
    /// "TABLE_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="TABLE_SOURCE_TYPE_UNSPECIFIED")]
    TABLESOURCETYPEUNSPECIFIED,
    

    /// Table view.
    ///
    /// "BIGQUERY_VIEW"
    #[serde(rename="BIGQUERY_VIEW")]
    BIGQUERYVIEW,
    

    /// BigQuery native table.
    ///
    /// "BIGQUERY_TABLE"
    #[serde(rename="BIGQUERY_TABLE")]
    BIGQUERYTABLE,
    

    /// BigQuery materialized view.
    ///
    /// "BIGQUERY_MATERIALIZED_VIEW"
    #[serde(rename="BIGQUERY_MATERIALIZED_VIEW")]
    BIGQUERYMATERIALIZEDVIEW,
}

impl AsRef<str> for GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum::TABLESOURCETYPEUNSPECIFIED => "TABLE_SOURCE_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYVIEW => "BIGQUERY_VIEW",
            GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYTABLE => "BIGQUERY_TABLE",
            GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYMATERIALIZEDVIEW => "BIGQUERY_MATERIALIZED_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TABLE_SOURCE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum::TABLESOURCETYPEUNSPECIFIED),
           "BIGQUERY_VIEW" => Ok(GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYVIEW),
           "BIGQUERY_TABLE" => Ok(GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYTABLE),
           "BIGQUERY_MATERIALIZED_VIEW" => Ok(GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYMATERIALIZEDVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. This field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub.
pub enum GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum {
    

    /// Default unknown system.
    ///
    /// "INTEGRATED_SYSTEM_UNSPECIFIED"
    #[serde(rename="INTEGRATED_SYSTEM_UNSPECIFIED")]
    INTEGRATEDSYSTEMUNSPECIFIED,
    

    /// BigQuery.
    ///
    /// "BIGQUERY"
    #[serde(rename="BIGQUERY")]
    BIGQUERY,
    

    /// Cloud Pub/Sub.
    ///
    /// "CLOUD_PUBSUB"
    #[serde(rename="CLOUD_PUBSUB")]
    CLOUDPUBSUB,
}

impl AsRef<str> for GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum::INTEGRATEDSYSTEMUNSPECIFIED => "INTEGRATED_SYSTEM_UNSPECIFIED",
            GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum::BIGQUERY => "BIGQUERY",
            GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum::CLOUDPUBSUB => "CLOUD_PUBSUB",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTEGRATED_SYSTEM_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum::INTEGRATEDSYSTEMUNSPECIFIED),
           "BIGQUERY" => Ok(GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum::BIGQUERY),
           "CLOUD_PUBSUB" => Ok(GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum::CLOUDPUBSUB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1beta1EntryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the entry. Only used for Entries with types in the EntryType enum.
pub enum GoogleCloudDatacatalogV1beta1EntryTypeEnum {
    

    /// Default unknown type.
    ///
    /// "ENTRY_TYPE_UNSPECIFIED"
    #[serde(rename="ENTRY_TYPE_UNSPECIFIED")]
    ENTRYTYPEUNSPECIFIED,
    

    /// Output only. The type of entry that has a GoogleSQL schema, including logical views.
    ///
    /// "TABLE"
    #[serde(rename="TABLE")]
    TABLE,
    

    /// Output only. The type of models. https://cloud.google.com/bigquery-ml/docs/bigqueryml-intro
    ///
    /// "MODEL"
    #[serde(rename="MODEL")]
    MODEL,
    

    /// Output only. An entry type which is used for streaming entries. Example: Pub/Sub topic.
    ///
    /// "DATA_STREAM"
    #[serde(rename="DATA_STREAM")]
    DATASTREAM,
    

    /// An entry type which is a set of files or objects. Example: Cloud Storage fileset.
    ///
    /// "FILESET"
    #[serde(rename="FILESET")]
    FILESET,
}

impl AsRef<str> for GoogleCloudDatacatalogV1beta1EntryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1beta1EntryTypeEnum::ENTRYTYPEUNSPECIFIED => "ENTRY_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1beta1EntryTypeEnum::TABLE => "TABLE",
            GoogleCloudDatacatalogV1beta1EntryTypeEnum::MODEL => "MODEL",
            GoogleCloudDatacatalogV1beta1EntryTypeEnum::DATASTREAM => "DATA_STREAM",
            GoogleCloudDatacatalogV1beta1EntryTypeEnum::FILESET => "FILESET",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1beta1EntryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTRY_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1beta1EntryTypeEnum::ENTRYTYPEUNSPECIFIED),
           "TABLE" => Ok(GoogleCloudDatacatalogV1beta1EntryTypeEnum::TABLE),
           "MODEL" => Ok(GoogleCloudDatacatalogV1beta1EntryTypeEnum::MODEL),
           "DATA_STREAM" => Ok(GoogleCloudDatacatalogV1beta1EntryTypeEnum::DATASTREAM),
           "FILESET" => Ok(GoogleCloudDatacatalogV1beta1EntryTypeEnum::FILESET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1beta1EntryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Represents primitive types - string, bool etc.
pub enum GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum {
    

    /// This is the default invalid value for a type.
    ///
    /// "PRIMITIVE_TYPE_UNSPECIFIED"
    #[serde(rename="PRIMITIVE_TYPE_UNSPECIFIED")]
    PRIMITIVETYPEUNSPECIFIED,
    

    /// A double precision number.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
    

    /// An UTF-8 string.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// A boolean value.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// A timestamp.
    ///
    /// "TIMESTAMP"
    #[serde(rename="TIMESTAMP")]
    TIMESTAMP,
}

impl AsRef<str> for GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum::PRIMITIVETYPEUNSPECIFIED => "PRIMITIVE_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum::DOUBLE => "DOUBLE",
            GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum::STRING => "STRING",
            GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum::BOOL => "BOOL",
            GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum::TIMESTAMP => "TIMESTAMP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIMITIVE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum::PRIMITIVETYPEUNSPECIFIED),
           "DOUBLE" => Ok(GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum::DOUBLE),
           "STRING" => Ok(GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum::STRING),
           "BOOL" => Ok(GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum::BOOL),
           "TIMESTAMP" => Ok(GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum::TIMESTAMP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the search result. This field can be used to determine which Get method to call to fetch the full resource.
pub enum GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum {
    

    /// Default unknown type.
    ///
    /// "SEARCH_RESULT_TYPE_UNSPECIFIED"
    #[serde(rename="SEARCH_RESULT_TYPE_UNSPECIFIED")]
    SEARCHRESULTTYPEUNSPECIFIED,
    

    /// An Entry.
    ///
    /// "ENTRY"
    #[serde(rename="ENTRY")]
    ENTRY,
    

    /// A TagTemplate.
    ///
    /// "TAG_TEMPLATE"
    #[serde(rename="TAG_TEMPLATE")]
    TAGTEMPLATE,
    

    /// An EntryGroup.
    ///
    /// "ENTRY_GROUP"
    #[serde(rename="ENTRY_GROUP")]
    ENTRYGROUP,
}

impl AsRef<str> for GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum::SEARCHRESULTTYPEUNSPECIFIED => "SEARCH_RESULT_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum::ENTRY => "ENTRY",
            GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum::TAGTEMPLATE => "TAG_TEMPLATE",
            GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum::ENTRYGROUP => "ENTRY_GROUP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEARCH_RESULT_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum::SEARCHRESULTTYPEUNSPECIFIED),
           "ENTRY" => Ok(GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum::ENTRY),
           "TAG_TEMPLATE" => Ok(GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum::TAGTEMPLATE),
           "ENTRY_GROUP" => Ok(GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum::ENTRYGROUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of policy types that are activated for a taxonomy.
pub enum GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum {
    

    /// Unspecified policy type.
    ///
    /// "POLICY_TYPE_UNSPECIFIED"
    #[serde(rename="POLICY_TYPE_UNSPECIFIED")]
    POLICYTYPEUNSPECIFIED,
    

    /// Fine grained access control policy, which enables access control on tagged resources.
    ///
    /// "FINE_GRAINED_ACCESS_CONTROL"
    #[serde(rename="FINE_GRAINED_ACCESS_CONTROL")]
    FINEGRAINEDACCESSCONTROL,
}

impl AsRef<str> for GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum::POLICYTYPEUNSPECIFIED => "POLICY_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum::FINEGRAINEDACCESSCONTROL => "FINE_GRAINED_ACCESS_CONTROL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POLICY_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum::POLICYTYPEUNSPECIFIED),
           "FINE_GRAINED_ACCESS_CONTROL" => Ok(GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum::FINEGRAINEDACCESSCONTROL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1beta1TagTemplateDataplexTransferStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Transfer status of the TagTemplate
pub enum GoogleCloudDatacatalogV1beta1TagTemplateDataplexTransferStatusEnum {
    

    /// Default value. TagTemplate and its tags are only visible and editable in DataCatalog.
    ///
    /// "DATAPLEX_TRANSFER_STATUS_UNSPECIFIED"
    #[serde(rename="DATAPLEX_TRANSFER_STATUS_UNSPECIFIED")]
    DATAPLEXTRANSFERSTATUSUNSPECIFIED,
    

    /// TagTemplate and its tags are auto-copied to Dataplex service. Visible in both services. Editable in DataCatalog, read-only in Dataplex.
    ///
    /// "MIGRATED"
    #[serde(rename="MIGRATED")]
    MIGRATED,
}

impl AsRef<str> for GoogleCloudDatacatalogV1beta1TagTemplateDataplexTransferStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1beta1TagTemplateDataplexTransferStatusEnum::DATAPLEXTRANSFERSTATUSUNSPECIFIED => "DATAPLEX_TRANSFER_STATUS_UNSPECIFIED",
            GoogleCloudDatacatalogV1beta1TagTemplateDataplexTransferStatusEnum::MIGRATED => "MIGRATED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1beta1TagTemplateDataplexTransferStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATAPLEX_TRANSFER_STATUS_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1beta1TagTemplateDataplexTransferStatusEnum::DATAPLEXTRANSFERSTATUSUNSPECIFIED),
           "MIGRATED" => Ok(GoogleCloudDatacatalogV1beta1TagTemplateDataplexTransferStatusEnum::MIGRATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1beta1TagTemplateDataplexTransferStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A list of policy types that are activated for this taxonomy. If not set, defaults to an empty list.
pub enum GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum {
    

    /// Unspecified policy type.
    ///
    /// "POLICY_TYPE_UNSPECIFIED"
    #[serde(rename="POLICY_TYPE_UNSPECIFIED")]
    POLICYTYPEUNSPECIFIED,
    

    /// Fine grained access control policy, which enables access control on tagged resources.
    ///
    /// "FINE_GRAINED_ACCESS_CONTROL"
    #[serde(rename="FINE_GRAINED_ACCESS_CONTROL")]
    FINEGRAINEDACCESSCONTROL,
}

impl AsRef<str> for GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum::POLICYTYPEUNSPECIFIED => "POLICY_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum::FINEGRAINEDACCESSCONTROL => "FINE_GRAINED_ACCESS_CONTROL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POLICY_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum::POLICYTYPEUNSPECIFIED),
           "FINE_GRAINED_ACCESS_CONTROL" => Ok(GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum::FINEGRAINEDACCESSCONTROL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Google Cloud service name.
pub enum GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum {
    

    /// Default value
    ///
    /// "MANAGING_SYSTEM_UNSPECIFIED"
    #[serde(rename="MANAGING_SYSTEM_UNSPECIFIED")]
    MANAGINGSYSTEMUNSPECIFIED,
    

    /// Dataplex.
    ///
    /// "MANAGING_SYSTEM_DATAPLEX"
    #[serde(rename="MANAGING_SYSTEM_DATAPLEX")]
    MANAGINGSYSTEMDATAPLEX,
    

    /// Other
    ///
    /// "MANAGING_SYSTEM_OTHER"
    #[serde(rename="MANAGING_SYSTEM_OTHER")]
    MANAGINGSYSTEMOTHER,
}

impl AsRef<str> for GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum::MANAGINGSYSTEMUNSPECIFIED => "MANAGING_SYSTEM_UNSPECIFIED",
            GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum::MANAGINGSYSTEMDATAPLEX => "MANAGING_SYSTEM_DATAPLEX",
            GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum::MANAGINGSYSTEMOTHER => "MANAGING_SYSTEM_OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGING_SYSTEM_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum::MANAGINGSYSTEMUNSPECIFIED),
           "MANAGING_SYSTEM_DATAPLEX" => Ok(GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum::MANAGINGSYSTEMDATAPLEX),
           "MANAGING_SYSTEM_OTHER" => Ok(GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum::MANAGINGSYSTEMOTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


