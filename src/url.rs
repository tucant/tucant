use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    iter::Peekable,
};

use derive_more::TryInto;
use serde::{Deserialize, Serialize};
use url::{Host, Origin, Url};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TucanUrl {
    pub session_nr: Option<u64>,
    pub program: TucanProgram,
}

impl TucanUrl {
    fn to_tucan_url(&self) -> String {
        //         let url = url.unwrap_or(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{},-N000311,-A", self.session.nr));

        [TucanArgument::Number(311), TucanArgument::String("")].iter()
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct StartpageDispatch;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Externalpages {
    id: u64,
    name: String,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Moduledetails {
    pub id: u64,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Registration {
    pub path: Option<[u64; 4]>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Mlsstart;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Mymodules;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Profcourses;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Studentchoicecourses;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Myexams;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Courseresults;


#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Examresults;


#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct StudentResult;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, TryInto)]
pub enum TucanProgram {
    Mlsstart(Mlsstart),
    Mymodules(Mymodules),
    Profcourses(Profcourses),
    Studentchoicecourses(Studentchoicecourses),
    Registration(Registration),
    Myexams(Myexams),
    Courseresults(Courseresults),
    Examresults(Examresults),
    StudentResult(StudentResult),
    Moduledetails(Moduledetails),
    StartpageDispatch(StartpageDispatch),
    Externalpages(Externalpages),
}

#[derive(Debug)]
pub enum TucanArgument<'a> {
    Number(u64),
    String(&'a str),
}

impl<'a> TucanArgument<'a> {
    pub fn number(&self) -> u64 {
        match self {
            TucanArgument::Number(number) => *number,
            _ => panic!(),
        }
    }

    pub fn string(&self) -> &'a str {
        match self {
            TucanArgument::String(string) => string,
            _ => panic!(),
        }
    }
}

pub fn parse_arguments(
    arguments: &str,
) -> Peekable<impl Iterator<Item = TucanArgument> + std::fmt::Debug> {
    arguments
        .split_terminator(',')
        .map(|a| match a.get(0..2) {
            Some("-N") => TucanArgument::Number(a[2..].parse::<u64>().unwrap()),
            Some("-A") => TucanArgument::String(&a[2..]),
            _ => panic!(),
        })
        .peekable()
}

fn number<'a>(arguments: &mut (impl Iterator<Item = TucanArgument<'a>> + std::fmt::Debug)) -> u64 {
    arguments.next().unwrap().number()
}

fn string<'a>(
    arguments: &mut (impl Iterator<Item = TucanArgument<'a>> + std::fmt::Debug),
) -> &'a str {
    arguments.next().unwrap().string()
}

pub fn parse_tucan_url(url: &str) -> TucanUrl {
    let url = Url::parse(url).unwrap();
    assert_eq!(
        url.origin(),
        Origin::Tuple(
            "https".into(),
            Host::Domain("www.tucan.tu-darmstadt.de".into()),
            443,
        )
    );
    assert_eq!(url.path(), "/scripts/mgrqispi.dll");
    let query_pairs = url.query_pairs();
    let query_pairs = query_pairs.collect::<HashMap<_, _>>();
    let app_name = query_pairs.get("APPNAME").unwrap().as_ref();
    let arguments = query_pairs.get("ARGUMENTS").unwrap().as_ref();
    let prgname = query_pairs.get("PRGNAME").unwrap().as_ref();
    let mut arguments = parse_arguments(arguments);
    assert_eq!(app_name, "CampusNet");

    let session_nr = if prgname == "ACTION" {
        1
    } else {
        number(&mut arguments)
    };
    let session_nr = if session_nr == 1 {
        Err(Error::new(ErrorKind::Other, "not logged in".to_string()))
    } else {
        Ok(session_nr)
    };

    let result = match prgname {
        "STARTPAGE_DISPATCH" => {
            assert_eq!(number(&mut arguments), 19);
            assert_eq!(number(&mut arguments), 0);
            TucanUrl {
                session_nr: session_nr.ok(),
                program: TucanProgram::StartpageDispatch(StartpageDispatch),
            }
        }
        "EXTERNALPAGES" => TucanUrl {
            session_nr: session_nr.ok(),
            program: TucanProgram::Externalpages(Externalpages {
                id: number(&mut arguments),
                name: string(&mut arguments).to_string(),
            }),
        },
        "MLSSTART" => {
            assert_eq!(number(&mut arguments), 19);
            TucanUrl {
                session_nr,
                program: TucanProgram::Mlsstart(Mlsstart),
            }
        }
        "MYMODULES" => {
            assert_eq!(number(&mut arguments), 275);
            TucanUrl {
                session_nr,
                program: TucanProgram::Mymodules(Mymodules),
            }
        }
        "PROFCOURSES" => {
            assert_eq!(number(&mut arguments), 274);
            TucanUrl {
                session_nr,
                program: TucanProgram::Profcourses(Profcourses),
            }
        }
        "STUDENTCHOICECOURSES" => {
            assert_eq!(number(&mut arguments), 307);
            TucanUrl {
                session_nr,
                program: TucanProgram::Studentchoicecourses(Studentchoicecourses),
            }
        }
        "REGISTRATION" => {
            assert_eq!(number(&mut arguments), 311);
            match arguments.peek().unwrap() {
                TucanArgument::Number(_) => TucanUrl {
                    session_nr,
                    program: TucanProgram::Registration(Registration {
                        path: Some([
                            number(&mut arguments),
                            number(&mut arguments),
                            number(&mut arguments),
                            number(&mut arguments),
                        ]),
                    }),
                },
                TucanArgument::String(_) => {
                    assert_eq!(string(&mut arguments), "");
                    TucanUrl {
                        session_nr,
                        program: TucanProgram::Registration(Registration { path: None }),
                    }
                }
            }
        }
        "MYEXAMS" => {
            assert_eq!(number(&mut arguments), 318);
            TucanUrl {
                session_nr,
                program: TucanProgram::Myexams(Myexams),
            }
        }
        "COURSERESULTS" => {
            assert_eq!(number(&mut arguments), 324);
            TucanUrl {
                session_nr,
                program: TucanProgram::Courseresults(Courseresults),
            }
        }
        "EXAMRESULTS" => {
            assert_eq!(number(&mut arguments), 325);
            TucanUrl {
                session_nr,
                program: TucanProgram::Examresults(Examresults),
            }
        }
        "STUDENT_RESULT" => {
            assert_eq!(number(&mut arguments), 316);
            assert_eq!(number(&mut arguments), 0);
            assert_eq!(number(&mut arguments), 0);
            assert_eq!(number(&mut arguments), 0);
            assert_eq!(number(&mut arguments), 0);
            assert_eq!(number(&mut arguments), 0);
            assert_eq!(number(&mut arguments), 0);
            TucanUrl {
                session_nr,
                program: TucanProgram::StudentResult(StudentResult),
            }
        }
        "MODULEDETAILS" => {
            assert_eq!(number(&mut arguments), 311);
            let result = TucanUrl {
                session_nr,
                program: TucanProgram::Moduledetails(Moduledetails {
                    id: number(&mut arguments),
                }),
            };
            string(&mut arguments);
            result
        }
        other => {
            panic!("invalid appname: {}", other);
        }
    };

    if arguments.peek().is_some() {
        panic!("too many arguments {:?}", arguments.collect::<Vec<_>>())
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_urls() -> anyhow::Result<()> {
        /*let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001")?;
        assert_eq!(
            TucanUrl::MaybeAuthenticated {
                url: MaybeAuthenticatedTucanUrl::StartpageDispatch,
                session_nr: None
            },
            url
        );*/

        // unauthenticated start page
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome");

        // authenticated start page
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N707546050471776,-N000019,");

        // Veranstaltungen
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N428926119975172,-N000273,-Astudveranst%2Ehtml");

        // Veranstaltungen -> Meine Module
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYMODULES&ARGUMENTS=-N428926119975172,-N000275,");

        // Veranstaltungen -> Meine Veranstaltungen
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PROFCOURSES&ARGUMENTS=-N428926119975172,-N000274,");

        // Veranstaltungen -> Meine Wahlbereiche
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENTCHOICECOURSES&ARGUMENTS=-N428926119975172,-N000307,");

        // Veranstaltungen -> Anmeldung
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N428926119975172,-N000311,-A");

        // Veranstaltungen -> Anmeldung -> Pflichtbereich
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N988222970824392,-N000311,-N376333755785484,-N0,-N356173456785530,-N000000000000000");

        // Prüfungen
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N428926119975172,-N000280,-Astudpruefungen%2Ehtml");

        // Prüfungen -> Meine Prüfungen
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYEXAMS&ARGUMENTS=-N428926119975172,-N000318,");

        // Prüfungen -> Semesterergebnisse
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N428926119975172,-N000323,-Astudergebnis%2Ehtml");

        // Prüfungen -> Semesterergebnisse -> Modulergebnisse
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSERESULTS&ARGUMENTS=-N428926119975172,-N000324,");

        // Prüfungen -> Semesterergebnisse -> Prüfungsergebnisse
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXAMRESULTS&ARGUMENTS=-N428926119975172,-N000325,");

        // Prüfungen -> Leistungsspiegel
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N428926119975172,-N000316,-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000");

        // Moduldetails

        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N988222970824392,-N000311,-N376373060881867,-A3f5EHWl9PqwMeD2AvWmMWDl-QUpCmjaN7ZKJmNFt7-UpvMAx4omKmd6gmUR9mfft3oRQP-PaxNZtPqGdRUpsOZmeQNHv7URzmQVdOBBF3SftxMo8PU5S7dwZfbZYmdPfQd5ycYntWopZmoUBYDotPMPNmkZdPILZ7gmT4SPXHjV-cBUxxNPWR-m9QkZLvUovfgPXvqR5YBG-eZo8WqmAHjHfeMpqRkZ97DKZQIo5PfP9HSRBeqAHvDZjrUUeHWV6xZR7YIL3OuULPQHHVNK8f-5wvZ5kYUUvYWlNQoljQIU5eUBjHDPmmZLb4YGhPIUTmuWXYfnAvfWAYWW54D6hQ-58HWPpmQBNWqeFYM5HvDUgcupLmMfAxM5D4MoAcuopQuPjfYHvfqLqeqwZeMWXVDZjPMHVcocZcNmt7ZDjQZedWfmyfDWUWSAeHDajVdmUOjBtmNWpvqP9OqG3VNHlPQPKvocZWqZYVWoxfSLlcDPQQWKZegeNQY5afu5COzH-fDoKWU79CQoErUPHYDHVQtin");

        // urls we still need to reverse

        // https://cryptii.com/pipes/text-to-base64
        // Nachrichten
        // likely base64 with ~- and _ as padding

        // per session urls but inside a session potentially consistent
        //let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AkaTs6g9bP0R3xzedtop4miuoi56H3Qg425njHHR6KzEmalVi4oDtkK6~xkg9cRLwvmeiajCHw3PN266Zf3GOdaSKKSxNL-p6ZaQI~5oVIcIdkWSynh328JX-tBVvFclUzM2edYwexy0VrWXCiHjX7-Mr3oU_")?;

        // Vorlesungsverzeichnis
        //let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AhFucMz0tWte~bVYvS7ZXP5dvkkE-wxWuME0Wqlj3rB-iwNGmsluzwhK5irFXFH0SgStfWj6FpAVtU2MQ32Ym4VKspT-EJN252qy~QgsOsLLZU7b~VRfzznhHKnzAJuhARdmMM1nx~31tKkgN6ETdcmIeCTfaeM874hp8aM3ass8q8PkZovZFJHWUlQ__")?;

        Ok(())
    }
}
