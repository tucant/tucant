
pub enum TucanUrl {
    StartpageDispatch,
    Externalpages,
    Mlsstart,
    Mymodules,
    Profcourse,
    Studentchoicecourses,
    Registration,
    Courseresults,
    Examresults,
    StudentResult
}

pub fn parse_tucan_url(url: &str) {

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_urls() {
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/");
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001");
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome");
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll");
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N707546050471776,-N000019,-N000000000000000");
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N707546050471776,-N000019,");
       
        // Veranstaltungen
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N428926119975172,-N000273,-Astudveranst%2Ehtml");

        // Veranstaltungen -> Meine Module
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYMODULES&ARGUMENTS=-N428926119975172,-N000275,");

        // Veranstaltungen -> Meine Veranstaltungen
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PROFCOURSES&ARGUMENTS=-N428926119975172,-N000274,");

        // Veranstaltungen -> Meine Wahlbereiche
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENTCHOICECOURSES&ARGUMENTS=-N428926119975172,-N000307,");

        // Veranstaltungen -> Anmeldung
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N428926119975172,-N000311,-A");

        // Prüfungen
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N428926119975172,-N000280,-Astudpruefungen%2Ehtml");

        // Prüfungen -> Meine Prüfungen
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYEXAMS&ARGUMENTS=-N428926119975172,-N000318,");

        // Prüfungen -> Semesterergebnisse
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N428926119975172,-N000323,-Astudergebnis%2Ehtml");

        // Prüfungen -> Semesterergebnisse -> Modulergebnisse
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSERESULTS&ARGUMENTS=-N428926119975172,-N000324,");

        // Prüfungen -> Semesterergebnisse -> Prüfungsergebnisse
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXAMRESULTS&ARGUMENTS=-N428926119975172,-N000325,");

        // Prüfungen -> Leistungsspiegel
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N428926119975172,-N000316,-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000");



        // urls we still need to reverse

        // Nachrichten
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AkaTs6g9bP0R3xzedtop4miuoi56H3Qg425njHHR6KzEmalVi4oDtkK6~xkg9cRLwvmeiajCHw3PN266Zf3GOdaSKKSxNL-p6ZaQI~5oVIcIdkWSynh328JX-tBVvFclUzM2edYwexy0VrWXCiHjX7-Mr3oU_");

        // Vorlesungsverzeichnis
        parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AhFucMz0tWte~bVYvS7ZXP5dvkkE-wxWuME0Wqlj3rB-iwNGmsluzwhK5irFXFH0SgStfWj6FpAVtU2MQ32Ym4VKspT-EJN252qy~QgsOsLLZU7b~VRfzznhHKnzAJuhARdmMM1nx~31tKkgN6ETdcmIeCTfaeM874hp8aM3ass8q8PkZovZFJHWUlQ__");

    }
}