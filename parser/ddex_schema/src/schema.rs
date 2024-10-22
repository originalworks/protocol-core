use crate::validation::*;

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct AdditionalTitle {
    #[yaserde(rename = "TitleText", prefix = "ern")]
    pub title_text: String,
    #[yaserde(rename = "SubTitle", prefix = "ern")]
    pub sub_titles: Vec<DisplaySubTitle>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "TitleType",
        validation = "AvsAdditionalTitleTypeValidator"
    )]
    pub title_type: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[yaserde(attribute, rename = "IsInOriginalLanguage")]
    pub is_in_original_language: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct AdministratingRecordCompanyWithReference {
    #[yaserde(
        rename = "RecordCompanyPartyReference",
        prefix = "ern",
        validation = "RecordCompanyPartyReferenceValidator"
    )]
    pub record_company_party_reference: String,
    #[yaserde(rename = "Role", prefix = "ern")]
    pub role: AdministratingRecordCompanyRole,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct AudioDeliveryFile {
    #[yaserde(rename = "Type", prefix = "ern", validation = "TypeValidator")]
    pub kind: String,
    #[yaserde(rename = "ContainerFormat", prefix = "ern")]
    pub container_format: Option<ContainerFormat>,
    #[yaserde(rename = "AudioCodecType", prefix = "ern")]
    pub audio_codec_type: Option<AudioCodecType>,
    #[yaserde(rename = "BitRate", prefix = "ern")]
    pub bit_rate: Option<BitRate>,
    #[yaserde(rename = "OriginalBitRate", prefix = "ern")]
    pub original_bit_rate: Option<BitRate>,
    #[yaserde(rename = "NumberOfChannels", prefix = "ern")]
    pub number_of_channels: Option<String>,
    #[yaserde(rename = "NumberOfAudioObjects", prefix = "ern")]
    pub number_of_audio_objects: Option<i32>,
    #[yaserde(rename = "SamplingRate", prefix = "ern")]
    pub sampling_rate: Option<SamplingRate>,
    #[yaserde(rename = "OriginalSamplingRate", prefix = "ern")]
    pub original_sampling_rate: Option<SamplingRate>,
    #[yaserde(rename = "BitsPerSample", prefix = "ern")]
    pub bits_per_sample: Option<i32>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: Option<String>,
    #[yaserde(rename = "BitDepth", prefix = "ern")]
    pub bit_depth: Option<i32>,
    #[yaserde(rename = "File", prefix = "ern")]
    pub file: Option<File>,
    #[yaserde(rename = "Fingerprint", prefix = "ern")]
    pub fingerprints: Vec<Fingerprint>,
    #[yaserde(rename = "IsProvidedInDelivery", prefix = "ern")]
    pub is_provided_in_delivery: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct AvRating {
    #[yaserde(rename = "Rating", prefix = "ern")]
    pub rating: String,
    #[yaserde(rename = "Agency", prefix = "ern")]
    pub agency: RatingAgency,
    #[yaserde(rename = "Reason", prefix = "ern")]
    pub reason: Option<RatingReason>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CLineWithDefault {
    #[yaserde(rename = "Year", prefix = "ern")]
    pub year: Option<u16>,
    #[yaserde(rename = "CLineCompany", prefix = "ern")]
    pub c_line_company: Option<String>,
    #[yaserde(rename = "CLineText", prefix = "ern")]
    pub c_line_text: String,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Channel {
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    pub proprietary_ids: Vec<ProprietaryId>,
    #[yaserde(rename = "URL", prefix = "ern")]
    pub urls: Vec<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Chapter {
    #[yaserde(
        rename = "ChapterReference",
        prefix = "ern",
        validation = "ChapterReferenceValidator"
    )]
    pub chapter_reference: String,
    #[yaserde(rename = "ChapterId", prefix = "ern")]
    pub chapter_ids: Vec<ProprietaryId>,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "SequenceNumber", prefix = "ern")]
    pub sequence_number: Option<i32>,
    #[yaserde(rename = "Contributor", prefix = "ern")]
    pub contributors: Vec<Contributor>,
    #[yaserde(rename = "Character", prefix = "ern")]
    pub characters: Vec<Character>,
    #[yaserde(
        rename = "RepresentativeImageReference",
        prefix = "ern",
        validation = "RepresentativeImageReferenceValidator"
    )]
    pub representative_image_reference: Option<String>,
    #[yaserde(rename = "StartTime", prefix = "ern")]
    pub start_time: Option<String>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: Option<String>,
    #[yaserde(rename = "EndTime", prefix = "ern")]
    pub end_time: Option<String>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ChapterList {
    #[yaserde(rename = "Chapter", prefix = "ern")]
    pub chapters: Vec<Chapter>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Character {
    #[yaserde(
        rename = "CharacterPartyReference",
        prefix = "ern",
        validation = "CharacterPartyReferenceValidator"
    )]
    pub character_party_reference: String,
    #[yaserde(rename = "Performer", prefix = "ern")]
    pub performer: Option<Contributor>,
    #[yaserde(attribute, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ClipDetails {
    #[yaserde(rename = "ClipType", prefix = "ern")]
    pub clip_type: ClipType,
    #[yaserde(
        rename = "TopLeftCorner",
        prefix = "ern",
        validation = "TopLeftCornerValidator"
    )]
    pub top_left_corner: Option<String>,
    #[yaserde(
        rename = "BottomRightCorner",
        prefix = "ern",
        validation = "BottomRightCornerValidator"
    )]
    pub bottom_right_corner: Option<String>,
    #[yaserde(
        rename = "ExpressionType",
        prefix = "ern",
        validation = "AvsExpressionTypeValidator"
    )]
    pub expression_type: String,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ClipRelease {
    #[yaserde(
        rename = "ReleaseReference",
        prefix = "ern",
        validation = "ReleaseReferenceValidator"
    )]
    pub release_reference: String,
    #[yaserde(rename = "ReleaseId", prefix = "ern")]
    pub release_id: ReleaseId,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(
        rename = "ReleaseResourceReference",
        prefix = "ern",
        validation = "ReleaseResourceReferenceValidator"
    )]
    pub release_resource_reference: String,
    #[yaserde(rename = "ReleaseLabelReference", prefix = "ern")]
    pub release_label_references: Vec<ReleaseLabelReferenceWithParty>,
    #[yaserde(rename = "Genre", prefix = "ern")]
    pub genres: Vec<GenreWithTerritory>,
    #[yaserde(rename = "RelatedRelease", prefix = "ern")]
    pub related_releases: Vec<RelatedRelease>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CommercialModelType {
    #[yaserde(text, validation = "AvsCommercialModelTypeERNValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ConditionForRightsClaimPolicy {
    #[yaserde(rename = "Value", prefix = "ern")]
    pub value: String,
    #[yaserde(
        rename = "Unit",
        prefix = "ern",
        validation = "AvsUnitOfConditionValueValidator"
    )]
    pub unit: String,
    #[yaserde(
        rename = "ReferenceCreation",
        prefix = "ern",
        validation = "AvsReferenceCreationValidator"
    )]
    pub reference_creation: Option<String>,
    #[yaserde(
        rename = "RelationalRelator",
        prefix = "ern",
        validation = "AvsRelationalRelatorValidator"
    )]
    pub relational_relator: Option<String>,
    #[yaserde(
        rename = "MeasurementType",
        prefix = "ern",
        validation = "AvsMeasurementTypeValidator"
    )]
    pub measurement_type: Option<String>,
    #[yaserde(rename = "Segment", prefix = "ern")]
    pub segments: Vec<Segment>,
    #[yaserde(rename = "ServiceException", prefix = "ern")]
    pub service_exceptions: Vec<ServiceException>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Contributor {
    #[yaserde(
        rename = "ContributorPartyReference",
        prefix = "ern",
        validation = "ContributorPartyReferenceValidator"
    )]
    pub contributor_party_reference: String,
    #[yaserde(rename = "Role", prefix = "ern")]
    pub roles: Vec<ContributorRole>,
    #[yaserde(rename = "InstrumentType", prefix = "ern")]
    pub instrument_types: Vec<InstrumentType>,
    #[yaserde(rename = "HasMadeFeaturedContribution", prefix = "ern")]
    pub has_made_featured_contribution: Option<bool>,
    #[yaserde(rename = "HasMadeContractedContribution", prefix = "ern")]
    pub has_made_contracted_contribution: Option<bool>,
    #[yaserde(rename = "IsCredited", prefix = "ern")]
    pub is_credited: Option<IsCredited>,
    #[yaserde(rename = "DisplayCredits", prefix = "ern")]
    pub display_creditss: Vec<DisplayCredits>,
    #[yaserde(attribute, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CoreArea {
    #[yaserde(
        rename = "TopLeftCorner",
        prefix = "ern",
        validation = "TopLeftCornerValidator"
    )]
    pub top_left_corner: String,
    #[yaserde(
        rename = "BottomRightCorner",
        prefix = "ern",
        validation = "BottomRightCornerValidator"
    )]
    pub bottom_right_corner: String,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CourtesyLineWithDefault {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Cue {
    #[yaserde(rename = "CueUseType", prefix = "ern")]
    pub cue_use_type: Option<CueUseType>,
    #[yaserde(rename = "CueThemeType", prefix = "ern")]
    pub cue_theme_type: Option<CueThemeType>,
    #[yaserde(rename = "CueVocalType", prefix = "ern")]
    pub cue_vocal_type: Option<CueVocalType>,
    #[yaserde(rename = "CueVisualPerceptionType", prefix = "ern")]
    pub cue_visual_perception_type: Option<CueVisualPerceptionType>,
    #[yaserde(rename = "CueOrigin", prefix = "ern")]
    pub cue_origin: Option<CueOrigin>,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "Contributor", prefix = "ern")]
    pub contributors: Vec<Contributor>,
    #[yaserde(rename = "IsDance", prefix = "ern")]
    pub is_dance: Option<bool>,
    #[yaserde(rename = "HasMusicalContent", prefix = "ern")]
    pub has_musical_content: Option<bool>,
    #[yaserde(rename = "PLine", prefix = "ern")]
    pub p_lines: Vec<PLine>,
    #[yaserde(rename = "CLine", prefix = "ern")]
    pub c_lines: Vec<CLine>,
    #[yaserde(rename = "StartTime", prefix = "ern")]
    pub start_time: Option<String>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: Option<String>,
    #[yaserde(rename = "EndTime", prefix = "ern")]
    pub end_time: Option<String>,
    #[yaserde(rename = "ResourceId", prefix = "ern")]
    pub resource_id: Option<ResourceId>,
    #[yaserde(rename = "WorkId", prefix = "ern")]
    pub work_id: Option<MusicalWorkId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CueSheet {
    #[yaserde(rename = "CueSheetId", prefix = "ern")]
    pub cue_sheet_ids: Vec<ProprietaryId>,
    #[yaserde(
        rename = "CueSheetReference",
        prefix = "ern",
        validation = "CueSheetReferenceValidator"
    )]
    pub cue_sheet_reference: String,
    #[yaserde(rename = "CueSheetType", prefix = "ern")]
    pub cue_sheet_type: CueSheetType,
    #[yaserde(rename = "Cue", prefix = "ern")]
    pub cues: Vec<Cue>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CueSheetList {
    #[yaserde(rename = "CueSheet", prefix = "ern")]
    pub cue_sheets: Vec<CueSheet>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Deal {
    #[yaserde(rename = "DealReference", prefix = "ern")]
    pub deal_references: Vec<String>,
    #[yaserde(rename = "IsCommunicatedOutOfBand", prefix = "ern")]
    pub is_communicated_out_of_band: Option<bool>,
    #[yaserde(rename = "DealTerms", prefix = "ern")]
    pub deal_terms: Option<DealTerms>,
    #[yaserde(rename = "DealTechnicalResourceDetailsReferenceList", prefix = "ern")]
    pub deal_technical_resource_details_reference_list:
        Option<DealTechnicalResourceDetailsReferenceList>,
    #[yaserde(rename = "DistributionChannelPage", prefix = "ern")]
    pub distribution_channel_pages: Vec<DistributionChannelPage>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DealList {
    #[yaserde(rename = "ReleaseDeal", prefix = "ern")]
    pub release_deals: Vec<ReleaseDeal>,
    #[yaserde(rename = "ReleaseVisibility", prefix = "ern")]
    pub release_visibilitys: Vec<ReleaseVisibility>,
    #[yaserde(rename = "TrackReleaseVisibility", prefix = "ern")]
    pub track_release_visibilitys: Vec<TrackReleaseVisibility>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DealResourceReferenceList {
    #[yaserde(
        rename = "DealResourceReference",
        prefix = "ern",
        validation = "DealResourceReferenceValidator"
    )]
    pub deal_resource_references: Vec<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DealTechnicalResourceDetailsReferenceList {
    #[yaserde(
        rename = "DealTechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "DealTechnicalResourceDetailsReferenceValidator"
    )]
    pub deal_technical_resource_details_references: Vec<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DealTerms {
    #[yaserde(rename = "TerritoryCode", prefix = "ern")]
    pub territory_codes: Vec<CurrentTerritoryCode>,
    #[yaserde(rename = "ExcludedTerritoryCode", prefix = "ern")]
    pub excluded_territory_codes: Vec<CurrentTerritoryCode>,
    #[yaserde(rename = "DistributionChannel", prefix = "ern")]
    pub distribution_channels: Vec<Dsp>,
    #[yaserde(rename = "ExcludedDistributionChannel", prefix = "ern")]
    pub excluded_distribution_channels: Vec<Dsp>,
    #[yaserde(rename = "ValidityPeriod", prefix = "ern")]
    pub validity_periods: Vec<PeriodWithStartDate>,
    #[yaserde(rename = "CommercialModelType", prefix = "ern")]
    pub commercial_model_types: Vec<CommercialModelType>,
    #[yaserde(rename = "UseType", prefix = "ern")]
    pub use_types: Vec<DiscoverableUseType>,
    #[yaserde(rename = "UserInterfaceType", prefix = "ern")]
    pub user_interface_types: Vec<UserInterfaceType>,
    #[yaserde(rename = "CarrierType", prefix = "ern")]
    pub carrier_types: Vec<CarrierType>,
    #[yaserde(rename = "TechnicalInstantiation", prefix = "ern")]
    pub technical_instantiation: Option<DealTermsTechnicalInstantiation>,
    #[yaserde(rename = "NumberOfUsages", prefix = "ern")]
    pub number_of_usages: Option<i32>,
    #[yaserde(rename = "RightsClaimPolicy", prefix = "ern")]
    pub rights_claim_policys: Vec<RightsClaimPolicy>,
    #[yaserde(rename = "PriceInformation", prefix = "ern")]
    pub price_informations: Vec<PriceInformationWithType>,
    #[yaserde(rename = "IsPreOrderDeal", prefix = "ern")]
    pub is_pre_order_deal: Option<bool>,
    #[yaserde(rename = "InstantGratificationResourceList", prefix = "ern")]
    pub instant_gratification_resource_list: Option<DealResourceReferenceList>,
    #[yaserde(rename = "PhysicalReturns", prefix = "ern")]
    pub physical_returns: Option<PhysicalReturns>,
    #[yaserde(rename = "NumberOfProductsPerCarton", prefix = "ern")]
    pub number_of_products_per_carton: Option<i32>,
    #[yaserde(rename = "IsPromotional", prefix = "ern")]
    pub is_promotional: Option<bool>,
    #[yaserde(rename = "PromotionalCode", prefix = "ern")]
    pub promotional_code: Option<PromotionalCode>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DealTermsTechnicalInstantiation {
    #[yaserde(rename = "VideoDefinitionType", prefix = "ern")]
    pub video_definition_type: Option<VideoDefinitionType>,
    #[yaserde(
        rename = "CodingType",
        prefix = "ern",
        validation = "AvsCodingTypeValidator"
    )]
    pub coding_type: Option<String>,
    #[yaserde(rename = "BitRate", prefix = "ern")]
    pub bit_rate: Option<BitRate>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Deity {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DelegatedUsageRights {
    #[yaserde(rename = "UseType", prefix = "ern")]
    pub use_types: Vec<UseType>,
    #[yaserde(rename = "PeriodOfRightsDelegation", prefix = "ern")]
    pub period_of_rights_delegation: Option<Period>,
    #[yaserde(rename = "TerritoryOfRightsDelegation", prefix = "ern")]
    pub territory_of_rights_delegations: Vec<AllTerritoryCode>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DescriptionWithTerritory {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DetailedResourceContributor {
    #[yaserde(rename = "Role", prefix = "ern")]
    pub roles: Vec<ContributorRole>,
    #[yaserde(rename = "InstrumentType", prefix = "ern")]
    pub instrument_types: Vec<InstrumentType>,
    #[yaserde(rename = "HasMadeFeaturedContribution", prefix = "ern")]
    pub has_made_featured_contribution: Option<bool>,
    #[yaserde(rename = "HasMadeContractedContribution", prefix = "ern")]
    pub has_made_contracted_contribution: Option<bool>,
    #[yaserde(rename = "DisplayCredits", prefix = "ern")]
    pub display_creditss: Vec<DisplayCredits>,
    #[yaserde(attribute, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
    #[yaserde(rename = "PartyName", prefix = "ern")]
    pub parties_names: Vec<PartyName>,
    #[yaserde(rename = "PartyId", prefix = "ern")]
    pub parties_ids: Vec<DetailedPartyId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DiscoverableUseType {
    #[yaserde(text, validation = "AvsUseTypeERNValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "IsDiscoverable")]
    pub is_discoverable: Option<bool>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DisplayArtist {
    #[yaserde(
        rename = "ArtistPartyReference",
        prefix = "ern",
        validation = "ArtistPartyReferenceValidator"
    )]
    pub artist_party_reference: String,
    #[yaserde(rename = "DisplayArtistRole", prefix = "ern")]
    pub display_artist_role: DisplayArtistRole,
    #[yaserde(rename = "ArtisticRole", prefix = "ern")]
    pub artistic_roles: Vec<ContributorRole>,
    #[yaserde(rename = "TitleDisplayInformation", prefix = "ern")]
    pub title_display_informations: Vec<TitleDisplayInformation>,
    #[yaserde(attribute, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DisplayArtistNameWithDefault {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "IsInOriginalLanguage")]
    pub is_in_original_language: Option<bool>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DisplaySubTitle {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
    #[yaserde(attribute, rename = "IsDisplayedInTitle")]
    pub is_displayed_in_title: Option<bool>,
    #[yaserde(
        attribute,
        rename = "SubTitleType",
        validation = "AvsSubTitleTypeValidator"
    )]
    pub sub_title_type: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DisplayTitle {
    #[yaserde(rename = "TitleText", prefix = "ern")]
    pub title_text: String,
    #[yaserde(rename = "SubTitle", prefix = "ern")]
    pub sub_titles: Vec<DisplaySubTitle>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[yaserde(attribute, rename = "IsInOriginalLanguage")]
    pub is_in_original_language: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DisplayTitleText {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[yaserde(attribute, rename = "IsInOriginalLanguage")]
    pub is_in_original_language: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DistributionChannelPage {
    #[yaserde(rename = "PartyId", prefix = "ern")]
    pub party_ids: Vec<DetailedPartyId>,
    #[yaserde(rename = "PageName", prefix = "ern")]
    pub page_name: Option<Name>,
    #[yaserde(rename = "URL", prefix = "ern")]
    pub url: Option<String>,
    #[yaserde(rename = "UserName", prefix = "ern")]
    pub user_name: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EditionContributor {
    #[yaserde(
        rename = "ContributorPartyReference",
        prefix = "ern",
        validation = "ContributorPartyReferenceValidator"
    )]
    pub contributor_party_reference: String,
    #[yaserde(rename = "Role", prefix = "ern")]
    pub roles: Vec<ContributorRole>,
    #[yaserde(rename = "HasMadeFeaturedContribution", prefix = "ern")]
    pub has_made_featured_contribution: Option<bool>,
    #[yaserde(rename = "HasMadeContractedContribution", prefix = "ern")]
    pub has_made_contracted_contribution: Option<bool>,
    #[yaserde(rename = "IsCredited", prefix = "ern")]
    pub is_credited: Option<IsCredited>,
    #[yaserde(rename = "DisplayCredits", prefix = "ern")]
    pub display_creditss: Vec<DisplayCredits>,
    #[yaserde(attribute, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDateTimeWithoutFlags {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "IsApproximate")]
    pub is_approximate: Option<bool>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "LocationDescription")]
    pub location_description: Option<String>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::
            YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDateWithCurrentTerritory {
    #[yaserde(text, validation = "DdexIsoDateValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "IsApproximate")]
    pub is_approximate: Option<bool>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "LocationDescription")]
    pub location_description: Option<String>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDateWithCurrentTerritoryOptional {
    #[yaserde(rename = "StartDate", prefix = "ern")]
    pub start_date: Option<EventDateWithCurrentTerritory>,
    #[yaserde(rename = "EndDate", prefix = "ern")]
    pub end_date: Option<EventDateWithCurrentTerritory>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDateWithCurrentTerritoryRequired {
    #[yaserde(rename = "StartDate", prefix = "ern")]
    pub start_date: EventDateWithCurrentTerritory,
    #[yaserde(rename = "EndDate", prefix = "ern")]
    pub end_date: Option<EventDateWithCurrentTerritory>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDateWithDefault {
    #[yaserde(text, validation = "DdexIsoDateValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "IsApproximate")]
    pub is_approximate: Option<bool>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsAllTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDateWithoutFlags {
    #[yaserde(text, validation = "DdexIsoDateValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "IsApproximate")]
    pub is_approximate: Option<bool>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsAllTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "LocationDescription")]
    pub location_description: Option<String>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDateTimeWithoutFlagsOptional {
    #[yaserde(rename = "StartDateTime", prefix = "ern")]
    pub start_date_time: Option<EventDateTimeWithoutFlags>,
    #[yaserde(rename = "EndDateTime", prefix = "ern")]
    pub end_date_time: Option<EventDateTimeWithoutFlags>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDateTimeWithoutFlagsRequired {
    #[yaserde(rename = "StartDateTime", prefix = "ern")]
    pub start_date_time: EventDateTimeWithoutFlags,
    #[yaserde(rename = "EndDateTime", prefix = "ern")]
    pub end_date_time: Option<EventDateTimeWithoutFlags>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ExternalResourceLink {
    #[yaserde(rename = "URL", prefix = "ern")]
    pub urls: Vec<String>,
    #[yaserde(rename = "ValidityPeriod", prefix = "ern")]
    pub validity_period: Option<PeriodWithoutFlags>,
    #[yaserde(rename = "ExternalLink", prefix = "ern")]
    pub external_link: Option<String>,
    #[yaserde(rename = "ExternallyLinkedResourceType", prefix = "ern")]
    pub externally_linked_resource_types: Vec<ExternallyLinkedResourceType>,
    #[yaserde(rename = "FileFormat", prefix = "ern")]
    pub file_format: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Fingerprint {
    #[yaserde(rename = "Algorithm", prefix = "ern")]
    pub algorithm: FingerprintAlgorithmType,
    #[yaserde(rename = "Version", prefix = "ern")]
    pub version: Option<String>,
    #[yaserde(rename = "Parameter", prefix = "ern")]
    pub parameter: Option<String>,
    #[yaserde(rename = "File", prefix = "ern")]
    pub file: Option<File>,
    #[yaserde(
        rename = "DataType",
        prefix = "ern",
        validation = "AvsBinaryDataTypeValidator"
    )]
    pub data_type: Option<String>,
    #[yaserde(rename = "FingerprintValue", prefix = "ern")]
    pub fingerprint_value: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct HdrVideoDynamicMetadataType {
    #[yaserde(text, validation = "AvsHdrVideoDynamicMetadataTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Version")]
    pub version: Option<String>,
    #[yaserde(attribute, rename = "SdrDerivationPermitted")]
    pub sdr_derivation_permitted: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Image {
    #[yaserde(
        rename = "ResourceReference",
        prefix = "ern",
        validation = "ResourceReferenceValidator"
    )]
    pub resource_reference: String,
    #[yaserde(rename = "Type", prefix = "ern")]
    pub kind: ImageType,
    #[yaserde(rename = "ResourceId", prefix = "ern")]
    pub resource_ids: Vec<ResourceProprietaryId>,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "VersionType", prefix = "ern")]
    pub version_types: Vec<VersionType>,
    #[yaserde(rename = "DisplayArtistName", prefix = "ern")]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "Contributor", prefix = "ern")]
    pub contributors: Vec<Contributor>,
    #[yaserde(rename = "ResourceRightsController", prefix = "ern")]
    pub resource_rights_controllers: Vec<ResourceRightsController>,
    #[yaserde(rename = "WorkRightsController", prefix = "ern")]
    pub work_rights_controllers: Vec<WorkRightsController>,
    #[yaserde(rename = "CLine", prefix = "ern")]
    pub c_lines: Vec<CLineWithDefault>,
    #[yaserde(rename = "CourtesyLine", prefix = "ern")]
    pub courtesy_lines: Vec<CourtesyLineWithDefault>,
    #[yaserde(rename = "CreationDate", prefix = "ern")]
    pub creation_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "FirstPublicationDate", prefix = "ern")]
    pub first_publication_dates: Vec<FulfillmentDateWithTerritory>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "RelatedRelease", prefix = "ern")]
    pub related_releases: Vec<RelatedRelease>,
    #[yaserde(rename = "RelatedResource", prefix = "ern")]
    pub related_resources: Vec<RelatedResource>,
    #[yaserde(rename = "ContainsHiddenContent", prefix = "ern")]
    pub contains_hidden_content: Option<bool>,
    #[yaserde(rename = "Description", prefix = "ern")]
    pub descriptions: Vec<DescriptionWithTerritory>,
    #[yaserde(rename = "TechnicalDetails", prefix = "ern")]
    pub technical_detailss: Vec<TechnicalImageDetails>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "IsSupplemental")]
    pub is_supplemental: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct IsCredited {
    pub content: bool,
    #[yaserde(attribute, rename = "MayBeShared")]
    pub may_be_shared: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct LinkedReleaseResourceReference {
    #[yaserde(text, validation = "DdexLocalResourceAnchorReferenceValidator")]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "LinkDescription",
        validation = "AvsLinkDescriptionValidator"
    )]
    pub link_description: Option<String>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
    #[yaserde(attribute, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
    #[yaserde(attribute, rename = "IsMultiFile")]
    pub is_multi_file: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct LocationAndDateOfSession {
    #[yaserde(rename = "SessionType", prefix = "ern")]
    pub session_types: Vec<SessionType>,
    #[yaserde(rename = "Period", prefix = "ern")]
    pub period: Option<Period>,
    #[yaserde(rename = "Venue", prefix = "ern")]
    pub venues: Vec<Venue>,
    #[yaserde(rename = "Comment", prefix = "ern")]
    pub comment: Option<TextWithFormat>,
    #[yaserde(rename = "Contributor", prefix = "ern")]
    pub contributors: Vec<PartyWithRole>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Party {
    #[yaserde(
        rename = "PartyReference",
        prefix = "ern",
        validation = "PartyReferenceValidator"
    )]
    pub party_reference: String,
    #[yaserde(rename = "Affiliation", prefix = "ern")]
    pub affiliations: Vec<Affiliation>,
    #[yaserde(rename = "RelatedParty", prefix = "ern")]
    pub related_partys: Vec<RelatedParty>,
    #[yaserde(rename = "ArtistProfilePage", prefix = "ern")]
    pub artist_profile_pages: Vec<String>,
    #[yaserde(rename = "PartyName", prefix = "ern")]
    pub parties_names: Vec<PartyNameWithTerritory>,
    #[yaserde(rename = "PartyId", prefix = "ern")]
    pub parties_ids: Vec<DetailedPartyId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PartyList {
    #[yaserde(rename = "Party", prefix = "ern")]
    pub partys: Vec<Party>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PartyNameWithTerritory {
    #[yaserde(rename = "FullName", prefix = "ern")]
    pub full_name: Name,
    #[yaserde(rename = "FullNameAsciiTranscribed", prefix = "ern")]
    pub full_name_ascii_transcribed: Option<String>,
    #[yaserde(rename = "FullNameIndexed", prefix = "ern")]
    pub full_name_indexed: Option<Name>,
    #[yaserde(rename = "NamesBeforeKeyName", prefix = "ern")]
    pub names_before_key_name: Option<Name>,
    #[yaserde(rename = "KeyName", prefix = "ern")]
    pub key_name: Option<Name>,
    #[yaserde(rename = "NamesAfterKeyName", prefix = "ern")]
    pub names_after_key_name: Option<Name>,
    #[yaserde(rename = "AbbreviatedName", prefix = "ern")]
    pub abbreviated_name: Option<Name>,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "IsNickname")]
    pub is_nickname: Option<bool>,
    #[yaserde(attribute, rename = "IsStageName")]
    pub is_stage_name: Option<bool>,
    #[yaserde(attribute, rename = "IsLegalName")]
    pub is_legal_name: Option<bool>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[yaserde(attribute, rename = "IsInOriginalLanguage")]
    pub is_in_original_language: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PartyWithRole {
    #[yaserde(rename = "ISNI", prefix = "ern")]
    pub isni: Option<String>,
    #[yaserde(rename = "DPID", prefix = "ern", validation = "DPIDValidator")]
    pub dpid: Option<String>,
    #[yaserde(
        rename = "IpiNameNumber",
        prefix = "ern",
        validation = "IpiNameNumberValidator"
    )]
    pub ipi_name_number: Option<String>,
    #[yaserde(rename = "IPN", prefix = "ern")]
    pub ipn: Option<String>,
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    pub proprietary_ids: Vec<ProprietaryId>,
    #[yaserde(rename = "PartyName", prefix = "ern")]
    pub party_name: Option<PartyNameWithTerritory>,
    #[yaserde(rename = "Role", prefix = "ern")]
    pub role: Option<ResourceContributorRole>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PhysicalReturns {
    #[yaserde(rename = "PhysicalReturnsAllowed", prefix = "ern")]
    pub physical_returns_allowed: bool,
    #[yaserde(
        rename = "LatestDateForPhysicalReturns",
        prefix = "ern",
        validation = "DdexIsoDateValidator"
    )]
    pub latest_date_for_physical_returns: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PriceInformationWithType {
    #[yaserde(rename = "PriceCode", prefix = "ern")]
    pub price_code: Option<PriceType>,
    #[yaserde(rename = "WholesalePricePerUnit", prefix = "ern")]
    pub wholesale_price_per_unit: Option<Price>,
    #[yaserde(rename = "BulkOrderWholesalePricePerUnit", prefix = "ern")]
    pub bulk_order_wholesale_price_per_unit: Option<Price>,
    #[yaserde(rename = "SuggestedRetailPrice", prefix = "ern")]
    pub suggested_retail_price: Option<Price>,
    #[yaserde(
        attribute,
        rename = "PriceType",
        validation = "AvsPriceInformationTypeValidator"
    )]
    pub price_type: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PurgedRelease {
    #[yaserde(rename = "ReleaseId", prefix = "ern")]
    pub release_id: Option<ReleaseId>,
    #[yaserde(rename = "Title", prefix = "ern")]
    pub titles: Vec<Title>,
    #[yaserde(rename = "Contributor", prefix = "ern")]
    pub contributors: Vec<DetailedResourceContributor>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Raga {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct RecordingFormat {
    #[yaserde(text, validation = "AvsRecordingFormatValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct RelatedRelease {
    #[yaserde(rename = "ReleaseRelationshipType", prefix = "ern")]
    pub release_relationship_type: ReleaseRelationshipType,
    #[yaserde(rename = "ReleaseId", prefix = "ern")]
    pub release_id: ReleaseId,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "DisplayArtistName", prefix = "ern")]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "ReleaseLabelReference", prefix = "ern")]
    pub release_label_references: Vec<ReleaseLabelReference>,
    #[yaserde(rename = "ReleaseDate", prefix = "ern")]
    pub release_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "OriginalReleaseDate", prefix = "ern")]
    pub original_release_date: Option<EventDateWithoutFlags>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct RelatedResource {
    #[yaserde(
        rename = "ResourceRelationshipType",
        prefix = "ern",
        validation = "AvsResourceRelationshipTypeValidator"
    )]
    pub resource_relationship_type: String,
    #[yaserde(rename = "Timing", prefix = "ern")]
    pub timings: Vec<Timing>,
    #[yaserde(
        rename = "ResourceRelatedResourceReference",
        prefix = "ern",
        validation = "ResourceRelatedResourceReferenceValidator"
    )]
    pub resource_related_resource_reference: Option<String>,
    #[yaserde(rename = "ResourceId", prefix = "ern")]
    pub resource_id: Option<ResourceId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Release {
    #[yaserde(
        rename = "ReleaseReference",
        prefix = "ern",
        validation = "ReleaseReferenceValidator"
    )]
    pub release_reference: String,
    #[yaserde(rename = "ReleaseType", prefix = "ern")]
    pub release_types: Vec<ReleaseTypeForReleaseNotification>,
    #[yaserde(rename = "ReleaseId", prefix = "ern")]
    pub release_id: ReleaseId,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "VersionType", prefix = "ern")]
    pub version_types: Vec<VersionType>,
    #[yaserde(rename = "DisplayArtistName", prefix = "ern")]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "ReleaseLabelReference", prefix = "ern")]
    pub release_label_references: Vec<ReleaseLabelReferenceWithParty>,
    #[yaserde(rename = "AdministratingRecordCompany", prefix = "ern")]
    pub administrating_record_companys: Vec<AdministratingRecordCompanyWithReference>,
    #[yaserde(rename = "PLine", prefix = "ern")]
    pub p_lines: Vec<PLineWithDefault>,
    #[yaserde(rename = "CLine", prefix = "ern")]
    pub c_lines: Vec<CLineWithDefault>,
    #[yaserde(rename = "CourtesyLine", prefix = "ern")]
    pub courtesy_lines: Vec<CourtesyLineWithDefault>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: Option<String>,
    #[yaserde(rename = "Genre", prefix = "ern")]
    pub genres: Vec<GenreWithTerritory>,
    #[yaserde(rename = "ReleaseDate", prefix = "ern")]
    pub release_dates: Vec<EventDateWithDefault>,
    #[yaserde(rename = "OriginalReleaseDate", prefix = "ern")]
    pub original_release_dates: Vec<EventDateWithDefault>,
    #[yaserde(
        rename = "ReleaseVisibilityReference",
        prefix = "ern",
        validation = "ReleaseVisibilityReferenceValidator"
    )]
    pub release_visibility_references: Vec<String>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "AvRating", prefix = "ern")]
    pub av_ratings: Vec<AvRating>,
    #[yaserde(rename = "RelatedRelease", prefix = "ern")]
    pub related_releases: Vec<RelatedRelease>,
    #[yaserde(rename = "RelatedResource", prefix = "ern")]
    pub related_resources: Vec<RelatedResource>,
    #[yaserde(rename = "ResourceGroup", prefix = "ern")]
    pub resource_group: ResourceGroup,
    #[yaserde(rename = "ExternalResourceLink", prefix = "ern")]
    pub external_resource_links: Vec<ExternalResourceLink>,
    #[yaserde(rename = "TargetURL", prefix = "ern")]
    pub target_url: Option<String>,
    #[yaserde(rename = "Keywords", prefix = "ern")]
    pub keywordss: Vec<KeywordsWithTerritory>,
    #[yaserde(rename = "Synopsis", prefix = "ern")]
    pub synopsiss: Vec<SynopsisWithTerritory>,
    #[yaserde(rename = "Raga", prefix = "ern")]
    pub ragas: Vec<Raga>,
    #[yaserde(rename = "Tala", prefix = "ern")]
    pub talas: Vec<Tala>,
    #[yaserde(rename = "Deity", prefix = "ern")]
    pub deitys: Vec<Deity>,
    #[yaserde(rename = "HiResMusicDescription", prefix = "ern")]
    pub hi_res_music_description: Option<String>,
    #[yaserde(rename = "IsSoundtrack", prefix = "ern")]
    pub is_soundtrack: Option<bool>,
    #[yaserde(rename = "IsHiResMusic", prefix = "ern")]
    pub is_hi_res_music: Option<bool>,
    #[yaserde(rename = "MarketingComment", prefix = "ern")]
    pub marketing_comments: Vec<MarketingComment>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(rename = "IsSingleArtistCompilation", prefix = "ern")]
    pub is_single_artist_compilation: Option<bool>,
    #[yaserde(rename = "IsMultiArtistCompilation", prefix = "ern")]
    pub is_multi_artist_compilation: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseAdmin {
    #[yaserde(rename = "ReleaseAdminId", prefix = "ern")]
    pub release_admin_id: String,
    #[yaserde(rename = "PersonnelDescription", prefix = "ern")]
    pub personnel_description: Option<String>,
    #[yaserde(rename = "SystemDescription", prefix = "ern")]
    pub system_descriptions: Vec<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseDeal {
    #[yaserde(
        rename = "DealReleaseReference",
        prefix = "ern",
        validation = "DealReleaseReferenceValidator"
    )]
    pub deal_release_references: Vec<String>,
    #[yaserde(rename = "Deal", prefix = "ern")]
    pub deals: Vec<Deal>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseId {
    #[yaserde(rename = "GRid", prefix = "ern")]
    pub g_rid: Option<String>,
    #[yaserde(rename = "ICPN", prefix = "ern")]
    pub icpn: Option<String>,
    #[yaserde(rename = "CatalogNumber", prefix = "ern")]
    pub catalog_number: Option<CatalogNumber>,
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    pub proprietary_ids: Vec<ProprietaryId>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseLabelReference {
    #[yaserde(text, validation = "DdexLocalPartyAnchorReferenceValidator")]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[yaserde(attribute, rename = "LabelType", validation = "AvsLabelTypeValidator")]
    pub label_type: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseLabelReferenceWithParty {
    #[yaserde(text, validation = "DdexLocalPartyAnchorReferenceValidator")]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[yaserde(attribute, rename = "LabelType", validation = "AvsLabelTypeValidator")]
    pub label_type: Option<String>,
    #[yaserde(attribute, rename = "AccessControlParty")]
    pub access_control_party: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseList {
    #[yaserde(rename = "Release", prefix = "ern")]
    pub release: Option<Release>,
    #[yaserde(rename = "TrackRelease", prefix = "ern")]
    pub track_releases: Vec<TrackRelease>,
    #[yaserde(rename = "ClipRelease", prefix = "ern")]
    pub clip_releases: Vec<ClipRelease>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseVisibility {
    #[yaserde(
        rename = "VisibilityReference",
        prefix = "ern",
        validation = "VisibilityReferenceValidator"
    )]
    pub visibility_reference: String,
    #[yaserde(rename = "ReleaseDisplayStartDateTime", prefix = "ern")]
    pub release_display_start_date_time: Option<String>,
    #[yaserde(rename = "CoverArtPreviewStartDateTime", prefix = "ern")]
    pub cover_art_preview_start_date_time: Option<String>,
    #[yaserde(rename = "FullTrackListingPreviewStartDateTime", prefix = "ern")]
    pub full_track_listing_preview_start_date_time: Option<String>,
    #[yaserde(rename = "ClipPreviewStartDateTime", prefix = "ern")]
    pub clip_preview_start_date_time: Option<String>,
    #[yaserde(attribute, rename = "DoNotDisplayDates")]
    pub do_not_display_dates: Option<bool>,
    #[yaserde(
        rename = "TerritoryCode",
        prefix = "ern",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub territory_codes: Vec<String>,
    #[yaserde(
        rename = "ExcludedTerritoryCode",
        prefix = "ern",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub excluded_territory_codes: Vec<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceGroup {
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "SequenceNumber", prefix = "ern")]
    pub sequence_number: Option<i32>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "CarrierType", prefix = "ern")]
    pub carrier_types: Vec<CarrierType>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: Option<String>,
    // TODO
    #[yaserde(rename = "ResourceGroup", prefix = "ern")]
    pub resource_groups: Vec<ResourceSubGroup>,
    #[yaserde(rename = "ResourceGroupContentItem", prefix = "ern")]
    pub resource_group_content_items: Vec<ResourceGroupContentItem>,
    #[yaserde(rename = "LinkedReleaseResourceReference", prefix = "ern")]
    pub linked_release_resource_references: Vec<LinkedReleaseResourceReference>,
    #[yaserde(rename = "NoDisplaySequence", prefix = "ern")]
    pub no_display_sequence: Option<bool>,
    #[yaserde(rename = "DisplaySequence", prefix = "ern")]
    pub display_sequence: Option<String>,
    #[yaserde(
        rename = "ResourceGroupReleaseReference",
        prefix = "ern",
        validation = "ResourceGroupReleaseReferenceValidator"
    )]
    pub resource_group_release_reference: Option<String>,
    #[yaserde(rename = "ReleaseId", prefix = "ern")]
    pub release_id: Option<ReleaseId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceGroupContentItem {
    #[yaserde(rename = "SequenceNumber", prefix = "ern")]
    pub sequence_number: Option<i32>,
    #[yaserde(
        rename = "ReleaseResourceReference",
        prefix = "ern",
        validation = "ReleaseResourceReferenceValidator"
    )]
    pub release_resource_reference: String,
    #[yaserde(rename = "LinkedReleaseResourceReference", prefix = "ern")]
    pub linked_release_resource_references: Vec<LinkedReleaseResourceReference>,
    #[yaserde(rename = "IsBonusResource", prefix = "ern")]
    pub is_bonus_resource: Option<bool>,
    #[yaserde(rename = "IsInstantGratificationResource", prefix = "ern")]
    pub is_instant_gratification_resource: Option<bool>,
    #[yaserde(rename = "IsPreOrderIncentiveResource", prefix = "ern")]
    pub is_pre_order_incentive_resource: Option<bool>,
    #[yaserde(rename = "NoDisplaySequence", prefix = "ern")]
    pub no_display_sequence: Option<bool>,
    #[yaserde(rename = "DisplaySequence", prefix = "ern")]
    pub display_sequence: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceList {
    #[yaserde(rename = "SoundRecording", prefix = "ern")]
    pub sound_recordings: Vec<SoundRecording>,
    #[yaserde(rename = "Video", prefix = "ern")]
    pub videos: Vec<Video>,
    #[yaserde(rename = "Image", prefix = "ern")]
    pub images: Vec<Image>,
    #[yaserde(rename = "Text", prefix = "ern")]
    pub texts: Vec<Text>,
    #[yaserde(rename = "SheetMusic", prefix = "ern")]
    pub sheet_musics: Vec<SheetMusic>,
    #[yaserde(rename = "Software", prefix = "ern")]
    pub softwares: Vec<Software>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceRightsController {
    #[yaserde(
        rename = "RightsControllerPartyReference",
        prefix = "ern",
        validation = "RightsControllerPartyReferenceValidator"
    )]
    pub rights_controller_party_reference: String,
    #[yaserde(
        rename = "RightsControlType",
        prefix = "ern",
        validation = "AvsRightsControllerRoleValidator"
    )]
    pub rights_control_types: Vec<String>,
    #[yaserde(rename = "DelegatedUsageRights", prefix = "ern")]
    pub delegated_usage_rightss: Vec<DelegatedUsageRights>,
    #[yaserde(attribute, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
    #[yaserde(rename = "RightShareUnknown", prefix = "ern")]
    pub right_share_unknown: Option<bool>,
    #[yaserde(rename = "RightSharePercentage", prefix = "ern")]
    pub right_share_percentage: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceSubGroup {
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "SequenceNumber", prefix = "ern")]
    pub sequence_number: Option<i32>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "CarrierType", prefix = "ern")]
    pub carrier_types: Vec<CarrierType>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: Option<String>,
    #[yaserde(rename = "ResourceGroup", prefix = "ern")]
    pub resource_groups: Vec<ResourceSubGroup>,
    #[yaserde(rename = "ResourceGroupContentItem", prefix = "ern")]
    pub resource_group_content_items: Vec<ResourceGroupContentItem>,
    #[yaserde(rename = "LinkedReleaseResourceReference", prefix = "ern")]
    pub linked_release_resource_references: Vec<LinkedReleaseResourceReference>,
    #[yaserde(
        attribute,
        rename = "ResourceGroupType",
        validation = "AvsResourceGroupTypeValidator"
    )]
    pub resource_group_type: String,
    #[yaserde(rename = "NoDisplaySequence", prefix = "ern")]
    pub no_display_sequence: Option<bool>,
    #[yaserde(rename = "DisplaySequence", prefix = "ern")]
    pub display_sequence: Option<String>,
    #[yaserde(
        rename = "ResourceGroupReleaseReference",
        prefix = "ern",
        validation = "ResourceGroupReleaseReferenceValidator"
    )]
    pub resource_group_release_reference: Option<String>,
    #[yaserde(rename = "ReleaseId", prefix = "ern")]
    pub release_id: Option<ReleaseId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct RightsClaimPolicy {
    #[yaserde(rename = "Condition", prefix = "ern")]
    pub conditions: Vec<ConditionForRightsClaimPolicy>,
    #[yaserde(
        rename = "RightsClaimPolicyType",
        prefix = "ern",
        validation = "AvsRightsClaimPolicyTypeValidator"
    )]
    pub rights_claim_policy_type: String,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Segment {
    #[yaserde(rename = "StartTime", prefix = "ern")]
    pub start_time: String,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: Option<String>,
    #[yaserde(rename = "EndTime", prefix = "ern")]
    pub end_time: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ServiceException {
    #[yaserde(rename = "TradingName", prefix = "ern")]
    pub trading_name: Option<Name>,
    #[yaserde(rename = "URL", prefix = "ern")]
    pub urls: Vec<String>,
    #[yaserde(rename = "Channel", prefix = "ern")]
    pub channels: Vec<Channel>,
    #[yaserde(rename = "PartyName", prefix = "ern")]
    pub parties_names: Vec<PartyName>,
    #[yaserde(rename = "PartyId", prefix = "ern")]
    pub parties_ids: Vec<DetailedPartyId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SheetMusic {
    #[yaserde(
        rename = "ResourceReference",
        prefix = "ern",
        validation = "ResourceReferenceValidator"
    )]
    pub resource_reference: String,
    #[yaserde(rename = "Type", prefix = "ern")]
    pub kind: SheetMusicType,
    #[yaserde(rename = "ResourceId", prefix = "ern")]
    pub resource_ids: Vec<SheetMusicId>,
    #[yaserde(rename = "WorkId", prefix = "ern")]
    pub work_ids: Vec<MusicalWorkId>,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "VersionType", prefix = "ern")]
    pub version_types: Vec<VersionType>,
    #[yaserde(rename = "DisplayArtistName", prefix = "ern")]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "Contributor", prefix = "ern")]
    pub contributors: Vec<Contributor>,
    #[yaserde(rename = "ResourceRightsController", prefix = "ern")]
    pub resource_rights_controllers: Vec<ResourceRightsController>,
    #[yaserde(rename = "WorkRightsController", prefix = "ern")]
    pub work_rights_controllers: Vec<WorkRightsController>,
    #[yaserde(rename = "CLine", prefix = "ern")]
    pub c_lines: Vec<CLineWithDefault>,
    #[yaserde(rename = "CourtesyLine", prefix = "ern")]
    pub courtesy_lines: Vec<CourtesyLineWithDefault>,
    #[yaserde(rename = "CreationDate", prefix = "ern")]
    pub creation_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "FirstPublicationDate", prefix = "ern")]
    pub first_publication_dates: Vec<FulfillmentDateWithTerritory>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "RelatedRelease", prefix = "ern")]
    pub related_releases: Vec<RelatedRelease>,
    #[yaserde(rename = "RelatedResource", prefix = "ern")]
    pub related_resources: Vec<RelatedResource>,
    #[yaserde(rename = "ContainsHiddenContent", prefix = "ern")]
    pub contains_hidden_content: Option<bool>,
    #[yaserde(
        rename = "LanguageOfLyrics",
        prefix = "ern",
        validation = "LanguageOfLyricsValidator"
    )]
    pub language_of_lyrics: Option<String>,
    #[yaserde(rename = "ResourceContainedResourceReferenceList", prefix = "ern")]
    pub resource_contained_resource_reference_list: Option<ResourceContainedResourceReferenceList>,
    #[yaserde(rename = "TechnicalDetails", prefix = "ern")]
    pub technical_detailss: Vec<TechnicalSheetMusicDetails>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "IsSupplemental")]
    pub is_supplemental: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Software {
    #[yaserde(
        rename = "ResourceReference",
        prefix = "ern",
        validation = "ResourceReferenceValidator"
    )]
    pub resource_reference: String,
    #[yaserde(rename = "Type", prefix = "ern")]
    pub kind: SoftwareType,
    #[yaserde(rename = "ResourceId", prefix = "ern")]
    pub resource_ids: Vec<ResourceProprietaryId>,
    #[yaserde(rename = "WorkId", prefix = "ern")]
    pub work_ids: Vec<MusicalWorkId>,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "VersionType", prefix = "ern")]
    pub version_types: Vec<VersionType>,
    #[yaserde(rename = "DisplayArtistName", prefix = "ern")]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "Contributor", prefix = "ern")]
    pub contributors: Vec<Contributor>,
    #[yaserde(rename = "ResourceRightsController", prefix = "ern")]
    pub resource_rights_controllers: Vec<ResourceRightsController>,
    #[yaserde(rename = "WorkRightsController", prefix = "ern")]
    pub work_rights_controllers: Vec<WorkRightsController>,
    #[yaserde(rename = "PLine", prefix = "ern")]
    pub p_lines: Vec<PLineWithDefault>,
    #[yaserde(rename = "CLine", prefix = "ern")]
    pub c_lines: Vec<CLineWithDefault>,
    #[yaserde(rename = "CourtesyLine", prefix = "ern")]
    pub courtesy_lines: Vec<CourtesyLineWithDefault>,
    #[yaserde(rename = "CreationDate", prefix = "ern")]
    pub creation_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "FirstPublicationDate", prefix = "ern")]
    pub first_publication_dates: Vec<FulfillmentDateWithTerritory>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "RelatedRelease", prefix = "ern")]
    pub related_releases: Vec<RelatedRelease>,
    #[yaserde(rename = "RelatedResource", prefix = "ern")]
    pub related_resources: Vec<RelatedResource>,
    #[yaserde(rename = "ContainsHiddenContent", prefix = "ern")]
    pub contains_hidden_content: Option<bool>,
    #[yaserde(rename = "ResourceContainedResourceReferenceList", prefix = "ern")]
    pub resource_contained_resource_reference_list: Option<ResourceContainedResourceReferenceList>,
    #[yaserde(rename = "TechnicalDetails", prefix = "ern")]
    pub technical_detailss: Vec<TechnicalSoftwareDetails>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "IsSupplemental")]
    pub is_supplemental: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SoundRecording {
    #[yaserde(
        rename = "ResourceReference",
        prefix = "ern",
        validation = "ResourceReferenceValidator"
    )]
    pub resource_reference: String,
    #[yaserde(rename = "Type", prefix = "ern")]
    pub kind: SoundRecordingType,
    #[yaserde(rename = "SoundRecordingEdition", prefix = "ern")]
    pub sound_recording_editions: Vec<SoundRecordingEdition>,
    #[yaserde(rename = "RecordingFormat", prefix = "ern")]
    pub recording_formats: Vec<RecordingFormat>,
    #[yaserde(rename = "WorkId", prefix = "ern")]
    pub work_ids: Vec<MusicalWorkId>,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "VersionType", prefix = "ern")]
    pub version_types: Vec<VersionType>,
    #[yaserde(rename = "DisplayArtistName", prefix = "ern")]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "Contributor", prefix = "ern")]
    pub contributors: Vec<Contributor>,
    #[yaserde(rename = "Character", prefix = "ern")]
    pub characters: Vec<Character>,
    #[yaserde(rename = "ResourceRightsController", prefix = "ern")]
    pub resource_rights_controllers: Vec<ResourceRightsController>,
    #[yaserde(rename = "WorkRightsController", prefix = "ern")]
    pub work_rights_controllers: Vec<WorkRightsController>,
    #[yaserde(rename = "CourtesyLine", prefix = "ern")]
    pub courtesy_lines: Vec<CourtesyLineWithDefault>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: String,
    #[yaserde(rename = "CreationDate", prefix = "ern")]
    pub creation_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "MasteredDate", prefix = "ern")]
    pub mastered_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "RemasteredDate", prefix = "ern")]
    pub remastered_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "FirstPublicationDate", prefix = "ern")]
    pub first_publication_dates: Vec<FirstPublicationDate>,
    #[yaserde(rename = "LocationAndDateOfSession", prefix = "ern")]
    pub location_and_date_of_sessions: Vec<LocationAndDateOfSession>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "RelatedRelease", prefix = "ern")]
    pub related_releases: Vec<RelatedRelease>,
    #[yaserde(rename = "RelatedResource", prefix = "ern")]
    pub related_resources: Vec<RelatedResource>,
    #[yaserde(
        rename = "CompositeMusicalWorkType",
        prefix = "ern",
        validation = "AvsCompositeMusicalWorkTypeValidator"
    )]
    pub composite_musical_work_type: Option<String>,
    #[yaserde(rename = "IsCover", prefix = "ern")]
    pub is_cover: Option<bool>,
    #[yaserde(rename = "HasVocalPerformance", prefix = "ern")]
    pub has_vocal_performance: Option<bool>,
    #[yaserde(rename = "HasForegroundVocalPerformance", prefix = "ern")]
    pub has_foreground_vocal_performance: Option<bool>,
    #[yaserde(rename = "IsInstrumental", prefix = "ern")]
    pub is_instrumental: Option<bool>,
    #[yaserde(rename = "ContainsHiddenContent", prefix = "ern")]
    pub contains_hidden_content: Option<bool>,
    #[yaserde(rename = "IsRemastered", prefix = "ern")]
    pub is_remastered: Option<bool>,
    #[yaserde(rename = "IsHiResMusic", prefix = "ern")]
    pub is_hi_res_music: Option<bool>,
    #[yaserde(rename = "DisableCrossfade", prefix = "ern")]
    pub disable_crossfade: Option<bool>,
    #[yaserde(rename = "DisableSearch", prefix = "ern")]
    pub disable_search: Option<bool>,
    #[yaserde(rename = "DisplayCredits", prefix = "ern")]
    pub display_creditss: Vec<DisplayCredits>,
    #[yaserde(rename = "LanguageOfPerformance", prefix = "ern")]
    pub language_of_performances: Vec<Language>,
    #[yaserde(rename = "Raga", prefix = "ern")]
    pub ragas: Vec<Raga>,
    #[yaserde(rename = "Tala", prefix = "ern")]
    pub talas: Vec<Tala>,
    #[yaserde(rename = "Deity", prefix = "ern")]
    pub deitys: Vec<Deity>,
    #[yaserde(
        rename = "AudioChapterReference",
        prefix = "ern",
        validation = "AudioChapterReferenceValidator"
    )]
    pub audio_chapter_references: Vec<String>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "IsSupplemental")]
    pub is_supplemental: Option<bool>,
    #[yaserde(attribute, rename = "ApplyClassicalProfileVariant")]
    pub apply_classical_profile_variant: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SoundRecordingClipDetails {
    #[yaserde(
        rename = "TechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "TechnicalResourceDetailsReferenceValidator"
    )]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "ClipType", prefix = "ern")]
    pub clip_type: ClipType,
    #[yaserde(rename = "Timing", prefix = "ern")]
    pub timings: Vec<Timing>,
    #[yaserde(
        rename = "ExpressionType",
        prefix = "ern",
        validation = "AvsExpressionTypeValidator"
    )]
    pub expression_type: String,
    #[yaserde(rename = "DeliveryFile", prefix = "ern")]
    pub delivery_files: Vec<AudioDeliveryFile>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SoundRecordingEdition {
    #[yaserde(
        rename = "Type",
        prefix = "ern",
        validation = "AvsEditionTypeValidator"
    )]
    pub kind: Option<String>,
    #[yaserde(rename = "ResourceId", prefix = "ern")]
    pub resource_ids: Vec<SoundRecordingId>,
    #[yaserde(rename = "EditionContributor", prefix = "ern")]
    pub edition_contributors: Vec<EditionContributor>,
    #[yaserde(rename = "PLine", prefix = "ern")]
    pub p_lines: Vec<PLineWithDefault>,
    #[yaserde(
        rename = "RecordingMode",
        prefix = "ern",
        validation = "AvsRecordingModeValidator"
    )]
    pub recording_mode: Option<String>,
    #[yaserde(rename = "TechnicalDetails", prefix = "ern")]
    pub technical_detailss: Vec<TechnicalSoundRecordingDetails>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SupplementalDocumentList {
    #[yaserde(rename = "SupplementalDocument", prefix = "ern")]
    pub supplemental_documents: Vec<File>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SynopsisWithTerritory {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[yaserde(attribute, rename = "IsShortSynopsis")]
    pub is_short_synopsis: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Tala {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TechnicalImageDetails {
    #[yaserde(
        rename = "TechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "TechnicalResourceDetailsReferenceValidator"
    )]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "ImageCodecType", prefix = "ern")]
    pub image_codec_type: Option<ImageCodecType>,
    #[yaserde(rename = "ImageHeight", prefix = "ern")]
    pub image_height: Option<Extent>,
    #[yaserde(rename = "ImageWidth", prefix = "ern")]
    pub image_width: Option<Extent>,
    #[yaserde(rename = "AspectRatio", prefix = "ern")]
    pub aspect_ratios: Vec<AspectRatio>,
    #[yaserde(rename = "ColorDepth", prefix = "ern")]
    pub color_depth: Option<i32>,
    #[yaserde(rename = "ImageResolution", prefix = "ern")]
    pub image_resolution: Option<i32>,
    #[yaserde(rename = "BitDepth", prefix = "ern")]
    pub bit_depth: Option<i32>,
    #[yaserde(rename = "IsClip", prefix = "ern")]
    pub is_clip: Option<bool>,
    #[yaserde(rename = "ClipDetails", prefix = "ern")]
    pub clip_detailss: Vec<ClipDetails>,
    #[yaserde(rename = "File", prefix = "ern")]
    pub file: Option<File>,
    #[yaserde(rename = "IsProvidedInDelivery", prefix = "ern")]
    pub is_provided_in_delivery: Option<bool>,
    #[yaserde(rename = "Fingerprint", prefix = "ern")]
    pub fingerprints: Vec<Fingerprint>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TechnicalSheetMusicDetails {
    #[yaserde(
        rename = "TechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "TechnicalResourceDetailsReferenceValidator"
    )]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "SheetMusicCodecType", prefix = "ern")]
    pub sheet_music_codec_type: Option<SheetMusicCodecType>,
    #[yaserde(rename = "BitDepth", prefix = "ern")]
    pub bit_depth: Option<i32>,
    #[yaserde(rename = "IsClip", prefix = "ern")]
    pub is_clip: Option<bool>,
    #[yaserde(rename = "ClipDetails", prefix = "ern")]
    pub clip_detailss: Vec<ClipDetails>,
    #[yaserde(rename = "File", prefix = "ern")]
    pub file: Option<File>,
    #[yaserde(rename = "IsProvidedInDelivery", prefix = "ern")]
    pub is_provided_in_delivery: Option<bool>,
    #[yaserde(rename = "Fingerprint", prefix = "ern")]
    pub fingerprints: Vec<Fingerprint>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TechnicalSoftwareDetails {
    #[yaserde(
        rename = "TechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "TechnicalResourceDetailsReferenceValidator"
    )]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "OperatingSystemType", prefix = "ern")]
    pub operating_system_type: Option<OperatingSystemType>,
    #[yaserde(rename = "BitDepth", prefix = "ern")]
    pub bit_depth: Option<i32>,
    #[yaserde(rename = "IsClip", prefix = "ern")]
    pub is_clip: Option<bool>,
    #[yaserde(rename = "ClipDetails", prefix = "ern")]
    pub clip_detailss: Vec<ClipDetails>,
    #[yaserde(rename = "File", prefix = "ern")]
    pub file: Option<File>,
    #[yaserde(rename = "IsProvidedInDelivery", prefix = "ern")]
    pub is_provided_in_delivery: Option<bool>,
    #[yaserde(rename = "Fingerprint", prefix = "ern")]
    pub fingerprints: Vec<Fingerprint>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TechnicalSoundRecordingDetails {
    #[yaserde(
        rename = "TechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "TechnicalResourceDetailsReferenceValidator"
    )]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "DeliveryFile", prefix = "ern")]
    pub delivery_files: Vec<AudioDeliveryFile>,
    #[yaserde(rename = "HasImmersiveAudioMetadata", prefix = "ern")]
    pub has_immersive_audio_metadata: Option<bool>,
    #[yaserde(rename = "IsClip", prefix = "ern")]
    pub is_clip: Option<bool>,
    #[yaserde(rename = "ClipDetails", prefix = "ern")]
    pub clip_detailss: Vec<SoundRecordingClipDetails>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TechnicalTextDetails {
    #[yaserde(
        rename = "TechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "TechnicalResourceDetailsReferenceValidator"
    )]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "TextCodecType", prefix = "ern")]
    pub text_codec_type: Option<TextCodecType>,
    #[yaserde(rename = "BitDepth", prefix = "ern")]
    pub bit_depth: Option<i32>,
    #[yaserde(rename = "IsClip", prefix = "ern")]
    pub is_clip: Option<bool>,
    #[yaserde(rename = "ClipDetails", prefix = "ern")]
    pub clip_detailss: Vec<ClipDetails>,
    #[yaserde(rename = "File", prefix = "ern")]
    pub file: Option<File>,
    #[yaserde(rename = "IsProvidedInDelivery", prefix = "ern")]
    pub is_provided_in_delivery: Option<bool>,
    #[yaserde(rename = "Fingerprint", prefix = "ern")]
    pub fingerprints: Vec<Fingerprint>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TechnicalVideoDetails {
    #[yaserde(
        rename = "TechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "TechnicalResourceDetailsReferenceValidator"
    )]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "OverallBitRate", prefix = "ern")]
    pub overall_bit_rate: Option<BitRate>,
    #[yaserde(rename = "DeliveryFile", prefix = "ern")]
    pub delivery_files: Vec<VideoDeliveryFile>,
    #[yaserde(rename = "IsClip", prefix = "ern")]
    pub is_clip: Option<bool>,
    #[yaserde(rename = "ClipDetails", prefix = "ern")]
    pub clip_detailss: Vec<VideoClipDetails>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Text {
    #[yaserde(
        rename = "ResourceReference",
        prefix = "ern",
        validation = "ResourceReferenceValidator"
    )]
    pub resource_reference: String,
    #[yaserde(rename = "Type", prefix = "ern")]
    pub kind: TextType,
    #[yaserde(rename = "ResourceId", prefix = "ern")]
    pub resource_ids: Vec<TextId>,
    #[yaserde(rename = "WorkId", prefix = "ern")]
    pub work_ids: Vec<MusicalWorkId>,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "VersionType", prefix = "ern")]
    pub version_types: Vec<VersionType>,
    #[yaserde(rename = "DisplayArtistName", prefix = "ern")]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "Contributor", prefix = "ern")]
    pub contributors: Vec<Contributor>,
    #[yaserde(rename = "ResourceRightsController", prefix = "ern")]
    pub resource_rights_controllers: Vec<ResourceRightsController>,
    #[yaserde(rename = "WorkRightsController", prefix = "ern")]
    pub work_rights_controllers: Vec<WorkRightsController>,
    #[yaserde(rename = "CLine", prefix = "ern")]
    pub c_lines: Vec<CLineWithDefault>,
    #[yaserde(rename = "CourtesyLine", prefix = "ern")]
    pub courtesy_lines: Vec<CourtesyLineWithDefault>,
    #[yaserde(rename = "CreationDate", prefix = "ern")]
    pub creation_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "FirstPublicationDate", prefix = "ern")]
    pub first_publication_dates: Vec<FulfillmentDateWithTerritory>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "RelatedRelease", prefix = "ern")]
    pub related_releases: Vec<RelatedRelease>,
    #[yaserde(rename = "RelatedResource", prefix = "ern")]
    pub related_resources: Vec<RelatedResource>,
    #[yaserde(rename = "ContainsHiddenContent", prefix = "ern")]
    pub contains_hidden_content: Option<bool>,
    #[yaserde(rename = "ResourceContainedResourceReferenceList", prefix = "ern")]
    pub resource_contained_resource_reference_list: Option<ResourceContainedResourceReferenceList>,
    #[yaserde(rename = "TechnicalDetails", prefix = "ern")]
    pub technical_detailss: Vec<TechnicalTextDetails>,
    #[yaserde(rename = "LanguageOfText", prefix = "ern")]
    pub language_of_texts: Vec<Language>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "IsSupplemental")]
    pub is_supplemental: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Timing {
    #[yaserde(rename = "StartPoint", prefix = "ern")]
    pub start_point: String,
    #[yaserde(rename = "EndPoint", prefix = "ern")]
    pub end_point: Option<String>,
    #[yaserde(rename = "DurationUsed", prefix = "ern")]
    pub duration_useds: Vec<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Title {
    #[yaserde(rename = "TitleText", prefix = "ern")]
    pub title_text: String,
    #[yaserde(rename = "SubTitle", prefix = "ern")]
    pub sub_title: Option<String>,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "TitleType",
        validation = "AvsAdditionalTitleTypeValidator"
    )]
    pub title_type: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TrackRelease {
    #[yaserde(
        rename = "ReleaseReference",
        prefix = "ern",
        validation = "ReleaseReferenceValidator"
    )]
    pub release_reference: String,
    #[yaserde(rename = "ReleaseId", prefix = "ern")]
    pub release_id: ReleaseId,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(
        rename = "ReleaseResourceReference",
        prefix = "ern",
        validation = "ReleaseResourceReferenceValidator"
    )]
    pub release_resource_reference: String,
    #[yaserde(rename = "LinkedReleaseResourceReference", prefix = "ern")]
    pub linked_release_resource_references: Vec<LinkedReleaseResourceReference>,
    #[yaserde(rename = "ReleaseLabelReference", prefix = "ern")]
    pub release_label_references: Vec<ReleaseLabelReferenceWithParty>,
    #[yaserde(rename = "Genre", prefix = "ern")]
    pub genres: Vec<GenreWithTerritory>,
    #[yaserde(rename = "ReleaseVisibilityReference", prefix = "ern", , validation = "ReleaseVisibilityReferenceValidator")]
    pub release_visibility_references: Vec<String>,
    #[yaserde(rename = "RelatedRelease", prefix = "ern")]
    pub related_releases: Vec<RelatedRelease>,
    #[yaserde(rename = "RelatedResource", prefix = "ern")]
    pub related_resources: Vec<RelatedResource>,
    #[yaserde(rename = "TargetURL", prefix = "ern")]
    pub target_url: Option<String>,
    #[yaserde(rename = "Keywords", prefix = "ern")]
    pub keywordss: Vec<KeywordsWithTerritory>,
    #[yaserde(rename = "Synopsis", prefix = "ern")]
    pub synopsiss: Vec<SynopsisWithTerritory>,
    #[yaserde(rename = "MarketingComment", prefix = "ern")]
    pub marketing_comments: Vec<MarketingComment>,
    #[yaserde(attribute, rename = "IsMainRelease")]
    pub is_main_release: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TrackReleaseVisibility {
    #[yaserde(
        rename = "VisibilityReference",
        prefix = "ern",
        validation = "VisibilityReferenceValidator"
    )]
    pub visibility_reference: String,
    #[yaserde(rename = "TrackListingPreviewStartDateTime", prefix = "ern")]
    pub track_listing_preview_start_date_time: String,
    #[yaserde(rename = "ClipPreviewStartDateTime", prefix = "ern")]
    pub clip_preview_start_date_time: Option<String>,
    #[yaserde(attribute, rename = "DoNotDisplayDates")]
    pub do_not_display_dates: Option<bool>,
    #[yaserde(
        rename = "TerritoryCode",
        prefix = "ern",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub territory_codes: Vec<String>,
    #[yaserde(
        rename = "ExcludedTerritoryCode",
        prefix = "ern",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub excluded_territory_codes: Vec<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct UseType {
    #[yaserde(text, validation = "AvsUseTypeERNValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct UserInterfaceType {
    #[yaserde(text, validation = "AvsUserInterfaceTypeERNValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Video {
    #[yaserde(
        rename = "ResourceReference",
        prefix = "ern",
        validation = "ResourceReferenceValidator"
    )]
    pub resource_reference: String,
    #[yaserde(rename = "Type", prefix = "ern")]
    pub kind: VideoType,
    #[yaserde(rename = "VideoEdition", prefix = "ern")]
    pub video_editions: Vec<VideoEdition>,
    #[yaserde(rename = "RecordingFormat", prefix = "ern")]
    pub recording_formats: Vec<RecordingFormat>,
    #[yaserde(rename = "WorkId", prefix = "ern")]
    pub work_ids: Vec<MusicalWorkId>,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "AdditionalTitle", prefix = "ern")]
    pub additional_titles: Vec<AdditionalTitle>,
    #[yaserde(rename = "VersionType", prefix = "ern")]
    pub version_types: Vec<VersionType>,
    #[yaserde(rename = "DisplayArtistName", prefix = "ern")]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "Contributor", prefix = "ern")]
    pub contributors: Vec<Contributor>,
    #[yaserde(rename = "Character", prefix = "ern")]
    pub characters: Vec<Character>,
    #[yaserde(rename = "ResourceRightsController", prefix = "ern")]
    pub resource_rights_controllers: Vec<ResourceRightsController>,
    #[yaserde(rename = "WorkRightsController", prefix = "ern")]
    pub work_rights_controllers: Vec<WorkRightsController>,
    #[yaserde(rename = "CourtesyLine", prefix = "ern")]
    pub courtesy_lines: Vec<CourtesyLineWithDefault>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: String,
    #[yaserde(rename = "CreationDate", prefix = "ern")]
    pub creation_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "MasteredDate", prefix = "ern")]
    pub mastered_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "RemasteredDate", prefix = "ern")]
    pub remastered_dates: Vec<EventDateWithoutFlags>,
    #[yaserde(rename = "FirstPublicationDate", prefix = "ern")]
    pub first_publication_dates: Vec<FulfillmentDateWithTerritory>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "AvRating", prefix = "ern")]
    pub av_ratings: Vec<AvRating>,
    #[yaserde(rename = "RelatedRelease", prefix = "ern")]
    pub related_releases: Vec<RelatedRelease>,
    #[yaserde(rename = "RelatedResource", prefix = "ern")]
    pub related_resources: Vec<RelatedResource>,
    #[yaserde(
        rename = "CompositeMusicalWorkType",
        prefix = "ern",
        validation = "AvsCompositeMusicalWorkTypeValidator"
    )]
    pub composite_musical_work_type: Option<String>,
    #[yaserde(rename = "IsCover", prefix = "ern")]
    pub is_cover: Option<bool>,
    #[yaserde(rename = "HasVocalPerformance", prefix = "ern")]
    pub has_vocal_performance: Option<bool>,
    #[yaserde(rename = "HasForegroundVocalPerformance", prefix = "ern")]
    pub has_foreground_vocal_performance: Option<bool>,
    #[yaserde(rename = "IsInstrumental", prefix = "ern")]
    pub is_instrumental: Option<bool>,
    #[yaserde(rename = "ContainsHiddenContent", prefix = "ern")]
    pub contains_hidden_content: Option<bool>,
    #[yaserde(rename = "IsRemastered", prefix = "ern")]
    pub is_remastered: Option<bool>,
    #[yaserde(rename = "DisplayCredits", prefix = "ern")]
    pub display_creditss: Vec<DisplayCredits>,
    #[yaserde(rename = "LanguageOfPerformance", prefix = "ern")]
    pub language_of_performances: Vec<Language>,
    #[yaserde(
        rename = "LanguageOfDubbing",
        prefix = "ern",
        validation = "LanguageOfDubbingValidator"
    )]
    pub language_of_dubbings: Vec<String>,
    #[yaserde(
        rename = "SubTitleLanguage",
        prefix = "ern",
        validation = "SubTitleLanguageValidator"
    )]
    pub sub_title_languages: Vec<String>,
    #[yaserde(rename = "ResourceContainedResourceReferenceList", prefix = "ern")]
    pub resource_contained_resource_reference_list: Option<ResourceContainedResourceReferenceList>,
    #[yaserde(rename = "Raga", prefix = "ern")]
    pub ragas: Vec<String>,
    #[yaserde(rename = "Tala", prefix = "ern")]
    pub talas: Vec<String>,
    #[yaserde(rename = "Deity", prefix = "ern")]
    pub deitys: Vec<String>,
    #[yaserde(
        rename = "VideoChapterReference",
        prefix = "ern",
        validation = "VideoChapterReferenceValidator"
    )]
    pub video_chapter_references: Vec<String>,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "IsSupplemental")]
    pub is_supplemental: Option<bool>,
    #[yaserde(attribute, rename = "ApplyClassicalProfileVariant")]
    pub apply_classical_profile_variant: Option<bool>,
    #[yaserde(
        rename = "VideoCueSheetReference",
        prefix = "ern",
        validation = "VideoCueSheetReferenceValidator"
    )]
    pub video_cue_sheet_references: Vec<String>,
    #[yaserde(rename = "ReasonForCueSheetAbsence", prefix = "ern")]
    pub reason_for_cue_sheet_absence: Option<Reason>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct VideoClipDetails {
    #[yaserde(
        rename = "TechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "TechnicalResourceDetailsReferenceValidator"
    )]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "ClipType", prefix = "ern")]
    pub clip_type: ClipType,
    #[yaserde(rename = "Timing", prefix = "ern")]
    pub timings: Vec<Timing>,
    #[yaserde(
        rename = "TopLeftCorner",
        prefix = "ern",
        validation = "TopLeftCornerValidator"
    )]
    pub top_left_corner: Option<String>,
    #[yaserde(
        rename = "BottomRightCorner",
        prefix = "ern",
        validation = "BottomRightCornerValidator"
    )]
    pub bottom_right_corner: Option<String>,
    #[yaserde(
        rename = "ExpressionType",
        prefix = "ern",
        validation = "AvsExpressionTypeValidator"
    )]
    pub expression_type: String,
    #[yaserde(rename = "DeliveryFile", prefix = "ern")]
    pub delivery_files: Vec<VideoDeliveryFile>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct VideoDeliveryFile {
    #[yaserde(
        rename = "Type",
        prefix = "ern",
        validation = "AvsDeliveryFileTypeValidator"
    )]
    pub kind: String,
    #[yaserde(rename = "ContainerFormat", prefix = "ern")]
    pub container_format: Option<ContainerFormat>,
    #[yaserde(rename = "VideoCodecType", prefix = "ern")]
    pub video_codec_type: Option<VideoCodecType>,
    #[yaserde(rename = "VideoBitRate", prefix = "ern")]
    pub video_bit_rate: Option<BitRate>,
    #[yaserde(rename = "FrameRate", prefix = "ern")]
    pub frame_rate: Option<FrameRate>,
    #[yaserde(rename = "ImageHeight", prefix = "ern")]
    pub image_height: Option<Extent>,
    #[yaserde(rename = "ImageWidth", prefix = "ern")]
    pub image_width: Option<Extent>,
    #[yaserde(rename = "AspectRatio", prefix = "ern")]
    pub aspect_ratios: Vec<AspectRatio>,
    #[yaserde(rename = "CoreArea", prefix = "ern")]
    pub core_area: Option<CoreArea>,
    #[yaserde(rename = "ColorDepth", prefix = "ern")]
    pub color_depth: Option<i32>,
    #[yaserde(rename = "VideoDefinitionType", prefix = "ern")]
    pub video_definition_type: Option<VideoDefinitionType>,
    #[yaserde(rename = "AudioCodecType", prefix = "ern")]
    pub audio_codec_type: Option<AudioCodecType>,
    #[yaserde(rename = "HasImmersiveAudioMetadata", prefix = "ern")]
    pub has_immersive_audio_metadata: Option<bool>,
    #[yaserde(
        rename = "ElectroOpticalTransferFunctionType",
        prefix = "ern",
        validation = "AvsElectroOpticalTransferFunctionTypeValidator"
    )]
    pub electro_optical_transfer_function_type: Option<String>,
    #[yaserde(
        rename = "PrimaryColorType",
        prefix = "ern",
        validation = "AvsPrimaryColorTypeValidator"
    )]
    pub primary_color_type: Option<String>,
    #[yaserde(rename = "HdrVideoDynamicMetadataType", prefix = "ern")]
    pub hdr_video_dynamic_metadata_type: Option<HdrVideoDynamicMetadataType>,
    #[yaserde(
        rename = "HdrVideoStaticMetadataType",
        prefix = "ern",
        validation = "AvsHdrVideoStaticMetadataTypeValidator"
    )]
    pub hdr_video_static_metadata_type: Option<String>,
    #[yaserde(rename = "AudioBitRate", prefix = "ern")]
    pub audio_bit_rate: Option<BitRate>,
    #[yaserde(rename = "NumberOfAudioChannels", prefix = "ern")]
    pub number_of_audio_channels: Option<i32>,
    #[yaserde(rename = "NumberOfAudioObjects", prefix = "ern")]
    pub number_of_audio_objects: Option<i32>,
    #[yaserde(rename = "AudioSamplingRate", prefix = "ern")]
    pub audio_sampling_rate: Option<SamplingRate>,
    #[yaserde(rename = "AudioBitsPerSample", prefix = "ern")]
    pub audio_bits_per_sample: Option<i32>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: Option<String>,
    #[yaserde(rename = "BitDepth", prefix = "ern")]
    pub bit_depth: Option<i32>,
    #[yaserde(rename = "File", prefix = "ern")]
    pub file: Option<File>,
    #[yaserde(rename = "Fingerprint", prefix = "ern")]
    pub fingerprints: Vec<Fingerprint>,
    #[yaserde(rename = "IsProvidedInDelivery", prefix = "ern")]
    pub is_provided_in_delivery: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct VideoEdition {
    #[yaserde(
        rename = "Type",
        prefix = "ern",
        validation = "AvsEditionTypeValidator"
    )]
    pub kind: Option<String>,
    #[yaserde(rename = "ResourceId", prefix = "ern")]
    pub resource_ids: Vec<VideoId>,
    #[yaserde(rename = "EditionContributor", prefix = "ern")]
    pub edition_contributors: Vec<EditionContributor>,
    #[yaserde(rename = "PLine", prefix = "ern")]
    pub p_lines: Vec<PLineWithDefault>,
    #[yaserde(rename = "CLine", prefix = "ern")]
    pub c_lines: Vec<CLineWithDefault>,
    #[yaserde(
        rename = "RecordingMode",
        prefix = "ern",
        validation = "AvsRecordingModeValidator"
    )]
    pub recording_mode: Option<String>,
    #[yaserde(rename = "TechnicalDetails", prefix = "ern")]
    pub technical_detailss: Vec<TechnicalVideoDetails>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct VideoType {
    #[yaserde(text, validation = "AvsVideoTypeERN43Validator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct WorkRightsController {
    #[yaserde(
        rename = "RightsControllerPartyReference",
        prefix = "ern",
        validation = "RightsControllerPartyReferenceValidator"
    )]
    pub rights_controller_party_reference: String,
    #[yaserde(
        rename = "RightsControlType",
        prefix = "ern",
        validation = "AvsRightsControllerRoleValidator"
    )]
    pub rights_control_types: Vec<String>,
    #[yaserde(
        rename = "RightsControllerType",
        prefix = "ern",
        validation = "AvsRightsControllerTypeValidator"
    )]
    pub rights_controller_type: Option<String>,
    #[yaserde(rename = "Territory", prefix = "ern")]
    pub territorys: Vec<AllTerritoryCode>,
    #[yaserde(
        rename = "StartDate",
        prefix = "ern",
        validation = "DdexIsoDateValidator"
    )]
    pub start_date: Option<String>,
    #[yaserde(
        rename = "EndDate",
        prefix = "ern",
        validation = "DdexIsoDateValidator"
    )]
    pub end_date: Option<String>,
    #[yaserde(rename = "RightShareUnknown", prefix = "ern")]
    pub right_share_unknown: Option<bool>,
    #[yaserde(rename = "RightSharePercentage", prefix = "ern")]
    pub right_share_percentage: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct AdministratingRecordCompanyRole {
    #[yaserde(text, validation = "AvsAdministratingRecordCompanyRoleValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Affiliation {
    #[yaserde(
        rename = "Type",
        prefix = "ern",
        validation = "AvsAffiliationTypeValidator"
    )]
    pub kind: String,
    #[yaserde(rename = "ValidityPeriod", prefix = "ern")]
    pub validity_period: Option<ValidityPeriod>,
    #[yaserde(rename = "RightsType", prefix = "ern")]
    pub rights_types: Vec<RightsType>,
    #[yaserde(rename = "PercentageOfRightsAssignment", prefix = "ern")]
    pub percentage_of_rights_assignment: Option<String>,
    #[yaserde(rename = "CompanyName", prefix = "ern")]
    pub company_name: Option<String>,
    #[yaserde(
        rename = "PartyAffiliateReference",
        prefix = "ern",
        validation = "PartyAffiliateReferenceValidator"
    )]
    pub party_affiliate_reference: Option<String>,
    #[yaserde(
        rename = "TerritoryCode",
        prefix = "ern",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub territory_codes: Vec<String>,
    #[yaserde(
        rename = "ExcludedTerritoryCode",
        prefix = "ern",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub excluded_territory_codes: Vec<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct AllTerritoryCode {
    #[yaserde(text, validation = "AvsAllTerritoryCodeValidator")]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "IdentifierType",
        validation = "AvsTerritoryCodeTypeIncludingDeprecatedCodesValidator"
    )]
    pub identifier_type: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct AspectRatio {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "AspectRatioType",
        validation = "AvsAspectRatioTypeValidator"
    )]
    pub aspect_ratio_type: Option<String>,
    #[yaserde(attribute, rename = "AppliesToCroppedResource")]
    pub applies_to_cropped_resource: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct AudioCodecType {
    #[yaserde(text, validation = "AvsAudioCodecTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Version")]
    pub version: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct BitRate {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "UnitOfMeasure",
        validation = "AvsUnitOfBitRateValidator"
    )]
    pub unit_of_measure: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CLine {
    #[yaserde(rename = "Year", prefix = "ern")]
    pub year: Option<u16>,
    #[yaserde(rename = "CLineCompany", prefix = "ern")]
    pub c_line_company: Option<String>,
    #[yaserde(rename = "CLineText", prefix = "ern")]
    pub c_line_text: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CarrierType {
    #[yaserde(text, validation = "AvsCarrierTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CatalogNumber {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: String,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ClipType {
    #[yaserde(text, validation = "AvsClipTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ContainerFormat {
    #[yaserde(text, validation = "AvsContainerFormatValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ContributorRole {
    #[yaserde(text, validation = "AvsContributorRoleValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CueOrigin {
    #[yaserde(text, validation = "AvsCueOriginValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CueSheetType {
    #[yaserde(text, validation = "AvsCueSheetTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CueThemeType {
    #[yaserde(text, validation = "AvsThemeTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CueUseType {
    #[yaserde(text, validation = "AvsCueUseTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CueVisualPerceptionType {
    #[yaserde(text, validation = "AvsVisualPerceptionTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CueVocalType {
    #[yaserde(text, validation = "AvsVocalTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct CurrentTerritoryCode {
    #[yaserde(text, validation = "AvsCurrentTerritoryCodeValidator")]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "IdentifierType",
        validation = "AvsTerritoryCodeTypeValidator"
    )]
    pub identifier_type: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Dsp {
    #[yaserde(rename = "TradingName", prefix = "ern")]
    pub trading_name: Option<Name>,
    #[yaserde(rename = "URL", prefix = "ern")]
    pub urls: Vec<String>,
    #[yaserde(rename = "PartyName", prefix = "ern")]
    pub parties_names: Vec<PartyName>,
    #[yaserde(rename = "PartyId", prefix = "ern")]
    pub parties_ids: Vec<DetailedPartyId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DetailedHashSum {
    #[yaserde(rename = "Algorithm", prefix = "ern")]
    pub algorithm: HashSumAlgorithmType,
    #[yaserde(rename = "Version", prefix = "ern")]
    pub version: Option<String>,
    #[yaserde(rename = "Parameter", prefix = "ern")]
    pub parameter: Option<String>,
    #[yaserde(
        rename = "DataType",
        prefix = "ern",
        validation = "AvsBinaryDataTypeValidator"
    )]
    pub data_type: Option<String>,
    #[yaserde(rename = "HashSumValue", prefix = "ern")]
    pub hash_sum_value: String,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DetailedPartyId {
    #[yaserde(rename = "ISNI", prefix = "ern")]
    pub isni: Option<String>,
    #[yaserde(rename = "DPID", prefix = "ern", validation = "DPIDValidator")]
    pub dpid: Option<String>,
    #[yaserde(
        rename = "IpiNameNumber",
        prefix = "ern",
        validation = "IpiNameNumberValidator"
    )]
    pub ipi_name_number: Option<String>,
    #[yaserde(rename = "IPN", prefix = "ern")]
    pub ipn: Option<String>,
    #[yaserde(rename = "CisacSocietyId", prefix = "ern")]
    pub cisac_society_id: Option<String>,
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    pub proprietary_ids: Vec<ProprietaryId>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::
    YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DisplayArtistRole {
    #[yaserde(text, validation = "AvsDisplayArtistRoleValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DisplayCredits {
    #[yaserde(rename = "DisplayCreditText", prefix = "ern")]
    pub display_credit_text: String,
    #[yaserde(
        rename = "DisplayCreditParty",
        prefix = "ern",
        validation = "DisplayCreditPartyValidator"
    )]
    pub display_credit_parties: Vec<String>,
    #[yaserde(rename = "NameUsedInDisplayCredit", prefix = "ern")]
    pub names_used_in_display_credits: Vec<String>,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDate {
    #[yaserde(text, validation = "DdexIsoDateValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "IsApproximate")]
    pub is_approximate: Option<bool>,
    #[yaserde(attribute, rename = "IsBefore")]
    pub is_before: Option<bool>,
    #[yaserde(attribute, rename = "IsAfter")]
    pub is_after: Option<bool>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsAllTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "LocationDescription")]
    pub location_description: Option<String>,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct EventDateOptional {
    #[yaserde(rename = "StartDate", prefix = "ern")]
    pub start_date: Option<EventDate>,
    #[yaserde(rename = "EndDate", prefix = "ern")]
    pub end_date: Option<EventDate>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDateTime {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "IsApproximate")]
    pub is_approximate: Option<bool>,
    #[yaserde(attribute, rename = "IsBefore")]
    pub is_before: Option<bool>,
    #[yaserde(attribute, rename = "IsAfter")]
    pub is_after: Option<bool>,
    #[yaserde(
        attribute,
        rename = "TerritoryCode",
        validation = "AvsAllTerritoryCodeValidator"
    )]
    pub territory_code: Option<String>,
    #[yaserde(attribute, rename = "LocationDescription")]
    pub location_description: Option<String>,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDateTimeOptional {
    #[yaserde(rename = "StartDateTime", prefix = "ern")]
    pub start_date_time: Option<EventDateTime>,
    #[yaserde(rename = "EndDateTime", prefix = "ern")]
    pub end_date_time: Option<EventDateTime>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Extent {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "UnitOfMeasure",
        validation = "AvsUnitOfExtentValidator"
    )]
    pub unit_of_measure: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ExternallyLinkedResourceType {
    #[yaserde(text, validation = "AvsExternallyLinkedResourceTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct File {
    #[yaserde(rename = "URI", prefix = "ern")]
    pub uri: String,
    #[yaserde(rename = "HashSum", prefix = "ern")]
    pub hash_sum: Option<DetailedHashSum>,
    #[yaserde(rename = "FileSize", prefix = "ern")]
    pub file_size: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct FingerprintAlgorithmType {
    #[yaserde(text, validation = "AvsFingerprintAlgorithmTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct FirstPublicationDate {
    #[yaserde(text, validation = "DdexIsoDateValidator")]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct FrameRate {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "UnitOfMeasure",
        validation = "AvsUnitOfFrameRateValidator"
    )]
    pub unit_of_measure: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct FulfillmentDateWithTerritory {
    #[yaserde(
        rename = "FulfillmentDate",
        prefix = "ern",
        validation = "DdexIsoDateValidator"
    )]
    pub fulfillment_date: String,
    #[yaserde(
        rename = "ResourceReleaseReference",
        prefix = "ern",
        validation = "ResourceReleaseReferenceValidator"
    )]
    pub resource_release_references: Vec<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct GenreCategory {
    #[yaserde(rename = "Value", prefix = "ern")]
    pub value: GenreCategoryValue,
    #[yaserde(rename = "Description", prefix = "ern")]
    pub descriptions: Vec<TextWithoutTerritory>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct GenreCategoryValue {
    #[yaserde(text, validation = "AvsClassifiedGenreValidator")]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct GenreWithTerritory {
    #[yaserde(rename = "GenreText", prefix = "ern")]
    pub genre_text: String,
    #[yaserde(rename = "SubGenre", prefix = "ern")]
    pub sub_genre: Option<String>,
    #[yaserde(rename = "GenreCategory", prefix = "ern")]
    pub genre_categorys: Vec<GenreCategory>,
    #[yaserde(rename = "SubGenreCategory", prefix = "ern")]
    pub sub_genre_categorys: Vec<SubGenreCategory>,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct HashSumAlgorithmType {
    #[yaserde(text, validation = "AvsHashSumAlgorithmTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ImageCodecType {
    #[yaserde(text, validation = "AvsImageCodecTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Version")]
    pub version: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ImageType {
    #[yaserde(text, validation = "AvsImageTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct InstrumentType {
    #[yaserde(text, validation = "AvsInstrumentTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct KeywordsWithTerritory {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Language {
    #[yaserde(text, validation = "DdexLanguageAndScriptCodeWithRestrictionValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "IsMainLanguage")]
    pub is_main_language: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct MarketingComment {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct MessageAuditTrail {
    #[yaserde(rename = "MessageAuditTrailEvent", prefix = "ern")]
    pub message_audit_trail_events: Vec<MessageAuditTrailEvent>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct MessageAuditTrailEvent {
    #[yaserde(rename = "MessagingPartyDescriptor", prefix = "ern")]
    pub messaging_party_descriptor: MessagingPartyWithoutCode,
    #[yaserde(rename = "DateTime", prefix = "ern")]
    pub date_time: String,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct MessageHeader {
    #[yaserde(rename = "MessageThreadId", prefix = "ern")]
    pub message_thread_id: Option<String>,
    #[yaserde(rename = "MessageId", prefix = "ern")]
    pub message_id: String,
    #[yaserde(rename = "MessageFileName", prefix = "ern")]
    pub message_file_name: Option<String>,
    #[yaserde(rename = "MessageSender", prefix = "ern")]
    pub message_sender: MessagingPartyWithoutCode,
    #[yaserde(rename = "SentOnBehalfOf", prefix = "ern")]
    pub sent_on_behalf_of: Option<MessagingPartyWithoutCode>,
    #[yaserde(rename = "MessageRecipient", prefix = "ern")]
    pub message_recipients: Vec<MessagingPartyWithoutCode>,
    #[yaserde(rename = "MessageCreatedDateTime", prefix = "ern")]
    pub message_created_date_time: String,
    #[yaserde(rename = "MessageAuditTrail", prefix = "ern")]
    pub message_audit_trail: Option<MessageAuditTrail>,
    #[yaserde(
        rename = "MessageControlType",
        prefix = "ern",
        validation = "AvsMessageControlTypeValidator"
    )]
    pub message_control_type: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct MessagingPartyWithoutCode {
    #[yaserde(rename = "PartyId", prefix = "ern", validation = "PartyIdValidator")]
    pub party_id: String,
    #[yaserde(rename = "PartyName", prefix = "ern")]
    pub party_name: Option<PartyNameWithoutCode>,
    #[yaserde(rename = "TradingName", prefix = "ern")]
    pub trading_name: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct MusicalWorkId {
    #[yaserde(rename = "ISWC", prefix = "ern")]
    pub iswc: Option<String>,
    #[yaserde(rename = "OpusNumber", prefix = "ern")]
    pub opus_number: Option<String>,
    #[yaserde(rename = "ComposerCatalogNumber", prefix = "ern")]
    pub composer_catalog_numbers: Vec<String>,
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    pub proprietary_ids: Vec<ProprietaryId>,
    #[yaserde(attribute, rename = "IsReplaced")]
    pub is_replaced: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Name {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct OperatingSystemType {
    #[yaserde(text, validation = "AvsOperatingSystemTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Version")]
    pub version: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PLine {
    #[yaserde(rename = "Year", prefix = "ern")]
    pub year: Option<u16>,
    #[yaserde(rename = "PLineCompany", prefix = "ern")]
    pub p_line_company: Option<String>,
    #[yaserde(rename = "PLineText", prefix = "ern")]
    pub p_line_text: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "PLineType", validation = "AvsPLineTypeValidator")]
    pub p_line_type: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PLineWithDefault {
    #[yaserde(rename = "Year", prefix = "ern")]
    pub year: Option<u16>,
    #[yaserde(rename = "PLineCompany", prefix = "ern")]
    pub p_line_company: Option<String>,
    #[yaserde(rename = "PLineText", prefix = "ern")]
    pub p_line_text: String,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ParentalWarningTypeWithTerritory {
    #[yaserde(text, validation = "AvsParentalWarningTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PartyName {
    #[yaserde(rename = "FullName", prefix = "ern")]
    pub full_name: Name,
    #[yaserde(rename = "FullNameAsciiTranscribed", prefix = "ern")]
    pub full_name_ascii_transcribed: Option<String>,
    #[yaserde(rename = "FullNameIndexed", prefix = "ern")]
    pub full_name_indexed: Option<Name>,
    #[yaserde(rename = "NamesBeforeKeyName", prefix = "ern")]
    pub names_before_key_name: Option<Name>,
    #[yaserde(rename = "KeyName", prefix = "ern")]
    pub key_name: Option<Name>,
    #[yaserde(rename = "NamesAfterKeyName", prefix = "ern")]
    pub names_after_key_name: Option<Name>,
    #[yaserde(rename = "AbbreviatedName", prefix = "ern")]
    pub abbreviated_name: Option<Name>,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PartyNameWithoutCode {
    #[yaserde(rename = "FullName", prefix = "ern")]
    pub full_name: String,
    #[yaserde(rename = "FullNameAsciiTranscribed", prefix = "ern")]
    pub full_name_ascii_transcribed: Option<String>,
    #[yaserde(rename = "FullNameIndexed", prefix = "ern")]
    pub full_name_indexed: Option<String>,
    #[yaserde(rename = "NamesBeforeKeyName", prefix = "ern")]
    pub names_before_key_name: Option<String>,
    #[yaserde(rename = "KeyName", prefix = "ern")]
    pub key_name: Option<String>,
    #[yaserde(rename = "NamesAfterKeyName", prefix = "ern")]
    pub names_after_key_name: Option<String>,
    #[yaserde(rename = "AbbreviatedName", prefix = "ern")]
    pub abbreviated_name: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PartyRelationshipType {
    #[yaserde(text, validation = "AvsPartyRelationshipTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
    #[yaserde(attribute, rename = "MayBeShared")]
    pub may_be_shared: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Percentage {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "HasMaxValueOfOne")]
    pub has_max_value_of_one: Option<bool>,
}

#[derive(
    Clone, Debug, PartialEq, yaserde_derive::YaSerialize, serde::Serialize, serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub enum Period {
    EventDate(EventDateOptional),
    EventDateTime(EventDateTimeOptional),
}

#[derive(
    Clone, Debug, PartialEq, yaserde_derive::YaSerialize, serde::Serialize, serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub enum PeriodWithStartDate {
    EventDateWithCurrentTerritory(EventDateWithCurrentTerritoryRequired),
    EventDateTimeWithoutFlags(EventDateTimeWithoutFlagsRequired),
}

#[derive(
    Clone, Debug, PartialEq, yaserde_derive::YaSerialize, serde::Serialize, serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub enum PeriodWithoutFlags {
    EventDateWithCurrentTerritory(EventDateWithCurrentTerritoryOptional),
    EventDateTimeWithoutFlags(EventDateTimeWithoutFlagsOptional),
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Prefix {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Price {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "CurrencyCode",
        validation = "AvsCurrencyCodeValidator"
    )]
    pub currency_code: String,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PriceType {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: String,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PromotionalCode {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ProprietaryId {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: String,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Purpose {
    #[yaserde(text, validation = "AvsPurposeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct RatingAgency {
    #[yaserde(text, validation = "AvsRatingAgencyValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct RatingReason {
    #[yaserde(text, validation = "AvsRatingReasonValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Reason {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct RelatedParty {
    #[yaserde(
        rename = "PartyRelatedPartyReference",
        prefix = "ern",
        validation = "PartyRelatedPartyReferenceValidator"
    )]
    pub party_related_party_reference: String,
    #[yaserde(rename = "PartyRelationshipType", prefix = "ern")]
    pub party_relationship_type: PartyRelationshipType,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseRelationshipType {
    #[yaserde(text, validation = "AvsReleaseRelationshipTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseTypeForReleaseNotification {
    #[yaserde(text, validation = "AvsReleaseTypeERN4Validator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceContainedResourceReference {
    #[yaserde(
        rename = "ResourceContainedResourceReference",
        prefix = "ern",
        validation = "ResourceContainedResourceReferenceValidator"
    )]
    pub resource_contained_resource_reference: String,
    #[yaserde(rename = "DurationUsed", prefix = "ern")]
    pub duration_used: Option<String>,
    #[yaserde(rename = "StartPoint", prefix = "ern")]
    pub start_point: Option<String>,
    #[yaserde(rename = "Purpose", prefix = "ern")]
    pub purpose: Option<Purpose>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceContainedResourceReferenceList {
    #[yaserde(rename = "ResourceContainedResourceReference", prefix = "ern")]
    pub resource_contained_resource_references: Vec<ResourceContainedResourceReference>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceContributorRole {
    #[yaserde(text, validation = "AvsResourceContributorRoleValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceId {
    #[yaserde(rename = "ISRC", prefix = "ern")]
    pub isrc: Option<String>,
    #[yaserde(rename = "ISMN", prefix = "ern")]
    pub ismn: Option<String>,
    #[yaserde(rename = "ISAN", prefix = "ern")]
    pub isan: Option<String>,
    #[yaserde(rename = "VISAN", prefix = "ern")]
    pub visan: Option<String>,
    #[yaserde(rename = "ISBN", prefix = "ern")]
    pub isbn: Option<String>,
    #[yaserde(rename = "ISSN", prefix = "ern")]
    pub issn: Option<String>,
    #[yaserde(rename = "SICI", prefix = "ern")]
    pub sici: Option<String>,
    #[yaserde(rename = "CatalogNumber", prefix = "ern")]
    pub catalog_number: Option<CatalogNumber>,
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    pub proprietary_ids: Vec<ProprietaryId>,
    #[yaserde(attribute, rename = "IsReplaced")]
    pub is_replaced: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceProprietaryId {
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    pub proprietary_ids: Vec<ProprietaryId>,
    #[yaserde(attribute, rename = "IsReplaced")]
    pub is_replaced: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct RightsType {
    #[yaserde(text, validation = "AvsRightsCoverageValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SamplingRate {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "UnitOfMeasure",
        validation = "AvsUnitOfFrequencyValidator"
    )]
    pub unit_of_measure: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SessionType {
    #[yaserde(text, validation = "AvsSessionTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SheetMusicCodecType {
    #[yaserde(text, validation = "AvsSheetMusicCodecTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Version")]
    pub version: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SheetMusicId {
    #[yaserde(rename = "ISMN", prefix = "ern")]
    pub ismn: Option<String>,
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    pub proprietary_ids: Vec<ProprietaryId>,
    #[yaserde(attribute, rename = "IsReplaced")]
    pub is_replaced: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SheetMusicType {
    #[yaserde(text, validation = "AvsSheetMusicTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SoftwareType {
    #[yaserde(text, validation = "AvsSoftwareTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SoundRecordingId {
    #[yaserde(rename = "ISRC", prefix = "ern")]
    pub isrc: Option<String>,
    #[yaserde(rename = "CatalogNumber", prefix = "ern")]
    pub catalog_number: Option<CatalogNumber>,
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    pub proprietary_ids: Vec<ProprietaryId>,
    #[yaserde(attribute, rename = "IsReplaced")]
    pub is_replaced: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SoundRecordingType {
    #[yaserde(text, validation = "AvsSoundRecordingTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SubGenreCategory {
    #[yaserde(rename = "Value", prefix = "ern")]
    pub values: Vec<SubGenreCategoryValue>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SubGenreCategoryValue {
    #[yaserde(text, validation = "AvsSubGenreValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TextCodecType {
    #[yaserde(text, validation = "AvsTextCodecTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Version")]
    pub version: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TextId {
    #[yaserde(rename = "ISBN", prefix = "ern")]
    pub isbn: Option<String>,
    #[yaserde(rename = "ISSN", prefix = "ern")]
    pub issn: Option<String>,
    #[yaserde(rename = "SICI", prefix = "ern")]
    pub sici: Option<String>,
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    pub proprietary_ids: Vec<ProprietaryId>,
    #[yaserde(attribute, rename = "IsReplaced")]
    pub is_replaced: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TextType {
    #[yaserde(text, validation = "AvsTextTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TextWithFormat {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[yaserde(attribute, rename = "Format", validation = "AvsTextCodecTypeValidator")]
    pub format: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TextWithoutTerritory {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute, rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[yaserde(attribute, rename = "Format", validation = "AvsTextCodecTypeValidator")]
    pub format: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TitleDisplayInformation {
    #[yaserde(rename = "IsDisplayedInTitle", prefix = "ern")]
    pub is_displayed_in_title: bool,
    #[yaserde(rename = "Prefix", prefix = "ern")]
    pub prefixs: Vec<Prefix>,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ValidityPeriod {
    #[yaserde(rename = "StartDate", prefix = "ern")]
    pub start_date: Option<EventDate>,
    #[yaserde(rename = "EndDate", prefix = "ern")]
    pub end_date: Option<EventDate>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Venue {
    #[yaserde(rename = "VenueName", prefix = "ern")]
    pub venue_name: Option<String>,
    #[yaserde(rename = "VenueAddress", prefix = "ern")]
    pub venue_address: Option<String>,
    #[yaserde(rename = "TerritoryCode", prefix = "ern")]
    pub territory_code: Option<AllTerritoryCode>,
    #[yaserde(rename = "LocationCode", prefix = "ern")]
    pub location_code: Option<String>,
    #[yaserde(rename = "VenueRoom", prefix = "ern")]
    pub venue_room: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct VersionType {
    #[yaserde(text, validation = "AvsVersionTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct VideoCodecType {
    #[yaserde(text, validation = "AvsVideoCodecTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Version")]
    pub version: Option<String>,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct VideoDefinitionType {
    #[yaserde(text, validation = "AvsVideoDefinitionTypeValidator")]
    pub content: String,
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct VideoId {
    #[yaserde(rename = "ISRC", prefix = "ern")]
    pub isrc: Option<String>,
    #[yaserde(rename = "ISAN", prefix = "ern")]
    pub isan: Option<String>,
    #[yaserde(rename = "VISAN", prefix = "ern")]
    pub visan: Option<String>,
    #[yaserde(rename = "CatalogNumber", prefix = "ern")]
    pub catalog_number: Option<CatalogNumber>,
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    pub proprietary_ids: Vec<ProprietaryId>,
    #[yaserde(rename = "EIDR", prefix = "ern")]
    pub eidrs: Vec<String>,
    #[yaserde(attribute, rename = "IsReplaced")]
    pub is_replaced: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::
    YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct NewReleaseMessage {
    #[yaserde(rename = "MessageHeader", prefix = "ern")]
    pub message_header: MessageHeader,
    #[yaserde(rename = "ReleaseAdmin", prefix = "ern")]
    pub release_admins: Vec<ReleaseAdmin>,
    #[yaserde(rename = "PartyList", prefix = "ern")]
    pub party_list: PartyList,
    #[yaserde(rename = "CueSheetList", prefix = "ern")]
    pub cue_sheet_list: Option<CueSheetList>,
    #[yaserde(rename = "ResourceList", prefix = "ern")]
    pub resource_list: ResourceList,
    #[yaserde(rename = "ChapterList", prefix = "ern")]
    pub chapter_list: Option<ChapterList>,
    #[yaserde(rename = "ReleaseList", prefix = "ern")]
    pub release_list: ReleaseList,
    #[yaserde(rename = "DealList", prefix = "ern")]
    pub deal_list: Option<DealList>,
    #[yaserde(rename = "SupplementalDocumentList", prefix = "ern")]
    pub supplemental_document_list: Option<SupplementalDocumentList>,
    #[yaserde(
        attribute,
        rename = "ReleaseProfileVersionId",
        prefix = "ern",
        validation = "AvsReleaseProfileVersionIdValidator"
    )]
    pub release_profile_version_id: Option<String>,
    #[yaserde(
        attribute,
        rename = "ReleaseProfileVariantVersionId",
        prefix = "ern",
        validation = "AvsReleaseProfileVariantVersionIdValidator"
    )]
    pub release_profile_variant_version_id: Option<String>,
    #[yaserde(attribute, rename = "AvsVersionId", prefix = "ern")]
    pub avs_version_id: String,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        prefix = "ern",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: String,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::
    YaDeserialize,
    yaserde_derive::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PurgeReleaseMessage {
    #[yaserde(rename = "MessageHeader", prefix = "ern")]
    pub message_header: MessageHeader,
    #[yaserde(rename = "PurgedRelease", prefix = "ern")]
    pub purged_release: PurgedRelease,
    #[yaserde(attribute, rename = "AvsVersionId", prefix = "ern")]
    pub avs_version_id: String,
    #[yaserde(
        attribute,
        rename = "LanguageAndScriptCode",
        prefix = "ern",
        validation = "LanguageAndScriptCodeValidator"
    )]
    pub language_and_script_code: String,
}
