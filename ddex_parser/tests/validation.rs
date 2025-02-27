use std::fs;

use ddex_parser::{
    Affiliation, DelegatedUsageRights, NewReleaseMessage, Party, ResourceList,
    ResourceRightsController,
};
use rand::Rng;
use serde_json::{json, Value};
use serde_valid::json::FromJsonStr;
#[test]
fn choice_affiliation() {
    // Working: company_name
    let mut src = json!({
        "kind": "MusicLicensingCompany",
        "rights_types": [
            {"content":"MakeAvailableRight"}
        ],
        "company_name": "Eloszki",
        "party_affiliate_reference": null,
        "territory_codes": ["AD"]
    })
    .to_string();

    assert_eq!(Affiliation::from_json_str(src.as_str()).is_err(), false);

    // Working: party_affiliate_reference
    src = json!({
        "kind": "MusicLicensingCompany",
        "rights_types": [
            {"content":"MakeAvailableRight"}
        ],
        "company_name": null,
        "party_affiliate_reference": "P1234",
        "territory_codes": ["AD"]
    })
    .to_string();

    assert_eq!(Affiliation::from_json_str(src.as_str()).is_err(), false);

    // Error: missing both
    src = json!({
        "kind": "MusicLicensingCompany",
        "rights_types": [
            {"content":"MakeAvailableRight"}
        ],
        "company_name": null,
        "party_affiliate_reference": null,
        "territory_codes": ["AD"]
    })
    .to_string();

    assert_eq!(Affiliation::from_json_str(src.as_str()).is_err(), true);

    // Error: both present
    src = json!({
        "kind": "MusicLicensingCompany",
        "rights_types": [
            {"content":"MakeAvailableRight"}
        ],
        "company_name": "Eloszki",
        "party_affiliate_reference": "P1234",
        "territory_codes": ["AD"]
    })
    .to_string();

    assert_eq!(Affiliation::from_json_str(src.as_str()).is_err(), true);
}

#[test]
fn choice_resource_rights_controller() {
    // Working: right_share_percentage
    let mut src = json!({
      "rights_controller_party_reference": "P1234",
      "rights_control_types": [ "RoyaltyAdministrator" ],
      "right_share_percentage": "50",
      "right_share_unknown": null,
      "delegated_usage_rights": []
    })
    .to_string();

    assert_eq!(
        ResourceRightsController::from_json_str(src.as_str()).is_err(),
        false
    );

    // Working: right_share_unknown
    src = json!({
      "rights_controller_party_reference": "P1234",
      "rights_control_types": [ "RoyaltyAdministrator" ],
      "right_share_percentage": null,
      "right_share_unknown": true,
      "delegated_usage_rights": []
    })
    .to_string();

    assert_eq!(
        ResourceRightsController::from_json_str(src.as_str()).is_err(),
        false
    );

    // Error: None present
    src = json!({
      "rights_controller_party_reference": "P1234",
      "rights_control_types": [ "RoyaltyAdministrator" ],
      "right_share_percentage": null,
      "right_share_unknown": null,
      "delegated_usage_rights": []
    })
    .to_string();

    assert_eq!(
        ResourceRightsController::from_json_str(src.as_str()).is_err(),
        true
    );

    // Error: Both present
    src = json!({
      "rights_controller_party_reference": "P1234",
      "rights_control_types": [ "RoyaltyAdministrator" ],
      "right_share_percentage": "50",
      "right_share_unknown": true,
      "delegated_usage_rights": []
    })
    .to_string();

    assert_eq!(
        ResourceRightsController::from_json_str(src.as_str()).is_err(),
        true
    );

    // EDGE CASE: right_shares_unknown: false && right_share_percentage present which is technically correct
    src = json!({
      "rights_controller_party_reference": "P1234",
      "rights_control_types": [ "RoyaltyAdministrator" ],
      "right_share_percentage": "50",
      "right_share_unknown": false,
      "delegated_usage_rights": []
    })
    .to_string();

    assert_eq!(
        ResourceRightsController::from_json_str(src.as_str()).is_err(),
        true
    );
}

#[test]
fn choice_party() {
    // Working: party_name
    let mut src = json!({
      "party_reference": "P1234",
      "affiliations": [],
      "parties_names": [{"full_name": {"content": "Elou"}}],
      "parties_ids": []
    })
    .to_string();

    assert_eq!(Party::from_json_str(src.as_str()).is_err(), false);

    // Working: party_id
    src = json!({
      "party_reference": "P1234",
      "affiliations": [],
      "parties_names": [],
      "parties_ids": [{"isni": "blah"}]
    })
    .to_string();

    assert_eq!(Party::from_json_str(src.as_str()).is_err(), false);

    // Working: Both present
    src = json!({
      "party_reference": "P1234",
      "affiliations": [],
      "parties_names": [{"full_name": {"content": "Elou"}}],
      "parties_ids": [{"isni": "blah"}]
    })
    .to_string();

    assert_eq!(Party::from_json_str(src.as_str()).is_err(), false);

    // Error: both missing
    src = json!({
      "party_reference": "P1234",
      "affiliations": [],
      "parties_names": [],
      "parties_ids": []
    })
    .to_string();

    assert_eq!(Party::from_json_str(src.as_str()).is_err(), true);

    // EDGE CASE: party_id can be empty object...
    src = json!({
      "party_reference": "P1234",
      "affiliations": [],
      "parties_names": [],
      "parties_ids": [{}]
    })
    .to_string();

    assert_eq!(Party::from_json_str(src.as_str()).is_err(), false);
}

#[test]
fn protocol_affiliation() {
    // Working: MusicLicensingCompany
    let mut src = json!({
        "kind": "MusicLicensingCompany",
        "rights_types": [
            {"content":"MakeAvailableRight"}
        ],
        "company_name": "Eloszki",
        "party_affiliate_reference": null,
        "territory_codes": ["AD"]
    })
    .to_string();

    assert_eq!(Affiliation::from_json_str(src.as_str()).is_err(), false);

    // Error: !MusicLicensingCompany
    src = json!({
        "kind": "MusicPublisher",
        "rights_types": [
            {"content":"MakeAvailableRight"}
        ],
        "company_name": "Eloszki",
        "party_affiliate_reference": null,
        "territory_codes": ["AD"]
    })
    .to_string();

    assert_eq!(Affiliation::from_json_str(src.as_str()).is_err(), true);
}

#[test]
fn protocol_rights_type() {
    // Working: MakeAvailableRight
    let mut src = json!({
        "kind": "MusicLicensingCompany",
        "rights_types": [
            {"content":"MakeAvailableRight"}
        ],
        "company_name": "Eloszki",
        "party_affiliate_reference": null,
        "territory_codes": ["AD"]
    })
    .to_string();

    assert_eq!(Affiliation::from_json_str(src.as_str()).is_err(), false);

    // Works: Includes MakeAvailableRight
    src = json!({
        "kind": "MusicLicensingCompany",
        "rights_types": [
            {"content":"MakeAvailableRight"}, {"content": "MechanicalRight"}
        ],
        "company_name": "Eloszki",
        "party_affiliate_reference": null,
        "territory_codes": ["AD"]
    })
    .to_string();

    assert_eq!(Affiliation::from_json_str(src.as_str()).is_err(), false);

    // Error: !Includes MakeAvailableRight
    src = json!({
        "kind": "MusicLicensingCompany",
        "rights_types": [
            {"content": "MechanicalRight"}
        ],
        "company_name": "Eloszki",
        "party_affiliate_reference": null,
        "territory_codes": ["AD"]
    })
    .to_string();

    assert_eq!(Affiliation::from_json_str(src.as_str()).is_err(), true);
}

struct EditionProps<'a> {
    isrc: &'a String,
    kind: Option<String>,
}

fn generate_isrc() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "US1Q1{}",
        (0..7)
            .map(|_| rng.gen_range(0..10).to_string())
            .collect::<String>()
    )
}

fn create_sound_recording(editions: Vec<EditionProps>) -> Value {
    let mut rng = rand::thread_rng();
    let filled_editions = editions
        .iter()
        .map(|props| {
            json!({
                "kind": props.kind,
                "resource_ids": [{
                    "isrc": props.isrc.clone(),
                    "proprietary_ids": []
                }],
                "edition_contributors": [{
                    "contributor_party_reference": "P1234",
                    "roles": [],
                    "has_made_featured_contribution": null,
                    "has_made_contracted_contribution": null,
                    "is_credited": null,
                    "display_credits": [],
                    "sequence_number": null
                }],
                "p_lines": [],
                "recording_mode": null,
                "technical_details": []
            })
        })
        .collect::<Vec<Value>>();

    let sound_recording = json!({
        "resource_reference": format!("A{}", rng.gen::<u32>()),
        "kind": {
            "content": "AudioStem"
        },
        "sound_recording_editions": filled_editions,
        "recording_formats": [],
        "work_ids": [],
        "display_title_texts":[{
            "content": "Elou"
        }],
        "display_titles":[{
            "title_text": "Elou",
            "sub_titles": [],
            "applicable_territory_code": null,
            "is_default": null

        }],
        "version_types": [],
        "display_artist_names": [{
            "content": "Elou",
            "language_and_script_code": null,
            "applicable_territory_code": null,
            "is_default": null

        }],
        "display_artists": [{
            "artist_party_reference": "P1234",
            "display_artist_role": {
                "content": "MainArtist"
            },
            "artistic_roles": [],
            "title_display_informations": [],
            "sequence_number": null
        }],
        "contributors": [],
        "characters": [],
        "resource_rights_controllers": [],
        "duration": "3:45",
        "creation_date": null,
        "parental_warning_types": [{
            "content": "NotExplicit"
        }],
        "is_instrumental": null,
        "language_of_performances": [],
    });

    return sound_recording;
}

#[test]
fn protocol_sound_recordings() {
    let isrc_a = generate_isrc();
    let isrc_b = generate_isrc();
    let isrc_c = generate_isrc();
    let isrc_d = generate_isrc();
    // Working: Unique ISRCs & types
    let mut sound_recording_1 = create_sound_recording(vec![
        EditionProps {
            isrc: &isrc_a,
            kind: Some("ImmersiveEdition".to_string()),
        },
        EditionProps {
            isrc: &isrc_b,
            kind: Some("NonImmersiveEdition".to_string()),
        },
    ]);

    let mut sound_recording_2 = create_sound_recording(vec![
        EditionProps {
            isrc: &isrc_c,
            kind: Some("ImmersiveEdition".to_string()),
        },
        EditionProps {
            isrc: &isrc_d,
            kind: Some("NonImmersiveEdition".to_string()),
        },
    ]);

    let mut src = json!({
        "sound_recordings": [sound_recording_1, sound_recording_2],
        "images": []
    })
    .to_string();

    assert_eq!(ResourceList::from_json_str(src.as_str()).is_err(), false);

    // Working: Kind not set
    sound_recording_1 = create_sound_recording(vec![
        EditionProps {
            isrc: &isrc_a,
            kind: None,
        },
        EditionProps {
            isrc: &isrc_b,
            kind: None,
        },
    ]);

    sound_recording_2 = create_sound_recording(vec![
        EditionProps {
            isrc: &isrc_c,
            kind: None,
        },
        EditionProps {
            isrc: &isrc_d,
            kind: None,
        },
    ]);

    src = json!({
        "sound_recordings": [sound_recording_1, sound_recording_2],
        "images": []
    })
    .to_string();

    assert_eq!(ResourceList::from_json_str(src.as_str()).is_err(), false);

    // Error: No sound recordings
    src = json!({
        "sound_recordings": [],
        "images": []
    })
    .to_string();

    assert_eq!(ResourceList::from_json_str(src.as_str()).is_err(), true);

    // Error: Duplicate Type within same SoundRecording
    sound_recording_1 = create_sound_recording(vec![
        EditionProps {
            isrc: &isrc_a,
            kind: Some("ImmersiveEdition".to_string()),
        },
        EditionProps {
            isrc: &isrc_b,
            kind: Some("ImmersiveEdition".to_string()),
        },
    ]);

    sound_recording_2 = create_sound_recording(vec![
        EditionProps {
            isrc: &isrc_c,
            kind: Some("ImmersiveEdition".to_string()),
        },
        EditionProps {
            isrc: &isrc_d,
            kind: Some("NonImmersiveEdition".to_string()),
        },
    ]);

    src = json!({
        "sound_recordings": [sound_recording_1, sound_recording_2],
        "images": []
    })
    .to_string();

    assert_eq!(ResourceList::from_json_str(src.as_str()).is_err(), true);

    // Error: Duplicate ISRC within same SoundRecording
    sound_recording_1 = create_sound_recording(vec![
        EditionProps {
            isrc: &isrc_a,
            kind: Some("ImmersiveEdition".to_string()),
        },
        EditionProps {
            isrc: &isrc_a,
            kind: Some("NonImmersiveEdition".to_string()),
        },
    ]);

    sound_recording_2 = create_sound_recording(vec![
        EditionProps {
            isrc: &isrc_c,
            kind: Some("ImmersiveEdition".to_string()),
        },
        EditionProps {
            isrc: &isrc_d,
            kind: Some("NonImmersiveEdition".to_string()),
        },
    ]);

    src = json!({
        "sound_recordings": [sound_recording_1, sound_recording_2],
        "images": []
    })
    .to_string();

    assert_eq!(ResourceList::from_json_str(src.as_str()).is_err(), true);

    // Error: Duplicate ISRC across all SoundRecordings
    sound_recording_1 = create_sound_recording(vec![
        EditionProps {
            isrc: &isrc_a,
            kind: Some("ImmersiveEdition".to_string()),
        },
        EditionProps {
            isrc: &isrc_b,
            kind: Some("NonImmersiveEdition".to_string()),
        },
    ]);

    sound_recording_2 = create_sound_recording(vec![
        EditionProps {
            isrc: &isrc_a,
            kind: Some("ImmersiveEdition".to_string()),
        },
        EditionProps {
            isrc: &isrc_d,
            kind: Some("NonImmersiveEdition".to_string()),
        },
    ]);

    src = json!({
        "sound_recordings": [sound_recording_1, sound_recording_2],
        "images": []
    })
    .to_string();

    assert_eq!(ResourceList::from_json_str(src.as_str()).is_err(), true);

    // Error: ISRC is not ISO compliant
    sound_recording_1 = create_sound_recording(vec![EditionProps {
        isrc: &"HardkorowoPadaDeszcz".to_string(),
        kind: Some("ImmersiveEdition".to_string()),
    }]);

    sound_recording_2 = create_sound_recording(vec![EditionProps {
        isrc: &"MilaZaneta2001".to_string(),
        kind: Some("ImmersiveEdition".to_string()),
    }]);

    src = json!({
        "sound_recordings": [sound_recording_1, sound_recording_2],
        "images": []
    })
    .to_string();

    assert_eq!(ResourceList::from_json_str(src.as_str()).is_err(), true);
}

#[test]
fn protocol_right_control_types() {
    // Working: At least one control types is RoyaltyAdministrator
    let mut src = json!({
      "rights_controller_party_reference": "P1234",
      "rights_control_types": [ "RoyaltyAdministrator", "LocalPayee"],
      "right_share_percentage": "50",
      "right_share_unknown": null,
      "delegated_usage_rights": []
    })
    .to_string();

    assert_eq!(
        ResourceRightsController::from_json_str(src.as_str()).is_err(),
        false
    );

    // Error: No rights control types
    src = json!({
      "rights_controller_party_reference": "P1234",
      "rights_control_types": [],
      "right_share_percentage": "50",
      "right_share_unknown": null,
      "delegated_usage_rights": []
    })
    .to_string();

    assert_eq!(
        ResourceRightsController::from_json_str(src.as_str()).is_err(),
        true
    );

    // Error: No RoyaltyAdministrator
    src = json!({
      "rights_controller_party_reference": "P1234",
      "rights_control_types": ["LocalPayee"],
      "right_share_percentage": "50",
      "right_share_unknown": null,
      "delegated_usage_rights": []
    })
    .to_string();

    assert_eq!(
        ResourceRightsController::from_json_str(src.as_str()).is_err(),
        true
    );
}

#[test]
fn protocol_use_types() {
    // Working: All use types are accepted values
    let mut src = json!({
        "use_types": [
            {"content": "Stream"},
            {"content": "PermanentDownload"},
            {"content": "ConditionalDownload"},
            {"content": "TetheredDownload"}
        ],
        "territory_of_rights_delegations": [ {"content": "AD"}],
    })
    .to_string();

    assert_eq!(
        DelegatedUsageRights::from_json_str(src.as_str()).is_err(),
        false
    );

    // Working: At least one value is accepted
    src = json!({
        "use_types": [
            {"content": "Stream"},
            {"content": "Cable"}
        ],
        "territory_of_rights_delegations": [ {"content": "AD"}],
    })
    .to_string();

    assert_eq!(
        DelegatedUsageRights::from_json_str(src.as_str()).is_err(),
        false
    );

    // Error: No use types
    src = json!({
        "use_types": [],
        "territory_of_rights_delegations": [ {"content": "AD"}],
    })
    .to_string();

    assert_eq!(
        DelegatedUsageRights::from_json_str(src.as_str()).is_err(),
        true
    );

    // Error: All values are not accepted
    src = json!({
        "use_types": [
            {"content": "Broadcast"},
            {"content": "Cable"}
        ],
        "territory_of_rights_delegations": [ {"content": "AD"}],
    })
    .to_string();

    assert_eq!(
        DelegatedUsageRights::from_json_str(src.as_str()).is_err(),
        true
    );
}

fn create_resource_rights_controllers(refs: Vec<&String>) -> Vec<ResourceRightsController> {
    let controllers: Vec<ResourceRightsController> = refs
        .iter()
        .map(|reference| {
            serde_json::from_value::<ResourceRightsController>(json!({
              "rights_controller_party_reference": reference,
              "rights_control_types": ["RoyaltyAdministrator"],
              "right_share_percentage": "50",
              "right_share_unknown": null,
              "delegated_usage_rights": []
            }))
            .unwrap()
        })
        .collect();
    return controllers;
}

fn create_parties(refs_with_kinds: Vec<(&String, Vec<&String>)>) -> Vec<Party> {
    let parties: Vec<Party> = refs_with_kinds
        .iter()
        .map(|(reference, kinds)| {
            let affiliations: Vec<Value> = kinds
                .iter()
                .map(|kind| {
                    json!({
                      "kind": kind,
                      "validity_period": null,
                      "rights_types": [],
                      "percentage_of_rights_assignment": null,
                      "company_name": null,
                      "party_affiliate_reference": null,
                      "territory_codes": ["AD"]
                    })
                })
                .collect();
            serde_json::from_value::<Party>(json!({
                "party_reference": reference,
                "affiliations": affiliations,
                "parties_names": [],
                "parties_ids": []
            }))
            .unwrap()
        })
        .collect();
    return parties;
}

fn create_new_release_message_json(
    template: &NewReleaseMessage,
    rrcs_refs: Vec<&String>,
    parties_props: Vec<(&String, Vec<&String>)>,
) -> String {
    let mut src = template.clone();
    src.party_list.partys = create_parties(parties_props);
    src.resource_list.sound_recordings[0].resource_rights_controllers =
        create_resource_rights_controllers(rrcs_refs);
    serde_json::to_string_pretty(&src).unwrap()
}

#[test]
fn protocol_music_licensing_company() {
    let ref_a = "P1234".to_string();
    let ref_b = "P2345".to_string();
    let ref_c = "P3456".to_string();
    let correct_type = "MusicLicensingCompany".to_string();
    let wrong_type_a = "MusicPublisher".to_string();

    let template_string = fs::read_to_string("tests/template.json").unwrap();
    let template: NewReleaseMessage = serde_json::from_str(&template_string).unwrap();

    // Works: 0 rcc, 1 party
    let mut src = create_new_release_message_json(&template, vec![], vec![(&ref_a, vec![])]);

    assert_eq!(
        NewReleaseMessage::from_json_str(src.as_str()).is_err(),
        false
    );

    // Works: 1 rrc, 1 party that is rcc with 1 affiliation of type MusicLicensingCompany
    src = create_new_release_message_json(
        &template,
        vec![&ref_a],
        vec![(&ref_a, vec![&correct_type])],
    );

    assert_eq!(
        NewReleaseMessage::from_json_str(src.as_str()).is_err(),
        false
    );

    // Works: 1 rrc, 1 party that is rcc with 2 affiliations and one of them is MusicLicensingCompany
    src = create_new_release_message_json(
        &template,
        vec![&ref_a],
        vec![(&ref_a, vec![&correct_type, &wrong_type_a])],
    );

    assert_eq!(
        NewReleaseMessage::from_json_str(src.as_str()).is_err(),
        false
    );

    // Works: 1 rrc, 2 parties:
    //               - a: is rcc with 2 affiliations, one of them is MusicLicensingCompany
    //               - b: is NOT rcc with 2 affiliations with any types
    src = create_new_release_message_json(
        &template,
        vec![&ref_a],
        vec![
            (&ref_a, vec![&correct_type, &wrong_type_a]),
            (&ref_b, vec![&correct_type, &wrong_type_a]),
        ],
    );

    assert_eq!(
        NewReleaseMessage::from_json_str(src.as_str()).is_err(),
        false
    );

    // Works: 2 rrcs, 2 parties:
    //               - a: is rcc with 2 affiliations, one of them is MusicLicensingCompany
    //               - b: is rcc with 2 affiliations, one of them is MusicLicensingCompany

    src = create_new_release_message_json(
        &template,
        vec![&ref_a, &ref_b],
        vec![
            (&ref_a, vec![&correct_type, &wrong_type_a]),
            (&ref_b, vec![&correct_type, &wrong_type_a]),
        ],
    );

    assert_eq!(
        NewReleaseMessage::from_json_str(src.as_str()).is_err(),
        false
    );

    // Error: 1 rcc, 2 parties, none of them is rcc
    src = create_new_release_message_json(
        &template,
        vec![&ref_a],
        vec![
            (&ref_b, vec![&correct_type, &wrong_type_a]),
            (&ref_c, vec![&correct_type, &wrong_type_a]),
        ],
    );

    assert_eq!(
        NewReleaseMessage::from_json_str(src.as_str()).is_err(),
        true
    );

    // Error: 1 rcc, 2 parties, 1 of them is rcc but has no affiliations
    src = create_new_release_message_json(
        &template,
        vec![&ref_a],
        vec![(&ref_a, vec![]), (&ref_b, vec![])],
    );

    assert_eq!(
        NewReleaseMessage::from_json_str(src.as_str()).is_err(),
        true
    );

    // Error: 1 rcc, 2 parties, 1 of them is rcc but with affiliation of wrong type
    src = create_new_release_message_json(
        &template,
        vec![&ref_a],
        vec![(&ref_a, vec![&wrong_type_a]), (&ref_b, vec![])],
    );

    assert_eq!(
        NewReleaseMessage::from_json_str(src.as_str()).is_err(),
        true
    );

    // Error: 2 rccs, 1 party
    src = create_new_release_message_json(
        &template,
        vec![&ref_a, &ref_b],
        vec![(&ref_c, vec![&wrong_type_a]), (&ref_b, vec![])],
    );

    assert_eq!(
        NewReleaseMessage::from_json_str(src.as_str()).is_err(),
        true
    );
}
