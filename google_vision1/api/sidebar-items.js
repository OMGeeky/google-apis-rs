initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["AddProductToProductSetRequest","Request message for the `AddProductToProductSet` method."],["AnnotateFileRequest","A request to annotate one single file, e.g. a PDF, TIFF or GIF file."],["AnnotateFileResponse","Response to a single file annotation request. A file may contain one or more images, which individually have their own responses."],["AnnotateImageRequest","Request for performing Google Cloud Vision API tasks over a user-provided image, with user-requested features, and with context information."],["AnnotateImageResponse","Response to an image annotation request."],["AsyncAnnotateFileRequest","An offline file annotation request."],["AsyncBatchAnnotateFilesRequest","Multiple async file annotation requests are batched into a single service call."],["AsyncBatchAnnotateImagesRequest","Request for async image annotation for a list of images."],["BatchAnnotateFilesRequest","A list of requests to annotate files using the BatchAnnotateFiles API."],["BatchAnnotateFilesResponse","A list of file annotation responses."],["BatchAnnotateImagesRequest","Multiple image annotation requests are batched into a single service call."],["BatchAnnotateImagesResponse","Response to a batch image annotation request."],["Block","Logical element on the page."],["BoundingPoly","A bounding polygon for the detected image annotation."],["CancelOperationRequest","The request message for Operations.CancelOperation."],["Color","Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness. For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript. This reference page doesn't carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB color space. When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor* fromProto(Color* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color* toProto(UIColor* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac * 255); var green = Math.floor(greenFrac * 255); var blue = Math.floor(blueFrac * 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ..."],["ColorInfo","Color information consists of RGB channels, score, and the fraction of the image that the color occupies in the image."],["CropHint","Single crop hint that is used to generate a new crop when serving an image."],["CropHintsAnnotation","Set of crop hints that are used to generate new crops when serving images."],["CropHintsParams","Parameters for crop hints annotation request."],["DetectedBreak","Detected start or end of a structural component."],["DetectedLanguage","Detected language for a structural component."],["DominantColorsAnnotation","Set of dominant colors and their corresponding scores."],["Empty","A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."],["EntityAnnotation","Set of detected entity features."],["FaceAnnotation","A face annotation object contains the results of face detection."],["Feature","The type of Google Cloud Vision API detection to perform, and the maximum number of results to return for that type. Multiple `Feature` objects can be specified in the `features` list."],["FileAnnotateCall","Service that performs image detection and annotation for a batch of files. Now only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported. This service will extract at most 5 (customers can specify which 5 in AnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each file provided and perform detection and annotation for each image extracted."],["FileAsyncBatchAnnotateCall","Run asynchronous image detection and annotation for a list of generic files, such as PDF files, which may contain multiple pages and multiple images per page. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results)."],["FileMethods","A builder providing access to all methods supported on file resources. It is not used directly, but through the `Vision` hub."],["GcsDestination","The Google Cloud Storage location where the output will be written to."],["GcsSource","The Google Cloud Storage location where the input will be read from."],["GroupedResult","Information about the products similar to a single product in a query image."],["Image","Client image to perform Google Cloud Vision API tasks over."],["ImageAnnotateCall","Run image detection and annotation for a batch of images."],["ImageAnnotationContext","If an image was produced from a file (e.g. a PDF), this message gives information about the source of that image."],["ImageAsyncBatchAnnotateCall","Run asynchronous image detection and annotation for a list of images. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results). This service will write image annotation outputs to json files in customer GCS bucket, each json file containing BatchAnnotateImagesResponse proto."],["ImageContext","Image context and/or feature-specific parameters."],["ImageMethods","A builder providing access to all methods supported on image resources. It is not used directly, but through the `Vision` hub."],["ImageProperties","Stores image properties, such as dominant colors."],["ImageSource","External image source (Google Cloud Storage or web URL image location)."],["ImportProductSetsGcsSource","The Google Cloud Storage location for a csv file which preserves a list of ImportProductSetRequests in each line."],["ImportProductSetsInputConfig","The input content for the `ImportProductSets` method."],["ImportProductSetsRequest","Request message for the `ImportProductSets` method."],["InputConfig","The desired input location and metadata."],["KeyValue","A product label represented as a key-value pair."],["Landmark","A face-specific landmark (for example, a face feature)."],["LatLng","An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges."],["LatLongRect","Rectangle determined by min and max `LatLng` pairs."],["ListOperationsResponse","The response message for Operations.ListOperations."],["ListProductSetsResponse","Response message for the `ListProductSets` method."],["ListProductsInProductSetResponse","Response message for the `ListProductsInProductSet` method."],["ListProductsResponse","Response message for the `ListProducts` method."],["ListReferenceImagesResponse","Response message for the `ListReferenceImages` method."],["LocalizedObjectAnnotation","Set of detected objects with bounding boxes."],["LocationInfo","Detected entity location information."],["LocationMethods","A builder providing access to all methods supported on location resources. It is not used directly, but through the `Vision` hub."],["LocationOperationGetCall","Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."],["NormalizedVertex","A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."],["ObjectAnnotation","Prediction for what the object in the bounding box is."],["Operation","This resource represents a long-running operation that is the result of a network API call."],["OperationCancelCall","Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."],["OperationDeleteCall","Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."],["OperationGetCall","Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."],["OperationListCall","Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."],["OperationMethods","A builder providing access to all methods supported on operation resources. It is not used directly, but through the `Vision` hub."],["OutputConfig","The desired output location and metadata."],["Page","Detected page from OCR."],["Paragraph","Structural unit of text representing a number of words in certain order."],["Position","A 3D position in the image, used primarily for Face detection landmarks. A valid Position must have both x and y coordinates. The position coordinates are in the same scale as the original image."],["Product","A Product contains ReferenceImages."],["ProductSearchParams","Parameters for a product search request."],["ProductSearchResults","Results for a product search request."],["ProductSet","A ProductSet contains Products. A ProductSet can contain a maximum of 1 million reference images. If the limit is exceeded, periodic indexing will fail."],["ProductSetPurgeConfig","Config to control which ProductSet contains the Products to be deleted."],["ProjectFileAnnotateCall","Service that performs image detection and annotation for a batch of files. Now only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported. This service will extract at most 5 (customers can specify which 5 in AnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each file provided and perform detection and annotation for each image extracted."],["ProjectFileAsyncBatchAnnotateCall","Run asynchronous image detection and annotation for a list of generic files, such as PDF files, which may contain multiple pages and multiple images per page. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results)."],["ProjectImageAnnotateCall","Run image detection and annotation for a batch of images."],["ProjectImageAsyncBatchAnnotateCall","Run asynchronous image detection and annotation for a list of images. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results). This service will write image annotation outputs to json files in customer GCS bucket, each json file containing BatchAnnotateImagesResponse proto."],["ProjectLocationFileAnnotateCall","Service that performs image detection and annotation for a batch of files. Now only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported. This service will extract at most 5 (customers can specify which 5 in AnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each file provided and perform detection and annotation for each image extracted."],["ProjectLocationFileAsyncBatchAnnotateCall","Run asynchronous image detection and annotation for a list of generic files, such as PDF files, which may contain multiple pages and multiple images per page. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results)."],["ProjectLocationImageAnnotateCall","Run image detection and annotation for a batch of images."],["ProjectLocationImageAsyncBatchAnnotateCall","Run asynchronous image detection and annotation for a list of images. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results). This service will write image annotation outputs to json files in customer GCS bucket, each json file containing BatchAnnotateImagesResponse proto."],["ProjectLocationOperationGetCall","Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."],["ProjectLocationProductCreateCall","Creates and returns a new product resource. Possible errors: * Returns INVALID_ARGUMENT if display_name is missing or longer than 4096 characters. * Returns INVALID_ARGUMENT if description is longer than 4096 characters. * Returns INVALID_ARGUMENT if product_category is missing or invalid."],["ProjectLocationProductDeleteCall","Permanently deletes a product and its reference images. Metadata of the product and all its images will be deleted right away, but search queries against ProductSets containing the product may still work until all related caches are refreshed."],["ProjectLocationProductGetCall","Gets information associated with a Product. Possible errors: * Returns NOT_FOUND if the Product does not exist."],["ProjectLocationProductListCall","Lists products in an unspecified order. Possible errors: * Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1."],["ProjectLocationProductPatchCall","Makes changes to a Product resource. Only the `display_name`, `description`, and `labels` fields can be updated right now. If labels are updated, the change will not be reflected in queries until the next index time. Possible errors: * Returns NOT_FOUND if the Product does not exist. * Returns INVALID_ARGUMENT if display_name is present in update_mask but is missing from the request or longer than 4096 characters. * Returns INVALID_ARGUMENT if description is present in update_mask but is longer than 4096 characters. * Returns INVALID_ARGUMENT if product_category is present in update_mask."],["ProjectLocationProductPurgeCall","Asynchronous API to delete all Products in a ProductSet or all Products that are in no ProductSet. If a Product is a member of the specified ProductSet in addition to other ProductSets, the Product will still be deleted. It is recommended to not delete the specified ProductSet until after this operation has completed. It is also recommended to not add any of the Products involved in the batch delete to a new ProductSet while this operation is running because those Products may still end up deleted. It's not possible to undo the PurgeProducts operation. Therefore, it is recommended to keep the csv files used in ImportProductSets (if that was how you originally built the Product Set) before starting PurgeProducts, in case you need to re-import the data after deletion. If the plan is to purge all of the Products from a ProductSet and then re-use the empty ProductSet to re-import new Products into the empty ProductSet, you must wait until the PurgeProducts operation has finished for that ProductSet. The google.longrunning.Operation API can be used to keep track of the progress and results of the request. `Operation.metadata` contains `BatchOperationMetadata`. (progress)"],["ProjectLocationProductReferenceImageCreateCall","Creates and returns a new ReferenceImage resource. The `bounding_poly` field is optional. If `bounding_poly` is not specified, the system will try to detect regions of interest in the image that are compatible with the product_category on the parent product. If it is specified, detection is ALWAYS skipped. The system converts polygons into non-rotated rectangles. Note that the pipeline will resize the image if the image resolution is too large to process (above 50MP). Possible errors: * Returns INVALID_ARGUMENT if the image_uri is missing or longer than 4096 characters. * Returns INVALID_ARGUMENT if the product does not exist. * Returns INVALID_ARGUMENT if bounding_poly is not provided, and nothing compatible with the parent product's product_category is detected. * Returns INVALID_ARGUMENT if bounding_poly contains more than 10 polygons."],["ProjectLocationProductReferenceImageDeleteCall","Permanently deletes a reference image. The image metadata will be deleted right away, but search queries against ProductSets containing the image may still work until all related caches are refreshed. The actual image files are not deleted from Google Cloud Storage."],["ProjectLocationProductReferenceImageGetCall","Gets information associated with a ReferenceImage. Possible errors: * Returns NOT_FOUND if the specified image does not exist."],["ProjectLocationProductReferenceImageListCall","Lists reference images. Possible errors: * Returns NOT_FOUND if the parent product does not exist. * Returns INVALID_ARGUMENT if the page_size is greater than 100, or less than 1."],["ProjectLocationProductSetAddProductCall","Adds a Product to the specified ProductSet. If the Product is already present, no change is made. One Product can be added to at most 100 ProductSets. Possible errors: * Returns NOT_FOUND if the Product or the ProductSet doesn't exist."],["ProjectLocationProductSetCreateCall","Creates and returns a new ProductSet resource. Possible errors: * Returns INVALID_ARGUMENT if display_name is missing, or is longer than 4096 characters."],["ProjectLocationProductSetDeleteCall","Permanently deletes a ProductSet. Products and ReferenceImages in the ProductSet are not deleted. The actual image files are not deleted from Google Cloud Storage."],["ProjectLocationProductSetGetCall","Gets information associated with a ProductSet. Possible errors: * Returns NOT_FOUND if the ProductSet does not exist."],["ProjectLocationProductSetImportCall","Asynchronous API that imports a list of reference images to specified product sets based on a list of image information. The google.longrunning.Operation API can be used to keep track of the progress and results of the request. `Operation.metadata` contains `BatchOperationMetadata`. (progress) `Operation.response` contains `ImportProductSetsResponse`. (results) The input source of this method is a csv file on Google Cloud Storage. For the format of the csv file please see ImportProductSetsGcsSource.csv_file_uri."],["ProjectLocationProductSetListCall","Lists ProductSets in an unspecified order. Possible errors: * Returns INVALID_ARGUMENT if page_size is greater than 100, or less than 1."],["ProjectLocationProductSetPatchCall","Makes changes to a ProductSet resource. Only display_name can be updated currently. Possible errors: * Returns NOT_FOUND if the ProductSet does not exist. * Returns INVALID_ARGUMENT if display_name is present in update_mask but missing from the request or longer than 4096 characters."],["ProjectLocationProductSetProductListCall","Lists the Products in a ProductSet, in an unspecified order. If the ProductSet does not exist, the products field of the response will be empty. Possible errors: * Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1."],["ProjectLocationProductSetRemoveProductCall","Removes a Product from the specified ProductSet."],["ProjectMethods","A builder providing access to all methods supported on project resources. It is not used directly, but through the `Vision` hub."],["ProjectOperationGetCall","Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."],["Property","A `Property` consists of a user-supplied name/value pair."],["PurgeProductsRequest","Request message for the `PurgeProducts` method."],["ReferenceImage","A `ReferenceImage` represents a product image and its associated metadata, such as bounding boxes."],["RemoveProductFromProductSetRequest","Request message for the `RemoveProductFromProductSet` method."],["Result","Information about a product."],["SafeSearchAnnotation","Set of features pertaining to the image, computed by computer vision methods over safe-search verticals (for example, adult, spoof, medical, violence)."],["Status","The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC. Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the API Design Guide."],["Symbol","A single symbol representation."],["TextAnnotation","TextAnnotation contains a structured representation of OCR extracted text. The hierarchy of an OCR extracted text structure is like this: TextAnnotation -> Page -> Block -> Paragraph -> Word -> Symbol Each structural component, starting from Page, may further have their own properties. Properties describe detected languages, breaks etc.. Please refer to the TextAnnotation.TextProperty message definition below for more detail."],["TextDetectionParams","Parameters for text detections. This is used to control TEXT_DETECTION and DOCUMENT_TEXT_DETECTION features."],["TextProperty","Additional information detected on the structural component."],["Vertex","A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image."],["Vision","Central instance to access all Vision related resource activities"],["WebDetection","Relevant information for the image from the Internet."],["WebDetectionParams","Parameters for web detection request."],["WebEntity","Entity deduced from similar images on the Internet."],["WebImage","Metadata for online images."],["WebLabel","Label to provide extra metadata for the web detection."],["WebPage","Metadata for web pages."],["Word","A word representation."]]});