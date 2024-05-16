use super::*;



// region GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of the layout element that is being referenced if any.
pub enum GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum {
    

    /// Layout Unspecified.
    ///
    /// "LAYOUT_TYPE_UNSPECIFIED"
    #[serde(rename="LAYOUT_TYPE_UNSPECIFIED")]
    LAYOUTTYPEUNSPECIFIED,
    

    /// References a Page.blocks element.
    ///
    /// "BLOCK"
    #[serde(rename="BLOCK")]
    BLOCK,
    

    /// References a Page.paragraphs element.
    ///
    /// "PARAGRAPH"
    #[serde(rename="PARAGRAPH")]
    PARAGRAPH,
    

    /// References a Page.lines element.
    ///
    /// "LINE"
    #[serde(rename="LINE")]
    LINE,
    

    /// References a Page.tokens element.
    ///
    /// "TOKEN"
    #[serde(rename="TOKEN")]
    TOKEN,
    

    /// References a Page.visual_elements element.
    ///
    /// "VISUAL_ELEMENT"
    #[serde(rename="VISUAL_ELEMENT")]
    VISUALELEMENT,
    

    /// Refrrences a Page.tables element.
    ///
    /// "TABLE"
    #[serde(rename="TABLE")]
    TABLE,
    

    /// References a Page.form_fields element.
    ///
    /// "FORM_FIELD"
    #[serde(rename="FORM_FIELD")]
    FORMFIELD,
}

impl AsRef<str> for GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::LAYOUTTYPEUNSPECIFIED => "LAYOUT_TYPE_UNSPECIFIED",
            GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::BLOCK => "BLOCK",
            GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::PARAGRAPH => "PARAGRAPH",
            GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::LINE => "LINE",
            GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::TOKEN => "TOKEN",
            GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::VISUALELEMENT => "VISUAL_ELEMENT",
            GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::TABLE => "TABLE",
            GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::FORMFIELD => "FORM_FIELD",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAYOUT_TYPE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::LAYOUTTYPEUNSPECIFIED),
           "BLOCK" => Ok(GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::BLOCK),
           "PARAGRAPH" => Ok(GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::PARAGRAPH),
           "LINE" => Ok(GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::LINE),
           "TOKEN" => Ok(GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::TOKEN),
           "VISUAL_ELEMENT" => Ok(GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::VISUALELEMENT),
           "TABLE" => Ok(GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::TABLE),
           "FORM_FIELD" => Ok(GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum::FORMFIELD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Detected orientation for the Layout.
pub enum GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum {
    

    /// Unspecified orientation.
    ///
    /// "ORIENTATION_UNSPECIFIED"
    #[serde(rename="ORIENTATION_UNSPECIFIED")]
    ORIENTATIONUNSPECIFIED,
    

    /// Orientation is aligned with page up.
    ///
    /// "PAGE_UP"
    #[serde(rename="PAGE_UP")]
    PAGEUP,
    

    /// Orientation is aligned with page right. Turn the head 90 degrees clockwise from upright to read.
    ///
    /// "PAGE_RIGHT"
    #[serde(rename="PAGE_RIGHT")]
    PAGERIGHT,
    

    /// Orientation is aligned with page down. Turn the head 180 degrees from upright to read.
    ///
    /// "PAGE_DOWN"
    #[serde(rename="PAGE_DOWN")]
    PAGEDOWN,
    

    /// Orientation is aligned with page left. Turn the head 90 degrees counterclockwise from upright to read.
    ///
    /// "PAGE_LEFT"
    #[serde(rename="PAGE_LEFT")]
    PAGELEFT,
}

impl AsRef<str> for GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum::ORIENTATIONUNSPECIFIED => "ORIENTATION_UNSPECIFIED",
            GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum::PAGEUP => "PAGE_UP",
            GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum::PAGERIGHT => "PAGE_RIGHT",
            GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum::PAGEDOWN => "PAGE_DOWN",
            GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum::PAGELEFT => "PAGE_LEFT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORIENTATION_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum::ORIENTATIONUNSPECIFIED),
           "PAGE_UP" => Ok(GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum::PAGEUP),
           "PAGE_RIGHT" => Ok(GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum::PAGERIGHT),
           "PAGE_DOWN" => Ok(GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum::PAGEDOWN),
           "PAGE_LEFT" => Ok(GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum::PAGELEFT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Detected break type.
pub enum GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum {
    

    /// Unspecified break type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// A single whitespace.
    ///
    /// "SPACE"
    #[serde(rename="SPACE")]
    SPACE,
    

    /// A wider whitespace.
    ///
    /// "WIDE_SPACE"
    #[serde(rename="WIDE_SPACE")]
    WIDESPACE,
    

    /// A hyphen that indicates that a token has been split across lines.
    ///
    /// "HYPHEN"
    #[serde(rename="HYPHEN")]
    HYPHEN,
}

impl AsRef<str> for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum::SPACE => "SPACE",
            GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum::WIDESPACE => "WIDE_SPACE",
            GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum::HYPHEN => "HYPHEN",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum::TYPEUNSPECIFIED),
           "SPACE" => Ok(GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum::SPACE),
           "WIDE_SPACE" => Ok(GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum::WIDESPACE),
           "HYPHEN" => Ok(GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum::HYPHEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of provenance operation.
pub enum GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum {
    

    /// Operation type unspecified. If no operation is specified a provenance entry is simply used to match against a `parent`.
    ///
    /// "OPERATION_TYPE_UNSPECIFIED"
    #[serde(rename="OPERATION_TYPE_UNSPECIFIED")]
    OPERATIONTYPEUNSPECIFIED,
    

    /// Add an element.
    ///
    /// "ADD"
    #[serde(rename="ADD")]
    ADD,
    

    /// Remove an element identified by `parent`.
    ///
    /// "REMOVE"
    #[serde(rename="REMOVE")]
    REMOVE,
    

    /// Updates any fields within the given provenance scope of the message. It overwrites the fields rather than replacing them. Use this when you want to update a field value of an entity without also updating all the child properties.
    ///
    /// "UPDATE"
    #[serde(rename="UPDATE")]
    UPDATE,
    

    /// Currently unused. Replace an element identified by `parent`.
    ///
    /// "REPLACE"
    #[serde(rename="REPLACE")]
    REPLACE,
    

    /// Deprecated. Request human review for the element identified by `parent`.
    ///
    /// "EVAL_REQUESTED"
    #[serde(rename="EVAL_REQUESTED")]
    EVALREQUESTED,
    

    /// Deprecated. Element is reviewed and approved at human review, confidence will be set to 1.0.
    ///
    /// "EVAL_APPROVED"
    #[serde(rename="EVAL_APPROVED")]
    EVALAPPROVED,
    

    /// Deprecated. Element is skipped in the validation process.
    ///
    /// "EVAL_SKIPPED"
    #[serde(rename="EVAL_SKIPPED")]
    EVALSKIPPED,
}

impl AsRef<str> for GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::OPERATIONTYPEUNSPECIFIED => "OPERATION_TYPE_UNSPECIFIED",
            GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::ADD => "ADD",
            GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::REMOVE => "REMOVE",
            GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::UPDATE => "UPDATE",
            GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::REPLACE => "REPLACE",
            GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::EVALREQUESTED => "EVAL_REQUESTED",
            GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::EVALAPPROVED => "EVAL_APPROVED",
            GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::EVALSKIPPED => "EVAL_SKIPPED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::OPERATIONTYPEUNSPECIFIED),
           "ADD" => Ok(GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::ADD),
           "REMOVE" => Ok(GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::REMOVE),
           "UPDATE" => Ok(GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::UPDATE),
           "REPLACE" => Ok(GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::REPLACE),
           "EVAL_REQUESTED" => Ok(GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::EVALREQUESTED),
           "EVAL_APPROVED" => Ok(GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::EVALAPPROVED),
           "EVAL_SKIPPED" => Ok(GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum::EVALSKIPPED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Occurrence type limits the number of instances an entity type appears in the document.
pub enum GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum {
    

    /// Unspecified occurrence type.
    ///
    /// "OCCURRENCE_TYPE_UNSPECIFIED"
    #[serde(rename="OCCURRENCE_TYPE_UNSPECIFIED")]
    OCCURRENCETYPEUNSPECIFIED,
    

    /// There will be zero or one instance of this entity type. The same entity instance may be mentioned multiple times.
    ///
    /// "OPTIONAL_ONCE"
    #[serde(rename="OPTIONAL_ONCE")]
    OPTIONALONCE,
    

    /// The entity type will appear zero or multiple times.
    ///
    /// "OPTIONAL_MULTIPLE"
    #[serde(rename="OPTIONAL_MULTIPLE")]
    OPTIONALMULTIPLE,
    

    /// The entity type will only appear exactly once. The same entity instance may be mentioned multiple times.
    ///
    /// "REQUIRED_ONCE"
    #[serde(rename="REQUIRED_ONCE")]
    REQUIREDONCE,
    

    /// The entity type will appear once or more times.
    ///
    /// "REQUIRED_MULTIPLE"
    #[serde(rename="REQUIRED_MULTIPLE")]
    REQUIREDMULTIPLE,
}

impl AsRef<str> for GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum::OCCURRENCETYPEUNSPECIFIED => "OCCURRENCE_TYPE_UNSPECIFIED",
            GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum::OPTIONALONCE => "OPTIONAL_ONCE",
            GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum::OPTIONALMULTIPLE => "OPTIONAL_MULTIPLE",
            GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum::REQUIREDONCE => "REQUIRED_ONCE",
            GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum::REQUIREDMULTIPLE => "REQUIRED_MULTIPLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OCCURRENCE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum::OCCURRENCETYPEUNSPECIFIED),
           "OPTIONAL_ONCE" => Ok(GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum::OPTIONALONCE),
           "OPTIONAL_MULTIPLE" => Ok(GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum::OPTIONALMULTIPLE),
           "REQUIRED_ONCE" => Ok(GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum::REQUIREDONCE),
           "REQUIRED_MULTIPLE" => Ok(GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum::REQUIREDMULTIPLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetricMetricsTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The metrics type for the label.
pub enum GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetricMetricsTypeEnum {
    

    /// The metrics type is unspecified. By default, metrics without a particular specification are for leaf entity types (i.e., top-level entity types without child types, or child types which are not parent types themselves).
    ///
    /// "METRICS_TYPE_UNSPECIFIED"
    #[serde(rename="METRICS_TYPE_UNSPECIFIED")]
    METRICSTYPEUNSPECIFIED,
    

    /// Indicates whether metrics for this particular label type represent an aggregate of metrics for other types instead of being based on actual TP/FP/FN values for the label type. Metrics for parent (i.e., non-leaf) entity types are an aggregate of metrics for their children.
    ///
    /// "AGGREGATE"
    #[serde(rename="AGGREGATE")]
    AGGREGATE,
}

impl AsRef<str> for GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetricMetricsTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetricMetricsTypeEnum::METRICSTYPEUNSPECIFIED => "METRICS_TYPE_UNSPECIFIED",
            GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetricMetricsTypeEnum::AGGREGATE => "AGGREGATE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetricMetricsTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRICS_TYPE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetricMetricsTypeEnum::METRICSTYPEUNSPECIFIED),
           "AGGREGATE" => Ok(GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetricMetricsTypeEnum::AGGREGATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetricMetricsTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1HumanReviewStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of human review on the processing request.
pub enum GoogleCloudDocumentaiV1HumanReviewStatusStateEnum {
    

    /// Human review state is unspecified. Most likely due to an internal error.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Human review is skipped for the document. This can happen because human review isn't enabled on the processor or the processing request has been set to skip this document.
    ///
    /// "SKIPPED"
    #[serde(rename="SKIPPED")]
    SKIPPED,
    

    /// Human review validation is triggered and passed, so no review is needed.
    ///
    /// "VALIDATION_PASSED"
    #[serde(rename="VALIDATION_PASSED")]
    VALIDATIONPASSED,
    

    /// Human review validation is triggered and the document is under review.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// Some error happened during triggering human review, see the state_message for details.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for GoogleCloudDocumentaiV1HumanReviewStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1HumanReviewStatusStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDocumentaiV1HumanReviewStatusStateEnum::SKIPPED => "SKIPPED",
            GoogleCloudDocumentaiV1HumanReviewStatusStateEnum::VALIDATIONPASSED => "VALIDATION_PASSED",
            GoogleCloudDocumentaiV1HumanReviewStatusStateEnum::INPROGRESS => "IN_PROGRESS",
            GoogleCloudDocumentaiV1HumanReviewStatusStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1HumanReviewStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1HumanReviewStatusStateEnum::STATEUNSPECIFIED),
           "SKIPPED" => Ok(GoogleCloudDocumentaiV1HumanReviewStatusStateEnum::SKIPPED),
           "VALIDATION_PASSED" => Ok(GoogleCloudDocumentaiV1HumanReviewStatusStateEnum::VALIDATIONPASSED),
           "IN_PROGRESS" => Ok(GoogleCloudDocumentaiV1HumanReviewStatusStateEnum::INPROGRESS),
           "ERROR" => Ok(GoogleCloudDocumentaiV1HumanReviewStatusStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1HumanReviewStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1ProcessorStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the processor.
pub enum GoogleCloudDocumentaiV1ProcessorStateEnum {
    

    /// The processor is in an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The processor is enabled, i.e., has an enabled version which can currently serve processing requests and all the feature dependencies have been successfully initialized.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// The processor is disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// The processor is being enabled, will become `ENABLED` if successful.
    ///
    /// "ENABLING"
    #[serde(rename="ENABLING")]
    ENABLING,
    

    /// The processor is being disabled, will become `DISABLED` if successful.
    ///
    /// "DISABLING"
    #[serde(rename="DISABLING")]
    DISABLING,
    

    /// The processor is being created, will become either `ENABLED` (for successful creation) or `FAILED` (for failed ones). Once a processor is in this state, it can then be used for document processing, but the feature dependencies of the processor might not be fully created yet.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The processor failed during creation or initialization of feature dependencies. The user should delete the processor and recreate one as all the functionalities of the processor are disabled.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The processor is being deleted, will be removed if successful.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for GoogleCloudDocumentaiV1ProcessorStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1ProcessorStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDocumentaiV1ProcessorStateEnum::ENABLED => "ENABLED",
            GoogleCloudDocumentaiV1ProcessorStateEnum::DISABLED => "DISABLED",
            GoogleCloudDocumentaiV1ProcessorStateEnum::ENABLING => "ENABLING",
            GoogleCloudDocumentaiV1ProcessorStateEnum::DISABLING => "DISABLING",
            GoogleCloudDocumentaiV1ProcessorStateEnum::CREATING => "CREATING",
            GoogleCloudDocumentaiV1ProcessorStateEnum::FAILED => "FAILED",
            GoogleCloudDocumentaiV1ProcessorStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1ProcessorStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1ProcessorStateEnum::STATEUNSPECIFIED),
           "ENABLED" => Ok(GoogleCloudDocumentaiV1ProcessorStateEnum::ENABLED),
           "DISABLED" => Ok(GoogleCloudDocumentaiV1ProcessorStateEnum::DISABLED),
           "ENABLING" => Ok(GoogleCloudDocumentaiV1ProcessorStateEnum::ENABLING),
           "DISABLING" => Ok(GoogleCloudDocumentaiV1ProcessorStateEnum::DISABLING),
           "CREATING" => Ok(GoogleCloudDocumentaiV1ProcessorStateEnum::CREATING),
           "FAILED" => Ok(GoogleCloudDocumentaiV1ProcessorStateEnum::FAILED),
           "DELETING" => Ok(GoogleCloudDocumentaiV1ProcessorStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1ProcessorStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Launch stage of the processor type
pub enum GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum {
    

    /// Do not use this default value.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// The feature is not yet implemented. Users can not use it.
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Prelaunch features are hidden from users and are only visible internally.
    ///
    /// "PRELAUNCH"
    #[serde(rename="PRELAUNCH")]
    PRELAUNCH,
    

    /// Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released.
    ///
    /// "EARLY_ACCESS"
    #[serde(rename="EARLY_ACCESS")]
    EARLYACCESS,
    

    /// Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// GA features are open to all developers and are considered stable and fully qualified for production use.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// Deprecated features are scheduled to be shut down and removed. For more information, see the "Deprecation Policy" section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::PRELAUNCH => "PRELAUNCH",
            GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::EARLYACCESS => "EARLY_ACCESS",
            GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::ALPHA => "ALPHA",
            GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::BETA => "BETA",
            GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::GA => "GA",
            GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "UNIMPLEMENTED" => Ok(GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::UNIMPLEMENTED),
           "PRELAUNCH" => Ok(GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::PRELAUNCH),
           "EARLY_ACCESS" => Ok(GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::EARLYACCESS),
           "ALPHA" => Ok(GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::ALPHA),
           "BETA" => Ok(GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::BETA),
           "GA" => Ok(GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::GA),
           "DEPRECATED" => Ok(GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1ProcessorVersionModelTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The model type of this processor version.
pub enum GoogleCloudDocumentaiV1ProcessorVersionModelTypeEnum {
    

    /// The processor version has unspecified model type.
    ///
    /// "MODEL_TYPE_UNSPECIFIED"
    #[serde(rename="MODEL_TYPE_UNSPECIFIED")]
    MODELTYPEUNSPECIFIED,
    

    /// The processor version has generative model type.
    ///
    /// "MODEL_TYPE_GENERATIVE"
    #[serde(rename="MODEL_TYPE_GENERATIVE")]
    MODELTYPEGENERATIVE,
    

    /// The processor version has custom model type.
    ///
    /// "MODEL_TYPE_CUSTOM"
    #[serde(rename="MODEL_TYPE_CUSTOM")]
    MODELTYPECUSTOM,
}

impl AsRef<str> for GoogleCloudDocumentaiV1ProcessorVersionModelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1ProcessorVersionModelTypeEnum::MODELTYPEUNSPECIFIED => "MODEL_TYPE_UNSPECIFIED",
            GoogleCloudDocumentaiV1ProcessorVersionModelTypeEnum::MODELTYPEGENERATIVE => "MODEL_TYPE_GENERATIVE",
            GoogleCloudDocumentaiV1ProcessorVersionModelTypeEnum::MODELTYPECUSTOM => "MODEL_TYPE_CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1ProcessorVersionModelTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODEL_TYPE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1ProcessorVersionModelTypeEnum::MODELTYPEUNSPECIFIED),
           "MODEL_TYPE_GENERATIVE" => Ok(GoogleCloudDocumentaiV1ProcessorVersionModelTypeEnum::MODELTYPEGENERATIVE),
           "MODEL_TYPE_CUSTOM" => Ok(GoogleCloudDocumentaiV1ProcessorVersionModelTypeEnum::MODELTYPECUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1ProcessorVersionModelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1ProcessorVersionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the processor version.
pub enum GoogleCloudDocumentaiV1ProcessorVersionStateEnum {
    

    /// The processor version is in an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The processor version is deployed and can be used for processing.
    ///
    /// "DEPLOYED"
    #[serde(rename="DEPLOYED")]
    DEPLOYED,
    

    /// The processor version is being deployed.
    ///
    /// "DEPLOYING"
    #[serde(rename="DEPLOYING")]
    DEPLOYING,
    

    /// The processor version is not deployed and cannot be used for processing.
    ///
    /// "UNDEPLOYED"
    #[serde(rename="UNDEPLOYED")]
    UNDEPLOYED,
    

    /// The processor version is being undeployed.
    ///
    /// "UNDEPLOYING"
    #[serde(rename="UNDEPLOYING")]
    UNDEPLOYING,
    

    /// The processor version is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The processor version is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The processor version failed and is in an indeterminate state.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The processor version is being imported.
    ///
    /// "IMPORTING"
    #[serde(rename="IMPORTING")]
    IMPORTING,
}

impl AsRef<str> for GoogleCloudDocumentaiV1ProcessorVersionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1ProcessorVersionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDocumentaiV1ProcessorVersionStateEnum::DEPLOYED => "DEPLOYED",
            GoogleCloudDocumentaiV1ProcessorVersionStateEnum::DEPLOYING => "DEPLOYING",
            GoogleCloudDocumentaiV1ProcessorVersionStateEnum::UNDEPLOYED => "UNDEPLOYED",
            GoogleCloudDocumentaiV1ProcessorVersionStateEnum::UNDEPLOYING => "UNDEPLOYING",
            GoogleCloudDocumentaiV1ProcessorVersionStateEnum::CREATING => "CREATING",
            GoogleCloudDocumentaiV1ProcessorVersionStateEnum::DELETING => "DELETING",
            GoogleCloudDocumentaiV1ProcessorVersionStateEnum::FAILED => "FAILED",
            GoogleCloudDocumentaiV1ProcessorVersionStateEnum::IMPORTING => "IMPORTING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1ProcessorVersionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1ProcessorVersionStateEnum::STATEUNSPECIFIED),
           "DEPLOYED" => Ok(GoogleCloudDocumentaiV1ProcessorVersionStateEnum::DEPLOYED),
           "DEPLOYING" => Ok(GoogleCloudDocumentaiV1ProcessorVersionStateEnum::DEPLOYING),
           "UNDEPLOYED" => Ok(GoogleCloudDocumentaiV1ProcessorVersionStateEnum::UNDEPLOYED),
           "UNDEPLOYING" => Ok(GoogleCloudDocumentaiV1ProcessorVersionStateEnum::UNDEPLOYING),
           "CREATING" => Ok(GoogleCloudDocumentaiV1ProcessorVersionStateEnum::CREATING),
           "DELETING" => Ok(GoogleCloudDocumentaiV1ProcessorVersionStateEnum::DELETING),
           "FAILED" => Ok(GoogleCloudDocumentaiV1ProcessorVersionStateEnum::FAILED),
           "IMPORTING" => Ok(GoogleCloudDocumentaiV1ProcessorVersionStateEnum::IMPORTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1ProcessorVersionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1ReviewDocumentRequestPriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The priority of the human review task.
pub enum GoogleCloudDocumentaiV1ReviewDocumentRequestPriorityEnum {
    

    /// The default priority level.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// The urgent priority level. The labeling manager should allocate labeler resource to the urgent task queue to respect this priority level.
    ///
    /// "URGENT"
    #[serde(rename="URGENT")]
    URGENT,
}

impl AsRef<str> for GoogleCloudDocumentaiV1ReviewDocumentRequestPriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1ReviewDocumentRequestPriorityEnum::DEFAULT => "DEFAULT",
            GoogleCloudDocumentaiV1ReviewDocumentRequestPriorityEnum::URGENT => "URGENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1ReviewDocumentRequestPriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(GoogleCloudDocumentaiV1ReviewDocumentRequestPriorityEnum::DEFAULT),
           "URGENT" => Ok(GoogleCloudDocumentaiV1ReviewDocumentRequestPriorityEnum::URGENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1ReviewDocumentRequestPriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDocumentaiV1TrainProcessorVersionRequestCustomDocumentExtractionOptionTrainingMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Training method to use for CDE training.
pub enum GoogleCloudDocumentaiV1TrainProcessorVersionRequestCustomDocumentExtractionOptionTrainingMethodEnum {
    
    /// "TRAINING_METHOD_UNSPECIFIED"
    #[serde(rename="TRAINING_METHOD_UNSPECIFIED")]
    TRAININGMETHODUNSPECIFIED,
    
    /// "MODEL_BASED"
    #[serde(rename="MODEL_BASED")]
    MODELBASED,
    
    /// "TEMPLATE_BASED"
    #[serde(rename="TEMPLATE_BASED")]
    TEMPLATEBASED,
}

impl AsRef<str> for GoogleCloudDocumentaiV1TrainProcessorVersionRequestCustomDocumentExtractionOptionTrainingMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDocumentaiV1TrainProcessorVersionRequestCustomDocumentExtractionOptionTrainingMethodEnum::TRAININGMETHODUNSPECIFIED => "TRAINING_METHOD_UNSPECIFIED",
            GoogleCloudDocumentaiV1TrainProcessorVersionRequestCustomDocumentExtractionOptionTrainingMethodEnum::MODELBASED => "MODEL_BASED",
            GoogleCloudDocumentaiV1TrainProcessorVersionRequestCustomDocumentExtractionOptionTrainingMethodEnum::TEMPLATEBASED => "TEMPLATE_BASED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDocumentaiV1TrainProcessorVersionRequestCustomDocumentExtractionOptionTrainingMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRAINING_METHOD_UNSPECIFIED" => Ok(GoogleCloudDocumentaiV1TrainProcessorVersionRequestCustomDocumentExtractionOptionTrainingMethodEnum::TRAININGMETHODUNSPECIFIED),
           "MODEL_BASED" => Ok(GoogleCloudDocumentaiV1TrainProcessorVersionRequestCustomDocumentExtractionOptionTrainingMethodEnum::MODELBASED),
           "TEMPLATE_BASED" => Ok(GoogleCloudDocumentaiV1TrainProcessorVersionRequestCustomDocumentExtractionOptionTrainingMethodEnum::TEMPLATEBASED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDocumentaiV1TrainProcessorVersionRequestCustomDocumentExtractionOptionTrainingMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


