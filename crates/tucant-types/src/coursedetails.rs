use std::{convert::Infallible, fmt::Display, str::FromStr, sync::LazyLock};

use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::InstructorImage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CourseDetailsRequest(String);

impl FromStr for CourseDetailsRequest {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl Display for CourseDetailsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CourseDetailsRequest {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        // for your own courses the first number is actually not zero.
        // the second number seems to be the reference to the course
        // the third number seems to be a reference to the material and messages
        // creating a mixed url https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N258189491926736,-N000326,-N0,-N391553568139410,-N391035558524778,-N0,-N0,-N0
        // shows that the third number influences the message tab and the termine.
        // the third number seems to be for subcourses like exercises.
        // the dates with stars are from the lecture and the ones without are for exercises
        // TODO for now we ignore the first number, maybe that is a mistake
        static COURSE_DETAILS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-N\d+,-N(?P<n2>\d+),-N(?P<n3>\d+),-N(0),-N(0+)(,-N(0|2|3)(,-A[a-zA-Z0-9_~-]+)?)?$").unwrap());
        let c = &COURSE_DETAILS_REGEX.captures(input).expect(input);
        Self(format!("-N0,-N{},-N{},-N0,-N0,-N0", &c["n2"], &c["n3"]))
    }

    #[must_use]
    pub const fn inner(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct CourseDetailsResponse {
    pub id: String,
    pub name: String,
    pub material_and_messages_url: Option<(String, String)>,
    pub r#type: String,
    pub type_number: u64,
    pub fachbereich: String,
    pub anzeige_im_stundenplan: Option<String>,
    pub courselevel: u64,
    pub sws: Option<f64>,
    pub credits: Option<u64>,
    pub language: String,
    pub language_id: u64,
    pub teilnehmer_min: Option<u64>,
    pub teilnehmer_max: Option<u64>,
    pub description: Vec<String>,
    pub uebungsgruppen: Vec<CourseUebungsGruppe>,
    pub course_anmeldefristen: Vec<CourseAnmeldefrist>,
    pub enhalten_in_modulen: Vec<String>,
    pub termine: Vec<Termin>,
    pub termine_kleingruppe: Vec<Termin>,
    pub short_termine: Vec<(String, String)>, // TODO verify is equivalent to termine
    pub instructors: Vec<(String, Option<InstructorImageWithLink>)>,
    pub plenumsveranstaltung_url: Option<CourseDetailsRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct InstructorImageWithLink {
    pub href: String,
    pub inner: InstructorImage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CourseUebungsGruppe {
    pub name: String,
    pub uebungsleiter: String,
    pub date_range: Option<String>,
    pub url: CourseDetailsRequest,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CourseAnmeldefrist {
    pub zulassungstyp: String,
    pub block_type: String,
    pub start: Option<String>,
    pub ende_anmeldung: Option<String>,
    pub ende_abmeldung: Option<String>,
    pub ende_hoerer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Room {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Termin {
    pub id: String,
    pub date: String,
    pub time_start: String,
    pub time_end: String,
    pub rooms: Vec<Room>,
    pub instructors: Option<String>,
}
