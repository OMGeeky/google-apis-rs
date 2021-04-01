initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["Action","Information about the action."],["ActionDetail","Data describing the type and additional information of an action."],["ActivityMethods","A builder providing access to all methods supported on activity resources. It is not used directly, but through the `DriveActivityHub` hub."],["ActivityQueryCall","Query past activity in Google Drive."],["Actor","The actor of a Drive activity."],["Administrator","Empty message representing an administrator."],["AnonymousUser","Empty message representing an anonymous user or indicating the authenticated user should be anonymized."],["Anyone","Represents any user (including a logged out user)."],["ApplicationReference","Activity in applications other than Drive."],["Assignment","A comment with an assignment."],["Comment","A change about comments on an object."],["ConsolidationStrategy","How the individual activities are consolidated. A set of activities may be consolidated into one combined activity if they are related in some way, such as one actor performing the same action on multiple targets, or multiple actors performing the same action on a single target. The strategy defines the rules for which activities are related."],["Copy","An object was created by copying an existing object."],["Create","An object was created."],["DataLeakPreventionChange","A change in the object's data leak prevention status."],["Delete","An object was deleted."],["DeletedUser","A user whose account has since been deleted."],["Domain","Information about a domain."],["Drive","Information about a shared drive."],["DriveActivity","A single Drive activity comprising one or more Actions by one or more Actors on one or more Targets. Some Action groupings occur spontaneously, such as moving an item into a shared folder triggering a permission change. Other groupings of related Actions, such as multiple Actors editing one item or moving multiple files into a new folder, are controlled by the selection of a ConsolidationStrategy in the QueryDriveActivityRequest."],["DriveActivityHub","Central instance to access all DriveActivityHub related resource activities"],["DriveFile","A Drive item which is a file."],["DriveFolder","A Drive item which is a folder."],["DriveItem","A Drive item, such as a file or folder."],["DriveItemReference","A lightweight reference to a Drive item, such as a file or folder."],["DriveReference","A lightweight reference to a shared drive."],["Edit","An empty message indicating an object was edited."],["File","This item is deprecated; please see `DriveFile` instead."],["FileComment","A comment on a file."],["Folder","This item is deprecated; please see `DriveFolder` instead."],["Group","Information about a group."],["Impersonation","Information about an impersonation, where an admin acts on behalf of an end user. Information about the acting admin is not currently available."],["KnownUser","A known user."],["Legacy","A strategy which consolidates activities using the grouping rules from the legacy V1 Activity API. Similar actions occurring within a window of time can be grouped across multiple targets (such as moving a set of files at once) or multiple actors (such as several users editing the same item). Grouping rules for this strategy are specific to each type of action."],["Move","An object was moved."],["New","An object was created from scratch."],["NoConsolidation","A strategy which does no consolidation of individual activities."],["Owner","Information about the owner of a Drive item."],["Permission","The permission setting of an object."],["PermissionChange","A change of the permission setting on an item."],["Post","A regular posted comment."],["QueryDriveActivityRequest","The request message for querying Drive activity."],["QueryDriveActivityResponse","Response message for querying Drive activity."],["Rename","An object was renamed."],["Restore","A deleted object was restored."],["RestrictionChange","Information about restriction policy changes to a feature."],["SettingsChange","Information about settings changes."],["Suggestion","A suggestion."],["SystemEvent","Event triggered by system operations instead of end users."],["Target","Information about the target of activity."],["TargetReference","A lightweight reference to the target of activity."],["TeamDrive","This item is deprecated; please see `Drive` instead."],["TeamDriveReference","This item is deprecated; please see `DriveReference` instead."],["TimeRange","Information about time ranges."],["UnknownUser","A user about whom nothing is currently known."],["Upload","An object was uploaded into Drive."],["User","Information about an end user."]]});