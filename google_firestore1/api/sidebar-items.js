initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["ArrayValue","An array value."],["BatchGetDocumentsRequest","The request for Firestore.BatchGetDocuments."],["BatchGetDocumentsResponse","The streamed response for Firestore.BatchGetDocuments."],["BatchWriteRequest","The request for Firestore.BatchWrite."],["BatchWriteResponse","The response from Firestore.BatchWrite."],["BeginTransactionRequest","The request for Firestore.BeginTransaction."],["BeginTransactionResponse","The response for Firestore.BeginTransaction."],["CollectionSelector","A selection of a collection, such as `messages as m1`."],["CommitRequest","The request for Firestore.Commit."],["CommitResponse","The response for Firestore.Commit."],["CompositeFilter","A filter that merges multiple other filters using the given operator."],["Cursor","A position in a query result set."],["Document","A Firestore document. Must not exceed 1 MiB - 4 bytes."],["DocumentChange","A Document has changed. May be the result of multiple writes, including deletes, that ultimately resulted in a new value for the Document. Multiple DocumentChange messages may be returned for the same logical change, if multiple targets are affected."],["DocumentDelete","A Document has been deleted. May be the result of multiple writes, including updates, the last of which deleted the Document. Multiple DocumentDelete messages may be returned for the same logical delete, if multiple targets are affected."],["DocumentMask","A set of field paths on a document. Used to restrict a get or update operation on a document to a subset of its fields. This is different from standard field masks, as this is always scoped to a Document, and takes in account the dynamic nature of Value."],["DocumentRemove","A Document has been removed from the view of the targets. Sent if the document is no longer relevant to a target and is out of view. Can be sent instead of a DocumentDelete or a DocumentChange if the server can not send the new value of the document. Multiple DocumentRemove messages may be returned for the same logical write or delete, if multiple targets are affected."],["DocumentTransform","A transformation of a document."],["DocumentsTarget","A target specified by a set of documents names."],["Empty","A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."],["ExistenceFilter","A digest of all the documents that match a given target."],["FieldFilter","A filter on a specific field."],["FieldReference","A reference to a field, such as `max(messages.time) as max_time`."],["FieldTransform","A transformation of a field of the document."],["Filter","A filter."],["Firestore","Central instance to access all Firestore related resource activities"],["GoogleFirestoreAdminV1ExportDocumentsRequest","The request for FirestoreAdmin.ExportDocuments."],["GoogleFirestoreAdminV1Field","Represents a single field in the database. Fields are grouped by their \"Collection Group\", which represent all collections in the database with the same id."],["GoogleFirestoreAdminV1ImportDocumentsRequest","The request for FirestoreAdmin.ImportDocuments."],["GoogleFirestoreAdminV1Index","Cloud Firestore indexes enable simple and complex queries against documents in a database."],["GoogleFirestoreAdminV1IndexConfig","The index configuration for this field."],["GoogleFirestoreAdminV1IndexField","A field in an index. The field_path describes which field is indexed, the value_mode describes how the field value is indexed."],["GoogleFirestoreAdminV1ListFieldsResponse","The response for FirestoreAdmin.ListFields."],["GoogleFirestoreAdminV1ListIndexesResponse","The response for FirestoreAdmin.ListIndexes."],["GoogleLongrunningCancelOperationRequest","The request message for Operations.CancelOperation."],["GoogleLongrunningListOperationsResponse","The response message for Operations.ListOperations."],["GoogleLongrunningOperation","This resource represents a long-running operation that is the result of a network API call."],["LatLng","An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges."],["ListCollectionIdsRequest","The request for Firestore.ListCollectionIds."],["ListCollectionIdsResponse","The response from Firestore.ListCollectionIds."],["ListDocumentsResponse","The response for Firestore.ListDocuments."],["ListLocationsResponse","The response message for Locations.ListLocations."],["ListenRequest","A request for Firestore.Listen"],["ListenResponse","The response for Firestore.Listen."],["Location","A resource that represents Google Cloud Platform location."],["MapValue","A map value."],["Order","An order on a field."],["PartitionQueryRequest","The request for Firestore.PartitionQuery."],["PartitionQueryResponse","The response for Firestore.PartitionQuery."],["Precondition","A precondition on a document, used for conditional operations."],["ProjectDatabaseCollectionGroupFieldGetCall","Gets the metadata and configuration for a Field."],["ProjectDatabaseCollectionGroupFieldListCall","Lists the field configuration and metadata for this database. Currently, FirestoreAdmin.ListFields only supports listing fields that have been explicitly overridden. To issue this query, call FirestoreAdmin.ListFields with the filter set to `indexConfig.usesAncestorConfig:false`."],["ProjectDatabaseCollectionGroupFieldPatchCall","Updates a field configuration. Currently, field updates apply only to single field index configuration. However, calls to FirestoreAdmin.UpdateField should provide a field mask to avoid changing any configuration that the caller isn't aware of. The field mask should be specified as: `{ paths: \"index_config\" }`. This call returns a google.longrunning.Operation which may be used to track the status of the field update. The metadata for the operation will be the type FieldOperationMetadata. To configure the default field settings for the database, use the special `Field` with resource name: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*`."],["ProjectDatabaseCollectionGroupIndexeCreateCall","Creates a composite index. This returns a google.longrunning.Operation which may be used to track the status of the creation. The metadata for the operation will be the type IndexOperationMetadata."],["ProjectDatabaseCollectionGroupIndexeDeleteCall","Deletes a composite index."],["ProjectDatabaseCollectionGroupIndexeGetCall","Gets a composite index."],["ProjectDatabaseCollectionGroupIndexeListCall","Lists composite indexes."],["ProjectDatabaseDocumentBatchGetCall","Gets multiple documents. Documents returned by this method are not guaranteed to be returned in the same order that they were requested."],["ProjectDatabaseDocumentBatchWriteCall","Applies a batch of write operations. The BatchWrite method does not apply the write operations atomically and can apply them out of order. Method does not allow more than one write per document. Each write succeeds or fails independently. See the BatchWriteResponse for the success status of each write. If you require an atomically applied set of writes, use Commit instead."],["ProjectDatabaseDocumentBeginTransactionCall","Starts a new transaction."],["ProjectDatabaseDocumentCommitCall","Commits a transaction, while optionally updating documents."],["ProjectDatabaseDocumentCreateDocumentCall","Creates a new document."],["ProjectDatabaseDocumentDeleteCall","Deletes a document."],["ProjectDatabaseDocumentGetCall","Gets a single document."],["ProjectDatabaseDocumentListCall","Lists documents."],["ProjectDatabaseDocumentListCollectionIdCall","Lists all the collection IDs underneath a document."],["ProjectDatabaseDocumentListenCall","Listens to changes."],["ProjectDatabaseDocumentPartitionQueryCall","Partitions a query by returning partition cursors that can be used to run the query in parallel. The returned partition cursors are split points that can be used by RunQuery as starting/end points for the query results."],["ProjectDatabaseDocumentPatchCall","Updates or inserts a document."],["ProjectDatabaseDocumentRollbackCall","Rolls back a transaction."],["ProjectDatabaseDocumentRunQueryCall","Runs a query."],["ProjectDatabaseDocumentWriteCall","Streams batches of document updates and deletes, in order."],["ProjectDatabaseExportDocumentCall","Exports a copy of all or a subset of documents from Google Cloud Firestore to another storage system, such as Google Cloud Storage. Recent updates to documents may not be reflected in the export. The export occurs in the background and its progress can be monitored and managed via the Operation resource that is created. The output of an export may only be used once the associated operation is done. If an export operation is cancelled before completion it may leave partial data behind in Google Cloud Storage. For more details on export behavior and output format, refer to: https://cloud.google.com/firestore/docs/manage-data/export-import"],["ProjectDatabaseImportDocumentCall","Imports documents into Google Cloud Firestore. Existing documents with the same name are overwritten. The import occurs in the background and its progress can be monitored and managed via the Operation resource that is created. If an ImportDocuments operation is cancelled, it is possible that a subset of the data has already been imported to Cloud Firestore."],["ProjectDatabaseOperationCancelCall","Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."],["ProjectDatabaseOperationDeleteCall","Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."],["ProjectDatabaseOperationGetCall","Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."],["ProjectDatabaseOperationListCall","Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."],["ProjectLocationGetCall","Gets information about a location."],["ProjectLocationListCall","Lists information about the supported locations for this service."],["ProjectMethods","A builder providing access to all methods supported on project resources. It is not used directly, but through the `Firestore` hub."],["Projection","The projection of document's fields to return."],["QueryTarget","A target specified by a query."],["ReadOnly","Options for a transaction that can only be used to read documents."],["ReadWrite","Options for a transaction that can be used to read and write documents."],["RollbackRequest","The request for Firestore.Rollback."],["RunQueryRequest","The request for Firestore.RunQuery."],["RunQueryResponse","The response for Firestore.RunQuery."],["Status","The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC. Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the API Design Guide."],["StructuredQuery","A Firestore query."],["Target","A specification of a set of documents to listen to."],["TargetChange","Targets being watched have changed."],["TransactionOptions","Options for creating a new transaction."],["UnaryFilter","A filter with a single operand."],["Value","A message that can hold any of the supported value types."],["Write","A write on a document."],["WriteRequest","The request for Firestore.Write. The first request creates a stream, or resumes an existing one from a token. When creating a new stream, the server replies with a response containing only an ID and a token, to use in the next request. When resuming a stream, the server first streams any responses later than the given token, then a response containing only an up-to-date token, to use in the next request."],["WriteResponse","The response for Firestore.Write."],["WriteResult","The result of applying a write."]]});