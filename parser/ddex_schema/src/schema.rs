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
pub struct NewReleaseMessage {
    #[yaserde(rename = "MessageHeader", prefix = "ern")]
    pub message_header: MessageHeader, //done
    #[yaserde(rename = "ReleaseAdmin", prefix = "ern")]
    pub release_admins: Vec<ReleaseAdmin>, //done
    #[yaserde(rename = "PartyList", prefix = "ern")]
    pub party_list: PartyList, //done
    #[yaserde(rename = "ResourceList", prefix = "ern")]
    pub resource_list: ResourceList, // done
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
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct MessageHeader {
    #[yaserde(rename = "MessageId", prefix = "ern")]
    pub message_id: String,
    #[yaserde(rename = "MessageSender", prefix = "ern")]
    pub message_sender: MessagingPartyWithoutCode,
    #[yaserde(rename = "MessageRecipient", prefix = "ern")]
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
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct MessagingPartyWithoutCode {
    #[yaserde(rename = "PartyId", prefix = "ern", validation = "PartyIdValidator")]
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
    yaserde_derive::YaSerialize,
    yaserde_derive::YaDeserialize,
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
    yaserde_derive::YaSerialize,
    yaserde_derive::YaDeserialize,
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
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct RightsType {
    #[yaserde(text, validation = "AvsRightsCoverageValidator")]
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
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct DetailedPartyId {
    #[yaserde(rename = "ISNI", prefix = "ern")]
    pub isni: Option<String>,
    #[yaserde(rename = "DPID", prefix = "ern", validation = "DPIDValidator")]
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
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceList {
    #[yaserde(rename = "SoundRecording", prefix = "ern")]
    pub sound_recordings: Vec<SoundRecording>, // done
    #[yaserde(rename = "Image", prefix = "ern")]
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
    pub sound_recording_editions: Vec<SoundRecordingEdition>, //done
    #[yaserde(rename = "RecordingFormat", prefix = "ern")]
    pub recording_formats: Vec<RecordingFormat>,
    #[yaserde(rename = "WorkId", prefix = "ern")]
    pub work_ids: Vec<MusicalWorkId>, //done
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
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
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: String,
    #[yaserde(rename = "CreationDate", prefix = "ern")]
    pub creation_date: Option<EventDateWithoutFlags>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "IsInstrumental", prefix = "ern")]
    pub is_instrumental: Option<bool>,
    #[yaserde(rename = "LanguageOfPerformance", prefix = "ern")]
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
pub struct SoundRecordingId {
    #[yaserde(rename = "ISRC", prefix = "ern")]
    pub isrc: Option<String>,
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
pub struct ContributorRole {
    #[yaserde(text, validation = "AvsContributorRoleValidator")]
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
pub struct AudioDeliveryFile {
    #[yaserde(rename = "Type", prefix = "ern", validation = "TypeValidator")]
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
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct Fingerprint {
    #[yaserde(rename = "Algorithm", prefix = "ern")]
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
pub struct RecordingFormat {
    #[yaserde(text, validation = "AvsRecordingFormatValidator")]
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
pub struct DisplayArtistRole {
    #[yaserde(text, validation = "AvsDisplayArtistRoleValidator")]
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
pub struct Character {
    #[yaserde(
        rename = "CharacterPartyReference",
        prefix = "ern",
        validation = "CharacterPartyReferenceValidator"
    )]
    pub character_party_reference: String,
    #[yaserde(rename = "Performer", prefix = "ern")]
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
pub struct AllTerritoryCode {
    #[yaserde(text, validation = "AvsAllTerritoryCodeValidator")]
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
pub struct EventDateWithoutFlags {
    #[yaserde(text, validation = "DdexIsoDateValidator")]
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
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ParentalWarningTypeWithTerritory {
    #[yaserde(text, validation = "AvsParentalWarningTypeValidator")]
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
pub struct Language {
    #[yaserde(text, validation = "DdexLanguageAndScriptCodeWithRestrictionValidator")]
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
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
    pub parental_warning_types: Vec<ParentalWarningTypeWithTerritory>,
    #[yaserde(rename = "TechnicalDetails", prefix = "ern")]
    pub technical_detailss: Vec<TechnicalImageDetails>,
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
pub struct ResourceProprietaryId {
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
pub struct TechnicalImageDetails {
    #[yaserde(
        rename = "TechnicalResourceDetailsReference",
        prefix = "ern",
        validation = "TechnicalResourceDetailsReferenceValidator"
    )]
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
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseList {
    #[yaserde(rename = "Release", prefix = "ern")]
    pub release: Option<Release>,
    #[yaserde(rename = "TrackRelease", prefix = "ern")]
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
    #[yaserde(rename = "DisplayArtistName", prefix = "ern")]
    pub display_artist_names: Vec<DisplayArtistNameWithDefault>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "ReleaseLabelReference", prefix = "ern")]
    pub release_label_references: Vec<ReleaseLabelReferenceWithParty>,
    #[yaserde(rename = "Duration", prefix = "ern")]
    pub duration: Option<String>,
    #[yaserde(rename = "Genre", prefix = "ern")]
    pub genres: Vec<GenreWithTerritory>,
    #[yaserde(rename = "ParentalWarningType", prefix = "ern")]
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
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseTypeForReleaseNotification {
    #[yaserde(text, validation = "AvsReleaseTypeERN4Validator")]
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
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ReleaseLabelReferenceWithParty {
    #[yaserde(text, validation = "DdexLocalPartyAnchorReferenceValidator")]
    pub content: String,
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
    yaserde_derive::YaSerialize,
    yaserde_derive::YaDeserialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
pub struct ResourceGroup {
    #[yaserde(rename = "DisplayTitleText", prefix = "ern")]
    pub display_title_texts: Vec<DisplayTitleText>,
    #[yaserde(rename = "DisplayTitle", prefix = "ern")]
    pub display_titles: Vec<DisplayTitle>,
    #[yaserde(rename = "SequenceNumber", prefix = "ern")]
    pub sequence_number: Option<i32>,
    #[yaserde(rename = "DisplayArtist", prefix = "ern")]
    pub display_artists: Vec<DisplayArtist>,
    #[yaserde(rename = "ResourceGroupContentItem", prefix = "ern")]
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
}
