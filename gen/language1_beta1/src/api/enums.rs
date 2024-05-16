use super::*;



// region AnalyzeEntitiesRequestEncodingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The encoding type used by the API to calculate offsets.
pub enum AnalyzeEntitiesRequestEncodingTypeEnum {
    

    /// If `EncodingType` is not specified, encoding-dependent information (such as `begin_offset`) will be set at `-1`.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-8 encoding of the input. C++ and Go are examples of languages that use this encoding natively.
    ///
    /// "UTF8"
    #[serde(rename="UTF8")]
    UTF8,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-16 encoding of the input. Java and Javascript are examples of languages that use this encoding natively.
    ///
    /// "UTF16"
    #[serde(rename="UTF16")]
    UTF16,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-32 encoding of the input. Python is an example of a language that uses this encoding natively.
    ///
    /// "UTF32"
    #[serde(rename="UTF32")]
    UTF32,
}

impl AsRef<str> for AnalyzeEntitiesRequestEncodingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AnalyzeEntitiesRequestEncodingTypeEnum::NONE => "NONE",
            AnalyzeEntitiesRequestEncodingTypeEnum::UTF8 => "UTF8",
            AnalyzeEntitiesRequestEncodingTypeEnum::UTF16 => "UTF16",
            AnalyzeEntitiesRequestEncodingTypeEnum::UTF32 => "UTF32",
        }
    }
}

impl std::convert::TryFrom< &str> for AnalyzeEntitiesRequestEncodingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(AnalyzeEntitiesRequestEncodingTypeEnum::NONE),
           "UTF8" => Ok(AnalyzeEntitiesRequestEncodingTypeEnum::UTF8),
           "UTF16" => Ok(AnalyzeEntitiesRequestEncodingTypeEnum::UTF16),
           "UTF32" => Ok(AnalyzeEntitiesRequestEncodingTypeEnum::UTF32),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AnalyzeEntitiesRequestEncodingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AnalyzeSentimentRequestEncodingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The encoding type used by the API to calculate sentence offsets for the sentence sentiment.
pub enum AnalyzeSentimentRequestEncodingTypeEnum {
    

    /// If `EncodingType` is not specified, encoding-dependent information (such as `begin_offset`) will be set at `-1`.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-8 encoding of the input. C++ and Go are examples of languages that use this encoding natively.
    ///
    /// "UTF8"
    #[serde(rename="UTF8")]
    UTF8,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-16 encoding of the input. Java and Javascript are examples of languages that use this encoding natively.
    ///
    /// "UTF16"
    #[serde(rename="UTF16")]
    UTF16,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-32 encoding of the input. Python is an example of a language that uses this encoding natively.
    ///
    /// "UTF32"
    #[serde(rename="UTF32")]
    UTF32,
}

impl AsRef<str> for AnalyzeSentimentRequestEncodingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AnalyzeSentimentRequestEncodingTypeEnum::NONE => "NONE",
            AnalyzeSentimentRequestEncodingTypeEnum::UTF8 => "UTF8",
            AnalyzeSentimentRequestEncodingTypeEnum::UTF16 => "UTF16",
            AnalyzeSentimentRequestEncodingTypeEnum::UTF32 => "UTF32",
        }
    }
}

impl std::convert::TryFrom< &str> for AnalyzeSentimentRequestEncodingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(AnalyzeSentimentRequestEncodingTypeEnum::NONE),
           "UTF8" => Ok(AnalyzeSentimentRequestEncodingTypeEnum::UTF8),
           "UTF16" => Ok(AnalyzeSentimentRequestEncodingTypeEnum::UTF16),
           "UTF32" => Ok(AnalyzeSentimentRequestEncodingTypeEnum::UTF32),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AnalyzeSentimentRequestEncodingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AnalyzeSyntaxRequestEncodingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The encoding type used by the API to calculate offsets.
pub enum AnalyzeSyntaxRequestEncodingTypeEnum {
    

    /// If `EncodingType` is not specified, encoding-dependent information (such as `begin_offset`) will be set at `-1`.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-8 encoding of the input. C++ and Go are examples of languages that use this encoding natively.
    ///
    /// "UTF8"
    #[serde(rename="UTF8")]
    UTF8,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-16 encoding of the input. Java and Javascript are examples of languages that use this encoding natively.
    ///
    /// "UTF16"
    #[serde(rename="UTF16")]
    UTF16,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-32 encoding of the input. Python is an example of a language that uses this encoding natively.
    ///
    /// "UTF32"
    #[serde(rename="UTF32")]
    UTF32,
}

impl AsRef<str> for AnalyzeSyntaxRequestEncodingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AnalyzeSyntaxRequestEncodingTypeEnum::NONE => "NONE",
            AnalyzeSyntaxRequestEncodingTypeEnum::UTF8 => "UTF8",
            AnalyzeSyntaxRequestEncodingTypeEnum::UTF16 => "UTF16",
            AnalyzeSyntaxRequestEncodingTypeEnum::UTF32 => "UTF32",
        }
    }
}

impl std::convert::TryFrom< &str> for AnalyzeSyntaxRequestEncodingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(AnalyzeSyntaxRequestEncodingTypeEnum::NONE),
           "UTF8" => Ok(AnalyzeSyntaxRequestEncodingTypeEnum::UTF8),
           "UTF16" => Ok(AnalyzeSyntaxRequestEncodingTypeEnum::UTF16),
           "UTF32" => Ok(AnalyzeSyntaxRequestEncodingTypeEnum::UTF32),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AnalyzeSyntaxRequestEncodingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AnnotateTextRequestEncodingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The encoding type used by the API to calculate offsets.
pub enum AnnotateTextRequestEncodingTypeEnum {
    

    /// If `EncodingType` is not specified, encoding-dependent information (such as `begin_offset`) will be set at `-1`.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-8 encoding of the input. C++ and Go are examples of languages that use this encoding natively.
    ///
    /// "UTF8"
    #[serde(rename="UTF8")]
    UTF8,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-16 encoding of the input. Java and Javascript are examples of languages that use this encoding natively.
    ///
    /// "UTF16"
    #[serde(rename="UTF16")]
    UTF16,
    

    /// Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-32 encoding of the input. Python is an example of a language that uses this encoding natively.
    ///
    /// "UTF32"
    #[serde(rename="UTF32")]
    UTF32,
}

impl AsRef<str> for AnnotateTextRequestEncodingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AnnotateTextRequestEncodingTypeEnum::NONE => "NONE",
            AnnotateTextRequestEncodingTypeEnum::UTF8 => "UTF8",
            AnnotateTextRequestEncodingTypeEnum::UTF16 => "UTF16",
            AnnotateTextRequestEncodingTypeEnum::UTF32 => "UTF32",
        }
    }
}

impl std::convert::TryFrom< &str> for AnnotateTextRequestEncodingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(AnnotateTextRequestEncodingTypeEnum::NONE),
           "UTF8" => Ok(AnnotateTextRequestEncodingTypeEnum::UTF8),
           "UTF16" => Ok(AnnotateTextRequestEncodingTypeEnum::UTF16),
           "UTF32" => Ok(AnnotateTextRequestEncodingTypeEnum::UTF32),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AnnotateTextRequestEncodingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DependencyEdgeLabelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The parse label for the token.
pub enum DependencyEdgeLabelEnum {
    

    /// Unknown
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Abbreviation modifier
    ///
    /// "ABBREV"
    #[serde(rename="ABBREV")]
    ABBREV,
    

    /// Adjectival complement
    ///
    /// "ACOMP"
    #[serde(rename="ACOMP")]
    ACOMP,
    

    /// Adverbial clause modifier
    ///
    /// "ADVCL"
    #[serde(rename="ADVCL")]
    ADVCL,
    

    /// Adverbial modifier
    ///
    /// "ADVMOD"
    #[serde(rename="ADVMOD")]
    ADVMOD,
    

    /// Adjectival modifier of an NP
    ///
    /// "AMOD"
    #[serde(rename="AMOD")]
    AMOD,
    

    /// Appositional modifier of an NP
    ///
    /// "APPOS"
    #[serde(rename="APPOS")]
    APPOS,
    

    /// Attribute dependent of a copular verb
    ///
    /// "ATTR"
    #[serde(rename="ATTR")]
    ATTR,
    

    /// Auxiliary (non-main) verb
    ///
    /// "AUX"
    #[serde(rename="AUX")]
    AUX,
    

    /// Passive auxiliary
    ///
    /// "AUXPASS"
    #[serde(rename="AUXPASS")]
    AUXPASS,
    

    /// Coordinating conjunction
    ///
    /// "CC"
    #[serde(rename="CC")]
    CC,
    

    /// Clausal complement of a verb or adjective
    ///
    /// "CCOMP"
    #[serde(rename="CCOMP")]
    CCOMP,
    

    /// Conjunct
    ///
    /// "CONJ"
    #[serde(rename="CONJ")]
    CONJ,
    

    /// Clausal subject
    ///
    /// "CSUBJ"
    #[serde(rename="CSUBJ")]
    CSUBJ,
    

    /// Clausal passive subject
    ///
    /// "CSUBJPASS"
    #[serde(rename="CSUBJPASS")]
    CSUBJPASS,
    

    /// Dependency (unable to determine)
    ///
    /// "DEP"
    #[serde(rename="DEP")]
    DEP,
    

    /// Determiner
    ///
    /// "DET"
    #[serde(rename="DET")]
    DET,
    

    /// Discourse
    ///
    /// "DISCOURSE"
    #[serde(rename="DISCOURSE")]
    DISCOURSE,
    

    /// Direct object
    ///
    /// "DOBJ"
    #[serde(rename="DOBJ")]
    DOBJ,
    

    /// Expletive
    ///
    /// "EXPL"
    #[serde(rename="EXPL")]
    EXPL,
    

    /// Goes with (part of a word in a text not well edited)
    ///
    /// "GOESWITH"
    #[serde(rename="GOESWITH")]
    GOESWITH,
    

    /// Indirect object
    ///
    /// "IOBJ"
    #[serde(rename="IOBJ")]
    IOBJ,
    

    /// Marker (word introducing a subordinate clause)
    ///
    /// "MARK"
    #[serde(rename="MARK")]
    MARK,
    

    /// Multi-word expression
    ///
    /// "MWE"
    #[serde(rename="MWE")]
    MWE,
    

    /// Multi-word verbal expression
    ///
    /// "MWV"
    #[serde(rename="MWV")]
    MWV,
    

    /// Negation modifier
    ///
    /// "NEG"
    #[serde(rename="NEG")]
    NEG,
    

    /// Noun compound modifier
    ///
    /// "NN"
    #[serde(rename="NN")]
    NN,
    

    /// Noun phrase used as an adverbial modifier
    ///
    /// "NPADVMOD"
    #[serde(rename="NPADVMOD")]
    NPADVMOD,
    

    /// Nominal subject
    ///
    /// "NSUBJ"
    #[serde(rename="NSUBJ")]
    NSUBJ,
    

    /// Passive nominal subject
    ///
    /// "NSUBJPASS"
    #[serde(rename="NSUBJPASS")]
    NSUBJPASS,
    

    /// Numeric modifier of a noun
    ///
    /// "NUM"
    #[serde(rename="NUM")]
    NUM,
    

    /// Element of compound number
    ///
    /// "NUMBER"
    #[serde(rename="NUMBER")]
    NUMBER,
    

    /// Punctuation mark
    ///
    /// "P"
    #[serde(rename="P")]
    P,
    

    /// Parataxis relation
    ///
    /// "PARATAXIS"
    #[serde(rename="PARATAXIS")]
    PARATAXIS,
    

    /// Participial modifier
    ///
    /// "PARTMOD"
    #[serde(rename="PARTMOD")]
    PARTMOD,
    

    /// The complement of a preposition is a clause
    ///
    /// "PCOMP"
    #[serde(rename="PCOMP")]
    PCOMP,
    

    /// Object of a preposition
    ///
    /// "POBJ"
    #[serde(rename="POBJ")]
    POBJ,
    

    /// Possession modifier
    ///
    /// "POSS"
    #[serde(rename="POSS")]
    POSS,
    

    /// Postverbal negative particle
    ///
    /// "POSTNEG"
    #[serde(rename="POSTNEG")]
    POSTNEG,
    

    /// Predicate complement
    ///
    /// "PRECOMP"
    #[serde(rename="PRECOMP")]
    PRECOMP,
    

    /// Preconjunt
    ///
    /// "PRECONJ"
    #[serde(rename="PRECONJ")]
    PRECONJ,
    

    /// Predeterminer
    ///
    /// "PREDET"
    #[serde(rename="PREDET")]
    PREDET,
    

    /// Prefix
    ///
    /// "PREF"
    #[serde(rename="PREF")]
    PREF,
    

    /// Prepositional modifier
    ///
    /// "PREP"
    #[serde(rename="PREP")]
    PREP,
    

    /// The relationship between a verb and verbal morpheme
    ///
    /// "PRONL"
    #[serde(rename="PRONL")]
    PRONL,
    

    /// Particle
    ///
    /// "PRT"
    #[serde(rename="PRT")]
    PRT,
    

    /// Associative or possessive marker
    ///
    /// "PS"
    #[serde(rename="PS")]
    PS,
    

    /// Quantifier phrase modifier
    ///
    /// "QUANTMOD"
    #[serde(rename="QUANTMOD")]
    QUANTMOD,
    

    /// Relative clause modifier
    ///
    /// "RCMOD"
    #[serde(rename="RCMOD")]
    RCMOD,
    

    /// Complementizer in relative clause
    ///
    /// "RCMODREL"
    #[serde(rename="RCMODREL")]
    RCMODREL,
    

    /// Ellipsis without a preceding predicate
    ///
    /// "RDROP"
    #[serde(rename="RDROP")]
    RDROP,
    

    /// Referent
    ///
    /// "REF"
    #[serde(rename="REF")]
    REF,
    

    /// Remnant
    ///
    /// "REMNANT"
    #[serde(rename="REMNANT")]
    REMNANT,
    

    /// Reparandum
    ///
    /// "REPARANDUM"
    #[serde(rename="REPARANDUM")]
    REPARANDUM,
    

    /// Root
    ///
    /// "ROOT"
    #[serde(rename="ROOT")]
    ROOT,
    

    /// Suffix specifying a unit of number
    ///
    /// "SNUM"
    #[serde(rename="SNUM")]
    SNUM,
    

    /// Suffix
    ///
    /// "SUFF"
    #[serde(rename="SUFF")]
    SUFF,
    

    /// Temporal modifier
    ///
    /// "TMOD"
    #[serde(rename="TMOD")]
    TMOD,
    

    /// Topic marker
    ///
    /// "TOPIC"
    #[serde(rename="TOPIC")]
    TOPIC,
    

    /// Clause headed by an infinite form of the verb that modifies a noun
    ///
    /// "VMOD"
    #[serde(rename="VMOD")]
    VMOD,
    

    /// Vocative
    ///
    /// "VOCATIVE"
    #[serde(rename="VOCATIVE")]
    VOCATIVE,
    

    /// Open clausal complement
    ///
    /// "XCOMP"
    #[serde(rename="XCOMP")]
    XCOMP,
    

    /// Name suffix
    ///
    /// "SUFFIX"
    #[serde(rename="SUFFIX")]
    SUFFIX,
    

    /// Name title
    ///
    /// "TITLE"
    #[serde(rename="TITLE")]
    TITLE,
    

    /// Adverbial phrase modifier
    ///
    /// "ADVPHMOD"
    #[serde(rename="ADVPHMOD")]
    ADVPHMOD,
    

    /// Causative auxiliary
    ///
    /// "AUXCAUS"
    #[serde(rename="AUXCAUS")]
    AUXCAUS,
    

    /// Helper auxiliary
    ///
    /// "AUXVV"
    #[serde(rename="AUXVV")]
    AUXVV,
    

    /// Rentaishi (Prenominal modifier)
    ///
    /// "DTMOD"
    #[serde(rename="DTMOD")]
    DTMOD,
    

    /// Foreign words
    ///
    /// "FOREIGN"
    #[serde(rename="FOREIGN")]
    FOREIGN,
    

    /// Keyword
    ///
    /// "KW"
    #[serde(rename="KW")]
    KW,
    

    /// List for chains of comparable items
    ///
    /// "LIST"
    #[serde(rename="LIST")]
    LIST,
    

    /// Nominalized clause
    ///
    /// "NOMC"
    #[serde(rename="NOMC")]
    NOMC,
    

    /// Nominalized clausal subject
    ///
    /// "NOMCSUBJ"
    #[serde(rename="NOMCSUBJ")]
    NOMCSUBJ,
    

    /// Nominalized clausal passive
    ///
    /// "NOMCSUBJPASS"
    #[serde(rename="NOMCSUBJPASS")]
    NOMCSUBJPASS,
    

    /// Compound of numeric modifier
    ///
    /// "NUMC"
    #[serde(rename="NUMC")]
    NUMC,
    

    /// Copula
    ///
    /// "COP"
    #[serde(rename="COP")]
    COP,
    

    /// Dislocated relation (for fronted/topicalized elements)
    ///
    /// "DISLOCATED"
    #[serde(rename="DISLOCATED")]
    DISLOCATED,
    

    /// Aspect marker
    ///
    /// "ASP"
    #[serde(rename="ASP")]
    ASP,
    

    /// Genitive modifier
    ///
    /// "GMOD"
    #[serde(rename="GMOD")]
    GMOD,
    

    /// Genitive object
    ///
    /// "GOBJ"
    #[serde(rename="GOBJ")]
    GOBJ,
    

    /// Infinitival modifier
    ///
    /// "INFMOD"
    #[serde(rename="INFMOD")]
    INFMOD,
    

    /// Measure
    ///
    /// "MES"
    #[serde(rename="MES")]
    MES,
    

    /// Nominal complement of a noun
    ///
    /// "NCOMP"
    #[serde(rename="NCOMP")]
    NCOMP,
}

impl AsRef<str> for DependencyEdgeLabelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DependencyEdgeLabelEnum::UNKNOWN => "UNKNOWN",
            DependencyEdgeLabelEnum::ABBREV => "ABBREV",
            DependencyEdgeLabelEnum::ACOMP => "ACOMP",
            DependencyEdgeLabelEnum::ADVCL => "ADVCL",
            DependencyEdgeLabelEnum::ADVMOD => "ADVMOD",
            DependencyEdgeLabelEnum::AMOD => "AMOD",
            DependencyEdgeLabelEnum::APPOS => "APPOS",
            DependencyEdgeLabelEnum::ATTR => "ATTR",
            DependencyEdgeLabelEnum::AUX => "AUX",
            DependencyEdgeLabelEnum::AUXPASS => "AUXPASS",
            DependencyEdgeLabelEnum::CC => "CC",
            DependencyEdgeLabelEnum::CCOMP => "CCOMP",
            DependencyEdgeLabelEnum::CONJ => "CONJ",
            DependencyEdgeLabelEnum::CSUBJ => "CSUBJ",
            DependencyEdgeLabelEnum::CSUBJPASS => "CSUBJPASS",
            DependencyEdgeLabelEnum::DEP => "DEP",
            DependencyEdgeLabelEnum::DET => "DET",
            DependencyEdgeLabelEnum::DISCOURSE => "DISCOURSE",
            DependencyEdgeLabelEnum::DOBJ => "DOBJ",
            DependencyEdgeLabelEnum::EXPL => "EXPL",
            DependencyEdgeLabelEnum::GOESWITH => "GOESWITH",
            DependencyEdgeLabelEnum::IOBJ => "IOBJ",
            DependencyEdgeLabelEnum::MARK => "MARK",
            DependencyEdgeLabelEnum::MWE => "MWE",
            DependencyEdgeLabelEnum::MWV => "MWV",
            DependencyEdgeLabelEnum::NEG => "NEG",
            DependencyEdgeLabelEnum::NN => "NN",
            DependencyEdgeLabelEnum::NPADVMOD => "NPADVMOD",
            DependencyEdgeLabelEnum::NSUBJ => "NSUBJ",
            DependencyEdgeLabelEnum::NSUBJPASS => "NSUBJPASS",
            DependencyEdgeLabelEnum::NUM => "NUM",
            DependencyEdgeLabelEnum::NUMBER => "NUMBER",
            DependencyEdgeLabelEnum::P => "P",
            DependencyEdgeLabelEnum::PARATAXIS => "PARATAXIS",
            DependencyEdgeLabelEnum::PARTMOD => "PARTMOD",
            DependencyEdgeLabelEnum::PCOMP => "PCOMP",
            DependencyEdgeLabelEnum::POBJ => "POBJ",
            DependencyEdgeLabelEnum::POSS => "POSS",
            DependencyEdgeLabelEnum::POSTNEG => "POSTNEG",
            DependencyEdgeLabelEnum::PRECOMP => "PRECOMP",
            DependencyEdgeLabelEnum::PRECONJ => "PRECONJ",
            DependencyEdgeLabelEnum::PREDET => "PREDET",
            DependencyEdgeLabelEnum::PREF => "PREF",
            DependencyEdgeLabelEnum::PREP => "PREP",
            DependencyEdgeLabelEnum::PRONL => "PRONL",
            DependencyEdgeLabelEnum::PRT => "PRT",
            DependencyEdgeLabelEnum::PS => "PS",
            DependencyEdgeLabelEnum::QUANTMOD => "QUANTMOD",
            DependencyEdgeLabelEnum::RCMOD => "RCMOD",
            DependencyEdgeLabelEnum::RCMODREL => "RCMODREL",
            DependencyEdgeLabelEnum::RDROP => "RDROP",
            DependencyEdgeLabelEnum::REF => "REF",
            DependencyEdgeLabelEnum::REMNANT => "REMNANT",
            DependencyEdgeLabelEnum::REPARANDUM => "REPARANDUM",
            DependencyEdgeLabelEnum::ROOT => "ROOT",
            DependencyEdgeLabelEnum::SNUM => "SNUM",
            DependencyEdgeLabelEnum::SUFF => "SUFF",
            DependencyEdgeLabelEnum::TMOD => "TMOD",
            DependencyEdgeLabelEnum::TOPIC => "TOPIC",
            DependencyEdgeLabelEnum::VMOD => "VMOD",
            DependencyEdgeLabelEnum::VOCATIVE => "VOCATIVE",
            DependencyEdgeLabelEnum::XCOMP => "XCOMP",
            DependencyEdgeLabelEnum::SUFFIX => "SUFFIX",
            DependencyEdgeLabelEnum::TITLE => "TITLE",
            DependencyEdgeLabelEnum::ADVPHMOD => "ADVPHMOD",
            DependencyEdgeLabelEnum::AUXCAUS => "AUXCAUS",
            DependencyEdgeLabelEnum::AUXVV => "AUXVV",
            DependencyEdgeLabelEnum::DTMOD => "DTMOD",
            DependencyEdgeLabelEnum::FOREIGN => "FOREIGN",
            DependencyEdgeLabelEnum::KW => "KW",
            DependencyEdgeLabelEnum::LIST => "LIST",
            DependencyEdgeLabelEnum::NOMC => "NOMC",
            DependencyEdgeLabelEnum::NOMCSUBJ => "NOMCSUBJ",
            DependencyEdgeLabelEnum::NOMCSUBJPASS => "NOMCSUBJPASS",
            DependencyEdgeLabelEnum::NUMC => "NUMC",
            DependencyEdgeLabelEnum::COP => "COP",
            DependencyEdgeLabelEnum::DISLOCATED => "DISLOCATED",
            DependencyEdgeLabelEnum::ASP => "ASP",
            DependencyEdgeLabelEnum::GMOD => "GMOD",
            DependencyEdgeLabelEnum::GOBJ => "GOBJ",
            DependencyEdgeLabelEnum::INFMOD => "INFMOD",
            DependencyEdgeLabelEnum::MES => "MES",
            DependencyEdgeLabelEnum::NCOMP => "NCOMP",
        }
    }
}

impl std::convert::TryFrom< &str> for DependencyEdgeLabelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(DependencyEdgeLabelEnum::UNKNOWN),
           "ABBREV" => Ok(DependencyEdgeLabelEnum::ABBREV),
           "ACOMP" => Ok(DependencyEdgeLabelEnum::ACOMP),
           "ADVCL" => Ok(DependencyEdgeLabelEnum::ADVCL),
           "ADVMOD" => Ok(DependencyEdgeLabelEnum::ADVMOD),
           "AMOD" => Ok(DependencyEdgeLabelEnum::AMOD),
           "APPOS" => Ok(DependencyEdgeLabelEnum::APPOS),
           "ATTR" => Ok(DependencyEdgeLabelEnum::ATTR),
           "AUX" => Ok(DependencyEdgeLabelEnum::AUX),
           "AUXPASS" => Ok(DependencyEdgeLabelEnum::AUXPASS),
           "CC" => Ok(DependencyEdgeLabelEnum::CC),
           "CCOMP" => Ok(DependencyEdgeLabelEnum::CCOMP),
           "CONJ" => Ok(DependencyEdgeLabelEnum::CONJ),
           "CSUBJ" => Ok(DependencyEdgeLabelEnum::CSUBJ),
           "CSUBJPASS" => Ok(DependencyEdgeLabelEnum::CSUBJPASS),
           "DEP" => Ok(DependencyEdgeLabelEnum::DEP),
           "DET" => Ok(DependencyEdgeLabelEnum::DET),
           "DISCOURSE" => Ok(DependencyEdgeLabelEnum::DISCOURSE),
           "DOBJ" => Ok(DependencyEdgeLabelEnum::DOBJ),
           "EXPL" => Ok(DependencyEdgeLabelEnum::EXPL),
           "GOESWITH" => Ok(DependencyEdgeLabelEnum::GOESWITH),
           "IOBJ" => Ok(DependencyEdgeLabelEnum::IOBJ),
           "MARK" => Ok(DependencyEdgeLabelEnum::MARK),
           "MWE" => Ok(DependencyEdgeLabelEnum::MWE),
           "MWV" => Ok(DependencyEdgeLabelEnum::MWV),
           "NEG" => Ok(DependencyEdgeLabelEnum::NEG),
           "NN" => Ok(DependencyEdgeLabelEnum::NN),
           "NPADVMOD" => Ok(DependencyEdgeLabelEnum::NPADVMOD),
           "NSUBJ" => Ok(DependencyEdgeLabelEnum::NSUBJ),
           "NSUBJPASS" => Ok(DependencyEdgeLabelEnum::NSUBJPASS),
           "NUM" => Ok(DependencyEdgeLabelEnum::NUM),
           "NUMBER" => Ok(DependencyEdgeLabelEnum::NUMBER),
           "P" => Ok(DependencyEdgeLabelEnum::P),
           "PARATAXIS" => Ok(DependencyEdgeLabelEnum::PARATAXIS),
           "PARTMOD" => Ok(DependencyEdgeLabelEnum::PARTMOD),
           "PCOMP" => Ok(DependencyEdgeLabelEnum::PCOMP),
           "POBJ" => Ok(DependencyEdgeLabelEnum::POBJ),
           "POSS" => Ok(DependencyEdgeLabelEnum::POSS),
           "POSTNEG" => Ok(DependencyEdgeLabelEnum::POSTNEG),
           "PRECOMP" => Ok(DependencyEdgeLabelEnum::PRECOMP),
           "PRECONJ" => Ok(DependencyEdgeLabelEnum::PRECONJ),
           "PREDET" => Ok(DependencyEdgeLabelEnum::PREDET),
           "PREF" => Ok(DependencyEdgeLabelEnum::PREF),
           "PREP" => Ok(DependencyEdgeLabelEnum::PREP),
           "PRONL" => Ok(DependencyEdgeLabelEnum::PRONL),
           "PRT" => Ok(DependencyEdgeLabelEnum::PRT),
           "PS" => Ok(DependencyEdgeLabelEnum::PS),
           "QUANTMOD" => Ok(DependencyEdgeLabelEnum::QUANTMOD),
           "RCMOD" => Ok(DependencyEdgeLabelEnum::RCMOD),
           "RCMODREL" => Ok(DependencyEdgeLabelEnum::RCMODREL),
           "RDROP" => Ok(DependencyEdgeLabelEnum::RDROP),
           "REF" => Ok(DependencyEdgeLabelEnum::REF),
           "REMNANT" => Ok(DependencyEdgeLabelEnum::REMNANT),
           "REPARANDUM" => Ok(DependencyEdgeLabelEnum::REPARANDUM),
           "ROOT" => Ok(DependencyEdgeLabelEnum::ROOT),
           "SNUM" => Ok(DependencyEdgeLabelEnum::SNUM),
           "SUFF" => Ok(DependencyEdgeLabelEnum::SUFF),
           "TMOD" => Ok(DependencyEdgeLabelEnum::TMOD),
           "TOPIC" => Ok(DependencyEdgeLabelEnum::TOPIC),
           "VMOD" => Ok(DependencyEdgeLabelEnum::VMOD),
           "VOCATIVE" => Ok(DependencyEdgeLabelEnum::VOCATIVE),
           "XCOMP" => Ok(DependencyEdgeLabelEnum::XCOMP),
           "SUFFIX" => Ok(DependencyEdgeLabelEnum::SUFFIX),
           "TITLE" => Ok(DependencyEdgeLabelEnum::TITLE),
           "ADVPHMOD" => Ok(DependencyEdgeLabelEnum::ADVPHMOD),
           "AUXCAUS" => Ok(DependencyEdgeLabelEnum::AUXCAUS),
           "AUXVV" => Ok(DependencyEdgeLabelEnum::AUXVV),
           "DTMOD" => Ok(DependencyEdgeLabelEnum::DTMOD),
           "FOREIGN" => Ok(DependencyEdgeLabelEnum::FOREIGN),
           "KW" => Ok(DependencyEdgeLabelEnum::KW),
           "LIST" => Ok(DependencyEdgeLabelEnum::LIST),
           "NOMC" => Ok(DependencyEdgeLabelEnum::NOMC),
           "NOMCSUBJ" => Ok(DependencyEdgeLabelEnum::NOMCSUBJ),
           "NOMCSUBJPASS" => Ok(DependencyEdgeLabelEnum::NOMCSUBJPASS),
           "NUMC" => Ok(DependencyEdgeLabelEnum::NUMC),
           "COP" => Ok(DependencyEdgeLabelEnum::COP),
           "DISLOCATED" => Ok(DependencyEdgeLabelEnum::DISLOCATED),
           "ASP" => Ok(DependencyEdgeLabelEnum::ASP),
           "GMOD" => Ok(DependencyEdgeLabelEnum::GMOD),
           "GOBJ" => Ok(DependencyEdgeLabelEnum::GOBJ),
           "INFMOD" => Ok(DependencyEdgeLabelEnum::INFMOD),
           "MES" => Ok(DependencyEdgeLabelEnum::MES),
           "NCOMP" => Ok(DependencyEdgeLabelEnum::NCOMP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DependencyEdgeLabelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DocumentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. If the type is not set or is `TYPE_UNSPECIFIED`, returns an `INVALID_ARGUMENT` error.
pub enum DocumentTypeEnum {
    

    /// The content type is not specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Plain text
    ///
    /// "PLAIN_TEXT"
    #[serde(rename="PLAIN_TEXT")]
    PLAINTEXT,
    

    /// HTML
    ///
    /// "HTML"
    #[serde(rename="HTML")]
    HTML,
}

impl AsRef<str> for DocumentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DocumentTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            DocumentTypeEnum::PLAINTEXT => "PLAIN_TEXT",
            DocumentTypeEnum::HTML => "HTML",
        }
    }
}

impl std::convert::TryFrom< &str> for DocumentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(DocumentTypeEnum::TYPEUNSPECIFIED),
           "PLAIN_TEXT" => Ok(DocumentTypeEnum::PLAINTEXT),
           "HTML" => Ok(DocumentTypeEnum::HTML),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DocumentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EntityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The entity type.
pub enum EntityTypeEnum {
    

    /// Unknown
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Person
    ///
    /// "PERSON"
    #[serde(rename="PERSON")]
    PERSON,
    

    /// Location
    ///
    /// "LOCATION"
    #[serde(rename="LOCATION")]
    LOCATION,
    

    /// Organization
    ///
    /// "ORGANIZATION"
    #[serde(rename="ORGANIZATION")]
    ORGANIZATION,
    

    /// Event
    ///
    /// "EVENT"
    #[serde(rename="EVENT")]
    EVENT,
    

    /// Work of art
    ///
    /// "WORK_OF_ART"
    #[serde(rename="WORK_OF_ART")]
    WORKOFART,
    

    /// Consumer goods
    ///
    /// "CONSUMER_GOOD"
    #[serde(rename="CONSUMER_GOOD")]
    CONSUMERGOOD,
    

    /// Other types
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for EntityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EntityTypeEnum::UNKNOWN => "UNKNOWN",
            EntityTypeEnum::PERSON => "PERSON",
            EntityTypeEnum::LOCATION => "LOCATION",
            EntityTypeEnum::ORGANIZATION => "ORGANIZATION",
            EntityTypeEnum::EVENT => "EVENT",
            EntityTypeEnum::WORKOFART => "WORK_OF_ART",
            EntityTypeEnum::CONSUMERGOOD => "CONSUMER_GOOD",
            EntityTypeEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for EntityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(EntityTypeEnum::UNKNOWN),
           "PERSON" => Ok(EntityTypeEnum::PERSON),
           "LOCATION" => Ok(EntityTypeEnum::LOCATION),
           "ORGANIZATION" => Ok(EntityTypeEnum::ORGANIZATION),
           "EVENT" => Ok(EntityTypeEnum::EVENT),
           "WORK_OF_ART" => Ok(EntityTypeEnum::WORKOFART),
           "CONSUMER_GOOD" => Ok(EntityTypeEnum::CONSUMERGOOD),
           "OTHER" => Ok(EntityTypeEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EntityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EntityMentionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the entity mention.
pub enum EntityMentionTypeEnum {
    

    /// Unknown
    ///
    /// "TYPE_UNKNOWN"
    #[serde(rename="TYPE_UNKNOWN")]
    TYPEUNKNOWN,
    

    /// Proper name
    ///
    /// "PROPER"
    #[serde(rename="PROPER")]
    PROPER,
    

    /// Common noun (or noun compound)
    ///
    /// "COMMON"
    #[serde(rename="COMMON")]
    COMMON,
}

impl AsRef<str> for EntityMentionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EntityMentionTypeEnum::TYPEUNKNOWN => "TYPE_UNKNOWN",
            EntityMentionTypeEnum::PROPER => "PROPER",
            EntityMentionTypeEnum::COMMON => "COMMON",
        }
    }
}

impl std::convert::TryFrom< &str> for EntityMentionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNKNOWN" => Ok(EntityMentionTypeEnum::TYPEUNKNOWN),
           "PROPER" => Ok(EntityMentionTypeEnum::PROPER),
           "COMMON" => Ok(EntityMentionTypeEnum::COMMON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EntityMentionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechAspectEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The grammatical aspect.
pub enum PartOfSpeechAspectEnum {
    

    /// Aspect is not applicable in the analyzed language or is not predicted.
    ///
    /// "ASPECT_UNKNOWN"
    #[serde(rename="ASPECT_UNKNOWN")]
    ASPECTUNKNOWN,
    

    /// Perfective
    ///
    /// "PERFECTIVE"
    #[serde(rename="PERFECTIVE")]
    PERFECTIVE,
    

    /// Imperfective
    ///
    /// "IMPERFECTIVE"
    #[serde(rename="IMPERFECTIVE")]
    IMPERFECTIVE,
    

    /// Progressive
    ///
    /// "PROGRESSIVE"
    #[serde(rename="PROGRESSIVE")]
    PROGRESSIVE,
}

impl AsRef<str> for PartOfSpeechAspectEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechAspectEnum::ASPECTUNKNOWN => "ASPECT_UNKNOWN",
            PartOfSpeechAspectEnum::PERFECTIVE => "PERFECTIVE",
            PartOfSpeechAspectEnum::IMPERFECTIVE => "IMPERFECTIVE",
            PartOfSpeechAspectEnum::PROGRESSIVE => "PROGRESSIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechAspectEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASPECT_UNKNOWN" => Ok(PartOfSpeechAspectEnum::ASPECTUNKNOWN),
           "PERFECTIVE" => Ok(PartOfSpeechAspectEnum::PERFECTIVE),
           "IMPERFECTIVE" => Ok(PartOfSpeechAspectEnum::IMPERFECTIVE),
           "PROGRESSIVE" => Ok(PartOfSpeechAspectEnum::PROGRESSIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechAspectEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechCaseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The grammatical case.
pub enum PartOfSpeechCaseEnum {
    

    /// Case is not applicable in the analyzed language or is not predicted.
    ///
    /// "CASE_UNKNOWN"
    #[serde(rename="CASE_UNKNOWN")]
    CASEUNKNOWN,
    

    /// Accusative
    ///
    /// "ACCUSATIVE"
    #[serde(rename="ACCUSATIVE")]
    ACCUSATIVE,
    

    /// Adverbial
    ///
    /// "ADVERBIAL"
    #[serde(rename="ADVERBIAL")]
    ADVERBIAL,
    

    /// Complementive
    ///
    /// "COMPLEMENTIVE"
    #[serde(rename="COMPLEMENTIVE")]
    COMPLEMENTIVE,
    

    /// Dative
    ///
    /// "DATIVE"
    #[serde(rename="DATIVE")]
    DATIVE,
    

    /// Genitive
    ///
    /// "GENITIVE"
    #[serde(rename="GENITIVE")]
    GENITIVE,
    

    /// Instrumental
    ///
    /// "INSTRUMENTAL"
    #[serde(rename="INSTRUMENTAL")]
    INSTRUMENTAL,
    

    /// Locative
    ///
    /// "LOCATIVE"
    #[serde(rename="LOCATIVE")]
    LOCATIVE,
    

    /// Nominative
    ///
    /// "NOMINATIVE"
    #[serde(rename="NOMINATIVE")]
    NOMINATIVE,
    

    /// Oblique
    ///
    /// "OBLIQUE"
    #[serde(rename="OBLIQUE")]
    OBLIQUE,
    

    /// Partitive
    ///
    /// "PARTITIVE"
    #[serde(rename="PARTITIVE")]
    PARTITIVE,
    

    /// Prepositional
    ///
    /// "PREPOSITIONAL"
    #[serde(rename="PREPOSITIONAL")]
    PREPOSITIONAL,
    

    /// Reflexive
    ///
    /// "REFLEXIVE_CASE"
    #[serde(rename="REFLEXIVE_CASE")]
    REFLEXIVECASE,
    

    /// Relative
    ///
    /// "RELATIVE_CASE"
    #[serde(rename="RELATIVE_CASE")]
    RELATIVECASE,
    

    /// Vocative
    ///
    /// "VOCATIVE"
    #[serde(rename="VOCATIVE")]
    VOCATIVE,
}

impl AsRef<str> for PartOfSpeechCaseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechCaseEnum::CASEUNKNOWN => "CASE_UNKNOWN",
            PartOfSpeechCaseEnum::ACCUSATIVE => "ACCUSATIVE",
            PartOfSpeechCaseEnum::ADVERBIAL => "ADVERBIAL",
            PartOfSpeechCaseEnum::COMPLEMENTIVE => "COMPLEMENTIVE",
            PartOfSpeechCaseEnum::DATIVE => "DATIVE",
            PartOfSpeechCaseEnum::GENITIVE => "GENITIVE",
            PartOfSpeechCaseEnum::INSTRUMENTAL => "INSTRUMENTAL",
            PartOfSpeechCaseEnum::LOCATIVE => "LOCATIVE",
            PartOfSpeechCaseEnum::NOMINATIVE => "NOMINATIVE",
            PartOfSpeechCaseEnum::OBLIQUE => "OBLIQUE",
            PartOfSpeechCaseEnum::PARTITIVE => "PARTITIVE",
            PartOfSpeechCaseEnum::PREPOSITIONAL => "PREPOSITIONAL",
            PartOfSpeechCaseEnum::REFLEXIVECASE => "REFLEXIVE_CASE",
            PartOfSpeechCaseEnum::RELATIVECASE => "RELATIVE_CASE",
            PartOfSpeechCaseEnum::VOCATIVE => "VOCATIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechCaseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CASE_UNKNOWN" => Ok(PartOfSpeechCaseEnum::CASEUNKNOWN),
           "ACCUSATIVE" => Ok(PartOfSpeechCaseEnum::ACCUSATIVE),
           "ADVERBIAL" => Ok(PartOfSpeechCaseEnum::ADVERBIAL),
           "COMPLEMENTIVE" => Ok(PartOfSpeechCaseEnum::COMPLEMENTIVE),
           "DATIVE" => Ok(PartOfSpeechCaseEnum::DATIVE),
           "GENITIVE" => Ok(PartOfSpeechCaseEnum::GENITIVE),
           "INSTRUMENTAL" => Ok(PartOfSpeechCaseEnum::INSTRUMENTAL),
           "LOCATIVE" => Ok(PartOfSpeechCaseEnum::LOCATIVE),
           "NOMINATIVE" => Ok(PartOfSpeechCaseEnum::NOMINATIVE),
           "OBLIQUE" => Ok(PartOfSpeechCaseEnum::OBLIQUE),
           "PARTITIVE" => Ok(PartOfSpeechCaseEnum::PARTITIVE),
           "PREPOSITIONAL" => Ok(PartOfSpeechCaseEnum::PREPOSITIONAL),
           "REFLEXIVE_CASE" => Ok(PartOfSpeechCaseEnum::REFLEXIVECASE),
           "RELATIVE_CASE" => Ok(PartOfSpeechCaseEnum::RELATIVECASE),
           "VOCATIVE" => Ok(PartOfSpeechCaseEnum::VOCATIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechCaseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechFormEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The grammatical form.
pub enum PartOfSpeechFormEnum {
    

    /// Form is not applicable in the analyzed language or is not predicted.
    ///
    /// "FORM_UNKNOWN"
    #[serde(rename="FORM_UNKNOWN")]
    FORMUNKNOWN,
    

    /// Adnomial
    ///
    /// "ADNOMIAL"
    #[serde(rename="ADNOMIAL")]
    ADNOMIAL,
    

    /// Auxiliary
    ///
    /// "AUXILIARY"
    #[serde(rename="AUXILIARY")]
    AUXILIARY,
    

    /// Complementizer
    ///
    /// "COMPLEMENTIZER"
    #[serde(rename="COMPLEMENTIZER")]
    COMPLEMENTIZER,
    

    /// Final ending
    ///
    /// "FINAL_ENDING"
    #[serde(rename="FINAL_ENDING")]
    FINALENDING,
    

    /// Gerund
    ///
    /// "GERUND"
    #[serde(rename="GERUND")]
    GERUND,
    

    /// Realis
    ///
    /// "REALIS"
    #[serde(rename="REALIS")]
    REALIS,
    

    /// Irrealis
    ///
    /// "IRREALIS"
    #[serde(rename="IRREALIS")]
    IRREALIS,
    

    /// Short form
    ///
    /// "SHORT"
    #[serde(rename="SHORT")]
    SHORT,
    

    /// Long form
    ///
    /// "LONG"
    #[serde(rename="LONG")]
    LONG,
    

    /// Order form
    ///
    /// "ORDER"
    #[serde(rename="ORDER")]
    ORDER,
    

    /// Specific form
    ///
    /// "SPECIFIC"
    #[serde(rename="SPECIFIC")]
    SPECIFIC,
}

impl AsRef<str> for PartOfSpeechFormEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechFormEnum::FORMUNKNOWN => "FORM_UNKNOWN",
            PartOfSpeechFormEnum::ADNOMIAL => "ADNOMIAL",
            PartOfSpeechFormEnum::AUXILIARY => "AUXILIARY",
            PartOfSpeechFormEnum::COMPLEMENTIZER => "COMPLEMENTIZER",
            PartOfSpeechFormEnum::FINALENDING => "FINAL_ENDING",
            PartOfSpeechFormEnum::GERUND => "GERUND",
            PartOfSpeechFormEnum::REALIS => "REALIS",
            PartOfSpeechFormEnum::IRREALIS => "IRREALIS",
            PartOfSpeechFormEnum::SHORT => "SHORT",
            PartOfSpeechFormEnum::LONG => "LONG",
            PartOfSpeechFormEnum::ORDER => "ORDER",
            PartOfSpeechFormEnum::SPECIFIC => "SPECIFIC",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechFormEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORM_UNKNOWN" => Ok(PartOfSpeechFormEnum::FORMUNKNOWN),
           "ADNOMIAL" => Ok(PartOfSpeechFormEnum::ADNOMIAL),
           "AUXILIARY" => Ok(PartOfSpeechFormEnum::AUXILIARY),
           "COMPLEMENTIZER" => Ok(PartOfSpeechFormEnum::COMPLEMENTIZER),
           "FINAL_ENDING" => Ok(PartOfSpeechFormEnum::FINALENDING),
           "GERUND" => Ok(PartOfSpeechFormEnum::GERUND),
           "REALIS" => Ok(PartOfSpeechFormEnum::REALIS),
           "IRREALIS" => Ok(PartOfSpeechFormEnum::IRREALIS),
           "SHORT" => Ok(PartOfSpeechFormEnum::SHORT),
           "LONG" => Ok(PartOfSpeechFormEnum::LONG),
           "ORDER" => Ok(PartOfSpeechFormEnum::ORDER),
           "SPECIFIC" => Ok(PartOfSpeechFormEnum::SPECIFIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechFormEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechGenderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The grammatical gender.
pub enum PartOfSpeechGenderEnum {
    

    /// Gender is not applicable in the analyzed language or is not predicted.
    ///
    /// "GENDER_UNKNOWN"
    #[serde(rename="GENDER_UNKNOWN")]
    GENDERUNKNOWN,
    

    /// Feminine
    ///
    /// "FEMININE"
    #[serde(rename="FEMININE")]
    FEMININE,
    

    /// Masculine
    ///
    /// "MASCULINE"
    #[serde(rename="MASCULINE")]
    MASCULINE,
    

    /// Neuter
    ///
    /// "NEUTER"
    #[serde(rename="NEUTER")]
    NEUTER,
}

impl AsRef<str> for PartOfSpeechGenderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechGenderEnum::GENDERUNKNOWN => "GENDER_UNKNOWN",
            PartOfSpeechGenderEnum::FEMININE => "FEMININE",
            PartOfSpeechGenderEnum::MASCULINE => "MASCULINE",
            PartOfSpeechGenderEnum::NEUTER => "NEUTER",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechGenderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GENDER_UNKNOWN" => Ok(PartOfSpeechGenderEnum::GENDERUNKNOWN),
           "FEMININE" => Ok(PartOfSpeechGenderEnum::FEMININE),
           "MASCULINE" => Ok(PartOfSpeechGenderEnum::MASCULINE),
           "NEUTER" => Ok(PartOfSpeechGenderEnum::NEUTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechGenderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechMoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The grammatical mood.
pub enum PartOfSpeechMoodEnum {
    

    /// Mood is not applicable in the analyzed language or is not predicted.
    ///
    /// "MOOD_UNKNOWN"
    #[serde(rename="MOOD_UNKNOWN")]
    MOODUNKNOWN,
    

    /// Conditional
    ///
    /// "CONDITIONAL_MOOD"
    #[serde(rename="CONDITIONAL_MOOD")]
    CONDITIONALMOOD,
    

    /// Imperative
    ///
    /// "IMPERATIVE"
    #[serde(rename="IMPERATIVE")]
    IMPERATIVE,
    

    /// Indicative
    ///
    /// "INDICATIVE"
    #[serde(rename="INDICATIVE")]
    INDICATIVE,
    

    /// Interrogative
    ///
    /// "INTERROGATIVE"
    #[serde(rename="INTERROGATIVE")]
    INTERROGATIVE,
    

    /// Jussive
    ///
    /// "JUSSIVE"
    #[serde(rename="JUSSIVE")]
    JUSSIVE,
    

    /// Subjunctive
    ///
    /// "SUBJUNCTIVE"
    #[serde(rename="SUBJUNCTIVE")]
    SUBJUNCTIVE,
}

impl AsRef<str> for PartOfSpeechMoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechMoodEnum::MOODUNKNOWN => "MOOD_UNKNOWN",
            PartOfSpeechMoodEnum::CONDITIONALMOOD => "CONDITIONAL_MOOD",
            PartOfSpeechMoodEnum::IMPERATIVE => "IMPERATIVE",
            PartOfSpeechMoodEnum::INDICATIVE => "INDICATIVE",
            PartOfSpeechMoodEnum::INTERROGATIVE => "INTERROGATIVE",
            PartOfSpeechMoodEnum::JUSSIVE => "JUSSIVE",
            PartOfSpeechMoodEnum::SUBJUNCTIVE => "SUBJUNCTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechMoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MOOD_UNKNOWN" => Ok(PartOfSpeechMoodEnum::MOODUNKNOWN),
           "CONDITIONAL_MOOD" => Ok(PartOfSpeechMoodEnum::CONDITIONALMOOD),
           "IMPERATIVE" => Ok(PartOfSpeechMoodEnum::IMPERATIVE),
           "INDICATIVE" => Ok(PartOfSpeechMoodEnum::INDICATIVE),
           "INTERROGATIVE" => Ok(PartOfSpeechMoodEnum::INTERROGATIVE),
           "JUSSIVE" => Ok(PartOfSpeechMoodEnum::JUSSIVE),
           "SUBJUNCTIVE" => Ok(PartOfSpeechMoodEnum::SUBJUNCTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechMoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechNumberEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The grammatical number.
pub enum PartOfSpeechNumberEnum {
    

    /// Number is not applicable in the analyzed language or is not predicted.
    ///
    /// "NUMBER_UNKNOWN"
    #[serde(rename="NUMBER_UNKNOWN")]
    NUMBERUNKNOWN,
    

    /// Singular
    ///
    /// "SINGULAR"
    #[serde(rename="SINGULAR")]
    SINGULAR,
    

    /// Plural
    ///
    /// "PLURAL"
    #[serde(rename="PLURAL")]
    PLURAL,
    

    /// Dual
    ///
    /// "DUAL"
    #[serde(rename="DUAL")]
    DUAL,
}

impl AsRef<str> for PartOfSpeechNumberEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechNumberEnum::NUMBERUNKNOWN => "NUMBER_UNKNOWN",
            PartOfSpeechNumberEnum::SINGULAR => "SINGULAR",
            PartOfSpeechNumberEnum::PLURAL => "PLURAL",
            PartOfSpeechNumberEnum::DUAL => "DUAL",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechNumberEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NUMBER_UNKNOWN" => Ok(PartOfSpeechNumberEnum::NUMBERUNKNOWN),
           "SINGULAR" => Ok(PartOfSpeechNumberEnum::SINGULAR),
           "PLURAL" => Ok(PartOfSpeechNumberEnum::PLURAL),
           "DUAL" => Ok(PartOfSpeechNumberEnum::DUAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechNumberEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechPersonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The grammatical person.
pub enum PartOfSpeechPersonEnum {
    

    /// Person is not applicable in the analyzed language or is not predicted.
    ///
    /// "PERSON_UNKNOWN"
    #[serde(rename="PERSON_UNKNOWN")]
    PERSONUNKNOWN,
    

    /// First
    ///
    /// "FIRST"
    #[serde(rename="FIRST")]
    FIRST,
    

    /// Second
    ///
    /// "SECOND"
    #[serde(rename="SECOND")]
    SECOND,
    

    /// Third
    ///
    /// "THIRD"
    #[serde(rename="THIRD")]
    THIRD,
    

    /// Reflexive
    ///
    /// "REFLEXIVE_PERSON"
    #[serde(rename="REFLEXIVE_PERSON")]
    REFLEXIVEPERSON,
}

impl AsRef<str> for PartOfSpeechPersonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechPersonEnum::PERSONUNKNOWN => "PERSON_UNKNOWN",
            PartOfSpeechPersonEnum::FIRST => "FIRST",
            PartOfSpeechPersonEnum::SECOND => "SECOND",
            PartOfSpeechPersonEnum::THIRD => "THIRD",
            PartOfSpeechPersonEnum::REFLEXIVEPERSON => "REFLEXIVE_PERSON",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechPersonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERSON_UNKNOWN" => Ok(PartOfSpeechPersonEnum::PERSONUNKNOWN),
           "FIRST" => Ok(PartOfSpeechPersonEnum::FIRST),
           "SECOND" => Ok(PartOfSpeechPersonEnum::SECOND),
           "THIRD" => Ok(PartOfSpeechPersonEnum::THIRD),
           "REFLEXIVE_PERSON" => Ok(PartOfSpeechPersonEnum::REFLEXIVEPERSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechPersonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechProperEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The grammatical properness.
pub enum PartOfSpeechProperEnum {
    

    /// Proper is not applicable in the analyzed language or is not predicted.
    ///
    /// "PROPER_UNKNOWN"
    #[serde(rename="PROPER_UNKNOWN")]
    PROPERUNKNOWN,
    

    /// Proper
    ///
    /// "PROPER"
    #[serde(rename="PROPER")]
    PROPER,
    

    /// Not proper
    ///
    /// "NOT_PROPER"
    #[serde(rename="NOT_PROPER")]
    NOTPROPER,
}

impl AsRef<str> for PartOfSpeechProperEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechProperEnum::PROPERUNKNOWN => "PROPER_UNKNOWN",
            PartOfSpeechProperEnum::PROPER => "PROPER",
            PartOfSpeechProperEnum::NOTPROPER => "NOT_PROPER",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechProperEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROPER_UNKNOWN" => Ok(PartOfSpeechProperEnum::PROPERUNKNOWN),
           "PROPER" => Ok(PartOfSpeechProperEnum::PROPER),
           "NOT_PROPER" => Ok(PartOfSpeechProperEnum::NOTPROPER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechProperEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechReciprocityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The grammatical reciprocity.
pub enum PartOfSpeechReciprocityEnum {
    

    /// Reciprocity is not applicable in the analyzed language or is not predicted.
    ///
    /// "RECIPROCITY_UNKNOWN"
    #[serde(rename="RECIPROCITY_UNKNOWN")]
    RECIPROCITYUNKNOWN,
    

    /// Reciprocal
    ///
    /// "RECIPROCAL"
    #[serde(rename="RECIPROCAL")]
    RECIPROCAL,
    

    /// Non-reciprocal
    ///
    /// "NON_RECIPROCAL"
    #[serde(rename="NON_RECIPROCAL")]
    NONRECIPROCAL,
}

impl AsRef<str> for PartOfSpeechReciprocityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechReciprocityEnum::RECIPROCITYUNKNOWN => "RECIPROCITY_UNKNOWN",
            PartOfSpeechReciprocityEnum::RECIPROCAL => "RECIPROCAL",
            PartOfSpeechReciprocityEnum::NONRECIPROCAL => "NON_RECIPROCAL",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechReciprocityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECIPROCITY_UNKNOWN" => Ok(PartOfSpeechReciprocityEnum::RECIPROCITYUNKNOWN),
           "RECIPROCAL" => Ok(PartOfSpeechReciprocityEnum::RECIPROCAL),
           "NON_RECIPROCAL" => Ok(PartOfSpeechReciprocityEnum::NONRECIPROCAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechReciprocityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechTagEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The part of speech tag.
pub enum PartOfSpeechTagEnum {
    

    /// Unknown
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Adjective
    ///
    /// "ADJ"
    #[serde(rename="ADJ")]
    ADJ,
    

    /// Adposition (preposition and postposition)
    ///
    /// "ADP"
    #[serde(rename="ADP")]
    ADP,
    

    /// Adverb
    ///
    /// "ADV"
    #[serde(rename="ADV")]
    ADV,
    

    /// Conjunction
    ///
    /// "CONJ"
    #[serde(rename="CONJ")]
    CONJ,
    

    /// Determiner
    ///
    /// "DET"
    #[serde(rename="DET")]
    DET,
    

    /// Noun (common and proper)
    ///
    /// "NOUN"
    #[serde(rename="NOUN")]
    NOUN,
    

    /// Cardinal number
    ///
    /// "NUM"
    #[serde(rename="NUM")]
    NUM,
    

    /// Pronoun
    ///
    /// "PRON"
    #[serde(rename="PRON")]
    PRON,
    

    /// Particle or other function word
    ///
    /// "PRT"
    #[serde(rename="PRT")]
    PRT,
    

    /// Punctuation
    ///
    /// "PUNCT"
    #[serde(rename="PUNCT")]
    PUNCT,
    

    /// Verb (all tenses and modes)
    ///
    /// "VERB"
    #[serde(rename="VERB")]
    VERB,
    

    /// Other: foreign words, typos, abbreviations
    ///
    /// "X"
    #[serde(rename="X")]
    X,
    

    /// Affix
    ///
    /// "AFFIX"
    #[serde(rename="AFFIX")]
    AFFIX,
}

impl AsRef<str> for PartOfSpeechTagEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechTagEnum::UNKNOWN => "UNKNOWN",
            PartOfSpeechTagEnum::ADJ => "ADJ",
            PartOfSpeechTagEnum::ADP => "ADP",
            PartOfSpeechTagEnum::ADV => "ADV",
            PartOfSpeechTagEnum::CONJ => "CONJ",
            PartOfSpeechTagEnum::DET => "DET",
            PartOfSpeechTagEnum::NOUN => "NOUN",
            PartOfSpeechTagEnum::NUM => "NUM",
            PartOfSpeechTagEnum::PRON => "PRON",
            PartOfSpeechTagEnum::PRT => "PRT",
            PartOfSpeechTagEnum::PUNCT => "PUNCT",
            PartOfSpeechTagEnum::VERB => "VERB",
            PartOfSpeechTagEnum::X => "X",
            PartOfSpeechTagEnum::AFFIX => "AFFIX",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechTagEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(PartOfSpeechTagEnum::UNKNOWN),
           "ADJ" => Ok(PartOfSpeechTagEnum::ADJ),
           "ADP" => Ok(PartOfSpeechTagEnum::ADP),
           "ADV" => Ok(PartOfSpeechTagEnum::ADV),
           "CONJ" => Ok(PartOfSpeechTagEnum::CONJ),
           "DET" => Ok(PartOfSpeechTagEnum::DET),
           "NOUN" => Ok(PartOfSpeechTagEnum::NOUN),
           "NUM" => Ok(PartOfSpeechTagEnum::NUM),
           "PRON" => Ok(PartOfSpeechTagEnum::PRON),
           "PRT" => Ok(PartOfSpeechTagEnum::PRT),
           "PUNCT" => Ok(PartOfSpeechTagEnum::PUNCT),
           "VERB" => Ok(PartOfSpeechTagEnum::VERB),
           "X" => Ok(PartOfSpeechTagEnum::X),
           "AFFIX" => Ok(PartOfSpeechTagEnum::AFFIX),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechTagEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechTenseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The grammatical tense.
pub enum PartOfSpeechTenseEnum {
    

    /// Tense is not applicable in the analyzed language or is not predicted.
    ///
    /// "TENSE_UNKNOWN"
    #[serde(rename="TENSE_UNKNOWN")]
    TENSEUNKNOWN,
    

    /// Conditional
    ///
    /// "CONDITIONAL_TENSE"
    #[serde(rename="CONDITIONAL_TENSE")]
    CONDITIONALTENSE,
    

    /// Future
    ///
    /// "FUTURE"
    #[serde(rename="FUTURE")]
    FUTURE,
    

    /// Past
    ///
    /// "PAST"
    #[serde(rename="PAST")]
    PAST,
    

    /// Present
    ///
    /// "PRESENT"
    #[serde(rename="PRESENT")]
    PRESENT,
    

    /// Imperfect
    ///
    /// "IMPERFECT"
    #[serde(rename="IMPERFECT")]
    IMPERFECT,
    

    /// Pluperfect
    ///
    /// "PLUPERFECT"
    #[serde(rename="PLUPERFECT")]
    PLUPERFECT,
}

impl AsRef<str> for PartOfSpeechTenseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechTenseEnum::TENSEUNKNOWN => "TENSE_UNKNOWN",
            PartOfSpeechTenseEnum::CONDITIONALTENSE => "CONDITIONAL_TENSE",
            PartOfSpeechTenseEnum::FUTURE => "FUTURE",
            PartOfSpeechTenseEnum::PAST => "PAST",
            PartOfSpeechTenseEnum::PRESENT => "PRESENT",
            PartOfSpeechTenseEnum::IMPERFECT => "IMPERFECT",
            PartOfSpeechTenseEnum::PLUPERFECT => "PLUPERFECT",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechTenseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TENSE_UNKNOWN" => Ok(PartOfSpeechTenseEnum::TENSEUNKNOWN),
           "CONDITIONAL_TENSE" => Ok(PartOfSpeechTenseEnum::CONDITIONALTENSE),
           "FUTURE" => Ok(PartOfSpeechTenseEnum::FUTURE),
           "PAST" => Ok(PartOfSpeechTenseEnum::PAST),
           "PRESENT" => Ok(PartOfSpeechTenseEnum::PRESENT),
           "IMPERFECT" => Ok(PartOfSpeechTenseEnum::IMPERFECT),
           "PLUPERFECT" => Ok(PartOfSpeechTenseEnum::PLUPERFECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechTenseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartOfSpeechVoiceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The grammatical voice.
pub enum PartOfSpeechVoiceEnum {
    

    /// Voice is not applicable in the analyzed language or is not predicted.
    ///
    /// "VOICE_UNKNOWN"
    #[serde(rename="VOICE_UNKNOWN")]
    VOICEUNKNOWN,
    

    /// Active
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Causative
    ///
    /// "CAUSATIVE"
    #[serde(rename="CAUSATIVE")]
    CAUSATIVE,
    

    /// Passive
    ///
    /// "PASSIVE"
    #[serde(rename="PASSIVE")]
    PASSIVE,
}

impl AsRef<str> for PartOfSpeechVoiceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartOfSpeechVoiceEnum::VOICEUNKNOWN => "VOICE_UNKNOWN",
            PartOfSpeechVoiceEnum::ACTIVE => "ACTIVE",
            PartOfSpeechVoiceEnum::CAUSATIVE => "CAUSATIVE",
            PartOfSpeechVoiceEnum::PASSIVE => "PASSIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for PartOfSpeechVoiceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VOICE_UNKNOWN" => Ok(PartOfSpeechVoiceEnum::VOICEUNKNOWN),
           "ACTIVE" => Ok(PartOfSpeechVoiceEnum::ACTIVE),
           "CAUSATIVE" => Ok(PartOfSpeechVoiceEnum::CAUSATIVE),
           "PASSIVE" => Ok(PartOfSpeechVoiceEnum::PASSIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartOfSpeechVoiceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


