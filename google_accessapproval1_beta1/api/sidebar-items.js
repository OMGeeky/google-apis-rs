initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["AccessApproval","Central instance to access all AccessApproval related resource activities"],["AccessApprovalSettings","Settings on a Project/Folder/Organization related to Access Approval."],["AccessLocations","Home office and physical location of the principal."],["AccessReason","There is no detailed description."],["ApprovalRequest","A request for the customer to approve access to a resource."],["ApproveApprovalRequestMessage","Request to approve an ApprovalRequest."],["ApproveDecision","A decision that has been made to approve access to a resource."],["DismissApprovalRequestMessage","Request to dismiss an approval request."],["DismissDecision","A decision that has been made to dismiss an approval request."],["Empty","A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance:"],["EnrolledService","Represents the enrollment of a cloud resource into a specific service."],["FolderApprovalRequestApproveCall","Approves a request and returns the updated ApprovalRequest."],["FolderApprovalRequestDismisCall","Dismisses a request. Returns the updated ApprovalRequest."],["FolderApprovalRequestGetCall","Gets an approval request. Returns NOT_FOUND if the request does not exist."],["FolderApprovalRequestListCall","Lists approval requests associated with a project, folder, or organization. Approval requests can be filtered by state (pending, active, dismissed). The order is reverse chronological."],["FolderDeleteAccessApprovalSettingCall","Deletes the settings associated with a project, folder, or organization. This will have the effect of disabling Access Approval for the project, folder, or organization, but only if all ancestors also have Access Approval disabled. If Access Approval is enabled at a higher level of the hierarchy, then Access Approval will still be enabled at this level as the settings are inherited."],["FolderGetAccessApprovalSettingCall","Gets the settings associated with a project, folder, or organization."],["FolderMethods","A builder providing access to all methods supported on folder resources. It is not used directly, but through the `AccessApproval` hub."],["FolderUpdateAccessApprovalSettingCall","Updates the settings associated with a project, folder, or organization. Settings to update are determined by the value of field_mask."],["ListApprovalRequestsResponse","Response to listing of ApprovalRequest objects."],["OrganizationApprovalRequestApproveCall","Approves a request and returns the updated ApprovalRequest."],["OrganizationApprovalRequestDismisCall","Dismisses a request. Returns the updated ApprovalRequest."],["OrganizationApprovalRequestGetCall","Gets an approval request. Returns NOT_FOUND if the request does not exist."],["OrganizationApprovalRequestListCall","Lists approval requests associated with a project, folder, or organization. Approval requests can be filtered by state (pending, active, dismissed). The order is reverse chronological."],["OrganizationDeleteAccessApprovalSettingCall","Deletes the settings associated with a project, folder, or organization. This will have the effect of disabling Access Approval for the project, folder, or organization, but only if all ancestors also have Access Approval disabled. If Access Approval is enabled at a higher level of the hierarchy, then Access Approval will still be enabled at this level as the settings are inherited."],["OrganizationGetAccessApprovalSettingCall","Gets the settings associated with a project, folder, or organization."],["OrganizationMethods","A builder providing access to all methods supported on organization resources. It is not used directly, but through the `AccessApproval` hub."],["OrganizationUpdateAccessApprovalSettingCall","Updates the settings associated with a project, folder, or organization. Settings to update are determined by the value of field_mask."],["ProjectApprovalRequestApproveCall","Approves a request and returns the updated ApprovalRequest."],["ProjectApprovalRequestDismisCall","Dismisses a request. Returns the updated ApprovalRequest."],["ProjectApprovalRequestGetCall","Gets an approval request. Returns NOT_FOUND if the request does not exist."],["ProjectApprovalRequestListCall","Lists approval requests associated with a project, folder, or organization. Approval requests can be filtered by state (pending, active, dismissed). The order is reverse chronological."],["ProjectDeleteAccessApprovalSettingCall","Deletes the settings associated with a project, folder, or organization. This will have the effect of disabling Access Approval for the project, folder, or organization, but only if all ancestors also have Access Approval disabled. If Access Approval is enabled at a higher level of the hierarchy, then Access Approval will still be enabled at this level as the settings are inherited."],["ProjectGetAccessApprovalSettingCall","Gets the settings associated with a project, folder, or organization."],["ProjectMethods","A builder providing access to all methods supported on project resources. It is not used directly, but through the `AccessApproval` hub."],["ProjectUpdateAccessApprovalSettingCall","Updates the settings associated with a project, folder, or organization. Settings to update are determined by the value of field_mask."],["ResourceProperties","The properties associated with the resource of the request."]]});