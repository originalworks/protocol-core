use ddex_schema::*;

#[test]
fn validation_content_success() {
    let input = r"
                <EventDateWithCurrentTerritory>1993-12-01</EventDateWithCurrentTerritory>
        ";

    let parsed: Result<EventDateWithCurrentTerritory, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), false);
}
#[test]
fn validation_content_failure() {
    let input = r"
                <EventDateWithCurrentTerritory>Invalid Format</EventDateWithCurrentTerritory>
        ";

    let parsed: Result<EventDateWithCurrentTerritory, String> = yaserde::de::from_str(&input);
    let error = parsed.unwrap_err();

    assert_eq!(
        error,
        "invalid value in field EventDateWithCurrentTerritory"
    );
}

#[test]
fn validation_content_vec_success() {
    let input = r#"
                <DealResourceReferenceList>
                    <DealResourceReference>AA</DealResourceReference>
                    <DealResourceReference>AB</DealResourceReference>
                    <DealResourceReference>AC</DealResourceReference>
                </DealResourceReferenceList>
        "#;

    let parsed: Result<DealResourceReferenceList, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), false);
}

#[test]
fn validation_content_empty_vec_success() {
    let input = r#"
                <DealResourceReferenceList>
                </DealResourceReferenceList>
        "#;

    let parsed: Result<DealResourceReferenceList, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), false);
}

#[test]
fn validation_content_vec_failure() {
    let input = r#"
                <DealResourceReferenceList>
                    <DealResourceReference>AA</DealResourceReference>
                    <DealResourceReference>AB</DealResourceReference>
                    <DealResourceReference>BB</DealResourceReference>
                </DealResourceReferenceList>
        "#;

    let parsed: Result<DealResourceReferenceList, String> = yaserde::de::from_str(&input);
    let error = parsed.unwrap_err();

    assert_eq!(error, "invalid value in field DealResourceReference");
}

#[test]
fn validation_required_attribute_success() {
    let input = r"
                <DisplayArtistName>SomeContent</DisplayArtistName>
        ";

    let parsed: Result<DisplayArtistNameWithDefault, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), false);
}

#[test]
fn validation_required_attribute_failure() {
    let input = r"
                <DisplayArtistName>SomeContent</DisplayArtistName>
        ";

    let parsed: Result<DisplayArtistNameWithDefault, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), false);
}

#[test]
fn validation_optional_attribute_not_present() {
    let input = r"
                <DisplayArtistName>SomeContent</DisplayArtistName>
        ";

    let parsed: Result<DisplayArtistNameWithDefault, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), false);
}

#[test]
fn validation_optional_attribute_success() {
    let input = r#"
                <DisplayArtistName ApplicableTerritoryCode="Worldwide">SomeContent</DisplayArtistName>
        "#;

    let parsed: Result<DisplayArtistNameWithDefault, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), false);
}

#[test]
fn validation_optional_attribute_failure() {
    let input = r#"
                <DisplayArtistName ApplicableTerritoryCode="Nowhere">SomeContent</DisplayArtistName>
        "#;

    let parsed: Result<DisplayArtistNameWithDefault, String> = yaserde::de::from_str(&input);
    let error = parsed.unwrap_err();

    assert_eq!(error, "invalid value in attribute ApplicableTerritoryCode");
}
