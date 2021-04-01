initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["AnalyticsInfo","Tracking parameters supported by Dynamic Link."],["AndroidInfo","Android related attributes to the Dynamic Link."],["CreateManagedShortLinkRequest","Request to create a managed Short Dynamic Link."],["CreateManagedShortLinkResponse","Response to create a short Dynamic Link."],["CreateShortDynamicLinkRequest","Request to create a short Dynamic Link."],["CreateShortDynamicLinkResponse","Response to create a short Dynamic Link."],["DesktopInfo","Desktop related attributes to the Dynamic Link."],["DeviceInfo","Signals associated with the device making the request."],["DynamicLinkEventStat","Dynamic Link event stat."],["DynamicLinkInfo","Information about a Dynamic Link."],["DynamicLinkStats","Analytics stats of a Dynamic Link for a given timeframe."],["DynamicLinkWarning","Dynamic Links warning messages."],["FirebaseDynamicLinks","Central instance to access all FirebaseDynamicLinks related resource activities"],["GetIosPostInstallAttributionRequest","Request for iSDK to execute strong match flow for post-install attribution. This is meant for iOS requests only. Requests from other platforms will not be honored."],["GetIosPostInstallAttributionResponse","Response for iSDK to execute strong match flow for post-install attribution."],["GetIosReopenAttributionRequest","Request for iSDK to get reopen attribution for app universal link open deeplinking. This endpoint is meant for only iOS requests."],["GetIosReopenAttributionResponse","Response for iSDK to get reopen attribution for app universal link open deeplinking. This endpoint is meant for only iOS requests."],["GooglePlayAnalytics","Parameters for Google Play Campaign Measurements. Learn more"],["ITunesConnectAnalytics","Parameters for iTunes Connect App Analytics."],["IosInfo","iOS related attributes to the Dynamic Link.."],["ManagedShortLink","Managed Short Link."],["ManagedShortLinkCreateCall","Creates a managed short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. This differs from CreateShortDynamicLink in the following ways: - The request will also contain a name for the link (non unique name for the front end). - The response must be authenticated with an auth token (generated with the admin service account). - The link will appear in the FDL list of links in the console front end. The Dynamic Link domain in the request must be owned by requester's Firebase project."],["ManagedShortLinkMethods","A builder providing access to all methods supported on managedShortLink resources. It is not used directly, but through the `FirebaseDynamicLinks` hub."],["MethodGetLinkStatCall","Fetches analytics stats of a short Dynamic Link for a given duration. Metrics include number of clicks, redirects, installs, app first opens, and app reopens."],["MethodInstallAttributionCall","Get iOS strong/weak-match info for post-install attribution."],["MethodMethods","A builder providing access to all free methods, which are not associated with a particular resource. It is not used directly, but through the `FirebaseDynamicLinks` hub."],["MethodReopenAttributionCall","Get iOS reopen attribution for app universal link open deeplinking."],["NavigationInfo","Information of navigation behavior."],["ShortLinkCreateCall","Creates a short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. Repeated calls with the same long Dynamic Link or Dynamic Link information will produce the same short Dynamic Link. The Dynamic Link domain in the request must be owned by requester's Firebase project."],["ShortLinkMethods","A builder providing access to all methods supported on shortLink resources. It is not used directly, but through the `FirebaseDynamicLinks` hub."],["SocialMetaTagInfo","Parameters for social meta tag params. Used to set meta tag data for link previews on social sites."],["Suffix","Short Dynamic Link suffix."]]});