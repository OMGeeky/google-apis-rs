use super::*;



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


// region BulkInsertOperationStatusStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Output Only] Creation status of BulkInsert operation - information if the flow is rolling forward or rolling back.
pub enum BulkInsertOperationStatusStatusEnum {
    
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// Rolling forward - creating VMs.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Rolling back - cleaning up after an error.
    ///
    /// "ROLLING_BACK"
    #[serde(rename="ROLLING_BACK")]
    ROLLINGBACK,
    

    /// Done
    ///
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
}

impl AsRef<str> for BulkInsertOperationStatusStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BulkInsertOperationStatusStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            BulkInsertOperationStatusStatusEnum::CREATING => "CREATING",
            BulkInsertOperationStatusStatusEnum::ROLLINGBACK => "ROLLING_BACK",
            BulkInsertOperationStatusStatusEnum::DONE => "DONE",
        }
    }
}

impl std::convert::TryFrom< &str> for BulkInsertOperationStatusStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(BulkInsertOperationStatusStatusEnum::STATUSUNSPECIFIED),
           "CREATING" => Ok(BulkInsertOperationStatusStatusEnum::CREATING),
           "ROLLING_BACK" => Ok(BulkInsertOperationStatusStatusEnum::ROLLINGBACK),
           "DONE" => Ok(BulkInsertOperationStatusStatusEnum::DONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BulkInsertOperationStatusStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OperationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`.
pub enum OperationStatusEnum {
    
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
}

impl AsRef<str> for OperationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationStatusEnum::PENDING => "PENDING",
            OperationStatusEnum::RUNNING => "RUNNING",
            OperationStatusEnum::DONE => "DONE",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PENDING" => Ok(OperationStatusEnum::PENDING),
           "RUNNING" => Ok(OperationStatusEnum::RUNNING),
           "DONE" => Ok(OperationStatusEnum::DONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OperationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResourceUpdateIntentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The intent of the resource: `PREVIEW`, `UPDATE`, or `CANCEL`.
pub enum ResourceUpdateIntentEnum {
    

    /// The resource is scheduled to be created, or if it already exists, acquired.
    ///
    /// "CREATE_OR_ACQUIRE"
    #[serde(rename="CREATE_OR_ACQUIRE")]
    CREATEORACQUIRE,
    

    /// The resource is scheduled to be deleted.
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
    

    /// The resource is scheduled to be acquired.
    ///
    /// "ACQUIRE"
    #[serde(rename="ACQUIRE")]
    ACQUIRE,
    

    /// The resource is scheduled to be updated via the UPDATE method.
    ///
    /// "UPDATE"
    #[serde(rename="UPDATE")]
    UPDATE,
    

    /// The resource is scheduled to be abandoned.
    ///
    /// "ABANDON"
    #[serde(rename="ABANDON")]
    ABANDON,
    

    /// The resource is scheduled to be created.
    ///
    /// "CREATE"
    #[serde(rename="CREATE")]
    CREATE,
}

impl AsRef<str> for ResourceUpdateIntentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResourceUpdateIntentEnum::CREATEORACQUIRE => "CREATE_OR_ACQUIRE",
            ResourceUpdateIntentEnum::DELETE => "DELETE",
            ResourceUpdateIntentEnum::ACQUIRE => "ACQUIRE",
            ResourceUpdateIntentEnum::UPDATE => "UPDATE",
            ResourceUpdateIntentEnum::ABANDON => "ABANDON",
            ResourceUpdateIntentEnum::CREATE => "CREATE",
        }
    }
}

impl std::convert::TryFrom< &str> for ResourceUpdateIntentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATE_OR_ACQUIRE" => Ok(ResourceUpdateIntentEnum::CREATEORACQUIRE),
           "DELETE" => Ok(ResourceUpdateIntentEnum::DELETE),
           "ACQUIRE" => Ok(ResourceUpdateIntentEnum::ACQUIRE),
           "UPDATE" => Ok(ResourceUpdateIntentEnum::UPDATE),
           "ABANDON" => Ok(ResourceUpdateIntentEnum::ABANDON),
           "CREATE" => Ok(ResourceUpdateIntentEnum::CREATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResourceUpdateIntentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResourceUpdateStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the resource.
pub enum ResourceUpdateStateEnum {
    

    /// There are changes pending for this resource.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The service is executing changes on the resource.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// The service is previewing changes on the resource.
    ///
    /// "IN_PREVIEW"
    #[serde(rename="IN_PREVIEW")]
    INPREVIEW,
    

    /// The service has failed to change the resource.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The service has aborted trying to change the resource.
    ///
    /// "ABORTED"
    #[serde(rename="ABORTED")]
    ABORTED,
}

impl AsRef<str> for ResourceUpdateStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResourceUpdateStateEnum::PENDING => "PENDING",
            ResourceUpdateStateEnum::INPROGRESS => "IN_PROGRESS",
            ResourceUpdateStateEnum::INPREVIEW => "IN_PREVIEW",
            ResourceUpdateStateEnum::FAILED => "FAILED",
            ResourceUpdateStateEnum::ABORTED => "ABORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for ResourceUpdateStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PENDING" => Ok(ResourceUpdateStateEnum::PENDING),
           "IN_PROGRESS" => Ok(ResourceUpdateStateEnum::INPROGRESS),
           "IN_PREVIEW" => Ok(ResourceUpdateStateEnum::INPREVIEW),
           "FAILED" => Ok(ResourceUpdateStateEnum::FAILED),
           "ABORTED" => Ok(ResourceUpdateStateEnum::ABORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResourceUpdateStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Output Only] Status of the action, which can be one of the following: `PROPAGATING`, `PROPAGATED`, `ABANDONED`, `FAILED`, or `DONE`.
pub enum SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum {
    
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Operation is not yet confirmed to have been created in the location.
    ///
    /// "PROPAGATING"
    #[serde(rename="PROPAGATING")]
    PROPAGATING,
    

    /// Operation is confirmed to be in the location.
    ///
    /// "PROPAGATED"
    #[serde(rename="PROPAGATED")]
    PROPAGATED,
    

    /// Operation not tracked in this location e.g. zone is marked as DOWN.
    ///
    /// "ABANDONED"
    #[serde(rename="ABANDONED")]
    ABANDONED,
    

    /// Operation is in an error state.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Operation has completed successfully.
    ///
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
}

impl AsRef<str> for SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::UNSPECIFIED => "UNSPECIFIED",
            SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::PROPAGATING => "PROPAGATING",
            SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::PROPAGATED => "PROPAGATED",
            SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::ABANDONED => "ABANDONED",
            SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::FAILED => "FAILED",
            SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::DONE => "DONE",
        }
    }
}

impl std::convert::TryFrom< &str> for SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::UNSPECIFIED),
           "PROPAGATING" => Ok(SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::PROPAGATING),
           "PROPAGATED" => Ok(SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::PROPAGATED),
           "ABANDONED" => Ok(SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::ABANDONED),
           "FAILED" => Ok(SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::FAILED),
           "DONE" => Ok(SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum::DONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfoStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OperationWarningCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
pub enum OperationWarningCodeEnum {
    

    /// A link to a deprecated resource was created.
    ///
    /// "DEPRECATED_RESOURCE_USED"
    #[serde(rename="DEPRECATED_RESOURCE_USED")]
    DEPRECATEDRESOURCEUSED,
    

    /// No results are present on a particular list page.
    ///
    /// "NO_RESULTS_ON_PAGE"
    #[serde(rename="NO_RESULTS_ON_PAGE")]
    NORESULTSONPAGE,
    

    /// A given scope cannot be reached.
    ///
    /// "UNREACHABLE"
    #[serde(rename="UNREACHABLE")]
    UNREACHABLE,
    

    /// The route's nextHopIp address is not assigned to an instance on the network.
    ///
    /// "NEXT_HOP_ADDRESS_NOT_ASSIGNED"
    #[serde(rename="NEXT_HOP_ADDRESS_NOT_ASSIGNED")]
    NEXTHOPADDRESSNOTASSIGNED,
    

    /// The route's nextHopInstance URL refers to an instance that does not exist.
    ///
    /// "NEXT_HOP_INSTANCE_NOT_FOUND"
    #[serde(rename="NEXT_HOP_INSTANCE_NOT_FOUND")]
    NEXTHOPINSTANCENOTFOUND,
    

    /// The route's nextHopInstance URL refers to an instance that is not on the same network as the route.
    ///
    /// "NEXT_HOP_INSTANCE_NOT_ON_NETWORK"
    #[serde(rename="NEXT_HOP_INSTANCE_NOT_ON_NETWORK")]
    NEXTHOPINSTANCENOTONNETWORK,
    

    /// The route's next hop instance cannot ip forward.
    ///
    /// "NEXT_HOP_CANNOT_IP_FORWARD"
    #[serde(rename="NEXT_HOP_CANNOT_IP_FORWARD")]
    NEXTHOPCANNOTIPFORWARD,
    

    /// The route's next hop instance does not have a status of RUNNING.
    ///
    /// "NEXT_HOP_NOT_RUNNING"
    #[serde(rename="NEXT_HOP_NOT_RUNNING")]
    NEXTHOPNOTRUNNING,
    

    /// The operation involved use of an injected kernel, which is deprecated.
    ///
    /// "INJECTED_KERNELS_DEPRECATED"
    #[serde(rename="INJECTED_KERNELS_DEPRECATED")]
    INJECTEDKERNELSDEPRECATED,
    

    /// The user attempted to use a resource that requires a TOS they have not accepted.
    ///
    /// "REQUIRED_TOS_AGREEMENT"
    #[serde(rename="REQUIRED_TOS_AGREEMENT")]
    REQUIREDTOSAGREEMENT,
    

    /// The user created a boot disk that is larger than image size.
    ///
    /// "DISK_SIZE_LARGER_THAN_IMAGE_SIZE"
    #[serde(rename="DISK_SIZE_LARGER_THAN_IMAGE_SIZE")]
    DISKSIZELARGERTHANIMAGESIZE,
    

    /// One or more of the resources set to auto-delete could not be deleted because they were in use.
    ///
    /// "RESOURCE_NOT_DELETED"
    #[serde(rename="RESOURCE_NOT_DELETED")]
    RESOURCENOTDELETED,
    

    /// Instance template used in instance group manager is valid as such, but its application does not make a lot of sense, because it allows only single instance in instance group.
    ///
    /// "SINGLE_INSTANCE_PROPERTY_TEMPLATE"
    #[serde(rename="SINGLE_INSTANCE_PROPERTY_TEMPLATE")]
    SINGLEINSTANCEPROPERTYTEMPLATE,
    

    /// Error which is not critical. We decided to continue the process despite the mentioned error.
    ///
    /// "NOT_CRITICAL_ERROR"
    #[serde(rename="NOT_CRITICAL_ERROR")]
    NOTCRITICALERROR,
    

    /// Warning about failed cleanup of transient changes made by a failed operation.
    ///
    /// "CLEANUP_FAILED"
    #[serde(rename="CLEANUP_FAILED")]
    CLEANUPFAILED,
    

    /// Warning that value of a field has been overridden. Deprecated unused field.
    ///
    /// "FIELD_VALUE_OVERRIDEN"
    #[serde(rename="FIELD_VALUE_OVERRIDEN")]
    FIELDVALUEOVERRIDEN,
    

    /// Warning that a resource is in use.
    ///
    /// "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING"
    #[serde(rename="RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING")]
    RESOURCEINUSEBYOTHERRESOURCEWARNING,
    

    /// A resource depends on a missing type
    ///
    /// "MISSING_TYPE_DEPENDENCY"
    #[serde(rename="MISSING_TYPE_DEPENDENCY")]
    MISSINGTYPEDEPENDENCY,
    

    /// Warning that is present in an external api call
    ///
    /// "EXTERNAL_API_WARNING"
    #[serde(rename="EXTERNAL_API_WARNING")]
    EXTERNALAPIWARNING,
    

    /// When a resource schema validation is ignored.
    ///
    /// "SCHEMA_VALIDATION_IGNORED"
    #[serde(rename="SCHEMA_VALIDATION_IGNORED")]
    SCHEMAVALIDATIONIGNORED,
    

    /// When undeclared properties in the schema are present
    ///
    /// "UNDECLARED_PROPERTIES"
    #[serde(rename="UNDECLARED_PROPERTIES")]
    UNDECLAREDPROPERTIES,
    

    /// When deploying and at least one of the resources has a type marked as experimental
    ///
    /// "EXPERIMENTAL_TYPE_USED"
    #[serde(rename="EXPERIMENTAL_TYPE_USED")]
    EXPERIMENTALTYPEUSED,
    

    /// When deploying and at least one of the resources has a type marked as deprecated
    ///
    /// "DEPRECATED_TYPE_USED"
    #[serde(rename="DEPRECATED_TYPE_USED")]
    DEPRECATEDTYPEUSED,
    

    /// Success is reported, but some results may be missing due to errors
    ///
    /// "PARTIAL_SUCCESS"
    #[serde(rename="PARTIAL_SUCCESS")]
    PARTIALSUCCESS,
    

    /// When deploying a deployment with a exceedingly large number of resources
    ///
    /// "LARGE_DEPLOYMENT_WARNING"
    #[serde(rename="LARGE_DEPLOYMENT_WARNING")]
    LARGEDEPLOYMENTWARNING,
    

    /// The route's nextHopInstance URL refers to an instance that does not have an ipv6 interface on the same network as the route.
    ///
    /// "NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE"
    #[serde(rename="NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE")]
    NEXTHOPINSTANCEHASNOIPV6INTERFACE,
    

    /// A WEIGHTED_MAGLEV backend service is associated with a health check that is not of type HTTP/HTTPS/HTTP2.
    ///
    /// "INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB"
    #[serde(rename="INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB")]
    INVALIDHEALTHCHECKFORDYNAMICWIEGHTEDLB,
    

    /// Resource can't be retrieved due to list overhead quota exceed which captures the amount of resources filtered out by user-defined list filter.
    ///
    /// "LIST_OVERHEAD_QUOTA_EXCEED"
    #[serde(rename="LIST_OVERHEAD_QUOTA_EXCEED")]
    LISTOVERHEADQUOTAEXCEED,
}

impl AsRef<str> for OperationWarningCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationWarningCodeEnum::DEPRECATEDRESOURCEUSED => "DEPRECATED_RESOURCE_USED",
            OperationWarningCodeEnum::NORESULTSONPAGE => "NO_RESULTS_ON_PAGE",
            OperationWarningCodeEnum::UNREACHABLE => "UNREACHABLE",
            OperationWarningCodeEnum::NEXTHOPADDRESSNOTASSIGNED => "NEXT_HOP_ADDRESS_NOT_ASSIGNED",
            OperationWarningCodeEnum::NEXTHOPINSTANCENOTFOUND => "NEXT_HOP_INSTANCE_NOT_FOUND",
            OperationWarningCodeEnum::NEXTHOPINSTANCENOTONNETWORK => "NEXT_HOP_INSTANCE_NOT_ON_NETWORK",
            OperationWarningCodeEnum::NEXTHOPCANNOTIPFORWARD => "NEXT_HOP_CANNOT_IP_FORWARD",
            OperationWarningCodeEnum::NEXTHOPNOTRUNNING => "NEXT_HOP_NOT_RUNNING",
            OperationWarningCodeEnum::INJECTEDKERNELSDEPRECATED => "INJECTED_KERNELS_DEPRECATED",
            OperationWarningCodeEnum::REQUIREDTOSAGREEMENT => "REQUIRED_TOS_AGREEMENT",
            OperationWarningCodeEnum::DISKSIZELARGERTHANIMAGESIZE => "DISK_SIZE_LARGER_THAN_IMAGE_SIZE",
            OperationWarningCodeEnum::RESOURCENOTDELETED => "RESOURCE_NOT_DELETED",
            OperationWarningCodeEnum::SINGLEINSTANCEPROPERTYTEMPLATE => "SINGLE_INSTANCE_PROPERTY_TEMPLATE",
            OperationWarningCodeEnum::NOTCRITICALERROR => "NOT_CRITICAL_ERROR",
            OperationWarningCodeEnum::CLEANUPFAILED => "CLEANUP_FAILED",
            OperationWarningCodeEnum::FIELDVALUEOVERRIDEN => "FIELD_VALUE_OVERRIDEN",
            OperationWarningCodeEnum::RESOURCEINUSEBYOTHERRESOURCEWARNING => "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING",
            OperationWarningCodeEnum::MISSINGTYPEDEPENDENCY => "MISSING_TYPE_DEPENDENCY",
            OperationWarningCodeEnum::EXTERNALAPIWARNING => "EXTERNAL_API_WARNING",
            OperationWarningCodeEnum::SCHEMAVALIDATIONIGNORED => "SCHEMA_VALIDATION_IGNORED",
            OperationWarningCodeEnum::UNDECLAREDPROPERTIES => "UNDECLARED_PROPERTIES",
            OperationWarningCodeEnum::EXPERIMENTALTYPEUSED => "EXPERIMENTAL_TYPE_USED",
            OperationWarningCodeEnum::DEPRECATEDTYPEUSED => "DEPRECATED_TYPE_USED",
            OperationWarningCodeEnum::PARTIALSUCCESS => "PARTIAL_SUCCESS",
            OperationWarningCodeEnum::LARGEDEPLOYMENTWARNING => "LARGE_DEPLOYMENT_WARNING",
            OperationWarningCodeEnum::NEXTHOPINSTANCEHASNOIPV6INTERFACE => "NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE",
            OperationWarningCodeEnum::INVALIDHEALTHCHECKFORDYNAMICWIEGHTEDLB => "INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB",
            OperationWarningCodeEnum::LISTOVERHEADQUOTAEXCEED => "LIST_OVERHEAD_QUOTA_EXCEED",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationWarningCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEPRECATED_RESOURCE_USED" => Ok(OperationWarningCodeEnum::DEPRECATEDRESOURCEUSED),
           "NO_RESULTS_ON_PAGE" => Ok(OperationWarningCodeEnum::NORESULTSONPAGE),
           "UNREACHABLE" => Ok(OperationWarningCodeEnum::UNREACHABLE),
           "NEXT_HOP_ADDRESS_NOT_ASSIGNED" => Ok(OperationWarningCodeEnum::NEXTHOPADDRESSNOTASSIGNED),
           "NEXT_HOP_INSTANCE_NOT_FOUND" => Ok(OperationWarningCodeEnum::NEXTHOPINSTANCENOTFOUND),
           "NEXT_HOP_INSTANCE_NOT_ON_NETWORK" => Ok(OperationWarningCodeEnum::NEXTHOPINSTANCENOTONNETWORK),
           "NEXT_HOP_CANNOT_IP_FORWARD" => Ok(OperationWarningCodeEnum::NEXTHOPCANNOTIPFORWARD),
           "NEXT_HOP_NOT_RUNNING" => Ok(OperationWarningCodeEnum::NEXTHOPNOTRUNNING),
           "INJECTED_KERNELS_DEPRECATED" => Ok(OperationWarningCodeEnum::INJECTEDKERNELSDEPRECATED),
           "REQUIRED_TOS_AGREEMENT" => Ok(OperationWarningCodeEnum::REQUIREDTOSAGREEMENT),
           "DISK_SIZE_LARGER_THAN_IMAGE_SIZE" => Ok(OperationWarningCodeEnum::DISKSIZELARGERTHANIMAGESIZE),
           "RESOURCE_NOT_DELETED" => Ok(OperationWarningCodeEnum::RESOURCENOTDELETED),
           "SINGLE_INSTANCE_PROPERTY_TEMPLATE" => Ok(OperationWarningCodeEnum::SINGLEINSTANCEPROPERTYTEMPLATE),
           "NOT_CRITICAL_ERROR" => Ok(OperationWarningCodeEnum::NOTCRITICALERROR),
           "CLEANUP_FAILED" => Ok(OperationWarningCodeEnum::CLEANUPFAILED),
           "FIELD_VALUE_OVERRIDEN" => Ok(OperationWarningCodeEnum::FIELDVALUEOVERRIDEN),
           "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING" => Ok(OperationWarningCodeEnum::RESOURCEINUSEBYOTHERRESOURCEWARNING),
           "MISSING_TYPE_DEPENDENCY" => Ok(OperationWarningCodeEnum::MISSINGTYPEDEPENDENCY),
           "EXTERNAL_API_WARNING" => Ok(OperationWarningCodeEnum::EXTERNALAPIWARNING),
           "SCHEMA_VALIDATION_IGNORED" => Ok(OperationWarningCodeEnum::SCHEMAVALIDATIONIGNORED),
           "UNDECLARED_PROPERTIES" => Ok(OperationWarningCodeEnum::UNDECLAREDPROPERTIES),
           "EXPERIMENTAL_TYPE_USED" => Ok(OperationWarningCodeEnum::EXPERIMENTALTYPEUSED),
           "DEPRECATED_TYPE_USED" => Ok(OperationWarningCodeEnum::DEPRECATEDTYPEUSED),
           "PARTIAL_SUCCESS" => Ok(OperationWarningCodeEnum::PARTIALSUCCESS),
           "LARGE_DEPLOYMENT_WARNING" => Ok(OperationWarningCodeEnum::LARGEDEPLOYMENTWARNING),
           "NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE" => Ok(OperationWarningCodeEnum::NEXTHOPINSTANCEHASNOIPV6INTERFACE),
           "INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB" => Ok(OperationWarningCodeEnum::INVALIDHEALTHCHECKFORDYNAMICWIEGHTEDLB),
           "LIST_OVERHEAD_QUOTA_EXCEED" => Ok(OperationWarningCodeEnum::LISTOVERHEADQUOTAEXCEED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OperationWarningCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResourceWarningCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
pub enum ResourceWarningCodeEnum {
    

    /// A link to a deprecated resource was created.
    ///
    /// "DEPRECATED_RESOURCE_USED"
    #[serde(rename="DEPRECATED_RESOURCE_USED")]
    DEPRECATEDRESOURCEUSED,
    

    /// No results are present on a particular list page.
    ///
    /// "NO_RESULTS_ON_PAGE"
    #[serde(rename="NO_RESULTS_ON_PAGE")]
    NORESULTSONPAGE,
    

    /// A given scope cannot be reached.
    ///
    /// "UNREACHABLE"
    #[serde(rename="UNREACHABLE")]
    UNREACHABLE,
    

    /// The route's nextHopIp address is not assigned to an instance on the network.
    ///
    /// "NEXT_HOP_ADDRESS_NOT_ASSIGNED"
    #[serde(rename="NEXT_HOP_ADDRESS_NOT_ASSIGNED")]
    NEXTHOPADDRESSNOTASSIGNED,
    

    /// The route's nextHopInstance URL refers to an instance that does not exist.
    ///
    /// "NEXT_HOP_INSTANCE_NOT_FOUND"
    #[serde(rename="NEXT_HOP_INSTANCE_NOT_FOUND")]
    NEXTHOPINSTANCENOTFOUND,
    

    /// The route's nextHopInstance URL refers to an instance that is not on the same network as the route.
    ///
    /// "NEXT_HOP_INSTANCE_NOT_ON_NETWORK"
    #[serde(rename="NEXT_HOP_INSTANCE_NOT_ON_NETWORK")]
    NEXTHOPINSTANCENOTONNETWORK,
    

    /// The route's next hop instance cannot ip forward.
    ///
    /// "NEXT_HOP_CANNOT_IP_FORWARD"
    #[serde(rename="NEXT_HOP_CANNOT_IP_FORWARD")]
    NEXTHOPCANNOTIPFORWARD,
    

    /// The route's next hop instance does not have a status of RUNNING.
    ///
    /// "NEXT_HOP_NOT_RUNNING"
    #[serde(rename="NEXT_HOP_NOT_RUNNING")]
    NEXTHOPNOTRUNNING,
    

    /// The operation involved use of an injected kernel, which is deprecated.
    ///
    /// "INJECTED_KERNELS_DEPRECATED"
    #[serde(rename="INJECTED_KERNELS_DEPRECATED")]
    INJECTEDKERNELSDEPRECATED,
    

    /// The user attempted to use a resource that requires a TOS they have not accepted.
    ///
    /// "REQUIRED_TOS_AGREEMENT"
    #[serde(rename="REQUIRED_TOS_AGREEMENT")]
    REQUIREDTOSAGREEMENT,
    

    /// The user created a boot disk that is larger than image size.
    ///
    /// "DISK_SIZE_LARGER_THAN_IMAGE_SIZE"
    #[serde(rename="DISK_SIZE_LARGER_THAN_IMAGE_SIZE")]
    DISKSIZELARGERTHANIMAGESIZE,
    

    /// One or more of the resources set to auto-delete could not be deleted because they were in use.
    ///
    /// "RESOURCE_NOT_DELETED"
    #[serde(rename="RESOURCE_NOT_DELETED")]
    RESOURCENOTDELETED,
    

    /// Instance template used in instance group manager is valid as such, but its application does not make a lot of sense, because it allows only single instance in instance group.
    ///
    /// "SINGLE_INSTANCE_PROPERTY_TEMPLATE"
    #[serde(rename="SINGLE_INSTANCE_PROPERTY_TEMPLATE")]
    SINGLEINSTANCEPROPERTYTEMPLATE,
    

    /// Error which is not critical. We decided to continue the process despite the mentioned error.
    ///
    /// "NOT_CRITICAL_ERROR"
    #[serde(rename="NOT_CRITICAL_ERROR")]
    NOTCRITICALERROR,
    

    /// Warning about failed cleanup of transient changes made by a failed operation.
    ///
    /// "CLEANUP_FAILED"
    #[serde(rename="CLEANUP_FAILED")]
    CLEANUPFAILED,
    

    /// Warning that value of a field has been overridden. Deprecated unused field.
    ///
    /// "FIELD_VALUE_OVERRIDEN"
    #[serde(rename="FIELD_VALUE_OVERRIDEN")]
    FIELDVALUEOVERRIDEN,
    

    /// Warning that a resource is in use.
    ///
    /// "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING"
    #[serde(rename="RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING")]
    RESOURCEINUSEBYOTHERRESOURCEWARNING,
    

    /// A resource depends on a missing type
    ///
    /// "MISSING_TYPE_DEPENDENCY"
    #[serde(rename="MISSING_TYPE_DEPENDENCY")]
    MISSINGTYPEDEPENDENCY,
    

    /// Warning that is present in an external api call
    ///
    /// "EXTERNAL_API_WARNING"
    #[serde(rename="EXTERNAL_API_WARNING")]
    EXTERNALAPIWARNING,
    

    /// When a resource schema validation is ignored.
    ///
    /// "SCHEMA_VALIDATION_IGNORED"
    #[serde(rename="SCHEMA_VALIDATION_IGNORED")]
    SCHEMAVALIDATIONIGNORED,
    

    /// When undeclared properties in the schema are present
    ///
    /// "UNDECLARED_PROPERTIES"
    #[serde(rename="UNDECLARED_PROPERTIES")]
    UNDECLAREDPROPERTIES,
    

    /// When deploying and at least one of the resources has a type marked as experimental
    ///
    /// "EXPERIMENTAL_TYPE_USED"
    #[serde(rename="EXPERIMENTAL_TYPE_USED")]
    EXPERIMENTALTYPEUSED,
    

    /// When deploying and at least one of the resources has a type marked as deprecated
    ///
    /// "DEPRECATED_TYPE_USED"
    #[serde(rename="DEPRECATED_TYPE_USED")]
    DEPRECATEDTYPEUSED,
    

    /// Success is reported, but some results may be missing due to errors
    ///
    /// "PARTIAL_SUCCESS"
    #[serde(rename="PARTIAL_SUCCESS")]
    PARTIALSUCCESS,
    

    /// When deploying a deployment with a exceedingly large number of resources
    ///
    /// "LARGE_DEPLOYMENT_WARNING"
    #[serde(rename="LARGE_DEPLOYMENT_WARNING")]
    LARGEDEPLOYMENTWARNING,
    

    /// The route's nextHopInstance URL refers to an instance that does not have an ipv6 interface on the same network as the route.
    ///
    /// "NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE"
    #[serde(rename="NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE")]
    NEXTHOPINSTANCEHASNOIPV6INTERFACE,
    

    /// A WEIGHTED_MAGLEV backend service is associated with a health check that is not of type HTTP/HTTPS/HTTP2.
    ///
    /// "INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB"
    #[serde(rename="INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB")]
    INVALIDHEALTHCHECKFORDYNAMICWIEGHTEDLB,
    

    /// Resource can't be retrieved due to list overhead quota exceed which captures the amount of resources filtered out by user-defined list filter.
    ///
    /// "LIST_OVERHEAD_QUOTA_EXCEED"
    #[serde(rename="LIST_OVERHEAD_QUOTA_EXCEED")]
    LISTOVERHEADQUOTAEXCEED,
}

impl AsRef<str> for ResourceWarningCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResourceWarningCodeEnum::DEPRECATEDRESOURCEUSED => "DEPRECATED_RESOURCE_USED",
            ResourceWarningCodeEnum::NORESULTSONPAGE => "NO_RESULTS_ON_PAGE",
            ResourceWarningCodeEnum::UNREACHABLE => "UNREACHABLE",
            ResourceWarningCodeEnum::NEXTHOPADDRESSNOTASSIGNED => "NEXT_HOP_ADDRESS_NOT_ASSIGNED",
            ResourceWarningCodeEnum::NEXTHOPINSTANCENOTFOUND => "NEXT_HOP_INSTANCE_NOT_FOUND",
            ResourceWarningCodeEnum::NEXTHOPINSTANCENOTONNETWORK => "NEXT_HOP_INSTANCE_NOT_ON_NETWORK",
            ResourceWarningCodeEnum::NEXTHOPCANNOTIPFORWARD => "NEXT_HOP_CANNOT_IP_FORWARD",
            ResourceWarningCodeEnum::NEXTHOPNOTRUNNING => "NEXT_HOP_NOT_RUNNING",
            ResourceWarningCodeEnum::INJECTEDKERNELSDEPRECATED => "INJECTED_KERNELS_DEPRECATED",
            ResourceWarningCodeEnum::REQUIREDTOSAGREEMENT => "REQUIRED_TOS_AGREEMENT",
            ResourceWarningCodeEnum::DISKSIZELARGERTHANIMAGESIZE => "DISK_SIZE_LARGER_THAN_IMAGE_SIZE",
            ResourceWarningCodeEnum::RESOURCENOTDELETED => "RESOURCE_NOT_DELETED",
            ResourceWarningCodeEnum::SINGLEINSTANCEPROPERTYTEMPLATE => "SINGLE_INSTANCE_PROPERTY_TEMPLATE",
            ResourceWarningCodeEnum::NOTCRITICALERROR => "NOT_CRITICAL_ERROR",
            ResourceWarningCodeEnum::CLEANUPFAILED => "CLEANUP_FAILED",
            ResourceWarningCodeEnum::FIELDVALUEOVERRIDEN => "FIELD_VALUE_OVERRIDEN",
            ResourceWarningCodeEnum::RESOURCEINUSEBYOTHERRESOURCEWARNING => "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING",
            ResourceWarningCodeEnum::MISSINGTYPEDEPENDENCY => "MISSING_TYPE_DEPENDENCY",
            ResourceWarningCodeEnum::EXTERNALAPIWARNING => "EXTERNAL_API_WARNING",
            ResourceWarningCodeEnum::SCHEMAVALIDATIONIGNORED => "SCHEMA_VALIDATION_IGNORED",
            ResourceWarningCodeEnum::UNDECLAREDPROPERTIES => "UNDECLARED_PROPERTIES",
            ResourceWarningCodeEnum::EXPERIMENTALTYPEUSED => "EXPERIMENTAL_TYPE_USED",
            ResourceWarningCodeEnum::DEPRECATEDTYPEUSED => "DEPRECATED_TYPE_USED",
            ResourceWarningCodeEnum::PARTIALSUCCESS => "PARTIAL_SUCCESS",
            ResourceWarningCodeEnum::LARGEDEPLOYMENTWARNING => "LARGE_DEPLOYMENT_WARNING",
            ResourceWarningCodeEnum::NEXTHOPINSTANCEHASNOIPV6INTERFACE => "NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE",
            ResourceWarningCodeEnum::INVALIDHEALTHCHECKFORDYNAMICWIEGHTEDLB => "INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB",
            ResourceWarningCodeEnum::LISTOVERHEADQUOTAEXCEED => "LIST_OVERHEAD_QUOTA_EXCEED",
        }
    }
}

impl std::convert::TryFrom< &str> for ResourceWarningCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEPRECATED_RESOURCE_USED" => Ok(ResourceWarningCodeEnum::DEPRECATEDRESOURCEUSED),
           "NO_RESULTS_ON_PAGE" => Ok(ResourceWarningCodeEnum::NORESULTSONPAGE),
           "UNREACHABLE" => Ok(ResourceWarningCodeEnum::UNREACHABLE),
           "NEXT_HOP_ADDRESS_NOT_ASSIGNED" => Ok(ResourceWarningCodeEnum::NEXTHOPADDRESSNOTASSIGNED),
           "NEXT_HOP_INSTANCE_NOT_FOUND" => Ok(ResourceWarningCodeEnum::NEXTHOPINSTANCENOTFOUND),
           "NEXT_HOP_INSTANCE_NOT_ON_NETWORK" => Ok(ResourceWarningCodeEnum::NEXTHOPINSTANCENOTONNETWORK),
           "NEXT_HOP_CANNOT_IP_FORWARD" => Ok(ResourceWarningCodeEnum::NEXTHOPCANNOTIPFORWARD),
           "NEXT_HOP_NOT_RUNNING" => Ok(ResourceWarningCodeEnum::NEXTHOPNOTRUNNING),
           "INJECTED_KERNELS_DEPRECATED" => Ok(ResourceWarningCodeEnum::INJECTEDKERNELSDEPRECATED),
           "REQUIRED_TOS_AGREEMENT" => Ok(ResourceWarningCodeEnum::REQUIREDTOSAGREEMENT),
           "DISK_SIZE_LARGER_THAN_IMAGE_SIZE" => Ok(ResourceWarningCodeEnum::DISKSIZELARGERTHANIMAGESIZE),
           "RESOURCE_NOT_DELETED" => Ok(ResourceWarningCodeEnum::RESOURCENOTDELETED),
           "SINGLE_INSTANCE_PROPERTY_TEMPLATE" => Ok(ResourceWarningCodeEnum::SINGLEINSTANCEPROPERTYTEMPLATE),
           "NOT_CRITICAL_ERROR" => Ok(ResourceWarningCodeEnum::NOTCRITICALERROR),
           "CLEANUP_FAILED" => Ok(ResourceWarningCodeEnum::CLEANUPFAILED),
           "FIELD_VALUE_OVERRIDEN" => Ok(ResourceWarningCodeEnum::FIELDVALUEOVERRIDEN),
           "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING" => Ok(ResourceWarningCodeEnum::RESOURCEINUSEBYOTHERRESOURCEWARNING),
           "MISSING_TYPE_DEPENDENCY" => Ok(ResourceWarningCodeEnum::MISSINGTYPEDEPENDENCY),
           "EXTERNAL_API_WARNING" => Ok(ResourceWarningCodeEnum::EXTERNALAPIWARNING),
           "SCHEMA_VALIDATION_IGNORED" => Ok(ResourceWarningCodeEnum::SCHEMAVALIDATIONIGNORED),
           "UNDECLARED_PROPERTIES" => Ok(ResourceWarningCodeEnum::UNDECLAREDPROPERTIES),
           "EXPERIMENTAL_TYPE_USED" => Ok(ResourceWarningCodeEnum::EXPERIMENTALTYPEUSED),
           "DEPRECATED_TYPE_USED" => Ok(ResourceWarningCodeEnum::DEPRECATEDTYPEUSED),
           "PARTIAL_SUCCESS" => Ok(ResourceWarningCodeEnum::PARTIALSUCCESS),
           "LARGE_DEPLOYMENT_WARNING" => Ok(ResourceWarningCodeEnum::LARGEDEPLOYMENTWARNING),
           "NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE" => Ok(ResourceWarningCodeEnum::NEXTHOPINSTANCEHASNOIPV6INTERFACE),
           "INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB" => Ok(ResourceWarningCodeEnum::INVALIDHEALTHCHECKFORDYNAMICWIEGHTEDLB),
           "LIST_OVERHEAD_QUOTA_EXCEED" => Ok(ResourceWarningCodeEnum::LISTOVERHEADQUOTAEXCEED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResourceWarningCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResourceUpdateWarningCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
pub enum ResourceUpdateWarningCodeEnum {
    

    /// A link to a deprecated resource was created.
    ///
    /// "DEPRECATED_RESOURCE_USED"
    #[serde(rename="DEPRECATED_RESOURCE_USED")]
    DEPRECATEDRESOURCEUSED,
    

    /// No results are present on a particular list page.
    ///
    /// "NO_RESULTS_ON_PAGE"
    #[serde(rename="NO_RESULTS_ON_PAGE")]
    NORESULTSONPAGE,
    

    /// A given scope cannot be reached.
    ///
    /// "UNREACHABLE"
    #[serde(rename="UNREACHABLE")]
    UNREACHABLE,
    

    /// The route's nextHopIp address is not assigned to an instance on the network.
    ///
    /// "NEXT_HOP_ADDRESS_NOT_ASSIGNED"
    #[serde(rename="NEXT_HOP_ADDRESS_NOT_ASSIGNED")]
    NEXTHOPADDRESSNOTASSIGNED,
    

    /// The route's nextHopInstance URL refers to an instance that does not exist.
    ///
    /// "NEXT_HOP_INSTANCE_NOT_FOUND"
    #[serde(rename="NEXT_HOP_INSTANCE_NOT_FOUND")]
    NEXTHOPINSTANCENOTFOUND,
    

    /// The route's nextHopInstance URL refers to an instance that is not on the same network as the route.
    ///
    /// "NEXT_HOP_INSTANCE_NOT_ON_NETWORK"
    #[serde(rename="NEXT_HOP_INSTANCE_NOT_ON_NETWORK")]
    NEXTHOPINSTANCENOTONNETWORK,
    

    /// The route's next hop instance cannot ip forward.
    ///
    /// "NEXT_HOP_CANNOT_IP_FORWARD"
    #[serde(rename="NEXT_HOP_CANNOT_IP_FORWARD")]
    NEXTHOPCANNOTIPFORWARD,
    

    /// The route's next hop instance does not have a status of RUNNING.
    ///
    /// "NEXT_HOP_NOT_RUNNING"
    #[serde(rename="NEXT_HOP_NOT_RUNNING")]
    NEXTHOPNOTRUNNING,
    

    /// The operation involved use of an injected kernel, which is deprecated.
    ///
    /// "INJECTED_KERNELS_DEPRECATED"
    #[serde(rename="INJECTED_KERNELS_DEPRECATED")]
    INJECTEDKERNELSDEPRECATED,
    

    /// The user attempted to use a resource that requires a TOS they have not accepted.
    ///
    /// "REQUIRED_TOS_AGREEMENT"
    #[serde(rename="REQUIRED_TOS_AGREEMENT")]
    REQUIREDTOSAGREEMENT,
    

    /// The user created a boot disk that is larger than image size.
    ///
    /// "DISK_SIZE_LARGER_THAN_IMAGE_SIZE"
    #[serde(rename="DISK_SIZE_LARGER_THAN_IMAGE_SIZE")]
    DISKSIZELARGERTHANIMAGESIZE,
    

    /// One or more of the resources set to auto-delete could not be deleted because they were in use.
    ///
    /// "RESOURCE_NOT_DELETED"
    #[serde(rename="RESOURCE_NOT_DELETED")]
    RESOURCENOTDELETED,
    

    /// Instance template used in instance group manager is valid as such, but its application does not make a lot of sense, because it allows only single instance in instance group.
    ///
    /// "SINGLE_INSTANCE_PROPERTY_TEMPLATE"
    #[serde(rename="SINGLE_INSTANCE_PROPERTY_TEMPLATE")]
    SINGLEINSTANCEPROPERTYTEMPLATE,
    

    /// Error which is not critical. We decided to continue the process despite the mentioned error.
    ///
    /// "NOT_CRITICAL_ERROR"
    #[serde(rename="NOT_CRITICAL_ERROR")]
    NOTCRITICALERROR,
    

    /// Warning about failed cleanup of transient changes made by a failed operation.
    ///
    /// "CLEANUP_FAILED"
    #[serde(rename="CLEANUP_FAILED")]
    CLEANUPFAILED,
    

    /// Warning that value of a field has been overridden. Deprecated unused field.
    ///
    /// "FIELD_VALUE_OVERRIDEN"
    #[serde(rename="FIELD_VALUE_OVERRIDEN")]
    FIELDVALUEOVERRIDEN,
    

    /// Warning that a resource is in use.
    ///
    /// "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING"
    #[serde(rename="RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING")]
    RESOURCEINUSEBYOTHERRESOURCEWARNING,
    

    /// A resource depends on a missing type
    ///
    /// "MISSING_TYPE_DEPENDENCY"
    #[serde(rename="MISSING_TYPE_DEPENDENCY")]
    MISSINGTYPEDEPENDENCY,
    

    /// Warning that is present in an external api call
    ///
    /// "EXTERNAL_API_WARNING"
    #[serde(rename="EXTERNAL_API_WARNING")]
    EXTERNALAPIWARNING,
    

    /// When a resource schema validation is ignored.
    ///
    /// "SCHEMA_VALIDATION_IGNORED"
    #[serde(rename="SCHEMA_VALIDATION_IGNORED")]
    SCHEMAVALIDATIONIGNORED,
    

    /// When undeclared properties in the schema are present
    ///
    /// "UNDECLARED_PROPERTIES"
    #[serde(rename="UNDECLARED_PROPERTIES")]
    UNDECLAREDPROPERTIES,
    

    /// When deploying and at least one of the resources has a type marked as experimental
    ///
    /// "EXPERIMENTAL_TYPE_USED"
    #[serde(rename="EXPERIMENTAL_TYPE_USED")]
    EXPERIMENTALTYPEUSED,
    

    /// When deploying and at least one of the resources has a type marked as deprecated
    ///
    /// "DEPRECATED_TYPE_USED"
    #[serde(rename="DEPRECATED_TYPE_USED")]
    DEPRECATEDTYPEUSED,
    

    /// Success is reported, but some results may be missing due to errors
    ///
    /// "PARTIAL_SUCCESS"
    #[serde(rename="PARTIAL_SUCCESS")]
    PARTIALSUCCESS,
    

    /// When deploying a deployment with a exceedingly large number of resources
    ///
    /// "LARGE_DEPLOYMENT_WARNING"
    #[serde(rename="LARGE_DEPLOYMENT_WARNING")]
    LARGEDEPLOYMENTWARNING,
    

    /// The route's nextHopInstance URL refers to an instance that does not have an ipv6 interface on the same network as the route.
    ///
    /// "NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE"
    #[serde(rename="NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE")]
    NEXTHOPINSTANCEHASNOIPV6INTERFACE,
    

    /// A WEIGHTED_MAGLEV backend service is associated with a health check that is not of type HTTP/HTTPS/HTTP2.
    ///
    /// "INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB"
    #[serde(rename="INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB")]
    INVALIDHEALTHCHECKFORDYNAMICWIEGHTEDLB,
    

    /// Resource can't be retrieved due to list overhead quota exceed which captures the amount of resources filtered out by user-defined list filter.
    ///
    /// "LIST_OVERHEAD_QUOTA_EXCEED"
    #[serde(rename="LIST_OVERHEAD_QUOTA_EXCEED")]
    LISTOVERHEADQUOTAEXCEED,
}

impl AsRef<str> for ResourceUpdateWarningCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResourceUpdateWarningCodeEnum::DEPRECATEDRESOURCEUSED => "DEPRECATED_RESOURCE_USED",
            ResourceUpdateWarningCodeEnum::NORESULTSONPAGE => "NO_RESULTS_ON_PAGE",
            ResourceUpdateWarningCodeEnum::UNREACHABLE => "UNREACHABLE",
            ResourceUpdateWarningCodeEnum::NEXTHOPADDRESSNOTASSIGNED => "NEXT_HOP_ADDRESS_NOT_ASSIGNED",
            ResourceUpdateWarningCodeEnum::NEXTHOPINSTANCENOTFOUND => "NEXT_HOP_INSTANCE_NOT_FOUND",
            ResourceUpdateWarningCodeEnum::NEXTHOPINSTANCENOTONNETWORK => "NEXT_HOP_INSTANCE_NOT_ON_NETWORK",
            ResourceUpdateWarningCodeEnum::NEXTHOPCANNOTIPFORWARD => "NEXT_HOP_CANNOT_IP_FORWARD",
            ResourceUpdateWarningCodeEnum::NEXTHOPNOTRUNNING => "NEXT_HOP_NOT_RUNNING",
            ResourceUpdateWarningCodeEnum::INJECTEDKERNELSDEPRECATED => "INJECTED_KERNELS_DEPRECATED",
            ResourceUpdateWarningCodeEnum::REQUIREDTOSAGREEMENT => "REQUIRED_TOS_AGREEMENT",
            ResourceUpdateWarningCodeEnum::DISKSIZELARGERTHANIMAGESIZE => "DISK_SIZE_LARGER_THAN_IMAGE_SIZE",
            ResourceUpdateWarningCodeEnum::RESOURCENOTDELETED => "RESOURCE_NOT_DELETED",
            ResourceUpdateWarningCodeEnum::SINGLEINSTANCEPROPERTYTEMPLATE => "SINGLE_INSTANCE_PROPERTY_TEMPLATE",
            ResourceUpdateWarningCodeEnum::NOTCRITICALERROR => "NOT_CRITICAL_ERROR",
            ResourceUpdateWarningCodeEnum::CLEANUPFAILED => "CLEANUP_FAILED",
            ResourceUpdateWarningCodeEnum::FIELDVALUEOVERRIDEN => "FIELD_VALUE_OVERRIDEN",
            ResourceUpdateWarningCodeEnum::RESOURCEINUSEBYOTHERRESOURCEWARNING => "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING",
            ResourceUpdateWarningCodeEnum::MISSINGTYPEDEPENDENCY => "MISSING_TYPE_DEPENDENCY",
            ResourceUpdateWarningCodeEnum::EXTERNALAPIWARNING => "EXTERNAL_API_WARNING",
            ResourceUpdateWarningCodeEnum::SCHEMAVALIDATIONIGNORED => "SCHEMA_VALIDATION_IGNORED",
            ResourceUpdateWarningCodeEnum::UNDECLAREDPROPERTIES => "UNDECLARED_PROPERTIES",
            ResourceUpdateWarningCodeEnum::EXPERIMENTALTYPEUSED => "EXPERIMENTAL_TYPE_USED",
            ResourceUpdateWarningCodeEnum::DEPRECATEDTYPEUSED => "DEPRECATED_TYPE_USED",
            ResourceUpdateWarningCodeEnum::PARTIALSUCCESS => "PARTIAL_SUCCESS",
            ResourceUpdateWarningCodeEnum::LARGEDEPLOYMENTWARNING => "LARGE_DEPLOYMENT_WARNING",
            ResourceUpdateWarningCodeEnum::NEXTHOPINSTANCEHASNOIPV6INTERFACE => "NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE",
            ResourceUpdateWarningCodeEnum::INVALIDHEALTHCHECKFORDYNAMICWIEGHTEDLB => "INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB",
            ResourceUpdateWarningCodeEnum::LISTOVERHEADQUOTAEXCEED => "LIST_OVERHEAD_QUOTA_EXCEED",
        }
    }
}

impl std::convert::TryFrom< &str> for ResourceUpdateWarningCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEPRECATED_RESOURCE_USED" => Ok(ResourceUpdateWarningCodeEnum::DEPRECATEDRESOURCEUSED),
           "NO_RESULTS_ON_PAGE" => Ok(ResourceUpdateWarningCodeEnum::NORESULTSONPAGE),
           "UNREACHABLE" => Ok(ResourceUpdateWarningCodeEnum::UNREACHABLE),
           "NEXT_HOP_ADDRESS_NOT_ASSIGNED" => Ok(ResourceUpdateWarningCodeEnum::NEXTHOPADDRESSNOTASSIGNED),
           "NEXT_HOP_INSTANCE_NOT_FOUND" => Ok(ResourceUpdateWarningCodeEnum::NEXTHOPINSTANCENOTFOUND),
           "NEXT_HOP_INSTANCE_NOT_ON_NETWORK" => Ok(ResourceUpdateWarningCodeEnum::NEXTHOPINSTANCENOTONNETWORK),
           "NEXT_HOP_CANNOT_IP_FORWARD" => Ok(ResourceUpdateWarningCodeEnum::NEXTHOPCANNOTIPFORWARD),
           "NEXT_HOP_NOT_RUNNING" => Ok(ResourceUpdateWarningCodeEnum::NEXTHOPNOTRUNNING),
           "INJECTED_KERNELS_DEPRECATED" => Ok(ResourceUpdateWarningCodeEnum::INJECTEDKERNELSDEPRECATED),
           "REQUIRED_TOS_AGREEMENT" => Ok(ResourceUpdateWarningCodeEnum::REQUIREDTOSAGREEMENT),
           "DISK_SIZE_LARGER_THAN_IMAGE_SIZE" => Ok(ResourceUpdateWarningCodeEnum::DISKSIZELARGERTHANIMAGESIZE),
           "RESOURCE_NOT_DELETED" => Ok(ResourceUpdateWarningCodeEnum::RESOURCENOTDELETED),
           "SINGLE_INSTANCE_PROPERTY_TEMPLATE" => Ok(ResourceUpdateWarningCodeEnum::SINGLEINSTANCEPROPERTYTEMPLATE),
           "NOT_CRITICAL_ERROR" => Ok(ResourceUpdateWarningCodeEnum::NOTCRITICALERROR),
           "CLEANUP_FAILED" => Ok(ResourceUpdateWarningCodeEnum::CLEANUPFAILED),
           "FIELD_VALUE_OVERRIDEN" => Ok(ResourceUpdateWarningCodeEnum::FIELDVALUEOVERRIDEN),
           "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING" => Ok(ResourceUpdateWarningCodeEnum::RESOURCEINUSEBYOTHERRESOURCEWARNING),
           "MISSING_TYPE_DEPENDENCY" => Ok(ResourceUpdateWarningCodeEnum::MISSINGTYPEDEPENDENCY),
           "EXTERNAL_API_WARNING" => Ok(ResourceUpdateWarningCodeEnum::EXTERNALAPIWARNING),
           "SCHEMA_VALIDATION_IGNORED" => Ok(ResourceUpdateWarningCodeEnum::SCHEMAVALIDATIONIGNORED),
           "UNDECLARED_PROPERTIES" => Ok(ResourceUpdateWarningCodeEnum::UNDECLAREDPROPERTIES),
           "EXPERIMENTAL_TYPE_USED" => Ok(ResourceUpdateWarningCodeEnum::EXPERIMENTALTYPEUSED),
           "DEPRECATED_TYPE_USED" => Ok(ResourceUpdateWarningCodeEnum::DEPRECATEDTYPEUSED),
           "PARTIAL_SUCCESS" => Ok(ResourceUpdateWarningCodeEnum::PARTIALSUCCESS),
           "LARGE_DEPLOYMENT_WARNING" => Ok(ResourceUpdateWarningCodeEnum::LARGEDEPLOYMENTWARNING),
           "NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE" => Ok(ResourceUpdateWarningCodeEnum::NEXTHOPINSTANCEHASNOIPV6INTERFACE),
           "INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB" => Ok(ResourceUpdateWarningCodeEnum::INVALIDHEALTHCHECKFORDYNAMICWIEGHTEDLB),
           "LIST_OVERHEAD_QUOTA_EXCEED" => Ok(ResourceUpdateWarningCodeEnum::LISTOVERHEADQUOTAEXCEED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResourceUpdateWarningCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeploymentDeletePolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sets the policy to use for deleting resources.
pub enum DeploymentDeletePolicyEnum {
    
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
    
    /// "ABANDON"
    #[serde(rename="ABANDON")]
    ABANDON,
}

impl AsRef<str> for DeploymentDeletePolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeploymentDeletePolicyEnum::DELETE => "DELETE",
            DeploymentDeletePolicyEnum::ABANDON => "ABANDON",
        }
    }
}

impl std::convert::TryFrom< &str> for DeploymentDeletePolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DELETE" => Ok(DeploymentDeletePolicyEnum::DELETE),
           "ABANDON" => Ok(DeploymentDeletePolicyEnum::ABANDON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeploymentDeletePolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for DeploymentDeletePolicyEnum {
    fn default() -> DeploymentDeletePolicyEnum {
        DeploymentDeletePolicyEnum::DELETE
    }
}

// endregion


// region DeploymentCreatePolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sets the policy to use for creating new resources.
pub enum DeploymentCreatePolicyEnum {
    
    /// "CREATE_OR_ACQUIRE"
    #[serde(rename="CREATE_OR_ACQUIRE")]
    CREATEORACQUIRE,
    
    /// "ACQUIRE"
    #[serde(rename="ACQUIRE")]
    ACQUIRE,
}

impl AsRef<str> for DeploymentCreatePolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeploymentCreatePolicyEnum::CREATEORACQUIRE => "CREATE_OR_ACQUIRE",
            DeploymentCreatePolicyEnum::ACQUIRE => "ACQUIRE",
        }
    }
}

impl std::convert::TryFrom< &str> for DeploymentCreatePolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATE_OR_ACQUIRE" => Ok(DeploymentCreatePolicyEnum::CREATEORACQUIRE),
           "ACQUIRE" => Ok(DeploymentCreatePolicyEnum::ACQUIRE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeploymentCreatePolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for DeploymentCreatePolicyEnum {
    fn default() -> DeploymentCreatePolicyEnum {
        DeploymentCreatePolicyEnum::CREATEORACQUIRE
    }
}

// endregion


