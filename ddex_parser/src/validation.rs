use once_cell::sync::Lazy;
use regex::Regex;
use serde_valid::validation::error::{Format, Message};
use serde_valid::validation::Error;
use serde_valid::PatternError;
use std::collections::HashSet;

use crate::{ReleaseId, RightsType, SoundRecording, UseType};

fn is_valid_reference(start_letter: char, value: &str) -> bool {
    let mut chars = value.chars();
    if chars.next() != Some(start_letter) {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn is_valid_padpida(s: &str) -> bool {
    s.starts_with("PADPIDA") && s.chars().skip(7).all(|c| c.is_ascii_alphanumeric())
}

fn is_valid_isrc(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() != 12 {
        return false;
    }
    if !chars[0].is_ascii_alphabetic() || !chars[1].is_ascii_alphabetic() {
        return false;
    }
    if !chars[2].is_ascii_alphanumeric() && chars[2] != '_'
        || !chars[3].is_ascii_alphanumeric() && chars[3] != '_'
        || !chars[4].is_ascii_alphanumeric() && chars[4] != '_'
    {
        return false;
    }
    if !chars[5..12].iter().all(|c| c.is_ascii_digit()) {
        return false;
    }

    true
}

// These regexes are less expensive than similar functions rewritten in rust :o
static DATE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[0-9]{4}(-[0-9]{2}){0,1}(-[0-9]{2}){0,1}$").unwrap());
static LANGUAGE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z]{2,3}(-[a-zA-Z]+){0,1}(-[a-zA-Z]{2}|-[0-9]{3}){0,1}(-[a-zA-Z][a-zA-Z0-9]{4}[a-zA-Z0-9]*){0,1}$").unwrap()
});
pub trait Validator {
    fn regex_check() -> Option<&'static Regex> {
        None
    }

    #[allow(unused_variables)]
    fn bool_check(value: &str) -> Option<bool> {
        None
    }

    #[allow(dead_code)]
    fn validate(value: &str) -> bool {
        if let Some(re) = Self::regex_check() {
            return re.is_match(value);
        }

        if let Some(result) = Self::bool_check(value) {
            return result;
        }

        panic!("Validation not set");
    }

    #[allow(dead_code)]
    fn json_validate<'a>(value: impl Into<Option<&'a String>>) -> Result<(), Error> {
        let opt_value: Option<&String> = value.into();

        if let Some(val) = opt_value {
            match (Self::regex_check(), Self::bool_check(val)) {
                (Some(re), None) => {
                    if re.is_match(val.as_str()) {
                        return Ok(());
                    } else {
                        return Err(Error::Pattern(Message::new(
                            PatternError::new(re.to_string()),
                            Format::Default,
                        )));
                    }
                }
                (None, Some(result)) => {
                    if result {
                        return Ok(());
                    } else {
                        return Err(Error::Custom(format!("Invalid value: {}", val)));
                    }
                }
                (None, None) => panic!("Validation not set"),
                (Some(_), Some(_)) => panic!("Only one validation method is allowed"),
            }
        } else {
            return Ok(());
        }
    }

    #[allow(dead_code)]
    fn json_validate_vec(values: &Vec<String>) -> Result<(), Vec<Error>> {
        let mut errors: Vec<Error> = vec![];

        for val in values {
            match (Self::regex_check(), Self::bool_check(val)) {
                (Some(re), None) => {
                    if !re.is_match(val.as_str()) {
                        errors.push(Error::Pattern(Message::new(
                            PatternError::new(re.to_string()),
                            Format::Default,
                        )));
                    }
                }
                (None, Some(result)) => {
                    if !result {
                        errors.push(Error::Custom(format!("Invalid value: {}", val)));
                    }
                }
                (None, None) => panic!("Validation not set"),
                (Some(_), Some(_)) => panic!("Only one validation method is allowed"),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
#[allow(dead_code)]
pub struct ISRCValidator;
impl Validator for ISRCValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_isrc(value))
    }
}
#[allow(dead_code)]
pub struct PartyIdValidator;
impl Validator for PartyIdValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_padpida(value))
    }
}

#[allow(dead_code)]
pub struct PartyReferenceValidator;
impl Validator for PartyReferenceValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('P', value))
    }
}

#[allow(dead_code)]
pub struct ContributorPartyReferenceValidator;
impl Validator for ContributorPartyReferenceValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('P', value))
    }
}

#[allow(dead_code)]
pub struct AvsAffiliationTypeValidator;
impl Validator for AvsAffiliationTypeValidator {
    fn bool_check(value: &str) -> Option<bool> {
        match value {
            "MusicLicensingCompany"
            | "MusicPublisher"
            | "MusicRightsSociety"
            | "RecordCompany"
            | "UserDefined" => Some(true),
            _ => Some(false),
        }
    }
}

#[allow(dead_code)]
pub struct PartyAffiliateReferenceValidator;
impl Validator for PartyAffiliateReferenceValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('P', value))
    }
}

#[allow(dead_code)]
pub struct AvsCurrentTerritoryCodeValidator;
impl Validator for AvsCurrentTerritoryCodeValidator {
    fn bool_check(value: &str) -> Option<bool> {
        static VALID_VALUES: Lazy<HashSet<&str>> = Lazy::new(|| {
            HashSet::from([
                "AD",
                "AE",
                "AF",
                "AG",
                "AI",
                "AL",
                "AM",
                "AN",
                "AO",
                "AQ",
                "AR",
                "AS",
                "AT",
                "AU",
                "AW",
                "AX",
                "AZ",
                "BA",
                "BB",
                "BD",
                "BE",
                "BF",
                "BG",
                "BH",
                "BI",
                "BJ",
                "BL",
                "BM",
                "BN",
                "BO",
                "BQ",
                "BR",
                "BS",
                "BT",
                "BV",
                "BW",
                "BY",
                "BZ",
                "CA",
                "CC",
                "CD",
                "CF",
                "CG",
                "CH",
                "CI",
                "CK",
                "CL",
                "CM",
                "CN",
                "CO",
                "CR",
                "CS",
                "CU",
                "CV",
                "CW",
                "CX",
                "CY",
                "CZ",
                "DE",
                "DJ",
                "DK",
                "DM",
                "DO",
                "DZ",
                "EC",
                "EE",
                "EG",
                "EH",
                "ER",
                "ES",
                "ES-CE",
                "ES-CN",
                "ES-ML",
                "ET",
                "FI",
                "FJ",
                "FK",
                "FM",
                "FO",
                "FR",
                "GA",
                "GB",
                "GD",
                "GE",
                "GF",
                "GG",
                "GH",
                "GI",
                "GL",
                "GM",
                "GN",
                "GP",
                "GQ",
                "GR",
                "GS",
                "GT",
                "GU",
                "GW",
                "GY",
                "HK",
                "HM",
                "HN",
                "HR",
                "HT",
                "HU",
                "ID",
                "IE",
                "IL",
                "IM",
                "IN",
                "IO",
                "IQ",
                "IR",
                "IS",
                "IT",
                "JE",
                "JM",
                "JO",
                "JP",
                "KE",
                "KG",
                "KH",
                "KI",
                "KM",
                "KN",
                "KP",
                "KR",
                "KW",
                "KY",
                "KZ",
                "LA",
                "LB",
                "LC",
                "LI",
                "LK",
                "LR",
                "LS",
                "LT",
                "LU",
                "LV",
                "LY",
                "MA",
                "MC",
                "MD",
                "ME",
                "MF",
                "MG",
                "MH",
                "MK",
                "ML",
                "MM",
                "MN",
                "MO",
                "MP",
                "MQ",
                "MR",
                "MS",
                "MT",
                "MU",
                "MV",
                "MW",
                "MX",
                "MY",
                "MZ",
                "NA",
                "NC",
                "NE",
                "NF",
                "NG",
                "NI",
                "NL",
                "NO",
                "NP",
                "NR",
                "NU",
                "NZ",
                "OM",
                "PA",
                "PE",
                "PF",
                "PG",
                "PH",
                "PK",
                "PL",
                "PM",
                "PN",
                "PR",
                "PS",
                "PT",
                "PW",
                "PY",
                "QA",
                "RE",
                "RO",
                "RS",
                "RU",
                "RW",
                "SA",
                "SB",
                "SC",
                "SD",
                "SE",
                "SG",
                "SH",
                "SI",
                "SJ",
                "SK",
                "SL",
                "SM",
                "SN",
                "SO",
                "SR",
                "SS",
                "ST",
                "SV",
                "SX",
                "SY",
                "SZ",
                "TC",
                "TD",
                "TF",
                "TG",
                "TH",
                "TJ",
                "TK",
                "TL",
                "TM",
                "TN",
                "TO",
                "TR",
                "TT",
                "TV",
                "TW",
                "TZ",
                "UA",
                "UG",
                "UM",
                "US",
                "UY",
                "UZ",
                "VA",
                "VC",
                "VE",
                "VG",
                "VI",
                "VN",
                "VU",
                "WF",
                "WS",
                "YE",
                "YT",
                "ZA",
                "ZM",
                "ZW",
                "4",
                "8",
                "12",
                "20",
                "24",
                "28",
                "31",
                "32",
                "36",
                "40",
                "44",
                "48",
                "50",
                "51",
                "52",
                "56",
                "64",
                "68",
                "70",
                "72",
                "76",
                "84",
                "90",
                "96",
                "100",
                "104",
                "108",
                "112",
                "116",
                "120",
                "124",
                "132",
                "140",
                "144",
                "148",
                "152",
                "156",
                "158",
                "170",
                "174",
                "178",
                "180",
                "188",
                "191",
                "192",
                "196",
                "200",
                "203",
                "204",
                "208",
                "212",
                "214",
                "218",
                "222",
                "226",
                "230",
                "231",
                "232",
                "233",
                "242",
                "246",
                "250",
                "258",
                "262",
                "266",
                "268",
                "270",
                "276",
                "278",
                "280",
                "288",
                "296",
                "300",
                "308",
                "320",
                "324",
                "328",
                "332",
                "336",
                "340",
                "344",
                "348",
                "352",
                "356",
                "360",
                "364",
                "368",
                "372",
                "376",
                "380",
                "384",
                "388",
                "392",
                "398",
                "400",
                "404",
                "408",
                "410",
                "414",
                "417",
                "418",
                "422",
                "426",
                "428",
                "430",
                "434",
                "438",
                "440",
                "442",
                "446",
                "450",
                "454",
                "458",
                "462",
                "466",
                "470",
                "478",
                "480",
                "484",
                "492",
                "496",
                "498",
                "499",
                "504",
                "508",
                "512",
                "516",
                "520",
                "524",
                "528",
                "540",
                "548",
                "554",
                "558",
                "562",
                "566",
                "578",
                "583",
                "584",
                "585",
                "586",
                "591",
                "598",
                "600",
                "604",
                "608",
                "616",
                "620",
                "624",
                "626",
                "630",
                "634",
                "642",
                "643",
                "646",
                "659",
                "662",
                "670",
                "674",
                "678",
                "682",
                "686",
                "688",
                "690",
                "694",
                "702",
                "703",
                "704",
                "705",
                "706",
                "710",
                "716",
                "720",
                "724",
                "728",
                "729",
                "732",
                "736",
                "740",
                "748",
                "752",
                "756",
                "760",
                "762",
                "764",
                "768",
                "776",
                "780",
                "784",
                "788",
                "792",
                "795",
                "798",
                "800",
                "804",
                "807",
                "810",
                "818",
                "826",
                "834",
                "840",
                "854",
                "858",
                "860",
                "862",
                "882",
                "886",
                "887",
                "890",
                "891",
                "894",
                "2100",
                "2101",
                "2102",
                "2103",
                "2104",
                "2105",
                "2106",
                "2107",
                "2108",
                "2109",
                "2110",
                "2111",
                "2112",
                "2113",
                "2114",
                "2115",
                "2116",
                "2117",
                "2118",
                "2119",
                "2120",
                "2121",
                "2122",
                "2123",
                "2124",
                "2125",
                "2126",
                "2127",
                "2128",
                "2129",
                "2130",
                "2131",
                "2132",
                "2133",
                "2134",
                "2136",
                "XK",
                "Worldwide",
            ])
        });

        Some(VALID_VALUES.contains(value))
    }
}

#[allow(dead_code)]
pub struct DdexIsoDateValidator;
impl Validator for DdexIsoDateValidator {
    fn regex_check() -> Option<&'static Regex> {
        Some(&DATE_RE)
    }
}

#[allow(dead_code)]
pub struct AvsAllTerritoryCodeValidator;
impl Validator for AvsAllTerritoryCodeValidator {
    fn bool_check(value: &str) -> Option<bool> {
        static VALID_VALUES: once_cell::sync::Lazy<HashSet<&'static str>> =
            once_cell::sync::Lazy::new(|| {
                HashSet::from([
                    "AD",
                    "AE",
                    "AF",
                    "AG",
                    "AI",
                    "AL",
                    "AM",
                    "AN",
                    "AO",
                    "AQ",
                    "AR",
                    "AS",
                    "AT",
                    "AU",
                    "AW",
                    "AX",
                    "AZ",
                    "BA",
                    "BB",
                    "BD",
                    "BE",
                    "BF",
                    "BG",
                    "BH",
                    "BI",
                    "BJ",
                    "BL",
                    "BM",
                    "BN",
                    "BO",
                    "BQ",
                    "BR",
                    "BS",
                    "BT",
                    "BV",
                    "BW",
                    "BY",
                    "BZ",
                    "CA",
                    "CC",
                    "CD",
                    "CF",
                    "CG",
                    "CH",
                    "CI",
                    "CK",
                    "CL",
                    "CM",
                    "CN",
                    "CO",
                    "CR",
                    "CS",
                    "CU",
                    "CV",
                    "CW",
                    "CX",
                    "CY",
                    "CZ",
                    "DE",
                    "DJ",
                    "DK",
                    "DM",
                    "DO",
                    "DZ",
                    "EC",
                    "EE",
                    "EG",
                    "EH",
                    "ER",
                    "ES",
                    "ES-CE",
                    "ES-CN",
                    "ES-ML",
                    "ET",
                    "FI",
                    "FJ",
                    "FK",
                    "FM",
                    "FO",
                    "FR",
                    "GA",
                    "GB",
                    "GD",
                    "GE",
                    "GF",
                    "GG",
                    "GH",
                    "GI",
                    "GL",
                    "GM",
                    "GN",
                    "GP",
                    "GQ",
                    "GR",
                    "GS",
                    "GT",
                    "GU",
                    "GW",
                    "GY",
                    "HK",
                    "HM",
                    "HN",
                    "HR",
                    "HT",
                    "HU",
                    "ID",
                    "IE",
                    "IL",
                    "IM",
                    "IN",
                    "IO",
                    "IQ",
                    "IR",
                    "IS",
                    "IT",
                    "JE",
                    "JM",
                    "JO",
                    "JP",
                    "KE",
                    "KG",
                    "KH",
                    "KI",
                    "KM",
                    "KN",
                    "KP",
                    "KR",
                    "KW",
                    "KY",
                    "KZ",
                    "LA",
                    "LB",
                    "LC",
                    "LI",
                    "LK",
                    "LR",
                    "LS",
                    "LT",
                    "LU",
                    "LV",
                    "LY",
                    "MA",
                    "MC",
                    "MD",
                    "ME",
                    "MF",
                    "MG",
                    "MH",
                    "MK",
                    "ML",
                    "MM",
                    "MN",
                    "MO",
                    "MP",
                    "MQ",
                    "MR",
                    "MS",
                    "MT",
                    "MU",
                    "MV",
                    "MW",
                    "MX",
                    "MY",
                    "MZ",
                    "NA",
                    "NC",
                    "NE",
                    "NF",
                    "NG",
                    "NI",
                    "NL",
                    "NO",
                    "NP",
                    "NR",
                    "NU",
                    "NZ",
                    "OM",
                    "PA",
                    "PE",
                    "PF",
                    "PG",
                    "PH",
                    "PK",
                    "PL",
                    "PM",
                    "PN",
                    "PR",
                    "PS",
                    "PT",
                    "PW",
                    "PY",
                    "QA",
                    "RE",
                    "RO",
                    "RS",
                    "RU",
                    "RW",
                    "SA",
                    "SB",
                    "SC",
                    "SD",
                    "SE",
                    "SG",
                    "SH",
                    "SI",
                    "SJ",
                    "SK",
                    "SL",
                    "SM",
                    "SN",
                    "SO",
                    "SR",
                    "SS",
                    "ST",
                    "SV",
                    "SX",
                    "SY",
                    "SZ",
                    "TC",
                    "TD",
                    "TF",
                    "TG",
                    "TH",
                    "TJ",
                    "TK",
                    "TL",
                    "TM",
                    "TN",
                    "TO",
                    "TR",
                    "TT",
                    "TV",
                    "TW",
                    "TZ",
                    "UA",
                    "UG",
                    "UM",
                    "US",
                    "UY",
                    "UZ",
                    "VA",
                    "VC",
                    "VE",
                    "VG",
                    "VI",
                    "VN",
                    "VU",
                    "WF",
                    "WS",
                    "YE",
                    "YT",
                    "ZA",
                    "ZM",
                    "ZW",
                    "XK",
                    "Worldwide",
                    "AIDJ",
                    "ANHH",
                    "BQAQ",
                    "BUMM",
                    "BYAA",
                    "CSHH",
                    "CSXX",
                    "CTKI",
                    "DDDE",
                    "DYBJ",
                    "FQHH",
                    "FXFR",
                    "GEHH",
                    "HVBF",
                    "JTUM",
                    "MIUM",
                    "NHVU",
                    "NQAQ",
                    "NTHH",
                    "PCHH",
                    "PUUM",
                    "PZPA",
                    "RHZW",
                    "SKIN",
                    "SUHH",
                    "TPTL",
                    "VDVN",
                    "WKUM",
                    "YDYE",
                    "YUCS",
                    "ZRCD",
                ])
            });

        Some(VALID_VALUES.contains(value))
    }
}

#[allow(dead_code)]
pub struct AvsRightsCoverageValidator;
impl Validator for AvsRightsCoverageValidator {
    fn bool_check(value: &str) -> Option<bool> {
        match value {
            "MakeAvailableRight"
            | "MechanicalRight"
            | "PerformingRight"
            | "PrintRight"
            | "ReproductionRight"
            | "SynchronizationRight"
            | "UserDefined" => Some(true),
            _ => Some(false),
        }
    }
}

#[allow(dead_code)]
pub struct DPIDValidator;
impl Validator for DPIDValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_padpida(value))
    }
}

#[allow(dead_code)]
pub struct ResourceReferenceValidator;
impl Validator for ResourceReferenceValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('A', value))
    }
}

#[allow(dead_code)]
pub struct AvsSoundRecordingTypeValidator;
impl Validator for AvsSoundRecordingTypeValidator {
    fn bool_check(value: &str) -> Option<bool> {
        match value {
            "AudioStem"
            | "Clip"
            | "MusicalWorkReadalongSoundRecording"
            | "MusicalWorkSoundRecording"
            | "NonMusicalWorkReadalongSoundRecording"
            | "NonMusicalWorkSoundRecording"
            | "SpokenWordSoundRecording"
            | "Unknown"
            | "UserDefined" => Some(true),
            _ => Some(false),
        }
    }
}

#[allow(dead_code)]
pub struct AvsEditionTypeValidator;
impl Validator for AvsEditionTypeValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(match value {
            "ImmersiveEdition" | "NonImmersiveEdition" => true,
            _ => false,
        })
    }
}

#[allow(dead_code)]
pub struct AvsRecordingModeValidator;
impl Validator for AvsRecordingModeValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(match value {
            "BinauralAudio" | "ImmersiveAudio" | "LCR" | "Mono" | "MultichannelAudio"
            | "MultiTrack" | "Quad" | "Stems" | "Stereo" | "SurroundSound" | "Unknown" => true,
            _ => false,
        })
    }
}

#[allow(dead_code)]
pub struct AvsContributorRoleValidator;
impl Validator for AvsContributorRoleValidator {
    fn bool_check(value: &str) -> Option<bool> {
        static VALID_VALUES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
            HashSet::from([
                "Adapter",
                "Architect",
                "Arranger",
                "Author",
                "AuthorInQuotations",
                "AuthorOfAfterword",
                "Compiler",
                "Composer",
                "ComposerLyricist",
                "Conceptor",
                "Creator",
                "DialogueAuthor",
                "Dissertant",
                "Engraver",
                "Etcher",
                "Journalist",
                "LandscapeArchitect",
                "Librettist",
                "Lithographer",
                "Lyricist",
                "MetalEngraver",
                "NonLyricAuthor",
                "PlateMaker",
                "Playwright",
                "Reporter",
                "Reviewer",
                "Rubricator",
                "ScreenplayAuthor",
                "Sculptor",
                "SubArranger",
                "SubLyricist",
                "Translator",
                "Woodcutter",
                "WoodEngraver",
                "WriterOfAccompanyingMaterial",
                "BookPublisher",
                "CopyrightClaimant",
                "CopyrightHolder",
                "MusicPublisher",
                "NewspaperPublisher",
                "OriginalPublisher",
                "PeriodicalPublisher",
                "SubPublisher",
                "SubstitutedPublisher",
                "Unknown",
                "UserDefined",
                "Accompanyist",
                "Actor",
                "AdditionalEngineer",
                "AdditionalMixingEngineer",
                "AdditionalPerformer",
                "AdditionalProgrammingEngineer",
                "AdditionalStudioProducer",
                "AnchorPerson",
                "AnimalTrainer",
                "Animator",
                "Annotator",
                "Announcer",
                "AAndRAdministrator",
                "AAndRCoordinator",
                "Armourer",
                "ArtCopyist",
                "ArtDirector",
                "Artist",
                "ArtistBackgroundVocalEngineer",
                "ArtistVocalEngineer",
                "ArtistVocalSecondEngineer",
                "AssistantCameraOperator",
                "AssistantChiefLightingTechnician",
                "AssistantConductor",
                "AssistantDirector",
                "AssistantEditor",
                "AssistantEngineer",
                "AssistantProducer",
                "AssistantVisualEditor",
                "AssociatedPerformer",
                "AssociateProducer",
                "AuralTrainer",
                "BackgroundVocalist",
                "BalanceEngineer",
                "BandLeader",
                "Binder",
                "BindingDesigner",
                "BookDesigner",
                "BookjackDesigner",
                "BookplateDesigner",
                "BookProducer",
                "BroadcastAssistant",
                "BroadcastJournalist",
                "Calligrapher",
                "CameraOperator",
                "Carpenter",
                "Cartographer",
                "Cartoonist",
                "CastingDirector",
                "Causeur",
                "Censor",
                "ChiefLightingTechnician",
                "Choir",
                "ChoirMember",
                "Choreographer",
                "ChorusMaster",
                "CircusArtist",
                "ClapperLoader",
                "ClubDJ",
                "CoDirector",
                "CoExecutiveProducer",
                "ColorSeparator",
                "Comedian",
                "CoMixer",
                "CoMixingEngineer",
                "Commentator",
                "CommissioningBroadcaster",
                "CompilationProducer",
                "ComputerGraphicCreator",
                "ComputerProgrammer",
                "ConcertMaster",
                "Conductor",
                "Consultant",
                "ContinuityChecker",
                "Contractor",
                "CoProducer",
                "Correspondent",
                "CostumeDesigner",
                "CoverDesigner",
                "Dancer",
                "Delineator",
                "Designer",
                "DialogueCoach",
                "DialogueDirector",
                "DigitalAudioWorkstationEngineer",
                "DigitalEditingEngineer",
                "DigitalEditingSecondEngineer",
                "Director",
                "DirectStreamDigitalEngineer",
                "DistributionCompany",
                "DJ",
                "Draughtsman",
                "Dresser",
                "Dubber",
                "Editor",
                "EditorInChief",
                "EditorOfTheDay",
                "Encoder",
                "Engineer",
                "Ensemble",
                "ExecutiveProducer",
                "Expert",
                "Facsimilist",
                "FightDirector",
                "FilmDirector",
                "FilmDistributor",
                "FilmEditor",
                "FilmProducer",
                "FilmSoundEngineer",
                "FloorManager",
                "FocusPuller",
                "FoleyArtist",
                "FoleyEditor",
                "FoleyMixer",
                "GraphicArtist",
                "GraphicAssistant",
                "GraphicDesigner",
                "Greensman",
                "Grip",
                "GuestConductor",
                "GroupMember",
                "Hairdresser",
                "Illustrator",
                "ImmersiveMasteringEngineer",
                "ImmersiveMixingEngineer",
                "InitialProducer",
                "InterviewedGuest",
                "Interviewer",
                "KeyCharacter",
                "KeyGrip",
                "KeyTalent",
                "Leadman",
                "LeadPerformer",
                "LeadVocalist",
                "LightingDirector",
                "LightingTechnician",
                "LocationManager",
                "MakeUpArtist",
                "Manufacturer",
                "MasteringEngineer",
                "MasteringSecondEngineer",
                "MatteArtist",
                "Mixer",
                "MixingEngineer",
                "MixingSecondEngineer",
                "MusicArranger",
                "MusicCopyist",
                "MusicDirector",
                "MusicGroup",
                "Musician",
                "Narrator",
                "NewsProducer",
                "NewsReader",
                "NotSpecified",
                "Orchestra",
                "OrchestraMember",
                "OriginalArtist",
                "OverdubEngineer",
                "OverdubSecondEngineer",
                "Painter",
                "Performer",
                "Photographer",
                "PhotographyDirector",
                "PlaybackSinger",
                "PostProducer",
                "PreProduction",
                "PreProductionEngineer",
                "PreProductionSecondEngineer",
                "Presenter",
                "PrimaryMusician",
                "ProductionAssistant",
                "ProductionCompany",
                "ProductionCoordinator",
                "ProductionDepartment",
                "ProductionManager",
                "ProductionSecretary",
                "ProjectEngineer",
                "Programmer",
                "ProgrammingEngineer",
                "ProgramProducer",
                "PropertyManager",
                "PublishingDirector",
                "Puppeteer",
                "Pyrotechnician",
                "RecordingEngineer",
                "RecordingSecondEngineer",
                "Redactor",
                "ReissueProducer",
                "RemixedArtist",
                "Remixer",
                "RemixingEngineer",
                "RemixingSecondEngineer",
                "Repetiteur",
                "Researcher",
                "ResearchTeamHead",
                "ResearchTeamMember",
                "Restager",
                "Rigger",
                "RightsControllerOnProduct",
                "Runner",
                "ScenicOperative",
                "ScientificAdvisor",
                "ScriptSupervisor",
                "SecondAssistantCameraOperator",
                "SecondAssistantDirector",
                "SecondConductor",
                "SecondEngineer",
                "SecondUnitDirector",
                "SeriesProducer",
                "SetDesigner",
                "SetDresser",
                "SignLanguageInterpreter",
                "Soloist",
                "SoundDesigner",
                "SoundMixer",
                "SoundRecordist",
                "SoundSupervisor",
                "Speaker",
                "SpecialEffectsTechnician",
                "Sponsor",
                "StageAssistantEngineer",
                "StageDirector",
                "StageEngineer",
                "StoryTeller",
                "StringEngineer",
                "StringProducer",
                "StringsDirector",
                "StudioConductor",
                "StudioMusician",
                "StudioPersonnel",
                "StudioProducer",
                "Stunts",
                "SubtitlesEditor",
                "SubtitlesTranslator",
                "SupportingActor",
                "SurroundMixingEngineer",
                "SurroundMixingSecondEngineer",
                "TapeOperator",
                "TechnicalDirector",
                "Tonmeister",
                "TrackingEngineer",
                "TrackingSecondEngineer",
                "TransfersAndSafetiesEngineer",
                "TransfersAndSafetiesSecondEngineer",
                "TransportationManager",
                "Treatment/ProgramProposal",
                "TypeDesigner",
                "VideoDirector",
                "Videographer",
                "VideoMusicalDirector",
                "VideoProducer",
                "VisionMixer",
                "VisualEditor",
                "VisualEffectsTechnician",
                "VocalArranger",
                "VocalEditingEngineer",
                "VocalEditingSecondEngineer",
                "VocalEngineer",
                "Vocalist",
                "VocalSecondEngineer",
                "VocalProducer",
                "VoiceActor",
                "Wardrobe",
            ])
        });

        Some(VALID_VALUES.contains(value))
    }
}

#[allow(dead_code)]
pub struct DisplayCreditPartyValidator;
impl Validator for DisplayCreditPartyValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('P', value))
    }
}

#[allow(dead_code)]
pub struct TechnicalResourceDetailsReferenceValidator;
impl Validator for TechnicalResourceDetailsReferenceValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('T', value))
    }
}

#[allow(dead_code)]
pub struct TypeValidator;
impl Validator for TypeValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(matches!(value, "AudioFile"))
    }
}

#[allow(dead_code)]
pub struct AvsFingerprintAlgorithmTypeValidator;
impl Validator for AvsFingerprintAlgorithmTypeValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(matches!(value, "UserDefined"))
    }
}

#[allow(dead_code)]
pub struct AvsRecordingFormatValidator;
impl Validator for AvsRecordingFormatValidator {
    fn bool_check(value: &str) -> Option<bool> {
        static VALID_VALUES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
            HashSet::from([
                "360Video",
                "Acoustic",
                "AdultContent",
                "AdvertisementVideo",
                "AdviceMagazine",
                "Animation",
                "AwardShow",
                "BalletVideo",
                "BehindTheMusic",
                "BehindTheScenes",
                "BlackAndWhiteVideo",
                "CauseRelatedRecording",
                "ChildrensFilm",
                "ColorizedVideo",
                "ColumnVideo",
                "ConcertClip",
                "ConcertVideo",
                "ContentProviderOriginals",
                "CorporateFilm",
                "Credits",
                "DanceVideo",
                "Documentary",
                "Drama",
                "DramaticoMusicalVideo",
                "EducationalVideo",
                "Episode",
                "FeatureFilm",
                "Fiction",
                "InfomercialVideo",
                "InteractiveResource",
                "Interview",
                "Karaoke",
                "LiveEventRecording",
                "LiveEventRecordingInStudio",
                "LiveEventVideo",
                "LiveStream",
                "LowComplexityVideo",
                "LyricVideo",
                "Magazine",
                "Menu",
                "MultimediaVideo",
                "MusicalWorkClip",
                "MusicalWorkReadalongVideo",
                "MusicalWorkTrailer",
                "MusicalWorkVideoChapter",
                "News",
                "NonMusicalWorkClip",
                "NonMusicalWorkReadalongVideo",
                "NonMusicalWorkTrailer",
                "NonMusicalWorkVideoChapter",
                "NonSerialAudioVisualRecording",
                "OperaVideo",
                "Performance",
                "RawFootage",
                "ReadalongVideo",
                "RealityTvShowVideo",
                "Excerpt",
                "Season",
                "SerialAudioVisualRecording",
                "Series",
                "Session",
                "ShortFilm",
                "SilentVideo",
                "SketchVideo",
                "SoapSitcom",
                "SpecialEvent",
                "Sport",
                "StaticVideo",
                "StudioRecording",
                "TheatricalWorkVideo",
                "TourDiary",
                "TrailerVideo",
                "Tutorial",
                "TvFilm",
                "TvFilmPerformance",
                "TvProgram",
                "TvShowVideo",
                "Unknown",
                "UserDefined",
                "VerticalVideo",
                "VideoChapter",
                "VideoClip",
                "VideoReport",
                "VideoStem",
                "VirtualRealityExperience",
                "Visualizer",
                "Vlog",
                "Webisode",
                "WebResource",
            ])
        });

        Some(VALID_VALUES.contains(value))
    }
}

#[allow(dead_code)]
pub struct AvsVersionTypeValidator;
impl Validator for AvsVersionTypeValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(match value {
            "ACappellaVersion"
            | "AlbumVersion"
            | "AlternativeVersion"
            | "CleanVersion"
            | "DemoVersion"
            | "EditedVersion"
            | "InstrumentalVersion"
            | "KaraokeVersion"
            | "LiveVersion"
            | "MixVersion"
            | "MonoVersion"
            | "RadioVersion"
            | "RemixVersion"
            | "SessionVersion"
            | "SingleVersion"
            | "StereoVersion"
            | "UserDefined" => true,
            _ => false,
        })
    }
}

#[allow(dead_code)]
pub struct AvsDisplayArtistRoleValidator;
impl Validator for AvsDisplayArtistRoleValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(match value {
            "Artist" | "Brand" | "Composer" | "FeaturedArtist" | "MainArtist" | "UserDefined" => {
                true
            }
            _ => false,
        })
    }
}

#[allow(dead_code)]
pub struct CharacterPartyReferenceValidator;
impl Validator for CharacterPartyReferenceValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('P', value))
    }
}

#[allow(dead_code)]
pub struct RightsControllerPartyReferenceValidator;
impl Validator for RightsControllerPartyReferenceValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('P', value))
    }
}

#[allow(dead_code)]
pub struct AvsRightsControllerRoleValidator;
impl Validator for AvsRightsControllerRoleValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(match value {
            "AdministratingRecordCompany"
            | "LocalPayee"
            | "RightsAdministrator"
            | "RightsController"
            | "RightsHolder"
            | "RoyaltyAdministrator"
            | "Unknown" => true,
            _ => false,
        })
    }
}

#[allow(dead_code)]
pub struct AvsUseTypeERNValidator;
impl Validator for AvsUseTypeERNValidator {
    fn bool_check(value: &str) -> Option<bool> {
        static VALID_VALUES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
            HashSet::from([
                "Broadcast",
                "Cable",
                "ConditionalDownload",
                "ContentInfluencedStream",
                "Display",
                "Download",
                "Dub",
                "DubForOnDemandStreaming",
                "DubForLivePerformance",
                "DubForMovies",
                "DubForMusicOnHold",
                "DubForPublicPerformance",
                "DubForRadio",
                "DubForTV",
                "ExtractForInternet",
                "KioskDownload",
                "Narrowcast",
                "NonInteractiveStream",
                "OnDemandStream",
                "Perform",
                "PerformAsMusicOnHold",
                "PerformInLivePerformance",
                "PerformInPublic",
                "PermanentDownload",
                "Playback",
                "PlayInPublic",
                "Podcast",
                "Print",
                "PrivateCopy",
                "PurchaseAsPhysicalProduct",
                "Rent",
                "Simulcast",
                "Stream",
                "TetheredDownload",
                "TimeInfluencedStream",
                "Use",
                "UseAsAlertTone",
                "UseAsDevice",
                "UseAsKaraoke",
                "UseAsRingbackTone",
                "UseAsRingbackTune",
                "UseAsRingtone",
                "UseAsRingtune",
                "UseAsScreensaver",
                "UseAsVoiceMail",
                "UseAsWallpaper",
                "UseForGenerativeAI",
                "UseForIdentification",
                "UseInMobilePhoneMessaging",
                "UseInPhoneListening",
                "UserDefined",
                "UserMakeAvailableLabelProvided",
                "UserMakeAvailableUserProvided",
                "Webcast",
            ])
        });

        Some(VALID_VALUES.contains(value))
    }
}

#[allow(dead_code)]
pub struct AvsParentalWarningTypeValidator;
impl Validator for AvsParentalWarningTypeValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(match value {
            "Explicit"
            | "ExplicitContentEdited"
            | "NoAdviceAvailable"
            | "NotExplicit"
            | "Unknown"
            | "UserDefined" => true,
            _ => false,
        })
    }
}

#[allow(dead_code)]
pub struct DdexLanguageAndScriptCodeWithRestrictionValidator;
impl Validator for DdexLanguageAndScriptCodeWithRestrictionValidator {
    fn regex_check() -> Option<&'static Regex> {
        Some(&LANGUAGE_RE)
    }
}

#[allow(dead_code)]
pub struct AvsImageTypeValidator;
impl Validator for AvsImageTypeValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(match value {
            "BackCoverImage" | "BookletBackImage" | "BookletFrontImage" | "DocumentImage"
            | "FrontCoverImage" | "Icon" | "Logo" | "Photograph" | "Portrait" | "Poster"
            | "ProfilePicture" | "SocialBannerImage" | "TrayImage" | "Unknown" | "UserDefined"
            | "VideoScreenCapture" | "Wallpaper" => true,
            _ => false,
        })
    }
}

#[allow(dead_code)]
pub struct ReleaseReferenceValidator;
impl Validator for ReleaseReferenceValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('R', value))
    }
}

#[allow(dead_code)]
pub struct AvsReleaseTypeERN4Validator;
impl Validator for AvsReleaseTypeERN4Validator {
    fn bool_check(value: &str) -> Option<bool> {
        static VALID_VALUES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
            HashSet::from([
                "Album",
                "AlertToneRelease",
                "AsPerContract",
                "AudioBookRelease",
                "AudioDramaRelease",
                "BackCoverImageRelease",
                "BookletBackImageRelease",
                "BookletFrontImageRelease",
                "BookletRelease",
                "Bundle",
                "ClassicalAlbum",
                "ClassicalDigitalBoxedSet",
                "ClassicalMultimediaAlbum",
                "ConcertVideo",
                "DigitalBoxSetRelease",
                "DjMix",
                "Documentary",
                "Drama",
                "DramaticoMusicalVideoRelease",
                "EBookRelease",
                "EP",
                "Episode",
                "FeatureFilm",
                "KaraokeRelease",
                "LiveEventVideo",
                "LogoRelease",
                "LongFormMusicalWorkVideoRelease",
                "LongFormNonMusicalWorkVideoRelease",
                "LyricSheetRelease",
                "MultimediaAlbum",
                "MultimediaDigitalBoxedSet",
                "MultimediaSingle",
                "MusicalWorkBasedGameRelease",
                "NonMusicalWorkBasedGameRelease",
                "PlayList",
                "RingbackToneRelease",
                "RingtoneRelease",
                "Season",
                "Series",
                "SheetMusicRelease",
                "ShortFilm",
                "Single",
                "SingleResourceRelease",
                "StemBundle",
                "UserDefined",
                "VideoAlbum",
                "VideoMastertoneRelease",
                "VideoSingle",
                "WallpaperRelease",
            ])
        });
        Some(VALID_VALUES.contains(value))
    }
}

#[allow(dead_code)]
pub struct DdexLocalPartyAnchorReferenceValidator;
impl Validator for DdexLocalPartyAnchorReferenceValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('P', value))
    }
}

#[allow(dead_code)]
pub struct ReleaseResourceReferenceValidator;
impl Validator for ReleaseResourceReferenceValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('A', value))
    }
}

#[allow(dead_code)]
pub struct ArtistPartyReferenceValidator;
impl Validator for ArtistPartyReferenceValidator {
    fn bool_check(value: &str) -> Option<bool> {
        Some(is_valid_reference('P', value))
    }
}

pub struct ChoiceValidator {}

impl ChoiceValidator {
    pub fn affiliation(
        company_name: &Option<String>,
        party_affiliate_reference: &Option<String>,
    ) -> Result<(), Error> {
        match (company_name.is_some(), party_affiliate_reference.is_some()) {
            (true, true) => Err(Error::Custom(
                "CompanyName and PartyAffiliateReference are exclusive".to_string(),
            )),
            (false, false) => Err(Error::Custom(
                "CompanyName or PartyAffiliateReference is required".to_string(),
            )),
            _ => Ok(()),
        }
    }

    pub fn resource_rights_controller(
        right_share_percentage: &Option<String>,
        right_share_unknown: &Option<bool>,
    ) -> Result<(), Error> {
        match (right_share_percentage, right_share_unknown) {
            (Some(_), Some(_)) => Err(Error::Custom(
                "RightSharePercentage and RightShareUnknown are exclusive".to_string(),
            )),
            (None, None) => Err(Error::Custom(
                "RightSharePercentage or RightShareUnknown is required".to_string(),
            )),
            (Some(percentage), None) => {
                let parsed = percentage.parse::<f64>();

                if let Ok(parsed_ok) = parsed {
                    if parsed_ok > 100 as f64 && parsed_ok < 0 as f64 {
                        return Err(Error::Custom(
                            "RightSharePercentage needs to be between 0 and 100".to_string(),
                        ));
                    }
                } else {
                    return Err(Error::Custom(
                        "Unable to parse RightSharePercentage".to_string(),
                    ));
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn party<G, T>(parties_ids: &Vec<G>, parties_names: &Vec<T>) -> Result<(), Error> {
        if parties_names.len() == 0 && parties_ids.len() == 0 {
            return Err(Error::Custom(
                "PartyName and/or PartyId is required".to_string(),
            ));
        }

        Ok(())
    }
}

pub struct ProtocolValidator {}

impl ProtocolValidator {
    pub fn affiliation_type(kind: &String) -> Result<(), Error> {
        let allowed_kind = "MusicLicensingCompany";

        if kind != allowed_kind {
            return Err(Error::Custom(format!(
                "Protocol Check 4 failed: PartyList:Party:Affiliation:Type has to be {}",
                allowed_kind
            )));
        }

        Ok(())
    }

    pub fn right_types(rights: &Vec<RightsType>) -> Result<(), Error> {
        let allowed_type = "MakeAvailableRight";

        if rights.iter().any(|e| e.content == allowed_type) {
            return Ok(());
        } else {
            return Err(Error::Custom(format!(
                "Protocol Check 5 failed: PartyList:Party:Affiliation:RightType has to include {}",
                allowed_type
            )));
        };
    }

    pub fn sound_recordings(sound_recordings: &Vec<SoundRecording>) -> Result<(), Error> {
        let mut isrcs_set = HashSet::new();
        let mut duplicate_isrc = false;
        let mut duplicate_type = false;

        sound_recordings.iter().for_each(|recording| {
            let mut types_set = HashSet::new();

            recording
                .sound_recording_editions
                .iter()
                .for_each(|edition| {
                    if !isrcs_set.insert(&edition.resource_ids[0].isrc) {
                        duplicate_isrc = true;
                    }

                    if let Some(kind) = &edition.kind {
                        if !types_set.insert(kind) {
                            duplicate_type = true;
                        }
                    }
                });
        });

        if duplicate_isrc {
            return Err(Error::Custom(format!(
                "Protocol Check 6 failed: ISRCs need to be unique"
            )));
        }

        if duplicate_type {
            return Err(Error::Custom(format!(
                "Protocol Check 6 failed: SoundRecordingEdition::SoundRecordingTypes within SoundRecording must be unique"
            )));
        }

        Ok(())
    }

    pub fn rights_control_types(types: &Vec<String>) -> Result<(), Error> {
        let expected_types = [
            "RightsAdministrator".to_string(),
            "RightsController".to_string(),
        ];

        if !expected_types.iter().any(|n| types.contains(n)) {
            return Err(Error::Custom(format!(
                "Protocol Check 8 failed: RightsControlTypes need to contain at lease one of {}",
                expected_types.join("/")
            )));
        }

        Ok(())
    }

    pub fn use_types(types: &Vec<UseType>) -> Result<(), Error> {
        let accepted_values = [
            "Stream",
            "PermanentDownload",
            "ConditionalDownload",
            "TetheredDownload",
        ];
        if !types
            .iter()
            .any(|t| accepted_values.contains(&t.content.as_str()))
        {
            return Err(Error::Custom(format!(
                "Protocol Check 10 failed: TerritoryOfRightsDelegation needs to contain at least one of Stream/PermanentDownload/ConditionalDownload/TetheredDownload"
            )));
        }

        Ok(())
    }

    pub fn release_id(release_id: &ReleaseId) -> Result<(), Error> {
        if let (None, None, true) = (
            &release_id.g_rid,
            &release_id.icpn,
            release_id.proprietary_ids.is_empty(),
        ) {
            return Err(Error::Custom(format!(
                "Protocol Check 11 failed: ReleaseId must specify at least one of grid/icpn/proprietary_id"
            )));
        }

        if let Some(icpn) = &release_id.icpn {
            if matches!(icpn.len(), 8 | 12 | 13) && icpn.chars().all(|c| c.is_ascii_digit()) {
                return Ok(());
            } else {
                return Err(Error::Custom(format!(
                    "Protocol Check 11 failed: ICPN must contain 8 or 13 digits for EAN or 12 digits for UPC"
                )));
            }
        }

        Ok(())
    }
}
