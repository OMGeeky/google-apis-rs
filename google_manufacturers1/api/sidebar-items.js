initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["AccountMethods","A builder providing access to all methods supported on account resources. It is not used directly, but through the `ManufacturerCenter` hub."],["AccountProductDeleteCall","Deletes the product from a Manufacturer Center account."],["AccountProductGetCall","Gets the product from a Manufacturer Center account, including product issues. A recently updated product takes around 15 minutes to process. Changes are only visible after it has been processed. While some issues may be available once the product has been processed, other issues may take days to appear."],["AccountProductListCall","Lists all the products in a Manufacturer Center account."],["AccountProductUpdateCall","Inserts or updates the attributes of the product in a Manufacturer Center account. Creates a product with the provided attributes. If the product already exists, then all attributes are replaced with the new ones. The checks at upload time are minimal. All required attributes need to be present for a product to be valid. Issues may show up later after the API has accepted a new upload for a product and it is possible to overwrite an existing valid product with an invalid product. To detect this, you should retrieve the product and check it for issues once the new version is available. Uploaded attributes first need to be processed before they can be retrieved. Until then, new products will be unavailable, and retrieval of previously uploaded products will return the original state of the product."],["Attributes","Attributes of the product. For more information, see https://support.google.com/manufacturers/answer/6124116."],["Capacity","The capacity of a product. For more information, see https://support.google.com/manufacturers/answer/6124116#capacity."],["Count","The number of products in a single package. For more information, see https://support.google.com/manufacturers/answer/6124116#count."],["DestinationStatus","The destination status."],["Empty","A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."],["FeatureDescription","A feature description of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#featuredesc."],["Image","An image."],["Issue","Product issue."],["ListProductsResponse","There is no detailed description."],["ManufacturerCenter","Central instance to access all ManufacturerCenter related resource activities"],["Price","A price."],["Product","Product data."],["ProductDetail","A product detail of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productdetail."]]});