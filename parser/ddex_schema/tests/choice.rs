use ddex_schema::*;

#[test]
fn period_with_start_date() {
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive::
            YaDeserialize,
        yaserde_derive:: YaSerialize,
    )]
    #[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
    pub struct DealTerms {
        #[yaserde(rename = "ValidityPeriod", prefix = "ern")]
        pub validity_periods: Vec<PeriodWithStartDate>,
    }

    let expected =
        PeriodWithStartDate::EventDateTimeWithoutFlags(EventDateTimeWithoutFlagsRequired {
            start_date_time: EventDateTimeWithoutFlags {
                content: "2014-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            },
            end_date_time: Some(EventDateTimeWithoutFlags {
                content: "2025-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            }),
        });

    // Valid StartDate & Valid EndDate
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDateTime>2014-01-01</StartDateTime>
                    <EndDateTime>2025-01-01</EndDateTime>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap().validity_periods.len(), 1);
    assert_eq!(
        parsed.as_ref().unwrap().validity_periods.first().unwrap(),
        &expected
    );

    let expected =
        PeriodWithStartDate::EventDateTimeWithoutFlags(EventDateTimeWithoutFlagsRequired {
            start_date_time: EventDateTimeWithoutFlags {
                content: "2014-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            },
            end_date_time: None,
        });

    // Valid StartDate & No EndDate
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDateTime>2014-01-01</StartDateTime>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap().validity_periods.len(), 1);
    assert_eq!(
        parsed.as_ref().unwrap().validity_periods.first().unwrap(),
        &expected
    );

    // No StartDate & Valid EndDate
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <EndDateTime>2014-01-01</EndDateTime>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), true);

    let expected =
        PeriodWithStartDate::EventDateWithCurrentTerritory(EventDateWithCurrentTerritoryRequired {
            start_date: EventDateWithCurrentTerritory {
                content: "2014-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            },
            end_date: Some(EventDateWithCurrentTerritory {
                content: "2025-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            }),
        });

    // Valid StartDate & Valid EndDate
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDate>2014-01-01</StartDate>
                    <EndDate>2025-01-01</EndDate>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap().validity_periods.len(), 1);
    assert_eq!(
        parsed.as_ref().unwrap().validity_periods.first().unwrap(),
        &expected
    );

    let expected =
        PeriodWithStartDate::EventDateWithCurrentTerritory(EventDateWithCurrentTerritoryRequired {
            start_date: EventDateWithCurrentTerritory {
                content: "2014-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            },
            end_date: None,
        });

    // Valid StartDate & Empty EndDate
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDate>2014-01-01</StartDate>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap().validity_periods.len(), 1);
    assert_eq!(
        parsed.as_ref().unwrap().validity_periods.first().unwrap(),
        &expected
    );

    // No StartDate & Valid EndDate
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <EndDate>2014-01-01</EndDate>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), true);

    // Valid StartDate & Empty EndDate
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDate>2014-01-01</StartDate>
                    <EndDate>2025-01-01</EndDate>
                </ValidityPeriod>
                <ValidityPeriod>
                    <StartDateTime>2014-01-01</StartDateTime>
                    <EndDateTime>2025-01-01</EndDateTime>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);
    let first_val = parsed.as_ref().unwrap().validity_periods.first().unwrap();

    if let PeriodWithStartDate::EventDateWithCurrentTerritory(_) = first_val {
        assert_eq!(true, true)
    } else {
        assert_eq!(true, false)
    }

    let second_val = parsed.as_ref().unwrap().validity_periods.last().unwrap();

    if let PeriodWithStartDate::EventDateTimeWithoutFlags(_) = second_val {
        assert_eq!(true, true)
    } else {
        assert_eq!(true, false)
    }

    // Empty tag
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), true);

    // Mixed format
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDate>2024-01-01</StartDate>
                    <EndDateTime>2025-01-01</EndDateTime>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), true);

    // Duplicate start date
    let input = r#"
         <DealTerms>
             <ValidityPeriod>
                 <StartDate>2024-01-01</StartDate>
                 <StartDateTime>2024-01-01</StartDateTime>
             </ValidityPeriod>
         </DealTerms>
     "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), true);
}

#[test]
fn period_without_flags() {
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive::
            YaDeserialize,
        yaserde_derive:: YaSerialize,
    )]
    #[yaserde(prefix = "ern", namespace = "ern: http://ddex.net/xml/ern/43")]
    pub struct DealTerms {
        #[yaserde(rename = "ValidityPeriod", prefix = "ern")]
        pub validity_periods: Vec<PeriodWithoutFlags>,
    }

    // Valid StartDate & Valid EndDate
    let expected =
        PeriodWithoutFlags::EventDateTimeWithoutFlags(EventDateTimeWithoutFlagsOptional {
            start_date_time: Some(EventDateTimeWithoutFlags {
                content: "2014-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            }),
            end_date_time: Some(EventDateTimeWithoutFlags {
                content: "2025-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            }),
        });

    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDateTime>2014-01-01</StartDateTime>
                    <EndDateTime>2025-01-01</EndDateTime>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap().validity_periods.len(), 1);
    assert_eq!(
        parsed.as_ref().unwrap().validity_periods.first().unwrap(),
        &expected
    );

    // Valid StartDate & No EndDate
    let expected =
        PeriodWithoutFlags::EventDateTimeWithoutFlags(EventDateTimeWithoutFlagsOptional {
            start_date_time: Some(EventDateTimeWithoutFlags {
                content: "2014-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            }),
            end_date_time: None,
        });

    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDateTime>2014-01-01</StartDateTime>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap().validity_periods.len(), 1);
    assert_eq!(
        parsed.as_ref().unwrap().validity_periods.first().unwrap(),
        &expected
    );

    // No StartDate & Valid EndDate
    let expected =
        PeriodWithoutFlags::EventDateTimeWithoutFlags(EventDateTimeWithoutFlagsOptional {
            start_date_time: None,
            end_date_time: Some(EventDateTimeWithoutFlags {
                content: "2025-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            }),
        });

    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <EndDateTime>2025-01-01</EndDateTime>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap().validity_periods.len(), 1);
    assert_eq!(
        parsed.as_ref().unwrap().validity_periods.first().unwrap(),
        &expected
    );

    // Valid StartDate & Valid EndDate
    let expected =
        PeriodWithoutFlags::EventDateWithCurrentTerritory(EventDateWithCurrentTerritoryOptional {
            start_date: Some(EventDateWithCurrentTerritory {
                content: "2014-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            }),
            end_date: Some(EventDateWithCurrentTerritory {
                content: "2025-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            }),
        });

    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDate>2014-01-01</StartDate>
                    <EndDate>2025-01-01</EndDate>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap().validity_periods.len(), 1);
    assert_eq!(
        parsed.as_ref().unwrap().validity_periods.first().unwrap(),
        &expected
    );

    // Valid StartDate & Empty EndDate
    let expected =
        PeriodWithoutFlags::EventDateWithCurrentTerritory(EventDateWithCurrentTerritoryOptional {
            start_date: Some(EventDateWithCurrentTerritory {
                content: "2014-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            }),
            end_date: None,
        });

    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDate>2014-01-01</StartDate>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap().validity_periods.len(), 1);
    assert_eq!(
        parsed.as_ref().unwrap().validity_periods.first().unwrap(),
        &expected
    );

    // No StartDate & Valid EndDate
    let expected =
        PeriodWithoutFlags::EventDateWithCurrentTerritory(EventDateWithCurrentTerritoryOptional {
            start_date: None,
            end_date: Some(EventDateWithCurrentTerritory {
                content: "2025-01-01".to_string(),
                applicable_territory_code: None,
                is_approximate: None,
                language_and_script_code: None,
                location_description: None,
            }),
        });

    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <EndDate>2025-01-01</EndDate>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.as_ref().unwrap().validity_periods.len(), 1);
    assert_eq!(
        parsed.as_ref().unwrap().validity_periods.first().unwrap(),
        &expected
    );

    // Multiple types
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDate>2014-01-01</StartDate>
                    <EndDate>2025-01-01</EndDate>
                </ValidityPeriod>
                <ValidityPeriod>
                    <StartDateTime>2014-01-01</StartDateTime>
                    <EndDateTime>2025-01-01</EndDateTime>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);
    let first_val = parsed.as_ref().unwrap().validity_periods.first().unwrap();

    if let PeriodWithoutFlags::EventDateWithCurrentTerritory(_) = first_val {
        assert_eq!(true, true)
    } else {
        assert_eq!(true, false)
    }

    let second_val = parsed.as_ref().unwrap().validity_periods.last().unwrap();

    if let PeriodWithoutFlags::EventDateTimeWithoutFlags(_) = second_val {
        assert_eq!(true, true)
    } else {
        assert_eq!(true, false)
    }

    // Empty tag
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), true);

    // Mixed format
    let input = r#"
            <DealTerms>
                <ValidityPeriod>
                    <StartDate>2024-01-01</StartDate>
                    <EndDateTime>2025-01-01</EndDateTime>
                </ValidityPeriod>
            </DealTerms>
        "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), true);

    // Duplicate start date
    let input = r#"
         <DealTerms>
             <ValidityPeriod>
                 <StartDate>2024-01-01</StartDate>
                 <StartDateTime>2024-01-01</StartDateTime>
             </ValidityPeriod>
         </DealTerms>
     "#;

    let parsed: Result<DealTerms, String> = yaserde::de::from_str(&input);

    assert_eq!(parsed.is_err(), true);
}
