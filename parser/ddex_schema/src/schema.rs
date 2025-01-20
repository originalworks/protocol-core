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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]

// Check 7 disabled until revelator starts sending messages that meet protocol requirements
// #[validate(custom = |s| ProtocolValidator::music_licensing_companies(&s.resource_list, &s.party_list))]

pub struct NewReleaseMessage {
    #[yaserde(rename = "MessageHeader", prefix = "ern")]
    #[validate]
    pub message_header: MessageHeader,
    #[yaserde(rename = "ReleaseAdmin", prefix = "ern")]
    pub release_admins: Vec<ReleaseAdmin>,
    #[yaserde(rename = "PartyList", prefix = "ern")]
    pub party_list: PartyList,
    #[yaserde(rename = "ResourceList", prefix = "ern")]
    pub resource_list: ResourceList,
    #[yaserde(rename = "ReleaseList", prefix = "ern")]
    pub release_list: ReleaseList,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct MessageHeader {
    #[yaserde(rename = "MessageId", prefix = "ern")]
    pub message_id: String,
    #[yaserde(rename = "MessageSender", prefix = "ern")]
    #[validate]
    pub message_sender: MessagingPartyWithoutCode,
    #[yaserde(rename = "MessageRecipient", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub message_recipients: Vec<MessagingPartyWithoutCode>,
    #[yaserde(rename = "MessageCreatedDateTime", prefix = "ern")]
    pub message_created_date_time: String,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct MessagingPartyWithoutCode {
    #[yaserde(rename = "PartyId", prefix = "ern", validation = "PartyIdValidator")]
    #[validate(custom = PartyIdValidator::json_validate)]
    pub party_id: String,
    #[yaserde(rename = "PartyName", prefix = "ern")]
    pub party_name: Option<PartyNameWithoutCode>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct PartyList {
    #[yaserde(rename = "Party", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub partys: Vec<Party>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    yaserde_derive::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
#[validate(custom = |s| ChoiceValidator::party(&s.parties_names, &s.parties_ids))]
pub struct Party {
    #[yaserde(
        rename = "PartyReference",
        prefix = "ern",
        validation = "PartyReferenceValidator"
    )]
    #[validate(custom = PartyReferenceValidator::json_validate)]
    pub party_reference: String,
    #[yaserde(rename = "Affiliation", prefix = "ern")]
    #[validate]
    pub affiliations: Vec<Affiliation>,
    #[yaserde(rename = "PartyName", prefix = "ern")]
    pub parties_names: Vec<PartyNameWithTerritory>,
    #[yaserde(rename = "PartyId", prefix = "ern")]
    #[validate]
    pub parties_ids: Vec<DetailedPartyId>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    yaserde_derive::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[validate(custom = |s| ChoiceValidator::affiliation(&s.company_name, &s.party_affiliate_reference))]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Affiliation {
    #[yaserde(
        rename = "Type",
        prefix = "ern",
        validation = "AvsAffiliationTypeValidator"
    )]
    #[validate(custom = AvsAffiliationTypeValidator::json_validate)]
    #[validate(custom = ProtocolValidator::affiliation_type)]
    pub kind: String,
    #[yaserde(rename = "ValidityPeriod", prefix = "ern")]
    pub validity_period: Option<ValidityPeriod>,
    #[yaserde(rename = "RightsType", prefix = "ern")]
    #[validate]
    #[validate(custom = ProtocolValidator::right_types)]
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
    #[validate(custom = PartyAffiliateReferenceValidator::json_validate)]
    pub party_affiliate_reference: Option<String>,
    #[yaserde(
        rename = "TerritoryCode",
        prefix = "ern",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate_vec)]
    #[validate(min_items = 1)]
    pub territory_codes: Vec<String>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ValidityPeriod {
    #[yaserde(rename = "StartDate", prefix = "ern")]
    #[validate]
    pub start_date: Option<EventDate>,
    #[yaserde(rename = "EndDate", prefix = "ern")]
    #[validate]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDate {
    #[yaserde(text, validation = "DdexIsoDateValidator")]
    #[validate(custom = DdexIsoDateValidator::json_validate)]
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
    #[validate(custom = AvsAllTerritoryCodeValidator::json_validate)]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct RightsType {
    #[yaserde(text, validation = "AvsRightsCoverageValidator")]
    #[validate(custom = AvsRightsCoverageValidator::json_validate)]
    pub content: String,
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
    #[yaserde(rename = "FullNameIndexed", prefix = "ern")]
    pub full_name_indexed: Option<Name>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DetailedPartyId {
    #[yaserde(rename = "ISNI", prefix = "ern")]
    pub isni: Option<String>,
    #[yaserde(rename = "DPID", prefix = "ern", validation = "DPIDValidator")]
    #[validate(custom = DPIDValidator::json_validate)]
    pub dpid: Option<String>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceList {
    #[yaserde(rename = "SoundRecording", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    #[validate(custom = ProtocolValidator::sound_recordings)]
    pub sound_recordings: Vec<SoundRecording>,
    #[yaserde(rename = "Image", prefix = "ern")]
    #[validate]
    pub images: Vec<Image>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SoundRecording {
    #[yaserde(
        rename = "ResourceReference",
        prefix = "ern",
        validation = "ResourceReferenceValidator"
    )]
    #[validate(custom = ResourceReferenceValidator::json_validate)]
    pub resource_reference: String,
    #[yaserde(rename = "Type", prefix = "ern")]
    #[validate]
    pub kind: SoundRecordingType,
    #[yaserde(rename = "SoundRecordingEdition", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    #[validate(max_items = 2)]
    pub sound_recording_editions: Vec<SoundRecordingEdition>,
    #[yaserde(rename = "RecordingFormat", prefix = "ern")]
    #[validate]
    pub recording_formats: Vec<RecordingFormat>,
    #[yaserde(rename = "WorkId", prefix = "ern")]
    pub work_ids: Vec<MusicalWorkId>,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    #[validate(min_items = 1)]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "VersionType", prefix = "ern")]
    #[validate]
    pub version_types: Vec<VersionType>,
    #[yaserde(rename = "DisplayArtistName", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "Contributor", prefix = "ern")]
    #[validate]
    pub contributors: Vec<Contributor>,
    #[yaserde(rename = "Character", prefix = "ern")]
    #[validate]
    pub characters: Vec<Character>,
    #[yaserde(rename = "ResourceRightsController", prefix = "ern")]
    #[validate]
    pub resource_rights_controllers: Vec<ResourceRightsController>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: String,
    #[yaserde(rename = "CreationDate", prefix = "ern")]
    #[validate]
    pub creation_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "IsInstrumental", prefix = "ern")]
    pub is_instrumental: Option<bool>,
    #[yaserde(rename = "LanguageOfPerformance", prefix = "ern")]
    #[validate]
    pub language_of_performances: Vec<Language>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SoundRecordingType {
    #[yaserde(text, validation = "AvsSoundRecordingTypeValidator")]
    #[validate(custom = AvsSoundRecordingTypeValidator::json_validate)]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SoundRecordingEdition {
    #[yaserde(
        rename = "Type",
        prefix = "ern",
        validation = "AvsEditionTypeValidator"
    )]
    #[validate(custom = AvsEditionTypeValidator::json_validate)]
    pub kind: Option<String>,
    #[yaserde(rename = "ResourceId", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    #[validate(max_items = 1)]
    pub resource_ids: Vec<SoundRecordingId>,
    #[yaserde(rename = "EditionContributor", prefix = "ern")]
    #[validate]
    pub edition_contributors: Vec<EditionContributor>,
    #[yaserde(rename = "PLine", prefix = "ern")]
    pub p_lines: Vec<PLineWithDefault>,
    #[yaserde(
        rename = "RecordingMode",
        prefix = "ern",
        validation = "AvsRecordingModeValidator"
    )]
    #[validate(custom = AvsRecordingModeValidator::json_validate)]
    pub recording_mode: Option<String>,
    #[yaserde(rename = "TechnicalDetails", prefix = "ern")]
    #[validate]
    pub technical_details: Vec<TechnicalSoundRecordingDetails>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct SoundRecordingId {
    #[yaserde(rename = "ISRC", prefix = "ern")]
    #[validate(pattern = r"^[A-Za-z]{2}\w{3}\d{7}$")]
    pub isrc: String,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EditionContributor {
    #[yaserde(
        rename = "ContributorPartyReference",
        prefix = "ern",
        validation = "ContributorPartyReferenceValidator"
    )]
    #[validate(custom = ContributorPartyReferenceValidator::json_validate)]
    pub contributor_party_reference: String,
    #[yaserde(rename = "Role", prefix = "ern")]
    #[validate]
    pub roles: Vec<ContributorRole>,
    #[yaserde(rename = "HasMadeFeaturedContribution", prefix = "ern")]
    pub has_made_featured_contribution: Option<bool>,
    #[yaserde(rename = "HasMadeContractedContribution", prefix = "ern")]
    pub has_made_contracted_contribution: Option<bool>,
    #[yaserde(rename = "IsCredited", prefix = "ern")]
    pub is_credited: Option<IsCredited>,
    #[yaserde(rename = "DisplayCredits", prefix = "ern")]
    #[validate]
    pub display_credits: Vec<DisplayCredits>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ContributorRole {
    #[yaserde(text, validation = "AvsContributorRoleValidator")]
    #[validate(custom = AvsContributorRoleValidator::json_validate)]
    pub content: String,
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
    yaserde_derive::YaSerialize,
    yaserde_derive::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
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
    #[validate(custom = DisplayCreditPartyValidator::json_validate_vec)]
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
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate)]
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
pub struct PLineWithDefault {
    #[yaserde(rename = "Year", prefix = "ern")]
    pub year: Option<u16>,
    #[yaserde(rename = "PLineText", prefix = "ern")]
    pub p_line_text: String,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TechnicalSoundRecordingDetails {
    #[yaserde(
        rename = "TechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "TechnicalResourceDetailsReferenceValidator"
    )]
    #[validate(custom = TechnicalResourceDetailsReferenceValidator::json_validate)]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "DeliveryFile", prefix = "ern")]
    #[validate]
    pub delivery_files: Vec<AudioDeliveryFile>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate)]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct AudioDeliveryFile {
    #[yaserde(rename = "Type", prefix = "ern", validation = "TypeValidator")]
    #[validate(custom = TypeValidator::json_validate)]
    pub kind: String,
    #[yaserde(rename = "File", prefix = "ern")]
    pub file: Option<File>,
    #[yaserde(rename = "Fingerprint", prefix = "ern")]
    pub fingerprints: Vec<Fingerprint>,
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
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    yaserde_derive::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Fingerprint {
    #[yaserde(rename = "Algorithm", prefix = "ern")]
    #[validate]
    pub algorithm: FingerprintAlgorithmType,
    #[yaserde(rename = "Version", prefix = "ern")]
    pub version: Option<String>,
    #[yaserde(rename = "Parameter", prefix = "ern")]
    pub parameter: Option<String>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct FingerprintAlgorithmType {
    #[yaserde(text, validation = "AvsFingerprintAlgorithmTypeValidator")]
    #[validate(custom = AvsFingerprintAlgorithmTypeValidator::json_validate)]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct RecordingFormat {
    #[yaserde(text, validation = "AvsRecordingFormatValidator")]
    #[validate(custom = AvsRecordingFormatValidator::json_validate)]
    pub content: String,
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
pub struct DisplayTitleText {
    #[yaserde(text)]
    pub content: String,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DisplayTitle {
    #[yaserde(rename = "TitleText", prefix = "ern")]
    pub title_text: String,
    #[yaserde(rename = "SubTitle", prefix = "ern")]
    pub sub_titles: Vec<DisplaySubTitle>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate)]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct VersionType {
    #[yaserde(text, validation = "AvsVersionTypeValidator")]
    #[validate(custom = AvsVersionTypeValidator::json_validate)]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DisplayArtistNameWithDefault {
    #[yaserde(text)]
    pub content: String,
    #[yaserde(attribute, rename = "LanguageAndScriptCode")]
    pub language_and_script_code: Option<String>,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate)]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DisplayArtist {
    #[yaserde(
        rename = "ArtistPartyReference",
        prefix = "ern",
        validation = "ArtistPartyReferenceValidator"
    )]
    #[validate(custom = ArtistPartyReferenceValidator::json_validate)]
    pub artist_party_reference: String,
    #[yaserde(rename = "DisplayArtistRole", prefix = "ern")]
    #[validate]
    pub display_artist_role: DisplayArtistRole,
    #[yaserde(rename = "ArtisticRole", prefix = "ern")]
    #[validate]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DisplayArtistRole {
    #[yaserde(text, validation = "AvsDisplayArtistRoleValidator")]
    #[validate(custom = AvsDisplayArtistRoleValidator::json_validate)]
    pub content: String,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Contributor {
    #[yaserde(
        rename = "ContributorPartyReference",
        prefix = "ern",
        validation = "ContributorPartyReferenceValidator"
    )]
    #[validate(custom = ContributorPartyReferenceValidator::json_validate)]
    pub contributor_party_reference: String,
    #[yaserde(rename = "Role", prefix = "ern")]
    #[validate]
    pub roles: Vec<ContributorRole>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Character {
    #[yaserde(
        rename = "CharacterPartyReference",
        prefix = "ern",
        validation = "CharacterPartyReferenceValidator"
    )]
    #[validate(custom = CharacterPartyReferenceValidator::json_validate)]
    pub character_party_reference: String,
    #[yaserde(rename = "Performer", prefix = "ern")]
    #[validate]
    pub performer: Option<Contributor>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    yaserde_derive::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[validate(custom = |s| ChoiceValidator::resource_rights_controller(&s.right_share_percentage, &s.right_share_unknown))]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceRightsController {
    #[yaserde(
        rename = "RightsControllerPartyReference",
        prefix = "ern",
        validation = "RightsControllerPartyReferenceValidator"
    )]
    #[validate(custom = RightsControllerPartyReferenceValidator::json_validate)]
    pub rights_controller_party_reference: String,
    #[yaserde(
        rename = "RightsControlType",
        prefix = "ern",
        validation = "AvsRightsControllerRoleValidator"
    )]
    #[validate(custom = AvsRightsControllerRoleValidator::json_validate_vec)]
    #[validate(custom = ProtocolValidator::rights_control_types)]
    #[validate(min_items = 1)]
    pub rights_control_types: Vec<String>,
    #[yaserde(rename = "RightSharePercentage", prefix = "ern")]
    pub right_share_percentage: Option<String>,
    #[yaserde(rename = "RightShareUnknown", prefix = "ern")]
    pub right_share_unknown: Option<bool>,
    #[yaserde(rename = "DelegatedUsageRights", prefix = "ern")]
    pub delegated_usage_rights: Vec<DelegatedUsageRights>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct UseType {
    #[yaserde(text, validation = "AvsUseTypeERNValidator")]
    #[validate(custom = AvsUseTypeERNValidator::json_validate)]
    pub content: String,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DelegatedUsageRights {
    #[yaserde(rename = "UseType", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    #[validate(custom = ProtocolValidator::use_types)]
    pub use_types: Vec<UseType>,
    #[yaserde(rename = "TerritoryOfRightsDelegation", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct AllTerritoryCode {
    #[yaserde(text, validation = "AvsAllTerritoryCodeValidator")]
    #[validate(custom = AvsAllTerritoryCodeValidator::json_validate)]
    pub content: String,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct EventDateWithoutFlags {
    #[yaserde(text, validation = "DdexIsoDateValidator")]
    #[validate(custom = DdexIsoDateValidator::json_validate)]
    pub content: String,
    #[yaserde(attribute, rename = "IsApproximate")]
    pub is_approximate: Option<bool>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ParentalWarningTypeWithTerritory {
    #[yaserde(text, validation = "AvsParentalWarningTypeValidator")]
    #[validate(custom = AvsParentalWarningTypeValidator::json_validate)]
    pub content: String,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Language {
    #[yaserde(text, validation = "DdexLanguageAndScriptCodeWithRestrictionValidator")]
    #[validate(custom = DdexLanguageAndScriptCodeWithRestrictionValidator::json_validate)]
    pub content: String,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Image {
    #[yaserde(
        rename = "ResourceReference",
        prefix = "ern",
        validation = "ResourceReferenceValidator"
    )]
    #[validate(custom = ResourceReferenceValidator::json_validate)]
    pub resource_reference: String,
    #[yaserde(rename = "Type", prefix = "ern")]
    #[validate]
    pub kind: ImageType,
    #[yaserde(rename = "ResourceId", prefix = "ern")]
    #[validate(min_items = 1)]
    pub resource_ids: Vec<ResourceProprietaryId>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    #[validate]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "TechnicalDetails", prefix = "ern")]
    #[validate]
    pub technical_details: Vec<TechnicalImageDetails>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ImageType {
    #[yaserde(text, validation = "AvsImageTypeValidator")]
    #[validate(custom = AvsImageTypeValidator::json_validate)]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceProprietaryId {
    #[yaserde(rename = "ProprietaryId", prefix = "ern")]
    #[validate(min_items = 1)]
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TechnicalImageDetails {
    #[yaserde(
        rename = "TechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "TechnicalResourceDetailsReferenceValidator"
    )]
    #[validate(custom = TechnicalResourceDetailsReferenceValidator::json_validate)]
    pub technical_resource_details_reference: String,
    #[yaserde(rename = "File", prefix = "ern")]
    pub file: Option<File>,
    #[yaserde(rename = "Fingerprint", prefix = "ern")]
    pub fingerprints: Vec<Fingerprint>,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseList {
    #[yaserde(rename = "Release", prefix = "ern")]
    #[validate]
    pub release: Release,
    #[yaserde(rename = "TrackRelease", prefix = "ern")]
    #[validate]
    pub track_releases: Vec<TrackRelease>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    yaserde_derive::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Release {
    #[yaserde(
        rename = "ReleaseReference",
        prefix = "ern",
        validation = "ReleaseReferenceValidator"
    )]
    #[validate(custom = ReleaseReferenceValidator::json_validate)]
    pub release_reference: String,
    #[yaserde(rename = "ReleaseType", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub release_types: Vec<ReleaseTypeForReleaseNotification>,
    #[yaserde(rename = "ReleaseId", prefix = "ern")]
    #[validate(custom = ProtocolValidator::release_id)]
    pub release_id: ReleaseId,
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    #[validate(min_items = 1)]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "DisplayArtistName", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "ReleaseLabelReference", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub release_label_references: Vec<ReleaseLabelReferenceWithParty>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: Option<String>,
    #[yaserde(rename = "Genre", prefix = "ern")]
    #[validate(min_items = 1)]
    pub genres: Vec<GenreWithTerritory>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "ResourceGroup", prefix = "ern")]
    pub resource_group: ResourceGroup,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseTypeForReleaseNotification {
    #[yaserde(text, validation = "AvsReleaseTypeERN4Validator")]
    #[validate(custom = AvsReleaseTypeERN4Validator::json_validate)]
    pub content: String,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseLabelReferenceWithParty {
    #[yaserde(text, validation = "DdexLocalPartyAnchorReferenceValidator")]
    #[validate(custom = DdexLocalPartyAnchorReferenceValidator::json_validate)]
    pub content: String,
    #[yaserde(
        attribute,
        rename = "ApplicableTerritoryCode",
        validation = "AvsCurrentTerritoryCodeValidator"
    )]
    #[validate(custom = AvsCurrentTerritoryCodeValidator::json_validate)]
    pub applicable_territory_code: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    yaserde_derive::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceGroup {
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    #[validate]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "SequenceNumber", prefix = "ern")]
    pub sequence_number: Option<i32>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    #[validate]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "ResourceGroupContentItem", prefix = "ern")]
    #[validate]
    pub resource_group_content_items: Vec<ResourceGroupContentItem>,
    #[yaserde(rename = "NoDisplaySequence", prefix = "ern")]
    pub no_display_sequence: Option<bool>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive::YaSerialize,
    yaserde_derive::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
    serde_valid::Validate,
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
    #[validate(custom = ReleaseResourceReferenceValidator::json_validate)]
    pub release_resource_reference: String,
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
    serde_valid::Validate,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct TrackRelease {
    #[yaserde(
        rename = "ReleaseReference",
        prefix = "ern",
        validation = "ReleaseReferenceValidator"
    )]
    #[validate(custom = ReleaseReferenceValidator::json_validate)]
    pub release_reference: String,
    #[yaserde(rename = "ReleaseId", prefix = "ern")]
    pub release_id: ReleaseId,
    #[yaserde(
        rename = "ReleaseResourceReference",
        prefix = "ern",
        validation = "ReleaseResourceReferenceValidator"
    )]
    #[validate(custom = ReleaseResourceReferenceValidator::json_validate)]
    pub release_resource_reference: String,
    #[yaserde(rename = "ReleaseLabelReference", prefix = "ern")]
    #[validate]
    #[validate(min_items = 1)]
    pub release_label_references: Vec<ReleaseLabelReferenceWithParty>,
    #[yaserde(rename = "Genre", prefix = "ern")]
    #[validate(min_items = 1)]
    pub genres: Vec<GenreWithTerritory>,
}
