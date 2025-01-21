// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct ProvedMessage {
    Release release;
    SoundRecording[] sound_recordings;
}
struct Release {
    string title_text;
    string subtitle;
    string display_title_text;
    string[] release_types;
    DisplayArtist[] display_artists;
    DisplayArtistName[] display_artist_names;
    ReleaseId release_id;
}

struct DisplayArtist {
    string artist_party_reference;
    int32 sequence_number;
    string[] display_artist_roles;
}

struct ReleaseId {
    string grid;
    string icpn;
    string[] proprietary_ids;
}

struct DisplayArtistName {
    string applicable_territory_code;
    string language_and_script_type;
    string display_artist_name;
}

struct SoundRecording {
    string display_title;
    string subtitle;
    string display_title_text;
    SoundRecordingEdition[] sound_recording_editions;
    WorkId[] work_ids;
    Contributor[] contributors;
    ReleaseId track_release_id;
}

struct SoundRecordingEdition {
    string isrc;
    PLine[] p_lines;
    Fingerprint[] fingerprints;
}
struct PLine {
    uint16 year;
    string p_line_text;
}

struct Fingerprint {
    string namespace;
    string user_defined_value;
    string parameter;
}

struct WorkId {
    string iswc;
}

struct Contributor {
    string contributor_party_reference;
    int32 sequence_number;
    string[] contributor_roles;
}

struct ProverPublicOutputs {
    bool is_valid;
    bytes32 digest;
    ProvedMessage[] messages;
}
