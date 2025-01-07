struct ProtocolSchema {
    pub release: Option<Release>,
    pub sound_recordings: Vec<SoundRecording>, // can be empty
    // Pair track release with sound recording by ResourceReference
    pub track_releases: Vec<TrackRelease>, // can be empty
}

struct Release {
    // release_list::release::display_titles[0]::title_text - only first element
    pub title_text: String,
    // release_list::release::display_titles[0]::sub_titles[0]::content - only first element
    pub subtitle: Option<String>,
    // release_list::release::display_title_texts[0]::content - only first element
    pub display_title_text: String,
    // release_list::release::release_types[x]::content - all items
    pub release_types: Vec<String>, // Min 1
    // release_list::release::display_artist_names[x] - all items
    pub display_artist_names: Vec<DisplayArtistName>, // Min 1
    // release_list::release::release_id
    pub release_id: ReleaseId, // Required but its content can be empty...
    // resource_list::sound_recordings[0]::contributors[x] -> !!SHOULD BE MOVED TO SOUND RECORDING
    pub contributors: Vec<Contributor>, // can be empty
    // resource_list::sound_recordings[0]::display_artists[x] -> !!SHOULD BE MOVED TO SOUND RECORDING
    pub display_artists: Vec<DisplayArtist>, // Min 1
}

struct DisplayArtistName {
    pub applicable_territory_code: Option<String>,
    pub language_and_script_type: Option<String>,
    pub display_artist_name: String,
}

struct ReleaseId {
    pub grid: Option<String>,
    pub icpn: Option<String>,
    pub proprietary_ids: Vec<String>, // can be empty
}

struct Contributor {
    pub contributor_party_reference: String,
    pub sequence_number: Option<String>,
    pub contributor_roles: Vec<String>, // can be empty
}

struct DisplayArtist {
    pub artist_party_reference: String,
    pub sequence_number: Option<String>,
    pub display_artist_roles: Vec<String>, // can be empty
}

struct SoundRecording {
    // resource_list::sound_recordings[x]::display_titles[0]::title_text - first item only
    pub display_title: String,
    // resource_list::sound_recordings[x]::display_titles[0]::sub_titles[0]::content - first item only
    pub subtitle: Option<String>,
    // resource_list::sound_recordings[x]::display_title_texts[0]::content - first item only
    pub display_title_text: String,
    pub sound_recording_editions: Vec<SoundRecordingEdition>, // Min 1, Max 2
    // resource_list::sound_recordings[x]::work_ids[x]
    pub work_ids: Vec<WorkId>, // can be empty
}

struct SoundRecordingEdition {
    // resource_list::sound_recordings[x]::sound_recording_editions[x]::resource_ids[x]::isrc
    pub isrc: String, // In theory it can edition can have multiple isrcs but due to our validation each edition can have only one isrc,
    // and since there are only 2 types of editions, sound recording can have 2 editions which gives 2 isrcs per sound recording
    // resource_list::sound_recordings[x]::sound_recording_editions[x]::p_lines[x] - all items
    pub p_lines: Vec<PLine>, // can be empty
}

struct PLine {
    pub year: Optional<String>,
    pub p_line_text: String,
}

struct WorkId {
    pub iswc: Optional<String>,
}

struct TrackRelease {
    pub display_title: String,
    pub subtitle: String,
    pub display_title_text: String,
}
