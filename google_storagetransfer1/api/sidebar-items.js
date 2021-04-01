initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["AwsAccessKey","AWS access key (see AWS Security Credentials). For information on our data retention policy for user credentials, see User credentials."],["AwsS3Data","An AwsS3Data resource can be a data source, but not a data sink. In an AwsS3Data resource, an object's name is the S3 object's key name."],["AzureBlobStorageData","An AzureBlobStorageData resource can be a data source, but not a data sink. An AzureBlobStorageData resource represents one Azure container. The storage account determines the Azure endpoint. In an AzureBlobStorageData resource, a blobs's name is the Azure Blob Storage blob's key name."],["AzureCredentials","Azure credentials For information on our data retention policy for user credentials, see User credentials."],["CancelOperationRequest","The request message for Operations.CancelOperation."],["Date","Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."],["Empty","A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."],["ErrorLogEntry","An entry describing an error that has occurred."],["ErrorSummary","A summary of errors by error code, plus a count and sample error log entries."],["GcsData","In a GcsData resource, an object's name is the Cloud Storage object's name and its \"last modification time\" refers to the object's `updated` property of Cloud Storage objects, which changes when the content or the metadata of the object is updated."],["GoogleServiceAccount","Google service account"],["GoogleServiceAccountGetCall","Returns the Google service account that is used by Storage Transfer Service to access buckets in the project where transfers run or in other projects. Each Google service account is associated with one Google Cloud Platform Console project. Users should add this service account to the Google Cloud Storage bucket ACLs to grant access to Storage Transfer Service. This service account is created and owned by Storage Transfer Service and can only be used by Storage Transfer Service."],["GoogleServiceAccountMethods","A builder providing access to all methods supported on googleServiceAccount resources. It is not used directly, but through the `Storagetransfer` hub."],["HttpData","An HttpData resource specifies a list of objects on the web to be transferred over HTTP. The information of the objects to be transferred is contained in a file referenced by a URL. The first line in the file must be `\"TsvHttpData-1.0\"`, which specifies the format of the file. Subsequent lines specify the information of the list of objects, one object per list entry. Each entry has the following tab-delimited fields: * HTTP URL — The location of the object. * Length — The size of the object in bytes. * MD5 — The base64-encoded MD5 hash of the object. For an example of a valid TSV file, see Transferring data from URLs. When transferring data based on a URL list, keep the following in mind: * When an object located at `http(s)://hostname:port/` is transferred to a data sink, the name of the object at the data sink is `/`. * If the specified size of an object does not match the actual size of the object fetched, the object will not be transferred. * If the specified MD5 does not match the MD5 computed from the transferred bytes, the object transfer will fail. * Ensure that each URL you specify is publicly accessible. For example, in Cloud Storage you can [share an object publicly] (https://cloud.google.com/storage/docs/cloud-console#_sharingdata) and get a link to it. * Storage Transfer Service obeys `robots.txt` rules and requires the source HTTP server to support `Range` requests and to return a `Content-Length` header in each response. * ObjectConditions have no effect when filtering objects to transfer."],["ListOperationsResponse","The response message for Operations.ListOperations."],["ListTransferJobsResponse","Response from ListTransferJobs."],["NotificationConfig","Specification to configure notifications published to Cloud Pub/Sub. Notifications will be published to the customer-provided topic using the following `PubsubMessage.attributes`: * `\"eventType\"`: one of the EventType values * `\"payloadFormat\"`: one of the PayloadFormat values * `\"projectId\"`: the project_id of the `TransferOperation` * `\"transferJobName\"`: the transfer_job_name of the `TransferOperation` * `\"transferOperationName\"`: the name of the `TransferOperation` The `PubsubMessage.data` will contain a TransferOperation resource formatted according to the specified `PayloadFormat`."],["ObjectConditions","Conditions that determine which objects will be transferred. Applies only to Cloud Data Sources such as S3, Azure, and Cloud Storage. The \"last modification time\" refers to the time of the last change to the object's content or metadata — specifically, this is the `updated` property of Cloud Storage objects, the `LastModified` field of S3 objects, and the `Last-Modified` header of Azure blobs."],["Operation","This resource represents a long-running operation that is the result of a network API call."],["PauseTransferOperationRequest","Request passed to PauseTransferOperation."],["ResumeTransferOperationRequest","Request passed to ResumeTransferOperation."],["RunTransferJobRequest","Request passed to RunTransferJob."],["Schedule","Transfers can be scheduled to recur or to run just once."],["Status","The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC. Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the API Design Guide."],["Storagetransfer","Central instance to access all Storagetransfer related resource activities"],["TimeOfDay","Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`."],["TransferCounters","A collection of counters that report the progress of a transfer operation."],["TransferJob","This resource represents the configuration of a transfer job that runs periodically."],["TransferJobCreateCall","Creates a transfer job that runs periodically."],["TransferJobGetCall","Gets a transfer job."],["TransferJobListCall","Lists transfer jobs."],["TransferJobMethods","A builder providing access to all methods supported on transferJob resources. It is not used directly, but through the `Storagetransfer` hub."],["TransferJobPatchCall","Updates a transfer job. Updating a job's transfer spec does not affect transfer operations that are running already. Note: The job's status field can be modified using this RPC (for example, to set a job's status to DELETED, DISABLED, or ENABLED)."],["TransferJobRunCall","Attempts to start a new TransferOperation for the current TransferJob. A TransferJob has a maximum of one active TransferOperation. If this method is called while a TransferOperation is active, an error wil be returned."],["TransferOperation","A description of the execution of a transfer."],["TransferOperationCancelCall","Cancels a transfer. Use the transferOperations.get method to check if the cancellation succeeded or if the operation completed despite the `cancel` request. When you cancel an operation, the currently running transfer is interrupted. For recurring transfer jobs, the next instance of the transfer job will still run. For example, if your job is configured to run every day at 1pm and you cancel Monday's operation at 1:05pm, Monday's transfer will stop. However, a transfer job will still be attempted on Tuesday. This applies only to currently running operations. If an operation is not currently running, `cancel` does nothing. Caution: Canceling a transfer job can leave your data in an unknown state. We recommend that you restore the state at both the destination and the source after the `cancel` request completes so that your data is in a consistent state. When you cancel a job, the next job computes a delta of files and may repair any inconsistent state. For instance, if you run a job every day, and today's job found 10 new files and transferred five files before you canceled the job, tomorrow's transfer operation will compute a new delta with the five files that were not copied today plus any new files discovered tomorrow."],["TransferOperationGetCall","Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."],["TransferOperationListCall","Lists transfer operations. Operations are ordered by their creation time in reverse chronological order."],["TransferOperationMethods","A builder providing access to all methods supported on transferOperation resources. It is not used directly, but through the `Storagetransfer` hub."],["TransferOperationPauseCall","Pauses a transfer operation."],["TransferOperationResumeCall","Resumes a transfer operation that is paused."],["TransferOptions","TransferOptions define the actions to be performed on objects in a transfer."],["TransferSpec","Configuration for running a transfer."],["UpdateTransferJobRequest","Request passed to UpdateTransferJob."]]});