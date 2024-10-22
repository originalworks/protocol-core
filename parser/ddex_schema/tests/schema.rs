use ddex_schema::*;

#[test]
fn display_credits() {
    // Multiple values
    let expected = DisplayCredits {
        language_and_script_code: Some("RU".to_string()),
        display_credit_text: "CreditText".to_string(),
        display_credit_parties: vec!["P1".to_string(), "P2".to_string()],
        names_used_in_display_credits: vec!["P1A".to_string(), "P2A".to_string()],
        applicable_territory_code: None,
        is_default: None,
    };

    let input = r#"
            <DisplayCredits LanguageAndScriptCode="RU">
                <DisplayCreditText>CreditText</DisplayCreditText>
                <DisplayCreditParty>P1</DisplayCreditParty>
                <NameUsedInDisplayCredit>P1A</NameUsedInDisplayCredit>
                <DisplayCreditParty>P2</DisplayCreditParty>
                <NameUsedInDisplayCredit>P2A</NameUsedInDisplayCredit>
            </DisplayCredits>
        "#;

    let parsed: Result<DisplayCredits, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap(), &expected);

    // empty vec
    let expected = DisplayCredits {
        language_and_script_code: Some("RU".to_string()),
        display_credit_text: "CreditText".to_string(),
        display_credit_parties: vec![],
        names_used_in_display_credits: vec![],
        applicable_territory_code: None,
        is_default: None,
    };

    let input = r#"
            <DisplayCredits LanguageAndScriptCode="RU">
                <DisplayCreditText>CreditText</DisplayCreditText>
            </DisplayCredits>
        "#;

    let parsed: Result<DisplayCredits, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap(), &expected);

    // No corresponding party for a name
    let input = r#"
            <DisplayCredits LanguageAndScriptCode="RU">
                <DisplayCreditText>CreditText</DisplayCreditText>
                <NameUsedInDisplayCredit>P1A</NameUsedInDisplayCredit>
            </DisplayCredits>
        "#;

    let parsed: Result<DisplayCredits, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().is_err(), true);

    let input = r#"
            <DisplayCredits LanguageAndScriptCode="RU">
                <DisplayCreditText>CreditText</DisplayCreditText>
                <NameUsedInDisplayCredit>P1A</NameUsedInDisplayCredit>
            </DisplayCredits>
        "#;

    let parsed: Result<DisplayCredits, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().is_err(), true);
}
