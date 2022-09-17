use std::{borrow::Borrow, collections::HashMap, convert::TryInto, io::{ErrorKind, Error}};

use url::{form_urlencoded, Host, Origin, Url};

#[derive(PartialEq, Debug)]
pub enum TucanUrl {
    Unauthenticated {
        url: UnauthenticatedTucanUrl,
    },
    Authenticated {
        session_nr: u64,
        url: AuthenticatedTucanUrl,
    },
    // MaybeAuthenticatedTucanUrl
}

#[derive(PartialEq, Debug)]
pub enum UnauthenticatedTucanUrl {
    StartpageDispatch,
}

#[derive(PartialEq, Debug)]
pub enum AuthenticatedTucanUrl {
    Externalpages { id: u64, name: String },
    Mlsstart,
    Mymodules,
    Profcourse,
    Studentchoicecourses,
    Registration,
    Courseresults,
    Examresults,
    StudentResult,
}

#[derive(Debug)]
pub enum TucanArgument<'a> {
    Number(u64),
    String(&'a str),
}

impl<'a> TucanArgument<'a> {
    pub fn number(&'a self) -> anyhow::Result<u64> {
        match self {
            TucanArgument::Number(number) => Ok(*number),
            _ => Err(
                Error::new(ErrorKind::Other, format!("not a number: {:?}", self)).into(),
            ),
        }
    }

    pub fn string(&'a self) -> anyhow::Result<&'a str> {
        match self {
            TucanArgument::String(string) => Ok(string),
            _ => Err(
                Error::new(ErrorKind::Other, format!("not a string: {:?}", self)).into(),
            ),
        }
    }
}

pub fn parse_arguments<'a>(
    arguments: &'a str,
) -> impl Iterator<Item = anyhow::Result<TucanArgument<'a>>> + std::fmt::Debug {
    arguments
        .split(",")
        .map(|a| -> anyhow::Result<TucanArgument> {
            Ok(match &a[0..2] {
                "-N" => TucanArgument::Number(a[2..].parse::<u64>()?),
                "-A" => TucanArgument::String(&a[2..]),
                other => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("invalid argument type: {:?}", other),
                    )
                    .into())
                }
            })
        })
}

pub fn parse_tucan_url<'a>(url: &'a str) -> anyhow::Result<TucanUrl> {
    let url = Url::parse(url)?;
    if url.origin()
        != Origin::Tuple(
            "https".into(),
            Host::Domain("www.tucan.tu-darmstadt.de".into()),
            443,
        )
    {
        return Err(Error::new(
            ErrorKind::Other,
            format!("invalid origin: {:?}", url.origin()),
        )
        .into());
    }
    if url.path() != "/scripts/mgrqispi.dll" {
        return Err(
            Error::new(ErrorKind::Other, format!("invalid path: {}", url.path())).into(),
        );
    }
    let query_pairs = url.query_pairs();
    let query_pairs = query_pairs.collect::<HashMap<_, _>>();
    let app_name = query_pairs
        .get("APPNAME")
        .ok_or(Error::new(
            ErrorKind::Other,
            format!("no APPNAME in url: {:?}", query_pairs),
        ))?
        .as_ref();
    let arguments = query_pairs
        .get("ARGUMENTS")
        .ok_or(Error::new(
            ErrorKind::Other,
            format!("no ARGUMENTS in url: {:?}", query_pairs),
        ))?
        .as_ref();
    let mut arguments = parse_arguments(arguments);
    if app_name != "CampusNet" {
        return Err(Error::new(
            ErrorKind::Other,
            format!("invalid appname: {}", app_name),
        )
        .into());
    }

    let session_nr = arguments.next().ok_or(Error::new(
        ErrorKind::Other,
        format!("no session_nr in arguments {:?}", arguments),
    ))??;

    match query_pairs
        .get("PRGNAME")
        .ok_or(Error::new(
            ErrorKind::Other,
            format!("no APPNAME in url: {:?}", query_pairs),
        ))?
        .as_ref()
    {
        "STARTPAGE_DISPATCH" => {
            if arguments.next().is_some() {
                return Err(
                    Error::new(ErrorKind::Other, format!("too many arguments")).into(),
                );
            }
            return Ok(TucanUrl::Unauthenticated {
                url: UnauthenticatedTucanUrl::StartpageDispatch,
            });
        }
        "EXTERNALPAGES" => {
            return Ok(TucanUrl::Authenticated {
                session_nr: session_nr.number()?,
                url: AuthenticatedTucanUrl::Externalpages {
                    id: arguments
                        .next()
                        .ok_or(Error::new(
                            ErrorKind::Other,
                            format!("not enough arguments"),
                        ))??
                        .number()?,
                    name: arguments
                        .next()
                        .ok_or(Error::new(
                            ErrorKind::Other,
                            format!("not enough arguments"),
                        ))??
                        .string()?
                        .to_string(),
                },
            })
        }
        other => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("invalid appname: {}", other),
            )
            .into())
        }
    }

    Err(Error::new(ErrorKind::Other, "oh no!").into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_urls() -> anyhow::Result<()> {
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001")?;
        assert_eq!(
            TucanUrl::Unauthenticated {
                url: UnauthenticatedTucanUrl::StartpageDispatch
            },
            url
        );

        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome")?;

        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N707546050471776,-N000019,")?;

        // Veranstaltungen
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N428926119975172,-N000273,-Astudveranst%2Ehtml")?;

        // Veranstaltungen -> Meine Module
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYMODULES&ARGUMENTS=-N428926119975172,-N000275,")?;

        // Veranstaltungen -> Meine Veranstaltungen
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PROFCOURSES&ARGUMENTS=-N428926119975172,-N000274,")?;

        // Veranstaltungen -> Meine Wahlbereiche
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENTCHOICECOURSES&ARGUMENTS=-N428926119975172,-N000307,")?;

        // Veranstaltungen -> Anmeldung
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N428926119975172,-N000311,-A")?;

        // Prüfungen
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N428926119975172,-N000280,-Astudpruefungen%2Ehtml")?;

        // Prüfungen -> Meine Prüfungen
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYEXAMS&ARGUMENTS=-N428926119975172,-N000318,")?;

        // Prüfungen -> Semesterergebnisse
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N428926119975172,-N000323,-Astudergebnis%2Ehtml")?;

        // Prüfungen -> Semesterergebnisse -> Modulergebnisse
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSERESULTS&ARGUMENTS=-N428926119975172,-N000324,")?;

        // Prüfungen -> Semesterergebnisse -> Prüfungsergebnisse
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXAMRESULTS&ARGUMENTS=-N428926119975172,-N000325,")?;

        // Prüfungen -> Leistungsspiegel
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N428926119975172,-N000316,-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000")?;

        // urls we still need to reverse

        // Nachrichten
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AkaTs6g9bP0R3xzedtop4miuoi56H3Qg425njHHR6KzEmalVi4oDtkK6~xkg9cRLwvmeiajCHw3PN266Zf3GOdaSKKSxNL-p6ZaQI~5oVIcIdkWSynh328JX-tBVvFclUzM2edYwexy0VrWXCiHjX7-Mr3oU_")?;

        // Vorlesungsverzeichnis
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AhFucMz0tWte~bVYvS7ZXP5dvkkE-wxWuME0Wqlj3rB-iwNGmsluzwhK5irFXFH0SgStfWj6FpAVtU2MQ32Ym4VKspT-EJN252qy~QgsOsLLZU7b~VRfzznhHKnzAJuhARdmMM1nx~31tKkgN6ETdcmIeCTfaeM874hp8aM3ass8q8PkZovZFJHWUlQ__")?;

        Ok(())
    }
}
