use std::{
    borrow::Borrow,
    collections::HashMap,
    convert::TryInto,
    io::{Error, ErrorKind},
};

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
    MaybeAuthenticated {
        session_nr: Option<u64>,
        url: MaybeAuthenticatedTucanUrl,
    },
}

#[derive(PartialEq, Debug)]
pub enum UnauthenticatedTucanUrl {}

#[derive(PartialEq, Debug)]
pub enum MaybeAuthenticatedTucanUrl {
    StartpageDispatch,
    Externalpages { id: u64, name: String },
}

#[derive(PartialEq, Debug)]
pub enum AuthenticatedTucanUrl {
    Mlsstart,
    Mymodules,
    Profcourses,
    Studentchoicecourses,
    Registration,
    Myexams,
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
    pub fn number(&self) -> anyhow::Result<u64> {
        match self {
            TucanArgument::Number(number) => Ok(*number),
            _ => Err(Error::new(ErrorKind::Other, format!("not a number: {:?}", self)).into()),
        }
    }

    pub fn string(&self) -> anyhow::Result<&'a str> {
        match self {
            TucanArgument::String(string) => Ok(string),
            _ => Err(Error::new(ErrorKind::Other, format!("not a string: {:?}", self)).into()),
        }
    }
}

pub fn parse_arguments<'a>(
    arguments: &'a str,
) -> impl Iterator<Item = anyhow::Result<TucanArgument<'a>>> + std::fmt::Debug {
    arguments
        .split_terminator(",")
        .map(|a| -> anyhow::Result<TucanArgument> {
            Ok(match a.get(0..2) {
                Some("-N") => TucanArgument::Number(a[2..].parse::<u64>()?),
                Some("-A") => TucanArgument::String(&a[2..]),
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

fn number<'a>(
    arguments: &mut (impl Iterator<Item = anyhow::Result<TucanArgument<'a>>> + std::fmt::Debug),
) -> Result<u64, anyhow::Error> {
    arguments
        .next()
        .ok_or(Error::new(
            ErrorKind::Other,
            format!("not enough arguments"),
        ))??
        .number()
}

fn string<'a>(
    arguments: &mut (impl Iterator<Item = anyhow::Result<TucanArgument<'a>>> + std::fmt::Debug),
) -> Result<&'a str, anyhow::Error> {
    let a: TucanArgument<'a> = arguments.next().ok_or(Error::new(
        ErrorKind::Other,
        format!("not enough arguments"),
    ))??;
    a.string()
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
        return Err(Error::new(ErrorKind::Other, format!("invalid path: {}", url.path())).into());
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
    let prgname = query_pairs
        .get("PRGNAME")
        .ok_or(Error::new(
            ErrorKind::Other,
            format!("no APPNAME in url: {:?}", query_pairs),
        ))?
        .as_ref();
    let mut arguments = parse_arguments(arguments);
    if app_name != "CampusNet" {
        return Err(Error::new(ErrorKind::Other, format!("invalid appname: {}", app_name)).into());
    }

    let session_nr = if prgname == "ACTION" {
        1
    } else {
        arguments
            .next()
            .ok_or(Error::new(
                ErrorKind::Other,
                format!("no session_nr in arguments {:?}", arguments),
            ))??
            .number()?
    };
    let session_nr = if session_nr == 1 {
        Err(Error::new(ErrorKind::Other, format!("not logged in")))
    } else {
        Ok(session_nr)
    };

    let result = match prgname {
        "STARTPAGE_DISPATCH" => {
            if number(&mut arguments)? != 19 {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("unknown STARTPAGE_DISPATCH number"),
                )
                .into());
            }
            if number(&mut arguments)? != 0 {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("unknown STARTPAGE_DISPATCH number"),
                )
                .into());
            }
            Ok(TucanUrl::MaybeAuthenticated {
                session_nr: session_nr.ok(),
                url: MaybeAuthenticatedTucanUrl::StartpageDispatch,
            })
        }
        "EXTERNALPAGES" => Ok(TucanUrl::MaybeAuthenticated {
            session_nr: session_nr.ok(),
            url: MaybeAuthenticatedTucanUrl::Externalpages {
                id: number(&mut arguments)?,
                name: string(&mut arguments)?.to_string(),
            },
        }),
        "MLSSTART" => {
            if number(&mut arguments)? != 19 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown mlsstart number")).into(),
                );
            }
            Ok(TucanUrl::Authenticated {
                session_nr: session_nr?,
                url: AuthenticatedTucanUrl::Mlsstart,
            })
        }
        "MYMODULES" => {
            if number(&mut arguments)? != 275 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown mymodules number")).into(),
                );
            }
            Ok(TucanUrl::Authenticated {
                session_nr: session_nr?,
                url: AuthenticatedTucanUrl::Mymodules,
            })
        }
        "PROFCOURSES" => {
            if number(&mut arguments)? != 274 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown profcourses number")).into(),
                );
            }
            Ok(TucanUrl::Authenticated {
                session_nr: session_nr?,
                url: AuthenticatedTucanUrl::Profcourses,
            })
        }
        "STUDENTCHOICECOURSES" => {
            if number(&mut arguments)? != 307 {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("unknown STUDENTCHOICECOURSES number"),
                )
                .into());
            }
            Ok(TucanUrl::Authenticated {
                session_nr: session_nr?,
                url: AuthenticatedTucanUrl::Studentchoicecourses,
            })
        }
        "REGISTRATION" => {
            if number(&mut arguments)? != 311 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown REGISTRATION number")).into(),
                );
            }
            if string(&mut arguments)? != "" {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown REGISTRATION string")).into(),
                );
            }
            Ok(TucanUrl::Authenticated {
                session_nr: session_nr?,
                url: AuthenticatedTucanUrl::Registration,
            })
        }
        "MYEXAMS" => {
            if number(&mut arguments)? != 318 {
                return Err(Error::new(ErrorKind::Other, format!("unknown MYEXAMS number")).into());
            }
            Ok(TucanUrl::Authenticated {
                session_nr: session_nr?,
                url: AuthenticatedTucanUrl::Myexams,
            })
        }
        "COURSERESULTS" => {
            if number(&mut arguments)? != 324 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown COURSERESULTS number")).into(),
                );
            }
            Ok(TucanUrl::Authenticated {
                session_nr: session_nr?,
                url: AuthenticatedTucanUrl::Courseresults,
            })
        }
        "EXAMRESULTS" => {
            if number(&mut arguments)? != 325 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown EXAMRESULTS number")).into(),
                );
            }
            Ok(TucanUrl::Authenticated {
                session_nr: session_nr?,
                url: AuthenticatedTucanUrl::Examresults,
            })
        }
        "STUDENT_RESULT" => {
            if number(&mut arguments)? != 316 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown STUDENTRESULT number")).into(),
                );
            }
            if number(&mut arguments)? != 0 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown STUDENTRESULT number")).into(),
                );
            }
            if number(&mut arguments)? != 0 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown STUDENTRESULT number")).into(),
                );
            }
            if number(&mut arguments)? != 0 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown STUDENTRESULT number")).into(),
                );
            }
            if number(&mut arguments)? != 0 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown STUDENTRESULT number")).into(),
                );
            }
            if number(&mut arguments)? != 0 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown STUDENTRESULT number")).into(),
                );
            }
            if number(&mut arguments)? != 0 {
                return Err(
                    Error::new(ErrorKind::Other, format!("unknown STUDENTRESULT number")).into(),
                );
            }
            Ok(TucanUrl::Authenticated {
                session_nr: session_nr?,
                url: AuthenticatedTucanUrl::StudentResult,
            })
        }
        other => {
            return Err(Error::new(ErrorKind::Other, format!("invalid appname: {}", other)).into())
        }
    };

    let mut peekable = arguments.peekable();
    if peekable.peek().is_some() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("too many arguments {:?}", peekable.collect::<Vec<_>>()),
        )
        .into());
    }

    println!("{:?}", result);

    return result;
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

        // unauthenticated start page
        let url = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome")?;

        // authenticated start page
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
