#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubgraphSchema {
    pub release: Release,
    pub sound_recordings: Vec<SoundRecording>, // Min 1
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Release {
    // release_list::release::display_titles[0]::title_text - only first element
    pub title_text: String,
    // release_list::release::display_titles[0]::sub_titles[0]::content - only first element
    pub subtitle: Option<String>,
    // release_list::release::display_title_texts[0]::content - only first element
    pub display_title_text: String,
    // release_list::release::release_types[x]::content - all items
    pub release_types: Vec<String>, // Min 1
    // release_list::release::display_artists[x] - all items
    pub display_artists: Vec<DisplayArtist>, // Min 1
    // release_list::release::display_artist_names[x] - all items
    pub display_artist_names: Vec<DisplayArtistName>, // Min 1
    // release_list::release::release_id
    pub release_id: ReleaseId,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisplayArtistName {
    pub applicable_territory_code: Option<String>,
    pub language_and_script_type: Option<String>,
    pub display_artist_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReleaseId {
    pub grid: Option<String>,
    pub icpn: Option<String>,
    pub proprietary_ids: Vec<String>, // can be empty
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Contributor {
    pub contributor_party_reference: String,
    pub sequence_number: Option<i32>,
    pub contributor_roles: Vec<String>, // can be empty
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisplayArtist {
    pub artist_party_reference: String,
    pub sequence_number: Option<i32>,
    pub display_artist_roles: Vec<String>, // can be empty
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SoundRecording {
    // resource_list::sound_recordings[x]::display_titles[0]::title_text - first item only
    pub display_title: String,
    // resource_list::sound_recordings[x]::display_titles[0]::sub_titles[0]::content - first item only
    pub subtitle: Option<String>,
    // resource_list::sound_recordings[x]::display_title_texts[0]::content - first item only
    pub display_title_text: String,
    pub sound_recording_editions: Vec<SoundRecordingEdition>, // Min 1, Max 2
    // resource_list::sound_recordings[x]::work_ids[x]
    pub work_ids: Vec<WorkId>, // can be empty
    // resource_list::sound_recordings[0]::contributors[x]
    pub contributors: Vec<Contributor>, // can be empty
    // resource_list::sound_recordings[x]::resource_reference -> release_list::track_releases[y]::release_id
    pub track_release_id: Option<ReleaseId>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SoundRecordingEdition {
    // resource_list::sound_recordings[x]::sound_recording_editions[x]::resource_ids[x]::isrc
    pub isrc: String, // In theory it can edition can have multiple isrcs but due to our validation each edition can have only one isrc,
    // and since there are only 2 types of editions, sound recording can have 2 editions which gives 2 isrcs per sound recording
    // resource_list::sound_recordings[x]::sound_recording_editions[x]::p_lines[x] - all items
    pub p_lines: Vec<PLine>,            // can be empty
    pub fingerprints: Vec<Fingerprint>, // can be empty
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fingerprint {
    pub namespace: Option<String>,
    pub user_defined_value: Option<String>,
    pub parameter: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PLine {
    pub year: Option<u16>,
    pub p_line_text: String,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkId {
    pub iswc: Option<String>,
}
