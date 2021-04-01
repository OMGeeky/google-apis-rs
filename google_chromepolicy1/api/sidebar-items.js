initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["ChromePolicy","Central instance to access all ChromePolicy related resource activities"],["CustomerMethods","A builder providing access to all methods supported on customer resources. It is not used directly, but through the `ChromePolicy` hub."],["CustomerPolicyOrgunitBatchInheritCall","Modify multiple policy values that are applied to a specific org unit so that they now inherit the value from a parent (if applicable). All targets must have the same target format. That is to say that they must point to the same target resource and must have the same keys specified in `additionalTargetKeyNames`. On failure the request will return the error details as part of the google.rpc.Status."],["CustomerPolicyOrgunitBatchModifyCall","Modify multiple policy values that are applied to a specific org unit. All targets must have the same target format. That is to say that they must point to the same target resource and must have the same keys specified in `additionalTargetKeyNames`. On failure the request will return the error details as part of the google.rpc.Status."],["CustomerPolicyResolveCall","Gets the resolved policy values for a list of policies that match a search query."],["CustomerPolicySchemaGetCall","Get a specific policy schema for a customer by its resource name."],["CustomerPolicySchemaListCall","Gets a list of policy schemas that match a specified filter value for a given customer."],["GoogleChromePolicyV1AdditionalTargetKeyName","Additional key names that will be used to identify the target of the policy value."],["GoogleChromePolicyV1BatchInheritOrgUnitPoliciesRequest","Request message for specifying that multiple policy values inherit their value from their parents."],["GoogleChromePolicyV1BatchModifyOrgUnitPoliciesRequest","Request message for modifying multiple policy values for a specific target."],["GoogleChromePolicyV1InheritOrgUnitPolicyRequest","Request parameters for inheriting policy value of a specific org unit target from the policy value of its parent org unit."],["GoogleChromePolicyV1ListPolicySchemasResponse","Response message for listing policy schemas that match a filter."],["GoogleChromePolicyV1ModifyOrgUnitPolicyRequest","Request parameters for modifying a policy value for a specific org unit target."],["GoogleChromePolicyV1PolicySchema","Resource representing a policy schema. Next ID: 10"],["GoogleChromePolicyV1PolicySchemaFieldDescription","Provides detailed information for a particular field that is part of a PolicySchema."],["GoogleChromePolicyV1PolicySchemaFieldKnownValueDescription","Provides detailed information about a known value that is allowed for a particular field in a PolicySchema."],["GoogleChromePolicyV1PolicySchemaNoticeDescription","Provides special notice messages related to a particular value in a field that is part of a PolicySchema."],["GoogleChromePolicyV1PolicyTargetKey","The key used to identify the target on which the policy will be applied."],["GoogleChromePolicyV1PolicyValue","A particular value for a policy managed by the service."],["GoogleChromePolicyV1ResolveRequest","Request message for getting the resolved policy value for a specific target."],["GoogleChromePolicyV1ResolveResponse","Response message for getting the resolved policy value for a specific target."],["GoogleChromePolicyV1ResolvedPolicy","The resolved value of a policy for a given target."],["GoogleProtobufEmpty","A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."],["Proto2DescriptorProto","Describes a message type."],["Proto2EnumDescriptorProto","Describes an enum type."],["Proto2EnumValueDescriptorProto","Describes a value within an enum."],["Proto2FieldDescriptorProto","Describes a field within a message."],["Proto2FileDescriptorProto","Describes a complete .proto file."],["Proto2OneofDescriptorProto","Describes a oneof."]]});