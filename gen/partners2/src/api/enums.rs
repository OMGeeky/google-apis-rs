use super::*;



// region EventDataKeyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Data type.
pub enum EventDataKeyEnum {
    

    /// Unchosen.
    ///
    /// "EVENT_DATA_TYPE_UNSPECIFIED"
    #[serde(rename="EVENT_DATA_TYPE_UNSPECIFIED")]
    EVENTDATATYPEUNSPECIFIED,
    

    /// Action data.
    ///
    /// "ACTION"
    #[serde(rename="ACTION")]
    ACTION,
    

    /// Agency ID data.
    ///
    /// "AGENCY_ID"
    #[serde(rename="AGENCY_ID")]
    AGENCYID,
    

    /// Agency name data.
    ///
    /// "AGENCY_NAME"
    #[serde(rename="AGENCY_NAME")]
    AGENCYNAME,
    

    /// Agency phone number data.
    ///
    /// "AGENCY_PHONE_NUMBER"
    #[serde(rename="AGENCY_PHONE_NUMBER")]
    AGENCYPHONENUMBER,
    

    /// Agency website data.
    ///
    /// "AGENCY_WEBSITE"
    #[serde(rename="AGENCY_WEBSITE")]
    AGENCYWEBSITE,
    

    /// Budget data.
    ///
    /// "BUDGET"
    #[serde(rename="BUDGET")]
    BUDGET,
    

    /// Center-point data.
    ///
    /// "CENTER_POINT"
    #[serde(rename="CENTER_POINT")]
    CENTERPOINT,
    

    /// Certification data.
    ///
    /// "CERTIFICATION"
    #[serde(rename="CERTIFICATION")]
    CERTIFICATION,
    

    /// Comment data.
    ///
    /// "COMMENT"
    #[serde(rename="COMMENT")]
    COMMENT,
    

    /// Country data.
    ///
    /// "COUNTRY"
    #[serde(rename="COUNTRY")]
    COUNTRY,
    

    /// Currency data.
    ///
    /// "CURRENCY"
    #[serde(rename="CURRENCY")]
    CURRENCY,
    

    /// Currently viewed agency ID data.
    ///
    /// "CURRENTLY_VIEWED_AGENCY_ID"
    #[serde(rename="CURRENTLY_VIEWED_AGENCY_ID")]
    CURRENTLYVIEWEDAGENCYID,
    

    /// Distance data.
    ///
    /// "DISTANCE"
    #[serde(rename="DISTANCE")]
    DISTANCE,
    

    /// Distance type data.
    ///
    /// "DISTANCE_TYPE"
    #[serde(rename="DISTANCE_TYPE")]
    DISTANCETYPE,
    

    /// Exam data.
    ///
    /// "EXAM"
    #[serde(rename="EXAM")]
    EXAM,
    

    /// History token data.
    ///
    /// "HISTORY_TOKEN"
    #[serde(rename="HISTORY_TOKEN")]
    HISTORYTOKEN,
    

    /// Identifier data.
    ///
    /// "ID"
    #[serde(rename="ID")]
    ID,
    

    /// Industry data.
    ///
    /// "INDUSTRY"
    #[serde(rename="INDUSTRY")]
    INDUSTRY,
    

    /// Insight tag data.
    ///
    /// "INSIGHT_TAG"
    #[serde(rename="INSIGHT_TAG")]
    INSIGHTTAG,
    

    /// Language data.
    ///
    /// "LANGUAGE"
    #[serde(rename="LANGUAGE")]
    LANGUAGE,
    

    /// Location  data.
    ///
    /// "LOCATION"
    #[serde(rename="LOCATION")]
    LOCATION,
    

    /// Marketing opt-in data.
    ///
    /// "MARKETING_OPT_IN"
    #[serde(rename="MARKETING_OPT_IN")]
    MARKETINGOPTIN,
    

    /// Query data.
    ///
    /// "QUERY"
    #[serde(rename="QUERY")]
    QUERY,
    

    /// Search start index data.
    ///
    /// "SEARCH_START_INDEX"
    #[serde(rename="SEARCH_START_INDEX")]
    SEARCHSTARTINDEX,
    

    /// Service data.
    ///
    /// "SERVICE"
    #[serde(rename="SERVICE")]
    SERVICE,
    

    /// Show vow data.
    ///
    /// "SHOW_VOW"
    #[serde(rename="SHOW_VOW")]
    SHOWVOW,
    

    /// Solution data.
    ///
    /// "SOLUTION"
    #[serde(rename="SOLUTION")]
    SOLUTION,
    

    /// Traffic source ID data.
    ///
    /// "TRAFFIC_SOURCE_ID"
    #[serde(rename="TRAFFIC_SOURCE_ID")]
    TRAFFICSOURCEID,
    

    /// Traffic sub ID data.
    ///
    /// "TRAFFIC_SUB_ID"
    #[serde(rename="TRAFFIC_SUB_ID")]
    TRAFFICSUBID,
    

    /// Viewport data.
    ///
    /// "VIEW_PORT"
    #[serde(rename="VIEW_PORT")]
    VIEWPORT,
    

    /// Website data.
    ///
    /// "WEBSITE"
    #[serde(rename="WEBSITE")]
    WEBSITE,
    

    /// Details data.
    ///
    /// "DETAILS"
    #[serde(rename="DETAILS")]
    DETAILS,
    

    /// Experiment ID data.
    ///
    /// "EXPERIMENT_ID"
    #[serde(rename="EXPERIMENT_ID")]
    EXPERIMENTID,
    

    /// Google Partner Search motivation data.
    ///
    /// "GPS_MOTIVATION"
    #[serde(rename="GPS_MOTIVATION")]
    GPSMOTIVATION,
    

    /// URL data.
    ///
    /// "URL"
    #[serde(rename="URL")]
    URL,
    

    /// Element we wanted user to focus on.
    ///
    /// "ELEMENT_FOCUS"
    #[serde(rename="ELEMENT_FOCUS")]
    ELEMENTFOCUS,
    

    /// Progress when viewing an item \[0-100\].
    ///
    /// "PROGRESS"
    #[serde(rename="PROGRESS")]
    PROGRESS,
}

impl AsRef<str> for EventDataKeyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventDataKeyEnum::EVENTDATATYPEUNSPECIFIED => "EVENT_DATA_TYPE_UNSPECIFIED",
            EventDataKeyEnum::ACTION => "ACTION",
            EventDataKeyEnum::AGENCYID => "AGENCY_ID",
            EventDataKeyEnum::AGENCYNAME => "AGENCY_NAME",
            EventDataKeyEnum::AGENCYPHONENUMBER => "AGENCY_PHONE_NUMBER",
            EventDataKeyEnum::AGENCYWEBSITE => "AGENCY_WEBSITE",
            EventDataKeyEnum::BUDGET => "BUDGET",
            EventDataKeyEnum::CENTERPOINT => "CENTER_POINT",
            EventDataKeyEnum::CERTIFICATION => "CERTIFICATION",
            EventDataKeyEnum::COMMENT => "COMMENT",
            EventDataKeyEnum::COUNTRY => "COUNTRY",
            EventDataKeyEnum::CURRENCY => "CURRENCY",
            EventDataKeyEnum::CURRENTLYVIEWEDAGENCYID => "CURRENTLY_VIEWED_AGENCY_ID",
            EventDataKeyEnum::DISTANCE => "DISTANCE",
            EventDataKeyEnum::DISTANCETYPE => "DISTANCE_TYPE",
            EventDataKeyEnum::EXAM => "EXAM",
            EventDataKeyEnum::HISTORYTOKEN => "HISTORY_TOKEN",
            EventDataKeyEnum::ID => "ID",
            EventDataKeyEnum::INDUSTRY => "INDUSTRY",
            EventDataKeyEnum::INSIGHTTAG => "INSIGHT_TAG",
            EventDataKeyEnum::LANGUAGE => "LANGUAGE",
            EventDataKeyEnum::LOCATION => "LOCATION",
            EventDataKeyEnum::MARKETINGOPTIN => "MARKETING_OPT_IN",
            EventDataKeyEnum::QUERY => "QUERY",
            EventDataKeyEnum::SEARCHSTARTINDEX => "SEARCH_START_INDEX",
            EventDataKeyEnum::SERVICE => "SERVICE",
            EventDataKeyEnum::SHOWVOW => "SHOW_VOW",
            EventDataKeyEnum::SOLUTION => "SOLUTION",
            EventDataKeyEnum::TRAFFICSOURCEID => "TRAFFIC_SOURCE_ID",
            EventDataKeyEnum::TRAFFICSUBID => "TRAFFIC_SUB_ID",
            EventDataKeyEnum::VIEWPORT => "VIEW_PORT",
            EventDataKeyEnum::WEBSITE => "WEBSITE",
            EventDataKeyEnum::DETAILS => "DETAILS",
            EventDataKeyEnum::EXPERIMENTID => "EXPERIMENT_ID",
            EventDataKeyEnum::GPSMOTIVATION => "GPS_MOTIVATION",
            EventDataKeyEnum::URL => "URL",
            EventDataKeyEnum::ELEMENTFOCUS => "ELEMENT_FOCUS",
            EventDataKeyEnum::PROGRESS => "PROGRESS",
        }
    }
}

impl std::convert::TryFrom< &str> for EventDataKeyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_DATA_TYPE_UNSPECIFIED" => Ok(EventDataKeyEnum::EVENTDATATYPEUNSPECIFIED),
           "ACTION" => Ok(EventDataKeyEnum::ACTION),
           "AGENCY_ID" => Ok(EventDataKeyEnum::AGENCYID),
           "AGENCY_NAME" => Ok(EventDataKeyEnum::AGENCYNAME),
           "AGENCY_PHONE_NUMBER" => Ok(EventDataKeyEnum::AGENCYPHONENUMBER),
           "AGENCY_WEBSITE" => Ok(EventDataKeyEnum::AGENCYWEBSITE),
           "BUDGET" => Ok(EventDataKeyEnum::BUDGET),
           "CENTER_POINT" => Ok(EventDataKeyEnum::CENTERPOINT),
           "CERTIFICATION" => Ok(EventDataKeyEnum::CERTIFICATION),
           "COMMENT" => Ok(EventDataKeyEnum::COMMENT),
           "COUNTRY" => Ok(EventDataKeyEnum::COUNTRY),
           "CURRENCY" => Ok(EventDataKeyEnum::CURRENCY),
           "CURRENTLY_VIEWED_AGENCY_ID" => Ok(EventDataKeyEnum::CURRENTLYVIEWEDAGENCYID),
           "DISTANCE" => Ok(EventDataKeyEnum::DISTANCE),
           "DISTANCE_TYPE" => Ok(EventDataKeyEnum::DISTANCETYPE),
           "EXAM" => Ok(EventDataKeyEnum::EXAM),
           "HISTORY_TOKEN" => Ok(EventDataKeyEnum::HISTORYTOKEN),
           "ID" => Ok(EventDataKeyEnum::ID),
           "INDUSTRY" => Ok(EventDataKeyEnum::INDUSTRY),
           "INSIGHT_TAG" => Ok(EventDataKeyEnum::INSIGHTTAG),
           "LANGUAGE" => Ok(EventDataKeyEnum::LANGUAGE),
           "LOCATION" => Ok(EventDataKeyEnum::LOCATION),
           "MARKETING_OPT_IN" => Ok(EventDataKeyEnum::MARKETINGOPTIN),
           "QUERY" => Ok(EventDataKeyEnum::QUERY),
           "SEARCH_START_INDEX" => Ok(EventDataKeyEnum::SEARCHSTARTINDEX),
           "SERVICE" => Ok(EventDataKeyEnum::SERVICE),
           "SHOW_VOW" => Ok(EventDataKeyEnum::SHOWVOW),
           "SOLUTION" => Ok(EventDataKeyEnum::SOLUTION),
           "TRAFFIC_SOURCE_ID" => Ok(EventDataKeyEnum::TRAFFICSOURCEID),
           "TRAFFIC_SUB_ID" => Ok(EventDataKeyEnum::TRAFFICSUBID),
           "VIEW_PORT" => Ok(EventDataKeyEnum::VIEWPORT),
           "WEBSITE" => Ok(EventDataKeyEnum::WEBSITE),
           "DETAILS" => Ok(EventDataKeyEnum::DETAILS),
           "EXPERIMENT_ID" => Ok(EventDataKeyEnum::EXPERIMENTID),
           "GPS_MOTIVATION" => Ok(EventDataKeyEnum::GPSMOTIVATION),
           "URL" => Ok(EventDataKeyEnum::URL),
           "ELEMENT_FOCUS" => Ok(EventDataKeyEnum::ELEMENTFOCUS),
           "PROGRESS" => Ok(EventDataKeyEnum::PROGRESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventDataKeyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExamStatusExamTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the exam.
pub enum ExamStatusExamTypeEnum {
    

    /// Unchosen.
    ///
    /// "CERTIFICATION_EXAM_TYPE_UNSPECIFIED"
    #[serde(rename="CERTIFICATION_EXAM_TYPE_UNSPECIFIED")]
    CERTIFICATIONEXAMTYPEUNSPECIFIED,
    

    /// Adwords Fundamentals exam.
    ///
    /// "CET_ADWORDS_FUNDAMENTALS"
    #[serde(rename="CET_ADWORDS_FUNDAMENTALS")]
    CETADWORDSFUNDAMENTALS,
    

    /// AdWords advanced search exam.
    ///
    /// "CET_ADWORDS_ADVANCED_SEARCH"
    #[serde(rename="CET_ADWORDS_ADVANCED_SEARCH")]
    CETADWORDSADVANCEDSEARCH,
    

    /// AdWords advanced display exam.
    ///
    /// "CET_ADWORDS_ADVANCED_DISPLAY"
    #[serde(rename="CET_ADWORDS_ADVANCED_DISPLAY")]
    CETADWORDSADVANCEDDISPLAY,
    

    /// VideoAds exam.
    ///
    /// "CET_VIDEO_ADS"
    #[serde(rename="CET_VIDEO_ADS")]
    CETVIDEOADS,
    

    /// DoubleClick exam.
    ///
    /// "CET_DOUBLECLICK"
    #[serde(rename="CET_DOUBLECLICK")]
    CETDOUBLECLICK,
    

    /// Analytics exam.
    ///
    /// "CET_ANALYTICS"
    #[serde(rename="CET_ANALYTICS")]
    CETANALYTICS,
    

    /// Shopping exam.
    ///
    /// "CET_SHOPPING"
    #[serde(rename="CET_SHOPPING")]
    CETSHOPPING,
    

    /// Mobile exam.
    ///
    /// "CET_MOBILE"
    #[serde(rename="CET_MOBILE")]
    CETMOBILE,
    

    /// Digital Sales exam.
    ///
    /// "CET_DIGITAL_SALES"
    #[serde(rename="CET_DIGITAL_SALES")]
    CETDIGITALSALES,
    

    /// Mobile Sites exam.
    ///
    /// "CET_MOBILE_SITES"
    #[serde(rename="CET_MOBILE_SITES")]
    CETMOBILESITES,
}

impl AsRef<str> for ExamStatusExamTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExamStatusExamTypeEnum::CERTIFICATIONEXAMTYPEUNSPECIFIED => "CERTIFICATION_EXAM_TYPE_UNSPECIFIED",
            ExamStatusExamTypeEnum::CETADWORDSFUNDAMENTALS => "CET_ADWORDS_FUNDAMENTALS",
            ExamStatusExamTypeEnum::CETADWORDSADVANCEDSEARCH => "CET_ADWORDS_ADVANCED_SEARCH",
            ExamStatusExamTypeEnum::CETADWORDSADVANCEDDISPLAY => "CET_ADWORDS_ADVANCED_DISPLAY",
            ExamStatusExamTypeEnum::CETVIDEOADS => "CET_VIDEO_ADS",
            ExamStatusExamTypeEnum::CETDOUBLECLICK => "CET_DOUBLECLICK",
            ExamStatusExamTypeEnum::CETANALYTICS => "CET_ANALYTICS",
            ExamStatusExamTypeEnum::CETSHOPPING => "CET_SHOPPING",
            ExamStatusExamTypeEnum::CETMOBILE => "CET_MOBILE",
            ExamStatusExamTypeEnum::CETDIGITALSALES => "CET_DIGITAL_SALES",
            ExamStatusExamTypeEnum::CETMOBILESITES => "CET_MOBILE_SITES",
        }
    }
}

impl std::convert::TryFrom< &str> for ExamStatusExamTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERTIFICATION_EXAM_TYPE_UNSPECIFIED" => Ok(ExamStatusExamTypeEnum::CERTIFICATIONEXAMTYPEUNSPECIFIED),
           "CET_ADWORDS_FUNDAMENTALS" => Ok(ExamStatusExamTypeEnum::CETADWORDSFUNDAMENTALS),
           "CET_ADWORDS_ADVANCED_SEARCH" => Ok(ExamStatusExamTypeEnum::CETADWORDSADVANCEDSEARCH),
           "CET_ADWORDS_ADVANCED_DISPLAY" => Ok(ExamStatusExamTypeEnum::CETADWORDSADVANCEDDISPLAY),
           "CET_VIDEO_ADS" => Ok(ExamStatusExamTypeEnum::CETVIDEOADS),
           "CET_DOUBLECLICK" => Ok(ExamStatusExamTypeEnum::CETDOUBLECLICK),
           "CET_ANALYTICS" => Ok(ExamStatusExamTypeEnum::CETANALYTICS),
           "CET_SHOPPING" => Ok(ExamStatusExamTypeEnum::CETSHOPPING),
           "CET_MOBILE" => Ok(ExamStatusExamTypeEnum::CETMOBILE),
           "CET_DIGITAL_SALES" => Ok(ExamStatusExamTypeEnum::CETDIGITALSALES),
           "CET_MOBILE_SITES" => Ok(ExamStatusExamTypeEnum::CETMOBILESITES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExamStatusExamTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ListOffersResponseNoOfferReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Reason why no Offers are available.
pub enum ListOffersResponseNoOfferReasonEnum {
    

    /// Unset.
    ///
    /// "NO_OFFER_REASON_UNSPECIFIED"
    #[serde(rename="NO_OFFER_REASON_UNSPECIFIED")]
    NOOFFERREASONUNSPECIFIED,
    

    /// Not an MCC.
    ///
    /// "NO_OFFER_REASON_NO_MCC"
    #[serde(rename="NO_OFFER_REASON_NO_MCC")]
    NOOFFERREASONNOMCC,
    

    /// Offer limit has been reached.
    ///
    /// "NO_OFFER_REASON_LIMIT_REACHED"
    #[serde(rename="NO_OFFER_REASON_LIMIT_REACHED")]
    NOOFFERREASONLIMITREACHED,
    

    /// Ineligible for offers.
    ///
    /// "NO_OFFER_REASON_INELIGIBLE"
    #[serde(rename="NO_OFFER_REASON_INELIGIBLE")]
    NOOFFERREASONINELIGIBLE,
}

impl AsRef<str> for ListOffersResponseNoOfferReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ListOffersResponseNoOfferReasonEnum::NOOFFERREASONUNSPECIFIED => "NO_OFFER_REASON_UNSPECIFIED",
            ListOffersResponseNoOfferReasonEnum::NOOFFERREASONNOMCC => "NO_OFFER_REASON_NO_MCC",
            ListOffersResponseNoOfferReasonEnum::NOOFFERREASONLIMITREACHED => "NO_OFFER_REASON_LIMIT_REACHED",
            ListOffersResponseNoOfferReasonEnum::NOOFFERREASONINELIGIBLE => "NO_OFFER_REASON_INELIGIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for ListOffersResponseNoOfferReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NO_OFFER_REASON_UNSPECIFIED" => Ok(ListOffersResponseNoOfferReasonEnum::NOOFFERREASONUNSPECIFIED),
           "NO_OFFER_REASON_NO_MCC" => Ok(ListOffersResponseNoOfferReasonEnum::NOOFFERREASONNOMCC),
           "NO_OFFER_REASON_LIMIT_REACHED" => Ok(ListOffersResponseNoOfferReasonEnum::NOOFFERREASONLIMITREACHED),
           "NO_OFFER_REASON_INELIGIBLE" => Ok(ListOffersResponseNoOfferReasonEnum::NOOFFERREASONINELIGIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ListOffersResponseNoOfferReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CountryOfferInfoOfferTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of offer country is eligible for.
pub enum CountryOfferInfoOfferTypeEnum {
    

    /// Unset.
    ///
    /// "OFFER_TYPE_UNSPECIFIED"
    #[serde(rename="OFFER_TYPE_UNSPECIFIED")]
    OFFERTYPEUNSPECIFIED,
    

    /// AdWords spend X get Y.
    ///
    /// "OFFER_TYPE_SPEND_X_GET_Y"
    #[serde(rename="OFFER_TYPE_SPEND_X_GET_Y")]
    OFFERTYPESPENDXGETY,
    

    /// Youtube video.
    ///
    /// "OFFER_TYPE_VIDEO"
    #[serde(rename="OFFER_TYPE_VIDEO")]
    OFFERTYPEVIDEO,
    

    /// Spend Match up to Y.
    ///
    /// "OFFER_TYPE_SPEND_MATCH"
    #[serde(rename="OFFER_TYPE_SPEND_MATCH")]
    OFFERTYPESPENDMATCH,
}

impl AsRef<str> for CountryOfferInfoOfferTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CountryOfferInfoOfferTypeEnum::OFFERTYPEUNSPECIFIED => "OFFER_TYPE_UNSPECIFIED",
            CountryOfferInfoOfferTypeEnum::OFFERTYPESPENDXGETY => "OFFER_TYPE_SPEND_X_GET_Y",
            CountryOfferInfoOfferTypeEnum::OFFERTYPEVIDEO => "OFFER_TYPE_VIDEO",
            CountryOfferInfoOfferTypeEnum::OFFERTYPESPENDMATCH => "OFFER_TYPE_SPEND_MATCH",
        }
    }
}

impl std::convert::TryFrom< &str> for CountryOfferInfoOfferTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFFER_TYPE_UNSPECIFIED" => Ok(CountryOfferInfoOfferTypeEnum::OFFERTYPEUNSPECIFIED),
           "OFFER_TYPE_SPEND_X_GET_Y" => Ok(CountryOfferInfoOfferTypeEnum::OFFERTYPESPENDXGETY),
           "OFFER_TYPE_VIDEO" => Ok(CountryOfferInfoOfferTypeEnum::OFFERTYPEVIDEO),
           "OFFER_TYPE_SPEND_MATCH" => Ok(CountryOfferInfoOfferTypeEnum::OFFERTYPESPENDMATCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CountryOfferInfoOfferTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OfferCustomerOfferTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the offer
pub enum OfferCustomerOfferTypeEnum {
    

    /// Unset.
    ///
    /// "OFFER_TYPE_UNSPECIFIED"
    #[serde(rename="OFFER_TYPE_UNSPECIFIED")]
    OFFERTYPEUNSPECIFIED,
    

    /// AdWords spend X get Y.
    ///
    /// "OFFER_TYPE_SPEND_X_GET_Y"
    #[serde(rename="OFFER_TYPE_SPEND_X_GET_Y")]
    OFFERTYPESPENDXGETY,
    

    /// Youtube video.
    ///
    /// "OFFER_TYPE_VIDEO"
    #[serde(rename="OFFER_TYPE_VIDEO")]
    OFFERTYPEVIDEO,
    

    /// Spend Match up to Y.
    ///
    /// "OFFER_TYPE_SPEND_MATCH"
    #[serde(rename="OFFER_TYPE_SPEND_MATCH")]
    OFFERTYPESPENDMATCH,
}

impl AsRef<str> for OfferCustomerOfferTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OfferCustomerOfferTypeEnum::OFFERTYPEUNSPECIFIED => "OFFER_TYPE_UNSPECIFIED",
            OfferCustomerOfferTypeEnum::OFFERTYPESPENDXGETY => "OFFER_TYPE_SPEND_X_GET_Y",
            OfferCustomerOfferTypeEnum::OFFERTYPEVIDEO => "OFFER_TYPE_VIDEO",
            OfferCustomerOfferTypeEnum::OFFERTYPESPENDMATCH => "OFFER_TYPE_SPEND_MATCH",
        }
    }
}

impl std::convert::TryFrom< &str> for OfferCustomerOfferTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFFER_TYPE_UNSPECIFIED" => Ok(OfferCustomerOfferTypeEnum::OFFERTYPEUNSPECIFIED),
           "OFFER_TYPE_SPEND_X_GET_Y" => Ok(OfferCustomerOfferTypeEnum::OFFERTYPESPENDXGETY),
           "OFFER_TYPE_VIDEO" => Ok(OfferCustomerOfferTypeEnum::OFFERTYPEVIDEO),
           "OFFER_TYPE_SPEND_MATCH" => Ok(OfferCustomerOfferTypeEnum::OFFERTYPESPENDMATCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OfferCustomerOfferTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificationStatusTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the certification.
pub enum CertificationStatusTypeEnum {
    

    /// Unchosen.
    ///
    /// "CERTIFICATION_TYPE_UNSPECIFIED"
    #[serde(rename="CERTIFICATION_TYPE_UNSPECIFIED")]
    CERTIFICATIONTYPEUNSPECIFIED,
    

    /// AdWords certified.
    ///
    /// "CT_ADWORDS"
    #[serde(rename="CT_ADWORDS")]
    CTADWORDS,
    

    /// YouTube certified.
    ///
    /// "CT_YOUTUBE"
    #[serde(rename="CT_YOUTUBE")]
    CTYOUTUBE,
    

    /// VideoAds certified.
    ///
    /// "CT_VIDEOADS"
    #[serde(rename="CT_VIDEOADS")]
    CTVIDEOADS,
    

    /// Analytics certified.
    ///
    /// "CT_ANALYTICS"
    #[serde(rename="CT_ANALYTICS")]
    CTANALYTICS,
    

    /// DoubleClick certified.
    ///
    /// "CT_DOUBLECLICK"
    #[serde(rename="CT_DOUBLECLICK")]
    CTDOUBLECLICK,
    

    /// Shopping certified.
    ///
    /// "CT_SHOPPING"
    #[serde(rename="CT_SHOPPING")]
    CTSHOPPING,
    

    /// Mobile certified.
    ///
    /// "CT_MOBILE"
    #[serde(rename="CT_MOBILE")]
    CTMOBILE,
    

    /// Digital sales certified.
    ///
    /// "CT_DIGITAL_SALES"
    #[serde(rename="CT_DIGITAL_SALES")]
    CTDIGITALSALES,
    

    /// AdWords Search certified.
    ///
    /// "CT_ADWORDS_SEARCH"
    #[serde(rename="CT_ADWORDS_SEARCH")]
    CTADWORDSSEARCH,
    

    /// AdWords Display certified.
    ///
    /// "CT_ADWORDS_DISPLAY"
    #[serde(rename="CT_ADWORDS_DISPLAY")]
    CTADWORDSDISPLAY,
    

    /// Mobile Sites certified.
    ///
    /// "CT_MOBILE_SITES"
    #[serde(rename="CT_MOBILE_SITES")]
    CTMOBILESITES,
}

impl AsRef<str> for CertificationStatusTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificationStatusTypeEnum::CERTIFICATIONTYPEUNSPECIFIED => "CERTIFICATION_TYPE_UNSPECIFIED",
            CertificationStatusTypeEnum::CTADWORDS => "CT_ADWORDS",
            CertificationStatusTypeEnum::CTYOUTUBE => "CT_YOUTUBE",
            CertificationStatusTypeEnum::CTVIDEOADS => "CT_VIDEOADS",
            CertificationStatusTypeEnum::CTANALYTICS => "CT_ANALYTICS",
            CertificationStatusTypeEnum::CTDOUBLECLICK => "CT_DOUBLECLICK",
            CertificationStatusTypeEnum::CTSHOPPING => "CT_SHOPPING",
            CertificationStatusTypeEnum::CTMOBILE => "CT_MOBILE",
            CertificationStatusTypeEnum::CTDIGITALSALES => "CT_DIGITAL_SALES",
            CertificationStatusTypeEnum::CTADWORDSSEARCH => "CT_ADWORDS_SEARCH",
            CertificationStatusTypeEnum::CTADWORDSDISPLAY => "CT_ADWORDS_DISPLAY",
            CertificationStatusTypeEnum::CTMOBILESITES => "CT_MOBILE_SITES",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificationStatusTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERTIFICATION_TYPE_UNSPECIFIED" => Ok(CertificationStatusTypeEnum::CERTIFICATIONTYPEUNSPECIFIED),
           "CT_ADWORDS" => Ok(CertificationStatusTypeEnum::CTADWORDS),
           "CT_YOUTUBE" => Ok(CertificationStatusTypeEnum::CTYOUTUBE),
           "CT_VIDEOADS" => Ok(CertificationStatusTypeEnum::CTVIDEOADS),
           "CT_ANALYTICS" => Ok(CertificationStatusTypeEnum::CTANALYTICS),
           "CT_DOUBLECLICK" => Ok(CertificationStatusTypeEnum::CTDOUBLECLICK),
           "CT_SHOPPING" => Ok(CertificationStatusTypeEnum::CTSHOPPING),
           "CT_MOBILE" => Ok(CertificationStatusTypeEnum::CTMOBILE),
           "CT_DIGITAL_SALES" => Ok(CertificationStatusTypeEnum::CTDIGITALSALES),
           "CT_ADWORDS_SEARCH" => Ok(CertificationStatusTypeEnum::CTADWORDSSEARCH),
           "CT_ADWORDS_DISPLAY" => Ok(CertificationStatusTypeEnum::CTADWORDSDISPLAY),
           "CT_MOBILE_SITES" => Ok(CertificationStatusTypeEnum::CTMOBILESITES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificationStatusTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpecializationStatusBadgeSpecializationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The specialization this status is for.
pub enum SpecializationStatusBadgeSpecializationEnum {
    

    /// Unknown specialization
    ///
    /// "BADGE_SPECIALIZATION_UNKNOWN"
    #[serde(rename="BADGE_SPECIALIZATION_UNKNOWN")]
    BADGESPECIALIZATIONUNKNOWN,
    

    /// AdWords Search specialization
    ///
    /// "BADGE_SPECIALIZATION_ADWORDS_SEARCH"
    #[serde(rename="BADGE_SPECIALIZATION_ADWORDS_SEARCH")]
    BADGESPECIALIZATIONADWORDSSEARCH,
    

    /// AdWords Display specialization
    ///
    /// "BADGE_SPECIALIZATION_ADWORDS_DISPLAY"
    #[serde(rename="BADGE_SPECIALIZATION_ADWORDS_DISPLAY")]
    BADGESPECIALIZATIONADWORDSDISPLAY,
    

    /// AdWords Mobile specialization
    ///
    /// "BADGE_SPECIALIZATION_ADWORDS_MOBILE"
    #[serde(rename="BADGE_SPECIALIZATION_ADWORDS_MOBILE")]
    BADGESPECIALIZATIONADWORDSMOBILE,
    

    /// AdWords Video specialization
    ///
    /// "BADGE_SPECIALIZATION_ADWORDS_VIDEO"
    #[serde(rename="BADGE_SPECIALIZATION_ADWORDS_VIDEO")]
    BADGESPECIALIZATIONADWORDSVIDEO,
    

    /// AdWords Shopping specialization
    ///
    /// "BADGE_SPECIALIZATION_ADWORDS_SHOPPING"
    #[serde(rename="BADGE_SPECIALIZATION_ADWORDS_SHOPPING")]
    BADGESPECIALIZATIONADWORDSSHOPPING,
}

impl AsRef<str> for SpecializationStatusBadgeSpecializationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONUNKNOWN => "BADGE_SPECIALIZATION_UNKNOWN",
            SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONADWORDSSEARCH => "BADGE_SPECIALIZATION_ADWORDS_SEARCH",
            SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONADWORDSDISPLAY => "BADGE_SPECIALIZATION_ADWORDS_DISPLAY",
            SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONADWORDSMOBILE => "BADGE_SPECIALIZATION_ADWORDS_MOBILE",
            SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONADWORDSVIDEO => "BADGE_SPECIALIZATION_ADWORDS_VIDEO",
            SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONADWORDSSHOPPING => "BADGE_SPECIALIZATION_ADWORDS_SHOPPING",
        }
    }
}

impl std::convert::TryFrom< &str> for SpecializationStatusBadgeSpecializationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BADGE_SPECIALIZATION_UNKNOWN" => Ok(SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONUNKNOWN),
           "BADGE_SPECIALIZATION_ADWORDS_SEARCH" => Ok(SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONADWORDSSEARCH),
           "BADGE_SPECIALIZATION_ADWORDS_DISPLAY" => Ok(SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONADWORDSDISPLAY),
           "BADGE_SPECIALIZATION_ADWORDS_MOBILE" => Ok(SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONADWORDSMOBILE),
           "BADGE_SPECIALIZATION_ADWORDS_VIDEO" => Ok(SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONADWORDSVIDEO),
           "BADGE_SPECIALIZATION_ADWORDS_SHOPPING" => Ok(SpecializationStatusBadgeSpecializationEnum::BADGESPECIALIZATIONADWORDSSHOPPING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpecializationStatusBadgeSpecializationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpecializationStatusBadgeSpecializationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of agency specialization.
pub enum SpecializationStatusBadgeSpecializationStateEnum {
    

    /// Unknown state
    ///
    /// "BADGE_SPECIALIZATION_STATE_UNKNOWN"
    #[serde(rename="BADGE_SPECIALIZATION_STATE_UNKNOWN")]
    BADGESPECIALIZATIONSTATEUNKNOWN,
    

    /// Specialization passed
    ///
    /// "BADGE_SPECIALIZATION_STATE_PASSED"
    #[serde(rename="BADGE_SPECIALIZATION_STATE_PASSED")]
    BADGESPECIALIZATIONSTATEPASSED,
    

    /// Specialization not passed
    ///
    /// "BADGE_SPECIALIZATION_STATE_NOT_PASSED"
    #[serde(rename="BADGE_SPECIALIZATION_STATE_NOT_PASSED")]
    BADGESPECIALIZATIONSTATENOTPASSED,
    

    /// Specialization in grace
    ///
    /// "BADGE_SPECIALIZATION_STATE_IN_GRACE"
    #[serde(rename="BADGE_SPECIALIZATION_STATE_IN_GRACE")]
    BADGESPECIALIZATIONSTATEINGRACE,
}

impl AsRef<str> for SpecializationStatusBadgeSpecializationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpecializationStatusBadgeSpecializationStateEnum::BADGESPECIALIZATIONSTATEUNKNOWN => "BADGE_SPECIALIZATION_STATE_UNKNOWN",
            SpecializationStatusBadgeSpecializationStateEnum::BADGESPECIALIZATIONSTATEPASSED => "BADGE_SPECIALIZATION_STATE_PASSED",
            SpecializationStatusBadgeSpecializationStateEnum::BADGESPECIALIZATIONSTATENOTPASSED => "BADGE_SPECIALIZATION_STATE_NOT_PASSED",
            SpecializationStatusBadgeSpecializationStateEnum::BADGESPECIALIZATIONSTATEINGRACE => "BADGE_SPECIALIZATION_STATE_IN_GRACE",
        }
    }
}

impl std::convert::TryFrom< &str> for SpecializationStatusBadgeSpecializationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BADGE_SPECIALIZATION_STATE_UNKNOWN" => Ok(SpecializationStatusBadgeSpecializationStateEnum::BADGESPECIALIZATIONSTATEUNKNOWN),
           "BADGE_SPECIALIZATION_STATE_PASSED" => Ok(SpecializationStatusBadgeSpecializationStateEnum::BADGESPECIALIZATIONSTATEPASSED),
           "BADGE_SPECIALIZATION_STATE_NOT_PASSED" => Ok(SpecializationStatusBadgeSpecializationStateEnum::BADGESPECIALIZATIONSTATENOTPASSED),
           "BADGE_SPECIALIZATION_STATE_IN_GRACE" => Ok(SpecializationStatusBadgeSpecializationStateEnum::BADGESPECIALIZATIONSTATEINGRACE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpecializationStatusBadgeSpecializationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificationCertificationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of certification, the area of expertise.
pub enum CertificationCertificationTypeEnum {
    

    /// Unchosen.
    ///
    /// "CERTIFICATION_TYPE_UNSPECIFIED"
    #[serde(rename="CERTIFICATION_TYPE_UNSPECIFIED")]
    CERTIFICATIONTYPEUNSPECIFIED,
    

    /// AdWords certified.
    ///
    /// "CT_ADWORDS"
    #[serde(rename="CT_ADWORDS")]
    CTADWORDS,
    

    /// YouTube certified.
    ///
    /// "CT_YOUTUBE"
    #[serde(rename="CT_YOUTUBE")]
    CTYOUTUBE,
    

    /// VideoAds certified.
    ///
    /// "CT_VIDEOADS"
    #[serde(rename="CT_VIDEOADS")]
    CTVIDEOADS,
    

    /// Analytics certified.
    ///
    /// "CT_ANALYTICS"
    #[serde(rename="CT_ANALYTICS")]
    CTANALYTICS,
    

    /// DoubleClick certified.
    ///
    /// "CT_DOUBLECLICK"
    #[serde(rename="CT_DOUBLECLICK")]
    CTDOUBLECLICK,
    

    /// Shopping certified.
    ///
    /// "CT_SHOPPING"
    #[serde(rename="CT_SHOPPING")]
    CTSHOPPING,
    

    /// Mobile certified.
    ///
    /// "CT_MOBILE"
    #[serde(rename="CT_MOBILE")]
    CTMOBILE,
    

    /// Digital sales certified.
    ///
    /// "CT_DIGITAL_SALES"
    #[serde(rename="CT_DIGITAL_SALES")]
    CTDIGITALSALES,
    

    /// AdWords Search certified.
    ///
    /// "CT_ADWORDS_SEARCH"
    #[serde(rename="CT_ADWORDS_SEARCH")]
    CTADWORDSSEARCH,
    

    /// AdWords Display certified.
    ///
    /// "CT_ADWORDS_DISPLAY"
    #[serde(rename="CT_ADWORDS_DISPLAY")]
    CTADWORDSDISPLAY,
    

    /// Mobile Sites certified.
    ///
    /// "CT_MOBILE_SITES"
    #[serde(rename="CT_MOBILE_SITES")]
    CTMOBILESITES,
}

impl AsRef<str> for CertificationCertificationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificationCertificationTypeEnum::CERTIFICATIONTYPEUNSPECIFIED => "CERTIFICATION_TYPE_UNSPECIFIED",
            CertificationCertificationTypeEnum::CTADWORDS => "CT_ADWORDS",
            CertificationCertificationTypeEnum::CTYOUTUBE => "CT_YOUTUBE",
            CertificationCertificationTypeEnum::CTVIDEOADS => "CT_VIDEOADS",
            CertificationCertificationTypeEnum::CTANALYTICS => "CT_ANALYTICS",
            CertificationCertificationTypeEnum::CTDOUBLECLICK => "CT_DOUBLECLICK",
            CertificationCertificationTypeEnum::CTSHOPPING => "CT_SHOPPING",
            CertificationCertificationTypeEnum::CTMOBILE => "CT_MOBILE",
            CertificationCertificationTypeEnum::CTDIGITALSALES => "CT_DIGITAL_SALES",
            CertificationCertificationTypeEnum::CTADWORDSSEARCH => "CT_ADWORDS_SEARCH",
            CertificationCertificationTypeEnum::CTADWORDSDISPLAY => "CT_ADWORDS_DISPLAY",
            CertificationCertificationTypeEnum::CTMOBILESITES => "CT_MOBILE_SITES",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificationCertificationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERTIFICATION_TYPE_UNSPECIFIED" => Ok(CertificationCertificationTypeEnum::CERTIFICATIONTYPEUNSPECIFIED),
           "CT_ADWORDS" => Ok(CertificationCertificationTypeEnum::CTADWORDS),
           "CT_YOUTUBE" => Ok(CertificationCertificationTypeEnum::CTYOUTUBE),
           "CT_VIDEOADS" => Ok(CertificationCertificationTypeEnum::CTVIDEOADS),
           "CT_ANALYTICS" => Ok(CertificationCertificationTypeEnum::CTANALYTICS),
           "CT_DOUBLECLICK" => Ok(CertificationCertificationTypeEnum::CTDOUBLECLICK),
           "CT_SHOPPING" => Ok(CertificationCertificationTypeEnum::CTSHOPPING),
           "CT_MOBILE" => Ok(CertificationCertificationTypeEnum::CTMOBILE),
           "CT_DIGITAL_SALES" => Ok(CertificationCertificationTypeEnum::CTDIGITALSALES),
           "CT_ADWORDS_SEARCH" => Ok(CertificationCertificationTypeEnum::CTADWORDSSEARCH),
           "CT_ADWORDS_DISPLAY" => Ok(CertificationCertificationTypeEnum::CTADWORDSDISPLAY),
           "CT_MOBILE_SITES" => Ok(CertificationCertificationTypeEnum::CTMOBILESITES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificationCertificationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyProfileStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The public viewability status of the company's profile.
pub enum CompanyProfileStatusEnum {
    

    /// Unchosen.
    ///
    /// "COMPANY_PROFILE_STATUS_UNSPECIFIED"
    #[serde(rename="COMPANY_PROFILE_STATUS_UNSPECIFIED")]
    COMPANYPROFILESTATUSUNSPECIFIED,
    

    /// Company profile does not show up publicly.
    ///
    /// "HIDDEN"
    #[serde(rename="HIDDEN")]
    HIDDEN,
    

    /// Company profile can only be viewed by the profile's URL
and not by Google Partner Search.
    ///
    /// "PUBLISHED"
    #[serde(rename="PUBLISHED")]
    PUBLISHED,
    

    /// Company profile can be viewed by the profile's URL
and by Google Partner Search.
    ///
    /// "SEARCHABLE"
    #[serde(rename="SEARCHABLE")]
    SEARCHABLE,
}

impl AsRef<str> for CompanyProfileStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyProfileStatusEnum::COMPANYPROFILESTATUSUNSPECIFIED => "COMPANY_PROFILE_STATUS_UNSPECIFIED",
            CompanyProfileStatusEnum::HIDDEN => "HIDDEN",
            CompanyProfileStatusEnum::PUBLISHED => "PUBLISHED",
            CompanyProfileStatusEnum::SEARCHABLE => "SEARCHABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyProfileStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPANY_PROFILE_STATUS_UNSPECIFIED" => Ok(CompanyProfileStatusEnum::COMPANYPROFILESTATUSUNSPECIFIED),
           "HIDDEN" => Ok(CompanyProfileStatusEnum::HIDDEN),
           "PUBLISHED" => Ok(CompanyProfileStatusEnum::PUBLISHED),
           "SEARCHABLE" => Ok(CompanyProfileStatusEnum::SEARCHABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyProfileStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyIndustriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Industries the company can help with.
pub enum CompanyIndustriesEnum {
    

    /// Unchosen.
    ///
    /// "INDUSTRY_UNSPECIFIED"
    #[serde(rename="INDUSTRY_UNSPECIFIED")]
    INDUSTRYUNSPECIFIED,
    

    /// The automotive industry.
    ///
    /// "I_AUTOMOTIVE"
    #[serde(rename="I_AUTOMOTIVE")]
    IAUTOMOTIVE,
    

    /// The business-to-business industry.
    ///
    /// "I_BUSINESS_TO_BUSINESS"
    #[serde(rename="I_BUSINESS_TO_BUSINESS")]
    IBUSINESSTOBUSINESS,
    

    /// The consumer packaged goods industry.
    ///
    /// "I_CONSUMER_PACKAGED_GOODS"
    #[serde(rename="I_CONSUMER_PACKAGED_GOODS")]
    ICONSUMERPACKAGEDGOODS,
    

    /// The education industry.
    ///
    /// "I_EDUCATION"
    #[serde(rename="I_EDUCATION")]
    IEDUCATION,
    

    /// The finance industry.
    ///
    /// "I_FINANCE"
    #[serde(rename="I_FINANCE")]
    IFINANCE,
    

    /// The healthcare industry.
    ///
    /// "I_HEALTHCARE"
    #[serde(rename="I_HEALTHCARE")]
    IHEALTHCARE,
    

    /// The media and entertainment industry.
    ///
    /// "I_MEDIA_AND_ENTERTAINMENT"
    #[serde(rename="I_MEDIA_AND_ENTERTAINMENT")]
    IMEDIAANDENTERTAINMENT,
    

    /// The retail industry.
    ///
    /// "I_RETAIL"
    #[serde(rename="I_RETAIL")]
    IRETAIL,
    

    /// The technology industry.
    ///
    /// "I_TECHNOLOGY"
    #[serde(rename="I_TECHNOLOGY")]
    ITECHNOLOGY,
    

    /// The travel industry.
    ///
    /// "I_TRAVEL"
    #[serde(rename="I_TRAVEL")]
    ITRAVEL,
}

impl AsRef<str> for CompanyIndustriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyIndustriesEnum::INDUSTRYUNSPECIFIED => "INDUSTRY_UNSPECIFIED",
            CompanyIndustriesEnum::IAUTOMOTIVE => "I_AUTOMOTIVE",
            CompanyIndustriesEnum::IBUSINESSTOBUSINESS => "I_BUSINESS_TO_BUSINESS",
            CompanyIndustriesEnum::ICONSUMERPACKAGEDGOODS => "I_CONSUMER_PACKAGED_GOODS",
            CompanyIndustriesEnum::IEDUCATION => "I_EDUCATION",
            CompanyIndustriesEnum::IFINANCE => "I_FINANCE",
            CompanyIndustriesEnum::IHEALTHCARE => "I_HEALTHCARE",
            CompanyIndustriesEnum::IMEDIAANDENTERTAINMENT => "I_MEDIA_AND_ENTERTAINMENT",
            CompanyIndustriesEnum::IRETAIL => "I_RETAIL",
            CompanyIndustriesEnum::ITECHNOLOGY => "I_TECHNOLOGY",
            CompanyIndustriesEnum::ITRAVEL => "I_TRAVEL",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyIndustriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INDUSTRY_UNSPECIFIED" => Ok(CompanyIndustriesEnum::INDUSTRYUNSPECIFIED),
           "I_AUTOMOTIVE" => Ok(CompanyIndustriesEnum::IAUTOMOTIVE),
           "I_BUSINESS_TO_BUSINESS" => Ok(CompanyIndustriesEnum::IBUSINESSTOBUSINESS),
           "I_CONSUMER_PACKAGED_GOODS" => Ok(CompanyIndustriesEnum::ICONSUMERPACKAGEDGOODS),
           "I_EDUCATION" => Ok(CompanyIndustriesEnum::IEDUCATION),
           "I_FINANCE" => Ok(CompanyIndustriesEnum::IFINANCE),
           "I_HEALTHCARE" => Ok(CompanyIndustriesEnum::IHEALTHCARE),
           "I_MEDIA_AND_ENTERTAINMENT" => Ok(CompanyIndustriesEnum::IMEDIAANDENTERTAINMENT),
           "I_RETAIL" => Ok(CompanyIndustriesEnum::IRETAIL),
           "I_TECHNOLOGY" => Ok(CompanyIndustriesEnum::ITECHNOLOGY),
           "I_TRAVEL" => Ok(CompanyIndustriesEnum::ITRAVEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyIndustriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyServicesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Services the company can help with.
pub enum CompanyServicesEnum {
    

    /// Unchosen.
    ///
    /// "SERVICE_UNSPECIFIED"
    #[serde(rename="SERVICE_UNSPECIFIED")]
    SERVICEUNSPECIFIED,
    

    /// Help with advanced AdWords support.
    ///
    /// "S_ADVANCED_ADWORDS_SUPPORT"
    #[serde(rename="S_ADVANCED_ADWORDS_SUPPORT")]
    SADVANCEDADWORDSSUPPORT,
    

    /// Help with advertising on Google.
    ///
    /// "S_ADVERTISING_ON_GOOGLE"
    #[serde(rename="S_ADVERTISING_ON_GOOGLE")]
    SADVERTISINGONGOOGLE,
    

    /// Help with an enhanced website.
    ///
    /// "S_AN_ENHANCED_WEBSITE"
    #[serde(rename="S_AN_ENHANCED_WEBSITE")]
    SANENHANCEDWEBSITE,
    

    /// Help with an online marketing plan.
    ///
    /// "S_AN_ONLINE_MARKETING_PLAN"
    #[serde(rename="S_AN_ONLINE_MARKETING_PLAN")]
    SANONLINEMARKETINGPLAN,
    

    /// Help with mobile and video ads.
    ///
    /// "S_MOBILE_AND_VIDEO_ADS"
    #[serde(rename="S_MOBILE_AND_VIDEO_ADS")]
    SMOBILEANDVIDEOADS,
    

    /// Help with mobile websites.
    ///
    /// "S_MOBILE_WEBSITE_SERVICES"
    #[serde(rename="S_MOBILE_WEBSITE_SERVICES")]
    SMOBILEWEBSITESERVICES,
}

impl AsRef<str> for CompanyServicesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyServicesEnum::SERVICEUNSPECIFIED => "SERVICE_UNSPECIFIED",
            CompanyServicesEnum::SADVANCEDADWORDSSUPPORT => "S_ADVANCED_ADWORDS_SUPPORT",
            CompanyServicesEnum::SADVERTISINGONGOOGLE => "S_ADVERTISING_ON_GOOGLE",
            CompanyServicesEnum::SANENHANCEDWEBSITE => "S_AN_ENHANCED_WEBSITE",
            CompanyServicesEnum::SANONLINEMARKETINGPLAN => "S_AN_ONLINE_MARKETING_PLAN",
            CompanyServicesEnum::SMOBILEANDVIDEOADS => "S_MOBILE_AND_VIDEO_ADS",
            CompanyServicesEnum::SMOBILEWEBSITESERVICES => "S_MOBILE_WEBSITE_SERVICES",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyServicesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVICE_UNSPECIFIED" => Ok(CompanyServicesEnum::SERVICEUNSPECIFIED),
           "S_ADVANCED_ADWORDS_SUPPORT" => Ok(CompanyServicesEnum::SADVANCEDADWORDSSUPPORT),
           "S_ADVERTISING_ON_GOOGLE" => Ok(CompanyServicesEnum::SADVERTISINGONGOOGLE),
           "S_AN_ENHANCED_WEBSITE" => Ok(CompanyServicesEnum::SANENHANCEDWEBSITE),
           "S_AN_ONLINE_MARKETING_PLAN" => Ok(CompanyServicesEnum::SANONLINEMARKETINGPLAN),
           "S_MOBILE_AND_VIDEO_ADS" => Ok(CompanyServicesEnum::SMOBILEANDVIDEOADS),
           "S_MOBILE_WEBSITE_SERVICES" => Ok(CompanyServicesEnum::SMOBILEWEBSITESERVICES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyServicesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyBadgeTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Partner badge tier
pub enum CompanyBadgeTierEnum {
    

    /// Tier badge is not set.
    ///
    /// "BADGE_TIER_NONE"
    #[serde(rename="BADGE_TIER_NONE")]
    BADGETIERNONE,
    

    /// Agency has regular partner badge.
    ///
    /// "BADGE_TIER_REGULAR"
    #[serde(rename="BADGE_TIER_REGULAR")]
    BADGETIERREGULAR,
    

    /// Agency has premier badge.
    ///
    /// "BADGE_TIER_PREMIER"
    #[serde(rename="BADGE_TIER_PREMIER")]
    BADGETIERPREMIER,
}

impl AsRef<str> for CompanyBadgeTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyBadgeTierEnum::BADGETIERNONE => "BADGE_TIER_NONE",
            CompanyBadgeTierEnum::BADGETIERREGULAR => "BADGE_TIER_REGULAR",
            CompanyBadgeTierEnum::BADGETIERPREMIER => "BADGE_TIER_PREMIER",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyBadgeTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BADGE_TIER_NONE" => Ok(CompanyBadgeTierEnum::BADGETIERNONE),
           "BADGE_TIER_REGULAR" => Ok(CompanyBadgeTierEnum::BADGETIERREGULAR),
           "BADGE_TIER_PREMIER" => Ok(CompanyBadgeTierEnum::BADGETIERPREMIER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyBadgeTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyCompanyTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Company type labels listed on the company's profile.
pub enum CompanyCompanyTypesEnum {
    

    /// Unchosen.
    ///
    /// "COMPANY_TYPE_UNSPECIFIED"
    #[serde(rename="COMPANY_TYPE_UNSPECIFIED")]
    COMPANYTYPEUNSPECIFIED,
    

    /// Handles all aspects of the advertising process.
    ///
    /// "FULL_SERVICE_AGENCY"
    #[serde(rename="FULL_SERVICE_AGENCY")]
    FULLSERVICEAGENCY,
    

    /// Focuses solely on an advertiser's media placement.
    ///
    /// "MEDIA_AGENCY"
    #[serde(rename="MEDIA_AGENCY")]
    MEDIAAGENCY,
    

    /// Plans/executes advertising campaigns.
    ///
    /// "CREATIVE_AGENCY"
    #[serde(rename="CREATIVE_AGENCY")]
    CREATIVEAGENCY,
    

    /// Like a
FULL_SERVICE_AGENCY,
but specializing in digital.
    ///
    /// "CDIGITAL_AGENCY"
    #[serde(rename="CDIGITAL_AGENCY")]
    CDIGITALAGENCY,
    

    /// Increases visibility in search engine result pages.
    ///
    /// "SEM_SEO"
    #[serde(rename="SEM_SEO")]
    SEMSEO,
    

    /// Drives promotional efforts for immediate impact.
    ///
    /// "PERFORMANCE_MARKETING"
    #[serde(rename="PERFORMANCE_MARKETING")]
    PERFORMANCEMARKETING,
    

    /// Focuses on bid management, conversion, reporting.
    ///
    /// "ADVERTISING_TOOL_DEVELOPMENT"
    #[serde(rename="ADVERTISING_TOOL_DEVELOPMENT")]
    ADVERTISINGTOOLDEVELOPMENT,
    

    /// Establishes favorable relationship with public through low/no-cost
communications.
    ///
    /// "PR"
    #[serde(rename="PR")]
    PR,
    

    /// Does not manage other company's accounts, manages own marketing programs.
    ///
    /// "SELF_MANAGED"
    #[serde(rename="SELF_MANAGED")]
    SELFMANAGED,
    

    /// Full-service AdWords account management for local businesses.
    ///
    /// "RESELLER"
    #[serde(rename="RESELLER")]
    RESELLER,
}

impl AsRef<str> for CompanyCompanyTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyCompanyTypesEnum::COMPANYTYPEUNSPECIFIED => "COMPANY_TYPE_UNSPECIFIED",
            CompanyCompanyTypesEnum::FULLSERVICEAGENCY => "FULL_SERVICE_AGENCY",
            CompanyCompanyTypesEnum::MEDIAAGENCY => "MEDIA_AGENCY",
            CompanyCompanyTypesEnum::CREATIVEAGENCY => "CREATIVE_AGENCY",
            CompanyCompanyTypesEnum::CDIGITALAGENCY => "CDIGITAL_AGENCY",
            CompanyCompanyTypesEnum::SEMSEO => "SEM_SEO",
            CompanyCompanyTypesEnum::PERFORMANCEMARKETING => "PERFORMANCE_MARKETING",
            CompanyCompanyTypesEnum::ADVERTISINGTOOLDEVELOPMENT => "ADVERTISING_TOOL_DEVELOPMENT",
            CompanyCompanyTypesEnum::PR => "PR",
            CompanyCompanyTypesEnum::SELFMANAGED => "SELF_MANAGED",
            CompanyCompanyTypesEnum::RESELLER => "RESELLER",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyCompanyTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPANY_TYPE_UNSPECIFIED" => Ok(CompanyCompanyTypesEnum::COMPANYTYPEUNSPECIFIED),
           "FULL_SERVICE_AGENCY" => Ok(CompanyCompanyTypesEnum::FULLSERVICEAGENCY),
           "MEDIA_AGENCY" => Ok(CompanyCompanyTypesEnum::MEDIAAGENCY),
           "CREATIVE_AGENCY" => Ok(CompanyCompanyTypesEnum::CREATIVEAGENCY),
           "CDIGITAL_AGENCY" => Ok(CompanyCompanyTypesEnum::CDIGITALAGENCY),
           "SEM_SEO" => Ok(CompanyCompanyTypesEnum::SEMSEO),
           "PERFORMANCE_MARKETING" => Ok(CompanyCompanyTypesEnum::PERFORMANCEMARKETING),
           "ADVERTISING_TOOL_DEVELOPMENT" => Ok(CompanyCompanyTypesEnum::ADVERTISINGTOOLDEVELOPMENT),
           "PR" => Ok(CompanyCompanyTypesEnum::PR),
           "SELF_MANAGED" => Ok(CompanyCompanyTypesEnum::SELFMANAGED),
           "RESELLER" => Ok(CompanyCompanyTypesEnum::RESELLER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyCompanyTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreateLeadResponseRecaptchaStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The outcome of <a href="https://www.google.com/recaptcha/">reCaptcha</a>
validation.
pub enum CreateLeadResponseRecaptchaStatusEnum {
    

    /// Unchosen.
    ///
    /// "RECAPTCHA_STATUS_UNSPECIFIED"
    #[serde(rename="RECAPTCHA_STATUS_UNSPECIFIED")]
    RECAPTCHASTATUSUNSPECIFIED,
    

    /// No reCaptcha validation needed.
    ///
    /// "RS_NOT_NEEDED"
    #[serde(rename="RS_NOT_NEEDED")]
    RSNOTNEEDED,
    

    /// reCaptcha challenge passed.
    ///
    /// "RS_PASSED"
    #[serde(rename="RS_PASSED")]
    RSPASSED,
    

    /// reCaptcha challenge failed.
    ///
    /// "RS_FAILED"
    #[serde(rename="RS_FAILED")]
    RSFAILED,
}

impl AsRef<str> for CreateLeadResponseRecaptchaStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateLeadResponseRecaptchaStatusEnum::RECAPTCHASTATUSUNSPECIFIED => "RECAPTCHA_STATUS_UNSPECIFIED",
            CreateLeadResponseRecaptchaStatusEnum::RSNOTNEEDED => "RS_NOT_NEEDED",
            CreateLeadResponseRecaptchaStatusEnum::RSPASSED => "RS_PASSED",
            CreateLeadResponseRecaptchaStatusEnum::RSFAILED => "RS_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateLeadResponseRecaptchaStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECAPTCHA_STATUS_UNSPECIFIED" => Ok(CreateLeadResponseRecaptchaStatusEnum::RECAPTCHASTATUSUNSPECIFIED),
           "RS_NOT_NEEDED" => Ok(CreateLeadResponseRecaptchaStatusEnum::RSNOTNEEDED),
           "RS_PASSED" => Ok(CreateLeadResponseRecaptchaStatusEnum::RSPASSED),
           "RS_FAILED" => Ok(CreateLeadResponseRecaptchaStatusEnum::RSFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateLeadResponseRecaptchaStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificationExamStatusTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of certification exam.
pub enum CertificationExamStatusTypeEnum {
    

    /// Unchosen.
    ///
    /// "CERTIFICATION_EXAM_TYPE_UNSPECIFIED"
    #[serde(rename="CERTIFICATION_EXAM_TYPE_UNSPECIFIED")]
    CERTIFICATIONEXAMTYPEUNSPECIFIED,
    

    /// Adwords Fundamentals exam.
    ///
    /// "CET_ADWORDS_FUNDAMENTALS"
    #[serde(rename="CET_ADWORDS_FUNDAMENTALS")]
    CETADWORDSFUNDAMENTALS,
    

    /// AdWords advanced search exam.
    ///
    /// "CET_ADWORDS_ADVANCED_SEARCH"
    #[serde(rename="CET_ADWORDS_ADVANCED_SEARCH")]
    CETADWORDSADVANCEDSEARCH,
    

    /// AdWords advanced display exam.
    ///
    /// "CET_ADWORDS_ADVANCED_DISPLAY"
    #[serde(rename="CET_ADWORDS_ADVANCED_DISPLAY")]
    CETADWORDSADVANCEDDISPLAY,
    

    /// VideoAds exam.
    ///
    /// "CET_VIDEO_ADS"
    #[serde(rename="CET_VIDEO_ADS")]
    CETVIDEOADS,
    

    /// DoubleClick exam.
    ///
    /// "CET_DOUBLECLICK"
    #[serde(rename="CET_DOUBLECLICK")]
    CETDOUBLECLICK,
    

    /// Analytics exam.
    ///
    /// "CET_ANALYTICS"
    #[serde(rename="CET_ANALYTICS")]
    CETANALYTICS,
    

    /// Shopping exam.
    ///
    /// "CET_SHOPPING"
    #[serde(rename="CET_SHOPPING")]
    CETSHOPPING,
    

    /// Mobile exam.
    ///
    /// "CET_MOBILE"
    #[serde(rename="CET_MOBILE")]
    CETMOBILE,
    

    /// Digital Sales exam.
    ///
    /// "CET_DIGITAL_SALES"
    #[serde(rename="CET_DIGITAL_SALES")]
    CETDIGITALSALES,
    

    /// Mobile Sites exam.
    ///
    /// "CET_MOBILE_SITES"
    #[serde(rename="CET_MOBILE_SITES")]
    CETMOBILESITES,
}

impl AsRef<str> for CertificationExamStatusTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificationExamStatusTypeEnum::CERTIFICATIONEXAMTYPEUNSPECIFIED => "CERTIFICATION_EXAM_TYPE_UNSPECIFIED",
            CertificationExamStatusTypeEnum::CETADWORDSFUNDAMENTALS => "CET_ADWORDS_FUNDAMENTALS",
            CertificationExamStatusTypeEnum::CETADWORDSADVANCEDSEARCH => "CET_ADWORDS_ADVANCED_SEARCH",
            CertificationExamStatusTypeEnum::CETADWORDSADVANCEDDISPLAY => "CET_ADWORDS_ADVANCED_DISPLAY",
            CertificationExamStatusTypeEnum::CETVIDEOADS => "CET_VIDEO_ADS",
            CertificationExamStatusTypeEnum::CETDOUBLECLICK => "CET_DOUBLECLICK",
            CertificationExamStatusTypeEnum::CETANALYTICS => "CET_ANALYTICS",
            CertificationExamStatusTypeEnum::CETSHOPPING => "CET_SHOPPING",
            CertificationExamStatusTypeEnum::CETMOBILE => "CET_MOBILE",
            CertificationExamStatusTypeEnum::CETDIGITALSALES => "CET_DIGITAL_SALES",
            CertificationExamStatusTypeEnum::CETMOBILESITES => "CET_MOBILE_SITES",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificationExamStatusTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERTIFICATION_EXAM_TYPE_UNSPECIFIED" => Ok(CertificationExamStatusTypeEnum::CERTIFICATIONEXAMTYPEUNSPECIFIED),
           "CET_ADWORDS_FUNDAMENTALS" => Ok(CertificationExamStatusTypeEnum::CETADWORDSFUNDAMENTALS),
           "CET_ADWORDS_ADVANCED_SEARCH" => Ok(CertificationExamStatusTypeEnum::CETADWORDSADVANCEDSEARCH),
           "CET_ADWORDS_ADVANCED_DISPLAY" => Ok(CertificationExamStatusTypeEnum::CETADWORDSADVANCEDDISPLAY),
           "CET_VIDEO_ADS" => Ok(CertificationExamStatusTypeEnum::CETVIDEOADS),
           "CET_DOUBLECLICK" => Ok(CertificationExamStatusTypeEnum::CETDOUBLECLICK),
           "CET_ANALYTICS" => Ok(CertificationExamStatusTypeEnum::CETANALYTICS),
           "CET_SHOPPING" => Ok(CertificationExamStatusTypeEnum::CETSHOPPING),
           "CET_MOBILE" => Ok(CertificationExamStatusTypeEnum::CETMOBILE),
           "CET_DIGITAL_SALES" => Ok(CertificationExamStatusTypeEnum::CETDIGITALSALES),
           "CET_MOBILE_SITES" => Ok(CertificationExamStatusTypeEnum::CETMOBILESITES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificationExamStatusTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RankTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of rank.
pub enum RankTypeEnum {
    

    /// Unchosen.
    ///
    /// "RANK_TYPE_UNSPECIFIED"
    #[serde(rename="RANK_TYPE_UNSPECIFIED")]
    RANKTYPEUNSPECIFIED,
    

    /// Total final score.
    ///
    /// "RT_FINAL_SCORE"
    #[serde(rename="RT_FINAL_SCORE")]
    RTFINALSCORE,
}

impl AsRef<str> for RankTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RankTypeEnum::RANKTYPEUNSPECIFIED => "RANK_TYPE_UNSPECIFIED",
            RankTypeEnum::RTFINALSCORE => "RT_FINAL_SCORE",
        }
    }
}

impl std::convert::TryFrom< &str> for RankTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RANK_TYPE_UNSPECIFIED" => Ok(RankTypeEnum::RANKTYPEUNSPECIFIED),
           "RT_FINAL_SCORE" => Ok(RankTypeEnum::RTFINALSCORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RankTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HistoricalOfferStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of the offer.
pub enum HistoricalOfferStatusEnum {
    

    /// Unset.
    ///
    /// "OFFER_STATUS_UNSPECIFIED"
    #[serde(rename="OFFER_STATUS_UNSPECIFIED")]
    OFFERSTATUSUNSPECIFIED,
    

    /// Offer distributed.
    ///
    /// "OFFER_STATUS_DISTRIBUTED"
    #[serde(rename="OFFER_STATUS_DISTRIBUTED")]
    OFFERSTATUSDISTRIBUTED,
    

    /// Offer redeemed.
    ///
    /// "OFFER_STATUS_REDEEMED"
    #[serde(rename="OFFER_STATUS_REDEEMED")]
    OFFERSTATUSREDEEMED,
    

    /// Offer awarded.
    ///
    /// "OFFER_STATUS_AWARDED"
    #[serde(rename="OFFER_STATUS_AWARDED")]
    OFFERSTATUSAWARDED,
    

    /// Offer expired.
    ///
    /// "OFFER_STATUS_EXPIRED"
    #[serde(rename="OFFER_STATUS_EXPIRED")]
    OFFERSTATUSEXPIRED,
}

impl AsRef<str> for HistoricalOfferStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HistoricalOfferStatusEnum::OFFERSTATUSUNSPECIFIED => "OFFER_STATUS_UNSPECIFIED",
            HistoricalOfferStatusEnum::OFFERSTATUSDISTRIBUTED => "OFFER_STATUS_DISTRIBUTED",
            HistoricalOfferStatusEnum::OFFERSTATUSREDEEMED => "OFFER_STATUS_REDEEMED",
            HistoricalOfferStatusEnum::OFFERSTATUSAWARDED => "OFFER_STATUS_AWARDED",
            HistoricalOfferStatusEnum::OFFERSTATUSEXPIRED => "OFFER_STATUS_EXPIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for HistoricalOfferStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFFER_STATUS_UNSPECIFIED" => Ok(HistoricalOfferStatusEnum::OFFERSTATUSUNSPECIFIED),
           "OFFER_STATUS_DISTRIBUTED" => Ok(HistoricalOfferStatusEnum::OFFERSTATUSDISTRIBUTED),
           "OFFER_STATUS_REDEEMED" => Ok(HistoricalOfferStatusEnum::OFFERSTATUSREDEEMED),
           "OFFER_STATUS_AWARDED" => Ok(HistoricalOfferStatusEnum::OFFERSTATUSAWARDED),
           "OFFER_STATUS_EXPIRED" => Ok(HistoricalOfferStatusEnum::OFFERSTATUSEXPIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HistoricalOfferStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HistoricalOfferOfferTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of offer.
pub enum HistoricalOfferOfferTypeEnum {
    

    /// Unset.
    ///
    /// "OFFER_TYPE_UNSPECIFIED"
    #[serde(rename="OFFER_TYPE_UNSPECIFIED")]
    OFFERTYPEUNSPECIFIED,
    

    /// AdWords spend X get Y.
    ///
    /// "OFFER_TYPE_SPEND_X_GET_Y"
    #[serde(rename="OFFER_TYPE_SPEND_X_GET_Y")]
    OFFERTYPESPENDXGETY,
    

    /// Youtube video.
    ///
    /// "OFFER_TYPE_VIDEO"
    #[serde(rename="OFFER_TYPE_VIDEO")]
    OFFERTYPEVIDEO,
    

    /// Spend Match up to Y.
    ///
    /// "OFFER_TYPE_SPEND_MATCH"
    #[serde(rename="OFFER_TYPE_SPEND_MATCH")]
    OFFERTYPESPENDMATCH,
}

impl AsRef<str> for HistoricalOfferOfferTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HistoricalOfferOfferTypeEnum::OFFERTYPEUNSPECIFIED => "OFFER_TYPE_UNSPECIFIED",
            HistoricalOfferOfferTypeEnum::OFFERTYPESPENDXGETY => "OFFER_TYPE_SPEND_X_GET_Y",
            HistoricalOfferOfferTypeEnum::OFFERTYPEVIDEO => "OFFER_TYPE_VIDEO",
            HistoricalOfferOfferTypeEnum::OFFERTYPESPENDMATCH => "OFFER_TYPE_SPEND_MATCH",
        }
    }
}

impl std::convert::TryFrom< &str> for HistoricalOfferOfferTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFFER_TYPE_UNSPECIFIED" => Ok(HistoricalOfferOfferTypeEnum::OFFERTYPEUNSPECIFIED),
           "OFFER_TYPE_SPEND_X_GET_Y" => Ok(HistoricalOfferOfferTypeEnum::OFFERTYPESPENDXGETY),
           "OFFER_TYPE_VIDEO" => Ok(HistoricalOfferOfferTypeEnum::OFFERTYPEVIDEO),
           "OFFER_TYPE_SPEND_MATCH" => Ok(HistoricalOfferOfferTypeEnum::OFFERTYPESPENDMATCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HistoricalOfferOfferTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LogUserEventRequestEventScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The scope of the event.
pub enum LogUserEventRequestEventScopeEnum {
    

    /// Unchosen.
    ///
    /// "EVENT_SCOPE_UNSPECIFIED"
    #[serde(rename="EVENT_SCOPE_UNSPECIFIED")]
    EVENTSCOPEUNSPECIFIED,
    

    /// Based on visitor.
    ///
    /// "VISITOR"
    #[serde(rename="VISITOR")]
    VISITOR,
    

    /// Based on session.
    ///
    /// "SESSION"
    #[serde(rename="SESSION")]
    SESSION,
    

    /// Based on page visit.
    ///
    /// "PAGE"
    #[serde(rename="PAGE")]
    PAGE,
}

impl AsRef<str> for LogUserEventRequestEventScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LogUserEventRequestEventScopeEnum::EVENTSCOPEUNSPECIFIED => "EVENT_SCOPE_UNSPECIFIED",
            LogUserEventRequestEventScopeEnum::VISITOR => "VISITOR",
            LogUserEventRequestEventScopeEnum::SESSION => "SESSION",
            LogUserEventRequestEventScopeEnum::PAGE => "PAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for LogUserEventRequestEventScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_SCOPE_UNSPECIFIED" => Ok(LogUserEventRequestEventScopeEnum::EVENTSCOPEUNSPECIFIED),
           "VISITOR" => Ok(LogUserEventRequestEventScopeEnum::VISITOR),
           "SESSION" => Ok(LogUserEventRequestEventScopeEnum::SESSION),
           "PAGE" => Ok(LogUserEventRequestEventScopeEnum::PAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LogUserEventRequestEventScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LogUserEventRequestEventCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The category the action belongs to.
pub enum LogUserEventRequestEventCategoryEnum {
    

    /// Unchosen.
    ///
    /// "EVENT_CATEGORY_UNSPECIFIED"
    #[serde(rename="EVENT_CATEGORY_UNSPECIFIED")]
    EVENTCATEGORYUNSPECIFIED,
    

    /// Google Partner Search category.
    ///
    /// "GOOGLE_PARTNER_SEARCH"
    #[serde(rename="GOOGLE_PARTNER_SEARCH")]
    GOOGLEPARTNERSEARCH,
    

    /// Google Partner sign-up flow category.
    ///
    /// "GOOGLE_PARTNER_SIGNUP_FLOW"
    #[serde(rename="GOOGLE_PARTNER_SIGNUP_FLOW")]
    GOOGLEPARTNERSIGNUPFLOW,
    

    /// Google Partner portal category.
    ///
    /// "GOOGLE_PARTNER_PORTAL"
    #[serde(rename="GOOGLE_PARTNER_PORTAL")]
    GOOGLEPARTNERPORTAL,
    

    /// Google Partner portal my-profile category.
    ///
    /// "GOOGLE_PARTNER_PORTAL_MY_PROFILE"
    #[serde(rename="GOOGLE_PARTNER_PORTAL_MY_PROFILE")]
    GOOGLEPARTNERPORTALMYPROFILE,
    

    /// Google Partner portal certifications category.
    ///
    /// "GOOGLE_PARTNER_PORTAL_CERTIFICATIONS"
    #[serde(rename="GOOGLE_PARTNER_PORTAL_CERTIFICATIONS")]
    GOOGLEPARTNERPORTALCERTIFICATIONS,
    

    /// Google Partner portal community category.
    ///
    /// "GOOGLE_PARTNER_PORTAL_COMMUNITY"
    #[serde(rename="GOOGLE_PARTNER_PORTAL_COMMUNITY")]
    GOOGLEPARTNERPORTALCOMMUNITY,
    

    /// Google Partner portal insights category.
    ///
    /// "GOOGLE_PARTNER_PORTAL_INSIGHTS"
    #[serde(rename="GOOGLE_PARTNER_PORTAL_INSIGHTS")]
    GOOGLEPARTNERPORTALINSIGHTS,
    

    /// Google Partner portal clients category.
    ///
    /// "GOOGLE_PARTNER_PORTAL_CLIENTS"
    #[serde(rename="GOOGLE_PARTNER_PORTAL_CLIENTS")]
    GOOGLEPARTNERPORTALCLIENTS,
    

    /// Google Partner portal public user profile category.
    ///
    /// "GOOGLE_PARTNER_PUBLIC_USER_PROFILE"
    #[serde(rename="GOOGLE_PARTNER_PUBLIC_USER_PROFILE")]
    GOOGLEPARTNERPUBLICUSERPROFILE,
    

    /// Google Partner panel category.
    ///
    /// "GOOGLE_PARTNER_PANEL"
    #[serde(rename="GOOGLE_PARTNER_PANEL")]
    GOOGLEPARTNERPANEL,
    

    /// Google Partner portal last admin dialog category.
    ///
    /// "GOOGLE_PARTNER_PORTAL_LAST_ADMIN_DIALOG"
    #[serde(rename="GOOGLE_PARTNER_PORTAL_LAST_ADMIN_DIALOG")]
    GOOGLEPARTNERPORTALLASTADMINDIALOG,
    

    /// Google Partner client category.
    ///
    /// "GOOGLE_PARTNER_CLIENT"
    #[serde(rename="GOOGLE_PARTNER_CLIENT")]
    GOOGLEPARTNERCLIENT,
    

    /// Google Partner portal company profile category.
    ///
    /// "GOOGLE_PARTNER_PORTAL_COMPANY_PROFILE"
    #[serde(rename="GOOGLE_PARTNER_PORTAL_COMPANY_PROFILE")]
    GOOGLEPARTNERPORTALCOMPANYPROFILE,
    

    /// External links category.
    ///
    /// "EXTERNAL_LINKS"
    #[serde(rename="EXTERNAL_LINKS")]
    EXTERNALLINKS,
    

    /// Google Partner landing category.
    ///
    /// "GOOGLE_PARTNER_LANDING"
    #[serde(rename="GOOGLE_PARTNER_LANDING")]
    GOOGLEPARTNERLANDING,
}

impl AsRef<str> for LogUserEventRequestEventCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LogUserEventRequestEventCategoryEnum::EVENTCATEGORYUNSPECIFIED => "EVENT_CATEGORY_UNSPECIFIED",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERSEARCH => "GOOGLE_PARTNER_SEARCH",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERSIGNUPFLOW => "GOOGLE_PARTNER_SIGNUP_FLOW",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTAL => "GOOGLE_PARTNER_PORTAL",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALMYPROFILE => "GOOGLE_PARTNER_PORTAL_MY_PROFILE",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALCERTIFICATIONS => "GOOGLE_PARTNER_PORTAL_CERTIFICATIONS",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALCOMMUNITY => "GOOGLE_PARTNER_PORTAL_COMMUNITY",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALINSIGHTS => "GOOGLE_PARTNER_PORTAL_INSIGHTS",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALCLIENTS => "GOOGLE_PARTNER_PORTAL_CLIENTS",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPUBLICUSERPROFILE => "GOOGLE_PARTNER_PUBLIC_USER_PROFILE",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPANEL => "GOOGLE_PARTNER_PANEL",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALLASTADMINDIALOG => "GOOGLE_PARTNER_PORTAL_LAST_ADMIN_DIALOG",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERCLIENT => "GOOGLE_PARTNER_CLIENT",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALCOMPANYPROFILE => "GOOGLE_PARTNER_PORTAL_COMPANY_PROFILE",
            LogUserEventRequestEventCategoryEnum::EXTERNALLINKS => "EXTERNAL_LINKS",
            LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERLANDING => "GOOGLE_PARTNER_LANDING",
        }
    }
}

impl std::convert::TryFrom< &str> for LogUserEventRequestEventCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_CATEGORY_UNSPECIFIED" => Ok(LogUserEventRequestEventCategoryEnum::EVENTCATEGORYUNSPECIFIED),
           "GOOGLE_PARTNER_SEARCH" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERSEARCH),
           "GOOGLE_PARTNER_SIGNUP_FLOW" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERSIGNUPFLOW),
           "GOOGLE_PARTNER_PORTAL" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTAL),
           "GOOGLE_PARTNER_PORTAL_MY_PROFILE" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALMYPROFILE),
           "GOOGLE_PARTNER_PORTAL_CERTIFICATIONS" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALCERTIFICATIONS),
           "GOOGLE_PARTNER_PORTAL_COMMUNITY" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALCOMMUNITY),
           "GOOGLE_PARTNER_PORTAL_INSIGHTS" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALINSIGHTS),
           "GOOGLE_PARTNER_PORTAL_CLIENTS" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALCLIENTS),
           "GOOGLE_PARTNER_PUBLIC_USER_PROFILE" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPUBLICUSERPROFILE),
           "GOOGLE_PARTNER_PANEL" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPANEL),
           "GOOGLE_PARTNER_PORTAL_LAST_ADMIN_DIALOG" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALLASTADMINDIALOG),
           "GOOGLE_PARTNER_CLIENT" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERCLIENT),
           "GOOGLE_PARTNER_PORTAL_COMPANY_PROFILE" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERPORTALCOMPANYPROFILE),
           "EXTERNAL_LINKS" => Ok(LogUserEventRequestEventCategoryEnum::EXTERNALLINKS),
           "GOOGLE_PARTNER_LANDING" => Ok(LogUserEventRequestEventCategoryEnum::GOOGLEPARTNERLANDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LogUserEventRequestEventCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LogUserEventRequestEventActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The action that occurred.
pub enum LogUserEventRequestEventActionEnum {
    

    /// Unchosen.
    ///
    /// "EVENT_ACTION_UNSPECIFIED"
    #[serde(rename="EVENT_ACTION_UNSPECIFIED")]
    EVENTACTIONUNSPECIFIED,
    

    /// Advertiser clicked `Find a partner` bottom button.
    ///
    /// "SMB_CLICKED_FIND_A_PARTNER_BUTTON_BOTTOM"
    #[serde(rename="SMB_CLICKED_FIND_A_PARTNER_BUTTON_BOTTOM")]
    SMBCLICKEDFINDAPARTNERBUTTONBOTTOM,
    

    /// Advertiser clicked `Find a partner` top button.
    ///
    /// "SMB_CLICKED_FIND_A_PARTNER_BUTTON_TOP"
    #[serde(rename="SMB_CLICKED_FIND_A_PARTNER_BUTTON_TOP")]
    SMBCLICKEDFINDAPARTNERBUTTONTOP,
    

    /// Agency clicked `Join now` bottom button.
    ///
    /// "AGENCY_CLICKED_JOIN_NOW_BUTTON_BOTTOM"
    #[serde(rename="AGENCY_CLICKED_JOIN_NOW_BUTTON_BOTTOM")]
    AGENCYCLICKEDJOINNOWBUTTONBOTTOM,
    

    /// Agency clicked `Join now` top button.
    ///
    /// "AGENCY_CLICKED_JOIN_NOW_BUTTON_TOP"
    #[serde(rename="AGENCY_CLICKED_JOIN_NOW_BUTTON_TOP")]
    AGENCYCLICKEDJOINNOWBUTTONTOP,
    

    /// Advertiser canceled partner contact form.
    ///
    /// "SMB_CANCELED_PARTNER_CONTACT_FORM"
    #[serde(rename="SMB_CANCELED_PARTNER_CONTACT_FORM")]
    SMBCANCELEDPARTNERCONTACTFORM,
    

    /// Advertiser started partner contact form.
    ///
    /// "SMB_CLICKED_CONTACT_A_PARTNER"
    #[serde(rename="SMB_CLICKED_CONTACT_A_PARTNER")]
    SMBCLICKEDCONTACTAPARTNER,
    

    /// Advertiser completed partner contact form.
    ///
    /// "SMB_COMPLETED_PARTNER_CONTACT_FORM"
    #[serde(rename="SMB_COMPLETED_PARTNER_CONTACT_FORM")]
    SMBCOMPLETEDPARTNERCONTACTFORM,
    

    /// Advertiser entered email in contact form.
    ///
    /// "SMB_ENTERED_EMAIL_IN_CONTACT_PARTNER_FORM"
    #[serde(rename="SMB_ENTERED_EMAIL_IN_CONTACT_PARTNER_FORM")]
    SMBENTEREDEMAILINCONTACTPARTNERFORM,
    

    /// Advertiser entered name in contact form.
    ///
    /// "SMB_ENTERED_NAME_IN_CONTACT_PARTNER_FORM"
    #[serde(rename="SMB_ENTERED_NAME_IN_CONTACT_PARTNER_FORM")]
    SMBENTEREDNAMEINCONTACTPARTNERFORM,
    

    /// Advertiser entered phone in contact form.
    ///
    /// "SMB_ENTERED_PHONE_IN_CONTACT_PARTNER_FORM"
    #[serde(rename="SMB_ENTERED_PHONE_IN_CONTACT_PARTNER_FORM")]
    SMBENTEREDPHONEINCONTACTPARTNERFORM,
    

    /// Advertiser failed <a href="https://www.google.com/recaptcha/">reCaptcha</a>
in contact form.
    ///
    /// "SMB_FAILED_RECAPTCHA_IN_CONTACT_PARTNER_FORM"
    #[serde(rename="SMB_FAILED_RECAPTCHA_IN_CONTACT_PARTNER_FORM")]
    SMBFAILEDRECAPTCHAINCONTACTPARTNERFORM,
    

    /// Company viewed by advertiser.
    ///
    /// "PARTNER_VIEWED_BY_SMB"
    #[serde(rename="PARTNER_VIEWED_BY_SMB")]
    PARTNERVIEWEDBYSMB,
    

    /// Advertiser canceled partner contact form on Google Partner Search.
    ///
    /// "SMB_CANCELED_PARTNER_CONTACT_FORM_ON_GPS"
    #[serde(rename="SMB_CANCELED_PARTNER_CONTACT_FORM_ON_GPS")]
    SMBCANCELEDPARTNERCONTACTFORMONGPS,
    

    /// Advertiser changed a top search parameter.
    ///
    /// "SMB_CHANGED_A_SEARCH_PARAMETER_TOP"
    #[serde(rename="SMB_CHANGED_A_SEARCH_PARAMETER_TOP")]
    SMBCHANGEDASEARCHPARAMETERTOP,
    

    /// Advertiser started partner contact form on Google Partner Search.
    ///
    /// "SMB_CLICKED_CONTACT_A_PARTNER_ON_GPS"
    #[serde(rename="SMB_CLICKED_CONTACT_A_PARTNER_ON_GPS")]
    SMBCLICKEDCONTACTAPARTNERONGPS,
    

    /// Advertiser clicked `Show more partners` bottom button.
    ///
    /// "SMB_CLICKED_SHOW_MORE_PARTNERS_BUTTON_BOTTOM"
    #[serde(rename="SMB_CLICKED_SHOW_MORE_PARTNERS_BUTTON_BOTTOM")]
    SMBCLICKEDSHOWMOREPARTNERSBUTTONBOTTOM,
    

    /// Advertiser completed partner contact form on Google Partner Search.
    ///
    /// "SMB_COMPLETED_PARTNER_CONTACT_FORM_ON_GPS"
    #[serde(rename="SMB_COMPLETED_PARTNER_CONTACT_FORM_ON_GPS")]
    SMBCOMPLETEDPARTNERCONTACTFORMONGPS,
    

    /// Advertiser saw no partners available with search criteria.
    ///
    /// "SMB_NO_PARTNERS_AVAILABLE_WITH_SEARCH_CRITERIA"
    #[serde(rename="SMB_NO_PARTNERS_AVAILABLE_WITH_SEARCH_CRITERIA")]
    SMBNOPARTNERSAVAILABLEWITHSEARCHCRITERIA,
    

    /// Advertiser performed search on Google Partner Search.
    ///
    /// "SMB_PERFORMED_SEARCH_ON_GPS"
    #[serde(rename="SMB_PERFORMED_SEARCH_ON_GPS")]
    SMBPERFORMEDSEARCHONGPS,
    

    /// Advertiser viewed a partner on Google Partner Search.
    ///
    /// "SMB_VIEWED_A_PARTNER_ON_GPS"
    #[serde(rename="SMB_VIEWED_A_PARTNER_ON_GPS")]
    SMBVIEWEDAPARTNERONGPS,
    

    /// Advertiser canceled partner contact form on profile page.
    ///
    /// "SMB_CANCELED_PARTNER_CONTACT_FORM_ON_PROFILE_PAGE"
    #[serde(rename="SMB_CANCELED_PARTNER_CONTACT_FORM_ON_PROFILE_PAGE")]
    SMBCANCELEDPARTNERCONTACTFORMONPROFILEPAGE,
    

    /// Advertiser started partner contact form on profile page.
    ///
    /// "SMB_CLICKED_CONTACT_A_PARTNER_ON_PROFILE_PAGE"
    #[serde(rename="SMB_CLICKED_CONTACT_A_PARTNER_ON_PROFILE_PAGE")]
    SMBCLICKEDCONTACTAPARTNERONPROFILEPAGE,
    

    /// Advertiser clicked partner website.
    ///
    /// "SMB_CLICKED_PARTNER_WEBSITE"
    #[serde(rename="SMB_CLICKED_PARTNER_WEBSITE")]
    SMBCLICKEDPARTNERWEBSITE,
    

    /// Advertiser completed contact form on profile page.
    ///
    /// "SMB_COMPLETED_PARTNER_CONTACT_FORM_ON_PROFILE_PAGE"
    #[serde(rename="SMB_COMPLETED_PARTNER_CONTACT_FORM_ON_PROFILE_PAGE")]
    SMBCOMPLETEDPARTNERCONTACTFORMONPROFILEPAGE,
    

    /// Advertiser viewed a partner profile.
    ///
    /// "SMB_VIEWED_A_PARTNER_PROFILE"
    #[serde(rename="SMB_VIEWED_A_PARTNER_PROFILE")]
    SMBVIEWEDAPARTNERPROFILE,
    

    /// Agency clicked `accept Terms Of Service` button.
    ///
    /// "AGENCY_CLICKED_ACCEPT_TOS_BUTTON"
    #[serde(rename="AGENCY_CLICKED_ACCEPT_TOS_BUTTON")]
    AGENCYCLICKEDACCEPTTOSBUTTON,
    

    /// Agency changed Terms Of Service country.
    ///
    /// "AGENCY_CHANGED_TOS_COUNTRY"
    #[serde(rename="AGENCY_CHANGED_TOS_COUNTRY")]
    AGENCYCHANGEDTOSCOUNTRY,
    

    /// Agency added address in profile portal.
    ///
    /// "AGENCY_ADDED_ADDRESS_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_ADDED_ADDRESS_IN_MY_PROFILE_PORTAL")]
    AGENCYADDEDADDRESSINMYPROFILEPORTAL,
    

    /// Agency added phone number in profile portal.
    ///
    /// "AGENCY_ADDED_PHONE_NUMBER_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_ADDED_PHONE_NUMBER_IN_MY_PROFILE_PORTAL")]
    AGENCYADDEDPHONENUMBERINMYPROFILEPORTAL,
    

    /// Agency changed primary account association.
    ///
    /// "AGENCY_CHANGED_PRIMARY_ACCOUNT_ASSOCIATION"
    #[serde(rename="AGENCY_CHANGED_PRIMARY_ACCOUNT_ASSOCIATION")]
    AGENCYCHANGEDPRIMARYACCOUNTASSOCIATION,
    

    /// Agency changed primary country association.
    ///
    /// "AGENCY_CHANGED_PRIMARY_COUNTRY_ASSOCIATION"
    #[serde(rename="AGENCY_CHANGED_PRIMARY_COUNTRY_ASSOCIATION")]
    AGENCYCHANGEDPRIMARYCOUNTRYASSOCIATION,
    

    /// Agency clicked `affiliate` button in profile portal.
    ///
    /// "AGENCY_CLICKED_AFFILIATE_BUTTON_IN_MY_PROFILE_IN_PORTAL"
    #[serde(rename="AGENCY_CLICKED_AFFILIATE_BUTTON_IN_MY_PROFILE_IN_PORTAL")]
    AGENCYCLICKEDAFFILIATEBUTTONINMYPROFILEINPORTAL,
    

    /// Agency clicked `give edit access` in profile portal.
    ///
    /// "AGENCY_CLICKED_GIVE_EDIT_ACCESS_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_CLICKED_GIVE_EDIT_ACCESS_IN_MY_PROFILE_PORTAL")]
    AGENCYCLICKEDGIVEEDITACCESSINMYPROFILEPORTAL,
    

    /// Agency clicked `log out` in profile portal.
    ///
    /// "AGENCY_CLICKED_LOG_OUT_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_CLICKED_LOG_OUT_IN_MY_PROFILE_PORTAL")]
    AGENCYCLICKEDLOGOUTINMYPROFILEPORTAL,
    

    /// Agency clicked profile portal left nav.
    ///
    /// "AGENCY_CLICKED_MY_PROFILE_LEFT_NAV_IN_PORTAL"
    #[serde(rename="AGENCY_CLICKED_MY_PROFILE_LEFT_NAV_IN_PORTAL")]
    AGENCYCLICKEDMYPROFILELEFTNAVINPORTAL,
    

    /// Agency clicked `save and continue` at bottom of complete profile.
    ///
    /// "AGENCY_CLICKED_SAVE_AND_CONTINUE_AT_BOT_OF_COMPLETE_PROFILE"
    #[serde(rename="AGENCY_CLICKED_SAVE_AND_CONTINUE_AT_BOT_OF_COMPLETE_PROFILE")]
    AGENCYCLICKEDSAVEANDCONTINUEATBOTOFCOMPLETEPROFILE,
    

    /// Agency clicked `unaffiliate` in profile portal.
    ///
    /// "AGENCY_CLICKED_UNAFFILIATE_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_CLICKED_UNAFFILIATE_IN_MY_PROFILE_PORTAL")]
    AGENCYCLICKEDUNAFFILIATEINMYPROFILEPORTAL,
    

    /// Agency filled out company affiliation in profile portal.
    ///
    /// "AGENCY_FILLED_OUT_COMP_AFFILIATION_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_FILLED_OUT_COMP_AFFILIATION_IN_MY_PROFILE_PORTAL")]
    AGENCYFILLEDOUTCOMPAFFILIATIONINMYPROFILEPORTAL,
    

    /// Agency successfully connected with company in profile portal.
    ///
    /// "AGENCY_SUCCESSFULLY_CONNECTED_WITH_COMPANY_IN_MY_PROFILE"
    #[serde(rename="AGENCY_SUCCESSFULLY_CONNECTED_WITH_COMPANY_IN_MY_PROFILE")]
    AGENCYSUCCESSFULLYCONNECTEDWITHCOMPANYINMYPROFILE,
    

    /// Agency clicked create MCC in profile portal.
    ///
    /// "AGENCY_CLICKED_CREATE_MCC_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_CLICKED_CREATE_MCC_IN_MY_PROFILE_PORTAL")]
    AGENCYCLICKEDCREATEMCCINMYPROFILEPORTAL,
    

    /// Agency did not have an MCC associated on profile portal.
    ///
    /// "AGENCY_DIDNT_HAVE_AN_MCC_ASSOCIATED_ON_COMPLETE_PROFILE"
    #[serde(rename="AGENCY_DIDNT_HAVE_AN_MCC_ASSOCIATED_ON_COMPLETE_PROFILE")]
    AGENCYDIDNTHAVEANMCCASSOCIATEDONCOMPLETEPROFILE,
    

    /// Agency had an MCC associated on profile portal.
    ///
    /// "AGENCY_HAD_AN_MCC_ASSOCIATED_ON_COMPLETE_PROFILE"
    #[serde(rename="AGENCY_HAD_AN_MCC_ASSOCIATED_ON_COMPLETE_PROFILE")]
    AGENCYHADANMCCASSOCIATEDONCOMPLETEPROFILE,
    

    /// Agency added job function in profile portal.
    ///
    /// "AGENCY_ADDED_JOB_FUNCTION_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_ADDED_JOB_FUNCTION_IN_MY_PROFILE_PORTAL")]
    AGENCYADDEDJOBFUNCTIONINMYPROFILEPORTAL,
    

    /// Agency looked at job function drop-down.
    ///
    /// "AGENCY_LOOKED_AT_JOB_FUNCTION_DROP_DOWN"
    #[serde(rename="AGENCY_LOOKED_AT_JOB_FUNCTION_DROP_DOWN")]
    AGENCYLOOKEDATJOBFUNCTIONDROPDOWN,
    

    /// Agency selected `account manage` as job function.
    ///
    /// "AGENCY_SELECTED_ACCOUNT_MANAGER_AS_JOB_FUNCTION"
    #[serde(rename="AGENCY_SELECTED_ACCOUNT_MANAGER_AS_JOB_FUNCTION")]
    AGENCYSELECTEDACCOUNTMANAGERASJOBFUNCTION,
    

    /// Agency selected `account planner` as job function.
    ///
    /// "AGENCY_SELECTED_ACCOUNT_PLANNER_AS_JOB_FUNCTION"
    #[serde(rename="AGENCY_SELECTED_ACCOUNT_PLANNER_AS_JOB_FUNCTION")]
    AGENCYSELECTEDACCOUNTPLANNERASJOBFUNCTION,
    

    /// Agency selected `Analytics` as job function.
    ///
    /// "AGENCY_SELECTED_ANALYTICS_AS_JOB_FUNCTION"
    #[serde(rename="AGENCY_SELECTED_ANALYTICS_AS_JOB_FUNCTION")]
    AGENCYSELECTEDANALYTICSASJOBFUNCTION,
    

    /// Agency selected `creative` as job function.
    ///
    /// "AGENCY_SELECTED_CREATIVE_AS_JOB_FUNCTION"
    #[serde(rename="AGENCY_SELECTED_CREATIVE_AS_JOB_FUNCTION")]
    AGENCYSELECTEDCREATIVEASJOBFUNCTION,
    

    /// Agency selected `media buyer` as job function.
    ///
    /// "AGENCY_SELECTED_MEDIA_BUYER_AS_JOB_FUNCTION"
    #[serde(rename="AGENCY_SELECTED_MEDIA_BUYER_AS_JOB_FUNCTION")]
    AGENCYSELECTEDMEDIABUYERASJOBFUNCTION,
    

    /// Agency selected `media planner` as job function.
    ///
    /// "AGENCY_SELECTED_MEDIA_PLANNER_AS_JOB_FUNCTION"
    #[serde(rename="AGENCY_SELECTED_MEDIA_PLANNER_AS_JOB_FUNCTION")]
    AGENCYSELECTEDMEDIAPLANNERASJOBFUNCTION,
    

    /// Agency selected `other` as job function.
    ///
    /// "AGENCY_SELECTED_OTHER_AS_JOB_FUNCTION"
    #[serde(rename="AGENCY_SELECTED_OTHER_AS_JOB_FUNCTION")]
    AGENCYSELECTEDOTHERASJOBFUNCTION,
    

    /// Agency selected `production` as job function.
    ///
    /// "AGENCY_SELECTED_PRODUCTION_AS_JOB_FUNCTION"
    #[serde(rename="AGENCY_SELECTED_PRODUCTION_AS_JOB_FUNCTION")]
    AGENCYSELECTEDPRODUCTIONASJOBFUNCTION,
    

    /// Agency selected `SEO` as job function.
    ///
    /// "AGENCY_SELECTED_SEO_AS_JOB_FUNCTION"
    #[serde(rename="AGENCY_SELECTED_SEO_AS_JOB_FUNCTION")]
    AGENCYSELECTEDSEOASJOBFUNCTION,
    

    /// Agency selected `sales rep` as job function.
    ///
    /// "AGENCY_SELECTED_SALES_REP_AS_JOB_FUNCTION"
    #[serde(rename="AGENCY_SELECTED_SALES_REP_AS_JOB_FUNCTION")]
    AGENCYSELECTEDSALESREPASJOBFUNCTION,
    

    /// Agency selected `search specialist` as job function.
    ///
    /// "AGENCY_SELECTED_SEARCH_SPECIALIST_AS_JOB_FUNCTION"
    #[serde(rename="AGENCY_SELECTED_SEARCH_SPECIALIST_AS_JOB_FUNCTION")]
    AGENCYSELECTEDSEARCHSPECIALISTASJOBFUNCTION,
    

    /// Agency added channels in profile portal.
    ///
    /// "AGENCY_ADDED_CHANNELS_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_ADDED_CHANNELS_IN_MY_PROFILE_PORTAL")]
    AGENCYADDEDCHANNELSINMYPROFILEPORTAL,
    

    /// Agency looked at `add channel` drop-down.
    ///
    /// "AGENCY_LOOKED_AT_ADD_CHANNEL_DROP_DOWN"
    #[serde(rename="AGENCY_LOOKED_AT_ADD_CHANNEL_DROP_DOWN")]
    AGENCYLOOKEDATADDCHANNELDROPDOWN,
    

    /// Agency selected `cross channel` from add channel drop-down.
    ///
    /// "AGENCY_SELECTED_CROSS_CHANNEL_FROM_ADD_CHANNEL"
    #[serde(rename="AGENCY_SELECTED_CROSS_CHANNEL_FROM_ADD_CHANNEL")]
    AGENCYSELECTEDCROSSCHANNELFROMADDCHANNEL,
    

    /// Agency selected `display` from add channel drop-down.
    ///
    /// "AGENCY_SELECTED_DISPLAY_FROM_ADD_CHANNEL"
    #[serde(rename="AGENCY_SELECTED_DISPLAY_FROM_ADD_CHANNEL")]
    AGENCYSELECTEDDISPLAYFROMADDCHANNEL,
    

    /// Agency selected `mobile` from add channel drop-down.
    ///
    /// "AGENCY_SELECTED_MOBILE_FROM_ADD_CHANNEL"
    #[serde(rename="AGENCY_SELECTED_MOBILE_FROM_ADD_CHANNEL")]
    AGENCYSELECTEDMOBILEFROMADDCHANNEL,
    

    /// Agency selected `search` from add channel drop-down.
    ///
    /// "AGENCY_SELECTED_SEARCH_FROM_ADD_CHANNEL"
    #[serde(rename="AGENCY_SELECTED_SEARCH_FROM_ADD_CHANNEL")]
    AGENCYSELECTEDSEARCHFROMADDCHANNEL,
    

    /// Agency selected `social` from add channel drop-down.
    ///
    /// "AGENCY_SELECTED_SOCIAL_FROM_ADD_CHANNEL"
    #[serde(rename="AGENCY_SELECTED_SOCIAL_FROM_ADD_CHANNEL")]
    AGENCYSELECTEDSOCIALFROMADDCHANNEL,
    

    /// Agency selected `tools` from add channel drop-down.
    ///
    /// "AGENCY_SELECTED_TOOLS_FROM_ADD_CHANNEL"
    #[serde(rename="AGENCY_SELECTED_TOOLS_FROM_ADD_CHANNEL")]
    AGENCYSELECTEDTOOLSFROMADDCHANNEL,
    

    /// Agency selected `YouTube` from add channel drop-down.
    ///
    /// "AGENCY_SELECTED_YOUTUBE_FROM_ADD_CHANNEL"
    #[serde(rename="AGENCY_SELECTED_YOUTUBE_FROM_ADD_CHANNEL")]
    AGENCYSELECTEDYOUTUBEFROMADDCHANNEL,
    

    /// Agency added industries in profile portal.
    ///
    /// "AGENCY_ADDED_INDUSTRIES_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_ADDED_INDUSTRIES_IN_MY_PROFILE_PORTAL")]
    AGENCYADDEDINDUSTRIESINMYPROFILEPORTAL,
    

    /// Agency changed `add industries` drop-down.
    ///
    /// "AGENCY_CHANGED_ADD_INDUSTRIES_DROP_DOWN"
    #[serde(rename="AGENCY_CHANGED_ADD_INDUSTRIES_DROP_DOWN")]
    AGENCYCHANGEDADDINDUSTRIESDROPDOWN,
    

    /// Agency added markets in profile portal.
    ///
    /// "AGENCY_ADDED_MARKETS_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_ADDED_MARKETS_IN_MY_PROFILE_PORTAL")]
    AGENCYADDEDMARKETSINMYPROFILEPORTAL,
    

    /// Agency changed `add markets` drop-down.
    ///
    /// "AGENCY_CHANGED_ADD_MARKETS_DROP_DOWN"
    #[serde(rename="AGENCY_CHANGED_ADD_MARKETS_DROP_DOWN")]
    AGENCYCHANGEDADDMARKETSDROPDOWN,
    

    /// Agency checked `recieve mail promotions` in profile portal.
    ///
    /// "AGENCY_CHECKED_RECIEVE_MAIL_PROMOTIONS_MYPROFILE"
    #[serde(rename="AGENCY_CHECKED_RECIEVE_MAIL_PROMOTIONS_MYPROFILE")]
    AGENCYCHECKEDRECIEVEMAILPROMOTIONSMYPROFILE,
    

    /// Agency checked `recieve mail promotions` in sign-up.
    ///
    /// "AGENCY_CHECKED_RECIEVE_MAIL_PROMOTIONS_SIGNUP"
    #[serde(rename="AGENCY_CHECKED_RECIEVE_MAIL_PROMOTIONS_SIGNUP")]
    AGENCYCHECKEDRECIEVEMAILPROMOTIONSSIGNUP,
    

    /// Agency selected `opt-in beta tests and market research`.
    ///
    /// "AGENCY_SELECTED_OPT_IN_BETA_TESTS_AND_MKT_RESEARCH"
    #[serde(rename="AGENCY_SELECTED_OPT_IN_BETA_TESTS_AND_MKT_RESEARCH")]
    AGENCYSELECTEDOPTINBETATESTSANDMKTRESEARCH,
    

    /// Agency selected `opt-in beta tests` in profile portal.
    ///
    /// "AGENCY_SELECTED_OPT_IN_BETA_TESTS_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_SELECTED_OPT_IN_BETA_TESTS_IN_MY_PROFILE_PORTAL")]
    AGENCYSELECTEDOPTINBETATESTSINMYPROFILEPORTAL,
    

    /// Agency selected `opt-in news` in profile portal.
    ///
    /// "AGENCY_SELECTED_OPT_IN_NEWS_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_SELECTED_OPT_IN_NEWS_IN_MY_PROFILE_PORTAL")]
    AGENCYSELECTEDOPTINNEWSINMYPROFILEPORTAL,
    

    /// Agency selected `opt-in news invitations and promotions`.
    ///
    /// "AGENCY_SELECTED_OPT_IN_NEWS_INVITATIONS_AND_PROMOS"
    #[serde(rename="AGENCY_SELECTED_OPT_IN_NEWS_INVITATIONS_AND_PROMOS")]
    AGENCYSELECTEDOPTINNEWSINVITATIONSANDPROMOS,
    

    /// Agency selected `opt-in performance SUG` in profile portal.
    ///
    /// "AGENCY_SELECTED_OPT_IN_PERFORMANCE_SUG_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_SELECTED_OPT_IN_PERFORMANCE_SUG_IN_MY_PROFILE_PORTAL")]
    AGENCYSELECTEDOPTINPERFORMANCESUGINMYPROFILEPORTAL,
    

    /// Agency selected `opt-in performance suggestions`.
    ///
    /// "AGENCY_SELECTED_OPT_IN_PERFORMANCE_SUGGESTIONS"
    #[serde(rename="AGENCY_SELECTED_OPT_IN_PERFORMANCE_SUGGESTIONS")]
    AGENCYSELECTEDOPTINPERFORMANCESUGGESTIONS,
    

    /// Agency selected `opt-in select all email notifications`.
    ///
    /// "AGENCY_SELECTED_OPT_IN_SELECT_ALL_EMAIL_NOTIFICATIONS"
    #[serde(rename="AGENCY_SELECTED_OPT_IN_SELECT_ALL_EMAIL_NOTIFICATIONS")]
    AGENCYSELECTEDOPTINSELECTALLEMAILNOTIFICATIONS,
    

    /// Agency selected `select all opt-ins` in profile portal.
    ///
    /// "AGENCY_SELECTED_SELECT_ALL_OPT_INS_IN_MY_PROFILE_PORTAL"
    #[serde(rename="AGENCY_SELECTED_SELECT_ALL_OPT_INS_IN_MY_PROFILE_PORTAL")]
    AGENCYSELECTEDSELECTALLOPTINSINMYPROFILEPORTAL,
    

    /// Agency clicked back button on `connect with company`.
    ///
    /// "AGENCY_CLICKED_BACK_BUTTON_ON_CONNECT_WITH_COMPANY"
    #[serde(rename="AGENCY_CLICKED_BACK_BUTTON_ON_CONNECT_WITH_COMPANY")]
    AGENCYCLICKEDBACKBUTTONONCONNECTWITHCOMPANY,
    

    /// Agency clicked continue to overview on `connect with company`.
    ///
    /// "AGENCY_CLICKED_CONTINUE_TO_OVERVIEW_ON_CONNECT_WITH_COMPANY"
    #[serde(rename="AGENCY_CLICKED_CONTINUE_TO_OVERVIEW_ON_CONNECT_WITH_COMPANY")]
    AGENCYCLICKEDCONTINUETOOVERVIEWONCONNECTWITHCOMPANY,
    

    /// Agency clicked `create MCC connect with company not found`.
    ///
    /// "AGECNY_CLICKED_CREATE_MCC_CONNECT_WITH_COMPANY_NOT_FOUND"
    #[serde(rename="AGECNY_CLICKED_CREATE_MCC_CONNECT_WITH_COMPANY_NOT_FOUND")]
    AGECNYCLICKEDCREATEMCCCONNECTWITHCOMPANYNOTFOUND,
    

    /// Agency clicked `give edit access connect with company not found`.
    ///
    /// "AGECNY_CLICKED_GIVE_EDIT_ACCESS_CONNECT_WITH_COMPANY_NOT_FOUND"
    #[serde(rename="AGECNY_CLICKED_GIVE_EDIT_ACCESS_CONNECT_WITH_COMPANY_NOT_FOUND")]
    AGECNYCLICKEDGIVEEDITACCESSCONNECTWITHCOMPANYNOTFOUND,
    

    /// Agency clicked `log out connect with company not found`.
    ///
    /// "AGECNY_CLICKED_LOG_OUT_CONNECT_WITH_COMPANY_NOT_FOUND"
    #[serde(rename="AGECNY_CLICKED_LOG_OUT_CONNECT_WITH_COMPANY_NOT_FOUND")]
    AGECNYCLICKEDLOGOUTCONNECTWITHCOMPANYNOTFOUND,
    

    /// Agency clicked `skip for now on connect with company page`.
    ///
    /// "AGENCY_CLICKED_SKIP_FOR_NOW_ON_CONNECT_WITH_COMPANY_PAGE"
    #[serde(rename="AGENCY_CLICKED_SKIP_FOR_NOW_ON_CONNECT_WITH_COMPANY_PAGE")]
    AGENCYCLICKEDSKIPFORNOWONCONNECTWITHCOMPANYPAGE,
    

    /// Agency closed connection to company.
    ///
    /// "AGENCY_CLOSED_CONNECTED_TO_COMPANY_X_BUTTON_WRONG_COMPANY"
    #[serde(rename="AGENCY_CLOSED_CONNECTED_TO_COMPANY_X_BUTTON_WRONG_COMPANY")]
    AGENCYCLOSEDCONNECTEDTOCOMPANYXBUTTONWRONGCOMPANY,
    

    /// Agency completed field connect with company.
    ///
    /// "AGENCY_COMPLETED_FIELD_CONNECT_WITH_COMPANY"
    #[serde(rename="AGENCY_COMPLETED_FIELD_CONNECT_WITH_COMPANY")]
    AGENCYCOMPLETEDFIELDCONNECTWITHCOMPANY,
    

    /// Agency found company to connect with.
    ///
    /// "AGECNY_FOUND_COMPANY_TO_CONNECT_WITH"
    #[serde(rename="AGECNY_FOUND_COMPANY_TO_CONNECT_WITH")]
    AGECNYFOUNDCOMPANYTOCONNECTWITH,
    

    /// Agency successfully created company.
    ///
    /// "AGENCY_SUCCESSFULLY_CREATED_COMPANY"
    #[serde(rename="AGENCY_SUCCESSFULLY_CREATED_COMPANY")]
    AGENCYSUCCESSFULLYCREATEDCOMPANY,
    

    /// Agency added new company location.
    ///
    /// "AGENCY_ADDED_NEW_COMPANY_LOCATION"
    #[serde(rename="AGENCY_ADDED_NEW_COMPANY_LOCATION")]
    AGENCYADDEDNEWCOMPANYLOCATION,
    

    /// Agency clicked community `join now link` in portal notifications.
    ///
    /// "AGENCY_CLICKED_COMMUNITY_JOIN_NOW_LINK_IN_PORTAL_NOTIFICATIONS"
    #[serde(rename="AGENCY_CLICKED_COMMUNITY_JOIN_NOW_LINK_IN_PORTAL_NOTIFICATIONS")]
    AGENCYCLICKEDCOMMUNITYJOINNOWLINKINPORTALNOTIFICATIONS,
    

    /// Agency clicked `connect to company` link in portal notifications.
    ///
    /// "AGENCY_CLICKED_CONNECT_TO_COMPANY_LINK_IN_PORTAL_NOTIFICATIONS"
    #[serde(rename="AGENCY_CLICKED_CONNECT_TO_COMPANY_LINK_IN_PORTAL_NOTIFICATIONS")]
    AGENCYCLICKEDCONNECTTOCOMPANYLINKINPORTALNOTIFICATIONS,
    

    /// Agency cliecked `get certified` link in portal notifications.
    ///
    /// "AGENCY_CLICKED_GET_CERTIFIED_LINK_IN_PORTAL_NOTIFICATIONS"
    #[serde(rename="AGENCY_CLICKED_GET_CERTIFIED_LINK_IN_PORTAL_NOTIFICATIONS")]
    AGENCYCLICKEDGETCERTIFIEDLINKINPORTALNOTIFICATIONS,
    

    /// Agency clicked `get VideoAds certified` link in portal notifications.
    ///
    /// "AGENCY_CLICKED_GET_VIDEO_ADS_CERTIFIED_LINK_IN_PORTAL_NOTIFICATIONS"
    #[serde(rename="AGENCY_CLICKED_GET_VIDEO_ADS_CERTIFIED_LINK_IN_PORTAL_NOTIFICATIONS")]
    AGENCYCLICKEDGETVIDEOADSCERTIFIEDLINKINPORTALNOTIFICATIONS,
    

    /// Agency clicked `link to MCC` link in portal notifications.
    ///
    /// "AGENCY_CLICKED_LINK_TO_MCC_LINK_IN_PORTAL_NOTIFICATIONS"
    #[serde(rename="AGENCY_CLICKED_LINK_TO_MCC_LINK_IN_PORTAL_NOTIFICATIONS")]
    AGENCYCLICKEDLINKTOMCCLINKINPORTALNOTIFICATIONS,
    

    /// Agency clicked `insight content` in portal.
    ///
    /// "AGENCY_CLICKED_INSIGHT_CONTENT_IN_PORTAL"
    #[serde(rename="AGENCY_CLICKED_INSIGHT_CONTENT_IN_PORTAL")]
    AGENCYCLICKEDINSIGHTCONTENTINPORTAL,
    

    /// Agency clicked `insights view now pitch decks` in portal.
    ///
    /// "AGENCY_CLICKED_INSIGHTS_VIEW_NOW_PITCH_DECKS_IN_PORTAL"
    #[serde(rename="AGENCY_CLICKED_INSIGHTS_VIEW_NOW_PITCH_DECKS_IN_PORTAL")]
    AGENCYCLICKEDINSIGHTSVIEWNOWPITCHDECKSINPORTAL,
    

    /// Agency clicked `insights` left nav in portal.
    ///
    /// "AGENCY_CLICKED_INSIGHTS_LEFT_NAV_IN_PORTAL"
    #[serde(rename="AGENCY_CLICKED_INSIGHTS_LEFT_NAV_IN_PORTAL")]
    AGENCYCLICKEDINSIGHTSLEFTNAVINPORTAL,
    

    /// Agency clicked `insights upload content`.
    ///
    /// "AGENCY_CLICKED_INSIGHTS_UPLOAD_CONTENT"
    #[serde(rename="AGENCY_CLICKED_INSIGHTS_UPLOAD_CONTENT")]
    AGENCYCLICKEDINSIGHTSUPLOADCONTENT,
    

    /// Agency clicked `insights viewed deprecated`.
    ///
    /// "AGENCY_CLICKED_INSIGHTS_VIEWED_DEPRECATED"
    #[serde(rename="AGENCY_CLICKED_INSIGHTS_VIEWED_DEPRECATED")]
    AGENCYCLICKEDINSIGHTSVIEWEDDEPRECATED,
    

    /// Agency clicked `community` left nav in portal.
    ///
    /// "AGENCY_CLICKED_COMMUNITY_LEFT_NAV_IN_PORTAL"
    #[serde(rename="AGENCY_CLICKED_COMMUNITY_LEFT_NAV_IN_PORTAL")]
    AGENCYCLICKEDCOMMUNITYLEFTNAVINPORTAL,
    

    /// Agency clicked `join community` button in community portal.
    ///
    /// "AGENCY_CLICKED_JOIN_COMMUNITY_BUTTON_COMMUNITY_PORTAL"
    #[serde(rename="AGENCY_CLICKED_JOIN_COMMUNITY_BUTTON_COMMUNITY_PORTAL")]
    AGENCYCLICKEDJOINCOMMUNITYBUTTONCOMMUNITYPORTAL,
    

    /// Agency clicked `certifications` left nav in portal.
    ///
    /// "AGENCY_CLICKED_CERTIFICATIONS_LEFT_NAV_IN_PORTAL"
    #[serde(rename="AGENCY_CLICKED_CERTIFICATIONS_LEFT_NAV_IN_PORTAL")]
    AGENCYCLICKEDCERTIFICATIONSLEFTNAVINPORTAL,
    

    /// Agency clicked `certifications product` left nav in portal.
    ///
    /// "AGENCY_CLICKED_CERTIFICATIONS_PRODUCT_LEFT_NAV_IN_PORTAL"
    #[serde(rename="AGENCY_CLICKED_CERTIFICATIONS_PRODUCT_LEFT_NAV_IN_PORTAL")]
    AGENCYCLICKEDCERTIFICATIONSPRODUCTLEFTNAVINPORTAL,
    

    /// Agency clicked `partner status` left nav in portal.
    ///
    /// "AGENCY_CLICKED_PARTNER_STATUS_LEFT_NAV_IN_PORTAL"
    #[serde(rename="AGENCY_CLICKED_PARTNER_STATUS_LEFT_NAV_IN_PORTAL")]
    AGENCYCLICKEDPARTNERSTATUSLEFTNAVINPORTAL,
    

    /// Agency clicked `partner status product` left nav in portal.
    ///
    /// "AGENCY_CLICKED_PARTNER_STATUS_PRODUCT_LEFT_NAV_IN_PORTAL"
    #[serde(rename="AGENCY_CLICKED_PARTNER_STATUS_PRODUCT_LEFT_NAV_IN_PORTAL")]
    AGENCYCLICKEDPARTNERSTATUSPRODUCTLEFTNAVINPORTAL,
    

    /// Agency clicked `offers` left nav in portal.
    ///
    /// "AGENCY_CLICKED_OFFERS_LEFT_NAV_IN_PORTAL"
    #[serde(rename="AGENCY_CLICKED_OFFERS_LEFT_NAV_IN_PORTAL")]
    AGENCYCLICKEDOFFERSLEFTNAVINPORTAL,
    

    /// Agency clicked `send` button on offers page.
    ///
    /// "AGENCY_CLICKED_SEND_BUTTON_ON_OFFERS_PAGE"
    #[serde(rename="AGENCY_CLICKED_SEND_BUTTON_ON_OFFERS_PAGE")]
    AGENCYCLICKEDSENDBUTTONONOFFERSPAGE,
    

    /// Agency clicked `exam details` on certifications AdWords page.
    ///
    /// "AGENCY_CLICKED_EXAM_DETAILS_ON_CERT_ADWORDS_PAGE"
    #[serde(rename="AGENCY_CLICKED_EXAM_DETAILS_ON_CERT_ADWORDS_PAGE")]
    AGENCYCLICKEDEXAMDETAILSONCERTADWORDSPAGE,
    

    /// Agency clicked `see exams` certifications main page.
    ///
    /// "AGENCY_CLICKED_SEE_EXAMS_CERTIFICATION_MAIN_PAGE"
    #[serde(rename="AGENCY_CLICKED_SEE_EXAMS_CERTIFICATION_MAIN_PAGE")]
    AGENCYCLICKEDSEEEXAMSCERTIFICATIONMAINPAGE,
    

    /// Agency clicked `take exam` on certifications exam page.
    ///
    /// "AGENCY_CLICKED_TAKE_EXAM_ON_CERT_EXAM_PAGE"
    #[serde(rename="AGENCY_CLICKED_TAKE_EXAM_ON_CERT_EXAM_PAGE")]
    AGENCYCLICKEDTAKEEXAMONCERTEXAMPAGE,
    

    /// Agency opened `last admin` dialog.
    ///
    /// "AGENCY_OPENED_LAST_ADMIN_DIALOG"
    #[serde(rename="AGENCY_OPENED_LAST_ADMIN_DIALOG")]
    AGENCYOPENEDLASTADMINDIALOG,
    

    /// Agency opened dialog with no users.
    ///
    /// "AGENCY_OPENED_DIALOG_WITH_NO_USERS"
    #[serde(rename="AGENCY_OPENED_DIALOG_WITH_NO_USERS")]
    AGENCYOPENEDDIALOGWITHNOUSERS,
    

    /// Agency promoted user to admin.
    ///
    /// "AGENCY_PROMOTED_USER_TO_ADMIN"
    #[serde(rename="AGENCY_PROMOTED_USER_TO_ADMIN")]
    AGENCYPROMOTEDUSERTOADMIN,
    

    /// Agency unaffiliated.
    ///
    /// "AGENCY_UNAFFILIATED"
    #[serde(rename="AGENCY_UNAFFILIATED")]
    AGENCYUNAFFILIATED,
    

    /// Agency changed roles.
    ///
    /// "AGENCY_CHANGED_ROLES"
    #[serde(rename="AGENCY_CHANGED_ROLES")]
    AGENCYCHANGEDROLES,
    

    /// Advertiser clicked `company name` link to profile.
    ///
    /// "SMB_CLICKED_COMPANY_NAME_LINK_TO_PROFILE"
    #[serde(rename="SMB_CLICKED_COMPANY_NAME_LINK_TO_PROFILE")]
    SMBCLICKEDCOMPANYNAMELINKTOPROFILE,
    

    /// Advertiser viewed AdWords certificate.
    ///
    /// "SMB_VIEWED_ADWORDS_CERTIFICATE"
    #[serde(rename="SMB_VIEWED_ADWORDS_CERTIFICATE")]
    SMBVIEWEDADWORDSCERTIFICATE,
    

    /// Advertiser viewed AdWords Search certificate.
    ///
    /// "SMB_VIEWED_ADWORDS_SEARCH_CERTIFICATE"
    #[serde(rename="SMB_VIEWED_ADWORDS_SEARCH_CERTIFICATE")]
    SMBVIEWEDADWORDSSEARCHCERTIFICATE,
    

    /// Advertiser viewed AdWords Display certificate.
    ///
    /// "SMB_VIEWED_ADWORDS_DISPLAY_CERTIFICATE"
    #[serde(rename="SMB_VIEWED_ADWORDS_DISPLAY_CERTIFICATE")]
    SMBVIEWEDADWORDSDISPLAYCERTIFICATE,
    

    /// Advertiser clicked AdWords certificate help icon.
    ///
    /// "SMB_CLICKED_ADWORDS_CERTIFICATE_HELP_ICON"
    #[serde(rename="SMB_CLICKED_ADWORDS_CERTIFICATE_HELP_ICON")]
    SMBCLICKEDADWORDSCERTIFICATEHELPICON,
    

    /// Advertiser viewed Analytics certificate.
    ///
    /// "SMB_VIEWED_ANALYTICS_CERTIFICATE"
    #[serde(rename="SMB_VIEWED_ANALYTICS_CERTIFICATE")]
    SMBVIEWEDANALYTICSCERTIFICATE,
    

    /// Advertiser viewed DoubleClick certificate.
    ///
    /// "SMB_VIEWED_DOUBLECLICK_CERTIFICATE"
    #[serde(rename="SMB_VIEWED_DOUBLECLICK_CERTIFICATE")]
    SMBVIEWEDDOUBLECLICKCERTIFICATE,
    

    /// Advertiser viewed Mobile Sites certificate.
    ///
    /// "SMB_VIEWED_MOBILE_SITES_CERTIFICATE"
    #[serde(rename="SMB_VIEWED_MOBILE_SITES_CERTIFICATE")]
    SMBVIEWEDMOBILESITESCERTIFICATE,
    

    /// Advertiser viewed VideoAds certificate.
    ///
    /// "SMB_VIEWED_VIDEO_ADS_CERTIFICATE"
    #[serde(rename="SMB_VIEWED_VIDEO_ADS_CERTIFICATE")]
    SMBVIEWEDVIDEOADSCERTIFICATE,
    

    /// Advertiser clicked Shopping certificate help icon.
    ///
    /// "SMB_VIEWED_SHOPPING_CERTIFICATE"
    #[serde(rename="SMB_VIEWED_SHOPPING_CERTIFICATE")]
    SMBVIEWEDSHOPPINGCERTIFICATE,
    

    /// Advertiser clicked VideoAds certificate help icon.
    ///
    /// "SMB_CLICKED_VIDEO_ADS_CERTIFICATE_HELP_ICON"
    #[serde(rename="SMB_CLICKED_VIDEO_ADS_CERTIFICATE_HELP_ICON")]
    SMBCLICKEDVIDEOADSCERTIFICATEHELPICON,
    

    /// Advertiser viewed Digital Sales certificate.
    ///
    /// "SMB_VIEWED_DIGITAL_SALES_CERTIFICATE"
    #[serde(rename="SMB_VIEWED_DIGITAL_SALES_CERTIFICATE")]
    SMBVIEWEDDIGITALSALESCERTIFICATE,
    

    /// Clicked `help` at bottom.
    ///
    /// "CLICKED_HELP_AT_BOTTOM"
    #[serde(rename="CLICKED_HELP_AT_BOTTOM")]
    CLICKEDHELPATBOTTOM,
    

    /// Clicked `help` at top.
    ///
    /// "CLICKED_HELP_AT_TOP"
    #[serde(rename="CLICKED_HELP_AT_TOP")]
    CLICKEDHELPATTOP,
    

    /// Client error occurred.
    ///
    /// "CLIENT_ERROR"
    #[serde(rename="CLIENT_ERROR")]
    CLIENTERROR,
    

    /// Agency clicked left nav `stories`.
    ///
    /// "AGENCY_CLICKED_LEFT_NAV_STORIES"
    #[serde(rename="AGENCY_CLICKED_LEFT_NAV_STORIES")]
    AGENCYCLICKEDLEFTNAVSTORIES,
    

    /// Click occured.
    ///
    /// "CLICKED"
    #[serde(rename="CLICKED")]
    CLICKED,
    

    /// Advertiser clicked Mobile certificate help icon.
    ///
    /// "SMB_VIEWED_MOBILE_CERTIFICATE"
    #[serde(rename="SMB_VIEWED_MOBILE_CERTIFICATE")]
    SMBVIEWEDMOBILECERTIFICATE,
    

    /// Agency failed the company verification.
    ///
    /// "AGENCY_FAILED_COMPANY_VERIFICATION"
    #[serde(rename="AGENCY_FAILED_COMPANY_VERIFICATION")]
    AGENCYFAILEDCOMPANYVERIFICATION,
    

    /// User visited the landing portion of Google Partners.
    ///
    /// "VISITED_LANDING"
    #[serde(rename="VISITED_LANDING")]
    VISITEDLANDING,
    

    /// User visited the Google Partner Search portion of Google Partners.
    ///
    /// "VISITED_GPS"
    #[serde(rename="VISITED_GPS")]
    VISITEDGPS,
    

    /// User visited the agency portal portion of Google Partners.
    ///
    /// "VISITED_AGENCY_PORTAL"
    #[serde(rename="VISITED_AGENCY_PORTAL")]
    VISITEDAGENCYPORTAL,
    

    /// User cancelled signing up.
    ///
    /// "CANCELLED_INDIVIDUAL_SIGN_UP"
    #[serde(rename="CANCELLED_INDIVIDUAL_SIGN_UP")]
    CANCELLEDINDIVIDUALSIGNUP,
    

    /// User cancelled signing up their company.
    ///
    /// "CANCELLED_COMPANY_SIGN_UP"
    #[serde(rename="CANCELLED_COMPANY_SIGN_UP")]
    CANCELLEDCOMPANYSIGNUP,
    

    /// Agency clicked `Sign in` top button.
    ///
    /// "AGENCY_CLICKED_SIGN_IN_BUTTON_TOP"
    #[serde(rename="AGENCY_CLICKED_SIGN_IN_BUTTON_TOP")]
    AGENCYCLICKEDSIGNINBUTTONTOP,
    

    /// Agency clicked `save and continue` at bottom of incomplete profile.
    ///
    /// "AGENCY_CLICKED_SAVE_AND_CONTINUE_AT_BOT_OF_INCOMPLETE_PROFILE"
    #[serde(rename="AGENCY_CLICKED_SAVE_AND_CONTINUE_AT_BOT_OF_INCOMPLETE_PROFILE")]
    AGENCYCLICKEDSAVEANDCONTINUEATBOTOFINCOMPLETEPROFILE,
    

    /// Agency unselected `opt-in news invitations and promotions`.
    ///
    /// "AGENCY_UNSELECTED_OPT_IN_NEWS_INVITATIONS_AND_PROMOS"
    #[serde(rename="AGENCY_UNSELECTED_OPT_IN_NEWS_INVITATIONS_AND_PROMOS")]
    AGENCYUNSELECTEDOPTINNEWSINVITATIONSANDPROMOS,
    

    /// Agency unselected `opt-in beta tests and market research`.
    ///
    /// "AGENCY_UNSELECTED_OPT_IN_BETA_TESTS_AND_MKT_RESEARCH"
    #[serde(rename="AGENCY_UNSELECTED_OPT_IN_BETA_TESTS_AND_MKT_RESEARCH")]
    AGENCYUNSELECTEDOPTINBETATESTSANDMKTRESEARCH,
    

    /// Agency unselected `opt-in performance suggestions`.
    ///
    /// "AGENCY_UNSELECTED_OPT_IN_PERFORMANCE_SUGGESTIONS"
    #[serde(rename="AGENCY_UNSELECTED_OPT_IN_PERFORMANCE_SUGGESTIONS")]
    AGENCYUNSELECTEDOPTINPERFORMANCESUGGESTIONS,
    

    /// Agency selected `opt-out unselect all email notifications`.
    ///
    /// "AGENCY_SELECTED_OPT_OUT_UNSELECT_ALL_EMAIL_NOTIFICATIONS"
    #[serde(rename="AGENCY_SELECTED_OPT_OUT_UNSELECT_ALL_EMAIL_NOTIFICATIONS")]
    AGENCYSELECTEDOPTOUTUNSELECTALLEMAILNOTIFICATIONS,
    

    /// Agency linked their individual MCC.
    ///
    /// "AGENCY_LINKED_INDIVIDUAL_MCC"
    #[serde(rename="AGENCY_LINKED_INDIVIDUAL_MCC")]
    AGENCYLINKEDINDIVIDUALMCC,
    

    /// Agency was suggested to user for affiliation.
    ///
    /// "AGENCY_SUGGESTED_TO_USER"
    #[serde(rename="AGENCY_SUGGESTED_TO_USER")]
    AGENCYSUGGESTEDTOUSER,
    

    /// Agency ignored suggested agencies and begin searching.
    ///
    /// "AGENCY_IGNORED_SUGGESTED_AGENCIES_AND_SEARCHED"
    #[serde(rename="AGENCY_IGNORED_SUGGESTED_AGENCIES_AND_SEARCHED")]
    AGENCYIGNOREDSUGGESTEDAGENCIESANDSEARCHED,
    

    /// Agency picked a suggested agency.
    ///
    /// "AGENCY_PICKED_SUGGESTED_AGENCY"
    #[serde(rename="AGENCY_PICKED_SUGGESTED_AGENCY")]
    AGENCYPICKEDSUGGESTEDAGENCY,
    

    /// Agency searched for agencies.
    ///
    /// "AGENCY_SEARCHED_FOR_AGENCIES"
    #[serde(rename="AGENCY_SEARCHED_FOR_AGENCIES")]
    AGENCYSEARCHEDFORAGENCIES,
    

    /// Agency picked a searched agency.
    ///
    /// "AGENCY_PICKED_SEARCHED_AGENCY"
    #[serde(rename="AGENCY_PICKED_SEARCHED_AGENCY")]
    AGENCYPICKEDSEARCHEDAGENCY,
    

    /// Agency dismissed affiliation widget.
    ///
    /// "AGENCY_DISMISSED_AFFILIATION_WIDGET"
    #[serde(rename="AGENCY_DISMISSED_AFFILIATION_WIDGET")]
    AGENCYDISMISSEDAFFILIATIONWIDGET,
    

    /// Agency clicked on the download link for downloading content.
    ///
    /// "AGENCY_CLICKED_INSIGHTS_DOWNLOAD_CONTENT"
    #[serde(rename="AGENCY_CLICKED_INSIGHTS_DOWNLOAD_CONTENT")]
    AGENCYCLICKEDINSIGHTSDOWNLOADCONTENT,
    

    /// Agency user is maklingg progress viewing a content item.
    ///
    /// "AGENCY_PROGRESS_INSIGHTS_VIEW_CONTENT"
    #[serde(rename="AGENCY_PROGRESS_INSIGHTS_VIEW_CONTENT")]
    AGENCYPROGRESSINSIGHTSVIEWCONTENT,
    

    /// Agency clicked `cancel Terms Of Service` button.
    ///
    /// "AGENCY_CLICKED_CANCEL_ACCEPT_TOS_BUTTON"
    #[serde(rename="AGENCY_CLICKED_CANCEL_ACCEPT_TOS_BUTTON")]
    AGENCYCLICKEDCANCELACCEPTTOSBUTTON,
    

    /// Advertiser entered website in contact form.
    ///
    /// "SMB_ENTERED_WEBSITE_IN_CONTACT_PARTNER_FORM"
    #[serde(rename="SMB_ENTERED_WEBSITE_IN_CONTACT_PARTNER_FORM")]
    SMBENTEREDWEBSITEINCONTACTPARTNERFORM,
    

    /// Agency opted in for migrating their exams to Academy for Ads.
    ///
    /// "AGENCY_SELECTED_OPT_IN_AFA_MIGRATION"
    #[serde(rename="AGENCY_SELECTED_OPT_IN_AFA_MIGRATION")]
    AGENCYSELECTEDOPTINAFAMIGRATION,
    

    /// Agency opted out for migrating their exams to Academy for Ads.
    ///
    /// "AGENCY_SELECTED_OPT_OUT_AFA_MIGRATION"
    #[serde(rename="AGENCY_SELECTED_OPT_OUT_AFA_MIGRATION")]
    AGENCYSELECTEDOPTOUTAFAMIGRATION,
}

impl AsRef<str> for LogUserEventRequestEventActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LogUserEventRequestEventActionEnum::EVENTACTIONUNSPECIFIED => "EVENT_ACTION_UNSPECIFIED",
            LogUserEventRequestEventActionEnum::SMBCLICKEDFINDAPARTNERBUTTONBOTTOM => "SMB_CLICKED_FIND_A_PARTNER_BUTTON_BOTTOM",
            LogUserEventRequestEventActionEnum::SMBCLICKEDFINDAPARTNERBUTTONTOP => "SMB_CLICKED_FIND_A_PARTNER_BUTTON_TOP",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDJOINNOWBUTTONBOTTOM => "AGENCY_CLICKED_JOIN_NOW_BUTTON_BOTTOM",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDJOINNOWBUTTONTOP => "AGENCY_CLICKED_JOIN_NOW_BUTTON_TOP",
            LogUserEventRequestEventActionEnum::SMBCANCELEDPARTNERCONTACTFORM => "SMB_CANCELED_PARTNER_CONTACT_FORM",
            LogUserEventRequestEventActionEnum::SMBCLICKEDCONTACTAPARTNER => "SMB_CLICKED_CONTACT_A_PARTNER",
            LogUserEventRequestEventActionEnum::SMBCOMPLETEDPARTNERCONTACTFORM => "SMB_COMPLETED_PARTNER_CONTACT_FORM",
            LogUserEventRequestEventActionEnum::SMBENTEREDEMAILINCONTACTPARTNERFORM => "SMB_ENTERED_EMAIL_IN_CONTACT_PARTNER_FORM",
            LogUserEventRequestEventActionEnum::SMBENTEREDNAMEINCONTACTPARTNERFORM => "SMB_ENTERED_NAME_IN_CONTACT_PARTNER_FORM",
            LogUserEventRequestEventActionEnum::SMBENTEREDPHONEINCONTACTPARTNERFORM => "SMB_ENTERED_PHONE_IN_CONTACT_PARTNER_FORM",
            LogUserEventRequestEventActionEnum::SMBFAILEDRECAPTCHAINCONTACTPARTNERFORM => "SMB_FAILED_RECAPTCHA_IN_CONTACT_PARTNER_FORM",
            LogUserEventRequestEventActionEnum::PARTNERVIEWEDBYSMB => "PARTNER_VIEWED_BY_SMB",
            LogUserEventRequestEventActionEnum::SMBCANCELEDPARTNERCONTACTFORMONGPS => "SMB_CANCELED_PARTNER_CONTACT_FORM_ON_GPS",
            LogUserEventRequestEventActionEnum::SMBCHANGEDASEARCHPARAMETERTOP => "SMB_CHANGED_A_SEARCH_PARAMETER_TOP",
            LogUserEventRequestEventActionEnum::SMBCLICKEDCONTACTAPARTNERONGPS => "SMB_CLICKED_CONTACT_A_PARTNER_ON_GPS",
            LogUserEventRequestEventActionEnum::SMBCLICKEDSHOWMOREPARTNERSBUTTONBOTTOM => "SMB_CLICKED_SHOW_MORE_PARTNERS_BUTTON_BOTTOM",
            LogUserEventRequestEventActionEnum::SMBCOMPLETEDPARTNERCONTACTFORMONGPS => "SMB_COMPLETED_PARTNER_CONTACT_FORM_ON_GPS",
            LogUserEventRequestEventActionEnum::SMBNOPARTNERSAVAILABLEWITHSEARCHCRITERIA => "SMB_NO_PARTNERS_AVAILABLE_WITH_SEARCH_CRITERIA",
            LogUserEventRequestEventActionEnum::SMBPERFORMEDSEARCHONGPS => "SMB_PERFORMED_SEARCH_ON_GPS",
            LogUserEventRequestEventActionEnum::SMBVIEWEDAPARTNERONGPS => "SMB_VIEWED_A_PARTNER_ON_GPS",
            LogUserEventRequestEventActionEnum::SMBCANCELEDPARTNERCONTACTFORMONPROFILEPAGE => "SMB_CANCELED_PARTNER_CONTACT_FORM_ON_PROFILE_PAGE",
            LogUserEventRequestEventActionEnum::SMBCLICKEDCONTACTAPARTNERONPROFILEPAGE => "SMB_CLICKED_CONTACT_A_PARTNER_ON_PROFILE_PAGE",
            LogUserEventRequestEventActionEnum::SMBCLICKEDPARTNERWEBSITE => "SMB_CLICKED_PARTNER_WEBSITE",
            LogUserEventRequestEventActionEnum::SMBCOMPLETEDPARTNERCONTACTFORMONPROFILEPAGE => "SMB_COMPLETED_PARTNER_CONTACT_FORM_ON_PROFILE_PAGE",
            LogUserEventRequestEventActionEnum::SMBVIEWEDAPARTNERPROFILE => "SMB_VIEWED_A_PARTNER_PROFILE",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDACCEPTTOSBUTTON => "AGENCY_CLICKED_ACCEPT_TOS_BUTTON",
            LogUserEventRequestEventActionEnum::AGENCYCHANGEDTOSCOUNTRY => "AGENCY_CHANGED_TOS_COUNTRY",
            LogUserEventRequestEventActionEnum::AGENCYADDEDADDRESSINMYPROFILEPORTAL => "AGENCY_ADDED_ADDRESS_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYADDEDPHONENUMBERINMYPROFILEPORTAL => "AGENCY_ADDED_PHONE_NUMBER_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCHANGEDPRIMARYACCOUNTASSOCIATION => "AGENCY_CHANGED_PRIMARY_ACCOUNT_ASSOCIATION",
            LogUserEventRequestEventActionEnum::AGENCYCHANGEDPRIMARYCOUNTRYASSOCIATION => "AGENCY_CHANGED_PRIMARY_COUNTRY_ASSOCIATION",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDAFFILIATEBUTTONINMYPROFILEINPORTAL => "AGENCY_CLICKED_AFFILIATE_BUTTON_IN_MY_PROFILE_IN_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDGIVEEDITACCESSINMYPROFILEPORTAL => "AGENCY_CLICKED_GIVE_EDIT_ACCESS_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDLOGOUTINMYPROFILEPORTAL => "AGENCY_CLICKED_LOG_OUT_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDMYPROFILELEFTNAVINPORTAL => "AGENCY_CLICKED_MY_PROFILE_LEFT_NAV_IN_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDSAVEANDCONTINUEATBOTOFCOMPLETEPROFILE => "AGENCY_CLICKED_SAVE_AND_CONTINUE_AT_BOT_OF_COMPLETE_PROFILE",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDUNAFFILIATEINMYPROFILEPORTAL => "AGENCY_CLICKED_UNAFFILIATE_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYFILLEDOUTCOMPAFFILIATIONINMYPROFILEPORTAL => "AGENCY_FILLED_OUT_COMP_AFFILIATION_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYSUCCESSFULLYCONNECTEDWITHCOMPANYINMYPROFILE => "AGENCY_SUCCESSFULLY_CONNECTED_WITH_COMPANY_IN_MY_PROFILE",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDCREATEMCCINMYPROFILEPORTAL => "AGENCY_CLICKED_CREATE_MCC_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYDIDNTHAVEANMCCASSOCIATEDONCOMPLETEPROFILE => "AGENCY_DIDNT_HAVE_AN_MCC_ASSOCIATED_ON_COMPLETE_PROFILE",
            LogUserEventRequestEventActionEnum::AGENCYHADANMCCASSOCIATEDONCOMPLETEPROFILE => "AGENCY_HAD_AN_MCC_ASSOCIATED_ON_COMPLETE_PROFILE",
            LogUserEventRequestEventActionEnum::AGENCYADDEDJOBFUNCTIONINMYPROFILEPORTAL => "AGENCY_ADDED_JOB_FUNCTION_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYLOOKEDATJOBFUNCTIONDROPDOWN => "AGENCY_LOOKED_AT_JOB_FUNCTION_DROP_DOWN",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDACCOUNTMANAGERASJOBFUNCTION => "AGENCY_SELECTED_ACCOUNT_MANAGER_AS_JOB_FUNCTION",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDACCOUNTPLANNERASJOBFUNCTION => "AGENCY_SELECTED_ACCOUNT_PLANNER_AS_JOB_FUNCTION",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDANALYTICSASJOBFUNCTION => "AGENCY_SELECTED_ANALYTICS_AS_JOB_FUNCTION",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDCREATIVEASJOBFUNCTION => "AGENCY_SELECTED_CREATIVE_AS_JOB_FUNCTION",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDMEDIABUYERASJOBFUNCTION => "AGENCY_SELECTED_MEDIA_BUYER_AS_JOB_FUNCTION",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDMEDIAPLANNERASJOBFUNCTION => "AGENCY_SELECTED_MEDIA_PLANNER_AS_JOB_FUNCTION",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDOTHERASJOBFUNCTION => "AGENCY_SELECTED_OTHER_AS_JOB_FUNCTION",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDPRODUCTIONASJOBFUNCTION => "AGENCY_SELECTED_PRODUCTION_AS_JOB_FUNCTION",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDSEOASJOBFUNCTION => "AGENCY_SELECTED_SEO_AS_JOB_FUNCTION",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDSALESREPASJOBFUNCTION => "AGENCY_SELECTED_SALES_REP_AS_JOB_FUNCTION",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDSEARCHSPECIALISTASJOBFUNCTION => "AGENCY_SELECTED_SEARCH_SPECIALIST_AS_JOB_FUNCTION",
            LogUserEventRequestEventActionEnum::AGENCYADDEDCHANNELSINMYPROFILEPORTAL => "AGENCY_ADDED_CHANNELS_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYLOOKEDATADDCHANNELDROPDOWN => "AGENCY_LOOKED_AT_ADD_CHANNEL_DROP_DOWN",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDCROSSCHANNELFROMADDCHANNEL => "AGENCY_SELECTED_CROSS_CHANNEL_FROM_ADD_CHANNEL",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDDISPLAYFROMADDCHANNEL => "AGENCY_SELECTED_DISPLAY_FROM_ADD_CHANNEL",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDMOBILEFROMADDCHANNEL => "AGENCY_SELECTED_MOBILE_FROM_ADD_CHANNEL",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDSEARCHFROMADDCHANNEL => "AGENCY_SELECTED_SEARCH_FROM_ADD_CHANNEL",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDSOCIALFROMADDCHANNEL => "AGENCY_SELECTED_SOCIAL_FROM_ADD_CHANNEL",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDTOOLSFROMADDCHANNEL => "AGENCY_SELECTED_TOOLS_FROM_ADD_CHANNEL",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDYOUTUBEFROMADDCHANNEL => "AGENCY_SELECTED_YOUTUBE_FROM_ADD_CHANNEL",
            LogUserEventRequestEventActionEnum::AGENCYADDEDINDUSTRIESINMYPROFILEPORTAL => "AGENCY_ADDED_INDUSTRIES_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCHANGEDADDINDUSTRIESDROPDOWN => "AGENCY_CHANGED_ADD_INDUSTRIES_DROP_DOWN",
            LogUserEventRequestEventActionEnum::AGENCYADDEDMARKETSINMYPROFILEPORTAL => "AGENCY_ADDED_MARKETS_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCHANGEDADDMARKETSDROPDOWN => "AGENCY_CHANGED_ADD_MARKETS_DROP_DOWN",
            LogUserEventRequestEventActionEnum::AGENCYCHECKEDRECIEVEMAILPROMOTIONSMYPROFILE => "AGENCY_CHECKED_RECIEVE_MAIL_PROMOTIONS_MYPROFILE",
            LogUserEventRequestEventActionEnum::AGENCYCHECKEDRECIEVEMAILPROMOTIONSSIGNUP => "AGENCY_CHECKED_RECIEVE_MAIL_PROMOTIONS_SIGNUP",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINBETATESTSANDMKTRESEARCH => "AGENCY_SELECTED_OPT_IN_BETA_TESTS_AND_MKT_RESEARCH",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINBETATESTSINMYPROFILEPORTAL => "AGENCY_SELECTED_OPT_IN_BETA_TESTS_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINNEWSINMYPROFILEPORTAL => "AGENCY_SELECTED_OPT_IN_NEWS_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINNEWSINVITATIONSANDPROMOS => "AGENCY_SELECTED_OPT_IN_NEWS_INVITATIONS_AND_PROMOS",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINPERFORMANCESUGINMYPROFILEPORTAL => "AGENCY_SELECTED_OPT_IN_PERFORMANCE_SUG_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINPERFORMANCESUGGESTIONS => "AGENCY_SELECTED_OPT_IN_PERFORMANCE_SUGGESTIONS",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINSELECTALLEMAILNOTIFICATIONS => "AGENCY_SELECTED_OPT_IN_SELECT_ALL_EMAIL_NOTIFICATIONS",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDSELECTALLOPTINSINMYPROFILEPORTAL => "AGENCY_SELECTED_SELECT_ALL_OPT_INS_IN_MY_PROFILE_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDBACKBUTTONONCONNECTWITHCOMPANY => "AGENCY_CLICKED_BACK_BUTTON_ON_CONNECT_WITH_COMPANY",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDCONTINUETOOVERVIEWONCONNECTWITHCOMPANY => "AGENCY_CLICKED_CONTINUE_TO_OVERVIEW_ON_CONNECT_WITH_COMPANY",
            LogUserEventRequestEventActionEnum::AGECNYCLICKEDCREATEMCCCONNECTWITHCOMPANYNOTFOUND => "AGECNY_CLICKED_CREATE_MCC_CONNECT_WITH_COMPANY_NOT_FOUND",
            LogUserEventRequestEventActionEnum::AGECNYCLICKEDGIVEEDITACCESSCONNECTWITHCOMPANYNOTFOUND => "AGECNY_CLICKED_GIVE_EDIT_ACCESS_CONNECT_WITH_COMPANY_NOT_FOUND",
            LogUserEventRequestEventActionEnum::AGECNYCLICKEDLOGOUTCONNECTWITHCOMPANYNOTFOUND => "AGECNY_CLICKED_LOG_OUT_CONNECT_WITH_COMPANY_NOT_FOUND",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDSKIPFORNOWONCONNECTWITHCOMPANYPAGE => "AGENCY_CLICKED_SKIP_FOR_NOW_ON_CONNECT_WITH_COMPANY_PAGE",
            LogUserEventRequestEventActionEnum::AGENCYCLOSEDCONNECTEDTOCOMPANYXBUTTONWRONGCOMPANY => "AGENCY_CLOSED_CONNECTED_TO_COMPANY_X_BUTTON_WRONG_COMPANY",
            LogUserEventRequestEventActionEnum::AGENCYCOMPLETEDFIELDCONNECTWITHCOMPANY => "AGENCY_COMPLETED_FIELD_CONNECT_WITH_COMPANY",
            LogUserEventRequestEventActionEnum::AGECNYFOUNDCOMPANYTOCONNECTWITH => "AGECNY_FOUND_COMPANY_TO_CONNECT_WITH",
            LogUserEventRequestEventActionEnum::AGENCYSUCCESSFULLYCREATEDCOMPANY => "AGENCY_SUCCESSFULLY_CREATED_COMPANY",
            LogUserEventRequestEventActionEnum::AGENCYADDEDNEWCOMPANYLOCATION => "AGENCY_ADDED_NEW_COMPANY_LOCATION",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDCOMMUNITYJOINNOWLINKINPORTALNOTIFICATIONS => "AGENCY_CLICKED_COMMUNITY_JOIN_NOW_LINK_IN_PORTAL_NOTIFICATIONS",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDCONNECTTOCOMPANYLINKINPORTALNOTIFICATIONS => "AGENCY_CLICKED_CONNECT_TO_COMPANY_LINK_IN_PORTAL_NOTIFICATIONS",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDGETCERTIFIEDLINKINPORTALNOTIFICATIONS => "AGENCY_CLICKED_GET_CERTIFIED_LINK_IN_PORTAL_NOTIFICATIONS",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDGETVIDEOADSCERTIFIEDLINKINPORTALNOTIFICATIONS => "AGENCY_CLICKED_GET_VIDEO_ADS_CERTIFIED_LINK_IN_PORTAL_NOTIFICATIONS",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDLINKTOMCCLINKINPORTALNOTIFICATIONS => "AGENCY_CLICKED_LINK_TO_MCC_LINK_IN_PORTAL_NOTIFICATIONS",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTCONTENTINPORTAL => "AGENCY_CLICKED_INSIGHT_CONTENT_IN_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTSVIEWNOWPITCHDECKSINPORTAL => "AGENCY_CLICKED_INSIGHTS_VIEW_NOW_PITCH_DECKS_IN_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTSLEFTNAVINPORTAL => "AGENCY_CLICKED_INSIGHTS_LEFT_NAV_IN_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTSUPLOADCONTENT => "AGENCY_CLICKED_INSIGHTS_UPLOAD_CONTENT",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTSVIEWEDDEPRECATED => "AGENCY_CLICKED_INSIGHTS_VIEWED_DEPRECATED",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDCOMMUNITYLEFTNAVINPORTAL => "AGENCY_CLICKED_COMMUNITY_LEFT_NAV_IN_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDJOINCOMMUNITYBUTTONCOMMUNITYPORTAL => "AGENCY_CLICKED_JOIN_COMMUNITY_BUTTON_COMMUNITY_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDCERTIFICATIONSLEFTNAVINPORTAL => "AGENCY_CLICKED_CERTIFICATIONS_LEFT_NAV_IN_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDCERTIFICATIONSPRODUCTLEFTNAVINPORTAL => "AGENCY_CLICKED_CERTIFICATIONS_PRODUCT_LEFT_NAV_IN_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDPARTNERSTATUSLEFTNAVINPORTAL => "AGENCY_CLICKED_PARTNER_STATUS_LEFT_NAV_IN_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDPARTNERSTATUSPRODUCTLEFTNAVINPORTAL => "AGENCY_CLICKED_PARTNER_STATUS_PRODUCT_LEFT_NAV_IN_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDOFFERSLEFTNAVINPORTAL => "AGENCY_CLICKED_OFFERS_LEFT_NAV_IN_PORTAL",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDSENDBUTTONONOFFERSPAGE => "AGENCY_CLICKED_SEND_BUTTON_ON_OFFERS_PAGE",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDEXAMDETAILSONCERTADWORDSPAGE => "AGENCY_CLICKED_EXAM_DETAILS_ON_CERT_ADWORDS_PAGE",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDSEEEXAMSCERTIFICATIONMAINPAGE => "AGENCY_CLICKED_SEE_EXAMS_CERTIFICATION_MAIN_PAGE",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDTAKEEXAMONCERTEXAMPAGE => "AGENCY_CLICKED_TAKE_EXAM_ON_CERT_EXAM_PAGE",
            LogUserEventRequestEventActionEnum::AGENCYOPENEDLASTADMINDIALOG => "AGENCY_OPENED_LAST_ADMIN_DIALOG",
            LogUserEventRequestEventActionEnum::AGENCYOPENEDDIALOGWITHNOUSERS => "AGENCY_OPENED_DIALOG_WITH_NO_USERS",
            LogUserEventRequestEventActionEnum::AGENCYPROMOTEDUSERTOADMIN => "AGENCY_PROMOTED_USER_TO_ADMIN",
            LogUserEventRequestEventActionEnum::AGENCYUNAFFILIATED => "AGENCY_UNAFFILIATED",
            LogUserEventRequestEventActionEnum::AGENCYCHANGEDROLES => "AGENCY_CHANGED_ROLES",
            LogUserEventRequestEventActionEnum::SMBCLICKEDCOMPANYNAMELINKTOPROFILE => "SMB_CLICKED_COMPANY_NAME_LINK_TO_PROFILE",
            LogUserEventRequestEventActionEnum::SMBVIEWEDADWORDSCERTIFICATE => "SMB_VIEWED_ADWORDS_CERTIFICATE",
            LogUserEventRequestEventActionEnum::SMBVIEWEDADWORDSSEARCHCERTIFICATE => "SMB_VIEWED_ADWORDS_SEARCH_CERTIFICATE",
            LogUserEventRequestEventActionEnum::SMBVIEWEDADWORDSDISPLAYCERTIFICATE => "SMB_VIEWED_ADWORDS_DISPLAY_CERTIFICATE",
            LogUserEventRequestEventActionEnum::SMBCLICKEDADWORDSCERTIFICATEHELPICON => "SMB_CLICKED_ADWORDS_CERTIFICATE_HELP_ICON",
            LogUserEventRequestEventActionEnum::SMBVIEWEDANALYTICSCERTIFICATE => "SMB_VIEWED_ANALYTICS_CERTIFICATE",
            LogUserEventRequestEventActionEnum::SMBVIEWEDDOUBLECLICKCERTIFICATE => "SMB_VIEWED_DOUBLECLICK_CERTIFICATE",
            LogUserEventRequestEventActionEnum::SMBVIEWEDMOBILESITESCERTIFICATE => "SMB_VIEWED_MOBILE_SITES_CERTIFICATE",
            LogUserEventRequestEventActionEnum::SMBVIEWEDVIDEOADSCERTIFICATE => "SMB_VIEWED_VIDEO_ADS_CERTIFICATE",
            LogUserEventRequestEventActionEnum::SMBVIEWEDSHOPPINGCERTIFICATE => "SMB_VIEWED_SHOPPING_CERTIFICATE",
            LogUserEventRequestEventActionEnum::SMBCLICKEDVIDEOADSCERTIFICATEHELPICON => "SMB_CLICKED_VIDEO_ADS_CERTIFICATE_HELP_ICON",
            LogUserEventRequestEventActionEnum::SMBVIEWEDDIGITALSALESCERTIFICATE => "SMB_VIEWED_DIGITAL_SALES_CERTIFICATE",
            LogUserEventRequestEventActionEnum::CLICKEDHELPATBOTTOM => "CLICKED_HELP_AT_BOTTOM",
            LogUserEventRequestEventActionEnum::CLICKEDHELPATTOP => "CLICKED_HELP_AT_TOP",
            LogUserEventRequestEventActionEnum::CLIENTERROR => "CLIENT_ERROR",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDLEFTNAVSTORIES => "AGENCY_CLICKED_LEFT_NAV_STORIES",
            LogUserEventRequestEventActionEnum::CLICKED => "CLICKED",
            LogUserEventRequestEventActionEnum::SMBVIEWEDMOBILECERTIFICATE => "SMB_VIEWED_MOBILE_CERTIFICATE",
            LogUserEventRequestEventActionEnum::AGENCYFAILEDCOMPANYVERIFICATION => "AGENCY_FAILED_COMPANY_VERIFICATION",
            LogUserEventRequestEventActionEnum::VISITEDLANDING => "VISITED_LANDING",
            LogUserEventRequestEventActionEnum::VISITEDGPS => "VISITED_GPS",
            LogUserEventRequestEventActionEnum::VISITEDAGENCYPORTAL => "VISITED_AGENCY_PORTAL",
            LogUserEventRequestEventActionEnum::CANCELLEDINDIVIDUALSIGNUP => "CANCELLED_INDIVIDUAL_SIGN_UP",
            LogUserEventRequestEventActionEnum::CANCELLEDCOMPANYSIGNUP => "CANCELLED_COMPANY_SIGN_UP",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDSIGNINBUTTONTOP => "AGENCY_CLICKED_SIGN_IN_BUTTON_TOP",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDSAVEANDCONTINUEATBOTOFINCOMPLETEPROFILE => "AGENCY_CLICKED_SAVE_AND_CONTINUE_AT_BOT_OF_INCOMPLETE_PROFILE",
            LogUserEventRequestEventActionEnum::AGENCYUNSELECTEDOPTINNEWSINVITATIONSANDPROMOS => "AGENCY_UNSELECTED_OPT_IN_NEWS_INVITATIONS_AND_PROMOS",
            LogUserEventRequestEventActionEnum::AGENCYUNSELECTEDOPTINBETATESTSANDMKTRESEARCH => "AGENCY_UNSELECTED_OPT_IN_BETA_TESTS_AND_MKT_RESEARCH",
            LogUserEventRequestEventActionEnum::AGENCYUNSELECTEDOPTINPERFORMANCESUGGESTIONS => "AGENCY_UNSELECTED_OPT_IN_PERFORMANCE_SUGGESTIONS",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTOUTUNSELECTALLEMAILNOTIFICATIONS => "AGENCY_SELECTED_OPT_OUT_UNSELECT_ALL_EMAIL_NOTIFICATIONS",
            LogUserEventRequestEventActionEnum::AGENCYLINKEDINDIVIDUALMCC => "AGENCY_LINKED_INDIVIDUAL_MCC",
            LogUserEventRequestEventActionEnum::AGENCYSUGGESTEDTOUSER => "AGENCY_SUGGESTED_TO_USER",
            LogUserEventRequestEventActionEnum::AGENCYIGNOREDSUGGESTEDAGENCIESANDSEARCHED => "AGENCY_IGNORED_SUGGESTED_AGENCIES_AND_SEARCHED",
            LogUserEventRequestEventActionEnum::AGENCYPICKEDSUGGESTEDAGENCY => "AGENCY_PICKED_SUGGESTED_AGENCY",
            LogUserEventRequestEventActionEnum::AGENCYSEARCHEDFORAGENCIES => "AGENCY_SEARCHED_FOR_AGENCIES",
            LogUserEventRequestEventActionEnum::AGENCYPICKEDSEARCHEDAGENCY => "AGENCY_PICKED_SEARCHED_AGENCY",
            LogUserEventRequestEventActionEnum::AGENCYDISMISSEDAFFILIATIONWIDGET => "AGENCY_DISMISSED_AFFILIATION_WIDGET",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTSDOWNLOADCONTENT => "AGENCY_CLICKED_INSIGHTS_DOWNLOAD_CONTENT",
            LogUserEventRequestEventActionEnum::AGENCYPROGRESSINSIGHTSVIEWCONTENT => "AGENCY_PROGRESS_INSIGHTS_VIEW_CONTENT",
            LogUserEventRequestEventActionEnum::AGENCYCLICKEDCANCELACCEPTTOSBUTTON => "AGENCY_CLICKED_CANCEL_ACCEPT_TOS_BUTTON",
            LogUserEventRequestEventActionEnum::SMBENTEREDWEBSITEINCONTACTPARTNERFORM => "SMB_ENTERED_WEBSITE_IN_CONTACT_PARTNER_FORM",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINAFAMIGRATION => "AGENCY_SELECTED_OPT_IN_AFA_MIGRATION",
            LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTOUTAFAMIGRATION => "AGENCY_SELECTED_OPT_OUT_AFA_MIGRATION",
        }
    }
}

impl std::convert::TryFrom< &str> for LogUserEventRequestEventActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_ACTION_UNSPECIFIED" => Ok(LogUserEventRequestEventActionEnum::EVENTACTIONUNSPECIFIED),
           "SMB_CLICKED_FIND_A_PARTNER_BUTTON_BOTTOM" => Ok(LogUserEventRequestEventActionEnum::SMBCLICKEDFINDAPARTNERBUTTONBOTTOM),
           "SMB_CLICKED_FIND_A_PARTNER_BUTTON_TOP" => Ok(LogUserEventRequestEventActionEnum::SMBCLICKEDFINDAPARTNERBUTTONTOP),
           "AGENCY_CLICKED_JOIN_NOW_BUTTON_BOTTOM" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDJOINNOWBUTTONBOTTOM),
           "AGENCY_CLICKED_JOIN_NOW_BUTTON_TOP" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDJOINNOWBUTTONTOP),
           "SMB_CANCELED_PARTNER_CONTACT_FORM" => Ok(LogUserEventRequestEventActionEnum::SMBCANCELEDPARTNERCONTACTFORM),
           "SMB_CLICKED_CONTACT_A_PARTNER" => Ok(LogUserEventRequestEventActionEnum::SMBCLICKEDCONTACTAPARTNER),
           "SMB_COMPLETED_PARTNER_CONTACT_FORM" => Ok(LogUserEventRequestEventActionEnum::SMBCOMPLETEDPARTNERCONTACTFORM),
           "SMB_ENTERED_EMAIL_IN_CONTACT_PARTNER_FORM" => Ok(LogUserEventRequestEventActionEnum::SMBENTEREDEMAILINCONTACTPARTNERFORM),
           "SMB_ENTERED_NAME_IN_CONTACT_PARTNER_FORM" => Ok(LogUserEventRequestEventActionEnum::SMBENTEREDNAMEINCONTACTPARTNERFORM),
           "SMB_ENTERED_PHONE_IN_CONTACT_PARTNER_FORM" => Ok(LogUserEventRequestEventActionEnum::SMBENTEREDPHONEINCONTACTPARTNERFORM),
           "SMB_FAILED_RECAPTCHA_IN_CONTACT_PARTNER_FORM" => Ok(LogUserEventRequestEventActionEnum::SMBFAILEDRECAPTCHAINCONTACTPARTNERFORM),
           "PARTNER_VIEWED_BY_SMB" => Ok(LogUserEventRequestEventActionEnum::PARTNERVIEWEDBYSMB),
           "SMB_CANCELED_PARTNER_CONTACT_FORM_ON_GPS" => Ok(LogUserEventRequestEventActionEnum::SMBCANCELEDPARTNERCONTACTFORMONGPS),
           "SMB_CHANGED_A_SEARCH_PARAMETER_TOP" => Ok(LogUserEventRequestEventActionEnum::SMBCHANGEDASEARCHPARAMETERTOP),
           "SMB_CLICKED_CONTACT_A_PARTNER_ON_GPS" => Ok(LogUserEventRequestEventActionEnum::SMBCLICKEDCONTACTAPARTNERONGPS),
           "SMB_CLICKED_SHOW_MORE_PARTNERS_BUTTON_BOTTOM" => Ok(LogUserEventRequestEventActionEnum::SMBCLICKEDSHOWMOREPARTNERSBUTTONBOTTOM),
           "SMB_COMPLETED_PARTNER_CONTACT_FORM_ON_GPS" => Ok(LogUserEventRequestEventActionEnum::SMBCOMPLETEDPARTNERCONTACTFORMONGPS),
           "SMB_NO_PARTNERS_AVAILABLE_WITH_SEARCH_CRITERIA" => Ok(LogUserEventRequestEventActionEnum::SMBNOPARTNERSAVAILABLEWITHSEARCHCRITERIA),
           "SMB_PERFORMED_SEARCH_ON_GPS" => Ok(LogUserEventRequestEventActionEnum::SMBPERFORMEDSEARCHONGPS),
           "SMB_VIEWED_A_PARTNER_ON_GPS" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDAPARTNERONGPS),
           "SMB_CANCELED_PARTNER_CONTACT_FORM_ON_PROFILE_PAGE" => Ok(LogUserEventRequestEventActionEnum::SMBCANCELEDPARTNERCONTACTFORMONPROFILEPAGE),
           "SMB_CLICKED_CONTACT_A_PARTNER_ON_PROFILE_PAGE" => Ok(LogUserEventRequestEventActionEnum::SMBCLICKEDCONTACTAPARTNERONPROFILEPAGE),
           "SMB_CLICKED_PARTNER_WEBSITE" => Ok(LogUserEventRequestEventActionEnum::SMBCLICKEDPARTNERWEBSITE),
           "SMB_COMPLETED_PARTNER_CONTACT_FORM_ON_PROFILE_PAGE" => Ok(LogUserEventRequestEventActionEnum::SMBCOMPLETEDPARTNERCONTACTFORMONPROFILEPAGE),
           "SMB_VIEWED_A_PARTNER_PROFILE" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDAPARTNERPROFILE),
           "AGENCY_CLICKED_ACCEPT_TOS_BUTTON" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDACCEPTTOSBUTTON),
           "AGENCY_CHANGED_TOS_COUNTRY" => Ok(LogUserEventRequestEventActionEnum::AGENCYCHANGEDTOSCOUNTRY),
           "AGENCY_ADDED_ADDRESS_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYADDEDADDRESSINMYPROFILEPORTAL),
           "AGENCY_ADDED_PHONE_NUMBER_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYADDEDPHONENUMBERINMYPROFILEPORTAL),
           "AGENCY_CHANGED_PRIMARY_ACCOUNT_ASSOCIATION" => Ok(LogUserEventRequestEventActionEnum::AGENCYCHANGEDPRIMARYACCOUNTASSOCIATION),
           "AGENCY_CHANGED_PRIMARY_COUNTRY_ASSOCIATION" => Ok(LogUserEventRequestEventActionEnum::AGENCYCHANGEDPRIMARYCOUNTRYASSOCIATION),
           "AGENCY_CLICKED_AFFILIATE_BUTTON_IN_MY_PROFILE_IN_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDAFFILIATEBUTTONINMYPROFILEINPORTAL),
           "AGENCY_CLICKED_GIVE_EDIT_ACCESS_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDGIVEEDITACCESSINMYPROFILEPORTAL),
           "AGENCY_CLICKED_LOG_OUT_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDLOGOUTINMYPROFILEPORTAL),
           "AGENCY_CLICKED_MY_PROFILE_LEFT_NAV_IN_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDMYPROFILELEFTNAVINPORTAL),
           "AGENCY_CLICKED_SAVE_AND_CONTINUE_AT_BOT_OF_COMPLETE_PROFILE" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDSAVEANDCONTINUEATBOTOFCOMPLETEPROFILE),
           "AGENCY_CLICKED_UNAFFILIATE_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDUNAFFILIATEINMYPROFILEPORTAL),
           "AGENCY_FILLED_OUT_COMP_AFFILIATION_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYFILLEDOUTCOMPAFFILIATIONINMYPROFILEPORTAL),
           "AGENCY_SUCCESSFULLY_CONNECTED_WITH_COMPANY_IN_MY_PROFILE" => Ok(LogUserEventRequestEventActionEnum::AGENCYSUCCESSFULLYCONNECTEDWITHCOMPANYINMYPROFILE),
           "AGENCY_CLICKED_CREATE_MCC_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDCREATEMCCINMYPROFILEPORTAL),
           "AGENCY_DIDNT_HAVE_AN_MCC_ASSOCIATED_ON_COMPLETE_PROFILE" => Ok(LogUserEventRequestEventActionEnum::AGENCYDIDNTHAVEANMCCASSOCIATEDONCOMPLETEPROFILE),
           "AGENCY_HAD_AN_MCC_ASSOCIATED_ON_COMPLETE_PROFILE" => Ok(LogUserEventRequestEventActionEnum::AGENCYHADANMCCASSOCIATEDONCOMPLETEPROFILE),
           "AGENCY_ADDED_JOB_FUNCTION_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYADDEDJOBFUNCTIONINMYPROFILEPORTAL),
           "AGENCY_LOOKED_AT_JOB_FUNCTION_DROP_DOWN" => Ok(LogUserEventRequestEventActionEnum::AGENCYLOOKEDATJOBFUNCTIONDROPDOWN),
           "AGENCY_SELECTED_ACCOUNT_MANAGER_AS_JOB_FUNCTION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDACCOUNTMANAGERASJOBFUNCTION),
           "AGENCY_SELECTED_ACCOUNT_PLANNER_AS_JOB_FUNCTION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDACCOUNTPLANNERASJOBFUNCTION),
           "AGENCY_SELECTED_ANALYTICS_AS_JOB_FUNCTION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDANALYTICSASJOBFUNCTION),
           "AGENCY_SELECTED_CREATIVE_AS_JOB_FUNCTION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDCREATIVEASJOBFUNCTION),
           "AGENCY_SELECTED_MEDIA_BUYER_AS_JOB_FUNCTION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDMEDIABUYERASJOBFUNCTION),
           "AGENCY_SELECTED_MEDIA_PLANNER_AS_JOB_FUNCTION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDMEDIAPLANNERASJOBFUNCTION),
           "AGENCY_SELECTED_OTHER_AS_JOB_FUNCTION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDOTHERASJOBFUNCTION),
           "AGENCY_SELECTED_PRODUCTION_AS_JOB_FUNCTION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDPRODUCTIONASJOBFUNCTION),
           "AGENCY_SELECTED_SEO_AS_JOB_FUNCTION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDSEOASJOBFUNCTION),
           "AGENCY_SELECTED_SALES_REP_AS_JOB_FUNCTION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDSALESREPASJOBFUNCTION),
           "AGENCY_SELECTED_SEARCH_SPECIALIST_AS_JOB_FUNCTION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDSEARCHSPECIALISTASJOBFUNCTION),
           "AGENCY_ADDED_CHANNELS_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYADDEDCHANNELSINMYPROFILEPORTAL),
           "AGENCY_LOOKED_AT_ADD_CHANNEL_DROP_DOWN" => Ok(LogUserEventRequestEventActionEnum::AGENCYLOOKEDATADDCHANNELDROPDOWN),
           "AGENCY_SELECTED_CROSS_CHANNEL_FROM_ADD_CHANNEL" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDCROSSCHANNELFROMADDCHANNEL),
           "AGENCY_SELECTED_DISPLAY_FROM_ADD_CHANNEL" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDDISPLAYFROMADDCHANNEL),
           "AGENCY_SELECTED_MOBILE_FROM_ADD_CHANNEL" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDMOBILEFROMADDCHANNEL),
           "AGENCY_SELECTED_SEARCH_FROM_ADD_CHANNEL" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDSEARCHFROMADDCHANNEL),
           "AGENCY_SELECTED_SOCIAL_FROM_ADD_CHANNEL" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDSOCIALFROMADDCHANNEL),
           "AGENCY_SELECTED_TOOLS_FROM_ADD_CHANNEL" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDTOOLSFROMADDCHANNEL),
           "AGENCY_SELECTED_YOUTUBE_FROM_ADD_CHANNEL" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDYOUTUBEFROMADDCHANNEL),
           "AGENCY_ADDED_INDUSTRIES_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYADDEDINDUSTRIESINMYPROFILEPORTAL),
           "AGENCY_CHANGED_ADD_INDUSTRIES_DROP_DOWN" => Ok(LogUserEventRequestEventActionEnum::AGENCYCHANGEDADDINDUSTRIESDROPDOWN),
           "AGENCY_ADDED_MARKETS_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYADDEDMARKETSINMYPROFILEPORTAL),
           "AGENCY_CHANGED_ADD_MARKETS_DROP_DOWN" => Ok(LogUserEventRequestEventActionEnum::AGENCYCHANGEDADDMARKETSDROPDOWN),
           "AGENCY_CHECKED_RECIEVE_MAIL_PROMOTIONS_MYPROFILE" => Ok(LogUserEventRequestEventActionEnum::AGENCYCHECKEDRECIEVEMAILPROMOTIONSMYPROFILE),
           "AGENCY_CHECKED_RECIEVE_MAIL_PROMOTIONS_SIGNUP" => Ok(LogUserEventRequestEventActionEnum::AGENCYCHECKEDRECIEVEMAILPROMOTIONSSIGNUP),
           "AGENCY_SELECTED_OPT_IN_BETA_TESTS_AND_MKT_RESEARCH" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINBETATESTSANDMKTRESEARCH),
           "AGENCY_SELECTED_OPT_IN_BETA_TESTS_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINBETATESTSINMYPROFILEPORTAL),
           "AGENCY_SELECTED_OPT_IN_NEWS_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINNEWSINMYPROFILEPORTAL),
           "AGENCY_SELECTED_OPT_IN_NEWS_INVITATIONS_AND_PROMOS" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINNEWSINVITATIONSANDPROMOS),
           "AGENCY_SELECTED_OPT_IN_PERFORMANCE_SUG_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINPERFORMANCESUGINMYPROFILEPORTAL),
           "AGENCY_SELECTED_OPT_IN_PERFORMANCE_SUGGESTIONS" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINPERFORMANCESUGGESTIONS),
           "AGENCY_SELECTED_OPT_IN_SELECT_ALL_EMAIL_NOTIFICATIONS" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINSELECTALLEMAILNOTIFICATIONS),
           "AGENCY_SELECTED_SELECT_ALL_OPT_INS_IN_MY_PROFILE_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDSELECTALLOPTINSINMYPROFILEPORTAL),
           "AGENCY_CLICKED_BACK_BUTTON_ON_CONNECT_WITH_COMPANY" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDBACKBUTTONONCONNECTWITHCOMPANY),
           "AGENCY_CLICKED_CONTINUE_TO_OVERVIEW_ON_CONNECT_WITH_COMPANY" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDCONTINUETOOVERVIEWONCONNECTWITHCOMPANY),
           "AGECNY_CLICKED_CREATE_MCC_CONNECT_WITH_COMPANY_NOT_FOUND" => Ok(LogUserEventRequestEventActionEnum::AGECNYCLICKEDCREATEMCCCONNECTWITHCOMPANYNOTFOUND),
           "AGECNY_CLICKED_GIVE_EDIT_ACCESS_CONNECT_WITH_COMPANY_NOT_FOUND" => Ok(LogUserEventRequestEventActionEnum::AGECNYCLICKEDGIVEEDITACCESSCONNECTWITHCOMPANYNOTFOUND),
           "AGECNY_CLICKED_LOG_OUT_CONNECT_WITH_COMPANY_NOT_FOUND" => Ok(LogUserEventRequestEventActionEnum::AGECNYCLICKEDLOGOUTCONNECTWITHCOMPANYNOTFOUND),
           "AGENCY_CLICKED_SKIP_FOR_NOW_ON_CONNECT_WITH_COMPANY_PAGE" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDSKIPFORNOWONCONNECTWITHCOMPANYPAGE),
           "AGENCY_CLOSED_CONNECTED_TO_COMPANY_X_BUTTON_WRONG_COMPANY" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLOSEDCONNECTEDTOCOMPANYXBUTTONWRONGCOMPANY),
           "AGENCY_COMPLETED_FIELD_CONNECT_WITH_COMPANY" => Ok(LogUserEventRequestEventActionEnum::AGENCYCOMPLETEDFIELDCONNECTWITHCOMPANY),
           "AGECNY_FOUND_COMPANY_TO_CONNECT_WITH" => Ok(LogUserEventRequestEventActionEnum::AGECNYFOUNDCOMPANYTOCONNECTWITH),
           "AGENCY_SUCCESSFULLY_CREATED_COMPANY" => Ok(LogUserEventRequestEventActionEnum::AGENCYSUCCESSFULLYCREATEDCOMPANY),
           "AGENCY_ADDED_NEW_COMPANY_LOCATION" => Ok(LogUserEventRequestEventActionEnum::AGENCYADDEDNEWCOMPANYLOCATION),
           "AGENCY_CLICKED_COMMUNITY_JOIN_NOW_LINK_IN_PORTAL_NOTIFICATIONS" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDCOMMUNITYJOINNOWLINKINPORTALNOTIFICATIONS),
           "AGENCY_CLICKED_CONNECT_TO_COMPANY_LINK_IN_PORTAL_NOTIFICATIONS" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDCONNECTTOCOMPANYLINKINPORTALNOTIFICATIONS),
           "AGENCY_CLICKED_GET_CERTIFIED_LINK_IN_PORTAL_NOTIFICATIONS" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDGETCERTIFIEDLINKINPORTALNOTIFICATIONS),
           "AGENCY_CLICKED_GET_VIDEO_ADS_CERTIFIED_LINK_IN_PORTAL_NOTIFICATIONS" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDGETVIDEOADSCERTIFIEDLINKINPORTALNOTIFICATIONS),
           "AGENCY_CLICKED_LINK_TO_MCC_LINK_IN_PORTAL_NOTIFICATIONS" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDLINKTOMCCLINKINPORTALNOTIFICATIONS),
           "AGENCY_CLICKED_INSIGHT_CONTENT_IN_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTCONTENTINPORTAL),
           "AGENCY_CLICKED_INSIGHTS_VIEW_NOW_PITCH_DECKS_IN_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTSVIEWNOWPITCHDECKSINPORTAL),
           "AGENCY_CLICKED_INSIGHTS_LEFT_NAV_IN_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTSLEFTNAVINPORTAL),
           "AGENCY_CLICKED_INSIGHTS_UPLOAD_CONTENT" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTSUPLOADCONTENT),
           "AGENCY_CLICKED_INSIGHTS_VIEWED_DEPRECATED" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTSVIEWEDDEPRECATED),
           "AGENCY_CLICKED_COMMUNITY_LEFT_NAV_IN_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDCOMMUNITYLEFTNAVINPORTAL),
           "AGENCY_CLICKED_JOIN_COMMUNITY_BUTTON_COMMUNITY_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDJOINCOMMUNITYBUTTONCOMMUNITYPORTAL),
           "AGENCY_CLICKED_CERTIFICATIONS_LEFT_NAV_IN_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDCERTIFICATIONSLEFTNAVINPORTAL),
           "AGENCY_CLICKED_CERTIFICATIONS_PRODUCT_LEFT_NAV_IN_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDCERTIFICATIONSPRODUCTLEFTNAVINPORTAL),
           "AGENCY_CLICKED_PARTNER_STATUS_LEFT_NAV_IN_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDPARTNERSTATUSLEFTNAVINPORTAL),
           "AGENCY_CLICKED_PARTNER_STATUS_PRODUCT_LEFT_NAV_IN_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDPARTNERSTATUSPRODUCTLEFTNAVINPORTAL),
           "AGENCY_CLICKED_OFFERS_LEFT_NAV_IN_PORTAL" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDOFFERSLEFTNAVINPORTAL),
           "AGENCY_CLICKED_SEND_BUTTON_ON_OFFERS_PAGE" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDSENDBUTTONONOFFERSPAGE),
           "AGENCY_CLICKED_EXAM_DETAILS_ON_CERT_ADWORDS_PAGE" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDEXAMDETAILSONCERTADWORDSPAGE),
           "AGENCY_CLICKED_SEE_EXAMS_CERTIFICATION_MAIN_PAGE" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDSEEEXAMSCERTIFICATIONMAINPAGE),
           "AGENCY_CLICKED_TAKE_EXAM_ON_CERT_EXAM_PAGE" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDTAKEEXAMONCERTEXAMPAGE),
           "AGENCY_OPENED_LAST_ADMIN_DIALOG" => Ok(LogUserEventRequestEventActionEnum::AGENCYOPENEDLASTADMINDIALOG),
           "AGENCY_OPENED_DIALOG_WITH_NO_USERS" => Ok(LogUserEventRequestEventActionEnum::AGENCYOPENEDDIALOGWITHNOUSERS),
           "AGENCY_PROMOTED_USER_TO_ADMIN" => Ok(LogUserEventRequestEventActionEnum::AGENCYPROMOTEDUSERTOADMIN),
           "AGENCY_UNAFFILIATED" => Ok(LogUserEventRequestEventActionEnum::AGENCYUNAFFILIATED),
           "AGENCY_CHANGED_ROLES" => Ok(LogUserEventRequestEventActionEnum::AGENCYCHANGEDROLES),
           "SMB_CLICKED_COMPANY_NAME_LINK_TO_PROFILE" => Ok(LogUserEventRequestEventActionEnum::SMBCLICKEDCOMPANYNAMELINKTOPROFILE),
           "SMB_VIEWED_ADWORDS_CERTIFICATE" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDADWORDSCERTIFICATE),
           "SMB_VIEWED_ADWORDS_SEARCH_CERTIFICATE" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDADWORDSSEARCHCERTIFICATE),
           "SMB_VIEWED_ADWORDS_DISPLAY_CERTIFICATE" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDADWORDSDISPLAYCERTIFICATE),
           "SMB_CLICKED_ADWORDS_CERTIFICATE_HELP_ICON" => Ok(LogUserEventRequestEventActionEnum::SMBCLICKEDADWORDSCERTIFICATEHELPICON),
           "SMB_VIEWED_ANALYTICS_CERTIFICATE" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDANALYTICSCERTIFICATE),
           "SMB_VIEWED_DOUBLECLICK_CERTIFICATE" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDDOUBLECLICKCERTIFICATE),
           "SMB_VIEWED_MOBILE_SITES_CERTIFICATE" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDMOBILESITESCERTIFICATE),
           "SMB_VIEWED_VIDEO_ADS_CERTIFICATE" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDVIDEOADSCERTIFICATE),
           "SMB_VIEWED_SHOPPING_CERTIFICATE" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDSHOPPINGCERTIFICATE),
           "SMB_CLICKED_VIDEO_ADS_CERTIFICATE_HELP_ICON" => Ok(LogUserEventRequestEventActionEnum::SMBCLICKEDVIDEOADSCERTIFICATEHELPICON),
           "SMB_VIEWED_DIGITAL_SALES_CERTIFICATE" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDDIGITALSALESCERTIFICATE),
           "CLICKED_HELP_AT_BOTTOM" => Ok(LogUserEventRequestEventActionEnum::CLICKEDHELPATBOTTOM),
           "CLICKED_HELP_AT_TOP" => Ok(LogUserEventRequestEventActionEnum::CLICKEDHELPATTOP),
           "CLIENT_ERROR" => Ok(LogUserEventRequestEventActionEnum::CLIENTERROR),
           "AGENCY_CLICKED_LEFT_NAV_STORIES" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDLEFTNAVSTORIES),
           "CLICKED" => Ok(LogUserEventRequestEventActionEnum::CLICKED),
           "SMB_VIEWED_MOBILE_CERTIFICATE" => Ok(LogUserEventRequestEventActionEnum::SMBVIEWEDMOBILECERTIFICATE),
           "AGENCY_FAILED_COMPANY_VERIFICATION" => Ok(LogUserEventRequestEventActionEnum::AGENCYFAILEDCOMPANYVERIFICATION),
           "VISITED_LANDING" => Ok(LogUserEventRequestEventActionEnum::VISITEDLANDING),
           "VISITED_GPS" => Ok(LogUserEventRequestEventActionEnum::VISITEDGPS),
           "VISITED_AGENCY_PORTAL" => Ok(LogUserEventRequestEventActionEnum::VISITEDAGENCYPORTAL),
           "CANCELLED_INDIVIDUAL_SIGN_UP" => Ok(LogUserEventRequestEventActionEnum::CANCELLEDINDIVIDUALSIGNUP),
           "CANCELLED_COMPANY_SIGN_UP" => Ok(LogUserEventRequestEventActionEnum::CANCELLEDCOMPANYSIGNUP),
           "AGENCY_CLICKED_SIGN_IN_BUTTON_TOP" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDSIGNINBUTTONTOP),
           "AGENCY_CLICKED_SAVE_AND_CONTINUE_AT_BOT_OF_INCOMPLETE_PROFILE" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDSAVEANDCONTINUEATBOTOFINCOMPLETEPROFILE),
           "AGENCY_UNSELECTED_OPT_IN_NEWS_INVITATIONS_AND_PROMOS" => Ok(LogUserEventRequestEventActionEnum::AGENCYUNSELECTEDOPTINNEWSINVITATIONSANDPROMOS),
           "AGENCY_UNSELECTED_OPT_IN_BETA_TESTS_AND_MKT_RESEARCH" => Ok(LogUserEventRequestEventActionEnum::AGENCYUNSELECTEDOPTINBETATESTSANDMKTRESEARCH),
           "AGENCY_UNSELECTED_OPT_IN_PERFORMANCE_SUGGESTIONS" => Ok(LogUserEventRequestEventActionEnum::AGENCYUNSELECTEDOPTINPERFORMANCESUGGESTIONS),
           "AGENCY_SELECTED_OPT_OUT_UNSELECT_ALL_EMAIL_NOTIFICATIONS" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTOUTUNSELECTALLEMAILNOTIFICATIONS),
           "AGENCY_LINKED_INDIVIDUAL_MCC" => Ok(LogUserEventRequestEventActionEnum::AGENCYLINKEDINDIVIDUALMCC),
           "AGENCY_SUGGESTED_TO_USER" => Ok(LogUserEventRequestEventActionEnum::AGENCYSUGGESTEDTOUSER),
           "AGENCY_IGNORED_SUGGESTED_AGENCIES_AND_SEARCHED" => Ok(LogUserEventRequestEventActionEnum::AGENCYIGNOREDSUGGESTEDAGENCIESANDSEARCHED),
           "AGENCY_PICKED_SUGGESTED_AGENCY" => Ok(LogUserEventRequestEventActionEnum::AGENCYPICKEDSUGGESTEDAGENCY),
           "AGENCY_SEARCHED_FOR_AGENCIES" => Ok(LogUserEventRequestEventActionEnum::AGENCYSEARCHEDFORAGENCIES),
           "AGENCY_PICKED_SEARCHED_AGENCY" => Ok(LogUserEventRequestEventActionEnum::AGENCYPICKEDSEARCHEDAGENCY),
           "AGENCY_DISMISSED_AFFILIATION_WIDGET" => Ok(LogUserEventRequestEventActionEnum::AGENCYDISMISSEDAFFILIATIONWIDGET),
           "AGENCY_CLICKED_INSIGHTS_DOWNLOAD_CONTENT" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDINSIGHTSDOWNLOADCONTENT),
           "AGENCY_PROGRESS_INSIGHTS_VIEW_CONTENT" => Ok(LogUserEventRequestEventActionEnum::AGENCYPROGRESSINSIGHTSVIEWCONTENT),
           "AGENCY_CLICKED_CANCEL_ACCEPT_TOS_BUTTON" => Ok(LogUserEventRequestEventActionEnum::AGENCYCLICKEDCANCELACCEPTTOSBUTTON),
           "SMB_ENTERED_WEBSITE_IN_CONTACT_PARTNER_FORM" => Ok(LogUserEventRequestEventActionEnum::SMBENTEREDWEBSITEINCONTACTPARTNERFORM),
           "AGENCY_SELECTED_OPT_IN_AFA_MIGRATION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTINAFAMIGRATION),
           "AGENCY_SELECTED_OPT_OUT_AFA_MIGRATION" => Ok(LogUserEventRequestEventActionEnum::AGENCYSELECTEDOPTOUTAFAMIGRATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LogUserEventRequestEventActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AvailableOfferOfferLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Level of this offer.
pub enum AvailableOfferOfferLevelEnum {
    

    /// Unset.
    ///
    /// "OFFER_LEVEL_UNSPECIFIED"
    #[serde(rename="OFFER_LEVEL_UNSPECIFIED")]
    OFFERLEVELUNSPECIFIED,
    

    /// Users/Agencies that have no offers because of a problem.
    ///
    /// "OFFER_LEVEL_DENY_PROBLEM"
    #[serde(rename="OFFER_LEVEL_DENY_PROBLEM")]
    OFFERLEVELDENYPROBLEM,
    

    /// Users/Agencies that have no offers due to contractural agreements.
    ///
    /// "OFFER_LEVEL_DENY_CONTRACT"
    #[serde(rename="OFFER_LEVEL_DENY_CONTRACT")]
    OFFERLEVELDENYCONTRACT,
    

    /// Users/Agencies that have a manually-configured limit.
    ///
    /// "OFFER_LEVEL_MANUAL"
    #[serde(rename="OFFER_LEVEL_MANUAL")]
    OFFERLEVELMANUAL,
    

    /// Some Agencies don't get any offers.
    ///
    /// "OFFER_LEVEL_LIMIT_0"
    #[serde(rename="OFFER_LEVEL_LIMIT_0")]
    OFFERLEVELLIMIT0,
    

    /// Basic level gets 5 per month.
    ///
    /// "OFFER_LEVEL_LIMIT_5"
    #[serde(rename="OFFER_LEVEL_LIMIT_5")]
    OFFERLEVELLIMIT5,
    

    /// Agencies with adequate AHI and spend get 15/month.
    ///
    /// "OFFER_LEVEL_LIMIT_15"
    #[serde(rename="OFFER_LEVEL_LIMIT_15")]
    OFFERLEVELLIMIT15,
    

    /// Badged partners (even in grace) get 50 per month.
    ///
    /// "OFFER_LEVEL_LIMIT_50"
    #[serde(rename="OFFER_LEVEL_LIMIT_50")]
    OFFERLEVELLIMIT50,
}

impl AsRef<str> for AvailableOfferOfferLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AvailableOfferOfferLevelEnum::OFFERLEVELUNSPECIFIED => "OFFER_LEVEL_UNSPECIFIED",
            AvailableOfferOfferLevelEnum::OFFERLEVELDENYPROBLEM => "OFFER_LEVEL_DENY_PROBLEM",
            AvailableOfferOfferLevelEnum::OFFERLEVELDENYCONTRACT => "OFFER_LEVEL_DENY_CONTRACT",
            AvailableOfferOfferLevelEnum::OFFERLEVELMANUAL => "OFFER_LEVEL_MANUAL",
            AvailableOfferOfferLevelEnum::OFFERLEVELLIMIT0 => "OFFER_LEVEL_LIMIT_0",
            AvailableOfferOfferLevelEnum::OFFERLEVELLIMIT5 => "OFFER_LEVEL_LIMIT_5",
            AvailableOfferOfferLevelEnum::OFFERLEVELLIMIT15 => "OFFER_LEVEL_LIMIT_15",
            AvailableOfferOfferLevelEnum::OFFERLEVELLIMIT50 => "OFFER_LEVEL_LIMIT_50",
        }
    }
}

impl std::convert::TryFrom< &str> for AvailableOfferOfferLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFFER_LEVEL_UNSPECIFIED" => Ok(AvailableOfferOfferLevelEnum::OFFERLEVELUNSPECIFIED),
           "OFFER_LEVEL_DENY_PROBLEM" => Ok(AvailableOfferOfferLevelEnum::OFFERLEVELDENYPROBLEM),
           "OFFER_LEVEL_DENY_CONTRACT" => Ok(AvailableOfferOfferLevelEnum::OFFERLEVELDENYCONTRACT),
           "OFFER_LEVEL_MANUAL" => Ok(AvailableOfferOfferLevelEnum::OFFERLEVELMANUAL),
           "OFFER_LEVEL_LIMIT_0" => Ok(AvailableOfferOfferLevelEnum::OFFERLEVELLIMIT0),
           "OFFER_LEVEL_LIMIT_5" => Ok(AvailableOfferOfferLevelEnum::OFFERLEVELLIMIT5),
           "OFFER_LEVEL_LIMIT_15" => Ok(AvailableOfferOfferLevelEnum::OFFERLEVELLIMIT15),
           "OFFER_LEVEL_LIMIT_50" => Ok(AvailableOfferOfferLevelEnum::OFFERLEVELLIMIT50),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AvailableOfferOfferLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AvailableOfferOfferTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of offer.
pub enum AvailableOfferOfferTypeEnum {
    

    /// Unset.
    ///
    /// "OFFER_TYPE_UNSPECIFIED"
    #[serde(rename="OFFER_TYPE_UNSPECIFIED")]
    OFFERTYPEUNSPECIFIED,
    

    /// AdWords spend X get Y.
    ///
    /// "OFFER_TYPE_SPEND_X_GET_Y"
    #[serde(rename="OFFER_TYPE_SPEND_X_GET_Y")]
    OFFERTYPESPENDXGETY,
    

    /// Youtube video.
    ///
    /// "OFFER_TYPE_VIDEO"
    #[serde(rename="OFFER_TYPE_VIDEO")]
    OFFERTYPEVIDEO,
    

    /// Spend Match up to Y.
    ///
    /// "OFFER_TYPE_SPEND_MATCH"
    #[serde(rename="OFFER_TYPE_SPEND_MATCH")]
    OFFERTYPESPENDMATCH,
}

impl AsRef<str> for AvailableOfferOfferTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AvailableOfferOfferTypeEnum::OFFERTYPEUNSPECIFIED => "OFFER_TYPE_UNSPECIFIED",
            AvailableOfferOfferTypeEnum::OFFERTYPESPENDXGETY => "OFFER_TYPE_SPEND_X_GET_Y",
            AvailableOfferOfferTypeEnum::OFFERTYPEVIDEO => "OFFER_TYPE_VIDEO",
            AvailableOfferOfferTypeEnum::OFFERTYPESPENDMATCH => "OFFER_TYPE_SPEND_MATCH",
        }
    }
}

impl std::convert::TryFrom< &str> for AvailableOfferOfferTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFFER_TYPE_UNSPECIFIED" => Ok(AvailableOfferOfferTypeEnum::OFFERTYPEUNSPECIFIED),
           "OFFER_TYPE_SPEND_X_GET_Y" => Ok(AvailableOfferOfferTypeEnum::OFFERTYPESPENDXGETY),
           "OFFER_TYPE_VIDEO" => Ok(AvailableOfferOfferTypeEnum::OFFERTYPEVIDEO),
           "OFFER_TYPE_SPEND_MATCH" => Ok(AvailableOfferOfferTypeEnum::OFFERTYPESPENDMATCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AvailableOfferOfferTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LogMessageRequestLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Message level of client message.
pub enum LogMessageRequestLevelEnum {
    

    /// Unchosen.
    ///
    /// "MESSAGE_LEVEL_UNSPECIFIED"
    #[serde(rename="MESSAGE_LEVEL_UNSPECIFIED")]
    MESSAGELEVELUNSPECIFIED,
    

    /// Message level for tracing information.
    ///
    /// "ML_FINE"
    #[serde(rename="ML_FINE")]
    MLFINE,
    

    /// Message level for informational messages.
    ///
    /// "ML_INFO"
    #[serde(rename="ML_INFO")]
    MLINFO,
    

    /// Message level for potential problems.
    ///
    /// "ML_WARNING"
    #[serde(rename="ML_WARNING")]
    MLWARNING,
    

    /// Message level for serious failures.
    ///
    /// "ML_SEVERE"
    #[serde(rename="ML_SEVERE")]
    MLSEVERE,
}

impl AsRef<str> for LogMessageRequestLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LogMessageRequestLevelEnum::MESSAGELEVELUNSPECIFIED => "MESSAGE_LEVEL_UNSPECIFIED",
            LogMessageRequestLevelEnum::MLFINE => "ML_FINE",
            LogMessageRequestLevelEnum::MLINFO => "ML_INFO",
            LogMessageRequestLevelEnum::MLWARNING => "ML_WARNING",
            LogMessageRequestLevelEnum::MLSEVERE => "ML_SEVERE",
        }
    }
}

impl std::convert::TryFrom< &str> for LogMessageRequestLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MESSAGE_LEVEL_UNSPECIFIED" => Ok(LogMessageRequestLevelEnum::MESSAGELEVELUNSPECIFIED),
           "ML_FINE" => Ok(LogMessageRequestLevelEnum::MLFINE),
           "ML_INFO" => Ok(LogMessageRequestLevelEnum::MLINFO),
           "ML_WARNING" => Ok(LogMessageRequestLevelEnum::MLWARNING),
           "ML_SEVERE" => Ok(LogMessageRequestLevelEnum::MLSEVERE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LogMessageRequestLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LeadStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The lead's state in relation to the company.
pub enum LeadStateEnum {
    

    /// Unchosen.
    ///
    /// "LEAD_STATE_UNSPECIFIED"
    #[serde(rename="LEAD_STATE_UNSPECIFIED")]
    LEADSTATEUNSPECIFIED,
    

    /// Lead not yet contacted.
    ///
    /// "LEAD"
    #[serde(rename="LEAD")]
    LEAD,
    

    /// Lead has been contacted.
    ///
    /// "CONTACTED"
    #[serde(rename="CONTACTED")]
    CONTACTED,
    

    /// Lead has become a client.
    ///
    /// "CLIENT"
    #[serde(rename="CLIENT")]
    CLIENT,
    

    /// Lead in a state not covered by other options.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for LeadStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LeadStateEnum::LEADSTATEUNSPECIFIED => "LEAD_STATE_UNSPECIFIED",
            LeadStateEnum::LEAD => "LEAD",
            LeadStateEnum::CONTACTED => "CONTACTED",
            LeadStateEnum::CLIENT => "CLIENT",
            LeadStateEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for LeadStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LEAD_STATE_UNSPECIFIED" => Ok(LeadStateEnum::LEADSTATEUNSPECIFIED),
           "LEAD" => Ok(LeadStateEnum::LEAD),
           "CONTACTED" => Ok(LeadStateEnum::CONTACTED),
           "CLIENT" => Ok(LeadStateEnum::CLIENT),
           "OTHER" => Ok(LeadStateEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LeadStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LeadGpsMotivationsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of reasons for using Google Partner Search and creating a lead.
pub enum LeadGpsMotivationsEnum {
    

    /// Unchosen.
    ///
    /// "GPS_MOTIVATION_UNSPECIFIED"
    #[serde(rename="GPS_MOTIVATION_UNSPECIFIED")]
    GPSMOTIVATIONUNSPECIFIED,
    

    /// Advertiser needs help with their advertising.
    ///
    /// "GPSM_HELP_WITH_ADVERTISING"
    #[serde(rename="GPSM_HELP_WITH_ADVERTISING")]
    GPSMHELPWITHADVERTISING,
    

    /// Advertiser needs help with their website.
    ///
    /// "GPSM_HELP_WITH_WEBSITE"
    #[serde(rename="GPSM_HELP_WITH_WEBSITE")]
    GPSMHELPWITHWEBSITE,
    

    /// Advertiser does not have a website.
    ///
    /// "GPSM_NO_WEBSITE"
    #[serde(rename="GPSM_NO_WEBSITE")]
    GPSMNOWEBSITE,
}

impl AsRef<str> for LeadGpsMotivationsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LeadGpsMotivationsEnum::GPSMOTIVATIONUNSPECIFIED => "GPS_MOTIVATION_UNSPECIFIED",
            LeadGpsMotivationsEnum::GPSMHELPWITHADVERTISING => "GPSM_HELP_WITH_ADVERTISING",
            LeadGpsMotivationsEnum::GPSMHELPWITHWEBSITE => "GPSM_HELP_WITH_WEBSITE",
            LeadGpsMotivationsEnum::GPSMNOWEBSITE => "GPSM_NO_WEBSITE",
        }
    }
}

impl std::convert::TryFrom< &str> for LeadGpsMotivationsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GPS_MOTIVATION_UNSPECIFIED" => Ok(LeadGpsMotivationsEnum::GPSMOTIVATIONUNSPECIFIED),
           "GPSM_HELP_WITH_ADVERTISING" => Ok(LeadGpsMotivationsEnum::GPSMHELPWITHADVERTISING),
           "GPSM_HELP_WITH_WEBSITE" => Ok(LeadGpsMotivationsEnum::GPSMHELPWITHWEBSITE),
           "GPSM_NO_WEBSITE" => Ok(LeadGpsMotivationsEnum::GPSMNOWEBSITE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LeadGpsMotivationsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LeadTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of lead.
pub enum LeadTypeEnum {
    

    /// Unchosen.
    ///
    /// "LEAD_TYPE_UNSPECIFIED"
    #[serde(rename="LEAD_TYPE_UNSPECIFIED")]
    LEADTYPEUNSPECIFIED,
    

    /// Google Partner Search.
    ///
    /// "LT_GPS"
    #[serde(rename="LT_GPS")]
    LTGPS,
}

impl AsRef<str> for LeadTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LeadTypeEnum::LEADTYPEUNSPECIFIED => "LEAD_TYPE_UNSPECIFIED",
            LeadTypeEnum::LTGPS => "LT_GPS",
        }
    }
}

impl std::convert::TryFrom< &str> for LeadTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LEAD_TYPE_UNSPECIFIED" => Ok(LeadTypeEnum::LEADTYPEUNSPECIFIED),
           "LT_GPS" => Ok(LeadTypeEnum::LTGPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LeadTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ListUserStatesResponseUserStatesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// User's states.
pub enum ListUserStatesResponseUserStatesEnum {
    

    /// Unchosen.
    ///
    /// "USER_STATE_UNSPECIFIED"
    #[serde(rename="USER_STATE_UNSPECIFIED")]
    USERSTATEUNSPECIFIED,
    

    /// User must pass <a href="https://www.google.com/recaptcha/">reCaptcha</a> to
contact a Partner via Google Partner Search.
    ///
    /// "US_REQUIRES_RECAPTCHA_FOR_GPS_CONTACT"
    #[serde(rename="US_REQUIRES_RECAPTCHA_FOR_GPS_CONTACT")]
    USREQUIRESRECAPTCHAFORGPSCONTACT,
}

impl AsRef<str> for ListUserStatesResponseUserStatesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ListUserStatesResponseUserStatesEnum::USERSTATEUNSPECIFIED => "USER_STATE_UNSPECIFIED",
            ListUserStatesResponseUserStatesEnum::USREQUIRESRECAPTCHAFORGPSCONTACT => "US_REQUIRES_RECAPTCHA_FOR_GPS_CONTACT",
        }
    }
}

impl std::convert::TryFrom< &str> for ListUserStatesResponseUserStatesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_STATE_UNSPECIFIED" => Ok(ListUserStatesResponseUserStatesEnum::USERSTATEUNSPECIFIED),
           "US_REQUIRES_RECAPTCHA_FOR_GPS_CONTACT" => Ok(ListUserStatesResponseUserStatesEnum::USREQUIRESRECAPTCHAFORGPSCONTACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ListUserStatesResponseUserStatesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyRelationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of relationship, in terms of approvals.
pub enum CompanyRelationStateEnum {
    

    /// Default unspecified value.
    ///
    /// "USER_COMPANY_REATION_STATE_NONE_SPECIFIED"
    #[serde(rename="USER_COMPANY_REATION_STATE_NONE_SPECIFIED")]
    USERCOMPANYREATIONSTATENONESPECIFIED,
    

    /// User has filled in a request to be associated with an company.
Now waiting email confirmation.
    ///
    /// "USER_COMPANY_RELATION_STATE_AWAIT_EMAIL"
    #[serde(rename="USER_COMPANY_RELATION_STATE_AWAIT_EMAIL")]
    USERCOMPANYRELATIONSTATEAWAITEMAIL,
    

    /// Pending approval from company.
Email confirmation will not approve this one.
    ///
    /// "USER_COMPANY_RELATION_STATE_AWAIT_ADMIN"
    #[serde(rename="USER_COMPANY_RELATION_STATE_AWAIT_ADMIN")]
    USERCOMPANYRELATIONSTATEAWAITADMIN,
    

    /// Approved by company.
    ///
    /// "USER_COMPANY_RELATION_STATE_APPROVED"
    #[serde(rename="USER_COMPANY_RELATION_STATE_APPROVED")]
    USERCOMPANYRELATIONSTATEAPPROVED,
}

impl AsRef<str> for CompanyRelationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyRelationStateEnum::USERCOMPANYREATIONSTATENONESPECIFIED => "USER_COMPANY_REATION_STATE_NONE_SPECIFIED",
            CompanyRelationStateEnum::USERCOMPANYRELATIONSTATEAWAITEMAIL => "USER_COMPANY_RELATION_STATE_AWAIT_EMAIL",
            CompanyRelationStateEnum::USERCOMPANYRELATIONSTATEAWAITADMIN => "USER_COMPANY_RELATION_STATE_AWAIT_ADMIN",
            CompanyRelationStateEnum::USERCOMPANYRELATIONSTATEAPPROVED => "USER_COMPANY_RELATION_STATE_APPROVED",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyRelationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_COMPANY_REATION_STATE_NONE_SPECIFIED" => Ok(CompanyRelationStateEnum::USERCOMPANYREATIONSTATENONESPECIFIED),
           "USER_COMPANY_RELATION_STATE_AWAIT_EMAIL" => Ok(CompanyRelationStateEnum::USERCOMPANYRELATIONSTATEAWAITEMAIL),
           "USER_COMPANY_RELATION_STATE_AWAIT_ADMIN" => Ok(CompanyRelationStateEnum::USERCOMPANYRELATIONSTATEAWAITADMIN),
           "USER_COMPANY_RELATION_STATE_APPROVED" => Ok(CompanyRelationStateEnum::USERCOMPANYRELATIONSTATEAPPROVED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyRelationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyRelationSegmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The segment the company is classified as.
pub enum CompanyRelationSegmentEnum {
    

    /// Default segment indicates an unknown.
    ///
    /// "COMPANY_SEGMENT_UNKNOWN"
    #[serde(rename="COMPANY_SEGMENT_UNKNOWN")]
    COMPANYSEGMENTUNKNOWN,
    

    /// Segment representing a selected group of Partners
    ///
    /// "COMPANY_SEGMENT_NAL"
    #[serde(rename="COMPANY_SEGMENT_NAL")]
    COMPANYSEGMENTNAL,
    

    /// Segment representing Premier SMB Partners, an AdWords partnership program.
    ///
    /// "COMPANY_SEGMENT_PSP"
    #[serde(rename="COMPANY_SEGMENT_PSP")]
    COMPANYSEGMENTPSP,
    

    /// A segment of Premier SMB Partners that have relationship with Google.
    ///
    /// "COMPANY_SEGMENT_PPSP"
    #[serde(rename="COMPANY_SEGMENT_PPSP")]
    COMPANYSEGMENTPPSP,
}

impl AsRef<str> for CompanyRelationSegmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyRelationSegmentEnum::COMPANYSEGMENTUNKNOWN => "COMPANY_SEGMENT_UNKNOWN",
            CompanyRelationSegmentEnum::COMPANYSEGMENTNAL => "COMPANY_SEGMENT_NAL",
            CompanyRelationSegmentEnum::COMPANYSEGMENTPSP => "COMPANY_SEGMENT_PSP",
            CompanyRelationSegmentEnum::COMPANYSEGMENTPPSP => "COMPANY_SEGMENT_PPSP",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyRelationSegmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPANY_SEGMENT_UNKNOWN" => Ok(CompanyRelationSegmentEnum::COMPANYSEGMENTUNKNOWN),
           "COMPANY_SEGMENT_NAL" => Ok(CompanyRelationSegmentEnum::COMPANYSEGMENTNAL),
           "COMPANY_SEGMENT_PSP" => Ok(CompanyRelationSegmentEnum::COMPANYSEGMENTPSP),
           "COMPANY_SEGMENT_PPSP" => Ok(CompanyRelationSegmentEnum::COMPANYSEGMENTPPSP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyRelationSegmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyRelationBadgeTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the company is a Partner.
pub enum CompanyRelationBadgeTierEnum {
    

    /// Tier badge is not set.
    ///
    /// "BADGE_TIER_NONE"
    #[serde(rename="BADGE_TIER_NONE")]
    BADGETIERNONE,
    

    /// Agency has regular partner badge.
    ///
    /// "BADGE_TIER_REGULAR"
    #[serde(rename="BADGE_TIER_REGULAR")]
    BADGETIERREGULAR,
    

    /// Agency has premier badge.
    ///
    /// "BADGE_TIER_PREMIER"
    #[serde(rename="BADGE_TIER_PREMIER")]
    BADGETIERPREMIER,
}

impl AsRef<str> for CompanyRelationBadgeTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyRelationBadgeTierEnum::BADGETIERNONE => "BADGE_TIER_NONE",
            CompanyRelationBadgeTierEnum::BADGETIERREGULAR => "BADGE_TIER_REGULAR",
            CompanyRelationBadgeTierEnum::BADGETIERPREMIER => "BADGE_TIER_PREMIER",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyRelationBadgeTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BADGE_TIER_NONE" => Ok(CompanyRelationBadgeTierEnum::BADGETIERNONE),
           "BADGE_TIER_REGULAR" => Ok(CompanyRelationBadgeTierEnum::BADGETIERREGULAR),
           "BADGE_TIER_PREMIER" => Ok(CompanyRelationBadgeTierEnum::BADGETIERPREMIER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyRelationBadgeTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The view of the `Company` resource to be returned. This must not be
`COMPANY_VIEW_UNSPECIFIED`.
pub enum CompanyViewEnum {
    

    /// no description found
    ///
    /// "COMPANY_VIEW_UNSPECIFIED"
    #[serde(rename="COMPANY_VIEW_UNSPECIFIED")]
    COMPANYVIEWUNSPECIFIED,
    

    /// no description found
    ///
    /// "CV_GOOGLE_PARTNER_SEARCH"
    #[serde(rename="CV_GOOGLE_PARTNER_SEARCH")]
    CVGOOGLEPARTNERSEARCH,
}

impl AsRef<str> for CompanyViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyViewEnum::COMPANYVIEWUNSPECIFIED => "COMPANY_VIEW_UNSPECIFIED",
            CompanyViewEnum::CVGOOGLEPARTNERSEARCH => "CV_GOOGLE_PARTNER_SEARCH",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPANY_VIEW_UNSPECIFIED" => Ok(CompanyViewEnum::COMPANYVIEWUNSPECIFIED),
           "CV_GOOGLE_PARTNER_SEARCH" => Ok(CompanyViewEnum::CVGOOGLEPARTNERSEARCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanySpecializationsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of specializations that the returned agencies should provide. If this
is not empty, any returned agency must have at least one of these
specializations, or one of the services in the "services" field.
pub enum CompanySpecializationsEnum {
    

    /// no description found
    ///
    /// "BADGE_SPECIALIZATION_UNKNOWN"
    #[serde(rename="BADGE_SPECIALIZATION_UNKNOWN")]
    BADGESPECIALIZATIONUNKNOWN,
    

    /// no description found
    ///
    /// "BADGE_SPECIALIZATION_ADWORDS_SEARCH"
    #[serde(rename="BADGE_SPECIALIZATION_ADWORDS_SEARCH")]
    BADGESPECIALIZATIONADWORDSSEARCH,
    

    /// no description found
    ///
    /// "BADGE_SPECIALIZATION_ADWORDS_DISPLAY"
    #[serde(rename="BADGE_SPECIALIZATION_ADWORDS_DISPLAY")]
    BADGESPECIALIZATIONADWORDSDISPLAY,
    

    /// no description found
    ///
    /// "BADGE_SPECIALIZATION_ADWORDS_MOBILE"
    #[serde(rename="BADGE_SPECIALIZATION_ADWORDS_MOBILE")]
    BADGESPECIALIZATIONADWORDSMOBILE,
    

    /// no description found
    ///
    /// "BADGE_SPECIALIZATION_ADWORDS_VIDEO"
    #[serde(rename="BADGE_SPECIALIZATION_ADWORDS_VIDEO")]
    BADGESPECIALIZATIONADWORDSVIDEO,
    

    /// no description found
    ///
    /// "BADGE_SPECIALIZATION_ADWORDS_SHOPPING"
    #[serde(rename="BADGE_SPECIALIZATION_ADWORDS_SHOPPING")]
    BADGESPECIALIZATIONADWORDSSHOPPING,
}

impl AsRef<str> for CompanySpecializationsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanySpecializationsEnum::BADGESPECIALIZATIONUNKNOWN => "BADGE_SPECIALIZATION_UNKNOWN",
            CompanySpecializationsEnum::BADGESPECIALIZATIONADWORDSSEARCH => "BADGE_SPECIALIZATION_ADWORDS_SEARCH",
            CompanySpecializationsEnum::BADGESPECIALIZATIONADWORDSDISPLAY => "BADGE_SPECIALIZATION_ADWORDS_DISPLAY",
            CompanySpecializationsEnum::BADGESPECIALIZATIONADWORDSMOBILE => "BADGE_SPECIALIZATION_ADWORDS_MOBILE",
            CompanySpecializationsEnum::BADGESPECIALIZATIONADWORDSVIDEO => "BADGE_SPECIALIZATION_ADWORDS_VIDEO",
            CompanySpecializationsEnum::BADGESPECIALIZATIONADWORDSSHOPPING => "BADGE_SPECIALIZATION_ADWORDS_SHOPPING",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanySpecializationsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BADGE_SPECIALIZATION_UNKNOWN" => Ok(CompanySpecializationsEnum::BADGESPECIALIZATIONUNKNOWN),
           "BADGE_SPECIALIZATION_ADWORDS_SEARCH" => Ok(CompanySpecializationsEnum::BADGESPECIALIZATIONADWORDSSEARCH),
           "BADGE_SPECIALIZATION_ADWORDS_DISPLAY" => Ok(CompanySpecializationsEnum::BADGESPECIALIZATIONADWORDSDISPLAY),
           "BADGE_SPECIALIZATION_ADWORDS_MOBILE" => Ok(CompanySpecializationsEnum::BADGESPECIALIZATIONADWORDSMOBILE),
           "BADGE_SPECIALIZATION_ADWORDS_VIDEO" => Ok(CompanySpecializationsEnum::BADGESPECIALIZATIONADWORDSVIDEO),
           "BADGE_SPECIALIZATION_ADWORDS_SHOPPING" => Ok(CompanySpecializationsEnum::BADGESPECIALIZATIONADWORDSSHOPPING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanySpecializationsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyServicesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of services that the returned agencies should provide. If this is
not empty, any returned agency must have at least one of these services,
or one of the specializations in the "specializations" field.
pub enum CompanyServicesEnum {
    

    /// no description found
    ///
    /// "SERVICE_UNSPECIFIED"
    #[serde(rename="SERVICE_UNSPECIFIED")]
    SERVICEUNSPECIFIED,
    

    /// no description found
    ///
    /// "S_ADVANCED_ADWORDS_SUPPORT"
    #[serde(rename="S_ADVANCED_ADWORDS_SUPPORT")]
    SADVANCEDADWORDSSUPPORT,
    

    /// no description found
    ///
    /// "S_ADVERTISING_ON_GOOGLE"
    #[serde(rename="S_ADVERTISING_ON_GOOGLE")]
    SADVERTISINGONGOOGLE,
    

    /// no description found
    ///
    /// "S_AN_ENHANCED_WEBSITE"
    #[serde(rename="S_AN_ENHANCED_WEBSITE")]
    SANENHANCEDWEBSITE,
    

    /// no description found
    ///
    /// "S_AN_ONLINE_MARKETING_PLAN"
    #[serde(rename="S_AN_ONLINE_MARKETING_PLAN")]
    SANONLINEMARKETINGPLAN,
    

    /// no description found
    ///
    /// "S_MOBILE_AND_VIDEO_ADS"
    #[serde(rename="S_MOBILE_AND_VIDEO_ADS")]
    SMOBILEANDVIDEOADS,
    

    /// no description found
    ///
    /// "S_MOBILE_WEBSITE_SERVICES"
    #[serde(rename="S_MOBILE_WEBSITE_SERVICES")]
    SMOBILEWEBSITESERVICES,
}

impl AsRef<str> for CompanyServicesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyServicesEnum::SERVICEUNSPECIFIED => "SERVICE_UNSPECIFIED",
            CompanyServicesEnum::SADVANCEDADWORDSSUPPORT => "S_ADVANCED_ADWORDS_SUPPORT",
            CompanyServicesEnum::SADVERTISINGONGOOGLE => "S_ADVERTISING_ON_GOOGLE",
            CompanyServicesEnum::SANENHANCEDWEBSITE => "S_AN_ENHANCED_WEBSITE",
            CompanyServicesEnum::SANONLINEMARKETINGPLAN => "S_AN_ONLINE_MARKETING_PLAN",
            CompanyServicesEnum::SMOBILEANDVIDEOADS => "S_MOBILE_AND_VIDEO_ADS",
            CompanyServicesEnum::SMOBILEWEBSITESERVICES => "S_MOBILE_WEBSITE_SERVICES",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyServicesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVICE_UNSPECIFIED" => Ok(CompanyServicesEnum::SERVICEUNSPECIFIED),
           "S_ADVANCED_ADWORDS_SUPPORT" => Ok(CompanyServicesEnum::SADVANCEDADWORDSSUPPORT),
           "S_ADVERTISING_ON_GOOGLE" => Ok(CompanyServicesEnum::SADVERTISINGONGOOGLE),
           "S_AN_ENHANCED_WEBSITE" => Ok(CompanyServicesEnum::SANENHANCEDWEBSITE),
           "S_AN_ONLINE_MARKETING_PLAN" => Ok(CompanyServicesEnum::SANONLINEMARKETINGPLAN),
           "S_MOBILE_AND_VIDEO_ADS" => Ok(CompanyServicesEnum::SMOBILEANDVIDEOADS),
           "S_MOBILE_WEBSITE_SERVICES" => Ok(CompanyServicesEnum::SMOBILEWEBSITESERVICES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyServicesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyIndustriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of industries the company can help with.
pub enum CompanyIndustriesEnum {
    

    /// no description found
    ///
    /// "INDUSTRY_UNSPECIFIED"
    #[serde(rename="INDUSTRY_UNSPECIFIED")]
    INDUSTRYUNSPECIFIED,
    

    /// no description found
    ///
    /// "I_AUTOMOTIVE"
    #[serde(rename="I_AUTOMOTIVE")]
    IAUTOMOTIVE,
    

    /// no description found
    ///
    /// "I_BUSINESS_TO_BUSINESS"
    #[serde(rename="I_BUSINESS_TO_BUSINESS")]
    IBUSINESSTOBUSINESS,
    

    /// no description found
    ///
    /// "I_CONSUMER_PACKAGED_GOODS"
    #[serde(rename="I_CONSUMER_PACKAGED_GOODS")]
    ICONSUMERPACKAGEDGOODS,
    

    /// no description found
    ///
    /// "I_EDUCATION"
    #[serde(rename="I_EDUCATION")]
    IEDUCATION,
    

    /// no description found
    ///
    /// "I_FINANCE"
    #[serde(rename="I_FINANCE")]
    IFINANCE,
    

    /// no description found
    ///
    /// "I_HEALTHCARE"
    #[serde(rename="I_HEALTHCARE")]
    IHEALTHCARE,
    

    /// no description found
    ///
    /// "I_MEDIA_AND_ENTERTAINMENT"
    #[serde(rename="I_MEDIA_AND_ENTERTAINMENT")]
    IMEDIAANDENTERTAINMENT,
    

    /// no description found
    ///
    /// "I_RETAIL"
    #[serde(rename="I_RETAIL")]
    IRETAIL,
    

    /// no description found
    ///
    /// "I_TECHNOLOGY"
    #[serde(rename="I_TECHNOLOGY")]
    ITECHNOLOGY,
    

    /// no description found
    ///
    /// "I_TRAVEL"
    #[serde(rename="I_TRAVEL")]
    ITRAVEL,
}

impl AsRef<str> for CompanyIndustriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyIndustriesEnum::INDUSTRYUNSPECIFIED => "INDUSTRY_UNSPECIFIED",
            CompanyIndustriesEnum::IAUTOMOTIVE => "I_AUTOMOTIVE",
            CompanyIndustriesEnum::IBUSINESSTOBUSINESS => "I_BUSINESS_TO_BUSINESS",
            CompanyIndustriesEnum::ICONSUMERPACKAGEDGOODS => "I_CONSUMER_PACKAGED_GOODS",
            CompanyIndustriesEnum::IEDUCATION => "I_EDUCATION",
            CompanyIndustriesEnum::IFINANCE => "I_FINANCE",
            CompanyIndustriesEnum::IHEALTHCARE => "I_HEALTHCARE",
            CompanyIndustriesEnum::IMEDIAANDENTERTAINMENT => "I_MEDIA_AND_ENTERTAINMENT",
            CompanyIndustriesEnum::IRETAIL => "I_RETAIL",
            CompanyIndustriesEnum::ITECHNOLOGY => "I_TECHNOLOGY",
            CompanyIndustriesEnum::ITRAVEL => "I_TRAVEL",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyIndustriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INDUSTRY_UNSPECIFIED" => Ok(CompanyIndustriesEnum::INDUSTRYUNSPECIFIED),
           "I_AUTOMOTIVE" => Ok(CompanyIndustriesEnum::IAUTOMOTIVE),
           "I_BUSINESS_TO_BUSINESS" => Ok(CompanyIndustriesEnum::IBUSINESSTOBUSINESS),
           "I_CONSUMER_PACKAGED_GOODS" => Ok(CompanyIndustriesEnum::ICONSUMERPACKAGEDGOODS),
           "I_EDUCATION" => Ok(CompanyIndustriesEnum::IEDUCATION),
           "I_FINANCE" => Ok(CompanyIndustriesEnum::IFINANCE),
           "I_HEALTHCARE" => Ok(CompanyIndustriesEnum::IHEALTHCARE),
           "I_MEDIA_AND_ENTERTAINMENT" => Ok(CompanyIndustriesEnum::IMEDIAANDENTERTAINMENT),
           "I_RETAIL" => Ok(CompanyIndustriesEnum::IRETAIL),
           "I_TECHNOLOGY" => Ok(CompanyIndustriesEnum::ITECHNOLOGY),
           "I_TRAVEL" => Ok(CompanyIndustriesEnum::ITRAVEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyIndustriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyGpsMotivationsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of reasons for using Google Partner Search to get companies.
pub enum CompanyGpsMotivationsEnum {
    

    /// no description found
    ///
    /// "GPS_MOTIVATION_UNSPECIFIED"
    #[serde(rename="GPS_MOTIVATION_UNSPECIFIED")]
    GPSMOTIVATIONUNSPECIFIED,
    

    /// no description found
    ///
    /// "GPSM_HELP_WITH_ADVERTISING"
    #[serde(rename="GPSM_HELP_WITH_ADVERTISING")]
    GPSMHELPWITHADVERTISING,
    

    /// no description found
    ///
    /// "GPSM_HELP_WITH_WEBSITE"
    #[serde(rename="GPSM_HELP_WITH_WEBSITE")]
    GPSMHELPWITHWEBSITE,
    

    /// no description found
    ///
    /// "GPSM_NO_WEBSITE"
    #[serde(rename="GPSM_NO_WEBSITE")]
    GPSMNOWEBSITE,
}

impl AsRef<str> for CompanyGpsMotivationsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyGpsMotivationsEnum::GPSMOTIVATIONUNSPECIFIED => "GPS_MOTIVATION_UNSPECIFIED",
            CompanyGpsMotivationsEnum::GPSMHELPWITHADVERTISING => "GPSM_HELP_WITH_ADVERTISING",
            CompanyGpsMotivationsEnum::GPSMHELPWITHWEBSITE => "GPSM_HELP_WITH_WEBSITE",
            CompanyGpsMotivationsEnum::GPSMNOWEBSITE => "GPSM_NO_WEBSITE",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyGpsMotivationsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GPS_MOTIVATION_UNSPECIFIED" => Ok(CompanyGpsMotivationsEnum::GPSMOTIVATIONUNSPECIFIED),
           "GPSM_HELP_WITH_ADVERTISING" => Ok(CompanyGpsMotivationsEnum::GPSMHELPWITHADVERTISING),
           "GPSM_HELP_WITH_WEBSITE" => Ok(CompanyGpsMotivationsEnum::GPSMHELPWITHWEBSITE),
           "GPSM_NO_WEBSITE" => Ok(CompanyGpsMotivationsEnum::GPSMNOWEBSITE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyGpsMotivationsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserUserViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies what parts of the user information to return.
pub enum UserUserViewEnum {
    

    /// no description found
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// no description found
    ///
    /// "PROFILE"
    #[serde(rename="PROFILE")]
    PROFILE,
    

    /// no description found
    ///
    /// "PUBLIC_PROFILE"
    #[serde(rename="PUBLIC_PROFILE")]
    PUBLICPROFILE,
}

impl AsRef<str> for UserUserViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserUserViewEnum::BASIC => "BASIC",
            UserUserViewEnum::PROFILE => "PROFILE",
            UserUserViewEnum::PUBLICPROFILE => "PUBLIC_PROFILE",
        }
    }
}

impl std::convert::TryFrom< &str> for UserUserViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC" => Ok(UserUserViewEnum::BASIC),
           "PROFILE" => Ok(UserUserViewEnum::PROFILE),
           "PUBLIC_PROFILE" => Ok(UserUserViewEnum::PUBLICPROFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserUserViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


