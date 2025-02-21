use crate::validation::*;

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct NewReleaseMessage {
    #[yaserde(rename = "MessageHeader")]
    #[validate]
    pub message_header: MessageHeader,
    #[yaserde(rename = "ReleaseAdmin")]
    pub release_admins: Vec<ReleaseAdmin>,
    #[yaserde(rename = "PartyList")]
    #[validate]
    pub party_list: PartyList,
    #[yaserde(rename = "ResourceList")]
    #[validate]
    pub resource_list: ResourceList,
    #[yaserde(rename = "ReleaseList")]
    #[validate]
    pub release_list: ReleaseList,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct MessageHeader {
    #[yaserde(rename = "MessageId")]
    pub message_id: String,
    #[yaserde(rename = "MessageSender")]
    #[validate]
    pub message_sender: MessagingPartyWithoutCode,
    #[yaserde(rename = "MessageRecipient")]
    #[validate]
    #[validate(min_items = 1)]
    pub message_recipients: Vec<MessagingPartyWithoutCode>,
    #[yaserde(rename = "MessageCreatedDateTime")]
    pub message_created_date_time: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct MessagingPartyWithoutCode {
    #[yaserde(rename = "PartyId")]
    #[validate(custom = PartyIdValidator::json_validate)]
    pub party_id: String,
    #[yaserde(rename = "PartyName")]
    pub party_name: Option<PartyNameWithoutCode>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct PartyNameWithoutCode {
    #[yaserde(rename = "FullName")]
    pub full_name: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ReleaseAdmin {
    #[yaserde(rename = "ReleaseAdminId")]
    pub release_admin_id: String,
    #[yaserde(rename = "PersonnelDescription")]
    pub personnel_description: Option<String>,
    #[yaserde(rename = "SystemDescription")]
    pub system_descriptions: Vec<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct PartyList {
    #[yaserde(rename = "Party")]
    #[validate]
    #[validate(min_items = 1)]
    pub partys: Vec<Party>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
#[validate(custom = |s| ChoiceValidator::party(&s.parties_names, &s.parties_ids))]
pub struct Party {
    #[yaserde(rename = "PartyReference")]
    #[validate(custom = PartyReferenceValidator::json_validate)]
    pub party_reference: String,
    #[yaserde(rename = "Affiliation")]
    #[validate]
    pub affiliations: Vec<Affiliation>,
    #[yaserde(rename = "PartyName")]
    pub parties_names: Vec<PartyNameWithTerritory>,
    #[yaserde(rename = "PartyId")]
    #[validate]
    pub parties_ids: Vec<DetailedPartyId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[validate(custom = |s| ChoiceValidator::affiliation(&s.company_name, &s.party_affiliate_reference))]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct Affiliation {
    #[yaserde(rename = "Type")]
    #[validate(custom = AvsAffiliationTypeValidator::json_validate)]
    #[validate(custom = ProtocolValidator::affiliation_type)]
    pub kind: String,
    #[yaserde(rename = "ValidityPeriod")]
    pub validity_period: Option<ValidityPeriod>,
    #[yaserde(rename = "RightsType")]
    #[validate]
    #[validate(custom = ProtocolValidator::right_types)]
    pub rights_types: Vec<RightsType>,
    #[yaserde(rename = "PercentageOfRightsAssignment")]
    pub percentage_of_rights_assignment: Option<String>,
    #[yaserde(rename = "CompanyName")]
    pub company_name: Option<String>,
    #[yaserde(rename = "PartyAffiliateReference")]
    #[validate(custom = PartyAffiliateReferenceValidator::json_validate)]
    pub party_affiliate_reference: Option<String>,
    #[yaserde(rename = "TerritoryCode")]
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate_vec)]
    #[validate(min_items = 1)]
    pub territory_codes: Vec<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ValidityPeriod {
    #[yaserde(rename = "StartDate")]
    #[validate]
    pub start_date: Option<EventDate>,
    #[yaserde(rename = "EndDate")]
    #[validate]
    pub end_date: Option<EventDate>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct EventDate {
    #[yaserde(text = true)]
    #[validate(custom = DdexIsoDateValidator::json_validate)]
    pub content: String,
    #[yaserde(attribute = true, rename = "IsApproximate")]
    pub is_approximate: Option<bool>,
    #[yaserde(attribute = true, rename = "IsBefore")]
    pub is_before: Option<bool>,
    #[yaserde(attribute = true, rename = "IsAfter")]
    pub is_after: Option<bool>,
    #[yaserde(attribute = true, rename = "ApplicableTerritoryCode")]
    #[validate(custom = AvsAllTerritoryCodeValidator::json_validate)]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute = true, rename = "LocationDescription")]
    pub location_description: Option<String>,
    #[yaserde(attribute = true, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct RightsType {
    #[yaserde(text = true)]
    #[validate(custom = AvsRightsCoverageValidator::json_validate)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct PartyNameWithTerritory {
    #[yaserde(rename = "FullName")]
    pub full_name: Name,
    #[yaserde(rename = "FullNameIndexed")]
    pub full_name_indexed: Option<Name>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct Name {
    #[yaserde(text = true)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct DetailedPartyId {
    #[yaserde(rename = "ISNI")]
    pub isni: Option<String>,
    #[yaserde(rename = "DPID")]
    #[validate(custom = DPIDValidator::json_validate)]
    pub dpid: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ResourceList {
    #[yaserde(rename = "SoundRecording")]
    #[validate]
    #[validate(min_items = 1)]
    #[validate(custom = ProtocolValidator::sound_recordings)]
    pub sound_recordings: Vec<SoundRecording>,
    #[yaserde(rename = "Image")]
    #[validate]
    pub images: Vec<Image>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct SoundRecording {
    #[yaserde(rename = "ResourceReference")]
    #[validate(custom = ResourceReferenceValidator::json_validate)]
    pub resource_reference: String,
    #[yaserde(rename = "Type")]
    #[validate]
    pub kind: SoundRecordingType,
    #[yaserde(rename = "SoundRecordingEdition")]
    #[validate]
    #[validate(min_items = 1)]
    #[validate(max_items = 2)]
    pub sound_recording_editions: Vec<SoundRecordingEdition>,
    #[yaserde(rename = "RecordingFormat")]
    #[validate]
    pub recording_formats: Vec<RecordingFormat>,
    #[yaserde(rename = "WorkId")]
    pub work_ids: Vec<MusicalWorkId>,
    #[yaserde(rename = "DisplayTitleText")]
    #[validate(min_items = 1)]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "VersionType")]
    #[validate]
    pub version_types: Vec<VersionType>,
    #[yaserde(rename = "DisplayArtistName")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "Contributor")]
    #[validate]
    pub contributors: Vec<Contributor>,
    #[yaserde(rename = "Character")]
    #[validate]
    pub characters: Vec<Character>,
    #[yaserde(rename = "ResourceRightsController")]
    #[validate]
    pub resource_rights_controllers: Vec<ResourceRightsController>,
    #[yaserde(rename = "Duration")]
    pub duration: String,
    #[yaserde(rename = "CreationDate")]
    #[validate]
    pub creation_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "ParentalWarningType")]
    #[validate]
    #[validate(min_items = 1)]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "IsInstrumental")]
    pub is_instrumental: Option<bool>,
    #[yaserde(rename = "LanguageOfPerformance")]
    #[validate]
    pub language_of_performances: Vec<Language>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct SoundRecordingType {
    #[yaserde(text = true)]
    #[validate(custom = AvsSoundRecordingTypeValidator::json_validate)]
    pub content: String,
    #[yaserde(attribute = true, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute = true, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct SoundRecordingEdition {
    #[yaserde(rename = "Type")]
    #[validate(custom = AvsEditionTypeValidator::json_validate)]
    pub kind: Option<String>,
    #[yaserde(rename = "ResourceId")]
    #[validate]
    #[validate(min_items = 1)]
    #[validate(max_items = 1)]
    pub resource_ids: Vec<SoundRecordingId>,
    #[yaserde(rename = "EditionContributor")]
    #[validate]
    pub edition_contributors: Vec<EditionContributor>,
    #[yaserde(rename = "PLine")]
    pub p_lines: Vec<PLineWithDefault>,
    #[yaserde(rename = "RecordingMode")]
    #[validate(custom = AvsRecordingModeValidator::json_validate)]
    pub recording_mode: Option<String>,
    #[yaserde(rename = "TechnicalDetails")]
    #[validate]
    pub technical_details: Vec<TechnicalSoundRecordingDetails>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct SoundRecordingId {
    #[yaserde(rename = "ISRC")]
    #[validate(custom = ISRCValidator::json_validate)]
    pub isrc: String,
    #[yaserde(rename = "CatalogNumber")]
    pub catalog_number: Option<CatalogNumber>,
    #[yaserde(rename = "ProprietaryId")]
    pub proprietary_ids: Vec<ProprietaryId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct CatalogNumber {
    #[yaserde(text = true)]
    pub content: String,
    #[yaserde(attribute = true, rename = "Namespace")]
    pub namespace: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ProprietaryId {
    #[yaserde(text = true)]
    pub content: String,
    #[yaserde(attribute = true, rename = "Namespace")]
    pub namespace: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct EditionContributor {
    #[yaserde(rename = "ContributorPartyReference")]
    #[validate(custom = ContributorPartyReferenceValidator::json_validate)]
    pub contributor_party_reference: String,
    #[yaserde(rename = "Role")]
    #[validate]
    pub roles: Vec<ContributorRole>,
    #[yaserde(rename = "HasMadeFeaturedContribution")]
    pub has_made_featured_contribution: Option<bool>,
    #[yaserde(rename = "HasMadeContractedContribution")]
    pub has_made_contracted_contribution: Option<bool>,
    #[yaserde(rename = "IsCredited")]
    pub is_credited: Option<IsCredited>,
    #[yaserde(rename = "DisplayCredits")]
    #[validate]
    pub display_credits: Vec<DisplayCredits>,
    #[yaserde(attribute = true, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ContributorRole {
    #[yaserde(text = true)]
    #[validate(custom = AvsContributorRoleValidator::json_validate)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct IsCredited {
    pub content: bool,
    #[yaserde(attribute = true, rename = "MayBeShared")]
    pub may_be_shared: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct DisplayCredits {
    #[yaserde(rename = "DisplayCreditText")]
    pub display_credit_text: String,
    #[yaserde(rename = "DisplayCreditParty")]
    #[validate(custom = DisplayCreditPartyValidator::json_validate_vec)]
    pub display_credit_parties: Vec<String>,
    #[yaserde(rename = "NameUsedInDisplayCredit")]
    pub names_used_in_display_credits: Vec<String>,
    #[yaserde(attribute = true, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute = true, rename = "ApplicableTerritoryCode")]
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate)]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute = true, rename = "IsDefault")]
    pub is_default: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct PLineWithDefault {
    #[yaserde(rename = "Year")]
    pub year: Option<u16>,
    #[yaserde(rename = "PLineText")]
    pub p_line_text: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct TechnicalSoundRecordingDetails {
    #[yaserde(rename = "TechnicalResourceDetailsReference")]
    #[validate(custom = TechnicalResourceDetailsReferenceValidator::json_validate)]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "DeliveryFile")]
    #[validate]
    pub delivery_files: Vec<AudioDeliveryFile>,
    #[yaserde(attribute = true, rename = "ApplicableTerritoryCode")]
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate)]
    pub applicable_territory_code: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct AudioDeliveryFile {
    #[yaserde(rename = "Type")]
    #[validate(custom = TypeValidator::json_validate)]
    pub kind: String,
    #[yaserde(rename = "File")]
    pub file: Option<File>,
    #[yaserde(rename = "Fingerprint")]
    pub fingerprints: Vec<Fingerprint>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct File {
    #[yaserde(rename = "URI")]
    pub uri: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct Fingerprint {
    #[yaserde(rename = "Algorithm")]
    #[validate]
    pub algorithm: FingerprintAlgorithmType,
    #[yaserde(rename = "Version")]
    pub version: Option<String>,
    #[yaserde(rename = "Parameter")]
    pub parameter: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct FingerprintAlgorithmType {
    #[yaserde(text = true)]
    #[validate(custom = AvsFingerprintAlgorithmTypeValidator::json_validate)]
    pub content: String,
    #[yaserde(attribute = true, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute = true, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct RecordingFormat {
    #[yaserde(text = true)]
    #[validate(custom = AvsRecordingFormatValidator::json_validate)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct MusicalWorkId {
    #[yaserde(rename = "ISWC")]
    pub iswc: Option<String>,
    #[yaserde(attribute = true, rename = "IsReplaced")]
    pub is_replaced: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct DisplayTitleText {
    #[yaserde(text = true)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct DisplayTitle {
    #[yaserde(rename = "TitleText")]
    pub title_text: String,
    #[yaserde(rename = "SubTitle")]
    pub sub_titles: Vec<DisplaySubTitle>,
    #[yaserde(attribute = true, rename = "ApplicableTerritoryCode")]
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate)]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute = true, rename = "IsDefault")]
    pub is_default: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct DisplaySubTitle {
    #[yaserde(text = true)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct VersionType {
    #[yaserde(text = true)]
    #[validate(custom = AvsVersionTypeValidator::json_validate)]
    pub content: String,
    #[yaserde(attribute = true, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute = true, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct DisplayArtistNameWithDefault {
    #[yaserde(text = true)]
    pub content: String,
    #[yaserde(attribute = true, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(attribute = true, rename = "ApplicableTerritoryCode")]
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate)]
    pub applicable_territory_code: Option<String>,
    #[yaserde(attribute = true, rename = "IsDefault")]
    pub is_default: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct DisplayArtist {
    #[yaserde(rename = "ArtistPartyReference")]
    #[validate(custom = ArtistPartyReferenceValidator::json_validate)]
    pub artist_party_reference: String,
    #[yaserde(rename = "DisplayArtistRole")]
    #[validate]
    pub display_artist_role: DisplayArtistRole,
    #[yaserde(rename = "ArtisticRole")]
    #[validate]
    pub artistic_roles: Vec<ContributorRole>,
    #[yaserde(rename = "TitleDisplayInformation")]
    pub title_display_informations: Vec<TitleDisplayInformation>,
    #[yaserde(attribute = true, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct DisplayArtistRole {
    #[yaserde(text = true)]
    #[validate(custom = AvsDisplayArtistRoleValidator::json_validate)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct TitleDisplayInformation {
    #[yaserde(rename = "IsDisplayedInTitle")]
    pub is_displayed_in_title: bool,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct Contributor {
    #[yaserde(rename = "ContributorPartyReference")]
    #[validate(custom = ContributorPartyReferenceValidator::json_validate)]
    pub contributor_party_reference: String,
    #[yaserde(rename = "Role")]
    #[validate]
    pub roles: Vec<ContributorRole>,
    #[yaserde(attribute = true, rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct Character {
    #[yaserde(rename = "CharacterPartyReference")]
    #[validate(custom = CharacterPartyReferenceValidator::json_validate)]
    pub character_party_reference: String,
    #[yaserde(rename = "Performer")]
    #[validate]
    pub performer: Option<Contributor>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[validate(custom = |s| ChoiceValidator::resource_rights_controller(&s.right_share_percentage, &s.right_share_unknown))]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ResourceRightsController {
    #[yaserde(rename = "RightsControllerPartyReference")]
    #[validate(custom = RightsControllerPartyReferenceValidator::json_validate)]
    pub rights_controller_party_reference: String,
    #[yaserde(rename = "RightsControlType")]
    #[validate(custom = AvsRightsControllerRoleValidator::json_validate_vec)]
    #[validate(custom = ProtocolValidator::rights_control_types)]
    #[validate(min_items = 1)]
    pub rights_control_types: Vec<String>,
    #[yaserde(rename = "RightSharePercentage")]
    pub right_share_percentage: Option<String>,
    #[yaserde(rename = "RightShareUnknown")]
    pub right_share_unknown: Option<bool>,
    #[yaserde(rename = "DelegatedUsageRights")]
    pub delegated_usage_rights: Vec<DelegatedUsageRights>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct UseType {
    #[yaserde(text = true)]
    #[validate(custom = AvsUseTypeERNValidator::json_validate)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct DelegatedUsageRights {
    #[yaserde(rename = "UseType")]
    #[validate]
    #[validate(min_items = 1)]
    #[validate(custom = ProtocolValidator::use_types)]
    pub use_types: Vec<UseType>,
    #[yaserde(rename = "TerritoryOfRightsDelegation")]
    #[validate]
    #[validate(min_items = 1)]
    pub territory_of_rights_delegations: Vec<AllTerritoryCode>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct AllTerritoryCode {
    #[yaserde(text = true)]
    #[validate(custom = AvsAllTerritoryCodeValidator::json_validate)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct EventDateWithoutFlags {
    #[yaserde(text = true)]
    #[validate(custom = DdexIsoDateValidator::json_validate)]
    pub content: String,
    #[yaserde(attribute = true, rename = "IsApproximate")]
    pub is_approximate: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ParentalWarningTypeWithTerritory {
    #[yaserde(text = true)]
    #[validate(custom = AvsParentalWarningTypeValidator::json_validate)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct Language {
    #[yaserde(text = true)]
    #[validate(custom = DdexLanguageAndScriptCodeWithRestrictionValidator::json_validate)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct Image {
    #[yaserde(rename = "ResourceReference")]
    #[validate(custom = ResourceReferenceValidator::json_validate)]
    pub resource_reference: String,
    #[yaserde(rename = "Type")]
    #[validate]
    pub kind: ImageType,
    #[yaserde(rename = "ResourceId")]
    #[validate(min_items = 1)]
    pub resource_ids: Vec<ResourceProprietaryId>,
    #[yaserde(rename = "ParentalWarningType")]
    #[validate]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "TechnicalDetails")]
    #[validate]
    pub technical_details: Vec<TechnicalImageDetails>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ImageType {
    #[yaserde(text = true)]
    #[validate(custom = AvsImageTypeValidator::json_validate)]
    pub content: String,
    #[yaserde(attribute = true, rename = "Namespace")]
    pub namespace: Option<String>,
    #[yaserde(attribute = true, rename = "UserDefinedValue")]
    pub user_defined_value: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ResourceProprietaryId {
    #[yaserde(rename = "ProprietaryId")]
    #[validate(min_items = 1)]
    pub proprietary_ids: Vec<ProprietaryId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct TechnicalImageDetails {
    #[yaserde(rename = "TechnicalResourceDetailsReference")]
    #[validate(custom = TechnicalResourceDetailsReferenceValidator::json_validate)]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "File")]
    pub file: Option<File>,
    #[yaserde(rename = "Fingerprint")]
    pub fingerprints: Vec<Fingerprint>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ReleaseList {
    #[yaserde(rename = "Release")]
    #[validate]
    pub release: Release,
    #[yaserde(rename = "TrackRelease")]
    #[validate]
    pub track_releases: Vec<TrackRelease>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct Release {
    #[yaserde(rename = "ReleaseReference")]
    #[validate(custom = ReleaseReferenceValidator::json_validate)]
    pub release_reference: String,
    #[yaserde(rename = "ReleaseType")]
    #[validate]
    #[validate(min_items = 1)]
    pub release_types: Vec<ReleaseTypeForReleaseNotification>,
    #[yaserde(rename = "ReleaseId")]
    #[validate]
    #[validate(custom = ProtocolValidator::release_id)]
    pub release_id: ReleaseId,
    #[yaserde(rename = "DisplayTitleText")]
    #[validate(min_items = 1)]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "DisplayArtistName")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "ReleaseLabelReference")]
    #[validate]
    #[validate(min_items = 1)]
    pub release_label_references: Vec<ReleaseLabelReferenceWithParty>,
    #[yaserde(rename = "Duration")]
    pub duration: Option<String>,
    #[yaserde(rename = "Genre")]
    #[validate(min_items = 1)]
    pub genres: Vec<GenreWithTerritory>,
    #[yaserde(rename = "ParentalWarningType")]
    #[validate]
    #[validate(min_items = 1)]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "ResourceGroup")]
    pub resource_group: ResourceGroup,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ReleaseTypeForReleaseNotification {
    #[yaserde(text = true)]
    #[validate(custom = AvsReleaseTypeERN4Validator::json_validate)]
    pub content: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ReleaseId {
    #[yaserde(rename = "GRid")]
    pub g_rid: Option<String>,
    #[yaserde(rename = "ICPN")]
    pub icpn: Option<String>,
    #[yaserde(rename = "ProprietaryId")]
    pub proprietary_ids: Vec<ProprietaryId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ReleaseLabelReferenceWithParty {
    #[yaserde(text = true)]
    #[validate(custom = DdexLocalPartyAnchorReferenceValidator::json_validate)]
    pub content: String,
    #[yaserde(attribute = true, rename = "ApplicableTerritoryCode")]
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate)]
    pub applicable_territory_code: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ResourceGroup {
    #[yaserde(rename = "DisplayTitleText")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle")]
    #[validate]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
    #[yaserde(rename = "DisplayArtist")]
    #[validate]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "ResourceGroupContentItem")]
    #[validate]
    pub resource_group_content_items: Vec<ResourceGroupContentItem>,
    #[yaserde(rename = "NoDisplaySequence")]
    pub no_display_sequence: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct ResourceGroupContentItem {
    #[yaserde(rename = "SequenceNumber")]
    pub sequence_number: Option<i32>,
    #[yaserde(rename = "ReleaseResourceReference")]
    #[validate(custom = ReleaseResourceReferenceValidator::json_validate)]
    pub release_resource_reference: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct GenreWithTerritory {
    #[yaserde(rename = "GenreText")]
    pub genre_text: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde::YaDeserialize,
    yaserde::YaSerialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespaces = {"ern" = "http://ddex.net/xml/ern/43"})]
pub struct TrackRelease {
    #[yaserde(rename = "ReleaseReference")]
    #[validate(custom = ReleaseReferenceValidator::json_validate)]
    pub release_reference: String,
    #[yaserde(rename = "ReleaseId")]
    pub release_id: ReleaseId,
    #[yaserde(rename = "ReleaseResourceReference")]
    #[validate(custom = ReleaseResourceReferenceValidator::json_validate)]
    pub release_resource_reference: String,
    #[yaserde(rename = "ReleaseLabelReference")]
    #[validate]
    #[validate(min_items = 1)]
    pub release_label_references: Vec<ReleaseLabelReferenceWithParty>,
    #[yaserde(rename = "Genre")]
    #[validate(min_items = 1)]
    pub genres: Vec<GenreWithTerritory>,
}
