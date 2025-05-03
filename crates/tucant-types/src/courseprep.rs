use std::{convert::Infallible, fmt::Display, str::FromStr, sync::LazyLock};

use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CoursePrepRequest {
    timetable_id: u64,
    pub course_id: u64,
    pub r#type: CoursePrepType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub enum CoursePrepType {
    PersonalAppointment,
    Course,
    Module,
}

impl FromStr for CoursePrepRequest {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl Display for CoursePrepRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-N0,-N{},-A{},-N{}",
            self.timetable_id,
            match self.r#type {
                CoursePrepType::PersonalAppointment => "",
                CoursePrepType::Course => "CODA",
                CoursePrepType::Module => "MOFF",
            },
            self.course_id
        )
    }
}

impl CoursePrepRequest {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        // coda seems to be for courses and moff seems to be for modules
        // seems like we need to fix some stuff
        // CODA or MOFF
        // having -A with nothing here is a custom appointment just for you
        static COURSE_DETAILS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-N0,-N(?P<timetable>\d+),-A(?P<type>[a-zA-Z0-9_~-]*),-N(?P<course>\d+)$").unwrap());
        let c = &COURSE_DETAILS_REGEX.captures(input).expect(input);
        Self {
            timetable_id: c["timetable"].parse().unwrap(),
            course_id: c["course"].parse().unwrap(),
            r#type: match &c["type"] {
                "" => CoursePrepType::PersonalAppointment,
                "CODA" => CoursePrepType::Course,
                "MOFF" => CoursePrepType::Module,
                _ => panic!(),
            },
        }
    }
}
