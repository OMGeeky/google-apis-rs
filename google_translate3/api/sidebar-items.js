initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["BatchTranslateTextRequest","The batch translation request."],["CancelOperationRequest","The request message for Operations.CancelOperation."],["DetectLanguageRequest","The request message for language detection."],["DetectLanguageResponse","The response message for language detection."],["DetectedLanguage","The response message for language detection."],["Empty","A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."],["GcsDestination","The Google Cloud Storage location for the output content."],["GcsSource","The Google Cloud Storage location for the input content."],["Glossary","Represents a glossary built from user provided data."],["GlossaryInputConfig","Input configuration for glossaries."],["InputConfig","Input configuration for BatchTranslateText request."],["LanguageCodePair","Used with unidirectional glossaries."],["LanguageCodesSet","Used with equivalent term set glossaries."],["ListGlossariesResponse","Response message for ListGlossaries."],["ListLocationsResponse","The response message for Locations.ListLocations."],["ListOperationsResponse","The response message for Operations.ListOperations."],["Location","A resource that represents Google Cloud Platform location."],["Operation","This resource represents a long-running operation that is the result of a network API call."],["OutputConfig","Output configuration for BatchTranslateText request."],["ProjectDetectLanguageCall","Detects the language of text within a request."],["ProjectGetSupportedLanguageCall","Returns a list of supported languages for translation."],["ProjectLocationBatchTranslateTextCall","Translates a large volume of text in asynchronous batch mode. This function provides real-time output as the inputs are being processed. If caller cancels a request, the partial results (for an input file, it's all or nothing) may still be available on the specified output location. This call returns immediately and you can use google.longrunning.Operation.name to poll the status of the call."],["ProjectLocationDetectLanguageCall","Detects the language of text within a request."],["ProjectLocationGetCall","Gets information about a location."],["ProjectLocationGetSupportedLanguageCall","Returns a list of supported languages for translation."],["ProjectLocationGlossaryCreateCall","Creates a glossary and returns the long-running operation. Returns NOT_FOUND, if the project doesn't exist."],["ProjectLocationGlossaryDeleteCall","Deletes a glossary, or cancels glossary construction if the glossary isn't created yet. Returns NOT_FOUND, if the glossary doesn't exist."],["ProjectLocationGlossaryGetCall","Gets a glossary. Returns NOT_FOUND, if the glossary doesn't exist."],["ProjectLocationGlossaryListCall","Lists glossaries in a project. Returns NOT_FOUND, if the project doesn't exist."],["ProjectLocationListCall","Lists information about the supported locations for this service."],["ProjectLocationOperationCancelCall","Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."],["ProjectLocationOperationDeleteCall","Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."],["ProjectLocationOperationGetCall","Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."],["ProjectLocationOperationListCall","Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."],["ProjectLocationOperationWaitCall","Waits until the specified long-running operation is done or reaches at most a specified timeout, returning the latest state. If the operation is already done, the latest state is immediately returned. If the timeout specified is greater than the default HTTP/RPC timeout, the HTTP/RPC timeout is used. If the server does not support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Note that this method is on a best-effort basis. It may return the latest state before the specified timeout (including immediately), meaning even an immediate response is no guarantee that the operation is done."],["ProjectLocationTranslateTextCall","Translates input text and returns translated text."],["ProjectMethods","A builder providing access to all methods supported on project resources. It is not used directly, but through the `Translate` hub."],["ProjectTranslateTextCall","Translates input text and returns translated text."],["Status","The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC. Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the API Design Guide."],["SupportedLanguage","A single supported language response corresponds to information related to one supported language."],["SupportedLanguages","The response message for discovering supported languages."],["Translate","Central instance to access all Translate related resource activities"],["TranslateTextGlossaryConfig","Configures which glossary should be used for a specific target language, and defines options for applying that glossary."],["TranslateTextRequest","The request message for synchronous translation."],["TranslateTextResponse","There is no detailed description."],["Translation","A single translation response."],["WaitOperationRequest","The request message for Operations.WaitOperation."]]});