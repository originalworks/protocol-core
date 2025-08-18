use alloy_primitives::FixedBytes;
use ddex_parser::NewReleaseMessage;
use serde::{Deserialize, Serialize};

alloy_sol_types::sol!(
    #[sol(rpc, all_derives)]
    "../../contracts/contracts/interfaces/IProverPublicOutputs.sol"
);

impl ProvedMessage {
    pub fn from_ddex(message: NewReleaseMessage) -> Self {
        let ddex_release = message.release_list.release;
        let subgraph_release = Release {
            title_text: ddex_release.display_titles[0].title_text.to_owned(),
            subtitle: if let Some(s) = ddex_release.display_titles[0].sub_titles.first() {
                s.content.to_owned()
            } else {
                "".to_owned()
            },
            display_title_text: ddex_release.display_title_texts[0].content.to_owned(),
            release_id: ReleaseId {
                grid: ddex_release
                    .release_id
                    .g_rid
                    .unwrap_or_else(|| "".to_owned()),
                icpn: ddex_release
                    .release_id
                    .icpn
                    .unwrap_or_else(|| "".to_owned()),
                proprietary_ids: ddex_release
                    .release_id
                    .proprietary_ids
                    .iter()
                    .map(|p_id| p_id.content.to_owned())
                    .collect(),
            },
            release_types: ddex_release
                .release_types
                .iter()
                .map(|rt| rt.content.to_owned())
                .collect(),
            display_artists: ddex_release
                .display_artists
                .iter()
                .map(|da| DisplayArtist {
                    artist_party_reference: da.artist_party_reference.to_owned(),
                    sequence_number: da.sequence_number.unwrap_or_else(|| 0),
                    display_artist_roles: da
                        .artistic_roles
                        .iter()
                        .map(|cr| cr.content.to_owned())
                        .collect(),
                })
                .collect(),
            display_artist_names: ddex_release
                .display_artist_names
                .iter()
                .map(|da_name| DisplayArtistName {
                    display_artist_name: da_name.content.to_owned(),
                    applicable_territory_code: da_name
                        .applicable_territory_code
                        .to_owned()
                        .unwrap_or_else(|| "".to_owned()),
                    language_and_script_type: da_name
                        .language_and_script_code
                        .to_owned()
                        .unwrap_or_else(|| "".to_owned()),
                })
                .collect(),
        };

        let ddex_sound_recordings = &message.resource_list.sound_recordings;
        let ddex_track_releases = &message.release_list.track_releases;

        let subgraph_sound_recordings = ddex_sound_recordings
            .iter()
            .map(|sr| SoundRecording {
                display_title: sr.display_titles[0].title_text.to_owned(),
                display_title_text: sr.display_title_texts[0].content.to_owned(),
                subtitle: if let Some(s) = sr.display_titles[0].sub_titles.first() {
                    s.content.to_owned()
                } else {
                    "".to_owned()
                },
                work_ids: sr
                    .work_ids
                    .iter()
                    .map(|w_id| WorkId {
                        iswc: w_id.iswc.to_owned().unwrap_or_else(|| "".to_owned()),
                    })
                    .collect(),
                contributors: sr
                    .contributors
                    .iter()
                    .map(|c| Contributor {
                        contributor_party_reference: c.contributor_party_reference.to_owned(),
                        sequence_number: c.sequence_number.unwrap_or_else(|| 0),
                        contributor_roles: c.roles.iter().map(|cr| cr.content.to_owned()).collect(),
                    })
                    .collect(),
                sound_recording_editions: sr
                    .sound_recording_editions
                    .iter()
                    .map(|sre| SoundRecordingEdition {
                        isrc: sre.resource_ids[0].isrc.to_owned(),
                        p_lines: sre
                            .p_lines
                            .iter()
                            .map(|p_line| PLine {
                                p_line_text: p_line.p_line_text.to_owned(),
                                year: p_line.year.unwrap_or_else(|| 0),
                            })
                            .collect(),
                        fingerprints: sre
                            .technical_details
                            .iter()
                            .map(|td| {
                                td.delivery_files
                                    .iter()
                                    .map(|df| {
                                        df.fingerprints
                                            .iter()
                                            .map(|f| Fingerprint {
                                                parameter: f
                                                    .parameter
                                                    .to_owned()
                                                    .unwrap_or_else(|| "".to_owned()),
                                                namespace: f
                                                    .algorithm
                                                    .namespace
                                                    .to_owned()
                                                    .unwrap_or_else(|| "".to_owned()),
                                                user_defined_value: f
                                                    .algorithm
                                                    .user_defined_value
                                                    .to_owned()
                                                    .unwrap_or_else(|| "".to_owned()),
                                            })
                                            .collect::<Vec<Fingerprint>>()
                                    })
                                    .flatten()
                                    .collect::<Vec<Fingerprint>>()
                            })
                            .flatten()
                            .collect::<Vec<Fingerprint>>(),
                    })
                    .collect(),
                track_release_id: ddex_track_releases
                    .iter()
                    .find(|tr| tr.release_resource_reference == sr.resource_reference)
                    .and_then(|tr| {
                        Some(ReleaseId {
                            grid: tr
                                .release_id
                                .g_rid
                                .to_owned()
                                .unwrap_or_else(|| "".to_owned()),
                            icpn: tr
                                .release_id
                                .icpn
                                .to_owned()
                                .unwrap_or_else(|| "".to_owned()),
                            proprietary_ids: tr
                                .release_id
                                .proprietary_ids
                                .iter()
                                .map(|p_id| p_id.content.to_owned())
                                .collect(),
                        })
                    })
                    .unwrap_or_else(|| ReleaseId {
                        grid: "".to_owned(),
                        icpn: "".to_owned(),
                        proprietary_ids: vec![],
                    }),
            })
            .collect();

        ProvedMessage {
            release: subgraph_release,
            sound_recordings: subgraph_sound_recordings,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateOutputs {
    pub full_content: Option<NewReleaseMessage>,
    pub error: Option<String>,
}

pub fn parse_guest_id(guest_id: &[u32; 8]) -> alloy_primitives::FixedBytes<32> {
    alloy_primitives::FixedBytes::<32>::from_slice(
        &guest_id.map(|word| word.to_le_bytes()).concat(),
    )
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SubmitProofInput {
    pub image_id: FixedBytes<32>,
    pub journal: Vec<u8>,
    pub seal: Vec<u8>,
    pub ipfs_cid: String,
}
