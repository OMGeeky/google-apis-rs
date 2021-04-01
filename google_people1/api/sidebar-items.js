initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["Address","A person's physical address. May be a P.O. box or street address. All fields are optional."],["AgeRangeType","A person's age range."],["BatchCreateContactsRequest","A request to create a batch of contacts."],["BatchCreateContactsResponse","The response to a request to create a batch of contacts."],["BatchDeleteContactsRequest","A request to delete a batch of existing contacts."],["BatchGetContactGroupsResponse","The response to a batch get contact groups request."],["BatchUpdateContactsRequest","A request to update a batch of contacts."],["BatchUpdateContactsResponse","The response to a request to create a batch of contacts."],["Biography","A person's short biography."],["Birthday","A person's birthday. At least one of the `date` and `text` fields are specified. The `date` and `text` fields typically represent the same date, but are not guaranteed to."],["BraggingRights","DEPRECATED: No data will be returned A person's bragging rights."],["CalendarUrl","A person's calendar URL."],["ClientData","Arbitrary client data that is populated by clients. Duplicate keys and values are allowed."],["ContactGroup","A contact group."],["ContactGroupBatchGetCall","Get a list of contact groups owned by the authenticated user by specifying a list of contact group resource names."],["ContactGroupCreateCall","Create a new contact group owned by the authenticated user."],["ContactGroupDeleteCall","Delete an existing contact group owned by the authenticated user by specifying a contact group resource name."],["ContactGroupGetCall","Get a specific contact group owned by the authenticated user by specifying a contact group resource name."],["ContactGroupListCall","List all contact groups owned by the authenticated user. Members of the contact groups are not populated."],["ContactGroupMemberModifyCall","Modify the members of a contact group owned by the authenticated user. The only system contact groups that can have members added are `contactGroups/myContacts` and `contactGroups/starred`. Other system contact groups are deprecated and can only have contacts removed."],["ContactGroupMembership","A Google contact group membership."],["ContactGroupMetadata","The metadata about a contact group."],["ContactGroupMethods","A builder providing access to all methods supported on contactGroup resources. It is not used directly, but through the `PeopleService` hub."],["ContactGroupResponse","The response for a specific contact group."],["ContactGroupUpdateCall","Update the name of an existing contact group owned by the authenticated user."],["ContactToCreate","A wrapper that contains the person data to populate a newly created source."],["CopyOtherContactToMyContactsGroupRequest","A request to copy an \"Other contact\" to my contacts group."],["CoverPhoto","A person's cover photo. A large image shown on the person's profile page that represents who they are or what they care about."],["CreateContactGroupRequest","A request to create a new contact group."],["Date","Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."],["DeleteContactPhotoResponse","The response for deleting a contact's photo."],["DomainMembership","A G Suite Domain membership."],["EmailAddress","A person's email address."],["Empty","A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."],["Event","An event related to the person."],["ExternalId","An identifier from an external entity related to the person."],["FieldMetadata","Metadata about a field."],["FileAs","The name that should be used to sort the person in a list."],["Gender","A person's gender."],["GetPeopleResponse","The response to a get request for a list of people by resource name."],["GroupClientData","Arbitrary client data that is populated by clients. Duplicate keys and values are allowed."],["ImClient","A person's instant messaging client."],["Interest","One of the person's interests."],["ListConnectionsResponse","The response to a request for the authenticated user's connections."],["ListContactGroupsResponse","The response to a list contact groups request."],["ListDirectoryPeopleResponse","The response to a request for the authenticated user's domain directory."],["ListOtherContactsResponse","The response to a request for the authenticated user's \"Other contacts\"."],["Locale","A person's locale preference."],["Location","A person's location."],["Membership","A person's membership in a group. Only contact group memberships can be modified."],["MiscKeyword","A person's miscellaneous keyword."],["ModifyContactGroupMembersRequest","A request to modify an existing contact group's members. Contacts can be removed from any group but they can only be added to a user group or \"myContacts\" or \"starred\" system groups."],["ModifyContactGroupMembersResponse","The response to a modify contact group members request."],["Name","A person's name. If the name is a mononym, the family name is empty."],["Nickname","A person's nickname."],["Occupation","A person's occupation."],["Organization","A person's past or current organization. Overlapping date ranges are permitted."],["OtherContactCopyOtherContactToMyContactsGroupCall","Copies an \"Other contact\" to a new contact in the user's \"myContacts\" group"],["OtherContactListCall","List all \"Other contacts\", that is contacts that are not in a contact group. \"Other contacts\" are typically auto created contacts from interactions."],["OtherContactMethods","A builder providing access to all methods supported on otherContact resources. It is not used directly, but through the `PeopleService` hub."],["OtherContactSearchCall","Provides a list of contacts in the authenticated user's other contacts that matches the search query. The query matches on a contact's `names`, `emailAddresses`, and `phoneNumbers` fields that are from the OTHER_CONTACT source."],["PeopleBatchCreateContactCall","Create a batch of new contacts and return the PersonResponses for the newly created contacts. Limited to 10 parallel requests per user."],["PeopleBatchDeleteContactCall","Delete a batch of contacts. Any non-contact data will not be deleted. Limited to 10 parallel requests per user."],["PeopleBatchUpdateContactCall","Update a batch of contacts and return a map of resource names to PersonResponses for the updated contacts. Limited to 10 parallel requests per user."],["PeopleConnectionListCall","Provides a list of the authenticated user's contacts. The request returns a 400 error if `personFields` is not specified. The request returns a 410 error if `sync_token` is specified and is expired. Sync tokens expire after 7 days to prevent data drift between clients and the server. To handle a sync token expired error, a request should be sent without `sync_token` to get all contacts."],["PeopleCreateContactCall","Create a new contact and return the person resource for that contact. The request returns a 400 error if more than one field is specified on a field that is a singleton for contact sources: * biographies * birthdays * genders * names"],["PeopleDeleteContactCall","Delete a contact person. Any non-contact data will not be deleted."],["PeopleDeleteContactPhotoCall","Delete a contact's photo."],["PeopleGetBatchGetCall","Provides information about a list of specific people by specifying a list of requested resource names. Use `people/me` to indicate the authenticated user. The request returns a 400 error if 'personFields' is not specified."],["PeopleGetCall","Provides information about a person by specifying a resource name. Use `people/me` to indicate the authenticated user. The request returns a 400 error if 'personFields' is not specified."],["PeopleListDirectoryPeopleCall","Provides a list of domain profiles and domain contacts in the authenticated user's domain directory."],["PeopleMethods","A builder providing access to all methods supported on people resources. It is not used directly, but through the `PeopleService` hub."],["PeopleSearchContactCall","Provides a list of contacts in the authenticated user's grouped contacts that matches the search query. The query matches on a contact's `names`, `nickNames`, `emailAddresses`, `phoneNumbers`, and `organizations` fields that are from the CONTACT\" source."],["PeopleSearchDirectoryPeopleCall","Provides a list of domain profiles and domain contacts in the authenticated user's domain directory that match the search query."],["PeopleService","Central instance to access all PeopleService related resource activities"],["PeopleUpdateContactCall","Update contact data for an existing contact person. Any non-contact data will not be modified. Any non-contact data in the person to update will be ignored. All fields specified in the `update_mask` will be replaced. The server returns a 400 error if `person.metadata.sources` is not specified for the contact to be updated or if there is no contact source. The server returns a 400 error with reason `\"failedPrecondition\"` if `person.metadata.sources.etag` is different than the contact's etag, which indicates the contact has changed since its data was read. Clients should get the latest person and merge their updates into the latest person. The server returns a 400 error if `memberships` are being updated and there are no contact group memberships specified on the person. The server returns a 400 error if more than one field is specified on a field that is a singleton for contact sources: * biographies * birthdays * genders * names"],["PeopleUpdateContactPhotoCall","Update a contact's photo."],["Person","Information about a person merged from various data sources such as the authenticated user's contacts and profile data. Most fields can have multiple items. The items in a field have no guaranteed order, but each non-empty field is guaranteed to have exactly one field with `metadata.primary` set to true."],["PersonMetadata","The metadata about a person."],["PersonResponse","The response for a single person"],["PhoneNumber","A person's phone number."],["Photo","A person's photo. A picture shown next to the person's name to help others recognize the person."],["ProfileMetadata","The metadata about a profile."],["Relation","A person's relation to another person."],["RelationshipInterest","DEPRECATED: No data will be returned A person's relationship interest ."],["RelationshipStatus","DEPRECATED: No data will be returned A person's relationship status."],["Residence","DEPRECATED: Please use `person.locations` instead. A person's past or current residence."],["SearchDirectoryPeopleResponse","The response to a request for people in the authenticated user's domain directory that match the specified query."],["SearchResponse","The response to a search request for the authenticated user, given a query."],["SearchResult","A result of a search query."],["SipAddress","A person's SIP address. Session Initial Protocol addresses are used for VoIP communications to make voice or video calls over the internet."],["Skill","A skill that the person has."],["Source","The source of a field."],["Status","The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC. Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the API Design Guide."],["Tagline","DEPRECATED: No data will be returned A brief one-line description of the person."],["UpdateContactGroupRequest","A request to update an existing user contact group. All updated fields will be replaced."],["UpdateContactPhotoRequest","A request to update an existing contact's photo. All requests must have a valid photo format: JPEG or PNG."],["UpdateContactPhotoResponse","The response for updating a contact's photo."],["Url","A person's associated URLs."],["UserDefined","Arbitrary user data that is populated by the end users."]]});