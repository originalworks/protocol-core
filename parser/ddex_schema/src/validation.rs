use regex::Regex;

#[allow(dead_code)]
pub struct LanguageAndScriptCodeValidator;
impl LanguageAndScriptCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^[a-zA-Z]{2,3}(-[a-zA-Z]+){0,1}(-[a-zA-Z]{2}|-[0-9]{3}){0,1}(-[a-zA-Z][a-zA-Z0-9]{4}[a-zA-Z0-9]*){0,1}$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct RecordCompanyPartyReferenceValidator;
impl RecordCompanyPartyReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^P[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct TypeValidator;
impl TypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AudioFile$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct ChapterReferenceValidator;
impl ChapterReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^X[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct RepresentativeImageReferenceValidator;
impl RepresentativeImageReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^A[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct CharacterPartyReferenceValidator;
impl CharacterPartyReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^P[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct TopLeftCornerValidator;
impl TopLeftCornerValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^[0-9]+,[0-9]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct BottomRightCornerValidator;
impl BottomRightCornerValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^[0-9]+,[0-9]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct ReleaseReferenceValidator;
impl ReleaseReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^R[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct ReleaseResourceReferenceValidator;
impl ReleaseResourceReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^A[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct ContributorPartyReferenceValidator;
impl ContributorPartyReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^P[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct CueSheetReferenceValidator;
impl CueSheetReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Q[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct DealResourceReferenceValidator;
impl DealResourceReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^A[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct DealTechnicalResourceDetailsReferenceValidator;
impl DealTechnicalResourceDetailsReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^T[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct ArtistPartyReferenceValidator;
impl ArtistPartyReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^P[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct ResourceReferenceValidator;
impl ResourceReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^A[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct PartyReferenceValidator;
impl PartyReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^P[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct DPIDValidator;
impl DPIDValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^PADPIDA[a-zA-Z0-9]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct IpiNameNumberValidator;
impl IpiNameNumberValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^[0-9]{11}$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct ResourceRelatedResourceReferenceValidator;
impl ResourceRelatedResourceReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^A[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct ReleaseVisibilityReferenceValidator;
impl ReleaseVisibilityReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^V[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct DealReleaseReferenceValidator;
impl DealReleaseReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^R[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct VisibilityReferenceValidator;
impl VisibilityReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^V[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct ResourceGroupReleaseReferenceValidator;
impl ResourceGroupReleaseReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^R[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct RightsControllerPartyReferenceValidator;
impl RightsControllerPartyReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^P[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct LanguageOfLyricsValidator;
impl LanguageOfLyricsValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^[a-zA-Z]{2,3}(-[a-zA-Z]+){0,1}(-[a-zA-Z]{2}|-[0-9]{3}){0,1}(-[a-zA-Z][a-zA-Z0-9]{4}[a-zA-Z0-9]*){0,1}$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AudioChapterReferenceValidator;
impl AudioChapterReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^X[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct TechnicalResourceDetailsReferenceValidator;
impl TechnicalResourceDetailsReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^T[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct LanguageOfDubbingValidator;
impl LanguageOfDubbingValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^[a-zA-Z]{2,3}(-[a-zA-Z]+){0,1}(-[a-zA-Z]{2}|-[0-9]{3}){0,1}(-[a-zA-Z][a-zA-Z0-9]{4}[a-zA-Z0-9]*){0,1}$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct SubTitleLanguageValidator;
impl SubTitleLanguageValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^[a-zA-Z]{2,3}(-[a-zA-Z]+){0,1}(-[a-zA-Z]{2}|-[0-9]{3}){0,1}(-[a-zA-Z][a-zA-Z0-9]{4}[a-zA-Z0-9]*){0,1}$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct VideoChapterReferenceValidator;
impl VideoChapterReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^X[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct VideoCueSheetReferenceValidator;
impl VideoCueSheetReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Q[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct PartyAffiliateReferenceValidator;
impl PartyAffiliateReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^P[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct DisplayCreditPartyValidator;
impl DisplayCreditPartyValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^P[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct ResourceReleaseReferenceValidator;
impl ResourceReleaseReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^R[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct PartyIdValidator;
impl PartyIdValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^PADPIDA[a-zA-Z0-9]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct PartyRelatedPartyReferenceValidator;
impl PartyRelatedPartyReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^P[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct ResourceContainedResourceReferenceValidator;
impl ResourceContainedResourceReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^A[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct DdexIsoDateValidator;
impl DdexIsoDateValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^[0-9]{4}(-[0-9]{2}){0,1}(-[0-9]{2}){0,1}$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct DdexLanguageAndScriptCodeWithRestrictionValidator;
impl DdexLanguageAndScriptCodeWithRestrictionValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^[a-zA-Z]{2,3}(-[a-zA-Z]+){0,1}(-[a-zA-Z]{2}|-[0-9]{3}){0,1}(-[a-zA-Z][a-zA-Z0-9]{4}[a-zA-Z0-9]*){0,1}$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct DdexLocalPartyAnchorReferenceValidator;
impl DdexLocalPartyAnchorReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^P[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct DdexLocalResourceAnchorReferenceValidator;
impl DdexLocalResourceAnchorReferenceValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^A[\d\-_a-zA-Z]+$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsActivityValidator;
impl AvsActivityValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Afraid$|^Angst$|^BBQ$|^BibleStudy$|^BirthdayParty$|^Breakdown$|^Breakup$|^Breathe$|^Celebration$|^Cry$|^Dance$|^Dating$|^Daydream$|^Defeat$|^Dinner$|^Drink$|^Drive$|^Eat$|^Fight$|^Flirt$|^Focus$|^Funeral$|^HangOut$|^Honeymoon$|^Jump$|^Karaoke$|^Lazy$|^Leave$|^MakingLove$|^Meditation$|^Mourning$|^Party$|^Prayer$|^Regret$|^Relax$|^RoadTrip$|^Run$|^Travel$|^UserDefined$|^Victory$|^Wait$|^Waking$|^Walk$|^Wedding$|^Wish$|^Work$|^Workout$|^Worship$|^Yoga$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAdditionalContributorRoleValidator;
impl AvsAdditionalContributorRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Mime$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAdditionalRightsClaimStatusValidator;
impl AvsAdditionalRightsClaimStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Accepted$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAdditionalTitleTypeValidator;
impl AvsAdditionalTitleTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AlternativeTitle$|^FormalTitle$|^GroupingTitle$|^MusicalWorkTitle$|^OriginalTitle$|^TranslatedTitle$|^TransliteratedTitle$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAdditionalVideoTypeValidator;
impl AvsAdditionalVideoTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Drama$|^DramaticoMusicalVideo$|^InteractiveResource$|^ShortFormMusicalWorkVideo$|^ShortFormNonMusicalWorkVideo$|^UserDefined$|^WebResource$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAdministratingRecordCompanyRoleValidator;
impl AvsAdministratingRecordCompanyRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DesignatedDsrMessageRecipient$|^RightsAdministrator$|^RoyaltyAdministrator$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAffiliationTypeValidator;
impl AvsAffiliationTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^MusicLicensingCompany$|^MusicPublisher$|^MusicRightsSociety$|^RecordCompany$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAllTerritoryCodeValidator;
impl AvsAllTerritoryCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AD$|^AE$|^AF$|^AG$|^AI$|^AL$|^AM$|^AN$|^AO$|^AQ$|^AR$|^AS$|^AT$|^AU$|^AW$|^AX$|^AZ$|^BA$|^BB$|^BD$|^BE$|^BF$|^BG$|^BH$|^BI$|^BJ$|^BL$|^BM$|^BN$|^BO$|^BQ$|^BR$|^BS$|^BT$|^BV$|^BW$|^BY$|^BZ$|^CA$|^CC$|^CD$|^CF$|^CG$|^CH$|^CI$|^CK$|^CL$|^CM$|^CN$|^CO$|^CR$|^CS$|^CU$|^CV$|^CW$|^CX$|^CY$|^CZ$|^DE$|^DJ$|^DK$|^DM$|^DO$|^DZ$|^EC$|^EE$|^EG$|^EH$|^ER$|^ES$|^ES-CE$|^ES-CN$|^ES-ML$|^ET$|^FI$|^FJ$|^FK$|^FM$|^FO$|^FR$|^GA$|^GB$|^GD$|^GE$|^GF$|^GG$|^GH$|^GI$|^GL$|^GM$|^GN$|^GP$|^GQ$|^GR$|^GS$|^GT$|^GU$|^GW$|^GY$|^HK$|^HM$|^HN$|^HR$|^HT$|^HU$|^ID$|^IE$|^IL$|^IM$|^IN$|^IO$|^IQ$|^IR$|^IS$|^IT$|^JE$|^JM$|^JO$|^JP$|^KE$|^KG$|^KH$|^KI$|^KM$|^KN$|^KP$|^KR$|^KW$|^KY$|^KZ$|^LA$|^LB$|^LC$|^LI$|^LK$|^LR$|^LS$|^LT$|^LU$|^LV$|^LY$|^MA$|^MC$|^MD$|^ME$|^MF$|^MG$|^MH$|^MK$|^ML$|^MM$|^MN$|^MO$|^MP$|^MQ$|^MR$|^MS$|^MT$|^MU$|^MV$|^MW$|^MX$|^MY$|^MZ$|^NA$|^NC$|^NE$|^NF$|^NG$|^NI$|^NL$|^NO$|^NP$|^NR$|^NU$|^NZ$|^OM$|^PA$|^PE$|^PF$|^PG$|^PH$|^PK$|^PL$|^PM$|^PN$|^PR$|^PS$|^PT$|^PW$|^PY$|^QA$|^RE$|^RO$|^RS$|^RU$|^RW$|^SA$|^SB$|^SC$|^SD$|^SE$|^SG$|^SH$|^SI$|^SJ$|^SK$|^SL$|^SM$|^SN$|^SO$|^SR$|^SS$|^ST$|^SV$|^SX$|^SY$|^SZ$|^TC$|^TD$|^TF$|^TG$|^TH$|^TJ$|^TK$|^TL$|^TM$|^TN$|^TO$|^TR$|^TT$|^TV$|^TW$|^TZ$|^UA$|^UG$|^UM$|^US$|^UY$|^UZ$|^VA$|^VC$|^VE$|^VG$|^VI$|^VN$|^VU$|^WF$|^WS$|^YE$|^YT$|^ZA$|^ZM$|^ZW$|^4$|^8$|^12$|^20$|^24$|^28$|^31$|^32$|^36$|^40$|^44$|^48$|^50$|^51$|^52$|^56$|^64$|^68$|^70$|^72$|^76$|^84$|^90$|^96$|^100$|^104$|^108$|^112$|^116$|^120$|^124$|^132$|^140$|^144$|^148$|^152$|^156$|^158$|^170$|^174$|^178$|^180$|^188$|^191$|^192$|^196$|^200$|^203$|^204$|^208$|^212$|^214$|^218$|^222$|^226$|^230$|^231$|^232$|^233$|^242$|^246$|^250$|^258$|^262$|^266$|^268$|^270$|^276$|^278$|^280$|^288$|^296$|^300$|^308$|^320$|^324$|^328$|^332$|^336$|^340$|^344$|^348$|^352$|^356$|^360$|^364$|^368$|^372$|^376$|^380$|^384$|^388$|^392$|^398$|^400$|^404$|^408$|^410$|^414$|^417$|^418$|^422$|^426$|^428$|^430$|^434$|^438$|^440$|^442$|^446$|^450$|^454$|^458$|^462$|^466$|^470$|^478$|^480$|^484$|^492$|^496$|^498$|^499$|^504$|^508$|^512$|^516$|^520$|^524$|^528$|^540$|^548$|^554$|^558$|^562$|^566$|^578$|^583$|^584$|^585$|^586$|^591$|^598$|^600$|^604$|^608$|^616$|^620$|^624$|^626$|^630$|^634$|^642$|^643$|^646$|^659$|^662$|^670$|^674$|^678$|^682$|^686$|^688$|^690$|^694$|^702$|^703$|^704$|^705$|^706$|^710$|^716$|^720$|^724$|^728$|^729$|^732$|^736$|^740$|^748$|^752$|^756$|^760$|^762$|^764$|^768$|^776$|^780$|^784$|^788$|^792$|^795$|^798$|^800$|^804$|^807$|^810$|^818$|^826$|^834$|^840$|^854$|^858$|^860$|^862$|^882$|^886$|^887$|^890$|^891$|^894$|^2100$|^2101$|^2102$|^2103$|^2104$|^2105$|^2106$|^2107$|^2108$|^2109$|^2110$|^2111$|^2112$|^2113$|^2114$|^2115$|^2116$|^2117$|^2118$|^2119$|^2120$|^2121$|^2122$|^2123$|^2124$|^2125$|^2126$|^2127$|^2128$|^2129$|^2130$|^2131$|^2132$|^2133$|^2134$|^2136$|^XK$|^Worldwide$|^AIDJ$|^ANHH$|^BQAQ$|^BUMM$|^BYAA$|^CSHH$|^CSXX$|^CTKI$|^DDDE$|^DYBJ$|^FQHH$|^FXFR$|^GEHH$|^HVBF$|^JTUM$|^MIUM$|^NHVU$|^NQAQ$|^NTHH$|^PCHH$|^PUUM$|^PZPA$|^RHZW$|^SKIN$|^SUHH$|^TPTL$|^VDVN$|^WKUM$|^YDYE$|^YUCS$|^ZRCD$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAllTerritoryCodeNoWorldwideValidator;
impl AvsAllTerritoryCodeNoWorldwideValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AD$|^AE$|^AF$|^AG$|^AI$|^AL$|^AM$|^AN$|^AO$|^AQ$|^AR$|^AS$|^AT$|^AU$|^AW$|^AX$|^AZ$|^BA$|^BB$|^BD$|^BE$|^BF$|^BG$|^BH$|^BI$|^BJ$|^BL$|^BM$|^BN$|^BO$|^BQ$|^BR$|^BS$|^BT$|^BV$|^BW$|^BY$|^BZ$|^CA$|^CC$|^CD$|^CF$|^CG$|^CH$|^CI$|^CK$|^CL$|^CM$|^CN$|^CO$|^CR$|^CS$|^CU$|^CV$|^CW$|^CX$|^CY$|^CZ$|^DE$|^DJ$|^DK$|^DM$|^DO$|^DZ$|^EC$|^EE$|^EG$|^EH$|^ER$|^ES$|^ES-CE$|^ES-CN$|^ES-ML$|^ET$|^FI$|^FJ$|^FK$|^FM$|^FO$|^FR$|^GA$|^GB$|^GD$|^GE$|^GF$|^GG$|^GH$|^GI$|^GL$|^GM$|^GN$|^GP$|^GQ$|^GR$|^GS$|^GT$|^GU$|^GW$|^GY$|^HK$|^HM$|^HN$|^HR$|^HT$|^HU$|^ID$|^IE$|^IL$|^IM$|^IN$|^IO$|^IQ$|^IR$|^IS$|^IT$|^JE$|^JM$|^JO$|^JP$|^KE$|^KG$|^KH$|^KI$|^KM$|^KN$|^KP$|^KR$|^KW$|^KY$|^KZ$|^LA$|^LB$|^LC$|^LI$|^LK$|^LR$|^LS$|^LT$|^LU$|^LV$|^LY$|^MA$|^MC$|^MD$|^ME$|^MF$|^MG$|^MH$|^MK$|^ML$|^MM$|^MN$|^MO$|^MP$|^MQ$|^MR$|^MS$|^MT$|^MU$|^MV$|^MW$|^MX$|^MY$|^MZ$|^NA$|^NC$|^NE$|^NF$|^NG$|^NI$|^NL$|^NO$|^NP$|^NR$|^NU$|^NZ$|^OM$|^PA$|^PE$|^PF$|^PG$|^PH$|^PK$|^PL$|^PM$|^PN$|^PR$|^PS$|^PT$|^PW$|^PY$|^QA$|^RE$|^RO$|^RS$|^RU$|^RW$|^SA$|^SB$|^SC$|^SD$|^SE$|^SG$|^SH$|^SI$|^SJ$|^SK$|^SL$|^SM$|^SN$|^SO$|^SR$|^SS$|^ST$|^SV$|^SX$|^SY$|^SZ$|^TC$|^TD$|^TF$|^TG$|^TH$|^TJ$|^TK$|^TL$|^TM$|^TN$|^TO$|^TR$|^TT$|^TV$|^TW$|^TZ$|^UA$|^UG$|^UM$|^US$|^UY$|^UZ$|^VA$|^VC$|^VE$|^VG$|^VI$|^VN$|^VU$|^WF$|^WS$|^YE$|^YT$|^ZA$|^ZM$|^ZW$|^4$|^8$|^12$|^20$|^24$|^28$|^31$|^32$|^36$|^40$|^44$|^48$|^50$|^51$|^52$|^56$|^64$|^68$|^70$|^72$|^76$|^84$|^90$|^96$|^100$|^104$|^108$|^112$|^116$|^120$|^124$|^132$|^140$|^144$|^148$|^152$|^156$|^158$|^170$|^174$|^178$|^180$|^188$|^191$|^192$|^196$|^200$|^203$|^204$|^208$|^212$|^214$|^218$|^222$|^226$|^230$|^231$|^232$|^233$|^242$|^246$|^250$|^258$|^262$|^266$|^268$|^270$|^276$|^278$|^280$|^288$|^296$|^300$|^308$|^320$|^324$|^328$|^332$|^336$|^340$|^344$|^348$|^352$|^356$|^360$|^364$|^368$|^372$|^376$|^380$|^384$|^388$|^392$|^398$|^400$|^404$|^408$|^410$|^414$|^417$|^418$|^422$|^426$|^428$|^430$|^434$|^438$|^440$|^442$|^446$|^450$|^454$|^458$|^462$|^466$|^470$|^478$|^480$|^484$|^492$|^496$|^498$|^499$|^504$|^508$|^512$|^516$|^520$|^524$|^528$|^540$|^548$|^554$|^558$|^562$|^566$|^578$|^583$|^584$|^585$|^586$|^591$|^598$|^600$|^604$|^608$|^616$|^620$|^624$|^626$|^630$|^634$|^642$|^643$|^646$|^659$|^662$|^670$|^674$|^678$|^682$|^686$|^688$|^690$|^694$|^702$|^703$|^704$|^705$|^706$|^710$|^716$|^720$|^724$|^728$|^729$|^732$|^736$|^740$|^748$|^752$|^756$|^760$|^762$|^764$|^768$|^776$|^780$|^784$|^788$|^792$|^795$|^798$|^800$|^804$|^807$|^810$|^818$|^826$|^834$|^840$|^854$|^858$|^860$|^862$|^882$|^886$|^887$|^890$|^891$|^894$|^2100$|^2101$|^2102$|^2103$|^2104$|^2105$|^2106$|^2107$|^2108$|^2109$|^2110$|^2111$|^2112$|^2113$|^2114$|^2115$|^2116$|^2117$|^2118$|^2119$|^2120$|^2121$|^2122$|^2123$|^2124$|^2125$|^2126$|^2127$|^2128$|^2129$|^2130$|^2131$|^2132$|^2133$|^2134$|^2136$|^XK$|^AIDJ$|^ANHH$|^BQAQ$|^BUMM$|^BYAA$|^CSHH$|^CSXX$|^CTKI$|^DDDE$|^DYBJ$|^FQHH$|^FXFR$|^GEHH$|^HVBF$|^JTUM$|^MIUM$|^NHVU$|^NQAQ$|^NTHH$|^PCHH$|^PUUM$|^PZPA$|^RHZW$|^SKIN$|^SUHH$|^TPTL$|^VDVN$|^WKUM$|^YDYE$|^YUCS$|^ZRCD$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsArtistRoleValidator;
impl AvsArtistRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ArtCopyist$|^Calligrapher$|^Cartographer$|^Cartoonist$|^ComputerGraphicCreator$|^ComputerProgrammer$|^Delineator$|^Designer$|^Draughtsman$|^Facsimilist$|^GraphicArtist$|^Illustrator$|^MusicCopyist$|^NotSpecified$|^Painter$|^Photographer$|^TypeDesigner$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsArtistTypeValidator;
impl AvsArtistTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ACappellaEnsemble$|^BarbershopEnsemble$|^BigBand$|^BrassBand$|^ChamberOrchestra$|^CountryGroup$|^Duet$|^ElectronicGroup$|^FifeAndDrumCorps$|^FolkGroup$|^InstrumentAndAccompaniment$|^JazzCombo$|^LatinGroup$|^MarchingBand$|^MariachiBand$|^Orchestra$|^PianoEnsemble$|^PianoTrio$|^PianoQuartet$|^PianoQuintet$|^PipeAndDrumGroup$|^PopBand$|^ReggaeBand$|^RockBand$|^SoloInstrument$|^SoloVoice$|^StringEnsemble$|^StringQuartet$|^StringQuintet$|^Trio$|^UserDefined$|^VoiceAndAccompaniment$|^WindEnsemble$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAspectRatioTypeValidator;
impl AvsAspectRatioTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DAR$|^PAR$|^SAR$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAsserterTypeValidator;
impl AvsAsserterTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^CollectionSociety$|^InterestedPublisher$|^MusicLicensingCompany$|^Publisher$|^RecordCompanyWithInterestInResource$|^ThirdParty$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAssertionStatusValidator;
impl AvsAssertionStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Verified$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAudioCodecTypeValidator;
impl AvsAudioCodecTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AAC$|^AC-4$|^DolbyAtmosMasterADM$|^ADPCM$|^ALaw$|^AMR$|^AMR-NB$|^AMR-WB$|^DolbyDigitalPlus$|^FLAC$|^MP$|^MP2$|^MP3$|^MPEG-H3D$|^MQA$|^MuLaw$|^PCM$|^PDM$|^QCELP$|^RealAudio$|^Shockwave$|^Unknown$|^UserDefined$|^Vorbis$|^WMA$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsAudioVisualTypeValidator;
impl AvsAudioVisualTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AdultContent$|^AdvertisementVideo$|^AdviceMagazine$|^Animation$|^BalletVideo$|^BehindTheScenes$|^BlackAndWhiteVideo$|^ChildrensFilm$|^ColorizedVideo$|^ColumnVideo$|^ConcertClip$|^ConcertVideo$|^CorporateFilm$|^Credits$|^DramaticoMusicalVideo$|^Documentary$|^EducationalVideo$|^FeatureFilm$|^Fiction$|^InfomercialVideo$|^InteractiveResource$|^Interview$|^Karaoke$|^LiveEventVideo$|^LongFormMusicalWorkVideo$|^LongFormNonMusicalWorkVideo$|^LyricVideo$|^Magazine$|^Menu$|^MultimediaVideo$|^MusicalWorkClip$|^MusicalWorkReadalongVideo$|^MusicalWorkTrailer$|^MusicalWorkVideoChapter$|^News$|^NonMusicalWorkClip$|^NonMusicalWorkReadalongVideo$|^NonMusicalWorkTrailer$|^NonMusicalWorkVideoChapter$|^OperaVideo$|^Performance$|^ReadalongVideo$|^RealityTvShowVideo$|^ShortFilm$|^ShortFormMusicalWorkVideo$|^ShortFormNonMusicalWorkVideo$|^SilentVideo$|^SketchVideo$|^SoapSitcom$|^SpecialEvent$|^Sport$|^TheatricalWorkVideo$|^TrailerVideo$|^TvFilm$|^TvProgram$|^TvShowVideo$|^Unknown$|^UserDefined$|^VideoChapter$|^VideoClip$|^VideoReport$|^VideoStem$|^WebResource$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsBasisForRevenueAllocationValidator;
impl AvsBasisForRevenueAllocationValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^FullCensus$|^FullUsageLog$|^MarketShare$|^Proxy$|^SalesFigures$|^SampleCensus$|^SampleUsageLog$|^UnitMultipliedByDuration$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsBinaryDataTypeValidator;
impl AvsBinaryDataTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Binary64$|^HexBinary$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsBusinessMusicalWorkContributorRoleValidator;
impl AvsBusinessMusicalWorkContributorRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BookPublisher$|^CopyrightClaimant$|^CopyrightHolder$|^MusicPublisher$|^NewspaperPublisher$|^OriginalPublisher$|^PeriodicalPublisher$|^SubPublisher$|^SubstitutedPublisher$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCarrierTypeValidator;
impl AvsCarrierTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^12InchDiscoSingleRemix$|^33rpm10InchLP$|^33rpm10InchSingle$|^33rpm12InchLP$|^33rpm12InchLp20Tracks$|^33rpm12InchMaxiSingle$|^33rpm12InchSingle$|^33rpm7InchLP$|^33rpm7InchSingle$|^45rpm10InchLP$|^45rpm10InchMaxiSingle$|^45rpm10InchSingle$|^45rpm12InchLP$|^45rpm12InchMaxiSingle$|^45rpm12InchSingle$|^45rpm7InchEP$|^45rpm7InchSingle$|^7InchMaxiSingleRemix$|^BluRay$|^CD$|^CdCompilation$|^CdEp$|^CdEpEnhanced$|^CdExtraCompilation$|^CdExtraEP$|^CdExtraLP$|^CdExtraMaxiRemix$|^CdExtraMaxiSingle$|^CdExtraSingle$|^CdExtraSingle2Tracks$|^CdLp$|^CdLp5Inch$|^CdLpEnhanced$|^CdLpPlusCdVideo$|^CdLpPlusDvdAudio$|^CdLpPlusDvdVideo$|^CdLpPlusWeb$|^CdMaxiSingle$|^CdMaxiSingle3Inch$|^CdMaxiSingleEnhanced$|^CdMaxiSingleRemix$|^CdPlusCdBonus$|^CdPlusDvdBonus$|^CdRom$|^CdSingle$|^CdSingle3Inch$|^CdSingle5Inch$|^CdVideo5LpNTSC$|^CdVideo5LpPAL$|^CdVideoAudioCompatible$|^CombiPack$|^DCC$|^DccCompilation$|^DualDisc$|^DVD$|^DvdAudio$|^DvdAudio5MaxiSingle$|^DvdAudioLP$|^DvdAudioSingle$|^DvdRom$|^DvdSingle$|^DvdVideo$|^DvdVideo5MaxiSingleNTSC$|^DvdVideo5MaxiSinglePAL$|^DvdVideo5SingleNTSC$|^DvdVideo5SinglePAL$|^DvdVideoLpNTSC$|^DvdVideoLpPAL$|^DvdVideoLpPlusCdLpOrCdSingle$|^FanPack$|^FileSystem$|^HdDvdVideoLp$|^LaserDiscLp12InchNTSC$|^LpCompIdenticalToCdComp$|^LpCompilation$|^LpIdenticalToCD$|^MC$|^McCompIdenticalToCdComp$|^McCompilation$|^McDoubleLP$|^McEP$|^McIdenticalToCD$|^McLP$|^McMaxiSingle$|^McRemix$|^McSingle$|^McSingleIdenticalToCDS$|^MemoryDevice$|^MemoryDeviceAudioLP$|^MemoryDeviceMixLP$|^MemoryDeviceVideoLP$|^Merchandise$|^MiniDisc$|^MiniDiscCompilation$|^MiniDiscEP$|^MiniDiscMaxiRemix$|^MiniDiscSingleMaxiSingle$|^OnlineSystem$|^PrePaidCard$|^SACD$|^SacdCompilation$|^SacdLpStereo$|^SacdLpStereoCdAudio$|^SacdLpStereoSurround$|^SacdLpStereoSurroundCdAudio$|^SacdLpSurroundCdAudio$|^SacdPlusDvdVideo$|^UserDefined$|^VhsNTSC$|^VhsPAL$|^VhsPlusCdLp$|^VhsSECAM$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCatalogTransferAcknowledgementStatusValidator;
impl AvsCatalogTransferAcknowledgementStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Error$|^FileReceived$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCatalogTransferStatusValidator;
impl AvsCatalogTransferStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Confirmed$|^Pending$|^Rejected$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCatalogTransferTypeValidator;
impl AvsCatalogTransferTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^StandardCatalogTransfer$|^UsStatutoryReversion$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCdProtectionTypeValidator;
impl AvsCdProtectionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^CDS100$|^CDS200$|^CDS300$|^Key2Audio$|^MediaMaxCD3$|^NotProtected$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCharacterTypeValidator;
impl AvsCharacterTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^MainCharacter$|^OtherCharacter$|^SupportingCharacter$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsClaimBasisValidator;
impl AvsClaimBasisValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^CopCon$|^Direct$|^Unmatched$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsClaimStatusValidator;
impl AvsClaimStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^CompleteClaim$|^CompleteUnderClaim$|^IncompleteClaim$|^IncompleteUnderClaim$|^MajorOverClaim$|^MinorOverClaim$|^OverClaim$|^UnderClaim$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsClassifiedGenreValidator;
impl AvsClassifiedGenreValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Blues$|^ClassicalMusic$|^CountryMusic$|^ElectronicMusic$|^Folk$|^Gospel$|^HipHop$|^Jazz$|^Latin$|^Pop$|^R'n'B$|^Reggae$|^Rock$|^Spoken$|^Traditional$|^UserDefined$|^WorldMusic$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsClipTypeValidator;
impl AvsClipTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Preview$|^StandaloneClip$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCodingTypeValidator;
impl AvsCodingTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Lossless$|^Lossy$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCollectionMandateTypeValidator;
impl AvsCollectionMandateTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Performer$|^RightsOrganization$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCommentaryNoteTypeValidator;
impl AvsCommentaryNoteTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCommercialModelTypeValidator;
impl AvsCommercialModelTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AdvertisementSupportedModel$|^AsPerContract$|^DeviceFeeModel$|^FreeOfChargeModel$|^PayAsYouGoModel$|^PerformanceRoyaltiesModel$|^RightsClaimModel$|^SubscriptionModel$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCommercialModelTypeERNValidator;
impl AvsCommercialModelTypeERNValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AdvertisementSupportedModel$|^DeviceFeeModel$|^FreeOfChargeModel$|^PayAsYouGoModel$|^PerformanceRoyaltiesModel$|^RightsClaimModel$|^SubscriptionModel$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCommercialModelTypeMWNLValidator;
impl AvsCommercialModelTypeMWNLValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AdvertisementSupportedModel$|^PayAsYouGoModel$|^SubscriptionModel$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCompilationTypeValidator;
impl AvsCompilationTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^InternalCompilation$|^NonInternalCompilation$|^NotCompiled$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCompositeMusicalWorkTypeValidator;
impl AvsCompositeMusicalWorkTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Medley$|^Neither$|^Potpourri$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsContainerFormatValidator;
impl AvsContainerFormatValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation =
            r"^AIFF$|^AVI$|^MP4$|^Ogg$|^QuickTime$|^RealMedia$|^RMF$|^UserDefined$|^WAV$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsContributorClaimStatusValidator;
impl AvsContributorClaimStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Accepted$|^Conflict$|^DataInconsistent$|^NoConflict$|^PendingReview$|^Rejected$|^Revoked$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsContributorRoleValidator;
impl AvsContributorRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Adapter$|^Architect$|^Arranger$|^Author$|^AuthorInQuotations$|^AuthorOfAfterword$|^Compiler$|^Composer$|^ComposerLyricist$|^Conceptor$|^Creator$|^DialogueAuthor$|^Dissertant$|^Engraver$|^Etcher$|^Journalist$|^LandscapeArchitect$|^Librettist$|^Lithographer$|^Lyricist$|^MetalEngraver$|^NonLyricAuthor$|^PlateMaker$|^Playwright$|^Reporter$|^Reviewer$|^Rubricator$|^ScreenplayAuthor$|^Sculptor$|^SubArranger$|^SubLyricist$|^Translator$|^Woodcutter$|^WoodEngraver$|^WriterOfAccompanyingMaterial$|^BookPublisher$|^CopyrightClaimant$|^CopyrightHolder$|^MusicPublisher$|^NewspaperPublisher$|^OriginalPublisher$|^PeriodicalPublisher$|^SubPublisher$|^SubstitutedPublisher$|^Unknown$|^UserDefined$|^Accompanyist$|^Actor$|^AdditionalEngineer$|^AdditionalMixingEngineer$|^AdditionalPerformer$|^AdditionalProgrammingEngineer$|^AdditionalStudioProducer$|^AnchorPerson$|^AnimalTrainer$|^Animator$|^Annotator$|^Announcer$|^AAndRAdministrator$|^AAndRCoordinator$|^Armourer$|^ArtCopyist$|^ArtDirector$|^Artist$|^ArtistBackgroundVocalEngineer$|^ArtistVocalEngineer$|^ArtistVocalSecondEngineer$|^AssistantCameraOperator$|^AssistantChiefLightingTechnician$|^AssistantConductor$|^AssistantDirector$|^AssistantEditor$|^AssistantEngineer$|^AssistantProducer$|^AssistantVisualEditor$|^AssociatedPerformer$|^AssociateProducer$|^AuralTrainer$|^BackgroundVocalist$|^BalanceEngineer$|^BandLeader$|^Binder$|^BindingDesigner$|^BookDesigner$|^BookjackDesigner$|^BookplateDesigner$|^BookProducer$|^BroadcastAssistant$|^BroadcastJournalist$|^Calligrapher$|^CameraOperator$|^Carpenter$|^Cartographer$|^Cartoonist$|^CastingDirector$|^Causeur$|^Censor$|^ChiefLightingTechnician$|^Choir$|^ChoirMember$|^Choreographer$|^ChorusMaster$|^CircusArtist$|^ClapperLoader$|^ClubDJ$|^CoDirector$|^CoExecutiveProducer$|^ColorSeparator$|^Comedian$|^CoMixer$|^CoMixingEngineer$|^Commentator$|^CommissioningBroadcaster$|^CompilationProducer$|^ComputerGraphicCreator$|^ComputerProgrammer$|^ConcertMaster$|^Conductor$|^Consultant$|^ContinuityChecker$|^Contractor$|^CoProducer$|^Correspondent$|^CostumeDesigner$|^CoverDesigner$|^Dancer$|^Delineator$|^Designer$|^DialogueCoach$|^DialogueDirector$|^DigitalAudioWorkstationEngineer$|^DigitalEditingEngineer$|^DigitalEditingSecondEngineer$|^Director$|^DirectStreamDigitalEngineer$|^DistributionCompany$|^DJ$|^Draughtsman$|^Dresser$|^Dubber$|^Editor$|^EditorInChief$|^EditorOfTheDay$|^Encoder$|^Engineer$|^Ensemble$|^ExecutiveProducer$|^Expert$|^Facsimilist$|^FightDirector$|^FilmDirector$|^FilmDistributor$|^FilmEditor$|^FilmProducer$|^FilmSoundEngineer$|^FloorManager$|^FocusPuller$|^FoleyArtist$|^FoleyEditor$|^FoleyMixer$|^GraphicArtist$|^GraphicAssistant$|^GraphicDesigner$|^Greensman$|^Grip$|^GuestConductor$|^GroupMember$|^Hairdresser$|^Illustrator$|^ImmersiveMasteringEngineer$|^ImmersiveMixingEngineer$|^InitialProducer$|^InterviewedGuest$|^Interviewer$|^KeyCharacter$|^KeyGrip$|^KeyTalent$|^Leadman$|^LeadPerformer$|^LeadVocalist$|^LightingDirector$|^LightingTechnician$|^LocationManager$|^MakeUpArtist$|^Manufacturer$|^MasteringEngineer$|^MasteringSecondEngineer$|^MatteArtist$|^Mixer$|^MixingEngineer$|^MixingSecondEngineer$|^MusicArranger$|^MusicCopyist$|^MusicDirector$|^MusicGroup$|^Musician$|^Narrator$|^NewsProducer$|^NewsReader$|^NotSpecified$|^Orchestra$|^OrchestraMember$|^OriginalArtist$|^OverdubEngineer$|^OverdubSecondEngineer$|^Painter$|^Performer$|^Photographer$|^PhotographyDirector$|^PlaybackSinger$|^PostProducer$|^PreProduction$|^PreProductionEngineer$|^PreProductionSecondEngineer$|^Presenter$|^PrimaryMusician$|^ProductionAssistant$|^ProductionCompany$|^ProductionCoordinator$|^ProductionDepartment$|^ProductionManager$|^ProductionSecretary$|^ProjectEngineer$|^Programmer$|^ProgrammingEngineer$|^ProgramProducer$|^PropertyManager$|^PublishingDirector$|^Puppeteer$|^Pyrotechnician$|^RecordingEngineer$|^RecordingSecondEngineer$|^Redactor$|^ReissueProducer$|^RemixedArtist$|^Remixer$|^RemixingEngineer$|^RemixingSecondEngineer$|^Repetiteur$|^Researcher$|^ResearchTeamHead$|^ResearchTeamMember$|^Restager$|^Rigger$|^RightsControllerOnProduct$|^Runner$|^ScenicOperative$|^ScientificAdvisor$|^ScriptSupervisor$|^SecondAssistantCameraOperator$|^SecondAssistantDirector$|^SecondConductor$|^SecondEngineer$|^SecondUnitDirector$|^SeriesProducer$|^SetDesigner$|^SetDresser$|^SignLanguageInterpreter$|^Soloist$|^SoundDesigner$|^SoundMixer$|^SoundRecordist$|^SoundSupervisor$|^Speaker$|^SpecialEffectsTechnician$|^Sponsor$|^StageAssistantEngineer$|^StageDirector$|^StageEngineer$|^StoryTeller$|^StringEngineer$|^StringProducer$|^StringsDirector$|^StudioConductor$|^StudioMusician$|^StudioPersonnel$|^StudioProducer$|^Stunts$|^SubtitlesEditor$|^SubtitlesTranslator$|^SupportingActor$|^SurroundMixingEngineer$|^SurroundMixingSecondEngineer$|^TapeOperator$|^TechnicalDirector$|^Tonmeister$|^TrackingEngineer$|^TrackingSecondEngineer$|^TransfersAndSafetiesEngineer$|^TransfersAndSafetiesSecondEngineer$|^TransportationManager$|^Treatment/ProgramProposal$|^TypeDesigner$|^VideoDirector$|^Videographer$|^VideoMusicalDirector$|^VideoProducer$|^VisionMixer$|^VisualEditor$|^VisualEffectsTechnician$|^VocalArranger$|^VocalEditingEngineer$|^VocalEditingSecondEngineer$|^VocalEngineer$|^Vocalist$|^VocalSecondEngineer$|^VocalProducer$|^VoiceActor$|^Wardrobe$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsContributorRoleRDRValidator;
impl AvsContributorRoleRDRValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Accompanyist$|^Actor$|^AdditionalEngineer$|^AdditionalMixingEngineer$|^AdditionalPerformer$|^AdditionalProgrammingEngineer$|^AdditionalStudioProducer$|^AnchorPerson$|^AnimalTrainer$|^Animator$|^Annotator$|^Announcer$|^AAndRAdministrator$|^AAndRCoordinator$|^Armourer$|^ArtCopyist$|^ArtDirector$|^Artist$|^ArtistBackgroundVocalEngineer$|^ArtistVocalEngineer$|^ArtistVocalSecondEngineer$|^AssistantCameraOperator$|^AssistantChiefLightingTechnician$|^AssistantConductor$|^AssistantDirector$|^AssistantEditor$|^AssistantEngineer$|^AssistantProducer$|^AssistantVisualEditor$|^AssociatedPerformer$|^AssociateProducer$|^AuralTrainer$|^BackgroundVocalist$|^BalanceEngineer$|^BandLeader$|^Binder$|^BindingDesigner$|^BookDesigner$|^BookjackDesigner$|^BookplateDesigner$|^BookProducer$|^BroadcastAssistant$|^BroadcastJournalist$|^Calligrapher$|^CameraOperator$|^Carpenter$|^Cartographer$|^Cartoonist$|^CastingDirector$|^Causeur$|^Censor$|^ChiefLightingTechnician$|^Choir$|^ChoirMember$|^Choreographer$|^ChorusMaster$|^CircusArtist$|^ClapperLoader$|^ClubDJ$|^CoDirector$|^CoExecutiveProducer$|^ColorSeparator$|^Comedian$|^CoMixer$|^CoMixingEngineer$|^Commentator$|^CommissioningBroadcaster$|^CompilationProducer$|^ComputerGraphicCreator$|^ComputerProgrammer$|^ConcertMaster$|^Conductor$|^Consultant$|^ContinuityChecker$|^Contractor$|^CoProducer$|^Correspondent$|^CostumeDesigner$|^CoverDesigner$|^Dancer$|^Delineator$|^Designer$|^DialogueCoach$|^DialogueDirector$|^DigitalAudioWorkstationEngineer$|^DigitalEditingEngineer$|^DigitalEditingSecondEngineer$|^Director$|^DirectStreamDigitalEngineer$|^DistributionCompany$|^DJ$|^Draughtsman$|^Dresser$|^Dubber$|^Editor$|^EditorInChief$|^EditorOfTheDay$|^Encoder$|^Engineer$|^Ensemble$|^ExecutiveProducer$|^Expert$|^Facsimilist$|^FightDirector$|^FilmDirector$|^FilmDistributor$|^FilmEditor$|^FilmProducer$|^FilmSoundEngineer$|^FloorManager$|^FocusPuller$|^FoleyArtist$|^FoleyEditor$|^FoleyMixer$|^GraphicArtist$|^GraphicAssistant$|^GraphicDesigner$|^Greensman$|^Grip$|^GuestConductor$|^GroupMember$|^Hairdresser$|^Illustrator$|^ImmersiveMasteringEngineer$|^ImmersiveMixingEngineer$|^InitialProducer$|^InterviewedGuest$|^Interviewer$|^KeyCharacter$|^KeyGrip$|^KeyTalent$|^Leadman$|^LeadPerformer$|^LeadVocalist$|^LightingDirector$|^LightingTechnician$|^LocationManager$|^MakeUpArtist$|^Manufacturer$|^MasteringEngineer$|^MasteringSecondEngineer$|^MatteArtist$|^Mixer$|^MixingEngineer$|^MixingSecondEngineer$|^MusicArranger$|^MusicCopyist$|^MusicDirector$|^MusicGroup$|^Musician$|^Narrator$|^NewsProducer$|^NewsReader$|^NotSpecified$|^Orchestra$|^OrchestraMember$|^OriginalArtist$|^OverdubEngineer$|^OverdubSecondEngineer$|^Painter$|^Performer$|^Photographer$|^PhotographyDirector$|^PlaybackSinger$|^PostProducer$|^PreProduction$|^PreProductionEngineer$|^PreProductionSecondEngineer$|^Presenter$|^PrimaryMusician$|^ProductionAssistant$|^ProductionCompany$|^ProductionCoordinator$|^ProductionDepartment$|^ProductionManager$|^ProductionSecretary$|^ProjectEngineer$|^Programmer$|^ProgrammingEngineer$|^ProgramProducer$|^PropertyManager$|^PublishingDirector$|^Puppeteer$|^Pyrotechnician$|^RecordingEngineer$|^RecordingSecondEngineer$|^Redactor$|^ReissueProducer$|^RemixedArtist$|^Remixer$|^RemixingEngineer$|^RemixingSecondEngineer$|^Repetiteur$|^Researcher$|^ResearchTeamHead$|^ResearchTeamMember$|^Restager$|^Rigger$|^RightsControllerOnProduct$|^Runner$|^ScenicOperative$|^ScientificAdvisor$|^ScriptSupervisor$|^SecondAssistantCameraOperator$|^SecondAssistantDirector$|^SecondConductor$|^SecondEngineer$|^SecondUnitDirector$|^SeriesProducer$|^SetDesigner$|^SetDresser$|^SignLanguageInterpreter$|^Soloist$|^SoundDesigner$|^SoundMixer$|^SoundRecordist$|^SoundSupervisor$|^Speaker$|^SpecialEffectsTechnician$|^Sponsor$|^StageAssistantEngineer$|^StageDirector$|^StageEngineer$|^StoryTeller$|^StringEngineer$|^StringProducer$|^StringsDirector$|^StudioConductor$|^StudioMusician$|^StudioPersonnel$|^StudioProducer$|^Stunts$|^SubtitlesEditor$|^SubtitlesTranslator$|^SupportingActor$|^SurroundMixingEngineer$|^SurroundMixingSecondEngineer$|^TapeOperator$|^TechnicalDirector$|^Tonmeister$|^TrackingEngineer$|^TrackingSecondEngineer$|^TransfersAndSafetiesEngineer$|^TransfersAndSafetiesSecondEngineer$|^TransportationManager$|^Treatment/ProgramProposal$|^TypeDesigner$|^Unknown$|^UserDefined$|^VideoDirector$|^Videographer$|^VideoMusicalDirector$|^VideoProducer$|^VisionMixer$|^VisualEditor$|^VisualEffectsTechnician$|^VocalArranger$|^VocalEditingEngineer$|^VocalEditingSecondEngineer$|^VocalEngineer$|^Vocalist$|^VocalSecondEngineer$|^VocalProducer$|^VoiceActor$|^Wardrobe$|^Mime$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCreationTypeValidator;
impl AvsCreationTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^MusicalWork$|^Release$|^Resource$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCreativeMusicalWorkContributorRoleValidator;
impl AvsCreativeMusicalWorkContributorRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Adapter$|^Architect$|^Arranger$|^Author$|^AuthorInQuotations$|^AuthorOfAfterword$|^Compiler$|^Composer$|^ComposerLyricist$|^Conceptor$|^Creator$|^DialogueAuthor$|^Dissertant$|^Engraver$|^Etcher$|^Journalist$|^LandscapeArchitect$|^Librettist$|^Lithographer$|^Lyricist$|^MetalEngraver$|^NonLyricAuthor$|^PlateMaker$|^Playwright$|^Reporter$|^Reviewer$|^Rubricator$|^ScreenplayAuthor$|^Sculptor$|^SubArranger$|^SubLyricist$|^Translator$|^Woodcutter$|^WoodEngraver$|^WriterOfAccompanyingMaterial$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCtProposedActionTypeValidator;
impl AvsCtProposedActionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^HandleOutsideOfThread$|^SendUpdate$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCueOriginValidator;
impl AvsCueOriginValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^LibraryMusic$|^PreexistingMusic$|^SpeciallyCommissionedMusic$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCueSheetTypeValidator;
impl AvsCueSheetTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AverageCueSheet$|^CompositeCueSheet$|^StandardCueSheet$|^SummarisedCueSheet$|^SurrogateCueSheet$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCueUseTypeValidator;
impl AvsCueUseTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AudioLogo$|^Background$|^Bumper$|^EssentialPart$|^FilmTheme$|^IndistinguishableBackground$|^OnScreenMusic$|^RolledUpCue$|^Theme$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCurrencyCodeValidator;
impl AvsCurrencyCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AED$|^AFN$|^ALL$|^AMD$|^ANG$|^AOA$|^ARS$|^AUD$|^AWG$|^AZN$|^BAM$|^BBD$|^BDT$|^BGN$|^BHD$|^BIF$|^BMD$|^BND$|^BOB$|^BOV$|^BRL$|^BSD$|^BTN$|^BWP$|^BYR$|^BZD$|^CAD$|^CDF$|^CHF$|^CLF$|^CLP$|^CNY$|^COP$|^COU$|^CRC$|^CUC$|^CUP$|^CVE$|^CZK$|^DJF$|^DKK$|^DOP$|^DZD$|^EGP$|^ERN$|^ETB$|^EUR$|^FJD$|^FKP$|^GBP$|^GEL$|^GHS$|^GIP$|^GMD$|^GNF$|^GTQ$|^GYD$|^HKD$|^HNL$|^HTG$|^HUF$|^IDR$|^ILS$|^INR$|^IQD$|^IRR$|^ISK$|^JMD$|^JOD$|^JPY$|^KES$|^KGS$|^KHR$|^KMF$|^KPW$|^KRW$|^KWD$|^KYD$|^KZT$|^LAK$|^LBP$|^LKR$|^LRD$|^LSL$|^LYD$|^MAD$|^MDL$|^MGA$|^MKD$|^MMK$|^MNT$|^MOP$|^MRU$|^MUR$|^MVR$|^MWK$|^MXN$|^MXV$|^MYR$|^MZN$|^NAD$|^NGN$|^NIO$|^NOK$|^NPR$|^NZD$|^OMR$|^PAB$|^PEN$|^PGK$|^PHP$|^PKR$|^PLN$|^PYG$|^QAR$|^RON$|^RSD$|^RUB$|^RWF$|^SAR$|^SBD$|^SCR$|^SDG$|^SEK$|^SGD$|^SHP$|^SLE$|^SLL$|^SOS$|^SRD$|^SSP$|^STN$|^SVC$|^SYP$|^SZL$|^THB$|^TJS$|^TMT$|^TND$|^TOP$|^TRY$|^TTD$|^TWD$|^TZS$|^UAH$|^UGX$|^USD$|^UYI$|^UYU$|^UZS$|^VED$|^VES$|^VND$|^VUV$|^WST$|^XAF$|^XCD$|^XOF$|^XPF$|^YER$|^ZAR$|^ZMW$|^ZWL$|^CYP$|^EEK$|^HRK$|^LTL$|^LVL$|^MTL$|^MRO$|^ROL$|^SIT$|^SKK$|^STD$|^VEF$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsCurrentTerritoryCodeValidator;
impl AvsCurrentTerritoryCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AD$|^AE$|^AF$|^AG$|^AI$|^AL$|^AM$|^AN$|^AO$|^AQ$|^AR$|^AS$|^AT$|^AU$|^AW$|^AX$|^AZ$|^BA$|^BB$|^BD$|^BE$|^BF$|^BG$|^BH$|^BI$|^BJ$|^BL$|^BM$|^BN$|^BO$|^BQ$|^BR$|^BS$|^BT$|^BV$|^BW$|^BY$|^BZ$|^CA$|^CC$|^CD$|^CF$|^CG$|^CH$|^CI$|^CK$|^CL$|^CM$|^CN$|^CO$|^CR$|^CS$|^CU$|^CV$|^CW$|^CX$|^CY$|^CZ$|^DE$|^DJ$|^DK$|^DM$|^DO$|^DZ$|^EC$|^EE$|^EG$|^EH$|^ER$|^ES$|^ES-CE$|^ES-CN$|^ES-ML$|^ET$|^FI$|^FJ$|^FK$|^FM$|^FO$|^FR$|^GA$|^GB$|^GD$|^GE$|^GF$|^GG$|^GH$|^GI$|^GL$|^GM$|^GN$|^GP$|^GQ$|^GR$|^GS$|^GT$|^GU$|^GW$|^GY$|^HK$|^HM$|^HN$|^HR$|^HT$|^HU$|^ID$|^IE$|^IL$|^IM$|^IN$|^IO$|^IQ$|^IR$|^IS$|^IT$|^JE$|^JM$|^JO$|^JP$|^KE$|^KG$|^KH$|^KI$|^KM$|^KN$|^KP$|^KR$|^KW$|^KY$|^KZ$|^LA$|^LB$|^LC$|^LI$|^LK$|^LR$|^LS$|^LT$|^LU$|^LV$|^LY$|^MA$|^MC$|^MD$|^ME$|^MF$|^MG$|^MH$|^MK$|^ML$|^MM$|^MN$|^MO$|^MP$|^MQ$|^MR$|^MS$|^MT$|^MU$|^MV$|^MW$|^MX$|^MY$|^MZ$|^NA$|^NC$|^NE$|^NF$|^NG$|^NI$|^NL$|^NO$|^NP$|^NR$|^NU$|^NZ$|^OM$|^PA$|^PE$|^PF$|^PG$|^PH$|^PK$|^PL$|^PM$|^PN$|^PR$|^PS$|^PT$|^PW$|^PY$|^QA$|^RE$|^RO$|^RS$|^RU$|^RW$|^SA$|^SB$|^SC$|^SD$|^SE$|^SG$|^SH$|^SI$|^SJ$|^SK$|^SL$|^SM$|^SN$|^SO$|^SR$|^SS$|^ST$|^SV$|^SX$|^SY$|^SZ$|^TC$|^TD$|^TF$|^TG$|^TH$|^TJ$|^TK$|^TL$|^TM$|^TN$|^TO$|^TR$|^TT$|^TV$|^TW$|^TZ$|^UA$|^UG$|^UM$|^US$|^UY$|^UZ$|^VA$|^VC$|^VE$|^VG$|^VI$|^VN$|^VU$|^WF$|^WS$|^YE$|^YT$|^ZA$|^ZM$|^ZW$|^4$|^8$|^12$|^20$|^24$|^28$|^31$|^32$|^36$|^40$|^44$|^48$|^50$|^51$|^52$|^56$|^64$|^68$|^70$|^72$|^76$|^84$|^90$|^96$|^100$|^104$|^108$|^112$|^116$|^120$|^124$|^132$|^140$|^144$|^148$|^152$|^156$|^158$|^170$|^174$|^178$|^180$|^188$|^191$|^192$|^196$|^200$|^203$|^204$|^208$|^212$|^214$|^218$|^222$|^226$|^230$|^231$|^232$|^233$|^242$|^246$|^250$|^258$|^262$|^266$|^268$|^270$|^276$|^278$|^280$|^288$|^296$|^300$|^308$|^320$|^324$|^328$|^332$|^336$|^340$|^344$|^348$|^352$|^356$|^360$|^364$|^368$|^372$|^376$|^380$|^384$|^388$|^392$|^398$|^400$|^404$|^408$|^410$|^414$|^417$|^418$|^422$|^426$|^428$|^430$|^434$|^438$|^440$|^442$|^446$|^450$|^454$|^458$|^462$|^466$|^470$|^478$|^480$|^484$|^492$|^496$|^498$|^499$|^504$|^508$|^512$|^516$|^520$|^524$|^528$|^540$|^548$|^554$|^558$|^562$|^566$|^578$|^583$|^584$|^585$|^586$|^591$|^598$|^600$|^604$|^608$|^616$|^620$|^624$|^626$|^630$|^634$|^642$|^643$|^646$|^659$|^662$|^670$|^674$|^678$|^682$|^686$|^688$|^690$|^694$|^702$|^703$|^704$|^705$|^706$|^710$|^716$|^720$|^724$|^728$|^729$|^732$|^736$|^740$|^748$|^752$|^756$|^760$|^762$|^764$|^768$|^776$|^780$|^784$|^788$|^792$|^795$|^798$|^800$|^804$|^807$|^810$|^818$|^826$|^834$|^840$|^854$|^858$|^860$|^862$|^882$|^886$|^887$|^890$|^891$|^894$|^2100$|^2101$|^2102$|^2103$|^2104$|^2105$|^2106$|^2107$|^2108$|^2109$|^2110$|^2111$|^2112$|^2113$|^2114$|^2115$|^2116$|^2117$|^2118$|^2119$|^2120$|^2121$|^2122$|^2123$|^2124$|^2125$|^2126$|^2127$|^2128$|^2129$|^2130$|^2131$|^2132$|^2133$|^2134$|^2136$|^XK$|^Worldwide$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDanceStyleValidator;
impl AvsDanceStyleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AcroDance$|^Ballet$|^Ballroom$|^Barcarolle$|^Bolero$|^Breakdance$|^Breakdown$|^Bump$|^Cakewalk$|^CanCan$|^CarolinaShag$|^ChaCha$|^Charleston$|^CongaLine$|^ContemporaryDance$|^Contradance$|^CountryTwoStep$|^CountryWesternDance$|^CowboyChaCha$|^Dansband$|^DiscoDance$|^Dougie$|^EastCoastSwing$|^Forro$|^Foxtrot$|^HandJive$|^HipHopDance$|^Hustle$|^Interpretive$|^JazzDance$|^Jig$|^Jitterbug$|^Jive$|^LindyHop$|^LineDance$|^LiquidDance$|^Locking$|^LyricalHipHopDance$|^Mambo$|^Mazurka$|^ModernDance$|^Pasodoble$|^Polonaise$|^Popping$|^Quickstep$|^Robot$|^RodeoSwing$|^Rumba$|^Salsa$|^Samba$|^SlowWaltz$|^SquareDance$|^Stepping$|^Swing$|^Tango$|^TapDancing$|^TheTwist$|^TraditionalDance$|^TripleStep$|^Turfing$|^UpRocking$|^UserDefined$|^VienneseWaltz$|^Waltz$|^Watusi$|^WestCoastSwing$|^WesternSwing$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDataCarrierFormatValidator;
impl AvsDataCarrierFormatValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AFormatVideo$|^ADAT$|^AnalogAudio$|^BFormatVideo$|^Betacam$|^BetacamSP$|^BetacamSX$|^Betamax$|^CFormatVideo$|^CompactDiskDigitalAudio$|^D1DigitalVideo$|^D2DigitalVideo$|^D3DigitalVideo$|^D4DigitalVideo$|^D5DigitalVideo$|^D6DigitalVideo$|^DTRS$|^DVCAM$|^DVCPRO$|^DvcproProgressive$|^DVCPRO50$|^DVCPROHD$|^DigitalAudioStationaryHead$|^DigitalAudioTape$|^DigitalComponentVideocassette$|^DigitalDataStorageTape$|^DirectStreamDigital$|^FileAllocationTable$|^FileAllocationTable32Bit$|^HierarchicalFileSystem$|^HierarchicalFileSystemPlus$|^ISO9660$|^JvcPcmDigital$|^LinearTapeFileSystem$|^Masterlink$|^NewTechnologyFileSystem$|^NotApplicable$|^PcmDigital$|^ProDigi$|^Proprietary$|^RADAR$|^RADARII$|^SonyPCM1630$|^StreamingData$|^TransverseTrackQuadraplexVideo$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDataCarrierTypeValidator;
impl AvsDataCarrierTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^1InchAnalogAudioTape10.5InchReel$|^1InchAnalogAudioTape7InchReel$|^1InchAnalogAudioTapeUnspecifiedReelSize$|^1InchAnalogVideoTapeLargeReel$|^1InchAnalogVideoTapeMediumReel$|^1InchAnalogVideoTapeSmallReel$|^1InchAnalogVideoTapeUnspecifiedReelSize$|^1InchDigitalAudioTape10.5InchReel$|^1InchDigitalAudioTape7InchReel$|^1InchDigitalAudioTapeUnspecifiedReelSize$|^1/2InchAnalogAudioTape10.5InchReel$|^1/2InchAnalogAudioTape7InchReel$|^1/2InchAnalogAudioTapeUnspecifiedReelSize$|^1/2InchDigitalAudioTape10.5InchReel$|^1/2InchDigitalAudioTape7InchReel$|^1/2InchDigitalAudioTapeUnspecifiedReelSize$|^1/2InchDigitalVideoTapeLargeCassette$|^1/2InchDigitalVideoTapeMediumCassette$|^1/2InchDigitalVideoTapeSmallCassette$|^1/4InchAnalogAudioTape10.5InchReel$|^1/4InchAnalogAudioTape7InchReel$|^1/4InchAnalogAudioTapeUnspecifiedReelSize$|^1/4InchDigitalAudioTape10.5InchReel$|^1/4InchDigitalAudioTape7InchReel$|^1/4InchDigitalAudioTapeUnspecifiedReelSize$|^1/4InchDigitalVideoTapeExtraLargeCassette$|^1/4InchDigitalVideoTapeLargeCassette$|^1/4InchDigitalVideoTapeMediumCassette$|^1/4InchDigitalVideoTapeSmallCassette$|^16mmSepmagAnalogAudioFilmReel$|^16mmPictureAnalogVideoFilmReel$|^2InchAnalogAudioTape10.5InchReel$|^2InchAnalogAudioTape12InchReel$|^2InchAnalogAudioTape14InchReel$|^2InchAnalogAudioTapeUnspecifiedReelSize$|^2InchAnalogVideoTapeLargeReel$|^2InchAnalogVideoTapeMediumReel$|^2InchAnalogVideoTapeSmallReel$|^2InchAnalogVideoTapeUnspecifiedReelSize$|^3/4InchDigitalVideoTapeLargeCassette$|^3/4InchDigitalVideoTapeMediumCassette$|^3/4InchDigitalVideoTapeSmallCassette$|^35mmSepmagAnalogAudioFilmReel$|^35mmPictureAnalogVideoFilmReel$|^8mmPictureAnalogVideoFilmReel$|^AIT-1DigitalDataTape$|^AIT-2DigitalDataTape$|^AIT-3DigitalDataTape$|^AIT-3EXDigitalDataTape$|^AIT-4DigitalDataTape$|^AIT-5DigitalDataTape$|^BernoulliDisk20MB$|^BetacamSpAnalogVideoTapeLargeCassette$|^BetacamSpAnalogVideoTapeSmallCassette$|^BetacamSxDigitalVideoTapeLargeCassette$|^BetacamSxDigitalVideoTapeSmallCassette$|^BetacamAnalogVideoTapeLargeCassette$|^BetacamAnalogVideoTapeSmallCassette$|^Binder1InchRing$|^Binder1.5InchRing$|^Binder0.5InchRing$|^Binder2InchRing$|^BluRayRecordableOpticalDiskSingleSidedDoubleLayer12cm$|^BluRayRecordableOpticalDiskSingleSidedSingleLayer12cm$|^CdRRecordableOpticalDiskSingleSidedSingleLayer12cm$|^CdRomDigitalDataDisk$|^CdIInteractiveMultimediaDigitalDataDisk$|^CompactCassetteAnalogAudioTape$|^DAT160DigitalStorageTape$|^DAT320DigitalStorageTape$|^DAT72DigitalStorageTape$|^DDS-1DigitalDataTape$|^DDS-2DigitalDataTape$|^DDS-3DigitalDataTape$|^DDS-4DigitalDataTape$|^DLT-IIIDigitalDataTape$|^DLT-IVDigitalDataTape$|^DvDigitalVideoTapeMiniCassette$|^DvDigitalVideoTapeNormalCassette$|^DvcamDigitalVideoTape$|^Dvcpro50DigitalVideoTapeLargeCassette$|^Dvcpro50DigitalVideoTapeMediumCassette$|^Dvcpro50DigitalVideoTapeSmallCassette$|^DvcproHdDigitalVideoTapeExtraLargeCassette$|^DvcproHdDigitalVideoTapeLargeCassette$|^Dvd+RRecordableOpticalDiskSingleSidedDoubleLayer12cm$|^Dvd+RRecordableOpticalDiskSingleSidedSingleLayer12cm$|^Dvd+RwRewritableOpticalDiskSingleSidedSingleLayer12cm$|^DvdRRecordableOpticalDiskSingleSidedDoubleLayer12cm$|^DvdRRecordableOpticalDiskSingleSidedSingleLayer12cm$|^DvdRamRecordableOpticalDiskDoubleSided$|^DvdRamRecordableOpticalDiskSingleSided$|^DigitalBetacamDigitalVideoTapeLargeCassette$|^DigitalBetacamDigitalVideoTapeSmallCassette$|^DigitalAudioTape$|^DigitalCompactCassette$|^DigitalSDigitalVideoTapeCompactCassette$|^DigitalSDigitalVideoTapeStandardCassette$|^DoubleSidedDoubleDensityFloppyDigitalDataDisk3.5Inch$|^DoubleSidedDoubleDensityFloppyDigitalDataDisk5.25Inch$|^Envelope$|^Exabyte8500SeriesDigitalDataTape$|^Exabyte8700SeriesDigitalDataTape$|^Exabyte8900SeriesDigitalDataTape$|^ExabyteMammothDigitalDataTape$|^FileSystem$|^GlassBasedAcetatePhonographAnalogAudioDisk10Inch$|^GlassBasedAcetatePhonographAnalogAudioDisk12Inch$|^GlassBasedAcetatePhonographAnalogAudioDisk14Inch$|^GlassBasedAcetatePhonographAnalogAudioDisk16Inch$|^GlassBasedAcetatePhonographAnalogAudioDisk7Inch$|^GlassBasedAcetatePhonographAnalogAudioDiskUnspecifiedSize$|^HdDvdRecordableOpticalDiskDoubleSidedDoubleLayer12cm$|^HdDvdRecordableOpticalDiskDoubleSidedDoubleLayer8cm$|^HdDvdRecordableOpticalDiskDoubleSidedSingleLayer12cm$|^HdDvdRecordableOpticalDiskDoubleSidedSingleLayer8cm$|^HdDvdRecordableOpticalDiskSingleSidedDoubleLayer12cm$|^HdDvdRecordableOpticalDiskSingleSidedDoubleLayer8cm$|^HdDvdRecordableOpticalDiskSingleSidedSingleLayer12cm$|^HdDvdRecordableOpticalDiskSingleSidedSingleLayer8cm$|^HdcamSrDigitalVideoTapeLargeCassette$|^HdcamSrDigitalVideoTapeSmallCassette$|^HdcamDigitalVideoTape$|^HdvHdtvDigitalVideoTape$|^HardDiskDriveExternalUsb2.0Interface$|^HardDiskDriveExternalUsb3.0Interface$|^HardDiskDriveExternalUsbInterface$|^HardDiskDriveExternalFirewireInterface$|^HardDiskDriveExternalFirewire/UsbInterface$|^HardDiskDriveInternalRibbonCableInterface$|^HardDiskDriveUnspecifiedInterface$|^Hi-8AnalogVideoTape$|^IdeAtaHardDiskDriveExternalUsb2.0Interface$|^IdeAtaHardDiskDriveExternalUsb3.0Interface$|^IdeAtaHardDiskDriveExternalUsbInterface$|^IdeAtaHardDiskDriveExternalFirewireInterface$|^IdeAtaHardDiskDriveExternalFirewire/UsbInterface$|^IdeAtaHardDiskDriveExternalFirewire/Usb/SataInterface$|^IdeAtaHardDiskDriveInternalRibbonCableInterface$|^IdeAtaHardDiskDriveUnspecifiedExternalInterface$|^JazDigitalDataDisk$|^LTO-1UltriumDigitalDataTape$|^LTO-2UltriumDigitalDataTape$|^LTO-3UltriumDigitalDataTape$|^LTO-4UltriumDigitalDataTape$|^LTO-5UltriumDigitalDataTape$|^LTO-6UltriumDigitalDataTape$|^LTO-7UltriumDigitalDataTape$|^LacquerPhonographAnalogAudioDisk10Inch$|^LacquerPhonographAnalogAudioDisk12Inch$|^LacquerPhonographAnalogAudioDisk14Inch$|^LacquerPhonographAnalogAudioDisk16Inch$|^LacquerPhonographAnalogAudioDisk7Inch$|^LacquerPhonographAnalogAudioDiskUnspecifiedSize$|^LaserdiscOpticalDiskSingleSided$|^MiniDisk$|^MoDisk1.3GB$|^MoDisk1200MB$|^MoDisk2.6GB$|^MoDisk540MB$|^MoDisk650MB$|^MetalBasedAcetatePhonographAnalogAudioDisk10Inch$|^MetalBasedAcetatePhonographAnalogAudioDisk12Inch$|^MetalBasedAcetatePhonographAnalogAudioDisk14Inch$|^MetalBasedAcetatePhonographAnalogAudioDisk16Inch$|^MetalBasedAcetatePhonographAnalogAudioDisk7Inch$|^MetalBasedAcetatePhonographAnalogAudioDiskUnspecifiedSize$|^PreMasterCD$|^S-AtaHardDiskDriveExternalUsb2.0Interface$|^S-AtaHardDiskDriveExternalUsb3.0Interface$|^S-AtaHardDiskDriveExternalFirewireInterface$|^S-AtaHardDiskDriveInternalRibbonCableInterface$|^S-AtaHardDiskDriveUnspecifiedExternalInterface$|^S-VhsAnalogVideoTapeCompactCassette$|^S-VhsAnalogVideoTapeStandardCassette$|^ScsiIHardDiskDriveExternalDSubInterface$|^ScsiIHardDiskDriveInternalRibbonCableInterface$|^ScsiIHardDiskDriveUnspecifiedExternalInterface$|^ScsiIIHardDiskDriveExternal50PinInterface$|^ScsiIIHardDiskDriveExternal68PinInterface$|^ScsiIIHardDiskDriveExternalCentronixInterface$|^ScsiIIHardDiskDriveExternalDSubInterface$|^ScsiIIHardDiskDriveInternalRibbonCableInterface$|^ScsiIIILvdHardDiskDriveExternal50PinInterface$|^ScsiIIILvdHardDiskDriveExternal68PinInterface$|^ScsiIIILvdHardDiskDriveExternal80PinInterface$|^ScsiIIILvdHardDiskDriveInternalRibbonCableInterface$|^ShellacPhonographAnalogAudioDisk10Inch$|^ShellacPhonographAnalogAudioDisk12Inch$|^ShellacPhonographAnalogAudioDisk14Inch$|^ShellacPhonographAnalogAudioDisk16Inch$|^ShellacPhonographAnalogAudioDisk7Inch$|^ShellacPhonographAnalogAudioDiskUnspecifiedSize$|^SingleSidedDoubleDensityFloppyDigitalDataDisk3.5Inch$|^SingleSidedSingleDensityFloppyDigitalDataDisk3.5Inch$|^SingleSidedSingleDensityFloppyDigitalDataDisk5.25Inch$|^SingleSidedSingleDensityFloppyDigitalDataDisk8Inch$|^SolidStateMemoryStorageCards$|^StorageBox1.2CubicFeet$|^StorageBox2.0CubicFeet$|^StorageBox$|^StorageContainer1.2CubicFeet$|^StorageContainer2.0CubicFeet$|^StorageContainer$|^Super16mmPictureAnalogVideoFilmReel$|^Super8mmPictureAnalogVideoFilmReel$|^UMaticSpAnalogVideoTapeSmallCassette$|^UMaticAnalogVideoTapeLargeCassette$|^UMaticAnalogVideoTapeSmallCassette$|^VhsAnalogVideoTapeCompactCassette$|^VhsAnalogVideoTapeStandardCassette$|^VinylPhonographAnalogAudioDisk10Inch$|^VinylPhonographAnalogAudioDisk12Inch$|^VinylPhonographAnalogAudioDisk14Inch$|^VinylPhonographAnalogAudioDisk16Inch$|^VinylPhonographAnalogAudioDisk7Inch$|^VinylPhonographAnalogAudioDiskUnspecifiedSize$|^VXA-1DigitalDataTape$|^VXA-2DigitalDataTape$|^VXA-3DigitalDataTape$|^WaxCylinderPhonogramAnalogAudioDisk$|^XdcamRewritableOpticalDisk$|^ZipDigitalDataDisk$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDdexTerritoryCodeValidator;
impl AvsDdexTerritoryCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^XK$|^Worldwide$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDdexTerritoryCodeNoWorldwideValidator;
impl AvsDdexTerritoryCodeNoWorldwideValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^XK$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDeliveryFileTypeValidator;
impl AvsDeliveryFileTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AudioFile$|^AudioVisualFile$|^ColorInformationFile$|^VisualFile$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDeprecatedCurrencyCodeValidator;
impl AvsDeprecatedCurrencyCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^CYP$|^EEK$|^HRK$|^LTL$|^LVL$|^MTL$|^MRO$|^ROL$|^SIT$|^SKK$|^STD$|^VEF$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDeprecatedIsoTerritoryCodeValidator;
impl AvsDeprecatedIsoTerritoryCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AIDJ$|^ANHH$|^BQAQ$|^BUMM$|^BYAA$|^CSHH$|^CSXX$|^CTKI$|^DDDE$|^DYBJ$|^FQHH$|^FXFR$|^GEHH$|^HVBF$|^JTUM$|^MIUM$|^NHVU$|^NQAQ$|^NTHH$|^PCHH$|^PUUM$|^PZPA$|^RHZW$|^SKIN$|^SUHH$|^TPTL$|^VDVN$|^WKUM$|^YDYE$|^YUCS$|^ZRCD$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDeprecatedReleaseTypeValidator;
impl AvsDeprecatedReleaseTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^TrackRelease$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDigitizationModeValidator;
impl AvsDigitizationModeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AAD$|^ADD$|^DDD$|^Unknown$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDiscrepancyTypeValidator;
impl AvsDiscrepancyTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^CalculationError$|^DuplicatedClaimInMessage$|^ClaimBasis$|^OriginallyStatedClaimDoesNotMatch$|^Overclaim$|^OverclaimBySameLicensor$|^PreviouslyInvoiced$|^SalesDataIncorrect$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDisplayArtistRoleValidator;
impl AvsDisplayArtistRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Artist$|^Brand$|^Composer$|^FeaturedArtist$|^MainArtist$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDistributionChannelTypeValidator;
impl AvsDistributionChannelTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AsPerContract$|^Broadcast$|^Cable$|^Internet$|^InternetAndMobile$|^IPTV$|^MobileTelephone$|^Narrowcast$|^OnDemandStream$|^PeerToPeer$|^Physical$|^Satellite$|^Simulcast$|^Unknown$|^UserDefined$|^Webcast$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDocumentTypeLoDValidator;
impl AvsDocumentTypeLoDValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^LetterOfDirection$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDocumentTypeMWLValidator;
impl AvsDocumentTypeMWLValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Contract$|^RateCalculation$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDpidStatusValidator;
impl AvsDpidStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Active$|^Deleted$|^Replaced$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsDrmEnforcementTypeValidator;
impl AvsDrmEnforcementTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DrmEnforced$|^NotDrmEnforced$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsEditionTypeValidator;
impl AvsEditionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ImmersiveEdition$|^NonImmersiveEdition$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsElectroOpticalTransferFunctionTypeValidator;
impl AvsElectroOpticalTransferFunctionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BT.1886$|^ST2084$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsElementConfigurationValidator;
impl AvsElementConfigurationValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^12Track$|^16Track$|^24Track$|^3Track$|^32Track$|^4Track$|^48Track$|^6Track$|^8Track$|^AbletonLive$|^BruArchive$|^BandedDiscInsideOut$|^BandedDiscOutsideIn$|^Cubase$|^DataFiles$|^DigitalPerformer$|^FinalCutExpress$|^FinalCutPro$|^FruityLoops$|^FullTrackMono$|^GarageBand$|^HalfTrackMono$|^HalfTrackStereo$|^HardCopy$|^Interleaved5.1Files$|^InterleavedStereoFiles$|^Logic$|^LtfsArchive$|^LtfsBackup$|^MezzoArchive$|^MicrosoftBackup$|^MonoFiles$|^Nuendo$|^OrangeBook$|^Paris$|^ProTools$|^QuarterTrackMono$|^QuarterTrackStereo$|^QuickTime$|^Redbook$|^RetrospectArchive$|^RetrospectCatalog$|^SplitStereo$|^StudioOne$|^TarArchive$|^TarBackup$|^ToastArchive$|^TrackedDiscInsideOut$|^TrackedDiscOutsideIn$|^TwinTrack$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsElementDesignationValidator;
impl AvsElementDesignationValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Backup$|^Convenience$|^Copy$|^Documentation$|^LongTerm$|^Master$|^Safety$|^StorageContainer$|^Transfer$|^WorkElement$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsEncodingTypeValidator;
impl AvsEncodingTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^IPA$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsEquipmentManufacturerValidator;
impl AvsEquipmentManufacturerValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsEquipmentModelValidator;
impl AvsEquipmentModelValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsEquipmentTypeValidator;
impl AvsEquipmentTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Computer$|^Loudspeaker$|^Microphone$|^MusicalInstrument$|^Recorder$|^SignalProcessor$|^Software$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsErnMessageTypeValidator;
impl AvsErnMessageTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^NewReleaseMessage$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsErnTestMessageTypeValidator;
impl AvsErnTestMessageTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^NewReleaseMessage$|^PurgeReleaseMessage$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsErncFileStatusValidator;
impl AvsErncFileStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ArtistRoleUnknown$|^CommercialReleaseDateInvalid$|^ConflictingAvailabilityPeriods$|^DuplicatedPublisherNames$|^ErnMissing$|^FileOK$|^IdentifierInvalid$|^IdentifierSyntaxInvalid$|^InternalError$|^MetadataMissing$|^NewReleaseMessageInvalid$|^NoDealForTrackRelease$|^NoDealInNewReleaseMessage$|^OriginalReleaseDateLaterThanReleaseDate$|^PrimaryArtistNameMissing$|^ResourceCorrupt$|^ResourceMissing$|^ResourceNotMeetingSpecifications$|^SignatureOrHashSumWrongOrMissing$|^UnsupportedUsage$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsErncProposedActionTypeValidator;
impl AvsErncProposedActionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DoNotResendAffectedResource$|^DoNotResendRelease$|^ResendXmlOnly$|^ResendXmlAndResources$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsErrorSeverityValidator;
impl AvsErrorSeverityValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Critical$|^Information$|^Warning$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsErrorTypeValidator;
impl AvsErrorTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ConformanceError$|^LogicalError$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsEventTypeValidator;
impl AvsEventTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ActivityPeriod$|^Birth$|^Conceptualize$|^Death$|^Dissolution$|^FirstPerformance$|^Incorporation$|^LastPerformance$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsExceptionReasonValidator;
impl AvsExceptionReasonValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation =
            r"^DisputedByLicensee$|^DisputedByRelinquishingPublisher$|^NotFound$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsExpressionTypeValidator;
impl AvsExpressionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Informative$|^Instructive$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsExternallyLinkedResourceTypeValidator;
impl AvsExternallyLinkedResourceTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AdditionalMetadata$|^Logo$|^PromotionalImage$|^PromotionalInformation$|^PromotionalItem$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsFileTypeValidator;
impl AvsFileTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^3DmFile$|^3G2File$|^3GpFile$|^7ZFile$|^8BiFile$|^AacFile$|^AccdbFile$|^AifFile$|^AiFile$|^AnaFile$|^AppFile$|^AsfFile$|^AspFile$|^AsxFile$|^AudFile$|^AviFile$|^BakFile$|^BatFile$|^BinFile$|^BmpFile$|^BtFile$|^BwfFile$|^CabFile$|^CerFile$|^CfgFile$|^CFile$|^CgiFile$|^ClassFile$|^ComFile$|^CplFile$|^CppFile$|^CsFile$|^CsrFile$|^CssFile$|^CsvFile$|^CurFile$|^DatFile$|^DbFile$|^DbxFile$|^DebFile$|^DllFile$|^DmgFile$|^DmpFile$|^DocFile$|^DocxFile$|^DrvFile$|^DrwFile$|^DsdFile$|^DtdFile$|^DvFile$|^DwgFile$|^DxfFile$|^EfxFile$|^EpsFile$|^ExeFile$|^FlaFile$|^FlvFile$|^FntFile$|^FonFile$|^GadgetFile$|^GamFile$|^GhoFile$|^GifFile$|^GpxFile$|^GzFile$|^HqxFile$|^HtmFile$|^HtmlFile$|^IffFile$|^InddFile$|^IniFile$|^IsoFile$|^JarFile$|^JavaFile$|^JpgFile$|^JsFile$|^JspFile$|^KeychainFile$|^KeyFile$|^KmlFile$|^LnkFile$|^LogFile$|^M3UFile$|^MaxFile$|^MdbFile$|^MFile$|^MidFile$|^MimFile$|^MovFile$|^Mp3File$|^Mp4File$|^MpaFile$|^MpgFile$|^MsgFile$|^NesFile$|^NeuFile$|^OriFile$|^OtfFile$|^PagesFile$|^PcmFile$|^PctFile$|^PdbFile$|^PdfFile$|^PhpFile$|^PifFile$|^PkgFile$|^PlFile$|^PlnFile$|^PluginFile$|^PngFile$|^PpsFile$|^PptFile$|^PptxFile$|^PrfFile$|^PsdFile$|^PsFile$|^PspimageFile$|^PtsFile$|^QxdFile$|^QxpFile$|^RaFile$|^RarFile$|^RelsFile$|^RmFile$|^RomFile$|^RssFile$|^RtfFile$|^SavFile$|^Sd2File$|^SdfFile$|^SitFile$|^SitxFile$|^SqlFile$|^SvgFile$|^SwfFile$|^SysFile$|^TarFile$|^TarGzFile$|^ThmFile$|^TifFile$|^TmpFile$|^ToastFile$|^TtfFile$|^TxtFile$|^UueFile$|^VbFile$|^VcdFile$|^VcfFile$|^VobFile$|^WavFile$|^WksFile$|^WmaFile$|^WmvFile$|^WpdFile$|^WpsFile$|^WsfFile$|^XhtmlFile$|^XllFile$|^XlsFile$|^XlsxFile$|^XmlFile$|^YuvFile$|^ZipFile$|^ZipxFile$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsFingerprintAlgorithmTypeValidator;
impl AvsFingerprintAlgorithmTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsFormValidator;
impl AvsFormValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Adagio$|^Allemande$|^Aria$|^ArtSong$|^Bagatelle$|^Ballad$|^Ballade$|^Ballata$|^Barcarolle$|^Bolero$|^CanCan$|^Canon$|^Cantata$|^Canzona$|^Caprice$|^Carol$|^Cavatina$|^Chaconne$|^Chanson$|^Concerto$|^Courante$|^Dance$|^Divertimento$|^Dumka$|^EightBarBlues$|^Estampie$|^Etude$|^Fanfare$|^Fantasy$|^Fugue$|^Furiant$|^Galliard$|^Gigue$|^Hymn$|^Improvisation$|^Interlude$|^Intermezzo$|^Laude$|^Lied$|^Madrigal$|^March$|^Mass$|^Mazurka$|^Minimal$|^Melodie$|^Minuet$|^MomentForm$|^Motet$|^Nocturne$|^Overture$|^Partita$|^Passacaglia$|^Pavane$|^PerpetuumMobile$|^Polonaise$|^PowerBallad$|^Prelude$|^Rag$|^Raga$|^Rhapsody$|^RhythmChanges$|^Ricercar$|^Rondo$|^Saltarello$|^Sarabande$|^Scherzo$|^Sequence$|^Serenade$|^SinfoniaConcertante$|^Sonata$|^Sonatina$|^Suite$|^SymphonicPoem$|^Symphony$|^Tarantella$|^Tiento$|^Toccata$|^TwelveBarBlues$|^UserDefined$|^Variation$|^VerseOnly$|^Vocalise$|^Waltz$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsFrameRateValidator;
impl AvsFrameRateValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^24$|^25$|^29.97$|^30$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsGenderValidator;
impl AvsGenderValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Androgynous$|^Feminine$|^Masculine$|^Unknown$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsGenderPIEValidator;
impl AvsGenderPIEValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Female$|^Male$|^NotApplicable$|^NonBinary$|^NotStated$|^PreferNotToSay$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsGoverningAgreementTypeValidator;
impl AvsGoverningAgreementTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^SessionMusicUnionAgreement$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsHashSumAlgorithmTypeValidator;
impl AvsHashSumAlgorithmTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^CRC32$|^MD2$|^MD4$|^MD4(MLNET)$|^MD5$|^MDC2$|^RMD160$|^SHA$|^SHA1$|^SHA2$|^SHA-224$|^SHA-256$|^SHA3$|^SHA-384$|^SHA-512$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsHdrVideoDynamicMetadataTypeValidator;
impl AvsHdrVideoDynamicMetadataTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DolbyVisionEmbedded$|^DolbyVisionStandAlone$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsHdrVideoStaticMetadataTypeValidator;
impl AvsHdrVideoStaticMetadataTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^MaxCLL$|^MaxFALL$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsImageCodecTypeValidator;
impl AvsImageCodecTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^GIF$|^JPEG$|^JPEG2000$|^PNG$|^TIFF$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsImageTypeValidator;
impl AvsImageTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BackCoverImage$|^BookletBackImage$|^BookletFrontImage$|^DocumentImage$|^FrontCoverImage$|^Icon$|^Logo$|^Photograph$|^Portrait$|^Poster$|^ProfilePicture$|^SocialBannerImage$|^TrayImage$|^Unknown$|^UserDefined$|^VideoScreenCapture$|^Wallpaper$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsInstrumentManufacturerValidator;
impl AvsInstrumentManufacturerValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsInstrumentModelValidator;
impl AvsInstrumentModelValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsInstrumentTypeValidator;
impl AvsInstrumentTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Accordion$|^Bandoneon$|^ChromaticButtonAccordion$|^Concertina$|^Cordovox$|^Melodeon$|^Musette$|^PianoAccordion$|^ToyAccordion$|^AcousticBassGuitar$|^BabyBass$|^Bass$|^ElectricBassGuitar$|^FretlessBassGuitar$|^PiccoloBass$|^UprightBass$|^WashtubBass$|^DrumMachine$|^Breakbeat$|^DrumKit$|^DrumSample$|^12-StringElectricGuitar$|^12-StringGuitar$|^AcousticGuitar$|^BahianGuitar$|^BajoSexto$|^BaritoneGuitar$|^BaroqueGuitar$|^ChapmanStick$|^NylonStringGuitar$|^DobroGuitar$|^ElectricGuitar$|^ElectricSitar$|^FryingPanGuitar$|^Guitar$|^Guitarron$|^LapSteelGuitar$|^Pedabro$|^PedalSteelGuitar$|^PortugueseGuitar$|^RenaissanceGuitar$|^RomanticGuitar$|^TenorGuitar$|^Tiple$|^TouchGuitar$|^Tres$|^ViolaCaipira$|^AcousticKeyboard$|^Celesta$|^Chamberlin$|^Clavichord$|^Clavinet$|^Dulcitone$|^ElectricPiano$|^Harpsichord$|^Keyboard$|^Mellotron$|^Optigan$|^Pianet$|^Rhodes$|^SampledKeyboard$|^Spinet$|^VakoOrchestron$|^Virginals$|^ElectricOrgan$|^HammondOrgan$|^LowreyOrgan$|^Organ$|^PipeOrgan$|^PositiveOrgan$|^PumpOrgan$|^BarrelOrgan$|^BicyclePump$|^ChurchBells$|^Comb$|^Dictophone$|^HohnerGuitaret$|^JewsHarp$|^Kazoo$|^MusicBox$|^Omnichord$|^OtherInstrument$|^SpectrasonicsOmnisphere$|^ToyPiano$|^Turntable$|^AfricanPercussion$|^AgogoBells$|^Angklung$|^Anvil$|^Atumpan$|^Balafon$|^BassDrum(Concert)$|^BassDrum(Kick)$|^Bata$|^BellTree$|^Bendir$|^Berimbau$|^BinghiDrum$|^Bodhran$|^BodyPercussion$|^Bombo$|^BomboLeguero$|^Bones$|^Bongos$|^Bottles$|^BrazilianPercussion$|^Cabasa$|^Caixa$|^Caja$|^Cajon$|^Calabash$|^Carillon$|^Castanet$|^Caxixi$|^Chimes$|^Chocalho$|^Clapstick$|^Claves$|^Claypot$|^Congas$|^Cowbell$|^Crotales$|^Cuica$|^Cymbal(Crash)$|^Cymbal(Ride)$|^Cymbal(Suspended)$|^Cymbals$|^Daf$|^Damaru$|^Davul$|^Dayereh$|^Defi$|^Dhol$|^Dholak$|^Djembe$|^Dohol$|^Doumbek$|^DrumSticks$|^Duggi$|^Dunun$|^ElephantBell$|^FingerClicks$|^FingerCymbals$|^FingerSnaps$|^Flexatone$|^FolkloricPercussion$|^FootStomp$|^Frog$|^Gambang$|^Gamelan$|^Ganga$|^GlassHarmonica$|^GlassHarp$|^Glockenspiel$|^Gong$|^Guacharaca$|^Guache$|^Guira$|^Guiro$|^HandBells$|^HandChimes$|^HandClaps$|^HiHatCymbal$|^JamBlock$|^Jawbone$|^Jawharp$|^Jug$|^Kanjira$|^Katsa$|^Kendang$|^Khamak$|^Khartal$|^Khol$|^KhongWongLek$|^KhongWongYai$|^Knuckles$|^LatinPercussion$|^Lithophone$|^Lokole$|^Madal$|^Maracas$|^Marimba$|^Marimbaphone$|^Marimbula$|^Mazhar$|^Kalimba$|^MetalCans$|^MouthPercussion$|^Mridangam$|^Muharsing$|^Naal$|^Nagara$|^OboromDrum$|^Octoban$|^OrchestralPercussion$|^PaddleDrums$|^Pandeiro$|^PercussionInstrument$|^PitchedPercussionInstrument$|^PongLang$|^PotsAndPans$|^Qarkabeb$|^Rainstick$|^Ranat$|^Ratchet$|^Rattle$|^RecoReco$|^Repinique$|^RhythmStick$|^Riq$|^Rnga$|^Rolmo$|^Rototoms$|^Sabar$|^SandBlocks$|^Saw$|^Scratcher$|^Shaker$|^Shekere$|^SingingBowls$|^Sistrum$|^Slapstick$|^SleighBells$|^SnareDrum$|^SnareDrum(Marching)$|^Spoons$|^SpringDrum$|^SteelDrums$|^Sticks$|^Surdo$|^Taal$|^Taarija$|^Tabla$|^Tabor$|^Taiko$|^TalkingDrum$|^Tambora$|^Tamborim$|^Tambourine$|^TaongaPuoro$|^Tar(Percussion)$|^Tarol$|^TempleBell$|^TempleBlocks$|^TenorDrum$|^Thavil$|^ThunderSheet$|^TibetanBells$|^Timbales$|^Timbau$|^Timpani$|^Tingsha$|^Tompak$|^Toms$|^TongueDrum$|^Triangle$|^Txalaparta$|^Udu$|^UliUli$|^UnpitchedPercussionInstrument$|^Urumee$|^Vibraphone$|^Vibraslap$|^Washboard$|^Waterphone$|^WindChimes$|^WindMachine$|^WobbleBoard$|^WoodBlock$|^Xylophone$|^Xylorimba$|^Zerbaghali$|^ZydecoRubboard$|^Fortepiano$|^GrandPiano$|^Lutheal$|^Piano$|^PianoHarp$|^Pianola$|^PreparedPiano$|^SquarePiano$|^TackPiano$|^UprightPiano$|^AnimalSounds$|^Applause$|^BirdSong$|^CarSounds$|^Chatter$|^ChewingSounds$|^Gizmo$|^Gunshots$|^MagneticTapeTreatments$|^OrchestralHit$|^RecordNoise$|^Siren$|^SoundDesign$|^SoundEffects$|^TrainSounds$|^Treatments$|^UnintendedArtifacts$|^5-StringBanjo$|^AfricanHarp$|^AltoViol$|^AndeanHarp$|^ArchLute$|^Autoharp$|^Baglama$|^Balalaika$|^Bandura$|^Bandurria$|^Banhu$|^Banjo$|^BanjoGuitar$|^Banjolin$|^BaroqueCello$|^BaroqueViola$|^BaroqueViolin$|^Baryton$|^BassBanjo$|^BassCittern$|^BassRebec$|^BassViol$|^BassoDaBraccio$|^Biwa$|^Bouzouki$|^BowedStrings$|^Bozoq$|^BufoBass$|^Cavaquinho$|^Cello$|^CelloBanjo$|^CelticHarp$|^Charango$|^Cimbalom$|^Citole$|^Cittern$|^ConcertHarp$|^Craviola$|^Crwth$|^Cuatro$|^Cumbus$|^DanBau$|^DanTranh$|^Dilruba$|^Dombra$|^Domra$|^DoubleBass$|^DoubleHarp$|^DoubleViolin$|^DoublebassViol$|^Dranyen$|^Dutar$|^Dzuddahord$|^Ektara$|^Electric6StringViolin$|^ElectricCello$|^ElectricHarp$|^ElectricMandolin$|^ElectricViola$|^ElectricViolin$|^ElectroAcousticHurdyGurdy$|^Ennanga$|^EpinetteDesVosges$|^Erhu$|^Esraj$|^Fiddle$|^FolkHarp$|^Gadulka$|^Gardon$|^Gayageum$|^Ghaychak$|^Gittern$|^Guqin$|^Gusli$|^Guzheng$|^Haegeum$|^HammeredDulcimer$|^HammeredStrings$|^HardangerFiddle$|^Harp$|^Huapanguera$|^HurdyGurdy$|^IrishBouzouki$|^Jakhay$|^JaranaJarocha$|^Jinghu$|^Kacapi$|^Kantele$|^Kanun$|^Kemenche$|^Khim$|^Kora$|^Koto$|^Kugo$|^Langeleik$|^Laouto$|^Leona$|^Lirone$|^Lute$|^LyraViol$|^Lyre$|^Mandocello$|^Mandola$|^Mandolele$|^Mandolin$|^Mandolino$|^Mandore$|^Marxophone$|^MedievalFiddle$|^MedievalHarp$|^MohanVeena$|^MusicalBow$|^Ngoni$|^Njarka$|^Nyatiti$|^Nyckelharpa$|^Organistrum$|^Orpharion$|^Oud$|^Pandura$|^ParaguayanHarp$|^Phin$|^Phonofiddle$|^Pipa$|^PluckedDulcimer$|^PluckedStrings$|^Psaltery$|^Rabel$|^Rebab$|^Rebec$|^Santoor$|^Sarangi$|^Sarod$|^Saung$|^SawDuang$|^Shamisen$|^Simsimiyya$|^Sintir$|^Sitar$|^SopranoDomra$|^StringInstrument$|^StrohlViolin$|^Surbahar$|^Swarmandal$|^Tambura$|^Tanbour$|^Tanpura$|^Tar(String)$|^TenorBanjo$|^TenorRebec$|^TenorViol$|^Theorbo$|^Timple$|^TogamanGuitarViol$|^TrebleRebec$|^TrebleViol$|^TrombaMarina$|^Tumbi$|^Tzouras$|^Ukulele$|^Valiha$|^Veena$|^VenezuelanHarp$|^VeracruzHarp$|^ViTar$|^VichitraVeena$|^Vielle$|^Vihuela$|^Viol$|^Viola$|^ViolaDAmore$|^ViolaPomposa$|^Violin$|^ViolinoPiccolo$|^WelshTripleHarp$|^WireStrungHarp$|^Xalam$|^Yangqin$|^YayliTambur$|^Yokin$|^Yueqin$|^Zeze$|^Zhonghu$|^Zither$|^ArpeggiatingSynth$|^SynthBass$|^SynthBrass$|^SynthChoir$|^SynthFX$|^SynthLead$|^SynthPad$|^SynthSteelDrums$|^SynthStrings$|^Synthesizer$|^GroupBackgroundVocalists$|^BoyVoice$|^ChildVoice$|^ChildrensBackgroundVocalist$|^FemaleVoice$|^FemaleBackgroundVocalist$|^GirlVoice$|^LeadVocalist$|^MaleVoice$|^MaleBackgroundVocalist$|^MixedVoice$|^MixedBackgroundVocalist$|^NeutralVoice$|^Voice$|^Alboka$|^Alpenhorn$|^AltoClarinet$|^AltoCrumhorn$|^AltoFlute$|^AltoHorn$|^AltoRecorder$|^AltoSackbut$|^AltoSaxophone$|^AltoShawm$|^AltoTrombone$|^Apito$|^Arghul$|^Aulochrome$|^Bagpipes$|^Bansuri$|^BaritoneHorn$|^BaritoneOboe$|^BaritoneSaxophone$|^BaroqueBassoon$|^BaroqueClarinet$|^BaroqueFlute$|^BaroqueOboe$|^BaroqueRecorder$|^BassClarinet$|^BassDulcian$|^BassFlute$|^BassHarmonica$|^BassRecorder$|^BassSackbut$|^BassSaxophone$|^BassShawm$|^BassTrombone$|^BassTrumpet$|^BassTuba$|^BassetClarinet$|^BassetHorn$|^Bassoon$|^Bawu$|^BirdWhistle$|^Bombard$|^BosunsWhistle$|^BrassInstrument$|^Bugle$|^Calliope$|^Chalumeau$|^ChromaticHarmonica$|^Cimbasso$|^Clarinet$|^ClarinoTrumpet$|^ConchShell$|^ContraAltoClarinet$|^ContrabassClarinet$|^ContrabassRecorder$|^ContrabassSarrusophone$|^ContrabassSaxophone$|^ContrabassTrombone$|^Contrabassoon$|^Cornet$|^Cornetto$|^Crumhorn$|^Daegeum$|^Didgeridoo$|^Diple$|^Dizi$|^Duduk$|^Dulcian$|^Dungchen$|^EnglishHorn$|^Euphonium$|^Fife$|^Fiscorn$|^Flabiol$|^Flageolet$|^Floyera$|^Flugelhorn$|^Flute$|^FrenchHorn$|^Fujara$|^Gasba$|^Gemshorn$|^GermanFlute$|^Ghaita$|^GreatBassRecorder$|^Guanzi$|^Gyaling$|^Harmonica$|^Heckelphone$|^Helicon$|^HeraldTrumpet$|^HighlandPipes$|^HotFountainPen$|^IrishLowWhistle$|^Jagdhorn$|^Kaval$|^KeyedTrumpet$|^Khene$|^Khlui$|^Launeddas$|^Lur$|^Mellophone$|^Melodica$|^Mijwiz$|^MiniatureKhene$|^Mizmar$|^MouthOrgan$|^Nadaswaram$|^Nai$|^NativeAmericanFlute$|^NaturalHorn$|^NaturalTrumpet$|^NeyFlute$|^Oboe$|^OboeDAmore$|^OboeDaCaccia$|^Ocarina$|^Ophicleide$|^Paixiao$|^PanFlute$|^Pi$|^PiccoloClarinet$|^PiccoloFlute$|^PiccoloTrumpet$|^Pinkillu$|^PocketTrumpet$|^PoliceWhistle$|^PostHorn$|^Pungi$|^Quena$|^Quenacho$|^Rackett$|^Rauschpfeife$|^Recorder$|^ReedInstrument$|^Regal$|^Rondador$|^Sackbut$|^Sarrusophone$|^Saxophone$|^Serpent$|^Shakuhachi$|^Shawm$|^Shenai$|^Shelltone$|^Sheng$|^Sho$|^Shofar$|^ShrutiBox$|^ShviWhistle$|^Siku$|^SlideSaxophone$|^SlideTrumpet$|^SlideWhistle$|^SopraninoRecorder$|^SopraninoSaxophone$|^SopranoClarinet$|^SopranoCornet$|^SopranoCrumhorn$|^SopranoDulcian$|^SopranoRecorder$|^SopranoSaxophone$|^SopranoShawm$|^SopranoTrumpet$|^Sordun$|^Sousaphone$|^Suling$|^Suona$|^Tarka$|^Tarogato$|^TenorCrumhorn$|^TenorDulcian$|^TenorFlute$|^TenorRecorder$|^TenorSackbut$|^TenorSaxophone$|^TenorShawm$|^TenorTrombone$|^TinWhistle$|^Trombone$|^Trumpet$|^Tuba$|^Tusselfloyte$|^UilleanPipes$|^ValveTrombone$|^Vuvuzela$|^WagnerTuba$|^WillowFlute$|^WindInstrument$|^WoodFlute$|^WoodTrumpet$|^Wot$|^Xaphoon$|^Xiao$|^Xun$|^Zummara$|^Zurna$|^Choir$|^PercussionSection$|^StringSection$|^WindSection$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsIntensityValidator;
impl AvsIntensityValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^High$|^Low$|^Medium$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsIso31661TerritoryCodeValidator;
impl AvsIso31661TerritoryCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AD$|^AE$|^AF$|^AG$|^AI$|^AL$|^AM$|^AN$|^AO$|^AQ$|^AR$|^AS$|^AT$|^AU$|^AW$|^AX$|^AZ$|^BA$|^BB$|^BD$|^BE$|^BF$|^BG$|^BH$|^BI$|^BJ$|^BL$|^BM$|^BN$|^BO$|^BQ$|^BR$|^BS$|^BT$|^BV$|^BW$|^BY$|^BZ$|^CA$|^CC$|^CD$|^CF$|^CG$|^CH$|^CI$|^CK$|^CL$|^CM$|^CN$|^CO$|^CR$|^CS$|^CU$|^CV$|^CW$|^CX$|^CY$|^CZ$|^DE$|^DJ$|^DK$|^DM$|^DO$|^DZ$|^EC$|^EE$|^EG$|^EH$|^ER$|^ES$|^ET$|^FI$|^FJ$|^FK$|^FM$|^FO$|^FR$|^GA$|^GB$|^GD$|^GE$|^GF$|^GG$|^GH$|^GI$|^GL$|^GM$|^GN$|^GP$|^GQ$|^GR$|^GS$|^GT$|^GU$|^GW$|^GY$|^HK$|^HM$|^HN$|^HR$|^HT$|^HU$|^ID$|^IE$|^IL$|^IM$|^IN$|^IO$|^IQ$|^IR$|^IS$|^IT$|^JE$|^JM$|^JO$|^JP$|^KE$|^KG$|^KH$|^KI$|^KM$|^KN$|^KP$|^KR$|^KW$|^KY$|^KZ$|^LA$|^LB$|^LC$|^LI$|^LK$|^LR$|^LS$|^LT$|^LU$|^LV$|^LY$|^MA$|^MC$|^MD$|^ME$|^MF$|^MG$|^MH$|^MK$|^ML$|^MM$|^MN$|^MO$|^MP$|^MQ$|^MR$|^MS$|^MT$|^MU$|^MV$|^MW$|^MX$|^MY$|^MZ$|^NA$|^NC$|^NE$|^NF$|^NG$|^NI$|^NL$|^NO$|^NP$|^NR$|^NU$|^NZ$|^OM$|^PA$|^PE$|^PF$|^PG$|^PH$|^PK$|^PL$|^PM$|^PN$|^PR$|^PS$|^PT$|^PW$|^PY$|^QA$|^RE$|^RO$|^RS$|^RU$|^RW$|^SA$|^SB$|^SC$|^SD$|^SE$|^SG$|^SH$|^SI$|^SJ$|^SK$|^SL$|^SM$|^SN$|^SO$|^SR$|^SS$|^ST$|^SV$|^SX$|^SY$|^SZ$|^TC$|^TD$|^TF$|^TG$|^TH$|^TJ$|^TK$|^TL$|^TM$|^TN$|^TO$|^TR$|^TT$|^TV$|^TW$|^TZ$|^UA$|^UG$|^UM$|^US$|^UY$|^UZ$|^VA$|^VC$|^VE$|^VG$|^VI$|^VN$|^VU$|^WF$|^WS$|^YE$|^YT$|^ZA$|^ZM$|^ZW$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsIso639Part12LanguageCodeValidator;
impl AvsIso639Part12LanguageCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^aa$|^aar$|^ab$|^abk$|^ae$|^ave$|^af$|^afr$|^ak$|^aka$|^am$|^amh$|^an$|^arg$|^ar$|^ara$|^as$|^asm$|^av$|^ava$|^ay$|^aym$|^az$|^aze$|^ba$|^bak$|^be$|^bel$|^bg$|^bul$|^bh$|^bih$|^bi$|^bis$|^bm$|^bam$|^bn$|^ben$|^bo$|^bod$|^br$|^bre$|^bs$|^bos$|^ca$|^cat$|^ce$|^che$|^ch$|^cha$|^co$|^cos$|^cr$|^cre$|^cs$|^ces$|^cu$|^chu$|^cv$|^chv$|^cy$|^cym$|^da$|^dan$|^de$|^deu$|^dv$|^div$|^dz$|^dzo$|^ee$|^ewe$|^el$|^ell$|^en$|^eng$|^eo$|^epo$|^es$|^spa$|^et$|^est$|^eu$|^eus$|^fa$|^fas$|^ff$|^ful$|^fi$|^fin$|^fj$|^fij$|^fo$|^fao$|^fr$|^fra$|^fy$|^fry$|^ga$|^gle$|^gd$|^gla$|^gl$|^glg$|^gn$|^grn$|^gu$|^guj$|^gv$|^glv$|^ha$|^hau$|^he$|^heb$|^hi$|^hin$|^ho$|^hmo$|^hr$|^hrv$|^ht$|^hat$|^hu$|^hun$|^hy$|^hye$|^hz$|^her$|^ia$|^ina$|^id$|^ind$|^ie$|^ile$|^ig$|^ibo$|^ii$|^iii$|^ik$|^ipk$|^io$|^ido$|^is$|^isl$|^it$|^ita$|^iu$|^iku$|^ja$|^jpn$|^jv$|^jav$|^ka$|^kat$|^kg$|^kon$|^ki$|^kik$|^kj$|^kua$|^kk$|^kaz$|^kl$|^kal$|^km$|^khm$|^kn$|^kan$|^ko$|^kor$|^kr$|^kau$|^ks$|^kas$|^ku$|^kur$|^kv$|^kom$|^kw$|^cor$|^ky$|^kir$|^la$|^lat$|^lb$|^ltz$|^lg$|^lug$|^li$|^lim$|^ln$|^lin$|^lo$|^lao$|^lt$|^lit$|^lu$|^lub$|^lv$|^lav$|^mg$|^mlg$|^mh$|^mah$|^mi$|^mri$|^mk$|^mkd$|^ml$|^mal$|^mn$|^mon$|^mo$|^mr$|^mar$|^ms$|^msa$|^mt$|^mlt$|^my$|^mya$|^na$|^nau$|^nb$|^nob$|^nd$|^nde$|^ne$|^nep$|^ng$|^ndo$|^nl$|^nld$|^nn$|^nno$|^no$|^nor$|^nr$|^nbl$|^nv$|^nav$|^ny$|^nya$|^oc$|^oci$|^oj$|^oji$|^om$|^orm$|^or$|^ori$|^os$|^oss$|^pa$|^pan$|^pi$|^pli$|^pl$|^pol$|^ps$|^pus$|^pt$|^por$|^qu$|^que$|^rm$|^roh$|^rn$|^run$|^ro$|^ron$|^ru$|^rus$|^rw$|^kin$|^sa$|^san$|^sc$|^srd$|^sd$|^snd$|^se$|^sme$|^sg$|^sag$|^si$|^sin$|^sk$|^slk$|^sl$|^slv$|^sm$|^smo$|^sn$|^sna$|^so$|^som$|^sq$|^sqi$|^sr$|^srp$|^ss$|^ssw$|^st$|^sot$|^su$|^sun$|^sv$|^swe$|^sw$|^swa$|^ta$|^tam$|^te$|^tel$|^tg$|^tgk$|^th$|^tha$|^ti$|^tir$|^tk$|^tuk$|^tl$|^tgl$|^tn$|^tsn$|^to$|^ton$|^tr$|^tur$|^ts$|^tso$|^tt$|^tat$|^tw$|^twi$|^ty$|^tah$|^ug$|^uig$|^uk$|^ukr$|^ur$|^urd$|^uz$|^uzb$|^ve$|^ven$|^vi$|^vie$|^vo$|^vol$|^wa$|^wln$|^wo$|^wol$|^xh$|^xho$|^yi$|^yid$|^yo$|^yor$|^za$|^zha$|^zh$|^zho$|^zu$|^zul$|^ace$|^ach$|^ada$|^ady$|^afa$|^afh$|^ain$|^akk$|^ale$|^alg$|^alt$|^ang$|^anp$|^apa$|^arc$|^arn$|^arp$|^art$|^arw$|^ast$|^ath$|^aus$|^awa$|^bad$|^bai$|^bal$|^ban$|^bas$|^bat$|^bej$|^bem$|^ber$|^bgc$|^bho$|^bik$|^bin$|^bla$|^bnt$|^bra$|^btk$|^bua$|^bug$|^byn$|^cad$|^cai$|^car$|^cau$|^ceb$|^cel$|^chb$|^chg$|^chk$|^chm$|^chn$|^cho$|^chp$|^chr$|^chy$|^cmc$|^cnr$|^cop$|^cpe$|^cpf$|^cpp$|^crh$|^crp$|^csb$|^cus$|^dak$|^dar$|^day$|^del$|^den$|^dgr$|^din$|^doi$|^dra$|^dsb$|^dua$|^dum$|^dyu$|^efi$|^egy$|^eka$|^elx$|^enm$|^ewo$|^fan$|^fat$|^fil$|^fiu$|^fon$|^frm$|^fro$|^frr$|^frs$|^fur$|^gaa$|^gay$|^gba$|^gem$|^gez$|^gil$|^gmh$|^goh$|^gon$|^gor$|^got$|^grb$|^grc$|^gsw$|^gwi$|^hai$|^haw$|^hil$|^him$|^hit$|^hmn$|^hsb$|^hup$|^iba$|^ijo$|^ilo$|^inc$|^ine$|^inh$|^ira$|^iro$|^jbo$|^jpr$|^jrb$|^kaa$|^kab$|^kac$|^kam$|^kar$|^kaw$|^kbd$|^kha$|^khi$|^kho$|^kmb$|^kok$|^kos$|^kpe$|^krc$|^krl$|^kro$|^kru$|^kum$|^kut$|^lad$|^lah$|^lam$|^lez$|^lol$|^loz$|^lua$|^lui$|^lun$|^luo$|^lus$|^mad$|^mag$|^mai$|^mak$|^man$|^map$|^mas$|^mdf$|^mdr$|^men$|^mga$|^mic$|^min$|^mis$|^mkh$|^mnc$|^mni$|^mno$|^moh$|^mos$|^mul$|^mun$|^mus$|^mwl$|^mwr$|^myn$|^myv$|^nah$|^nai$|^nap$|^nds$|^new$|^nia$|^nic$|^niu$|^nog$|^non$|^nqo$|^nso$|^nub$|^nwc$|^nym$|^nyn$|^nyo$|^nzi$|^osa$|^ota$|^oto$|^paa$|^pag$|^pal$|^pam$|^pap$|^pau$|^peo$|^phi$|^phn$|^pon$|^pra$|^pro$|^qqa$|^qqb$|^qqc$|^qqd$|^qqe$|^qqf$|^qqg$|^qqh$|^qqi$|^qqj$|^raj$|^rap$|^rar$|^roa$|^rom$|^rup$|^sad$|^sah$|^sai$|^sal$|^sam$|^sas$|^sat$|^scn$|^sco$|^sel$|^sem$|^sga$|^sgn$|^shn$|^sid$|^sio$|^sit$|^sla$|^sma$|^smi$|^smj$|^smn$|^sms$|^snk$|^sog$|^son$|^srn$|^srr$|^ssa$|^suk$|^sus$|^sux$|^syc$|^syr$|^tai$|^tem$|^ter$|^tet$|^tig$|^tiv$|^tkl$|^tlh$|^tli$|^tmh$|^tog$|^tpi$|^tsi$|^tum$|^tup$|^tut$|^tvl$|^tyv$|^udm$|^uga$|^umb$|^und$|^vai$|^vot$|^wak$|^wal$|^war$|^was$|^wen$|^xal$|^yao$|^yap$|^ypk$|^zap$|^zbl$|^zen$|^zgh$|^znd$|^zun$|^zxx$|^zza$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsIso639Part3LanguageCodeValidator;
impl AvsIso639Part3LanguageCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^aaa$|^cmn$|^gbm$|^hne$|^khw$|^sck$|^scl$|^spv$|^yue$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsIsoCurrencyCodeValidator;
impl AvsIsoCurrencyCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AED$|^AFN$|^ALL$|^AMD$|^ANG$|^AOA$|^ARS$|^AUD$|^AWG$|^AZN$|^BAM$|^BBD$|^BDT$|^BGN$|^BHD$|^BIF$|^BMD$|^BND$|^BOB$|^BOV$|^BRL$|^BSD$|^BTN$|^BWP$|^BYR$|^BZD$|^CAD$|^CDF$|^CHF$|^CLF$|^CLP$|^CNY$|^COP$|^COU$|^CRC$|^CUC$|^CUP$|^CVE$|^CZK$|^DJF$|^DKK$|^DOP$|^DZD$|^EGP$|^ERN$|^ETB$|^EUR$|^FJD$|^FKP$|^GBP$|^GEL$|^GHS$|^GIP$|^GMD$|^GNF$|^GTQ$|^GYD$|^HKD$|^HNL$|^HTG$|^HUF$|^IDR$|^ILS$|^INR$|^IQD$|^IRR$|^ISK$|^JMD$|^JOD$|^JPY$|^KES$|^KGS$|^KHR$|^KMF$|^KPW$|^KRW$|^KWD$|^KYD$|^KZT$|^LAK$|^LBP$|^LKR$|^LRD$|^LSL$|^LYD$|^MAD$|^MDL$|^MGA$|^MKD$|^MMK$|^MNT$|^MOP$|^MRU$|^MUR$|^MVR$|^MWK$|^MXN$|^MXV$|^MYR$|^MZN$|^NAD$|^NGN$|^NIO$|^NOK$|^NPR$|^NZD$|^OMR$|^PAB$|^PEN$|^PGK$|^PHP$|^PKR$|^PLN$|^PYG$|^QAR$|^RON$|^RSD$|^RUB$|^RWF$|^SAR$|^SBD$|^SCR$|^SDG$|^SEK$|^SGD$|^SHP$|^SLE$|^SLL$|^SOS$|^SRD$|^SSP$|^STN$|^SVC$|^SYP$|^SZL$|^THB$|^TJS$|^TMT$|^TND$|^TOP$|^TRY$|^TTD$|^TWD$|^TZS$|^UAH$|^UGX$|^USD$|^UYI$|^UYU$|^UZS$|^VED$|^VES$|^VND$|^VUV$|^WST$|^XAF$|^XCD$|^XOF$|^XPF$|^YER$|^ZAR$|^ZMW$|^ZWL$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsIsoLanguageCodeValidator;
impl AvsIsoLanguageCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^aa$|^aar$|^ab$|^abk$|^ae$|^ave$|^af$|^afr$|^ak$|^aka$|^am$|^amh$|^an$|^arg$|^ar$|^ara$|^as$|^asm$|^av$|^ava$|^ay$|^aym$|^az$|^aze$|^ba$|^bak$|^be$|^bel$|^bg$|^bul$|^bh$|^bih$|^bi$|^bis$|^bm$|^bam$|^bn$|^ben$|^bo$|^bod$|^br$|^bre$|^bs$|^bos$|^ca$|^cat$|^ce$|^che$|^ch$|^cha$|^co$|^cos$|^cr$|^cre$|^cs$|^ces$|^cu$|^chu$|^cv$|^chv$|^cy$|^cym$|^da$|^dan$|^de$|^deu$|^dv$|^div$|^dz$|^dzo$|^ee$|^ewe$|^el$|^ell$|^en$|^eng$|^eo$|^epo$|^es$|^spa$|^et$|^est$|^eu$|^eus$|^fa$|^fas$|^ff$|^ful$|^fi$|^fin$|^fj$|^fij$|^fo$|^fao$|^fr$|^fra$|^fy$|^fry$|^ga$|^gle$|^gd$|^gla$|^gl$|^glg$|^gn$|^grn$|^gu$|^guj$|^gv$|^glv$|^ha$|^hau$|^he$|^heb$|^hi$|^hin$|^ho$|^hmo$|^hr$|^hrv$|^ht$|^hat$|^hu$|^hun$|^hy$|^hye$|^hz$|^her$|^ia$|^ina$|^id$|^ind$|^ie$|^ile$|^ig$|^ibo$|^ii$|^iii$|^ik$|^ipk$|^io$|^ido$|^is$|^isl$|^it$|^ita$|^iu$|^iku$|^ja$|^jpn$|^jv$|^jav$|^ka$|^kat$|^kg$|^kon$|^ki$|^kik$|^kj$|^kua$|^kk$|^kaz$|^kl$|^kal$|^km$|^khm$|^kn$|^kan$|^ko$|^kor$|^kr$|^kau$|^ks$|^kas$|^ku$|^kur$|^kv$|^kom$|^kw$|^cor$|^ky$|^kir$|^la$|^lat$|^lb$|^ltz$|^lg$|^lug$|^li$|^lim$|^ln$|^lin$|^lo$|^lao$|^lt$|^lit$|^lu$|^lub$|^lv$|^lav$|^mg$|^mlg$|^mh$|^mah$|^mi$|^mri$|^mk$|^mkd$|^ml$|^mal$|^mn$|^mon$|^mo$|^mr$|^mar$|^ms$|^msa$|^mt$|^mlt$|^my$|^mya$|^na$|^nau$|^nb$|^nob$|^nd$|^nde$|^ne$|^nep$|^ng$|^ndo$|^nl$|^nld$|^nn$|^nno$|^no$|^nor$|^nr$|^nbl$|^nv$|^nav$|^ny$|^nya$|^oc$|^oci$|^oj$|^oji$|^om$|^orm$|^or$|^ori$|^os$|^oss$|^pa$|^pan$|^pi$|^pli$|^pl$|^pol$|^ps$|^pus$|^pt$|^por$|^qu$|^que$|^rm$|^roh$|^rn$|^run$|^ro$|^ron$|^ru$|^rus$|^rw$|^kin$|^sa$|^san$|^sc$|^srd$|^sd$|^snd$|^se$|^sme$|^sg$|^sag$|^si$|^sin$|^sk$|^slk$|^sl$|^slv$|^sm$|^smo$|^sn$|^sna$|^so$|^som$|^sq$|^sqi$|^sr$|^srp$|^ss$|^ssw$|^st$|^sot$|^su$|^sun$|^sv$|^swe$|^sw$|^swa$|^ta$|^tam$|^te$|^tel$|^tg$|^tgk$|^th$|^tha$|^ti$|^tir$|^tk$|^tuk$|^tl$|^tgl$|^tn$|^tsn$|^to$|^ton$|^tr$|^tur$|^ts$|^tso$|^tt$|^tat$|^tw$|^twi$|^ty$|^tah$|^ug$|^uig$|^uk$|^ukr$|^ur$|^urd$|^uz$|^uzb$|^ve$|^ven$|^vi$|^vie$|^vo$|^vol$|^wa$|^wln$|^wo$|^wol$|^xh$|^xho$|^yi$|^yid$|^yo$|^yor$|^za$|^zha$|^zh$|^zho$|^zu$|^zul$|^ace$|^ach$|^ada$|^ady$|^afa$|^afh$|^ain$|^akk$|^ale$|^alg$|^alt$|^ang$|^anp$|^apa$|^arc$|^arn$|^arp$|^art$|^arw$|^ast$|^ath$|^aus$|^awa$|^bad$|^bai$|^bal$|^ban$|^bas$|^bat$|^bej$|^bem$|^ber$|^bgc$|^bho$|^bik$|^bin$|^bla$|^bnt$|^bra$|^btk$|^bua$|^bug$|^byn$|^cad$|^cai$|^car$|^cau$|^ceb$|^cel$|^chb$|^chg$|^chk$|^chm$|^chn$|^cho$|^chp$|^chr$|^chy$|^cmc$|^cnr$|^cop$|^cpe$|^cpf$|^cpp$|^crh$|^crp$|^csb$|^cus$|^dak$|^dar$|^day$|^del$|^den$|^dgr$|^din$|^doi$|^dra$|^dsb$|^dua$|^dum$|^dyu$|^efi$|^egy$|^eka$|^elx$|^enm$|^ewo$|^fan$|^fat$|^fil$|^fiu$|^fon$|^frm$|^fro$|^frr$|^frs$|^fur$|^gaa$|^gay$|^gba$|^gem$|^gez$|^gil$|^gmh$|^goh$|^gon$|^gor$|^got$|^grb$|^grc$|^gsw$|^gwi$|^hai$|^haw$|^hil$|^him$|^hit$|^hmn$|^hsb$|^hup$|^iba$|^ijo$|^ilo$|^inc$|^ine$|^inh$|^ira$|^iro$|^jbo$|^jpr$|^jrb$|^kaa$|^kab$|^kac$|^kam$|^kar$|^kaw$|^kbd$|^kha$|^khi$|^kho$|^kmb$|^kok$|^kos$|^kpe$|^krc$|^krl$|^kro$|^kru$|^kum$|^kut$|^lad$|^lah$|^lam$|^lez$|^lol$|^loz$|^lua$|^lui$|^lun$|^luo$|^lus$|^mad$|^mag$|^mai$|^mak$|^man$|^map$|^mas$|^mdf$|^mdr$|^men$|^mga$|^mic$|^min$|^mis$|^mkh$|^mnc$|^mni$|^mno$|^moh$|^mos$|^mul$|^mun$|^mus$|^mwl$|^mwr$|^myn$|^myv$|^nah$|^nai$|^nap$|^nds$|^new$|^nia$|^nic$|^niu$|^nog$|^non$|^nqo$|^nso$|^nub$|^nwc$|^nym$|^nyn$|^nyo$|^nzi$|^osa$|^ota$|^oto$|^paa$|^pag$|^pal$|^pam$|^pap$|^pau$|^peo$|^phi$|^phn$|^pon$|^pra$|^pro$|^qqa$|^qqb$|^qqc$|^qqd$|^qqe$|^qqf$|^qqg$|^qqh$|^qqi$|^qqj$|^raj$|^rap$|^rar$|^roa$|^rom$|^rup$|^sad$|^sah$|^sai$|^sal$|^sam$|^sas$|^sat$|^scn$|^sco$|^sel$|^sem$|^sga$|^sgn$|^shn$|^sid$|^sio$|^sit$|^sla$|^sma$|^smi$|^smj$|^smn$|^sms$|^snk$|^sog$|^son$|^srn$|^srr$|^ssa$|^suk$|^sus$|^sux$|^syc$|^syr$|^tai$|^tem$|^ter$|^tet$|^tig$|^tiv$|^tkl$|^tlh$|^tli$|^tmh$|^tog$|^tpi$|^tsi$|^tum$|^tup$|^tut$|^tvl$|^tyv$|^udm$|^uga$|^umb$|^und$|^vai$|^vot$|^wak$|^wal$|^war$|^was$|^wen$|^xal$|^yao$|^yap$|^ypk$|^zap$|^zbl$|^zen$|^zgh$|^znd$|^zun$|^zxx$|^zza$|^aaa$|^cmn$|^gbm$|^hne$|^khw$|^sck$|^scl$|^spv$|^yue$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsIsoTerritoryCodeValidator;
impl AvsIsoTerritoryCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AD$|^AE$|^AF$|^AG$|^AI$|^AL$|^AM$|^AN$|^AO$|^AQ$|^AR$|^AS$|^AT$|^AU$|^AW$|^AX$|^AZ$|^BA$|^BB$|^BD$|^BE$|^BF$|^BG$|^BH$|^BI$|^BJ$|^BL$|^BM$|^BN$|^BO$|^BQ$|^BR$|^BS$|^BT$|^BV$|^BW$|^BY$|^BZ$|^CA$|^CC$|^CD$|^CF$|^CG$|^CH$|^CI$|^CK$|^CL$|^CM$|^CN$|^CO$|^CR$|^CS$|^CU$|^CV$|^CW$|^CX$|^CY$|^CZ$|^DE$|^DJ$|^DK$|^DM$|^DO$|^DZ$|^EC$|^EE$|^EG$|^EH$|^ER$|^ES$|^ES-CE$|^ES-CN$|^ES-ML$|^ET$|^FI$|^FJ$|^FK$|^FM$|^FO$|^FR$|^GA$|^GB$|^GD$|^GE$|^GF$|^GG$|^GH$|^GI$|^GL$|^GM$|^GN$|^GP$|^GQ$|^GR$|^GS$|^GT$|^GU$|^GW$|^GY$|^HK$|^HM$|^HN$|^HR$|^HT$|^HU$|^ID$|^IE$|^IL$|^IM$|^IN$|^IO$|^IQ$|^IR$|^IS$|^IT$|^JE$|^JM$|^JO$|^JP$|^KE$|^KG$|^KH$|^KI$|^KM$|^KN$|^KP$|^KR$|^KW$|^KY$|^KZ$|^LA$|^LB$|^LC$|^LI$|^LK$|^LR$|^LS$|^LT$|^LU$|^LV$|^LY$|^MA$|^MC$|^MD$|^ME$|^MF$|^MG$|^MH$|^MK$|^ML$|^MM$|^MN$|^MO$|^MP$|^MQ$|^MR$|^MS$|^MT$|^MU$|^MV$|^MW$|^MX$|^MY$|^MZ$|^NA$|^NC$|^NE$|^NF$|^NG$|^NI$|^NL$|^NO$|^NP$|^NR$|^NU$|^NZ$|^OM$|^PA$|^PE$|^PF$|^PG$|^PH$|^PK$|^PL$|^PM$|^PN$|^PR$|^PS$|^PT$|^PW$|^PY$|^QA$|^RE$|^RO$|^RS$|^RU$|^RW$|^SA$|^SB$|^SC$|^SD$|^SE$|^SG$|^SH$|^SI$|^SJ$|^SK$|^SL$|^SM$|^SN$|^SO$|^SR$|^SS$|^ST$|^SV$|^SX$|^SY$|^SZ$|^TC$|^TD$|^TF$|^TG$|^TH$|^TJ$|^TK$|^TL$|^TM$|^TN$|^TO$|^TR$|^TT$|^TV$|^TW$|^TZ$|^UA$|^UG$|^UM$|^US$|^UY$|^UZ$|^VA$|^VC$|^VE$|^VG$|^VI$|^VN$|^VU$|^WF$|^WS$|^YE$|^YT$|^ZA$|^ZM$|^ZW$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsLabelNameTypeValidator;
impl AvsLabelNameTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DisplayLabelName$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsLabelTypeValidator;
impl AvsLabelTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DisplayLabel$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsLanguageLocalizationTypeValidator;
impl AvsLanguageLocalizationTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Dubbed$|^SubTitled$|^Multilingual$|^Original$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsLicenseRecordValidator;
impl AvsLicenseRecordValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^HasLicense$|^HasNoLicense$|^Unknown$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsLicenseRefusalReasonValidator;
impl AvsLicenseRefusalReasonValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^UserDefined$|^WorkInPublicDomain$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsLicenseRejectionReasonValidator;
impl AvsLicenseRejectionReasonValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DisagreementOverRoyalties$|^DisagreementOverScopeOfLicense$|^DuplicateLicenseRequestNumber$|^LicenseBlocked$|^LicenseExists$|^LicenseNotNeeded$|^ReferencedDocumentMissing$|^ShareSplitsDiffer$|^WorkInPublicDomain$|^WorkUsedMultipleTimes$|^WrongAddressee$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsLinkAcknowledgementStatusValidator;
impl AvsLinkAcknowledgementStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Accepted$|^Acknowledged$|^Conflict$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsLinkDescriptionValidator;
impl AvsLinkDescriptionValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^CoverArt$|^VideoScreenCapture$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsLyricsTypeValidator;
impl AvsLyricsTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Chorus$|^ChorusAndVerse$|^Complete$|^FirstLineOfText$|^Hook$|^JazzScats$|^Stanza$|^Unknown$|^UserDefined$|^Verse$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMeasurementTypeValidator;
impl AvsMeasurementTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BothAudioAndVideo$|^EitherAudioOrVideo$|^Audio$|^Video$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMembershipTypeValidator;
impl AvsMembershipTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^NationalMember$|^RegionalMember$|^WorldwideMember$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMessageActionTypeValidator;
impl AvsMessageActionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BackCatalogDelivery$|^HighPriorityDelivery$|^NewReleaseDelivery$|^ReDelivery$|^TakeDown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMessageControlTypeValidator;
impl AvsMessageControlTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^LiveMessage$|^TestMessage$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMessagePurposeValidator;
impl AvsMessagePurposeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^License$|^NdmaLicense$|^Acknowledgement$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMessageTypeValidator;
impl AvsMessageTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^MusicalWorkClaimNotificationMessage$|^MusicalWorkClaimRequestMessage$|^LicenseRequestMessage$|^LicenseMessage$|^LicenseRevocationMessage$|^LoDMessage$|^LoDConfirmationMessage$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMetadataSourceTypeValidator;
impl AvsMetadataSourceTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Journalist$|^MetadataProvider$|^RightsController$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMissingLinkReasonValidator;
impl AvsMissingLinkReasonValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^NoLinkFound$|^NoMatchFound$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsModeValidator;
impl AvsModeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMoodValidator;
impl AvsMoodValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Angry$|^Anticipation$|^Chill$|^Confident$|^Dark$|^Disgust$|^Dramatic$|^Empowered$|^Energized$|^Evil$|^FeelingDown$|^FeelingGood$|^Free$|^Happy$|^Hungover$|^Inspiring$|^LowKey$|^Mellow$|^Motivated$|^Peaceful$|^Quiet$|^RainyDay$|^Romantic$|^Sad$|^Soulful$|^Surprise$|^Swagger$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMoodOrThemeTypeValidator;
impl AvsMoodOrThemeTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Lyrics$|^LyricsAndMelody$|^Melody$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMusicalWorkContributorRoleValidator;
impl AvsMusicalWorkContributorRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Adapter$|^Architect$|^Arranger$|^Author$|^AuthorInQuotations$|^AuthorOfAfterword$|^Compiler$|^Composer$|^ComposerLyricist$|^Conceptor$|^Creator$|^DialogueAuthor$|^Dissertant$|^Engraver$|^Etcher$|^Journalist$|^LandscapeArchitect$|^Librettist$|^Lithographer$|^Lyricist$|^MetalEngraver$|^NonLyricAuthor$|^PlateMaker$|^Playwright$|^Reporter$|^Reviewer$|^Rubricator$|^ScreenplayAuthor$|^Sculptor$|^SubArranger$|^SubLyricist$|^Translator$|^Woodcutter$|^WoodEngraver$|^WriterOfAccompanyingMaterial$|^BookPublisher$|^CopyrightClaimant$|^CopyrightHolder$|^MusicPublisher$|^NewspaperPublisher$|^OriginalPublisher$|^PeriodicalPublisher$|^SubPublisher$|^SubstitutedPublisher$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMusicalWorkTypeValidator;
impl AvsMusicalWorkTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AdaptedInOriginalLanguage$|^AdaptedInstrumentalWork$|^AdaptedWithNewLyrics$|^ArrangedWithNewMusic$|^CompositeMusicalWork$|^DramaticoMusicalWork$|^LyricRemoval$|^LyricReplacement$|^LyricTranslation$|^Mashup$|^Medley$|^MultimediaProductionWork$|^MusicalWorkMovement$|^MusicalWorkWithSamples$|^MusicArrangement$|^MusicArrangementOfText$|^OriginalLyricsArrangement$|^OriginalMusicAdaptation$|^OriginalMusicalWork$|^Potpourri$|^ProductionMusicLibraryWork$|^RadioProductionWork$|^TheaterProductionWork$|^TvProductionWork$|^Unknown$|^UnspecifiedArrangement$|^UnspecifiedMusicalWorkExcerpt$|^UserDefined$|^VideoProductionWork$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMwnlFileStatusValidator;
impl AvsMwnlFileStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^FileOK$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsMwnlProposedActionTypeValidator;
impl AvsMwnlProposedActionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Resubmit$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsNewStudioRoleValidator;
impl AvsNewStudioRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AdditionalEngineer$|^AnimalTrainer$|^Animator$|^Annotator$|^AAndRCoordinator$|^Armourer$|^ArtDirector$|^ArtistBackgroundVocalEngineer$|^ArtistVocalEngineer$|^ArtistVocalSecondEngineer$|^AssistantCameraOperator$|^AssistantChiefLightingTechnician$|^AssistantDirector$|^AssistantProducer$|^AssistantVisualEditor$|^AuralTrainer$|^Binder$|^BindingDesigner$|^BookDesigner$|^BookjackDesigner$|^BookplateDesigner$|^BookProducer$|^BroadcastAssistant$|^BroadcastJournalist$|^CameraOperator$|^Carpenter$|^CastingDirector$|^Censor$|^ChiefLightingTechnician$|^Choreographer$|^ClapperLoader$|^CoExecutiveProducer$|^CommissioningBroadcaster$|^CompilationProducer$|^Consultant$|^ContinuityChecker$|^Contractor$|^CoProducer$|^Correspondent$|^CostumeDesigner$|^CoverDesigner$|^Designer$|^DialogueCoach$|^DigitalAudioWorkstationEngineer$|^DigitalEditingEngineer$|^DigitalEditingSecondEngineer$|^Director$|^DirectStreamDigitalEngineer$|^DistributionCompany$|^Dresser$|^Dubber$|^Editor$|^EditorInChief$|^EditorOfTheDay$|^Encoder$|^Engineer$|^ExecutiveProducer$|^Expert$|^FightDirector$|^FilmDirector$|^FilmDistributor$|^FilmEditor$|^FilmProducer$|^FilmSoundEngineer$|^FloorManager$|^FocusPuller$|^FoleyArtist$|^FoleyEditor$|^FoleyMixer$|^GraphicAssistant$|^GraphicDesigner$|^Greensman$|^Grip$|^Hairdresser$|^InitialProducer$|^KeyGrip$|^Leadman$|^LightingDirector$|^LightingTechnician$|^LocationManager$|^MakeUpArtist$|^Manufacturer$|^MasteringEngineer$|^MasteringSecondEngineer$|^MatteArtist$|^MixingEngineer$|^MixingSecondEngineer$|^MusicDirector$|^Musician$|^NewsProducer$|^OverdubEngineer$|^OverdubSecondEngineer$|^PhotographyDirector$|^PostProducer$|^ProgrammingEngineer$|^PreProduction$|^PreProductionEngineer$|^ProductionCompany$|^ProductionDepartment$|^ProductionManager$|^ProductionSecretary$|^ProgramProducer$|^ProgramProposalWriter$|^PropertyManager$|^PublishingDirector$|^Pyrotechnician$|^RecordingEngineer$|^RecordingSecondEngineer$|^Redactor$|^ReissueProducer$|^RemixingEngineer$|^RemixingSecondEngineer$|^Repetiteur$|^Researcher$|^ResearchTeamHead$|^ResearchTeamMember$|^Restager$|^Rigger$|^RightsControllerOnProduct$|^Runner$|^ScenicOperative$|^ScientificAdvisor$|^ScriptSupervisor$|^SecondAssistantCameraOperator$|^SecondAssistantDirector$|^SecondEngineer$|^SecondUnitDirector$|^SeriesProducer$|^SetDesigner$|^SetDresser$|^SoundDesigner$|^SoundMixer$|^SoundRecordist$|^SpecialEffectsTechnician$|^Sponsor$|^StageDirector$|^StringEngineer$|^StringProducer$|^StudioConductor$|^StudioPersonnel$|^StudioProducer$|^SubtitlesEditor$|^SubtitlesTranslator$|^TapeOperator$|^TechnicalDirector$|^Tonmeister$|^TrackingEngineer$|^TrackingSecondEngineer$|^TransfersAndSafetiesEngineer$|^TransfersAndSafetiesSecondEngineer$|^TransportationManager$|^Videographer$|^UserDefined$|^VideoProducer$|^VisionMixer$|^VisualEditor$|^VisualEffectsTechnician$|^VocalProducer$|^Wardrobe$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsOperatingSystemTypeValidator;
impl AvsOperatingSystemTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^MacOS$|^MsWindows$|^Symbian$|^Unknown$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsOriginalPurposeValidator;
impl AvsOriginalPurposeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^CommercialRelease$|^Karaoke$|^LibraryMusic$|^SpeciallyCommissionedMusic$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPLineTypeValidator;
impl AvsPLineTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^OriginalPLine$|^RemasteringPLine$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsParentalWarningTypeValidator;
impl AvsParentalWarningTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Explicit$|^ExplicitContentEdited$|^NoAdviceAvailable$|^NotExplicit$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPartyNameFormatValidator;
impl AvsPartyNameFormatValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Abbreviation$|^AsciiTranscribed$|^MisspelledName$|^NameIndexed$|^TranslatedName$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPartyNamePurposeValidator;
impl AvsPartyNamePurposeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Correspondence$|^Contract$|^LyricistCredits$|^Payment$|^PublicCommunication$|^RecordingCredits$|^UserDefined$|^WriterCredits$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPartyNameTypeValidator;
impl AvsPartyNameTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^IncorrectName$|^LegalName$|^Nickname$|^Pseudonym$|^StageName$|^TradingName$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPartyRelationshipTypeValidator;
impl AvsPartyRelationshipTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation =
            r"^HasMember$|^HasPart$|^IsChildOf$|^IsMemberOf$|^IsParentOf$|^IsPartOf$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPartyRelationshipTypePIEValidator;
impl AvsPartyRelationshipTypePIEValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^HasAffiliateMember$|^HasFullMember$|^HasMember$|^HasPart$|^IsAffiliateMemberOf$|^IsCharacterPlayedBy$|^IsChildOf$|^IsChildOrganizationOf$|^IsCoAuthorOf$|^IsCoContributorOf$|^IsConsideredTheSameAs$|^IsCoPerformerOf$|^IsDuplicateOf$|^IsFullMemberOf$|^IsHomonymOf$|^IsInfluencedBy$|^IsInfluencerOf$|^IsMarriedTo$|^IsMemberOf$|^IsNaturalPersonOf$|^IsParentOf$|^IsParentOrganizationOf$|^IsPartOf$|^IsPlayingCharacter$|^IsPseudonymOf$|^IsRelatedTo$|^IsRelatedStagePersonaOf$|^IsStagePersonaOf$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPartyRoleValidator;
impl AvsPartyRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Adapter$|^Arranger$|^Composer$|^ComposerLyricist$|^Creator$|^Lyricist$|^MusicPublisher$|^OriginalPublisher$|^RightsAdministrator$|^SubArranger$|^SubLyricist$|^SubPublisher$|^SubstitutedPublisher$|^Translator$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPartyTypeValidator;
impl AvsPartyTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Anthropomorph$|^AuthorPersona$|^Brand$|^Character$|^ComposingPersona$|^Department$|^Group$|^LegalOrganization$|^NaturalPerson$|^Persona$|^StagePersona$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPendingReasonValidator;
impl AvsPendingReasonValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation =
            r"^NotYetProcessedByDSP$|^NotYetProcessedByRelinquishingRecordCompany$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPercentageTypeValidator;
impl AvsPercentageTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^PercentageOfFreeGoodsPermitted$|^PercentageOfGrossRevenue$|^PercentageOfNetRevenue$|^PercentageOfNetSales$|^PercentageOfPriceConsumerPaid$|^PercentageOfStatutoryRoyaltyRate$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPeriodValidator;
impl AvsPeriodValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AncientMusic$|^ArsAntiqua$|^ArsNova$|^ArsSubtilior$|^Baroque$|^Classical$|^Contemporary$|^EarlyRomantic$|^Experimental$|^GalantMusic$|^HighModern$|^Impressionism$|^LateRomantic$|^Medieval$|^Modern$|^Neoclassicism$|^PostModern$|^Renaissance$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPhysicalCarrierTypeValidator;
impl AvsPhysicalCarrierTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BluRay$|^CD$|^CombiPack$|^CompactCassette$|^DualDisc$|^DVD$|^MemoryDevice$|^SACD$|^UserDefined$|^VideoCassette$|^VinylDisk$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPriceInformationTypeValidator;
impl AvsPriceInformationTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^StandardRetailPrice$|^PreOrderPrice$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPrimaryColorTypeValidator;
impl AvsPrimaryColorTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BT.601$|^BT.709$|^BT.2020$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsProductTypeValidator;
impl AvsProductTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AudioProduct$|^GraphicsProduct$|^MixedMediaBundleProduct$|^MobileProduct$|^UserDefined$|^VideoProduct$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsProfileIdValidator;
impl AvsProfileIdValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BasicAudioProfile$|^BasicAudioProfileMLC$|^BasicAudioProfileSRB$|^UGCProfile$|^UGCProfileSRB$|^AudioVisualProfile$|^AudioVisualProfileSRB$|^RoyaltyReportingProfile$|^RadioBroadcastProfile$|^FinancialReportingToRecordCompaniesProfileSRB$|^MasterListProfile$|^MasterListProfileSRB$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsPurposeValidator;
impl AvsPurposeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BackgroundMusic$|^ChannelTrailerMusic$|^Extract$|^FilmTrailerMusic$|^ForegroundMusic$|^TrailerMusic$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRatingAgencyValidator;
impl AvsRatingAgencyValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ACMA$|^AFR$|^AGCOM$|^ANATEL$|^BBFC$|^BFCO$|^BFSC$|^BFVC$|^BMUKK$|^CBFC$|^CBSC$|^CBSC-F$|^CCC$|^CCE$|^CHVRS$|^CICF$|^CNA$|^CNC$|^CPBC$|^CSA$|^CSCF$|^DJCTQ$|^Eirin$|^ESRB$|^FAB$|^FCB$|^FCO$|^FILM-CH$|^FILM-CZ$|^FILM-EG$|^FILM-EE$|^FILM-GR$|^FILM-PE$|^FILM-SK$|^Filmtilsynet$|^FPB$|^FRB$|^FSK$|^ICAA$|^IFCO$|^IFCOF$|^INCAA$|^KFCB$|^Kijkwijzer$|^KMRB$|^KR$|^KRRIT$|^LSF$|^MBACT$|^MBU$|^MCCAA$|^MDA$|^MDCB$|^Medietilsynet$|^MEKU$|^MFCB$|^MIC$|^MKRF$|^MOC$|^MOC-TW$|^MPAA$|^MPAAT$|^MTRCB$|^NBC$|^NCS$|^NFRC$|^NFVCB$|^NICAM$|^NKC$|^OFLC$|^OFLC-NZ$|^OFRB$|^PEGI$|^RCNOF$|^RDCQ$|^RIAA$|^RTC$|^RTE$|^SBB$|^SiBCI$|^Smais$|^SM-SA$|^SPIO-JK$|^USFA$|^TELA$|^TVPG$|^UserDefined$|^VET$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRatingReasonValidator;
impl AvsRatingReasonValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Behaviour$|^Blasphemy$|^Crime$|^DiscriminationOrPrejudice$|^Drugs$|^ExplicitSex$|^ExtremeViolence$|^FearOrHorror$|^Gambling$|^IllegalDrugs$|^Language$|^LegalDrugs$|^Nudity$|^OnlineGameplay$|^Sex$|^SexualViolence$|^Theme$|^UserDefined$|^Violence$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRdrMessageTypeValidator;
impl AvsRdrMessageTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DeclarationOfSoundRecordingRightsClaimMessage$|^RequestSoundRecordingRightsClaimMessage$|^RevokeSoundRecordingRightsClaimMessage$|^SalesReportMessage$|^DeclarationOfRevenueMessage$|^RightsClaimStatusUpdateMessage$|^AssertionOfCollectionMandateMessage$|^AssertionOfCollectionMandateStatusUpdateMessage$|^RevokeCollectionMandateMessage$|^RevenueDeclarationMessage$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRdrcBatchStatusValidator;
impl AvsRdrcBatchStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BatchOK$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRdrcFileStatusValidator;
impl AvsRdrcFileStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Error$|^FileReceived$|^FileValid$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsReasonForNameChangeValidator;
impl AvsReasonForNameChangeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Deed$|^Marriage$|^Religion$|^SexChange$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRecipientRevenueTypeValidator;
impl AvsRecipientRevenueTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^PerformerAndProducerRevenue$|^PerformerRevenue$|^ProducerRevenue$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRecordingFormatValidator;
impl AvsRecordingFormatValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^360Video$|^Acoustic$|^AdultContent$|^AdvertisementVideo$|^AdviceMagazine$|^Animation$|^AwardShow$|^BalletVideo$|^BehindTheMusic$|^BehindTheScenes$|^BlackAndWhiteVideo$|^CauseRelatedRecording$|^ChildrensFilm$|^ColorizedVideo$|^ColumnVideo$|^ConcertClip$|^ConcertVideo$|^ContentProviderOriginals$|^CorporateFilm$|^Credits$|^DanceVideo$|^Documentary$|^Drama$|^DramaticoMusicalVideo$|^EducationalVideo$|^Episode$|^FeatureFilm$|^Fiction$|^InfomercialVideo$|^InteractiveResource$|^Interview$|^Karaoke$|^LiveEventRecording$|^LiveEventRecordingInStudio$|^LiveEventVideo$|^LiveStream$|^LowComplexityVideo$|^LyricVideo$|^Magazine$|^Menu$|^MultimediaVideo$|^MusicalWorkClip$|^MusicalWorkReadalongVideo$|^MusicalWorkTrailer$|^MusicalWorkVideoChapter$|^News$|^NonMusicalWorkClip$|^NonMusicalWorkReadalongVideo$|^NonMusicalWorkTrailer$|^NonMusicalWorkVideoChapter$|^NonSerialAudioVisualRecording$|^OperaVideo$|^Performance$|^RawFootage$|^ReadalongVideo$|^RealityTvShowVideo$|^Excerpt$|^Season$|^SerialAudioVisualRecording$|^Series$|^Session$|^ShortFilm$|^SilentVideo$|^SketchVideo$|^SoapSitcom$|^SpecialEvent$|^Sport$|^StaticVideo$|^StudioRecording$|^TheatricalWorkVideo$|^TourDiary$|^TrailerVideo$|^Tutorial$|^TvFilm$|^TvFilmPerformance$|^TvProgram$|^TvShowVideo$|^Unknown$|^UserDefined$|^VerticalVideo$|^VideoChapter$|^VideoClip$|^VideoReport$|^VideoStem$|^VirtualRealityExperience$|^Visualizer$|^Vlog$|^Webisode$|^WebResource$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRecordingModeValidator;
impl AvsRecordingModeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BinauralAudio$|^ImmersiveAudio$|^LCR$|^Mono$|^MultichannelAudio$|^MultiTrack$|^Quad$|^Stems$|^Stereo$|^SurroundSound$|^Unknown$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsReferenceCreationValidator;
impl AvsReferenceCreationValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ReferenceResource$|^ConsumerResource$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsReferenceUnitValidator;
impl AvsReferenceUnitValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^PerLicense$|^PerUse$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRegistrationStatusValidator;
impl AvsRegistrationStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ClaimMeetsCoreDataProfile$|^ClaimMeetsRecommendedProfile$|^PendingReview$|^ResourceRegisteredInvalid$|^ResourceRegisteredValid$|^RegistrationRejected$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRejectionReasonValidator;
impl AvsRejectionReasonValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^NotFoundByDSP$|^NotFoundByRelinquishingRecordCompany$|^RejectedByRelinquishingRecordCompany$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRelatedResourceTypeValidator;
impl AvsRelatedResourceTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ACappellaVersion$|^AcousticVersion$|^AlbumVersion$|^AlternativeVersion$|^CleanVersion$|^Cover$|^DemoVersion$|^InstrumentalVersion$|^LiveVersion$|^Medley$|^OriginalRecording$|^RadioVersion$|^SingleVersion$|^StudioVersion$|^TvTrack$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRelationalRelatorValidator;
impl AvsRelationalRelatorValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation =
            r"^EqualTo$|^LessThan$|^LessThanOrEqualTo$|^MoreThan$|^MoreThanOrEqualTo$|^NotEqualTo$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsReleaseProfileVariantVersionIdValidator;
impl AvsReleaseProfileVariantVersionIdValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BoxedSet$|^BoxedSet Classical$|^Classical$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsReleaseProfileVersionIdValidator;
impl AvsReleaseProfileVersionIdValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Audio$|^DjMix$|^LongFormMusicalWorkVideo$|^MixedMedia$|^Ringtone$|^SimpleAudioSingle$|^SimpleVideoSingle$|^Video$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsReleaseRelationshipTypeValidator;
impl AvsReleaseRelationshipTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^HasArtistFromEnsemble$|^HasArtistFromSameEnsemble$|^HasContentFrom$|^HasEnsembleWithArtist$|^HasSameArtist$|^HasSameRecordingProject$|^HasSimilarContent$|^IsAudioUsedFor$|^IsDifferentEncoding$|^IsDigitalEquivalentToPhysical$|^IsEditedVersionOf$|^IsEquivalentToAudio$|^IsEquivalentToVideo$|^IsExtendedFromAlbum$|^IsFromAudio$|^IsFromVideo$|^IsImmersiveEditionOf$|^IsNonImmersiveEditionOf$|^IsParentRelease$|^IsPhysicalEquivalentToDigital$|^IsReleaseFromRelease$|^IsShortenedFromAlbum$|^IsSlowedDownOf$|^IsSourceOfEditedVersion$|^IsSpedUpOf$|^IsVideoUsedFor$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsReleaseResourceTypeValidator;
impl AvsReleaseResourceTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^PrimaryResource$|^SecondaryResource$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsReleaseTypeValidator;
impl AvsReleaseTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Album$|^AlertToneRelease$|^AsPerContract$|^AudioBookRelease$|^AudioDramaRelease$|^BackCoverImageRelease$|^BookletBackImageRelease$|^BookletFrontImageRelease$|^BookletRelease$|^Bundle$|^ClassicalAlbum$|^ClassicalDigitalBoxedSet$|^ClassicalMultimediaAlbum$|^ConcertVideo$|^DigitalBoxSetRelease$|^DjMix$|^Documentary$|^Drama$|^DramaticoMusicalVideoRelease$|^EBookRelease$|^EP$|^Episode$|^FeatureFilm$|^KaraokeRelease$|^LiveEventVideo$|^LogoRelease$|^LongFormMusicalWorkVideoRelease$|^LongFormNonMusicalWorkVideoRelease$|^LyricSheetRelease$|^MultimediaAlbum$|^MultimediaDigitalBoxedSet$|^MultimediaSingle$|^MusicalWorkBasedGameRelease$|^NonMusicalWorkBasedGameRelease$|^PlayList$|^RingbackToneRelease$|^RingtoneRelease$|^Season$|^Series$|^SheetMusicRelease$|^ShortFilm$|^Single$|^SingleResourceRelease$|^StemBundle$|^UserDefined$|^VideoAlbum$|^VideoMastertoneRelease$|^VideoSingle$|^WallpaperRelease$|^TrackRelease$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsReleaseTypeERN4Validator;
impl AvsReleaseTypeERN4Validator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Album$|^AlertToneRelease$|^AsPerContract$|^AudioBookRelease$|^AudioDramaRelease$|^BackCoverImageRelease$|^BookletBackImageRelease$|^BookletFrontImageRelease$|^BookletRelease$|^Bundle$|^ClassicalAlbum$|^ClassicalDigitalBoxedSet$|^ClassicalMultimediaAlbum$|^ConcertVideo$|^DigitalBoxSetRelease$|^DjMix$|^Documentary$|^Drama$|^DramaticoMusicalVideoRelease$|^EBookRelease$|^EP$|^Episode$|^FeatureFilm$|^KaraokeRelease$|^LiveEventVideo$|^LogoRelease$|^LongFormMusicalWorkVideoRelease$|^LongFormNonMusicalWorkVideoRelease$|^LyricSheetRelease$|^MultimediaAlbum$|^MultimediaDigitalBoxedSet$|^MultimediaSingle$|^MusicalWorkBasedGameRelease$|^NonMusicalWorkBasedGameRelease$|^PlayList$|^RingbackToneRelease$|^RingtoneRelease$|^Season$|^Series$|^SheetMusicRelease$|^ShortFilm$|^Single$|^SingleResourceRelease$|^StemBundle$|^UserDefined$|^VideoAlbum$|^VideoMastertoneRelease$|^VideoSingle$|^WallpaperRelease$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsReleaseTypeMCNOTIFValidator;
impl AvsReleaseTypeMCNOTIFValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Album$|^EP$|^RingbackToneRelease$|^RingtoneRelease$|^Single$|^VideoAlbum$|^VideoSingle$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRequestMessagePurposeValidator;
impl AvsRequestMessagePurposeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^LicenseRequest$|^NdmaLicenseRequest$|^Notification$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRequestReasonValidator;
impl AvsRequestReasonValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DisputeResolutionRequest$|^GeneralRequest$|^PublisherAddition$|^PublisherChange$|^PublisherRemoval$|^Recall$|^ReleaseListUpdate$|^SpecificRequest$|^UserDefined$|^WriterAddition$|^WriterChange$|^WriterRemoval$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsResourceContributorRoleValidator;
impl AvsResourceContributorRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Accompanyist$|^Actor$|^AdditionalEngineer$|^AdditionalMixingEngineer$|^AdditionalPerformer$|^AdditionalProgrammingEngineer$|^AdditionalStudioProducer$|^AnchorPerson$|^AnimalTrainer$|^Animator$|^Annotator$|^Announcer$|^AAndRAdministrator$|^AAndRCoordinator$|^Armourer$|^ArtCopyist$|^ArtDirector$|^Artist$|^ArtistBackgroundVocalEngineer$|^ArtistVocalEngineer$|^ArtistVocalSecondEngineer$|^AssistantCameraOperator$|^AssistantChiefLightingTechnician$|^AssistantConductor$|^AssistantDirector$|^AssistantEditor$|^AssistantEngineer$|^AssistantProducer$|^AssistantVisualEditor$|^AssociatedPerformer$|^AssociateProducer$|^AuralTrainer$|^BackgroundVocalist$|^BalanceEngineer$|^BandLeader$|^Binder$|^BindingDesigner$|^BookDesigner$|^BookjackDesigner$|^BookplateDesigner$|^BookProducer$|^BroadcastAssistant$|^BroadcastJournalist$|^Calligrapher$|^CameraOperator$|^Carpenter$|^Cartographer$|^Cartoonist$|^CastingDirector$|^Causeur$|^Censor$|^ChiefLightingTechnician$|^Choir$|^ChoirMember$|^Choreographer$|^ChorusMaster$|^CircusArtist$|^ClapperLoader$|^ClubDJ$|^CoDirector$|^CoExecutiveProducer$|^ColorSeparator$|^Comedian$|^CoMixer$|^CoMixingEngineer$|^Commentator$|^CommissioningBroadcaster$|^CompilationProducer$|^ComputerGraphicCreator$|^ComputerProgrammer$|^ConcertMaster$|^Conductor$|^Consultant$|^ContinuityChecker$|^Contractor$|^CoProducer$|^Correspondent$|^CostumeDesigner$|^CoverDesigner$|^Dancer$|^Delineator$|^Designer$|^DialogueCoach$|^DialogueDirector$|^DigitalAudioWorkstationEngineer$|^DigitalEditingEngineer$|^DigitalEditingSecondEngineer$|^Director$|^DirectStreamDigitalEngineer$|^DistributionCompany$|^DJ$|^Draughtsman$|^Dresser$|^Dubber$|^Editor$|^EditorInChief$|^EditorOfTheDay$|^Encoder$|^Engineer$|^Ensemble$|^ExecutiveProducer$|^Expert$|^Facsimilist$|^FightDirector$|^FilmDirector$|^FilmDistributor$|^FilmEditor$|^FilmProducer$|^FilmSoundEngineer$|^FloorManager$|^FocusPuller$|^FoleyArtist$|^FoleyEditor$|^FoleyMixer$|^GraphicArtist$|^GraphicAssistant$|^GraphicDesigner$|^Greensman$|^Grip$|^GuestConductor$|^GroupMember$|^Hairdresser$|^Illustrator$|^ImmersiveMasteringEngineer$|^ImmersiveMixingEngineer$|^InitialProducer$|^InterviewedGuest$|^Interviewer$|^KeyCharacter$|^KeyGrip$|^KeyTalent$|^Leadman$|^LeadPerformer$|^LeadVocalist$|^LightingDirector$|^LightingTechnician$|^LocationManager$|^MakeUpArtist$|^Manufacturer$|^MasteringEngineer$|^MasteringSecondEngineer$|^MatteArtist$|^Mixer$|^MixingEngineer$|^MixingSecondEngineer$|^MusicArranger$|^MusicCopyist$|^MusicDirector$|^MusicGroup$|^Musician$|^Narrator$|^NewsProducer$|^NewsReader$|^NotSpecified$|^Orchestra$|^OrchestraMember$|^OriginalArtist$|^OverdubEngineer$|^OverdubSecondEngineer$|^Painter$|^Performer$|^Photographer$|^PhotographyDirector$|^PlaybackSinger$|^PostProducer$|^PreProduction$|^PreProductionEngineer$|^PreProductionSecondEngineer$|^Presenter$|^PrimaryMusician$|^ProductionAssistant$|^ProductionCompany$|^ProductionCoordinator$|^ProductionDepartment$|^ProductionManager$|^ProductionSecretary$|^ProjectEngineer$|^Programmer$|^ProgrammingEngineer$|^ProgramProducer$|^PropertyManager$|^PublishingDirector$|^Puppeteer$|^Pyrotechnician$|^RecordingEngineer$|^RecordingSecondEngineer$|^Redactor$|^ReissueProducer$|^RemixedArtist$|^Remixer$|^RemixingEngineer$|^RemixingSecondEngineer$|^Repetiteur$|^Researcher$|^ResearchTeamHead$|^ResearchTeamMember$|^Restager$|^Rigger$|^RightsControllerOnProduct$|^Runner$|^ScenicOperative$|^ScientificAdvisor$|^ScriptSupervisor$|^SecondAssistantCameraOperator$|^SecondAssistantDirector$|^SecondConductor$|^SecondEngineer$|^SecondUnitDirector$|^SeriesProducer$|^SetDesigner$|^SetDresser$|^SignLanguageInterpreter$|^Soloist$|^SoundDesigner$|^SoundMixer$|^SoundRecordist$|^SoundSupervisor$|^Speaker$|^SpecialEffectsTechnician$|^Sponsor$|^StageAssistantEngineer$|^StageDirector$|^StageEngineer$|^StoryTeller$|^StringEngineer$|^StringProducer$|^StringsDirector$|^StudioConductor$|^StudioMusician$|^StudioPersonnel$|^StudioProducer$|^Stunts$|^SubtitlesEditor$|^SubtitlesTranslator$|^SupportingActor$|^SurroundMixingEngineer$|^SurroundMixingSecondEngineer$|^TapeOperator$|^TechnicalDirector$|^Tonmeister$|^TrackingEngineer$|^TrackingSecondEngineer$|^TransfersAndSafetiesEngineer$|^TransfersAndSafetiesSecondEngineer$|^TransportationManager$|^Treatment/ProgramProposal$|^TypeDesigner$|^Unknown$|^UserDefined$|^VideoDirector$|^Videographer$|^VideoMusicalDirector$|^VideoProducer$|^VisionMixer$|^VisualEditor$|^VisualEffectsTechnician$|^VocalArranger$|^VocalEditingEngineer$|^VocalEditingSecondEngineer$|^VocalEngineer$|^Vocalist$|^VocalSecondEngineer$|^VocalProducer$|^VoiceActor$|^Wardrobe$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsResourceGroupTypeValidator;
impl AvsResourceGroupTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation =
            r"^Component$|^ComponentRelease$|^MultiWorkPart$|^ReleaseComponent$|^Side$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsResourceRelationshipTypeValidator;
impl AvsResourceRelationshipTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ContainsSamplesFrom$|^HasClip$|^HasContentFrom$|^HasPart$|^IsClipFrom$|^IsCoveredBy$|^IsCoverOf$|^IsDifferentEncoding$|^IsEditedVersionOf$|^IsImmersiveEditionOf$|^IsNonImmersiveEditionOf$|^IsPartOf$|^IsSampledBy$|^IsSlowedDownOf$|^IsSourceOfEditedVersion$|^IsSpedUpOf$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsResourceTypeValidator;
impl AvsResourceTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Image$|^MIDI$|^SheetMusic$|^Software$|^SoundRecording$|^Text$|^UserDefinedResource$|^Video$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsResourceTypeMCNOTIFValidator;
impl AvsResourceTypeMCNOTIFValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^SheetMusic$|^SoundRecording$|^Video$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsResourceTypeRDRValidator;
impl AvsResourceTypeRDRValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^MusicVideo$|^SoundRecording$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsResourceWorkRelationshipTypeValidator;
impl AvsResourceWorkRelationshipTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Interpellation$|^Medley$|^MultipleWorkResource$|^MusicalWorkWithSamples$|^SingleWorkResource$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRevenueAllocationTypeValidator;
impl AvsRevenueAllocationTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Episode$|^NonSerial$|^Season$|^Series$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRevenueSourceTypeValidator;
impl AvsRevenueSourceTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^FinancialRevenue$|^IndemnityRevenue$|^RoyaltyRevenue$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRhythmStyleValidator;
impl AvsRhythmStyleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^4OnTheFloor$|^Blues$|^BoogieWoogie$|^Calypso$|^Cumbia$|^Dembow$|^Disco$|^Flamenco$|^Merengue$|^Nyabinghi$|^OneDrop$|^Polyrhythm$|^RockAndRoll$|^Rumba$|^Shuffle$|^Skank$|^Tala$|^Tejano$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRightShareTypeValidator;
impl AvsRightShareTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^LicensingShare$|^MusicalWorkCollectionShare$|^MusicalWorkManuscriptShare$|^OriginalPublisherShare$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRightsClaimPolicyTypeValidator;
impl AvsRightsClaimPolicyTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^BlockAccess$|^Monetize$|^ReportUsage$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRightsClaimStatusValidator;
impl AvsRightsClaimStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation =
            r"^Conflict$|^DataInconsistent$|^NoConflict$|^PendingReview$|^Rejected$|^Revoked$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRightsControlTypeValidator;
impl AvsRightsControlTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ExclusiveLicensee$|^LocalPayee$|^OriginalOwner$|^RightsAdministrator$|^SuccessorInTitle$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRightsControllerRoleValidator;
impl AvsRightsControllerRoleValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AdministratingRecordCompany$|^LocalPayee$|^RightsAdministrator$|^RightsController$|^RightsHolder$|^RoyaltyAdministrator$|^Unknown$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRightsControllerTypeValidator;
impl AvsRightsControllerTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^OriginalOwner$|^SuccessorInTitle$|^ExclusiveLicensee$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRightsCoverageValidator;
impl AvsRightsCoverageValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^MakeAvailableRight$|^MechanicalRight$|^PerformingRight$|^PrintRight$|^ReproductionRight$|^SynchronizationRight$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRightsCoverageMWNLValidator;
impl AvsRightsCoverageMWNLValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^MechanicalRight$|^PerformingRight$|^PrintRight$|^SynchronizationRight$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRightsStatementProfileValidator;
impl AvsRightsStatementProfileValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^MandatedUsageRights$|^RightsStatement$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRinFileStatusValidator;
impl AvsRinFileStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^FileOK$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRinMessageTypeValidator;
impl AvsRinMessageTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^RecordingInformationNotification$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRinProposedActionTypeValidator;
impl AvsRinProposedActionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Resubmit$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRootChordNoteValidator;
impl AvsRootChordNoteValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation =
            r"^A$|^Ab/G#$|^B$|^Bb/A#$|^C$|^C#/Db$|^D$|^E$|^Eb/D#$|^F$|^G$|^Gb/F#$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRootChordQualityValidator;
impl AvsRootChordQualityValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AugmentedSeventh$|^AugmentedTriad$|^DiminishedSeventh$|^DiminishedTriad$|^DominantSeventh$|^Fifth$|^HalfDiminishedSeventh$|^MajorSeventh$|^MajorSixth$|^MajorTriad$|^MajorMinorSeventh$|^MinorSeventh$|^MinorSixth$|^MinorTriad$|^Suspended$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRoyaltyRateCalculationTypeValidator;
impl AvsRoyaltyRateCalculationTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ControlledCompositionRoyaltyRate$|^MinimumStatutoryRoyaltyRate$|^NegotiatedRoyaltyRate$|^ReducedStatutoryRoyaltyRate$|^StatutoryRoyaltyRate$|^PPD$|^RetailPrice$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsRoyaltyRateTypeValidator;
impl AvsRoyaltyRateTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^PennyRate$|^PercentageRoyaltyRate$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsSessionTypeValidator;
impl AvsSessionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ArtistVocals$|^Demo$|^DigitalEditing$|^Editing$|^LivePerformance$|^Mastering$|^Mixing$|^Overdub$|^PreProduction$|^Preservation$|^Production$|^Project$|^Recording$|^Remixing$|^Tracking$|^Transfer$|^TransfersAndSafeties$|^UserDefined$|^Vocal$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsSheetMusicCodecTypeValidator;
impl AvsSheetMusicCodecTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsSheetMusicTypeValidator;
impl AvsSheetMusicTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsSoftwareTypeValidator;
impl AvsSoftwareTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^InteractiveBooklet$|^MusicalWorkBasedGame$|^NonMusicalWorkBasedGame$|^Screensaver$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsSoundRecordingTypeValidator;
impl AvsSoundRecordingTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AudioStem$|^Clip$|^MusicalWorkReadalongSoundRecording$|^MusicalWorkSoundRecording$|^NonMusicalWorkReadalongSoundRecording$|^NonMusicalWorkSoundRecording$|^SpokenWordSoundRecording$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsStatusValidator;
impl AvsStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AssetsNeeded$|^AwaitingMaterials$|^BackedUp$|^Canceled$|^Closed$|^Completed$|^InWork$|^NotStarted$|^UserDefined$|^Verified$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsSubGenreValidator;
impl AvsSubGenreValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AcousticChicagoBlues$|^BoogieWoogie$|^BritishBlues$|^ChicagoBlues$|^ClassicFemaleBlues$|^CountryBlues$|^DeltaBlues$|^ElectricTexasBlues$|^HillCountryBlues$|^JumpBlues$|^ModernBlues$|^NewOrleansBlues$|^PianoBlues$|^PiedmontBlues$|^Roots$|^SwampBlues$|^TexasBlues$|^TraditionalAcoustic$|^TraditionalElectric$|^WestCoastBlues$|^20thCentury$|^21stCentury$|^Acousmatic$|^AmbrosianChant$|^ArsAntiqua$|^ArsNova$|^Baroque$|^ByzantineChant$|^Classical$|^ClassicalCrossover$|^Contemporary$|^Early20thCentury$|^EarlyBaroque$|^EarlyElectronic$|^EarlyRenaissance$|^EarlyRomantic$|^ExperimentalClassical$|^Expressionism$|^FirstVienneseSchool$|^Futurism$|^GregorianChant$|^Impressionism$|^Late20thCentury$|^LateBaroque$|^LateRenaissance$|^LateRomantic$|^LightMusic$|^Medieval$|^Middle20thCentury$|^MiddleBaroque$|^MiddleRenaissance$|^MiddleRomantic$|^Minimalism$|^Modernism$|^MusiqueConcrete$|^Nationalist$|^NeoClassical$|^NeoRomantic$|^OrchestralFusion$|^Organum$|^Plainsong$|^PostClassical$|^PostMinimalism$|^PreClassical$|^Renaissance$|^Romantic$|^Serialism$|^Spectralism$|^AlternativeCountry$|^Americana$|^BakersfieldSound$|^Bluegrass$|^CountryPop$|^CountryRap$|^CountryRock$|^HonkyTonk$|^ModernCountry$|^NashvilleSound$|^NeoTraditionalCountry$|^OutlawCountry$|^TexasCountry$|^TraditionalCountry$|^WesternSwing$|^2StepGarage$|^AcidHouse$|^AcidTechno$|^Ambient$|^AmbientHouse$|^Bassline$|^BigBeat$|^Breakbeat$|^BrokenBeat$|^ChicagoHouse$|^DeepHouse$|^DetroitHouse$|^DetroitTechno$|^DigitalHardcore$|^Downtempo$|^DrillNBass$|^DrumNBass$|^Drumstep$|^DubstepUK$|^DubstepUS$|^Dubtronica$|^DutchHouse$|^EBM$|^Electro$|^ElectroHouse$|^Electronica/Eclectic$|^Eurodance$|^ExperimentalElectronic$|^FrenchHouse$|^FutureGarage$|^Gabba$|^GarageHouse$|^Glitch$|^HappyHardcore$|^HardTrance$|^Hardcore$|^HardcoreBreakbeat$|^Hardstyle$|^HipHouse$|^House$|^IDM$|^JazzHouse$|^Jungle$|^Kwaito$|^LatinHouse$|^MinimalHouse$|^MinimalTechno$|^NoiseMusic$|^NuDisco$|^ProgressiveHouse$|^ProgressiveTrance$|^PsychedelicTrance$|^Schranz$|^SpeedGarage$|^Synthwave$|^TechHouse$|^Techno$|^Trance$|^TribalHouse$|^TripHop$|^UKFunky$|^UKGarage$|^VocalHouse$|^AmericanFolk$|^AmericanPrimitiveGuitar$|^BarbershopMusic$|^BritishFolk$|^CanadianFiddling$|^CanadianFolk$|^Celtic$|^ElectricFolk$|^EnglishFolk$|^FolkBaroque$|^FolkRevival$|^IndieFolk$|^IrishFolk$|^NorthAmericanFolk$|^OldTime$|^ScottishFolk$|^SeaShanties$|^WelshFolk$|^Zydeco$|^ClassicGospel$|^SouthernGospel$|^AlternativeRap$|^ChristianRap$|^ClassicHipHop$|^ConsciousRap$|^Crunk$|^DirtyRap$|^EastCoastHipHop$|^ExperimentalHipHop$|^FunkCarioca$|^GFunk$|^GangstaRap$|^GoldenAge$|^Grime$|^HardcoreRap$|^InstrumentalHipHop$|^MiamiBass$|^PopRap$|^SouthernRap$|^Trap$|^WestCoastHipHop$|^AcidJazz$|^AfricanJazz$|^AvantGardeJazz$|^Bebop$|^BossaNova$|^BritishDanceBand$|^CapeJazz$|^CoolJazz$|^Dixieland$|^EthiopianJazz$|^FreeJazz$|^GypsyJazz$|^HardBop$|^JazzBlues$|^JazzFunk$|^JazzFusion$|^JazzRock$|^JazzPop$|^LatinJazz$|^ModalJazz$|^ModernCreative$|^ModernJazz$|^PostBop$|^SmoothJazz$|^SoulJazz$|^Swing$|^SwingRevival$|^TraditionalJazz$|^TraditionalPop$|^Bachata$|^Banda$|^Boogaloo$|^Brazilian$|^Conjunto$|^Corridos$|^Duranguense$|^Grupera$|^Hupango$|^Mariachi$|^NewMexicoMusic$|^Norteno$|^Ranchera$|^Reggaeton$|^RegionalMexican$|^Salsa$|^Sertanejo$|^Tejano$|^AfricanPop$|^Afrobeat$|^AlternativeDance$|^AlternativePop$|^Axe$|^Bikutsi$|^Bollywood$|^BrazilianPop$|^Brega$|^CantoPop$|^CaribbeanPop$|^ChamberPop$|^Chimurenga$|^ChinesePop$|^DreamPop$|^Electroclash$|^ElectronicPop$|^Enka$|^FilipinoPop$|^Folktronica$|^FrenchPop$|^GermanPop$|^GreekPop$|^Highlife$|^Hiplife$|^HokkienPop$|^IndianPop$|^IndiePop$|^Indietronica$|^IndonesianPop$|^JapanesePop$|^Kayokyoku$|^Kizomba$|^KoreanPop$|^Kuduro$|^LatinFreestyle$|^Madchester$|^Makossa$|^MandoPop$|^Mbalax$|^Mbaqanga$|^ModernLaiko$|^ModernPop$|^MPB$|^MusicOfThePhilippines$|^NDW$|^NeoPsychedelia$|^NewRomantic$|^NewWave$|^NoisePop$|^NouvelleChanson$|^PopRock$|^PsychedelicPop$|^Schlager$|^Soca$|^Soukous$|^SynthPop$|^Telugu$|^TweePop$|^AfroFunk$|^AlternativeR'n'B$|^BlueEyedSoul$|^Boogie$|^ChicagoSoul$|^ClassicR'n'B$|^ContemporaryR'n'B$|^DeepSoul$|^Disco$|^DooWop$|^Funk$|^GoGo$|^HiNRG$|^ItaloDisco$|^MemphisSoul$|^MinneapolisFunk$|^ModernR'n'B$|^MotownSound$|^NeoSoul$|^NewJackSwing$|^NewOrleansR'n'B$|^OGFunk$|^PFunk$|^Phillysound$|^PopFunk$|^PsychedelicSoul$|^QuietStorm$|^RetroSoul$|^Soul$|^SouthernSoul$|^TraditionalR'n'B$|^UrbanContemporaryGospel$|^WestCoastSoul$|^Dancehall$|^Dub$|^Rocksteady$|^RootsReggae$|^Ska$|^AfroRock$|^AltMetal$|^AlternativeRock$|^ArtRock$|^BlackMetal$|^BoogieRock$|^BritRock$|^BritishInvasion$|^BritPop$|^ClassicRock$|^DarkWave$|^DeathMetal$|^DoomMetal$|^EarlyRock$|^ElectroGoth$|^EmoRock$|^ExperimentalRock$|^FunkMetal$|^GarageRock$|^GlamRock$|^GothicMetal$|^GothicRock$|^Grindcore$|^Grunge$|^HairMetal$|^HardRock$|^HardcorePunk$|^HeartlandRock$|^IndieRock$|^Industrial$|^IndustrialMetal$|^JovemGuarda$|^Krautrock$|^MathRock$|^Merseybeat$|^Metal$|^Metalcore$|^NoWave$|^NoiseRock$|^NuMetal$|^Oi!$|^PopPunk$|^PostGrunge$|^PostRock$|^PostHardcore$|^PostPunk$|^PowerPop$|^ProgressiveMetal$|^ProgressiveRock$|^ProtoPunk$|^PsychedelicRock$|^Psychobilly$|^Punk$|^RiotGrrrl$|^RockNRoll$|^Rockabilly$|^SambaRock$|^Screamo$|^Shoegaze$|^SoftRock$|^SouthAmericanRock$|^SouthernRock$|^SpaceRock$|^SpeedMetal$|^StonerRock$|^Surf$|^SwampRock$|^ThirdWaveSka$|^ThrashMetal$|^Tropicalia$|^TwoTone$|^Underground$|^Commentary$|^Conversation$|^Interview$|^Monologue$|^Poetry$|^Skit$|^StandUpComedy$|^Afoxe$|^AfricanMusic$|^ArgentinianMusic$|^BalineseMusic$|^BrazilianMusic$|^Calypso$|^CapoeiraMusic$|^CaribbeanMusic$|^CarnaticMusic$|^ChaChaCha$|^ChileanMusic$|^Choro$|^ColombianMusic$|^Contradanza$|^CubanMusic$|^Cueca$|^Cumbia$|^CzechMusic$|^Danzon$|^Dimotiko$|^DominicanMusic$|^Fado$|^Flamenco$|^FrenchMusic$|^GauchoMusic$|^GermanMusic$|^GreekMusic$|^Guaracha$|^HindustaniClassicalMusic$|^IndianMusic$|^IndonesianMusic$|^IrishMusic$|^ItalianMusic$|^JamaicanMusic$|^JavaneseMusic$|^JewishMusic$|^JugEnsemble$|^Klezmer$|^Mambo$|^Maracatu$|^Merengue$|^MexicanMusic$|^MiddleEasternMusic$|^NeapolitanSong$|^NorthAmericanMusic$|^PakistaniMusic$|^Palo$|^Pilon$|^Polka$|^PortugueseMusic$|^PuertoRicanMusic$|^PunjabiMusic$|^Ragtime$|^Repente$|^RomanianMusic$|^Salves$|^Samba$|^Son$|^SouthAmericanMusic$|^SpanishMusic$|^Spiritual$|^Tango$|^TibetanMusic$|^Tonada$|^TurkishMusic$|^TurkishClassicalMusic$|^WorkSongs$|^Exotica$|^NewAge$|^NewFlamenco$|^WorldFusionJazz$|^Worldbeat$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsSubTitleTypeValidator;
impl AvsSubTitleTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Location$|^Version$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsSummaryTypeValidator;
impl AvsSummaryTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Contributor$|^RightsController$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsSupplyChainStatusValidator;
impl AvsSupplyChainStatusValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DeliveredToReleaseDistributor$|^InDeliveryToReleaseDistributor$|^InPreparationForDeliveryToReleaseDistributor$|^OrderPlacedForReleaseDistributor$|^ProcessingErrorAtReleaseCreator$|^ProcessingErrorAtReleaseDistributor$|^ReleaseMadeAvailableToConsumers$|^ReleaseNotAvailable$|^ReleaseReceivedByReleaseDistributor$|^ReleaseStagedForPublication$|^ReleaseViolatesTermsOfService$|^RightsConflict$|^SuccessfullyIngestedByReleaseDistributor$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTempoValidator;
impl AvsTempoValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Adagietto$|^Adagio$|^Adagissimo$|^Allegretto$|^Allegrissimo$|^Allegro$|^AllegroModerato$|^Andante$|^AndanteModerato$|^Andantino$|^Grave$|^Larghetto$|^Larghissimo$|^Largo$|^Lento$|^MarciaModerato$|^Moderato$|^Prestissimo$|^Presto$|^UserDefined$|^Vivace$|^Vivacissimo$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTerritoryCodeValidator;
impl AvsTerritoryCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AD$|^AE$|^AF$|^AG$|^AI$|^AL$|^AM$|^AN$|^AO$|^AQ$|^AR$|^AS$|^AT$|^AU$|^AW$|^AX$|^AZ$|^BA$|^BB$|^BD$|^BE$|^BF$|^BG$|^BH$|^BI$|^BJ$|^BL$|^BM$|^BN$|^BO$|^BQ$|^BR$|^BS$|^BT$|^BV$|^BW$|^BY$|^BZ$|^CA$|^CC$|^CD$|^CF$|^CG$|^CH$|^CI$|^CK$|^CL$|^CM$|^CN$|^CO$|^CR$|^CS$|^CU$|^CV$|^CW$|^CX$|^CY$|^CZ$|^DE$|^DJ$|^DK$|^DM$|^DO$|^DZ$|^EC$|^EE$|^EG$|^EH$|^ER$|^ES$|^ES-CE$|^ES-CN$|^ES-ML$|^ET$|^FI$|^FJ$|^FK$|^FM$|^FO$|^FR$|^GA$|^GB$|^GD$|^GE$|^GF$|^GG$|^GH$|^GI$|^GL$|^GM$|^GN$|^GP$|^GQ$|^GR$|^GS$|^GT$|^GU$|^GW$|^GY$|^HK$|^HM$|^HN$|^HR$|^HT$|^HU$|^ID$|^IE$|^IL$|^IM$|^IN$|^IO$|^IQ$|^IR$|^IS$|^IT$|^JE$|^JM$|^JO$|^JP$|^KE$|^KG$|^KH$|^KI$|^KM$|^KN$|^KP$|^KR$|^KW$|^KY$|^KZ$|^LA$|^LB$|^LC$|^LI$|^LK$|^LR$|^LS$|^LT$|^LU$|^LV$|^LY$|^MA$|^MC$|^MD$|^ME$|^MF$|^MG$|^MH$|^MK$|^ML$|^MM$|^MN$|^MO$|^MP$|^MQ$|^MR$|^MS$|^MT$|^MU$|^MV$|^MW$|^MX$|^MY$|^MZ$|^NA$|^NC$|^NE$|^NF$|^NG$|^NI$|^NL$|^NO$|^NP$|^NR$|^NU$|^NZ$|^OM$|^PA$|^PE$|^PF$|^PG$|^PH$|^PK$|^PL$|^PM$|^PN$|^PR$|^PS$|^PT$|^PW$|^PY$|^QA$|^RE$|^RO$|^RS$|^RU$|^RW$|^SA$|^SB$|^SC$|^SD$|^SE$|^SG$|^SH$|^SI$|^SJ$|^SK$|^SL$|^SM$|^SN$|^SO$|^SR$|^SS$|^ST$|^SV$|^SX$|^SY$|^SZ$|^TC$|^TD$|^TF$|^TG$|^TH$|^TJ$|^TK$|^TL$|^TM$|^TN$|^TO$|^TR$|^TT$|^TV$|^TW$|^TZ$|^UA$|^UG$|^UM$|^US$|^UY$|^UZ$|^VA$|^VC$|^VE$|^VG$|^VI$|^VN$|^VU$|^WF$|^WS$|^YE$|^YT$|^ZA$|^ZM$|^ZW$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTerritoryCodeTypeValidator;
impl AvsTerritoryCodeTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ISO$|^TIS$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTerritoryCodeTypeIncludingDeprecatedCodesValidator;
impl AvsTerritoryCodeTypeIncludingDeprecatedCodesValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^DeprecatedISO$|^ISO$|^TIS$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTextCodecTypeValidator;
impl AvsTextCodecTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ASCII$|^AsciiOrIso8859nText$|^EBU-TT$|^EnhancedLRC$|^EPUB$|^HTML$|^LRC$|^MicrosoftWord$|^OpenDocumentText$|^OOXML$|^PDF$|^PostScript$|^RTF$|^SimpleLRC$|^SRT$|^TTML$|^Unknown$|^UserDefined$|^UTF8Text$|^VTT$|^WindowsText$|^XHTML$|^XML$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTextTypeValidator;
impl AvsTextTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Caption$|^ClosedCaption$|^EBook$|^LinerNotes$|^LyricText$|^NonInteractiveBooklet$|^SubTitle$|^TextDocument$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTextTypeATOMValidator;
impl AvsTextTypeATOMValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^text$|^html$|^xhtml$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsThemeValidator;
impl AvsThemeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Abortion$|^Above$|^Accuse$|^Action$|^Activities$|^Actor$|^AddictedTo$|^Addiction$|^Adolescence$|^Adoption$|^Adoration$|^Advice$|^Affection$|^Afghanistan$|^Afraid$|^Africa$|^Afternoon$|^Air$|^Airplanes$|^Alabama$|^Alaska$|^Albania$|^Alcohol$|^Algeria$|^Alien$|^All$|^Alligator$|^Alone$|^Always$|^Ambition$|^Ambivalent$|^Ammunition$|^Amsterdam$|^AmusementParksAndRides$|^Andorra$|^Angel$|^Anger$|^Angola$|^Angst$|^Animals$|^Anniversary$|^AntiDrug$|^AntiguaAndBarbuda$|^Anxious$|^Anything$|^Apologize$|^April$|^Argentina$|^Arizona$|^Arkansas$|^Armageddon$|^Armenia$|^Arms$|^Art$|^Ashes$|^Asia$|^Assurance$|^Astrology$|^Astronaut$|^AtFirstSight$|^Atlanta$|^Attitude$|^Attractive$|^August$|^Aunt$|^Australia$|^Austria$|^Autumn$|^Awareness$|^Azerbaijan$|^Baby$|^Back$|^BackTogether$|^Bad$|^Bahamas$|^Bahrain$|^Bali$|^Balkans$|^Ballerina$|^BalticStates$|^Baltimore$|^Bangladesh$|^Barbados$|^Bars$|^Baseball$|^Basketball$|^Bass$|^Bathroom$|^BattleOfTheSexes$|^Bay$|^Beach$|^Bear$|^Beats$|^Beautiful$|^Beauty$|^Bedroom$|^Beg$|^Beginning$|^Behind$|^BeingIn$|^Belarus$|^Belgium$|^Belief$|^Believe$|^Belize$|^Bells$|^Belly$|^Below$|^Benelux$|^Benin$|^Berlin$|^Bermuda$|^Best$|^Betray$|^Beverage$|^Bhutan$|^Bicycles$|^Big$|^Bird$|^Birmingham$|^Birth$|^BirthControl$|^BirthdayParty$|^Black$|^Blame$|^Bless$|^Blonde$|^Blood$|^Blue$|^Boardwalk$|^Body$|^BodyLanguage$|^BodyParts$|^Bolivia$|^Book$|^Bored$|^BosniaAndHerzegovina$|^Boss$|^Boston$|^Botswana$|^Bounce$|^Boxing$|^Boys$|^Brag$|^Brain$|^Brass$|^Brazil$|^Breakdown$|^Breakup$|^Breathe$|^Bridge$|^Britain$|^BritishIsles$|^BrokenHome$|^Brother$|^Brown$|^Brunei$|^Brunette$|^Buddha$|^Bug$|^Build$|^Building$|^Bulgaria$|^Burden$|^BurkinaFaso$|^Burning$|^Burundi$|^Bus$|^Busy$|^Butterfly$|^California$|^CallOut$|^Calmness$|^Cambodia$|^Camera$|^Cameroon$|^Canada$|^Candle$|^Candy$|^CantGetOver$|^CantResist$|^CapeVerde$|^CapitalPunishment$|^CardGame$|^Carefree$|^Carnival$|^Carolinas$|^CarRacing$|^Cars$|^Casino$|^Cat$|^Celebration$|^Celebrity$|^CellPhone$|^Cemetary$|^CentralAfricanRepublic$|^CentralAmerica$|^Chad$|^Challenge$|^Change$|^Charity$|^Chase$|^Cheerleader$|^Chicago$|^Children$|^Chile$|^China$|^Choices$|^Choose$|^Christmas$|^Church$|^Cincinnati$|^Circus$|^City$|^CityLife$|^CivilRights$|^Clean$|^Cliff$|^Climb$|^Close$|^Clothing$|^Clown$|^Club$|^Coast$|^Cold$|^Colombia$|^Color$|^Colorado$|^Comfort$|^Comic$|^ComingHome$|^Commitment$|^Communication$|^Comoros$|^Compassion$|^Competitive$|^Complain$|^Compliments$|^Computer$|^Confidence$|^Conflict$|^Confused$|^Connecticut$|^Connection$|^Consciousness$|^Consistent$|^Container$|^Contempt$|^Continent$|^Conversation$|^CookIslands$|^Cool$|^Cosmetic$|^CostaRica$|^Country$|^CountryLife$|^Couple$|^Courage$|^Cousin$|^Cow$|^CowboyAndCowgirl$|^Crash$|^Crave$|^Crazy$|^Create$|^Crime$|^Criticize$|^Croatia$|^Crocodile$|^Crucifixion$|^Cruel$|^Crush$|^Cry$|^Cuba$|^Cyprus$|^Czechoslovakia$|^CzechRepublic$|^Dallas$|^Dance$|^DanceParty$|^Danger$|^Darkness$|^Date$|^Dating$|^Daughter$|^Dawn$|^Day$|^Daydream$|^Daytime$|^Death$|^December$|^Deep$|^Defeat$|^Defeated$|^Delaware$|^DemocraticRepublicOfTheCongo$|^Denmark$|^Denver$|^Desert$|^Desire$|^Despair$|^Desperate$|^Determination$|^Detroit$|^Devil$|^Difficult$|^Dinner$|^Dinosaur$|^Direction$|^Disappointment$|^Discovery$|^Disease$|^Dissatisfaction$|^Distance$|^Divorce$|^Dizzy$|^Djibouti$|^Doctor$|^Dog$|^Dolphin$|^DomesticViolence$|^Dominica$|^DominicanRepublic$|^DontGo$|^DontLeaveMe$|^Door$|^Down$|^DraftResistance$|^Dream$|^Drift$|^Drink$|^Drive$|^Drug$|^Drums$|^Dublin$|^Eagle$|^Ears$|^Earth$|^Earthquake$|^East$|^EasternEurope$|^Ecstatic$|^Ecuador$|^Education$|^Ego$|^Egypt$|^Elements$|^Elephant$|^ElSalvador$|^Elvis$|^Embarrassed$|^Emergency$|^Empowered$|^Empty$|^End$|^Enemy$|^Engagement$|^England$|^Enlightenment$|^Enlistment$|^Environment$|^Equality$|^EquatorialGuinea$|^Eritrea$|^Escape$|^Espionage$|^Estonia$|^Eswatini$|^Eternity$|^Ethiopia$|^Europe$|^EuropeanContinent$|^EuropeanEconomicArea$|^EuropeanEconomicCommunity$|^EuropeanUnion$|^Event$|^Everyday$|^Everything$|^Evil$|^Excellence$|^Excited$|^Excuse$|^Exercise$|^Explore$|^Eyes$|^Fabric$|^Face$|^Failure$|^Fair$|^Faith$|^FaithInDoubt$|^Fake$|^Fall$|^FallingIn$|^Fame$|^Family$|^FamilyLife$|^FamilyMember$|^Fantasy$|^Far$|^Farewell$|^Farm$|^Fascism$|^Fashion$|^Fast$|^Fat$|^Fate$|^Father$|^Favorite$|^February$|^Feel$|^Feelings$|^Feet$|^Feminism$|^Fever$|^Fight$|^Fiji$|^Find$|^Fingers$|^Finland$|^Fire$|^Fish$|^Fishing$|^Fix$|^Flirt$|^Flood$|^FloorAndCeiling$|^Florida$|^Flower$|^Fly$|^Focus$|^Follow$|^Food$|^Fool$|^Football$|^Forbidden$|^Forest$|^Forget$|^Forgive$|^FortuneTeller$|^Found$|^Fowl$|^Fox$|^Fragile$|^France$|^Free$|^Freedom$|^FrenchCaribbean$|^FrenchGuiana$|^FrenchPolynesia$|^Friday$|^Friendship$|^Frog$|^Fruits$|^Fun$|^Funeral$|^Fur$|^Furniture$|^Future$|^Gabon$|^Gambia$|^Gambling$|^Game$|^GangstaLife$|^Garbage$|^Garden$|^Gasoline$|^GasStation$|^Geek$|^Gender$|^General$|^Genocide$|^Gentle$|^GeographicLocation$|^Geography$|^Georgia$|^Germany$|^Gestures$|^GetLost$|^GettingOver$|^Ghana$|^Ghost$|^Girls$|^Give$|^Go$|^God$|^Gold$|^Golf$|^Good$|^GoodLife$|^Goodnight$|^Gossip$|^Government$|^Grandparent$|^Gratitude$|^GreatBritain$|^Greece$|^Greed$|^Green$|^Grenada$|^Grey$|^GrowOlder$|^GrowUp$|^Guatemala$|^Guilt$|^Guinea$|^Guinea-Bissau$|^Guitar$|^Gun$|^Guyana$|^Gypsy$|^Hair$|^Haiti$|^Halloween$|^Hands$|^Handsome$|^HangOut$|^Happiness$|^Harbor$|^Hat$|^Hate$|^Havana$|^Hawaii$|^Head$|^Healing$|^HealthAndWellness$|^Heart$|^Heartache$|^Heaven$|^Hell$|^Hello$|^Help$|^Hero$|^Heroism$|^Hide$|^High$|^Highest$|^Highland$|^Highway$|^Hillbilly$|^Hills$|^Hippie$|^Hips$|^History$|^HoldingOn$|^Hole$|^Holiday$|^Holland$|^Hollywood$|^Home$|^Honduras$|^Honesty$|^Honeymoon$|^HongKong$|^Hope$|^Horn$|^Horror$|^Horse$|^HorseRacing$|^Hospital$|^Hot$|^Hotel$|^House$|^Houston$|^Human$|^Hundred$|^Hungary$|^Husband$|^Iceland$|^Idaho$|^Idea$|^Ideal$|^Ideas$|^Identity$|^IdentityCrisis$|^Ignorance$|^Illinois$|^Immortality$|^Inch$|^Independence$|^India$|^Indiana$|^Individuality$|^Indonesia$|^Infatuated$|^Information$|^InFront$|^Innocence$|^Insect$|^Insecure$|^Insight$|^Insomnia$|^Inspiration$|^Insult$|^Integrity$|^Intention$|^Intoxicated$|^InTrouble$|^Invisible$|^Iowa$|^Iran$|^Iraq$|^Ireland$|^Island$|^IsleOfMan$|^Israel$|^Italy$|^IvoryCoast$|^Jamaica$|^January$|^Japan$|^Java$|^Jealous$|^JesusChrist$|^Jewelry$|^Jordan$|^Jukebox$|^July$|^Jump$|^June$|^Justice$|^Kangaroo$|^Kansas$|^KansasCity$|^Karaoke$|^Karma$|^Kazakhstan$|^Kentucky$|^Kenya$|^Keys$|^Kiribati$|^Kiss$|^Knees$|^Knowledge$|^Kuwait$|^Kyrgyzstan$|^Lake$|^Laos$|^Lasting$|^LastNight$|^LasVegas$|^Late$|^Latvia$|^Laughter$|^LawAndOrder$|^LawEnforcement$|^Lazy$|^Leader$|^Learn$|^Leave$|^Lebanon$|^Lecturing$|^Left$|^Legs$|^Lesotho$|^Liberia$|^Libya$|^Liechtenstein$|^Lies$|^Light$|^Lighthouse$|^Lightning$|^Lion$|^Lips$|^Listening$|^Lithuania$|^Living$|^Location$|^Lock$|^London$|^Lonely$|^Longing$|^Look$|^LosAngeles$|^LosingYou$|^Loss$|^Lost$|^LostThatLovingFeeling$|^Louisiana$|^Love$|^Lovely$|^Low$|^Loyalty$|^Lucky$|^Luxembourg$|^Macao$|^Madagascar$|^Magic$|^Magician$|^Mail$|^Maine$|^MakingLove$|^Malawi$|^Malaysia$|^Maldives$|^Mali$|^Malta$|^Mammal$|^Manipulate$|^March$|^MardiGras$|^MarriedLife$|^MarshallIslands$|^Maryland$|^Massachusetts$|^Mauritania$|^Mauritius$|^May$|^Me$|^Mean$|^Measurement$|^Medellín$|^Medical$|^Meditation$|^Memory$|^Memphis$|^Men$|^MensNames$|^MentalIllness$|^Mentality$|^MenTalkingToMen$|^MenTalkingToWomen$|^Mercy$|^Method$|^Mexico$|^Miami$|^Michigan$|^Micronesia$|^MiddleEast$|^Midnight$|^Mile$|^Military$|^Million$|^Mind$|^Mine$|^Minnesota$|^Miracle$|^Misbehavior$|^Misplace$|^MissingYou$|^Mississippi$|^Missouri$|^Mistakes$|^Mobile$|^Moldova$|^Monaco$|^Monday$|^Money$|^Mongolia$|^Monkey$|^Monster$|^Montana$|^Montenegro$|^Month$|^Moon$|^Morality$|^Morning$|^Morocco$|^Moscow$|^Mother$|^Motion$|^Motivation$|^Motorcycles$|^Mountain$|^Mourning$|^Mouse$|^Mouth$|^Move$|^Movie$|^MovingOn$|^Mozambique$|^Mule$|^MultipleBodyParts$|^Murder$|^Music$|^MusicalInstrument$|^MusicBusiness$|^Myanmar$|^Mystery$|^Namibia$|^Nashville$|^Nature$|^Nauru$|^Near$|^Nebraska$|^Neck$|^Need$|^Nepal$|^Nervous$|^Netherlands$|^Nevada$|^New$|^NewDay$|^NewHampshire$|^NewJersey$|^NewMexico$|^NewOrleans$|^News$|^NewYear$|^NewYork$|^NewYorkCity$|^NewZealand$|^Nicaragua$|^Niger$|^Nigeria$|^Night$|^Nightingale$|^Nightmare$|^None$|^NonRomantic$|^Noon$|^North$|^NorthAmerica$|^NorthDakota$|^NorthKorea$|^NorthMacedonia$|^Norway$|^Nostalgia$|^NotCommitted$|^NotMyType$|^November$|^Now$|^NuclearEnergy$|^NuclearWar$|^Number$|^Objects$|^Obsession$|^Ocean$|^Oceania$|^October$|^Ohio$|^Oklahoma$|^Old$|^Olympics$|^Oman$|^OneNightStand$|^Opinions$|^Opportunity$|^Oppression$|^Optimism$|^Orange$|^Oregon$|^Orphan$|^Outdoor$|^Outlaw$|^PacificIslands$|^Pad$|^Pain$|^Pakistan$|^Palau$|^Panama$|^Panic$|^Paper$|^PapuaNewGuinea$|^Parade$|^Paraguay$|^Parent$|^Paris$|^Park$|^Party$|^Past$|^Patience$|^Patriotism$|^Paw$|^Peace$|^Pennsylvania$|^People$|^Percussion$|^Perfection$|^Persevere$|^Peru$|^Pharmaceutical$|^Philadelphia$|^Philippines$|^Philosophy$|^Phone$|^Photograph$|^PhysicalPain$|^Piano$|^Pig$|^Pink$|^Pirate$|^Place$|^Planet$|^Platonic$|^Player$|^PlayMusic$|^Please$|^Poison$|^Poland$|^Political$|^PoliticalState$|^Politics$|^Polynesia$|^Pony$|^Porpoise$|^Portugal$|^Possessed$|^Possibility$|^Poverty$|^PowerAndControl$|^Prairie$|^Prayer$|^Pregnancy$|^Prejudice$|^Present$|^President$|^Pretty$|^Pride$|^Prison$|^Privacy$|^Problems$|^ProductAndBrand$|^Promise$|^Prophecy$|^Protect$|^Protest$|^PuertoRico$|^Purple$|^Qatar$|^Quality$|^Question$|^Questioning$|^Rabbit$|^Racism$|^Radio$|^Rain$|^Rainbow$|^Ranch$|^RapGame$|^Rat$|^Ready$|^Real$|^Rear$|^Rebellion$|^Recovery$|^Red$|^Redemption$|^Redhead$|^Reflect$|^Regret$|^Rehab$|^Rejection$|^Relax$|^Religion$|^Remember$|^Repeat$|^Reptile$|^RepublicOfTheCongo$|^Rescue$|^Resilient$|^Respect$|^Restaurant$|^Return$|^Reunite$|^Revenge$|^Revolution$|^Revolve$|^RhodeIsland$|^Rhythm$|^Ride$|^Right$|^RioDeJaneiro$|^Risk$|^River$|^Road$|^RoadAccident$|^RoadTrip$|^Robot$|^Rock$|^Rodeo$|^Romance$|^Romania$|^Romantic$|^Rome$|^Royalty$|^Run$|^Russia$|^Rwanda$|^Sad$|^Safety$|^Sail$|^SaintKittsAndNevis$|^SaintLucia$|^SaintVincentAndTheGrenadines$|^SaltLakeCity$|^Same$|^Samoa$|^SanAntonio$|^SanFrancisco$|^SanMarino$|^SantaClaus$|^SantaFe$|^SaoTomeAndPrincipe$|^Satisfaction$|^Saturday$|^SaudiArabia$|^Savannah$|^Save$|^Scandinavia$|^School$|^Scotland$|^Scream$|^Sea$|^Search$|^SearchingFor$|^Season$|^Secrets$|^Seduced$|^Segregation$|^Senegal$|^Sensuality$|^September$|^Serbia$|^Seychelles$|^Shake$|^Shame$|^Shark$|^Sheep$|^Shelter$|^Shine$|^Ship$|^Shoe$|^Shoot$|^Shopping$|^Shoulder$|^ShouldHaveSaid$|^Shouting$|^ShowBiz$|^Shy$|^SierraLeone$|^Signs$|^Silence$|^Silver$|^Simple$|^Sin$|^Sing$|^Singapore$|^SingleParent$|^SinglePerson$|^Sister$|^Sit$|^Situation$|^Size$|^Skate$|^Skateboard$|^Ski$|^Skin$|^Sky$|^Slavery$|^Sleazy$|^Sleep$|^Slovakia$|^Slovenia$|^Slow$|^Small$|^SmallTownLife$|^Smart$|^Smile$|^Smoke$|^Snake$|^Snow$|^Snowman$|^Sober$|^SocialOutcast$|^Solitude$|^SolomonIslands$|^Somalia$|^Son$|^Sounds$|^South$|^SouthAfrica$|^SouthAmerica$|^SouthDakota$|^SouthEastAsia$|^SouthKorea$|^SouthSudan$|^SpacedOut$|^Spain$|^Special$|^SpecificAge$|^SpecificTime$|^Speed$|^Spider$|^Spirit$|^Sport$|^Spring$|^SriLanka$|^St.Louis$|^Stalker$|^Stand$|^Star$|^Start$|^Steal$|^StepParent$|^Stick$|^Stop$|^Storm$|^StorybookCharacter$|^Strange$|^Street$|^Stress$|^String$|^StringAndRope$|^Strong$|^Stubborn$|^Stupid$|^Style$|^Substances$|^Success$|^Sudan$|^Suffrage$|^Suicide$|^Summer$|^Sun$|^Sunday$|^Sunrise$|^Sunshine$|^Superhero$|^Superiority$|^Supernatural$|^Support$|^Surf$|^Suriname$|^SurpriseParty$|^Survive$|^Swagger$|^Swamp$|^Sweden$|^Sweet$|^Swim$|^Switzerland$|^Sympathy$|^Synthesizer$|^Syria$|^Taiwan$|^Tajikistan$|^TakeMeBack$|^Talking$|^Tall$|^Tanzania$|^Tarot$|^Tattoo$|^Taxi$|^Teach$|^Technology$|^Teeth$|^Television$|^Tell$|^Temperature$|^Temptation$|^Tennessee$|^Terrible$|^Texas$|^TextMessage$|^Thailand$|^Theatre$|^Them$|^Thin$|^Thousand$|^Threaten$|^ThreeKings$|^Thursday$|^Tibet$|^Tiger$|^Time$|^TimeOfDay$|^Timor-Leste$|^Tobacco$|^Today$|^Togetherness$|^Togo$|^Tokyo$|^Tomorrow$|^Tonga$|^Tongue$|^Tonight$|^Tools$|^Torn$|^Toy$|^Tradition$|^Tragedy$|^Trail$|^Trains$|^Transportation$|^Trapped$|^Travel$|^Tree$|^TrinidadAndTobago$|^Trouble$|^Trucks$|^Trumpet$|^Trust$|^Truth$|^Try$|^Tuesday$|^Tulsa$|^Tunisia$|^Turkey$|^Turkmenistan$|^Turn$|^Tuvalu$|^UFO$|^Uganda$|^Ugly$|^Ukraine$|^Unbelievable$|^Uncle$|^Understanding$|^Unfair$|^Unfaithful$|^Union$|^Unique$|^UnitedArabEmirates$|^UnitedKingdom$|^UnitedStates$|^Unity$|^Universe$|^Unrequited$|^Up$|^Uruguay$|^Us$|^UsAgainstTheWorld$|^UserDefined$|^Utah$|^Utopia$|^Uzbekistan$|^Vacation$|^Valentine$|^Valley$|^Vanuatu$|^Vatican$|^Vegetable$|^Venezuela$|^Vermont$|^Victory$|^VideoGame$|^Vienna$|^Vietnam$|^Villain$|^Violence$|^Virginia$|^Voice$|^Volcano$|^Voodoo$|^Wait$|^Waiter$|^Waking$|^Wales$|^Walk$|^Wall$|^War$|^Washington$|^Wasteful$|^Water$|^Waterfall$|^Weapon$|^Wedding$|^Wednesday$|^Week$|^Weekend$|^Welcome$|^WeShouldBeTogether$|^West$|^WesternSahara$|^WestIndies$|^WestVirginia$|^Whale$|^Whisper$|^Whistle$|^White$|^Wife$|^Wild$|^Wind$|^WindChimes$|^Window$|^Winning$|^Winter$|^Wisconsin$|^Wisdom$|^Wish$|^Witch$|^Wizardry$|^Wolf$|^Women$|^WomensNames$|^WomenTalkingToMen$|^WomenTalkingToWomen$|^Wonderful$|^Woodwind$|^Words$|^Work$|^WorkingClass$|^Workout$|^World$|^Worry$|^Worship$|^Write$|^Wrong$|^Wyoming$|^Yellow$|^Yemen$|^Yesterday$|^Yoga$|^You$|^Young$|^Youth$|^Yugoslavia$|^Zambia$|^Zimbabwe$|^Zodiac$|^Zoo$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsThemeTypeValidator;
impl AvsThemeTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation =
            r"^ClosingTheme$|^MainTheme$|^OpeningTheme$|^SegmentTheme$|^TitleTheme$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTimecodeTypeValidator;
impl AvsTimecodeTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^FSK$|^MIDI$|^SMPTE$|^VITC$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTisTerritoryCodeValidator;
impl AvsTisTerritoryCodeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^4$|^8$|^12$|^20$|^24$|^28$|^31$|^32$|^36$|^40$|^44$|^48$|^50$|^51$|^52$|^56$|^64$|^68$|^70$|^72$|^76$|^84$|^90$|^96$|^100$|^104$|^108$|^112$|^116$|^120$|^124$|^132$|^140$|^144$|^148$|^152$|^156$|^158$|^170$|^174$|^178$|^180$|^188$|^191$|^192$|^196$|^200$|^203$|^204$|^208$|^212$|^214$|^218$|^222$|^226$|^230$|^231$|^232$|^233$|^242$|^246$|^250$|^258$|^262$|^266$|^268$|^270$|^276$|^278$|^280$|^288$|^296$|^300$|^308$|^320$|^324$|^328$|^332$|^336$|^340$|^344$|^348$|^352$|^356$|^360$|^364$|^368$|^372$|^376$|^380$|^384$|^388$|^392$|^398$|^400$|^404$|^408$|^410$|^414$|^417$|^418$|^422$|^426$|^428$|^430$|^434$|^438$|^440$|^442$|^446$|^450$|^454$|^458$|^462$|^466$|^470$|^478$|^480$|^484$|^492$|^496$|^498$|^499$|^504$|^508$|^512$|^516$|^520$|^524$|^528$|^540$|^548$|^554$|^558$|^562$|^566$|^578$|^583$|^584$|^585$|^586$|^591$|^598$|^600$|^604$|^608$|^616$|^620$|^624$|^626$|^630$|^634$|^642$|^643$|^646$|^659$|^662$|^670$|^674$|^678$|^682$|^686$|^688$|^690$|^694$|^702$|^703$|^704$|^705$|^706$|^710$|^716$|^720$|^724$|^728$|^729$|^732$|^736$|^740$|^748$|^752$|^756$|^760$|^762$|^764$|^768$|^776$|^780$|^784$|^788$|^792$|^795$|^798$|^800$|^804$|^807$|^810$|^818$|^826$|^834$|^840$|^854$|^858$|^860$|^862$|^882$|^886$|^887$|^890$|^891$|^894$|^2100$|^2101$|^2102$|^2103$|^2104$|^2105$|^2106$|^2107$|^2108$|^2109$|^2110$|^2111$|^2112$|^2113$|^2114$|^2115$|^2116$|^2117$|^2118$|^2119$|^2120$|^2121$|^2122$|^2123$|^2124$|^2125$|^2126$|^2127$|^2128$|^2129$|^2130$|^2131$|^2132$|^2133$|^2134$|^2136$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTitleTypeValidator;
impl AvsTitleTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AbbreviatedDisplayTitle$|^AlternativeTitle$|^DisplayTitle$|^FirstLineOfText$|^FormalTitle$|^GroupingTitle$|^IncorrectTitle$|^MisspelledTitle$|^MusicalWorkTitle$|^OriginalTitle$|^SearchTitle$|^SortingTitle$|^TitleAsPart$|^TitleWithoutPunctuation$|^TranslatedTitle$|^TransliteratedTitle$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTransferCategoryValidator;
impl AvsTransferCategoryValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AlignmentTones$|^AnalogToDigitalConverter$|^BitDepth$|^Cartridge$|^ClockSource$|^ConversionReferenceLevel$|^Emphasis$|^MaterialCondition$|^MicPreamp$|^NoiseReduction$|^NumberOfChannels$|^NumberOfSides$|^NumberOfTracks$|^PhonoPreamp$|^ReferenceClock$|^ReferenceTones$|^SamplingRate$|^Side1Condition$|^Side2Condition$|^SmpteFrameRate$|^SourceMachine$|^Speed$|^Storage$|^Stylus$|^StylusSize$|^TapeBakedDate$|^TapeBakedEquipment$|^TapeBakedHours$|^TapeBakedTemperature$|^TapeCoolHours$|^Tonearm$|^TrackConfiguration$|^TransferSoftware$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsTransferTypeValidator;
impl AvsTransferTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AnalogToAnalog$|^AnalogToDigital$|^DigitalCopy$|^DigitalToAnalog$|^DigitalToDigital$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUnitOfBitRateValidator;
impl AvsUnitOfBitRateValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^bps$|^Gbps$|^kbps$|^Mbps$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUnitOfConditionValueValidator;
impl AvsUnitOfConditionValueValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Millisecond$|^Minute$|^Percent$|^Pixel$|^Second$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUnitOfCuePointsValidator;
impl AvsUnitOfCuePointsValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Millisecond$|^Second$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUnitOfDurationValidator;
impl AvsUnitOfDurationValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Day$|^Month$|^UserDefined$|^Week$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUnitOfExtentValidator;
impl AvsUnitOfExtentValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^cm$|^Inch$|^mm$|^PercentOfScreen$|^Pixel$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUnitOfFrameRateValidator;
impl AvsUnitOfFrameRateValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Hz(interlaced)$|^Hz(non-interlaced)$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUnitOfFrequencyValidator;
impl AvsUnitOfFrequencyValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^GHz$|^Hz$|^kHz$|^MHz$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUnitTypeForRevenueAllocationValidator;
impl AvsUnitTypeForRevenueAllocationValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^IndividualUsages$|^Seconds$|^UnitOfAccounting$|^Usages$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUseTypeValidator;
impl AvsUseTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AsPerContract$|^Broadcast$|^Cable$|^ConditionalDownload$|^ContentInfluencedStream$|^Display$|^Download$|^Dub$|^DubForOnDemandStreaming$|^DubForLivePerformance$|^DubForMovies$|^DubForMusicOnHold$|^DubForPublicPerformance$|^DubForRadio$|^DubForTV$|^ExtractForInternet$|^KioskDownload$|^Narrowcast$|^NonInteractiveStream$|^OnDemandStream$|^Perform$|^PerformAsMusicOnHold$|^PerformInLivePerformance$|^PerformInPublic$|^PermanentDownload$|^Playback$|^PlayInPublic$|^Podcast$|^Print$|^PrivateCopy$|^PurchaseAsPhysicalProduct$|^Rent$|^Simulcast$|^Stream$|^TetheredDownload$|^TimeInfluencedStream$|^Unknown$|^Use$|^UseAsAlertTone$|^UseAsDevice$|^UseAsKaraoke$|^UseAsRingbackTone$|^UseAsRingbackTune$|^UseAsRingtone$|^UseAsRingtune$|^UseAsScreensaver$|^UseAsVoiceMail$|^UseAsWallpaper$|^UseForGenerativeAI$|^UseForIdentification$|^UseInMobilePhoneMessaging$|^UseInPhoneListening$|^UserDefined$|^UserMakeAvailableLabelProvided$|^UserMakeAvailableUserProvided$|^Webcast$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUseTypeDSRValidator;
impl AvsUseTypeDSRValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AsPerContract$|^Broadcast$|^ConditionalDownload$|^ContentInfluencedStream$|^Display$|^Download$|^DubForAdvertisement$|^DubForLivePerformance$|^DubForMovies$|^DubForMusicOnHold$|^DubForPublicPerformance$|^DubForRadio$|^DubForTV$|^ExtractForInternet$|^KioskDownload$|^LiveStream$|^Narrowcast$|^NonInteractiveStream$|^OnDemandStream$|^PerformAsMusicOnHold$|^PerformInLivePerformance$|^PerformInPublic$|^PermanentDownload$|^Playback$|^PlayInPublic$|^Podcast$|^Print$|^PrivateCopy$|^ProgrammedContentStream$|^PurchaseAsPhysicalProduct$|^Rent$|^Simulcast$|^Stream$|^TetheredDownload$|^TimeInfluencedStream$|^Unknown$|^UseAsAlertTone$|^UseAsDevice$|^UseAsKaraoke$|^UseAsRingbackTone$|^UseAsRingbackTune$|^UseAsRingtone$|^UseAsRingtune$|^UseAsScreensaver$|^UseAsVoiceMail$|^UseAsWallpaper$|^UseForIdentification$|^UseInMobilePhoneMessaging$|^UseInPhoneListening$|^UserDefined$|^UserMakeAvailableLabelProvided$|^UserMakeAvailableUserProvided$|^Webcast$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUseTypeERNValidator;
impl AvsUseTypeERNValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Broadcast$|^Cable$|^ConditionalDownload$|^ContentInfluencedStream$|^Display$|^Download$|^Dub$|^DubForOnDemandStreaming$|^DubForLivePerformance$|^DubForMovies$|^DubForMusicOnHold$|^DubForPublicPerformance$|^DubForRadio$|^DubForTV$|^ExtractForInternet$|^KioskDownload$|^Narrowcast$|^NonInteractiveStream$|^OnDemandStream$|^Perform$|^PerformAsMusicOnHold$|^PerformInLivePerformance$|^PerformInPublic$|^PermanentDownload$|^Playback$|^PlayInPublic$|^Podcast$|^Print$|^PrivateCopy$|^PurchaseAsPhysicalProduct$|^Rent$|^Simulcast$|^Stream$|^TetheredDownload$|^TimeInfluencedStream$|^Use$|^UseAsAlertTone$|^UseAsDevice$|^UseAsKaraoke$|^UseAsRingbackTone$|^UseAsRingbackTune$|^UseAsRingtone$|^UseAsRingtune$|^UseAsScreensaver$|^UseAsVoiceMail$|^UseAsWallpaper$|^UseForGenerativeAI$|^UseForIdentification$|^UseInMobilePhoneMessaging$|^UseInPhoneListening$|^UserDefined$|^UserMakeAvailableLabelProvided$|^UserMakeAvailableUserProvided$|^Webcast$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUseTypeMWNLValidator;
impl AvsUseTypeMWNLValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation =
            r"^OnDemandStream$|^PermanentDownload$|^PurchaseAsPhysicalProduct$|^UseAsRingtone$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUseTypeRDRValidator;
impl AvsUseTypeRDRValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^All$|^Broadcast$|^BroadcastRadio$|^BroadcastTV$|^CableRetransmission$|^CableRetransmissionRadio$|^CableRetransmissionTV$|^CatchUp$|^CatchUpRadio$|^CatchUpTV$|^CommercialRent$|^ConditionalDownload$|^Download$|^Dub$|^DubForDistribution$|^DubForOnDemandStreaming$|^DubForPublicPerformance$|^DubForRadio$|^DubForTV$|^Lend$|^NonInteractiveStream$|^OnDemandStream$|^PerformInPublic$|^PermanentDownload$|^Podcast$|^PrivateCopy$|^Retransmission$|^RingbackTone$|^Simulcast$|^SimulcastRadio$|^SimulcastTV$|^Stream$|^UseForEducationAndOrSocialPurposes$|^UseForGenerativeAI$|^UseForIdentification$|^UserDefined$|^Webcast$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUserInterfaceTypeValidator;
impl AvsUserInterfaceTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AsPerContract$|^ConnectedDevice$|^GameConsole$|^Jukebox$|^KaraokeMachine$|^Kiosk$|^LocalStorageJukebox$|^PersonalComputer$|^PhysicalMediaWriter$|^PortableDevice$|^RemoteStorageJukebox$|^SmartSpeakers$|^Unknown$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsUserInterfaceTypeERNValidator;
impl AvsUserInterfaceTypeERNValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ConnectedDevice$|^GameConsole$|^Jukebox$|^KaraokeMachine$|^Kiosk$|^LocalStorageJukebox$|^PersonalComputer$|^PhysicalMediaWriter$|^PortableDevice$|^RemoteStorageJukebox$|^SmartSpeakers$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsVersionTypeValidator;
impl AvsVersionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ACappellaVersion$|^AlbumVersion$|^AlternativeVersion$|^CleanVersion$|^DemoVersion$|^EditedVersion$|^InstrumentalVersion$|^KaraokeVersion$|^LiveVersion$|^MixVersion$|^MonoVersion$|^RadioVersion$|^RemixVersion$|^SessionVersion$|^SingleVersion$|^StereoVersion$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsVideoCodecTypeValidator;
impl AvsVideoCodecTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AVC$|^H.261$|^H.263$|^HEVC$|^MPEG-1$|^MPEG-2$|^MPEG-4$|^ProRes-422$|^ProRes-422HQ$|^ProRes-422LT$|^ProRes-422Proxy$|^ProRes-4444$|^ProRes-4444XQ$|^RealVideo$|^Shockwave$|^Unknown$|^UserDefined$|^WMV$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsVideoDefinitionTypeValidator;
impl AvsVideoDefinitionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^HighDefinition$|^StandardDefinition$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsVideoDefinitionTypeDSRValidator;
impl AvsVideoDefinitionTypeDSRValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^HighDefinition$|^StandardDefinition$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsVideoTypeValidator;
impl AvsVideoTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AdultContent$|^AdvertisementVideo$|^AdviceMagazine$|^Animation$|^BalletVideo$|^BehindTheScenes$|^BlackAndWhiteVideo$|^ChildrensFilm$|^ColorizedVideo$|^ColumnVideo$|^ConcertClip$|^ConcertVideo$|^CorporateFilm$|^Credits$|^Documentary$|^EducationalVideo$|^Episode$|^FeatureFilm$|^Fiction$|^InfomercialVideo$|^Interview$|^Karaoke$|^LiveEventVideo$|^LongFormMusicalWorkVideo$|^LongFormNonMusicalWorkVideo$|^LyricVideo$|^Magazine$|^Menu$|^MultimediaVideo$|^MusicalWorkClip$|^MusicalWorkReadalongVideo$|^MusicalWorkTrailer$|^MusicalWorkVideoChapter$|^News$|^NonMusicalWorkClip$|^NonMusicalWorkReadalongVideo$|^NonMusicalWorkTrailer$|^NonMusicalWorkVideoChapter$|^NonSerialAudioVisualRecording$|^OperaVideo$|^Performance$|^ReadalongVideo$|^RealityTvShowVideo$|^Season$|^SerialAudioVisualRecording$|^Series$|^ShortFilm$|^SilentVideo$|^SketchVideo$|^SoapSitcom$|^SpecialEvent$|^Sport$|^TheatricalWorkVideo$|^TrailerVideo$|^TvFilm$|^TvProgram$|^TvShowVideo$|^Unknown$|^VideoChapter$|^VideoClip$|^VideoReport$|^VideoStem$|^Drama$|^DramaticoMusicalVideo$|^InteractiveResource$|^ShortFormMusicalWorkVideo$|^ShortFormNonMusicalWorkVideo$|^UserDefined$|^WebResource$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsVideoTypeDSRFValidator;
impl AvsVideoTypeDSRFValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^AdultContent$|^AdvertisementVideo$|^AdviceMagazine$|^Animation$|^BalletVideo$|^BehindTheScenes$|^BlackAndWhiteVideo$|^ChildrensFilm$|^ColorizedVideo$|^ColumnVideo$|^ConcertClip$|^ConcertVideo$|^CorporateFilm$|^Credits$|^Documentary$|^EducationalVideo$|^Episode$|^FeatureFilm$|^Fiction$|^InfomercialVideo$|^Interview$|^Karaoke$|^LiveEventVideo$|^LongFormMusicalWorkVideo$|^LongFormNonMusicalWorkVideo$|^LyricVideo$|^Magazine$|^Menu$|^MultimediaVideo$|^MusicalWorkClip$|^MusicalWorkReadalongVideo$|^MusicalWorkTrailer$|^MusicalWorkVideoChapter$|^News$|^NonMusicalWorkClip$|^NonMusicalWorkReadalongVideo$|^NonMusicalWorkTrailer$|^NonMusicalWorkVideoChapter$|^NonSerialAudioVisualRecording$|^OperaVideo$|^Performance$|^ReadalongVideo$|^RealityTvShowVideo$|^Season$|^SerialAudioVisualRecording$|^Series$|^ShortFilm$|^SilentVideo$|^SketchVideo$|^SoapSitcom$|^SpecialEvent$|^Sport$|^TheatricalWorkVideo$|^TrailerVideo$|^TvFilm$|^TvProgram$|^TvShowVideo$|^Unknown$|^VideoChapter$|^VideoClip$|^VideoReport$|^VideoStem$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsVideoTypeERN43Validator;
impl AvsVideoTypeERN43Validator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Clip$|^FrontCoverVideo$|^LongFormMusicalWorkVideo$|^LongFormNonMusicalWorkVideo$|^ShortFormMusicalWorkVideo$|^ShortFormNonMusicalWorkVideo$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsVisualPerceptionTypeValidator;
impl AvsVisualPerceptionTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Background$|^UserDefined$|^Visual$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsVocalRegisterValidator;
impl AvsVocalRegisterValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Alto$|^Baritone$|^BassBaritone$|^Bass$|^Castrati$|^Contrabass$|^Contralto$|^Countertenor$|^Falsetto$|^MezzoSoprano$|^Paradon$|^Piccolo$|^Sopranino$|^Soprano$|^Tenor$|^Treble$|^ViolaParadon$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsVocalTypeValidator;
impl AvsVocalTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^Instrumental$|^UserDefined$|^Vocal$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsWorkRelationshipTypeValidator;
impl AvsWorkRelationshipTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^ArrangementOf$|^BasisForArrangement$|^MelodyBorrowedFrom$|^ReimaginingOf$|^TakenFrom$|^UserDefined$";
        Regex::new(validation).unwrap().is_match(value)
    }
}

#[allow(dead_code)]
pub struct AvsWorkTypeValidator;
impl AvsWorkTypeValidator {
    #[allow(dead_code)]
    pub fn validate(value: &String) -> bool {
        let validation = r"^GraphicalWork$|^LiteraryWork$";
        Regex::new(validation).unwrap().is_match(value)
    }
}
