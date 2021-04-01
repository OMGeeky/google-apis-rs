initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["ApplicationInfo","Application related details of a job posting."],["BatchDeleteJobsRequest","Input only. Batch delete jobs request."],["BucketRange","Represents starting and ending value of a range in double."],["BucketizedCount","Represents count of jobs within one bucket."],["ClientEvent","An event issued when an end user interacts with the application that implements Cloud Talent Solution. Providing this information improves the quality of search and recommendation for the API clients, enabling the service to perform optimally. The number of events sent must be consistent with other calls, such as job searches, issued to the service by the client."],["CloudTalentSolution","Central instance to access all CloudTalentSolution related resource activities"],["CommuteFilter","Input only. Parameters needed for commute search."],["CommuteInfo","Output only. Commute details related to this job."],["Company","A Company resource represents a company in the service. A company is the entity that owns job postings, that is, the hiring entity responsible for employing applicants for the job position."],["CompanyDerivedInfo","Derived details about the company."],["CompensationEntry","A compensation entry that represents one component of compensation, such as base pay, bonus, or other compensation type. Annualization: One compensation entry can be annualized if - it contains valid amount or range. - and its expected_units_per_year is set or can be derived. Its annualized range is determined as (amount or range) times expected_units_per_year."],["CompensationFilter","Input only. Filter on job compensation type and amount."],["CompensationHistogramRequest","Input only. Compensation based histogram request."],["CompensationHistogramResult","Output only. Compensation based histogram result."],["CompensationInfo","Job compensation details."],["CompensationRange","Compensation range."],["CompleteQueryResponse","Output only. Response of auto-complete query."],["CompletionResult","Output only. Resource that represents completion results."],["CreateClientEventRequest","The report event request."],["CreateCompanyRequest","Input only. The Request of the CreateCompany method."],["CreateJobRequest","Input only. Create job request."],["CustomAttribute","Custom attribute values that are either filterable or non-filterable."],["CustomAttributeHistogramRequest","Custom attributes histogram request. An error is thrown if neither string_value_histogram or long_value_histogram_bucketing_option has been defined."],["CustomAttributeHistogramResult","Output only. Custom attribute histogram result."],["DeviceInfo","Device information collected from the job seeker, candidate, or other entity conducting the job search. Providing this information improves the quality of the search results across devices."],["Empty","A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."],["HistogramFacets","Input only. Histogram facets to be specified in SearchJobsRequest."],["HistogramResult","Output only. Result of a histogram call. The response contains the histogram map for the search type specified by HistogramResult.field. The response is a map of each filter value to the corresponding count of jobs for that filter."],["HistogramResults","Output only. Histogram results that match HistogramFacets specified in SearchJobsRequest."],["Job","A Job resource represents a job posting (also referred to as a \"job listing\" or \"job requisition\"). A job belongs to a Company, which is the hiring entity responsible for the job."],["JobDerivedInfo","Output only. Derived details about the job posting."],["JobEvent","An event issued when a job seeker interacts with the application that implements Cloud Talent Solution."],["JobQuery","Input only. The query required to perform a search query."],["LatLng","An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges."],["ListCompaniesResponse","Output only. The List companies response object."],["ListJobsResponse","Output only. List jobs response."],["Location","Output only. A resource that represents a location with full geographic information."],["LocationFilter","Input only. Geographic region of the search."],["MatchingJob","Output only. Job entry with metadata inside SearchJobsResponse."],["Money","Represents an amount of money with its currency type."],["NumericBucketingOption","Input only. Use this field to specify bucketing option for the histogram search response."],["NumericBucketingResult","Output only. Custom numeric bucketing result."],["PostalAddress","Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an i18n-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478"],["ProcessingOptions","Input only. Options for job processing."],["ProjectClientEventCreateCall","Report events issued when end user interacts with customer's application that uses Cloud Talent Solution. You may inspect the created events in self service tools. Learn more about self service tools."],["ProjectCompanyCreateCall","Creates a new company entity."],["ProjectCompanyDeleteCall","Deletes specified company. Prerequisite: The company has no jobs associated with it."],["ProjectCompanyGetCall","Retrieves specified company."],["ProjectCompanyListCall","Lists all companies associated with the service account."],["ProjectCompanyPatchCall","Updates specified company. Company names can't be updated. To update a company name, delete the company and all jobs associated with it, and only then re-create them."],["ProjectCompleteCall","Completes the specified prefix with keyword suggestions. Intended for use by a job search auto-complete search box."],["ProjectJobBatchDeleteCall","Deletes a list of Jobs by filter."],["ProjectJobCreateCall","Creates a new job. Typically, the job becomes searchable within 10 seconds, but it may take up to 5 minutes."],["ProjectJobDeleteCall","Deletes the specified job. Typically, the job becomes unsearchable within 10 seconds, but it may take up to 5 minutes."],["ProjectJobGetCall","Retrieves the specified job, whose status is OPEN or recently EXPIRED within the last 90 days."],["ProjectJobListCall","Lists jobs by filter."],["ProjectJobPatchCall","Updates specified job. Typically, updated contents become visible in search results within 10 seconds, but it may take up to 5 minutes."],["ProjectJobSearchCall","Searches for jobs using the provided SearchJobsRequest. This call constrains the visibility of jobs present in the database, and only returns jobs that the caller has permission to search against."],["ProjectJobSearchForAlertCall","Searches for jobs using the provided SearchJobsRequest. This API call is intended for the use case of targeting passive job seekers (for example, job seekers who have signed up to receive email alerts about potential job opportunities), and has different algorithmic adjustments that are targeted to passive job seekers. This call constrains the visibility of jobs present in the database, and only returns jobs the caller has permission to search against."],["ProjectMethods","A builder providing access to all methods supported on project resources. It is not used directly, but through the `CloudTalentSolution` hub."],["RequestMetadata","Input only. Meta information related to the job searcher or entity conducting the job search. This information is used to improve the performance of the service."],["ResponseMetadata","Output only. Additional information returned to client, such as debugging information."],["SearchJobsRequest","Input only. The Request body of the `SearchJobs` call."],["SearchJobsResponse","Output only. Response for SearchJob method."],["SpellingCorrection","Output only. Spell check result."],["TimeOfDay","Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`."],["TimestampRange","Message representing a period of time between two timestamps."],["UpdateCompanyRequest","Input only. Request for updating a specified company."],["UpdateJobRequest","Input only. Update job request."]]});