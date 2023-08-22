// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, iter::Peekable};

use derive_more::{From, TryInto};
use serde::{Deserialize, Serialize};
use url::{Host, Origin, Url};

#[allow(clippy::module_name_repetitions)]
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct TucanUrl {
    pub session_nr: Option<u64>,
    pub program: TucanProgram,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct StartpageDispatch;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Externalpages {
    pub id: u64,
    pub name: String,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Moduledetails {
    pub id: Vec<u8>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Coursedetails {
    pub id: Vec<u8>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Registration {
    pub path: Vec<u8>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct RootRegistration {}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Mlsstart;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Mymodules;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Semester {
    CurrentSemester,
    AllSemesters,
    Semester(u64),
}

impl From<Option<u64>> for Semester {
    fn from(value: Option<u64>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Semester::CurrentSemester,
        }
    }
}

impl From<u64> for Semester {
    fn from(value: u64) -> Self {
        match value {
            999 => Semester::AllSemesters,
            semester => Semester::Semester(semester),
        }
    }
}

impl From<Semester> for u64 {
    fn from(value: Semester) -> Self {
        // TODO FIXME probably don't provide this impl
        match value {
            Semester::CurrentSemester => unreachable!(),
            Semester::AllSemesters => 999,
            Semester::Semester(semester) => semester,
        }
    }
}

impl From<Semester> for Option<u64> {
    fn from(value: Semester) -> Self {
        match value {
            Semester::CurrentSemester => None,
            Semester::AllSemesters => Some(999),
            Semester::Semester(semester) => Some(semester),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Profcourses {
    pub semester: Semester,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Studentchoicecourses;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Myexams {
    pub semester: Semester,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Courseresults {
    pub semester: Option<u64>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Examresults {
    pub semester: Option<u64>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct StudentResult;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Persaddress;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Examdetails {
    pub id: Vec<u8>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Courseprep {
    pub id: i64,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub magic: String,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, TryInto, From)]
pub enum TucanProgram {
    Mlsstart(Mlsstart),
    Mymodules(Mymodules),
    Profcourses(Profcourses),
    Studentchoicecourses(Studentchoicecourses),
    Registration(Registration),
    RootRegistration(RootRegistration),
    Myexams(Myexams),
    Courseresults(Courseresults),
    Examresults(Examresults),
    StudentResult(StudentResult),
    Moduledetails(Moduledetails),
    Coursedetails(Coursedetails),
    StartpageDispatch(StartpageDispatch),
    Externalpages(Externalpages),
    Persaddress(Persaddress),
    Examdetails(Examdetails),
    Courseprep(Courseprep),
    Action(Action),
}

impl TucanProgram {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn to_tucan_url(&self, session_nr: Option<u64>) -> String {
        let (progname, args): (&str, Box<dyn Iterator<Item = TucanArgument>>) = match self {
            Self::Mlsstart(_) => todo!(),
            Self::Mymodules(_) => (
                "MYMODULES",
                Box::new(
                    [
                        TucanArgument::Number(session_nr.unwrap_or(1)),
                        TucanArgument::Number(275),
                        TucanArgument::Number(999),
                    ]
                    .into_iter(),
                ),
            ),
            Self::Profcourses(Profcourses { semester }) => (
                "PROFCOURSES",
                Box::new(
                    [
                        TucanArgument::Number(session_nr.unwrap_or(1)),
                        TucanArgument::Number(274),
                    ]
                    .into_iter()
                    .chain(
                        match semester {
                            Semester::CurrentSemester => vec![],
                            Semester::AllSemesters => vec![TucanArgument::Number(999)],
                            Semester::Semester(semester) => vec![TucanArgument::Number(*semester)],
                        }
                        .into_iter(),
                    ),
                ),
            ),
            Self::Studentchoicecourses(_) => todo!(),
            Self::Registration(Registration { path }) => {
                let mut a = path.chunks(std::mem::size_of::<u64>());
                (
                    "REGISTRATION",
                    Box::new(
                        [
                            TucanArgument::Number(session_nr.unwrap_or(1)),
                            TucanArgument::Number(311),
                            TucanArgument::Number(u64::from_be_bytes(
                                a.next().unwrap().try_into().unwrap(),
                            )),
                            TucanArgument::Number(0),
                            TucanArgument::Number(u64::from_be_bytes(
                                a.next().unwrap().try_into().unwrap(),
                            )),
                            TucanArgument::Number(u64::from_be_bytes(
                                a.next().unwrap().try_into().unwrap(),
                            )),
                        ]
                        .into_iter(),
                    ),
                )
            }
            Self::RootRegistration(_) => (
                "REGISTRATION",
                Box::new(
                    [
                        TucanArgument::Number(session_nr.unwrap_or(1)),
                        TucanArgument::Number(311),
                        TucanArgument::String(""),
                    ]
                    .into_iter(),
                ),
            ),
            Self::Myexams(Myexams { semester }) => (
                "MYEXAMS",
                Box::new(
                    [
                        TucanArgument::Number(session_nr.unwrap_or(1)),
                        TucanArgument::Number(318),
                    ]
                    .into_iter()
                    .chain(
                        match semester {
                            Semester::CurrentSemester => vec![],
                            Semester::AllSemesters => vec![TucanArgument::Number(999)],
                            Semester::Semester(semester) => vec![TucanArgument::Number(*semester)],
                        }
                        .into_iter(),
                    ),
                ),
            ),
            Self::Courseresults(Courseresults { semester }) => (
                "COURSERESULTS",
                Box::new(
                    [
                        TucanArgument::Number(session_nr.unwrap_or(1)),
                        TucanArgument::Number(324),
                    ]
                    .into_iter()
                    .chain(semester.map(TucanArgument::Number)),
                ),
            ),
            Self::Examresults(_) => (
                "EXAMRESULTS",
                Box::new(
                    [
                        TucanArgument::Number(session_nr.unwrap_or(1)),
                        TucanArgument::Number(325),
                        TucanArgument::Number(999),
                    ]
                    .into_iter(),
                ),
            ),
            Self::StudentResult(_) => todo!(),
            Self::Moduledetails(Moduledetails { id }) => (
                "MODULEDETAILS",
                Box::new(
                    [
                        TucanArgument::Number(session_nr.unwrap_or(1)),
                        TucanArgument::Number(311),
                        TucanArgument::Number(u64::from_be_bytes(
                            id.as_slice().try_into().unwrap(),
                        )),
                    ]
                    .into_iter(),
                ),
            ),
            Self::Coursedetails(Coursedetails { id }) => {
                (
                    "COURSEDETAILS",
                    Box::new(
                        [
                            TucanArgument::Number(session_nr.unwrap_or(1)),
                            TucanArgument::Number(311),
                            TucanArgument::Number(0),
                        ]
                        .into_iter()
                        .chain(id.chunks(std::mem::size_of::<u64>()).map(|n| {
                            TucanArgument::Number(u64::from_be_bytes(n.try_into().unwrap()))
                        }))
                        .chain([TucanArgument::Number(0), TucanArgument::Number(0)].into_iter()),
                    ),
                )
            }
            Self::Persaddress(_) => (
                "PERSADDRESS",
                Box::new(
                    [
                        TucanArgument::Number(session_nr.unwrap_or(1)),
                        TucanArgument::Number(339),
                    ]
                    .into_iter(),
                ),
            ),
            Self::StartpageDispatch(_) => todo!(),
            Self::Externalpages(Externalpages { id, name }) => (
                "EXTERNALPAGES",
                Box::new(
                    [
                        TucanArgument::Number(session_nr.unwrap_or(1)),
                        TucanArgument::Number(*id),
                        TucanArgument::String(name),
                    ]
                    .into_iter(),
                ),
            ),
            Self::Examdetails(Examdetails { id }) => {
                let mut a = id.chunks(std::mem::size_of::<u64>());
                (
                    "EXAMDETAILS",
                    Box::new(
                        [
                            TucanArgument::Number(session_nr.unwrap_or(1)),
                            TucanArgument::Number(318),
                            TucanArgument::Number(u64::from_be_bytes(
                                a.next().unwrap().try_into().unwrap(),
                            )),
                        ]
                        .into_iter(),
                    ),
                )
            }
            Self::Courseprep(Courseprep { id }) => (
                "COURSEPREP",
                Box::new(
                    [
                        TucanArgument::Number(session_nr.unwrap_or(1)),
                        TucanArgument::Number(318),
                        TucanArgument::Number(0),
                        TucanArgument::Number((*id).try_into().unwrap()),
                    ]
                    .into_iter(),
                ),
            ),
            Self::Action(Action { magic }) => (
                "ACTION",
                Box::new(std::iter::once(TucanArgument::String(magic))),
            ),
        };
        let args = args.format(",");

        format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME={progname}&ARGUMENTS={args}")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TucanArgument<'a> {
    Number(u64),
    String(&'a str),
}

impl<'a> TucanArgument<'a> {
    #[must_use]
    pub fn number(&self) -> u64 {
        match self {
            TucanArgument::Number(number) => *number,
            TucanArgument::String(_) => panic!(),
        }
    }

    #[must_use]
    pub fn string(&self) -> &'a str {
        match self {
            TucanArgument::String(string) => string,
            TucanArgument::Number(_) => panic!(),
        }
    }
}

impl Display for TucanArgument<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TucanArgument::Number(v) => write!(f, "-N{v}"),
            TucanArgument::String(v) => write!(f, "-A{v}"),
        }
    }
}

pub fn parse_arguments(
    arguments: &str,
) -> Peekable<impl Iterator<Item = TucanArgument> + std::fmt::Debug> {
    arguments
        .split_terminator(',')
        .map(|a| match a.get(0..2) {
            Some("-N") => TucanArgument::Number(a[2..].parse().unwrap()),
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

#[allow(clippy::too_many_lines)]
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn parse_tucan_url(url: &str) -> TucanUrl {
    // TODO FIXME don't panic here as we use this for our login hack
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

    // TODO FIXME new URLSearchParams(new FormData(document.querySelector(`form.pageElementTop[method=POST][action="/scripts/mgrqispi.dll"]`))).toString()
    // "semester=000000015116000&APPNAME=CampusNet&PRGNAME=MYEXAMS&ARGUMENTS=sessionno%2Cmenuno%2Csemester&sessionno=848756870587019&menuno=000318"
    // based on the ARGUMENTS replaced by its value this should be able to reconstruct the exact same url and then the code after should work equally
    if query_pairs.is_empty() {
        return TucanUrl {
            session_nr: None,
            program: TucanProgram::Externalpages(Externalpages {
                id: 344,
                name: "welcome".to_string(),
            }),
        };
    }

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
        None
    } else {
        Some(session_nr)
    };

    let program = match prgname {
        "STARTPAGE_DISPATCH" => {
            number(&mut arguments);
            assert_eq!(number(&mut arguments), 0);

            TucanProgram::StartpageDispatch(StartpageDispatch)
        }
        "EXTERNALPAGES" => TucanProgram::Externalpages(Externalpages {
            id: number(&mut arguments),
            name: string(&mut arguments).to_string(),
        }),
        "MLSSTART" => {
            number(&mut arguments);
            TucanProgram::Mlsstart(Mlsstart)
        }
        "MYMODULES" => {
            number(&mut arguments);
            assert_eq!(number(&mut arguments), 999);
            TucanProgram::Mymodules(Mymodules)
        }
        "PROFCOURSES" => {
            number(&mut arguments);
            let semester = match arguments.next() {
                Some(TucanArgument::Number(999)) => Semester::AllSemesters,
                Some(TucanArgument::Number(semester)) => Semester::Semester(semester),
                None => Semester::CurrentSemester,
                _ => panic!(),
            };
            TucanProgram::Profcourses(Profcourses { semester })
        }
        "STUDENTCHOICECOURSES" => {
            number(&mut arguments);
            TucanProgram::Studentchoicecourses(Studentchoicecourses)
        }
        "REGISTRATION" => {
            number(&mut arguments);
            match arguments.peek().unwrap() {
                TucanArgument::Number(_) => {
                    let a = number(&mut arguments).to_be_bytes();
                    assert_eq!(number(&mut arguments), 0);
                    let b = number(&mut arguments).to_be_bytes();
                    let c = number(&mut arguments).to_be_bytes();
                    TucanProgram::Registration(Registration {
                        path: vec![a, b, c].concat(),
                    })
                }
                TucanArgument::String(_) => {
                    assert_eq!(string(&mut arguments), "");
                    TucanProgram::RootRegistration(RootRegistration {})
                }
            }
        }
        "MYEXAMS" => {
            number(&mut arguments);
            let semester = match arguments.next() {
                Some(TucanArgument::Number(999)) => Semester::AllSemesters,
                Some(TucanArgument::Number(semester)) => Semester::Semester(semester),
                None => Semester::CurrentSemester,
                _ => panic!(),
            };
            TucanProgram::Myexams(Myexams { semester })
        }
        "COURSERESULTS" => {
            assert_eq!(number(&mut arguments), 324);
            let semester = match arguments.next() {
                Some(TucanArgument::Number(n)) => Some(n),
                None => None,
                _ => panic!("unexpected"),
            };
            TucanProgram::Courseresults(Courseresults { semester })
        }
        "EXAMRESULTS" => {
            assert_eq!(number(&mut arguments), 325);
            let semester = match arguments.next() {
                Some(TucanArgument::Number(n)) => Some(n),
                None => None,
                _ => panic!("unexpected"),
            };
            TucanProgram::Examresults(Examresults { semester })
        }
        "STUDENT_RESULT" => {
            number(&mut arguments);
            assert_eq!(number(&mut arguments), 0);
            assert_eq!(number(&mut arguments), 0);
            assert_eq!(number(&mut arguments), 0);
            assert_eq!(number(&mut arguments), 0);
            assert_eq!(number(&mut arguments), 0);
            assert_eq!(number(&mut arguments), 0);
            TucanProgram::StudentResult(StudentResult)
        }
        "MODULEDETAILS" => {
            number(&mut arguments);
            let program = TucanProgram::Moduledetails(Moduledetails {
                id: number(&mut arguments).to_be_bytes().to_vec(),
            });
            assert!(matches!(
                arguments.next(),
                None | Some(TucanArgument::String(_))
            ));
            program
        }
        "COURSEDETAILS" => {
            number(&mut arguments);
            number(&mut arguments);
            let prog = TucanProgram::Coursedetails(Coursedetails {
                id: vec![
                    number(&mut arguments).to_be_bytes(), // this *should* be unique per course
                    number(&mut arguments).to_be_bytes(), // this *should* be different per sub-group in a course and the course itself - but you can't find out which of these is the top-level course page without fetching
                ]
                .concat(),
            });
            assert_eq!(number(&mut arguments), 0);
            assert_eq!(number(&mut arguments), 0);
            assert!([
                Some(TucanArgument::Number(0)),
                Some(TucanArgument::Number(3)),
                None
            ]
            .contains(&arguments.next()));
            assert!(matches!(
                arguments.next(),
                None | Some(TucanArgument::String(_))
            ));
            prog
        }
        "PERSADDRESS" => {
            number(&mut arguments);
            assert!(matches!(
                arguments.next(),
                None | Some(TucanArgument::String(_))
            ));
            TucanProgram::Persaddress(Persaddress)
        }
        "EXAMDETAILS" => {
            assert!(matches!(arguments.next(), Some(TucanArgument::Number(318))));
            let id = number(&mut arguments).to_be_bytes();
            assert!(matches!(arguments.next(), Some(TucanArgument::Number(0))));
            assert!(matches!(arguments.next(), Some(TucanArgument::String("M"))));
            number(&mut arguments); // nobody knows what this is
            TucanProgram::Examdetails(Examdetails { id: id.to_vec() })
        }
        "COURSEPREP" => {
            assert!(matches!(arguments.next(), Some(TucanArgument::Number(318))));
            assert!(matches!(arguments.next(), Some(TucanArgument::Number(0))));
            let id = number(&mut arguments);
            TucanProgram::Courseprep(Courseprep {
                id: id.try_into().unwrap(),
            })
        }
        "ACTION" => TucanProgram::Action(Action {
            magic: string(&mut arguments).to_string(),
        }),
        other => {
            panic!("invalid appname: {other}");
        }
    };

    assert!(
        arguments.peek().is_none(),
        "too many arguments while parsing {} {:?}",
        prgname,
        arguments.collect::<Vec<_>>()
    );

    TucanUrl {
        session_nr,
        program,
    }
}

#[cfg(test)]
mod tests {
    use super::parse_tucan_url;

    #[test]
    fn test_sample_urls() {
        // unauthenticated start page
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome");

        // authenticated start page
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N707546050471776,-N000019,");

        // Veranstaltungen
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N428926119975172,-N000273,-Astudveranst%2Ehtml");

        // Veranstaltungen -> Meine Module
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYMODULES&ARGUMENTS=-N428926119975172,-N000275,-N999");

        // Veranstaltungen -> Meine Veranstaltungen
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PROFCOURSES&ARGUMENTS=-N428926119975172,-N000274,-N999");

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

        // Kursdetails
        let _url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N967307082288504,-N000311,-N0,-N379144023730730,-N379144023752731,-N0,-N0");
    }
}
